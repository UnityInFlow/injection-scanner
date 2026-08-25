use serde::{Deserialize, Serialize};

use crate::context::MatchContext;
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
    /// Whether this pattern may run **only** against the raw source text,
    /// skipping the Unicode-normalized pass (#26).
    ///
    /// Defaults to `false` — a pattern runs on both passes, so an attacker
    /// cannot defeat it by swapping in confusable or zero-width characters.
    ///
    /// Set `raw_only: true` **only** for a detector whose signal is the raw
    /// bytes themselves, such as a mixed-script homoglyph detector. The
    /// normalizer folds confusables back to Latin, which leaves a
    /// Latin/non-Latin mash that is not a mixed-script token in the source —
    /// so running such a detector on the normalized text flags every bilingual
    /// document. For every other pattern this field weakens detection: it
    /// turns off the obfuscation-resistance that the normalized pass exists to
    /// provide. It is deliberately a schema field rather than a tag so that
    /// `deny_unknown_fields` catches typos and the choice is visible in review.
    #[serde(default)]
    pub raw_only: Option<bool>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMatch {
    pub pattern_id: String,
    pub pattern_name: String,
    pub severity: Severity,
    pub message: String,
    pub remediation: String,
    pub file: String,
    pub line: usize,
    pub matched_text: String,
    /// Where in the document this was found (issue #20).
    ///
    /// Additive with a default, so a consumer reading reports from a release
    /// that predates it is unaffected.
    #[serde(default = "default_context")]
    pub context: MatchContext,
    /// How likely this is a real attack rather than documentation of one, 0.0-1.0.
    ///
    /// Derived from `context`. Carried on the record rather than recomputed so a
    /// consumer filtering reports does not need to know the scoring table.
    #[serde(default = "default_confidence")]
    pub confidence: f32,
}

/// Reports written before `context` existed came from a scanner with no notion
/// of structure, which is exactly the prose assumption.
fn default_context() -> MatchContext {
    MatchContext::Prose
}

fn default_confidence() -> f32 {
    MatchContext::Prose.confidence()
}

/// Aggregated scan results for a single file.
///
/// Round-trips: `--format json` output deserializes back into this type. That
/// is what makes `#[serde(default)]` on `suppressed` mean anything — without
/// `Deserialize` the attribute was inert, and the backward-compatibility it
/// documents could not actually happen.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Findings withheld because their markdown context scored below the
    /// confidence threshold — a payload inside a fenced code block, an inline
    /// span, or a table cell.
    ///
    /// A **separate** array from `suppressed`, deliberately. `suppressed` means
    /// "a directive in this document disarmed the scanner", which is itself a
    /// signal worth raising; this means "the scanner judged it documentation".
    /// Merging them would let a benign README example masquerade as evidence of
    /// tampering.
    ///
    /// Recorded rather than discarded, for the same reason `suppressed` is.
    /// Markdown context is a guess about how a document will be *consumed*, and
    /// it can be wrong: a fenced block is inert in a rendered README but arrives
    /// as bare text when a page is flattened into an agent's context, which is
    /// the exact delivery path this tool exists to guard. Dropping these outright
    /// would hand an attacker a one-line bypass — wrap the payload in backticks
    /// and the scanner goes silent with no trace. `--strict` (or
    /// `--min-confidence 0`) moves these into `matches` unchanged.
    ///
    /// Additive field — the top-level JSON stays an array of report objects, so
    /// `spec-ci-plugin`'s `JSON.parse(output) as Array<...>` is unaffected.
    #[serde(default)]
    pub low_confidence: Vec<ScanMatch>,
    /// Findings withheld because a `--baseline <FILE>` accepted them in a
    /// prior run.
    ///
    /// Deliberately the **same** `ScanMatch` the `matches` array holds, filed
    /// under a THIRD distinct reason a finding can be withheld: `suppressed`
    /// means the scanned document disarmed the scanner, `low_confidence`
    /// means the scanner judged it documentation, and `baselined` means a
    /// human accepted it once and recorded that decision in a committed
    /// baseline file. Recorded rather than discarded, for the same reason
    /// the other two are: a decision worth acting on is a decision worth
    /// being able to see.
    ///
    /// Additive field — the top-level JSON stays an array of report objects,
    /// so `spec-ci-plugin`'s `JSON.parse(output) as Array<...>` is unaffected.
    #[serde(default)]
    pub baselined: Vec<ScanMatch>,
    /// Severity tallies over `matches` **only** — suppressed findings are not
    /// counted here, neither are `low_confidence` ones, nor are `baselined`
    /// ones.
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
        Self::with_withheld(file, matches, suppressed, Vec::new())
    }

    /// Create a report recording both reasons a finding can be withheld.
    pub fn with_withheld(
        file: String,
        matches: Vec<ScanMatch>,
        suppressed: Vec<ScanMatch>,
        low_confidence: Vec<ScanMatch>,
    ) -> Self {
        Self::with_baselined(file, matches, suppressed, low_confidence, Vec::new())
    }

    /// Create a report recording all three reasons a finding can be
    /// withheld, including baseline acceptance.
    ///
    /// The only constructor that recomputes the severity tallies — a
    /// baselined finding must move through here, not be spliced into an
    /// already-built report, because these tallies are what drive the exit
    /// code and CI gates.
    pub fn with_baselined(
        file: String,
        matches: Vec<ScanMatch>,
        suppressed: Vec<ScanMatch>,
        low_confidence: Vec<ScanMatch>,
        baselined: Vec<ScanMatch>,
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
            low_confidence,
            baselined,
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

    /// How many findings the markdown-context threshold withheld.
    pub fn low_confidence_count(&self) -> usize {
        self.low_confidence.len()
    }

    /// How many findings a `--baseline` file withheld.
    pub fn baselined_count(&self) -> usize {
        self.baselined.len()
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
