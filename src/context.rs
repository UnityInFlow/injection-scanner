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
            LineKind::BlockQuote => MatchContext::BlockQuote,
            LineKind::Table => MatchContext::Table,
            // Only prose lines are checked for inline spans; a backtick inside a
            // table cell is already covered by the weaker Table score.
            LineKind::Prose => {
                if in_inline_code(line_content, offset) {
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
