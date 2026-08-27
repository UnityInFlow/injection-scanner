use std::collections::HashMap;

use regex::{Regex, RegexBuilder};

use crate::allowlist::Suppressions;
use crate::context::{ContextMap, MatchContext, DEFAULT_MIN_CONFIDENCE};
use crate::multiline::joined_blocks;
use crate::normalize::{normalize, Normalized};
use crate::pattern::{PatternCategory, PatternError, ScanMatch, ScanReport, Severity};

/// Upper bound on reported matches per pattern per line.
///
/// `find_iter` replaced `find` so a line carrying several payloads reports each
/// one rather than only the first (audit C-05). The cap keeps a pathological
/// line — thousands of repetitions of a short payload — from flooding a report.
const MAX_MATCHES_PER_PATTERN_PER_LINE: usize = 10;

/// A pattern with its regex pre-compiled for efficient scanning.
struct CompiledPattern {
    id: String,
    name: String,
    severity: Severity,
    description: String,
    remediation: String,
    regex: Regex,
    /// When true, skip this pattern on the normalized (#26) pass.
    ///
    /// Homoglyph detectors look for mixed scripts in the *raw* bytes. The
    /// confusable skeleton leaves a Latin/non-Latin mash that is not a real
    /// mixed-script token in the source, so running them on normalized text
    /// flags every bilingual document.
    raw_only: bool,
}

impl CompiledPattern {
    /// Build the finding this pattern reports for `matched_text`.
    ///
    /// The single construction site for `ScanMatch`. Suppression decides which
    /// array a finding is filed into, never what the finding says, so having
    /// one arm of `scan` able to drift from the other is a bug waiting to
    /// happen — a new field added to `ScanMatch` would be populated in one
    /// place and defaulted in the other.
    fn record(
        &self,
        file_path: &str,
        line_number: usize,
        matched_text: &str,
        context: MatchContext,
    ) -> ScanMatch {
        ScanMatch {
            context,
            confidence: context.confidence(),
            pattern_id: self.id.clone(),
            pattern_name: self.name.clone(),
            severity: self.severity,
            message: self.description.clone(),
            remediation: self.remediation.clone(),
            file: file_path.to_string(),
            line: line_number,
            matched_text: matched_text.to_string(),
        }
    }
}

/// A ready-to-use scanner owning the compiled pattern set.
///
/// Construct **once** and reuse across every file. Compiling the pattern set is
/// far more expensive than matching against it: before this type existed the
/// patterns were recompiled per file, and a 500-file scan spent ~1.6ms per file
/// on compilation alone — 806ms total, against a 200ms budget — while a single
/// 20,000-line file scanned in 19ms. Cost scaled with file *count* rather than
/// content, which is precisely wrong for a pre-commit hook.
pub struct Scanner {
    compiled: Vec<CompiledPattern>,
}

impl Scanner {
    /// Compile every pattern, failing on the first that does not.
    ///
    /// Use this for the **embedded** pattern set. Those are compile-time
    /// constants covered by a CI test, so a failure here is a bug in this
    /// repository and should stop the run rather than silently shrink coverage.
    ///
    /// Do **not** use this for community `--patterns` directories: see
    /// [`Scanner::new_lenient`] for why.
    pub fn new(categories: &[PatternCategory]) -> Result<Self, PatternError> {
        let (scanner, errors) = Self::new_lenient(categories);
        match errors.into_iter().next() {
            Some(e) => Err(e),
            None => Ok(scanner),
        }
    }

    /// Compile every pattern, collecting failures instead of aborting.
    ///
    /// Use this for **external** `--patterns` directories. Those are an
    /// untrusted input surface: making one malformed YAML file abort every scan
    /// would hand a denial-of-service to anyone able to write into a shared
    /// patterns directory. Callers are expected to surface the returned errors
    /// loudly so a dropped pattern is never silent — the failure mode this
    /// guards against is a scanner quietly losing coverage while exiting green.
    ///
    /// `--strict-patterns` (issue #28) will let callers opt back into failing.
    pub fn new_lenient(categories: &[PatternCategory]) -> (Self, Vec<PatternError>) {
        let mut compiled = Vec::new();
        let mut errors = Vec::new();
        // id -> the category that first claimed it. A second claim is rejected:
        // two patterns sharing an id emit findings with the same `pattern_id`
        // but different severity, message and remediation, which corrupts the
        // output for any consumer keying on that id (audit M-01).
        let mut seen_ids: HashMap<&str, &str> = HashMap::new();

        for category in categories {
            for pattern in &category.patterns {
                if let Some(first) = seen_ids.get(pattern.id.as_str()) {
                    errors.push(PatternError::DuplicateId {
                        id: pattern.id.clone(),
                        first_category: (*first).to_string(),
                        second_category: category.category.clone(),
                    });
                    continue;
                }
                seen_ids.insert(&pattern.id, &category.category);

                let severity = pattern.severity.unwrap_or(category.default_severity);

                // Case-insensitive by default. Patterns opt out explicitly; see
                // `Pattern::case_sensitive` for why the default runs this way.
                let case_sensitive = pattern.case_sensitive.unwrap_or(false);

                match RegexBuilder::new(&pattern.pattern)
                    .case_insensitive(!case_sensitive)
                    .build()
                {
                    Ok(regex) => compiled.push(CompiledPattern {
                        id: pattern.id.clone(),
                        name: pattern.name.clone(),
                        severity,
                        description: pattern.description.clone(),
                        remediation: pattern.remediation.clone(),
                        regex,
                        raw_only: pattern.tags.iter().any(|t| t == "homoglyph"),
                    }),
                    Err(source) => errors.push(PatternError::InvalidRegex {
                        id: pattern.id.clone(),
                        pattern: pattern.pattern.clone(),
                        source,
                    }),
                }
            }
        }

        (Self { compiled }, errors)
    }

    /// Number of patterns successfully compiled into this scanner.
    pub fn pattern_count(&self) -> usize {
        self.compiled.len()
    }

    /// Scan content line by line against the compiled pattern set.
    ///
    /// Suppressed findings are recorded in the report rather than discarded, so a
    /// document that disarms the scanner leaves a visible trace.
    pub fn scan(&self, file_path: &str, content: &str, suppressions: &Suppressions) -> ScanReport {
        self.scan_with_confidence(file_path, content, suppressions, DEFAULT_MIN_CONFIDENCE)
    }

    /// Scan, reporting only findings scoring at or above `min_confidence`.
    ///
    /// Pass 0.0 to report everything regardless of markdown context — that is
    /// what `--strict` does.
    ///
    /// Findings below the threshold go to [`ScanReport::low_confidence`], not to
    /// `suppressed` and not to the floor. They are a *third* category: the two
    /// arrays answer different questions — "a directive in this document
    /// disarmed the scanner" versus "the scanner judged this documentation" —
    /// and merging them would let a benign README example masquerade as evidence
    /// of tampering.
    ///
    /// They are recorded rather than discarded because the context judgement is
    /// a guess about how the document will be *consumed*, and that guess can be
    /// wrong. A fenced block is inert in a rendered README, but a page flattened
    /// into an agent's context arrives without its fence markers — the exact
    /// delivery path this tool exists to guard. Discarding would make
    /// "wrap it in backticks" a silent, traceless bypass.
    pub fn scan_with_confidence(
        &self,
        file_path: &str,
        content: &str,
        suppressions: &Suppressions,
        min_confidence: f32,
    ) -> ScanReport {
        let contexts = ContextMap::build(content);
        let mut matches = Vec::new();
        let mut suppressed = Vec::new();
        let mut low_confidence = Vec::new();

        for (line_index, line) in content.lines().enumerate() {
            let line_number = line_index + 1;

            for cp in &self.compiled {
                // Suppression selects the destination; it does not change the
                // finding, and it does not change how findings are counted.
                //
                // Both arms run `find_iter` under the same cap. An earlier
                // version used `is_match` for the suppressed arm, which
                // undercounted: three payloads on one suppressed line reported
                // "1 suppressed" while the same line unsuppressed reports 3,
                // understating exactly the signal this record exists to raise.
                //
                // `find_iter` on a suppressed line does pay the full regex cost
                // rather than short-circuiting. That is the intended trade — the
                // information is being recorded rather than discarded, and the
                // cap bounds the worst case.
                let suppressed_here = suppressions.is_suppressed(line_number, &cp.id);

                // Every occurrence, not just the first: a line packing three
                // payloads previously yielded one finding, and `matched_text`
                // under-reported what was actually there.
                for matched in cp
                    .regex
                    .find_iter(line)
                    .take(MAX_MATCHES_PER_PATTERN_PER_LINE)
                {
                    let context = contexts.context_at(line_number, line, matched.start());

                    // Three destinations, and nothing is ever dropped. A
                    // suppression directive is the document's own act and takes
                    // precedence in the record, because "this file disarmed the
                    // scanner" is the louder signal; below-threshold context is
                    // the scanner's own judgement and is filed separately so it
                    // can be audited or overridden with `--strict`.
                    let destination = if suppressed_here {
                        &mut suppressed
                    } else if context.confidence() < min_confidence {
                        &mut low_confidence
                    } else {
                        &mut matches
                    };
                    destination.push(cp.record(file_path, line_number, matched.as_str(), context));
                }
            }
        }

        // Second pass: payloads split across a line break (#24). Runs after the
        // line pass because it only reports what that pass could not see — a
        // match whose span crosses a break. Everything else is already filed.
        let source_lines: Vec<&str> = content.lines().collect();
        for block in joined_blocks(content) {
            for cp in &self.compiled {
                for matched in cp
                    .regex
                    .find_iter(&block.text)
                    .take(MAX_MATCHES_PER_PATTERN_PER_LINE)
                {
                    let span = matched.range();
                    if !block.spans_a_break(&span) {
                        continue;
                    }
                    let (first, last) = block.line_span(&span);

                    // A suppression directive on ANY line the match touches
                    // silences it. The alternative — keying only on the first
                    // line — would let a payload evade an existing suppression
                    // by starting one line earlier, and suppression is meant to
                    // be a statement about the text, not about its offset.
                    let suppressed_here =
                        (first..=last).any(|line| suppressions.is_suppressed(line, &cp.id));

                    // Context is taken from where the match STARTS — the line
                    // and the offset within it. A payload that begins in prose
                    // and continues into a fence is prose: the attacker chose
                    // where to start it.
                    //
                    // The offset is load-bearing, not decoration. Inline-code
                    // and table detection are questions about position within a
                    // line, so passing a placeholder answers "prose" for
                    // everything — and a payload quoted inside backticks across
                    // a wrap gets reported as a live one. Our own
                    // ROADMAP-v0.1.0.md is exactly that shape.
                    let (start_line, offset) = block.line_and_offset(span.start);
                    let line_text = source_lines.get(start_line - 1).copied().unwrap_or("");
                    let context = contexts.context_at(start_line, line_text, offset);

                    let destination = if suppressed_here {
                        &mut suppressed
                    } else if context.confidence() < min_confidence {
                        &mut low_confidence
                    } else {
                        &mut matches
                    };
                    destination.push(cp.record(file_path, first, matched.as_str(), context));
                }
            }
        }

        // Third pass: obfuscated payloads (#26). Runs against normalized text
        // with every offset mapped back, so findings still name a real line and
        // quote text the user can find in their own file.
        //
        // Deduplication is by (pattern, line): if the raw or multi-line pass
        // already reported that pattern on that line, the normalized pass has
        // nothing to add. Obfuscation is a property of the text, not a second
        // finding about it.
        if let Some(normalized) = normalize(content) {
            let already: std::collections::HashSet<(String, usize)> = matches
                .iter()
                .chain(suppressed.iter())
                .chain(low_confidence.iter())
                .map(|m| (m.pattern_id.clone(), m.line))
                .collect();

            // Matched LINE BY LINE, not over the whole normalized document.
            // `\s` matches a newline, so scanning the joined text let a pattern
            // span line breaks — quietly making this a second multi-line pass,
            // but without the paragraph boundaries #24 established. It joined
            // straight across a blank list item that the multi-line pass
            // correctly refuses to cross.
            //
            // Newlines survive normalization one-for-one, so these lines
            // correspond to the source lines exactly.
            let mut normalized_line_start = 0usize;
            for (line_index, normalized_line) in normalized.text.lines().enumerate() {
                let line_start = normalized_line_start;
                normalized_line_start += normalized_line.len() + 1;

                for cp in &self.compiled {
                    if cp.raw_only {
                        continue;
                    }
                    for matched in cp
                        .regex
                        .find_iter(normalized_line)
                        .take(MAX_MATCHES_PER_PATTERN_PER_LINE)
                    {
                        let at = normalized.original_offset(line_start + matched.start());
                        let line_number = line_index + 1;
                        if already.contains(&(cp.id.clone(), line_number)) {
                            continue;
                        }

                        let line_text = source_lines.get(line_index).copied().unwrap_or("");
                        let offset = at
                            - content[..at.min(content.len())]
                                .rfind('\n')
                                .map_or(0, |n| n + 1);
                        let context = contexts.context_at(line_number, line_text, offset);

                        let destination = if suppressions.is_suppressed(line_number, &cp.id) {
                            &mut suppressed
                        } else if context.confidence() < min_confidence {
                            &mut low_confidence
                        } else {
                            &mut matches
                        };
                        // The ORIGINAL text is quoted, not the normalized form.
                        // A user told their file contains "ignore all previous
                        // instructions" when it visibly contains
                        // "ignore-all-previous-instructions" cannot act on that.
                        let quoted = original_slice(content, &normalized, line_start, &matched);
                        destination.push(cp.record(file_path, line_number, &quoted, context));
                    }
                }
            }
        }

        ScanReport::with_withheld(file_path.to_string(), matches, suppressed, low_confidence)
    }
}

/// The original text a normalized match came from.
///
/// Reporting the normalized form would tell the user their file contains
/// `ignore all previous instructions` when it visibly contains
/// `ignore-all-previous-instructions` — a quote they cannot find with a search,
/// about a file they are being asked to fix.
fn original_slice(
    content: &str,
    normalized: &Normalized,
    line_start: usize,
    matched: &regex::Match<'_>,
) -> String {
    let start = normalized.original_offset(line_start + matched.start());
    let end = normalized
        .original_offset(line_start + matched.end().saturating_sub(1))
        .saturating_add(1)
        .min(content.len());
    if start >= end {
        return matched.as_str().to_string();
    }
    // Snap to char boundaries; the map points at the start of the source char,
    // but a multi-byte char at the end would slice mid-sequence.
    let mut lo = start;
    while lo > 0 && !content.is_char_boundary(lo) {
        lo -= 1;
    }
    let mut hi = end;
    while hi < content.len() && !content.is_char_boundary(hi) {
        hi += 1;
    }
    content[lo..hi].to_string()
}
