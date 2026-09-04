//! Lexical markdown context, so documentation *about* injection is not reported
//! as injection (issue #20, audit H-01, backlog engine E7).
//!
//! The scanner had no notion of structure, so it flagged its own README fifteen
//! times: nine payloads quoted inside a table describing what the patterns
//! detect, six inside fenced blocks showing sample output. Every security guide,
//! test fixture and RAG corpus containing security content has the same shape.
//!
//! Context does not mean "safe". A model ingesting a document reads the fenced
//! blocks too, so a payload there is not harmless — it is *less likely to be an
//! attack* than the same text in prose. That distinction is why this produces a
//! confidence score rather than dropping matches outright, and why `--strict`
//! can put every finding back.

use serde::{Deserialize, Serialize};

/// Where in a markdown document a match was found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchContext {
    /// Ordinary text. A payload here is what the scanner exists to find.
    Prose,
    /// Inside ``` or ~~~ fences — overwhelmingly sample output or a quoted
    /// attack in documentation.
    FencedCode,
    /// Inside a `backtick` span, typically naming a pattern rather than using it.
    InlineCode,
    /// A markdown table row. The dominant shape of pattern-library docs: one
    /// row per rule with its trigger text quoted as an example.
    Table,
    /// A `>` quoted block. Quoted untrusted content is still ingested, so this
    /// stays close to prose.
    BlockQuote,
    /// Inside an HTML comment. Deliberately NOT downgraded — hidden text is a
    /// delivery mechanism, not a disclaimer.
    HtmlComment,
    /// Inside an HTML element a browser would not show: a bare `hidden`
    /// attribute, or an inline style with `display:none`, `visibility:hidden`,
    /// `opacity:0`, `font-size:0`, or white-on-white / black-on-black text.
    ///
    /// The same rationale as `HtmlComment`, and the same score: a reader never
    /// sees the text, a model ingesting the page does. This used to be a
    /// pattern of its own (`PI017`, "hidden HTML styling"), reported at HIGH
    /// on the *element* regardless of what it wrapped. Five real pages scanned
    /// end to end — a telecom homepage, two encyclopedia/OWASP articles, a blog
    /// post, a project README — produced that finding on all five, every hit a
    /// collapsed menu, a share widget or a cookie banner. Hiding is what every
    /// page does; it is evidence only when it hides something a pattern
    /// recognises. So it is a context on the payload finding now, carrying the
    /// mechanism into the report (`[hidden html · confidence 1.0]`) without
    /// being a finding on its own.
    HiddenHtml,
    /// YAML/TOML frontmatter, detected lexically. Structured config an agent
    /// loads directly, but matched as raw text.
    Frontmatter,
    /// A finding from the **parsed** configuration tree (ENG-01, #32), not from
    /// raw text.
    ///
    /// Confidence 1.0, above lexical `Frontmatter`'s 0.9, and deliberately so:
    /// this finding exists because a real parser resolved a real key to a real
    /// value. There is no question of it being documentation *about* the key,
    /// which is the ambiguity the 0.9 discount on lexical frontmatter pays for.
    FrontmatterStructural,
}

impl MatchContext {
    /// How likely a match in this context is a real attack rather than a
    /// mention of one, from 0.0 to 1.0.
    ///
    /// The split that matters is above or below [`DEFAULT_MIN_CONFIDENCE`].
    /// `Table`, `InlineCode` and `FencedCode` fall below it; everything else
    /// stays above, including `HtmlComment`, which is where injections hide.
    pub fn confidence(self) -> f32 {
        match self {
            MatchContext::Prose => 1.0,
            MatchContext::HtmlComment => 1.0,
            MatchContext::HiddenHtml => 1.0,
            MatchContext::Frontmatter => 0.9,
            MatchContext::FrontmatterStructural => 1.0,
            MatchContext::BlockQuote => 0.9,
            MatchContext::Table => 0.3,
            MatchContext::InlineCode => 0.3,
            MatchContext::FencedCode => 0.2,
        }
    }

    /// Human-readable name, used in text output and error messages.
    pub fn label(self) -> &'static str {
        match self {
            MatchContext::Prose => "prose",
            MatchContext::FencedCode => "fenced code",
            MatchContext::InlineCode => "inline code",
            MatchContext::Table => "table",
            MatchContext::BlockQuote => "block quote",
            MatchContext::HtmlComment => "html comment",
            MatchContext::HiddenHtml => "hidden html",
            MatchContext::Frontmatter => "frontmatter",
            MatchContext::FrontmatterStructural => "frontmatter (structural)",
        }
    }
}

/// Findings at or above this score are reported by default.
///
/// Chosen to sit between `Table`/`InlineCode` (0.3) and `BlockQuote` (0.9): the
/// contexts that generate documentation noise fall below, and every context
/// where an agent would actually act on the text stays above.
pub const DEFAULT_MIN_CONFIDENCE: f32 = 0.5;

/// Classifies every line of a document, tracking state that spans lines.
///
/// Line-based rather than a full markdown parse. The scanner already works line
/// by line, a real parser would be a much larger dependency, and the cases that
/// matter — fences, frontmatter, HTML comments — are all line-oriented. Inline
/// spans are the exception and are resolved per byte offset.
pub struct ContextMap {
    lines: Vec<LineKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineKind {
    Prose,
    FencedCode,
    Table,
    BlockQuote,
    HtmlComment,
    /// A line entirely inside a hidden element opened on an earlier line. An
    /// element opened and closed on one line is resolved per byte offset
    /// instead, like inline code.
    HiddenHtml,
    Frontmatter,
}

impl ContextMap {
    /// Walk the document once, recording the context of each line.
    pub fn build(content: &str) -> Self {
        let mut lines = Vec::new();

        // Fences are tracked by their marker and length: a ``` inside a ~~~~
        // block is content, not a close, and a longer run cannot be closed by a
        // shorter one. Sample output that itself contains fences depends on this.
        let mut fence: Option<(char, usize)> = None;
        let mut in_html_comment = false;
        let mut frontmatter = FrontmatterState::Unstarted;
        // A hidden element still open at the end of a line: its tag name and
        // how deeply that tag is nested, so `<div hidden><div>…</div></div>`
        // closes on the right `</div>`. Counting only the opener's own tag
        // name keeps the tracking honest on real markup, where the hidden
        // block contains lists, links and paragraphs of its own.
        let mut hidden_block: Option<(String, usize)> = None;

        for (index, raw) in content.lines().enumerate() {
            let line = raw.trim_start();

            // Frontmatter only counts at the very top of the file; a `---`
            // later on is a horizontal rule.
            frontmatter = frontmatter.advance(index, line);
            if frontmatter == FrontmatterState::Inside {
                lines.push(LineKind::Frontmatter);
                continue;
            }

            if let Some((marker, width)) = fence {
                if closes_fence(line, marker, width) {
                    fence = None;
                }
                // The closing fence itself is still code, not prose.
                lines.push(LineKind::FencedCode);
                continue;
            }

            if let Some(opened) = opens_fence(line) {
                fence = Some(opened);
                lines.push(LineKind::FencedCode);
                continue;
            }

            let opened_comment = in_html_comment;
            if !in_html_comment && line.contains("<!--") && !line.contains("-->") {
                in_html_comment = true;
            } else if in_html_comment && line.contains("-->") {
                in_html_comment = false;
            }
            if opened_comment || in_html_comment || is_single_line_comment(line) {
                lines.push(LineKind::HtmlComment);
                continue;
            }

            if let Some((tag, depth)) = hidden_block.take() {
                // Every line of an open hidden block is hidden, including the
                // one that closes it — the same rule fences and comments use.
                let remaining = nesting_after(line, 0, &tag, depth);
                if remaining > 0 {
                    hidden_block = Some((tag, remaining));
                }
                lines.push(LineKind::HiddenHtml);
                continue;
            }

            // A hidden element that opens here and does not close here hides
            // the lines that follow. This line itself is classified per offset
            // by `context_at`, because the text before the opener is visible.
            //
            // Every opener on the line is checked, not only the first (review
            // on #110): a collapsed widget closing and a cookie banner opening
            // on the same line is the realistic shape, and the first opener
            // closing must not hide the second one staying open. The FIRST
            // opener still open at the end of the line is the one tracked,
            // because a later one still open is nested inside it and closes
            // before it does.
            for opener in hidden_openers(line) {
                if hidden_block.is_some() {
                    break;
                }
                let remaining = nesting_after(line, opener.end, &opener.tag, 1);
                if remaining > 0 {
                    hidden_block = Some((opener.tag, remaining));
                }
            }

            if line.starts_with('>') {
                lines.push(LineKind::BlockQuote);
                continue;
            }

            if is_table_row(line) {
                lines.push(LineKind::Table);
                continue;
            }

            lines.push(LineKind::Prose);
        }

        Self { lines }
    }

    /// Context of a match, given its 1-based line and byte offset within that line.
    ///
    /// `line_content` is passed rather than stored: the scanner already holds it,
    /// and inline-span detection needs the text, not just the classification.
    pub fn context_at(
        &self,
        line_number: usize,
        line_content: &str,
        offset: usize,
    ) -> MatchContext {
        let kind = self
            .lines
            .get(line_number.saturating_sub(1))
            .copied()
            .unwrap_or(LineKind::Prose);

        match kind {
            LineKind::FencedCode => MatchContext::FencedCode,
            LineKind::Frontmatter => MatchContext::Frontmatter,
            LineKind::HtmlComment => MatchContext::HtmlComment,
            LineKind::HiddenHtml => MatchContext::HiddenHtml,
            LineKind::BlockQuote => MatchContext::BlockQuote,
            LineKind::Table => MatchContext::Table,
            // Only prose lines are checked for inline spans; a backtick inside a
            // table cell is already covered by the weaker Table score.
            //
            // Hidden markup is checked before inline code. Both score as
            // "real" or "quoted" respectively, and a payload an attacker wrapped
            // in `<span hidden>` is the former even if they also put backticks
            // around it.
            LineKind::Prose => {
                if in_hidden_element(line_content, offset) {
                    MatchContext::HiddenHtml
                } else if in_inline_code(line_content, offset) {
                    MatchContext::InlineCode
                } else {
                    MatchContext::Prose
                }
            }
        }
    }
}

/// Tracks the `---` delimiters that can only mean frontmatter at the file head.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FrontmatterState {
    Unstarted,
    Inside,
    Closed,
}

impl FrontmatterState {
    fn advance(self, index: usize, line: &str) -> Self {
        match self {
            // Must be the literal first line.
            FrontmatterState::Unstarted if index == 0 && (line == "---" || line == "+++") => {
                FrontmatterState::Inside
            }
            FrontmatterState::Unstarted => FrontmatterState::Closed,
            FrontmatterState::Inside if line == "---" || line == "+++" => FrontmatterState::Closed,
            other => other,
        }
    }
}

/// The marker character and run length if this line opens a fence.
fn opens_fence(line: &str) -> Option<(char, usize)> {
    for marker in ['`', '~'] {
        let run = line.chars().take_while(|c| *c == marker).count();
        if run >= 3 {
            return Some((marker, run));
        }
    }
    None
}

/// A fence closes only on the same marker, at least as long, with nothing after it.
fn closes_fence(line: &str, marker: char, width: usize) -> bool {
    let run = line.chars().take_while(|c| *c == marker).count();
    run >= width && line[run..].trim().is_empty()
}

fn is_single_line_comment(line: &str) -> bool {
    line.starts_with("<!--") && line.contains("-->")
}

/// A pipe-delimited row, or the `|---|---|` separator beneath a header.
fn is_table_row(line: &str) -> bool {
    if !line.starts_with('|') {
        return false;
    }
    // A lone `|` is not a table.
    line.matches('|').count() >= 2
}

/// An opening tag on one line whose attributes hide the element.
#[derive(Debug, Clone, PartialEq, Eq)]
struct HiddenOpener {
    /// Byte offset of the `<`.
    start: usize,
    /// Byte offset just past the `>`.
    end: usize,
    /// Lower-cased tag name.
    tag: String,
}

/// Elements that never have content, so a `hidden` on them hides nothing that
/// a pattern could match. Tracking them as block openers would leave a
/// `<input hidden>` "open" until the end of the document.
const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// Every hidden opener on the line, in document order.
///
/// Hand-parsed rather than a regex: the question is about *attributes*, and
/// `aria-hidden="true"` or `class="menu--hidden"` must not count. The old
/// PI017 regex needed a terminator trick for exactly that, because the regex
/// crate has no lookaround; reading the attribute list directly is both
/// simpler and correct.
fn hidden_openers(line: &str) -> Vec<HiddenOpener> {
    let bytes = line.as_bytes();
    let mut found = Vec::new();
    let mut index = 0usize;

    while let Some(rel) = line[index..].find('<') {
        let start = index + rel;
        let name_start = start + 1;
        let name_end = name_start
            + line[name_start..]
                .bytes()
                .take_while(|b| b.is_ascii_alphanumeric())
                .count();
        if name_end == name_start {
            index = name_start;
            continue;
        }
        let Some(close_rel) = line[name_end..].find('>') else {
            break;
        };
        let end = name_end + close_rel + 1;
        let tag = line[name_start..name_end].to_ascii_lowercase();
        let attributes = &line[name_end..end - 1];

        if !VOID_ELEMENTS.contains(&tag.as_str()) && attributes_hide(attributes) {
            found.push(HiddenOpener { start, end, tag });
        }
        index = end;
        if index >= bytes.len() {
            break;
        }
    }

    found
}

/// Whether an attribute list hides its element from a reader.
fn attributes_hide(attributes: &str) -> bool {
    if has_bare_hidden_attribute(attributes) {
        return true;
    }
    match style_attribute(attributes) {
        Some(style) => style_hides(&style),
        None => false,
    }
}

/// A `hidden` attribute of its own — not the suffix of `aria-hidden` or
/// `data-hidden`, and not a class name.
fn has_bare_hidden_attribute(attributes: &str) -> bool {
    let lower = attributes.to_ascii_lowercase();
    let mut search = 0usize;
    while let Some(rel) = lower[search..].find("hidden") {
        let at = search + rel;
        let before = lower[..at].chars().next_back();
        let after = lower[at + "hidden".len()..].chars().next();
        // Spelled out rather than `Option::is_none_or`, which needs Rust 1.82
        // and the crate pins no MSRV (review on #110).
        let starts_attribute = match before {
            None => true,
            Some(c) => c.is_whitespace(),
        };
        let ends_attribute = match after {
            None => true,
            Some(c) => c.is_whitespace() || c == '=' || c == '/',
        };
        if starts_attribute && ends_attribute {
            return true;
        }
        search = at + "hidden".len();
    }
    false
}

/// The value of a `style="…"` / `style='…'` attribute, lower-cased with
/// whitespace removed so `display : none` and `display:none` compare equal.
fn style_attribute(attributes: &str) -> Option<String> {
    let lower = attributes.to_ascii_lowercase();
    let at = lower.find("style")?;
    let rest = lower[at + "style".len()..].trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let body = &rest[1..];
    let end = body.find(quote).unwrap_or(body.len());
    Some(body[..end].chars().filter(|c| !c.is_whitespace()).collect())
}

/// The inline-style mechanisms that hide text from a reader.
///
/// `font-size:0` needs a terminator: `font-size:0.8rem` is the most common
/// inline style on the web and must not match on its leading zero. The colour
/// arms need the same, or `#fff000` matches on its `#fff` prefix.
fn style_hides(style: &str) -> bool {
    let declarations = style.split(';');
    for declaration in declarations {
        let Some((property, value)) = declaration.split_once(':') else {
            continue;
        };
        let hides = match property {
            "display" => value == "none",
            "visibility" => value == "hidden",
            "opacity" => value == "0",
            "font-size" => {
                matches!(value, "0" | "0px" | "0pt" | "0em" | "0rem" | "0%")
            }
            "color" => matches!(value, "#fff" | "#ffffff" | "#000" | "#000000"),
            _ => false,
        };
        if hides {
            return true;
        }
    }
    false
}

/// Nesting depth of `tag` after scanning `line[from..]`, starting at `depth`.
///
/// Counts `<tag` openers and `</tag` closers of that one name. Zero means the
/// hidden element closed on this line.
fn nesting_after(line: &str, from: usize, tag: &str, depth: usize) -> usize {
    let lower = line[from..].to_ascii_lowercase();
    let mut depth = depth;
    let mut index = 0usize;
    while let Some(rel) = lower[index..].find('<') {
        let at = index + rel;
        let rest = &lower[at + 1..];
        if let Some(after) = rest.strip_prefix('/') {
            if tag_name_at(after) == tag {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return 0;
                }
            }
        } else if tag_name_at(rest) == tag {
            depth += 1;
        }
        index = at + 1;
    }
    depth
}

/// The tag name starting at the head of `text`, or "" if there is none.
fn tag_name_at(text: &str) -> &str {
    let len = text
        .bytes()
        .take_while(|b| b.is_ascii_alphanumeric())
        .count();
    &text[..len]
}

/// Whether `offset` falls inside a hidden element opened earlier on this line
/// and not yet closed by that point.
fn in_hidden_element(line: &str, offset: usize) -> bool {
    hidden_openers(line)
        .into_iter()
        .filter(|opener| opener.end <= offset)
        .any(|opener| nesting_after(&line[..offset], opener.end, &opener.tag, 1) > 0)
}

/// Whether `offset` falls inside a backtick span on this line.
///
/// Counts backtick runs before the offset: an odd number of openers means the
/// position sits inside a span. Handles ``code with ` inside`` by treating a run
/// of N backticks as a single delimiter.
fn in_inline_code(line: &str, offset: usize) -> bool {
    let bytes = line.as_bytes();
    let mut index = 0usize;
    let mut open: Option<usize> = None;

    while index < bytes.len() && index < offset {
        if bytes[index] != b'`' {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && bytes[index] == b'`' {
            index += 1;
        }
        let run = index - start;
        match open {
            Some(width) if width == run => open = None,
            Some(_) => {}
            None => open = Some(run),
        }
    }

    open.is_some()
}
