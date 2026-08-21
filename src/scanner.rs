use std::collections::HashMap;

use regex::{Regex, RegexBuilder};

use crate::allowlist::is_suppressed;
use crate::pattern::{PatternCategory, PatternError, ScanMatch, ScanReport, Severity};

/// A pattern with its regex pre-compiled for efficient scanning.
struct CompiledPattern {
    id: String,
    name: String,
    severity: Severity,
    description: String,
    remediation: String,
    regex: Regex,
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
    /// Compile every pattern in every category.
    ///
    /// Returns `Err` on the first pattern whose regex fails to compile. This is
    /// deliberate: a security scanner that silently drops an unparsable pattern
    /// reduces its own coverage while still exiting green.
    pub fn new(categories: &[PatternCategory]) -> Result<Self, PatternError> {
        let mut compiled = Vec::new();

        for category in categories {
            for pattern in &category.patterns {
                let severity = pattern.severity.unwrap_or(category.default_severity);

                // Case-insensitive by default. Patterns opt out explicitly; see
                // `Pattern::case_sensitive` for why the default runs this way.
                let case_sensitive = pattern.case_sensitive.unwrap_or(false);

                let regex = RegexBuilder::new(&pattern.pattern)
                    .case_insensitive(!case_sensitive)
                    .build()
                    .map_err(|source| PatternError::InvalidRegex {
                        id: pattern.id.clone(),
                        pattern: pattern.pattern.clone(),
                        source,
                    })?;

                compiled.push(CompiledPattern {
                    id: pattern.id.clone(),
                    name: pattern.name.clone(),
                    severity,
                    description: pattern.description.clone(),
                    remediation: pattern.remediation.clone(),
                    regex,
                });
            }
        }

        Ok(Self { compiled })
    }

    /// Number of patterns successfully compiled into this scanner.
    pub fn pattern_count(&self) -> usize {
        self.compiled.len()
    }

    /// Scan content line by line against the compiled pattern set.
    ///
    /// Per-line suppressions are honoured via [`is_suppressed`].
    pub fn scan(
        &self,
        file_path: &str,
        content: &str,
        suppressions: &HashMap<usize, Vec<String>>,
    ) -> ScanReport {
        let mut matches = Vec::new();

        for (line_index, line) in content.lines().enumerate() {
            let line_number = line_index + 1;

            for cp in &self.compiled {
                if is_suppressed(suppressions, line_number, &cp.id) {
                    continue;
                }

                if let Some(matched) = cp.regex.find(line) {
                    matches.push(ScanMatch {
                        pattern_id: cp.id.clone(),
                        pattern_name: cp.name.clone(),
                        severity: cp.severity,
                        message: cp.description.clone(),
                        remediation: cp.remediation.clone(),
                        file: file_path.to_string(),
                        line: line_number,
                        matched_text: matched.as_str().to_string(),
                    });
                }
            }
        }

        ScanReport::new(file_path.to_string(), matches)
    }
}
