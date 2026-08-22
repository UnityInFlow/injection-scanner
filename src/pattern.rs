use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Severity level for a scan finding.
///
/// Ordered from least to most severe. Used both as category defaults
/// and per-pattern overrides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// A single pattern definition loaded from YAML or embedded at compile time.
///
/// The `severity` field is optional — when absent, the parent category's
/// `default_severity` applies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pattern {
    pub id: String,
    pub name: String,
    pub pattern: String,
    #[serde(default)]
    pub severity: Option<Severity>,
    /// Whether this pattern must match the exact casing written in `pattern`.
    ///
    /// Defaults to `false` — patterns are **case-insensitive** unless they opt
    /// out. This is deliberate: prompt-injection payloads are natural language,
    /// and an attacker capitalising a sentence must not defeat detection.
    /// Set `case_sensitive: true` only where casing itself carries the signal.
    #[serde(default)]
    pub case_sensitive: Option<bool>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub remediation: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// A category grouping related patterns with a shared default severity.
///
/// Maps directly to a YAML pattern file (e.g., `role-override.yaml`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatternCategory {
    pub category: String,
    pub default_severity: Severity,
    pub patterns: Vec<Pattern>,
}

/// A single match found during scanning.
#[derive(Debug, Clone, Serialize)]
pub struct ScanMatch {
    pub pattern_id: String,
    pub pattern_name: String,
    pub severity: Severity,
    pub message: String,
    pub remediation: String,
    pub file: String,
    pub line: usize,
    pub matched_text: String,
}

/// Aggregated scan results for a single file.
#[derive(Debug, Clone, Serialize)]
pub struct ScanReport {
    pub file: String,
    pub matches: Vec<ScanMatch>,
    /// Findings withheld by a suppression directive in the scanned file.
    ///
    /// Recorded rather than discarded: suppression is a control the *scanned
    /// document* invokes, so silent suppression would let an attacker disarm the
    /// scanner without leaving a trace.
    ///
    /// Deliberately the **same** `ScanMatch` the `matches` array holds. These are
    /// not lesser findings — they are findings, filed under the reason they are
    /// not being shown. `--no-suppress` moves a record from here to `matches`
    /// unchanged, and a consumer needs the message, the pattern name and the
    /// matched text to judge whether a suppression was legitimate. A thinner
    /// record saved nothing: every field is in scope where it is built.
    ///
    /// Additive field — the top-level JSON stays an array of report objects, so
    /// `spec-ci-plugin`'s `JSON.parse(output) as Array<...>` is unaffected.
    #[serde(default)]
    pub suppressed: Vec<ScanMatch>,
    /// Severity tallies over `matches` **only** — suppressed findings are not
    /// counted here.
    ///
    /// These answer "what is the user being asked to act on", which is what
    /// drives exit codes and CI gates; a suppressed finding is by definition
    /// not that. So `critical_count` can be 0 for a file that suppressed a
    /// CRITICAL, and a consumer wanting the full picture must read `suppressed`
    /// as well — `spec-ci-plugin` printed "No injection patterns detected" for
    /// exactly that file before it did.
    ///
    /// Under `--no-suppress` nothing is suppressed, so the tallies cover
    /// everything found.
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
}

impl ScanReport {
    /// Create a new report, automatically computing severity counts.
    pub fn new(file: String, matches: Vec<ScanMatch>) -> Self {
        Self::with_suppressed(file, matches, Vec::new())
    }

    /// Create a report that also records what suppression withheld.
    pub fn with_suppressed(
        file: String,
        matches: Vec<ScanMatch>,
        suppressed: Vec<ScanMatch>,
    ) -> Self {
        let critical_count = matches
            .iter()
            .filter(|m| m.severity == Severity::Critical)
            .count();
        let high_count = matches
            .iter()
            .filter(|m| m.severity == Severity::High)
            .count();
        let medium_count = matches
            .iter()
            .filter(|m| m.severity == Severity::Medium)
            .count();
        let low_count = matches
            .iter()
            .filter(|m| m.severity == Severity::Low)
            .count();
        Self {
            file,
            matches,
            suppressed,
            critical_count,
            high_count,
            medium_count,
            low_count,
        }
    }

    /// Returns `true` if any findings were detected.
    pub fn has_findings(&self) -> bool {
        !self.matches.is_empty()
    }

    /// How many findings this file's own directives withheld.
    pub fn suppressed_count(&self) -> usize {
        self.suppressed.len()
    }
}

/// Errors that can occur when loading or compiling patterns.
#[derive(Debug, Error)]
pub enum PatternError {
    #[error("Failed to parse pattern file: {0}")]
    ParseError(String),
    #[error("Invalid regex pattern '{pattern}' in {id}: {source}")]
    InvalidRegex {
        id: String,
        pattern: String,
        source: regex::Error,
    },
    #[error(
        "Duplicate pattern id '{id}': defined in category '{first_category}' and again in \
         '{second_category}'. Two patterns sharing an id produce contradictory findings for the \
         same id, which breaks any consumer that keys on pattern_id."
    )]
    DuplicateId {
        id: String,
        first_category: String,
        second_category: String,
    },
}
