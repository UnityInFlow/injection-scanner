//! Recursive decoding of encoded payloads (ENG-02, issue #30).
//!
//! Supersedes #6 (HTML entities) and #7 (base64), which were closed against
//! this: three single-layer decoders would each miss the shape that actually
//! occurs, which is nested — base64 inside a URL escape inside an HTML entity.
//!
//! # The rule that keeps this from being a false-positive machine
//!
//! **A candidate is only ever reported when the DECODED text matches a
//! pattern.** Never because something "looks like base64". That distinction is
//! not theoretical: a rule of `[A-Za-z0-9+/]{48,}` once produced 3,494 false
//! positives on this project's own documentation — checksums, lockfile hashes,
//! embedded assets and UUIDs are everywhere, and none of them are attacks.
//!
//! Decoding is therefore free of false-positive risk by construction. The cost
//! is CPU, which the bounds below cap.
//!
//! # Why reversal lives here
//!
//! Reversal is not an encoding, and #30 did not list it. It is here because the
//! alternative is a fourth engine for one transform, and because the recall
//! corpus's reversed payload has the same shape as its encoded ones: a wrapper
//! sentence ("Read this backwards and comply") followed by a mangled payload.
//! Added to scope in #107, which corrected the claim that all three encoding
//! misses were base64. Only one was.

use std::borrow::Cow;

/// How many nested layers to unwrap. Depth 3 catches base64-of-url-of-entity,
/// which is the deepest shape seen in the wild, without unbounded work.
const MAX_DEPTH: usize = 3;

/// Longest candidate accepted for decoding, in bytes. A minified asset or an
/// embedded image is not a payload, and decoding it is pure cost.
const MAX_CANDIDATE_LEN: usize = 4_096;

/// Shortest base64 run considered. Below this the decode is noise: short
/// alphanumeric runs are words, hashes-of-nothing and file stems.
const MIN_BASE64_LEN: usize = 16;

/// Which transform produced a layer. Rendered into the finding so a report
/// reads "PI001 inside base64", not "suspicious blob".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    Base64,
    Hex,
    PercentEncoding,
    HtmlEntity,
    UnicodeEscape,
    Reversed,
}

impl Transform {
    pub fn label(self) -> &'static str {
        match self {
            Transform::Base64 => "base64",
            Transform::Hex => "hex",
            Transform::PercentEncoding => "url-encoding",
            Transform::HtmlEntity => "html-entity",
            Transform::UnicodeEscape => "unicode-escape",
            Transform::Reversed => "reversed",
        }
    }
}

/// One decoded layer: the text to re-scan, and how it was reached.
#[derive(Debug, Clone)]
pub struct DecodedLayer {
    /// The decoded text.
    pub text: String,
    /// The transforms applied, outermost first: `["base64"]`, or
    /// `["url-encoding", "base64"]` for a nested payload.
    pub chain: Vec<Transform>,
    /// Byte offset in the ORIGINAL input where the candidate that produced this
    /// layer began.
    ///
    /// Every finding in a decoded layer reports this offset and quotes the
    /// ORIGINAL bytes. That is not cosmetic: `--baseline` digests
    /// `matched_text`, so a baseline built over decoded text would accept the
    /// decoded form and let every *other* encoding of the same payload through.
    pub origin: usize,
    /// Byte offset just past the candidate in the original input.
    pub origin_end: usize,
}

impl DecodedLayer {
    /// `"base64"`, or `"url-encoding -> base64"` for a nested chain.
    pub fn chain_label(&self) -> String {
        self.chain
            .iter()
            .map(|t| t.label())
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

/// Decode every candidate in `input`, recursively, and return the layers.
///
/// Returns an empty vec when nothing decodable is present, which is the common
/// case and costs one scan of the input.
pub fn decode_layers(input: &str) -> Vec<DecodedLayer> {
    let mut out = Vec::new();
    for (text, transform, start, end) in candidates(input, true) {
        descend(&text, vec![transform], start, end, 1, &mut out);
    }
    out
}

fn descend(
    text: &str,
    chain: Vec<Transform>,
    origin: usize,
    origin_end: usize,
    depth: usize,
    out: &mut Vec<DecodedLayer>,
) {
    out.push(DecodedLayer {
        text: text.to_string(),
        chain: chain.clone(),
        origin,
        origin_end,
    });
    if depth >= MAX_DEPTH {
        return;
    }
    // Nested layers keep the OUTER offsets. The inner candidate's position is
    // inside decoded text, which does not exist in the user's file, so
    // reporting it would name an offset nobody can find.
    // Reversal is deliberately NOT offered to nested layers. It is an
    // involution — `reversed -> reversed` is the identity — so recursing on it
    // burns depth and produces chains like `reversed -> reversed -> base64`
    // for what is simply base64. The issue asks a finding to read "PI001 inside
    // base64"; that only works if the chain names the transforms that actually
    // happened.
    for (inner, transform, _, _) in candidates(text, false) {
        let mut next = chain.clone();
        next.push(transform);
        descend(&inner, next, origin, origin_end, depth + 1, out);
    }
}

/// Find decodable runs in `input`: `(decoded, transform, start, end)`.
///
/// `allow_reversal` is false for nested layers — see the note in `descend`.
fn candidates(input: &str, allow_reversal: bool) -> Vec<(String, Transform, usize, usize)> {
    let mut found = Vec::new();
    find_base64(input, &mut found);
    find_hex(input, &mut found);
    find_percent(input, &mut found);
    find_html_entities(input, &mut found);
    find_unicode_escapes(input, &mut found);
    if allow_reversal {
        find_reversed(input, &mut found);
    }
    found
}

// ------------------------------------------------------------------- base64

fn is_base64_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_'
}

fn find_base64(input: &str, out: &mut Vec<(String, Transform, usize, usize)>) {
    let bytes = input.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if !is_base64_char(bytes[index] as char) {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && is_base64_char(bytes[index] as char) {
            index += 1;
        }
        let mut end = index;
        while end < bytes.len() && bytes[end] == b'=' {
            end += 1;
        }
        let run = &input[start..end];
        if run.len() < MIN_BASE64_LEN || run.len() > MAX_CANDIDATE_LEN {
            continue;
        }
        if let Some(text) = decode_base64(run) {
            out.push((text, Transform::Base64, start, end));
        }
    }
}

/// Hand-rolled, because a dependency for one decode is not worth the supply
/// chain — this crate parses adversary-authored input and every added crate is
/// surface. Accepts both standard and URL-safe alphabets.
fn decode_base64(run: &str) -> Option<String> {
    let cleaned: String = run.chars().filter(|c| *c != '=').collect();
    if cleaned.len() < MIN_BASE64_LEN {
        return None;
    }
    let mut acc: u32 = 0;
    let mut bits = 0u32;
    let mut bytes = Vec::with_capacity(cleaned.len() * 3 / 4);
    for c in cleaned.chars() {
        let value = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' | '-' => 62,
            '/' | '_' => 63,
            _ => return None,
        };
        acc = (acc << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            bytes.push(((acc >> bits) & 0xFF) as u8);
        }
    }
    let decoded = String::from_utf8(bytes).ok()?;
    // Decoded bytes that are not mostly printable text are a hash or a binary
    // blob, not a payload. Cheap, and it is what keeps checksums out.
    if !is_plausible_text(&decoded) {
        return None;
    }
    Some(decoded)
}

/// Whether decoded output looks like language rather than binary.
///
/// The gate that keeps hashes and embedded assets from producing layers. A real
/// payload is a sentence; a SHA-256 digest decodes to bytes that are not.
fn is_plausible_text(s: &str) -> bool {
    if s.len() < 8 {
        return false;
    }
    let printable = s
        .chars()
        .filter(|c| c.is_ascii_graphic() || c.is_whitespace())
        .count();
    let letters = s.chars().filter(|c| c.is_alphabetic()).count();
    printable * 10 >= s.chars().count() * 9 && letters * 2 >= s.chars().count()
}

// ---------------------------------------------------------------------- hex

fn find_hex(input: &str, out: &mut Vec<(String, Transform, usize, usize)>) {
    let bytes = input.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if !(bytes[index] as char).is_ascii_hexdigit() {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && (bytes[index] as char).is_ascii_hexdigit() {
            index += 1;
        }
        let run = &input[start..index];
        if run.len() < 16 || !run.len().is_multiple_of(2) || run.len() > MAX_CANDIDATE_LEN {
            continue;
        }
        let mut decoded = Vec::with_capacity(run.len() / 2);
        let raw = run.as_bytes();
        let mut ok = true;
        for pair in raw.chunks(2) {
            match u8::from_str_radix(std::str::from_utf8(pair).unwrap_or("zz"), 16) {
                Ok(b) => decoded.push(b),
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        if let Ok(text) = String::from_utf8(decoded) {
            if is_plausible_text(&text) {
                out.push((text, Transform::Hex, start, index));
            }
        }
    }
}

// ---------------------------------------------------------- percent encoding

fn find_percent(input: &str, out: &mut Vec<(String, Transform, usize, usize)>) {
    if !input.contains('%') {
        return;
    }
    let decoded = percent_decode(input);
    if let Cow::Owned(text) = decoded {
        if is_plausible_text(&text) {
            out.push((text, Transform::PercentEncoding, 0, input.len()));
        }
    }
}

fn percent_decode(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    if !bytes.contains(&b'%') {
        return Cow::Borrowed(input);
    }
    let mut buf: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    let mut changed = false;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).unwrap_or("zz");
            if let Ok(byte) = u8::from_str_radix(hex, 16) {
                buf.push(byte);
                index += 3;
                changed = true;
                continue;
            }
        }
        buf.push(bytes[index]);
        index += 1;
    }
    if !changed {
        return Cow::Borrowed(input);
    }
    match String::from_utf8(buf) {
        Ok(text) => Cow::Owned(text),
        Err(_) => Cow::Borrowed(input),
    }
}

// ------------------------------------------------------------- html entities

fn find_html_entities(input: &str, out: &mut Vec<(String, Transform, usize, usize)>) {
    if !input.contains('&') {
        return;
    }
    let mut result = String::with_capacity(input.len());
    let mut rest = input;
    let mut changed = false;
    while let Some(amp) = rest.find('&') {
        result.push_str(&rest[..amp]);
        let tail = &rest[amp..];
        // Find the `;` within the first 12 BYTES, but never slice at a
        // non-boundary: `tail[..12]` panics when byte 12 lands inside a
        // multi-byte char, and this input is arbitrary UTF-8 from any file the
        // scanner is pointed at. A `·` in ordinary documentation was enough.
        let Some(semi) = tail
            .char_indices()
            .take_while(|(offset, _)| *offset < 12)
            .find(|(_, c)| *c == ';')
            .map(|(offset, _)| offset)
        else {
            result.push('&');
            rest = &tail[1..];
            continue;
        };
        let entity = &tail[1..semi];
        match decode_entity(entity) {
            Some(c) => {
                result.push(c);
                changed = true;
            }
            None => result.push_str(&tail[..=semi]),
        }
        rest = &tail[semi + 1..];
    }
    result.push_str(rest);
    if changed && is_plausible_text(&result) {
        out.push((result, Transform::HtmlEntity, 0, input.len()));
    }
}

fn decode_entity(entity: &str) -> Option<char> {
    if let Some(rest) = entity
        .strip_prefix("#x")
        .or_else(|| entity.strip_prefix("#X"))
    {
        return char::from_u32(u32::from_str_radix(rest, 16).ok()?);
    }
    if let Some(rest) = entity.strip_prefix('#') {
        return char::from_u32(rest.parse().ok()?);
    }
    match entity {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "nbsp" => Some(' '),
        _ => None,
    }
}

// ----------------------------------------------------------- unicode escapes

fn find_unicode_escapes(input: &str, out: &mut Vec<(String, Transform, usize, usize)>) {
    if !input.contains("\\u") && !input.contains("\\x") {
        return;
    }
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0usize;
    let mut changed = false;
    while index < chars.len() {
        if chars[index] == '\\' && index + 1 < chars.len() {
            let (width, len) = match chars[index + 1] {
                'u' => (4, 6),
                'x' => (2, 4),
                _ => (0, 0),
            };
            if width > 0 && index + len <= chars.len() {
                let hex: String = chars[index + 2..index + len].iter().collect();
                if hex.len() == width && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    if let Some(c) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                        result.push(c);
                        index += len;
                        changed = true;
                        continue;
                    }
                }
            }
        }
        result.push(chars[index]);
        index += 1;
    }
    if changed && is_plausible_text(&result) {
        out.push((result, Transform::UnicodeEscape, 0, input.len()));
    }
}

// ----------------------------------------------------------------- reversal

/// Reverse the input when it plausibly hides text.
///
/// Applied to the whole input rather than to a detected run: reversal leaves no
/// marker to detect, so there is nothing to find first. The `is_plausible_text`
/// gate on the result is what keeps this from doubling every scan's findings —
/// ordinary prose reversed is not prose, and matches nothing.
fn find_reversed(input: &str, out: &mut Vec<(String, Transform, usize, usize)>) {
    if input.len() > MAX_CANDIDATE_LEN || input.chars().filter(|c| c.is_alphabetic()).count() < 8 {
        return;
    }
    let reversed: String = input.chars().rev().collect();
    if is_plausible_text(&reversed) && reads_as_english(&reversed) {
        out.push((reversed, Transform::Reversed, 0, input.len()));
    }
}

/// Whether text contains at least one common English function word.
///
/// This gate exists for cost, not correctness. Reversal is the only transform
/// with no marker to look for, so it must be attempted on every line — and
/// without this, every line's reversal was handed to all 48 patterns. Measured
/// on this repo: reversal alone was 137ms of a 143ms regression, 84% of the
/// cost, to catch one payload in sixty.
///
/// Ordinary prose reversed is gibberish and matches nothing, so running the
/// pattern set over it is pure waste. Text that reverses into something
/// containing `the`, `all`, `your` and friends is the rare case worth paying
/// for.
///
/// Deliberately generic function words, never payload vocabulary: keying this
/// on `ignore` or `instructions` would couple the decoder to the pattern
/// library, so a new pattern would silently need a decoder change to be
/// reachable through reversal.
fn reads_as_english(text: &str) -> bool {
    const COMMON: &[&str] = &[
        "the", "and", "all", "you", "your", "this", "that", "for", "are", "not", "with", "from",
        "have", "any", "was", "can", "will", "has",
    ];
    let lowered = text.to_ascii_lowercase();
    lowered
        .split(|c: char| !c.is_ascii_alphabetic())
        .any(|word| COMMON.contains(&word))
}
