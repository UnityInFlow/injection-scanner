use std::collections::HashMap;

use regex::{Regex, RegexBuilder};

use crate::allowlist::Suppressions;
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
}

impl CompiledPattern {
    /// Build the finding this pattern reports for `matched_text`.
    ///
    /// The single construction site for `ScanMatch`. Suppression decides which
    /// array a finding is filed into, never what the finding says, so having
    /// one arm of `scan` able to drift from the other is a bug waiting to
    /// happen — a new field added to `ScanMatch` would be populated in one
    /// place and defaulted in the other.
    fn record(&self, file_path: &str, line_number: usize, matched_text: &str) -> ScanMatch {
        ScanMatch {
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
        let mut matches = Vec::new();
        let mut suppressed = Vec::new();

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
                let destination = if suppressions.is_suppressed(line_number, &cp.id) {
                    // Recorded rather than dropped: a document that disarms the
                    // scanner should leave a trace. See `ScanReport::suppressed`.
                    &mut suppressed
                } else {
                    &mut matches
                };

                // Every occurrence, not just the first: a line packing three
                // payloads previously yielded one finding, and `matched_text`
                // under-reported what was actually there.
                for matched in cp
                    .regex
                    .find_iter(line)
                    .take(MAX_MATCHES_PER_PATTERN_PER_LINE)
                {
                    destination.push(cp.record(file_path, line_number, matched.as_str()));
                }
            }
        }

        ScanReport::with_suppressed(file_path.to_string(), matches, suppressed)
    }
}
