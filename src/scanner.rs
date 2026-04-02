use std::collections::HashMap;

use regex::Regex;

use crate::allowlist::is_suppressed;
use crate::pattern::{PatternCategory, ScanMatch, ScanReport, Severity};

/// A pattern with its regex pre-compiled for efficient scanning.
struct CompiledPattern {
    id: String,
    name: String,
    severity: Severity,
    description: String,
    remediation: String,
    regex: Regex,
}

/// Compile all patterns from all categories into ready-to-match regexes.
///
/// Invalid regexes are logged to stderr and skipped rather than
/// failing the entire scan.
fn compile_patterns(categories: &[PatternCategory]) -> Vec<CompiledPattern> {
    let mut compiled = Vec::new();
    for category in categories {
        for pattern in &category.patterns {
            let severity = pattern.severity.unwrap_or(category.default_severity);
            match Regex::new(&pattern.pattern) {
                Ok(regex) => compiled.push(CompiledPattern {
                    id: pattern.id.clone(),
                    name: pattern.name.clone(),
                    severity,
                    description: pattern.description.clone(),
                    remediation: pattern.remediation.clone(),
                    regex,
                }),
                Err(e) => eprintln!("Warning: invalid regex in {}: {}", pattern.id, e),
            }
        }
    }
    compiled
}

/// Scan content line-by-line against all pattern categories.
///
/// Regexes are compiled once before the scan loop (not per-line).
/// Per-line suppressions are checked via `is_suppressed()`.
pub fn scan_content(
    file_path: &str,
    content: &str,
    categories: &[PatternCategory],
    suppressions: &HashMap<usize, Vec<String>>,
) -> ScanReport {
    let compiled = compile_patterns(categories);
    let mut matches = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line_number = line_num + 1;

        for cp in &compiled {
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
