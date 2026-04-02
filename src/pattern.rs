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
pub struct Pattern {
    pub id: String,
    pub name: String,
    pub pattern: String,
    #[serde(default)]
    pub severity: Option<Severity>,
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
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
}

impl ScanReport {
    /// Create a new report, automatically computing severity counts.
    pub fn new(file: String, matches: Vec<ScanMatch>) -> Self {
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
}
