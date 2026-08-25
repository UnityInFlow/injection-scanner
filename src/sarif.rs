//! SARIF 2.1.0 output (CLI-04, issue #5).
//!
//! `--format sarif` was withheld from [`crate` `OutputFormat`] in `src/main.rs`
//! until a writer existed — see the doc comment on that enum. This is the
//! writer, plus the document model it serializes.
//!
//! Deliberately its own file rather than an addition to `src/reporter.rs`:
//! `format_json` in that file *is* the contract `spec-ci-plugin` parses, and
//! keeping SARIF out of it means "did this change the JSON contract?" is
//! answerable by looking at a file whose diff is empty.
//!
//! Task 1 wires the thinnest complete path — one result per finding, no rule
//! catalogue, no fingerprints. Task 2 expands the document to carry the full
//! SARIF contract (rules, `ruleIndex`, `partialFingerprints`, `properties`).

use serde::Serialize;

use crate::pattern::{ScanReport, Severity};

/// The official SARIF 2.1.0 schema URL. Never fetched — this is a string
/// literal that identifies the schema version, not a network dependency.
const SARIF_SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/Schemata/sarif-schema-2.1.0.json";

const SARIF_VERSION: &str = "2.1.0";

/// A SARIF 2.1.0 log — the top-level document `--format sarif` writes.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifDocument {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

/// One analysis run. This tool always emits exactly one.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

/// The analysis tool that produced the run.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifTool {
    pub driver: SarifDriver,
}

/// Identifies this scanner to a SARIF consumer.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    pub information_uri: String,
}

/// Free-text carried under `message`, `shortDescription`, `fullDescription`
/// and `help` — SARIF gives each of those the same `{ "text": ... }` shape.
#[derive(Debug, Serialize)]
pub struct SarifText {
    pub text: String,
}

/// One finding, translated from a [`crate::pattern::ScanMatch`].
///
/// Built ONLY from `ScanReport.matches` — see [`format_sarif`], which is the
/// one place D-1 is enforced.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifResult {
    pub rule_id: String,
    pub level: String,
    pub message: SarifText,
    pub locations: Vec<SarifLocation>,
}

/// Where a result was found.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifLocation {
    pub physical_location: SarifPhysicalLocation,
}

/// A file and a region within it.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifPhysicalLocation {
    pub artifact_location: SarifArtifactLocation,
    pub region: SarifRegion,
}

/// The file a result was found in, as a relative URI reference.
#[derive(Debug, Serialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
}

/// A single line within a file. SARIF `region.startLine` is 1-based, matching
/// `ScanMatch.line`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifRegion {
    pub start_line: usize,
}

/// Maps this tool's four-level severity onto SARIF's closed `level` set.
///
/// CRITICAL and HIGH collapse to `error` — SARIF has no fifth level, and
/// `error` is what most consumers treat as build-blocking. MEDIUM is
/// `warning`, LOW is `note`. Exhaustive: a new [`Severity`] variant will not
/// compile here until this match grows an arm for it.
fn level_for(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical => "error",
        Severity::High => "error",
        Severity::Medium => "warning",
        Severity::Low => "note",
    }
}

/// Builds a SARIF 2.1.0 document as pretty-printed JSON.
///
/// Reads `ScanReport.matches` and nothing else. This is the ONE place D-1 is
/// enforced: `suppressed`, `low_confidence` and `baselined` are never in scope
/// here, so a SARIF result means exactly what the exit code already acts on —
/// "here is a finding to act on" — and the three withheld arrays never
/// resurface as alerts.
///
/// Mirrors [`crate::reporter::format_json`]'s signature — `Result<String,
/// serde_json::Error>` rather than `anyhow`, for the same reason documented
/// there: callers that want to distinguish a serialization failure from
/// everything else can.
pub fn format_sarif(reports: &[ScanReport]) -> Result<String, serde_json::Error> {
    let results: Vec<SarifResult> = reports
        .iter()
        .flat_map(|report| report.matches.iter())
        .map(|m| SarifResult {
            rule_id: m.pattern_id.clone(),
            level: level_for(m.severity).to_string(),
            message: SarifText {
                text: m.message.clone(),
            },
            locations: vec![SarifLocation {
                physical_location: SarifPhysicalLocation {
                    artifact_location: SarifArtifactLocation {
                        uri: m.file.clone(),
                    },
                    region: SarifRegion { start_line: m.line },
                },
            }],
        })
        .collect();

    let document = SarifDocument {
        schema: SARIF_SCHEMA_URL.to_string(),
        version: SARIF_VERSION.to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "injection-scanner".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    information_uri: "https://github.com/UnityInFlow/injection-scanner".to_string(),
                },
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&document)
}
