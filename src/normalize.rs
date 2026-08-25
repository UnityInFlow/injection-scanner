//! Defeating obfuscation by normalizing before matching (issue #26, engine E1).
//!
//! Every pattern matched raw bytes, so the whole library fell to a find-and-
//! replace. All of these were invisible:
//!
//! ```text
//! ignore-all-previous-instructions          separator injection
//! i g n o r e   a l l   p r e v i o u s     spacing
//! іgnore all previous instructions          Cyrillic і (U+0456) homoglyph
//! ｉｇｎｏｒｅ all previous instructions        fullwidth
//! ig<U+200B>nore all previous instructions  zero-width interleave
//! ```
//!
//! One pass defeats all five, which is far more leverage than adding literal
//! patterns for each spelling — an attacker has unbounded spellings and we do
//! not.
//!
//! # Offsets are the whole problem
//!
//! Normalizing is easy; reporting is not. A finding has to name a real line and
//! quote real text, so every byte of the normalized string carries the offset it
//! came from. Without that the scanner would report matches against text the
//! user cannot find in their own file.
//!
//! # What this deliberately does not do
//!
//! Spacing evasion is handled only in its common form, where letters are split
//! by single spaces and words by runs of two or more. Fully despaced text
//! (`i g n o r e a l l p r e v i o u s`) is **not** normalized, because the
//! result would be `ignoreallprevious` and every pattern in the library joins
//! its words with `\s+`. Matching that would mean rewriting the pattern set, not
//! the input. Recorded rather than papered over.

use unicode_normalization::UnicodeNormalization;

/// Text with a byte-for-byte map back to where it came from.
pub struct Normalized {
    /// The normalized text.
    pub text: String,
    /// For each byte offset in `text`, the byte offset it originated at.
    origin: Vec<usize>,
    /// Length of the original input, for offsets at the very end.
    original_len: usize,
}

impl Normalized {
    /// Byte offset in the original input for byte offset `at` in `text`.
    pub fn original_offset(&self, at: usize) -> usize {
        self.origin
            .get(at)
            .copied()
            .unwrap_or(self.original_len)
            .min(self.original_len)
    }
}

/// Is this character invisible to a reader but present to a matcher?
///
/// Zero-width joiners, bidi controls, the soft hyphen, variation selectors and
/// combining marks. Interleaving any of them splits a word for the regex engine
/// while leaving it identical on screen — which is the entire attack.
fn is_invisible(c: char) -> bool {
    matches!(c,
        '\u{200B}'..='\u{200D}'   // zero-width space / non-joiner / joiner
        | '\u{FEFF}'              // zero-width no-break space
        | '\u{00AD}'              // soft hyphen
        | '\u{200E}' | '\u{200F}' // LTR / RTL marks
        | '\u{202A}'..='\u{202E}' // bidi embedding and override
        | '\u{2060}'..='\u{2064}' // word joiner, invisible operators
        | '\u{2066}'..='\u{2069}' // bidi isolates
        | '\u{FE00}'..='\u{FE0F}' // variation selectors
        | '\u{0300}'..='\u{036F}' // combining diacritical marks
        | '\u{E0000}'..='\u{E007F}' // Unicode tag block
    )
}

/// Separator punctuation an attacker uses in place of a space.
///
/// `ignore-all-previous-instructions` reads identically to a human and matches
/// nothing. Folded to a space so the library's `\s+` does the rest.
fn is_separator(c: char) -> bool {
    matches!(c, '-' | '_' | '.' | '*' | '+' | '~' | '/' | '|' | '\\')
}

/// Is this separator being used *as* a separator, rather than as punctuation?
///
/// Only when it sits directly between two word characters. The full stop ending
/// "…about prompts." is punctuation; the hyphens in `ignore-all-previous` are
/// load-bearing. Without this distinction every ordinary sentence counted as
/// changed, and the normalization pass ran on every document in the tree for
/// nothing.
fn is_injected_separator(chars: &[(usize, char)], index: usize) -> bool {
    if !is_separator(chars[index].1) {
        return false;
    }
    let before = index
        .checked_sub(1)
        .and_then(|i| chars.get(i))
        .is_some_and(|(_, c)| c.is_alphanumeric());
    let after = chars
        .get(index + 1)
        .is_some_and(|(_, c)| c.is_alphanumeric());
    before && after
}

/// Normalize `input`, or `None` if nothing changed.
///
/// `None` is the common case on real documents and lets the caller skip the
/// second matching pass entirely rather than scanning identical text twice.
pub fn normalize(input: &str) -> Option<Normalized> {
    let mut text = String::with_capacity(input.len());
    let mut origin: Vec<usize> = Vec::with_capacity(input.len());
    let mut changed = false;
    let mut pending_space: Option<(usize, usize)> = None;

    // Which lines look spaced out. Decided per LINE rather than per word: no
    // per-word threshold can be both low enough to rejoin "a l l" and high
    // enough to leave "a b" in ordinary prose alone.
    let spaced_out: Vec<bool> = input.lines().map(line_is_spaced_out).collect();
    let mut line_index = 0usize;

    let chars: Vec<(usize, char)> = input.char_indices().collect();
    for index in 0..chars.len() {
        let (offset, source) = chars[index];
        if source == '\n' {
            // Line structure is preserved. The caller maps offsets back to line
            // numbers, and collapsing newlines would make every finding in a
            // document report against line 1.
            pending_space = None;
            push(&mut text, &mut origin, '\n', offset);
            line_index += 1;
            continue;
        }

        if is_invisible(source) {
            changed = true;
            continue;
        }

        let separator_here = is_injected_separator(&chars, index);
        let folded = if source.is_whitespace() || separator_here {
            if separator_here {
                changed = true;
            }
            // Runs collapse to one space, emitted lazily so a trailing run never
            // reaches the output. The count matters on a spaced-out line, where
            // a single space is the evasion and a run of two or more is the real
            // word boundary.
            pending_space = Some(match pending_space {
                Some((at, count)) => (at, count + 1),
                None => (offset, 1),
            });
            continue;
        } else {
            source
        };

        if let Some((at, count)) = pending_space.take() {
            if spaced_out.get(line_index).copied().unwrap_or(false) && count == 1 {
                // `i g n o r e` -> `ignore`: the single space is the evasion.
                changed = true;
            } else {
                push(&mut text, &mut origin, ' ', at);
            }
        }

        // NFKC first (fullwidth, ligatures, compatibility forms), then the
        // confusable skeleton (Cyrillic/Greek lookalikes to their ASCII twin).
        // Order matters: NFKC turns `ｉ` into `i`, and the skeleton is defined
        // over the normalized form.
        for normalized in folded.nfkc() {
            // The skeleton is applied ONLY to non-ASCII. Its job is mapping a
            // lookalike onto its ASCII twin, and ASCII is already the twin —
            // running it over plain text is all cost and real damage: Unicode
            // considers `m` confusable with `rn`, so "normal clean text" came
            // back as "norrnal clean text". Every ASCII word in every document
            // would have been quietly rewritten before matching.
            let emitted = if normalized.is_ascii() {
                normalized.to_string()
            } else {
                let skeleton: String =
                    unicode_security::skeleton(&normalized.to_string()).collect();
                if skeleton.is_empty() {
                    normalized.to_string()
                } else {
                    skeleton
                }
            };
            for c in emitted.chars() {
                if c != source {
                    changed = true;
                }
                push(&mut text, &mut origin, c, offset);
            }
        }
    }

    // A trailing whitespace run is dropped, which is a change worth recording
    // only if something else already changed — trailing space alone is not
    // obfuscation.
    if !changed {
        return None;
    }

    Some(Normalized {
        text,
        origin,
        original_len: input.len(),
    })
}

fn push(text: &mut String, origin: &mut Vec<usize>, c: char, at: usize) {
    let before = text.len();
    text.push(c);
    origin.resize(text.len().max(before), at);
    for slot in origin.iter_mut().skip(before) {
        *slot = at;
    }
}

/// Does this line look like spacing evasion?
///
/// `i g n o r e   a l l   p r e v i o u s` is four-fifths single-character
/// tokens across seventeen of them. Ordinary prose is not: "a b test" is three
/// tokens, and a table row of initials is short. Both thresholds have to hold.
fn line_is_spaced_out(line: &str) -> bool {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let singles = tokens.iter().filter(|t| t.chars().count() == 1).count();
    tokens.len() >= 6 && singles * 5 >= tokens.len() * 4
}
