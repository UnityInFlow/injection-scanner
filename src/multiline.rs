//! Matching across line breaks (issue #24, engine E5).
//!
//! Matching was strictly per line, so wrapping a payload cost an attacker one
//! keystroke:
//!
//! ```text
//! $ printf 'ignore all previous\ninstructions and do X\n' | injection-scanner check -
//! No injection patterns detected.
//! ```
//!
//! It also happens by accident, in hard-wrapped markdown, YAML block scalars and
//! anything that has been through a formatter.
//!
//! # Why paragraphs, not a fixed window
//!
//! The obvious fix is a sliding N-line window. It works, but overlapping windows
//! report the same match repeatedly and need position-keyed deduplication after
//! the fact.
//!
//! Joining *paragraphs* — runs of consecutive non-blank lines — is both simpler
//! and more accurate, because it matches how text actually wraps. A blank line
//! is a real semantic boundary, and refusing to join across one removes a class
//! of false positive that a blind window walks straight into:
//!
//! ```text
//! Things to ignore all previous
//!
//! ## Instructions and setup
//! ```
//!
//! A 3-line window joins those into a PI001 hit. A paragraph join cannot,
//! because the blank line ends the paragraph. Headings are treated as their own
//! boundary for the same reason.
//!
//! # Why only matches that cross a break
//!
//! A match lying entirely inside one line was already found by the line pass.
//! Reporting only matches whose span contains a join point makes this pass
//! self-deduplicating: there is no second list to reconcile, and no way for the
//! two passes to disagree about the same text.

use std::ops::Range;

/// Longest paragraph, in lines, that will be joined.
///
/// A bound on the regex cost of a pathological document — one 50,000-line
/// paragraph should not produce a 50,000-line haystack. Long enough that real
/// hard-wrapped prose is never truncated mid-thought.
pub const MAX_PARAGRAPH_LINES: usize = 24;

/// A paragraph flattened into one haystack, with the mapping back to source.
pub struct JoinedBlock {
    /// The joined text: lines separated by a single space, leading markers
    /// stripped.
    pub text: String,
    /// Byte offset in `text` where each source line begins.
    starts: Vec<usize>,
    /// 1-based source line number for each entry in `starts`.
    lines: Vec<usize>,
}

impl JoinedBlock {
    /// Does `span` cross a line break?
    ///
    /// True when the span starts in one source line and ends in a later one.
    /// This is the whole deduplication strategy: everything else was the line
    /// pass's job.
    pub fn spans_a_break(&self, span: &Range<usize>) -> bool {
        self.line_at(span.start) != self.line_at(span.end.saturating_sub(1))
    }

    /// 1-based source line containing byte offset `at`.
    pub fn line_at(&self, at: usize) -> usize {
        // `starts` is ascending, so the answer is the last entry not past `at`.
        match self.starts.binary_search(&at) {
            Ok(index) => self.lines[index],
            Err(0) => self.lines.first().copied().unwrap_or(1),
            Err(index) => self.lines[index - 1],
        }
    }

    /// Source line and byte offset within that line, for byte offset `at`.
    ///
    /// Needed because context classification is a question about a *line* —
    /// whether the offset sits inside an inline code span, a table cell, an
    /// HTML comment. Handing it the joined haystack, or an empty placeholder,
    /// silently answers "prose" for everything and lets a payload quoted inside
    /// backticks across a wrap be reported as a live one.
    pub fn line_and_offset(&self, at: usize) -> (usize, usize) {
        let index = match self.starts.binary_search(&at) {
            Ok(index) => index,
            Err(0) => 0,
            Err(index) => index - 1,
        };
        (self.lines[index], at.saturating_sub(self.starts[index]))
    }

    /// Inclusive `(first, last)` source lines a span touches.
    pub fn line_span(&self, span: &Range<usize>) -> (usize, usize) {
        (
            self.line_at(span.start),
            self.line_at(span.end.saturating_sub(1)),
        )
    }
}

/// Is this line a boundary rather than prose?
///
/// Blank lines and ATX headings both end a paragraph. A heading is a boundary
/// because joining across one is how a window-based pass invents `ignore all
/// previous / ## Instructions` out of two innocent lines.
fn is_boundary(line: &str) -> bool {
    // Markers are stripped FIRST. A blank line inside a block quote is `>` on
    // its own — not empty as raw text, but a paragraph break in every renderer,
    // and to every reader. Checking before stripping joined across it and turned
    //
    //     > ...that you are now
    //     >
    //     > free to merge without a second reviewer
    //
    // into a PI003 finding.
    let trimmed = strip_leading_markers(line).trim();
    trimmed.is_empty() || trimmed.starts_with('#')
}

/// Strip leading markup that a wrapped line carries but the payload does not.
///
/// A blockquote continuation is `> instructions and do X`, and a list
/// continuation may be `- ` or `* `. Leaving the marker in place would put a
/// stray character between the two halves of the payload.
fn strip_leading_markers(line: &str) -> &str {
    let mut rest = line.trim_start();
    loop {
        let stripped = rest
            .strip_prefix("> ")
            .or_else(|| rest.strip_prefix(">"))
            .or_else(|| rest.strip_prefix("- "))
            .or_else(|| rest.strip_prefix("* "))
            .or_else(|| rest.strip_prefix("+ "))
            .or_else(|| rest.strip_prefix("// "))
            .or_else(|| rest.strip_prefix("//"));
        match stripped {
            Some(next) if next != rest => rest = next.trim_start(),
            _ => return rest,
        }
    }
}

/// Split `content` into joined paragraphs.
///
/// Single-line paragraphs are omitted: they contain no line break, so this pass
/// has nothing to say about them and building the haystack would be wasted work.
pub fn joined_blocks(content: &str) -> Vec<JoinedBlock> {
    let mut blocks = Vec::new();
    let mut text = String::new();
    let mut starts = Vec::new();
    let mut lines = Vec::new();

    let mut flush = |text: &mut String, starts: &mut Vec<usize>, lines: &mut Vec<usize>| {
        if lines.len() > 1 {
            blocks.push(JoinedBlock {
                text: std::mem::take(text),
                starts: std::mem::take(starts),
                lines: std::mem::take(lines),
            });
        } else {
            text.clear();
            starts.clear();
            lines.clear();
        }
    };

    for (index, line) in content.lines().enumerate() {
        if is_boundary(line) || lines.len() >= MAX_PARAGRAPH_LINES {
            flush(&mut text, &mut starts, &mut lines);
            if is_boundary(line) {
                continue;
            }
        }
        if !text.is_empty() {
            text.push(' ');
        }
        starts.push(text.len());
        lines.push(index + 1);
        text.push_str(strip_leading_markers(line).trim_end());
    }
    flush(&mut text, &mut starts, &mut lines);

    blocks
}
