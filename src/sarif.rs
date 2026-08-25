//! SARIF 2.1.0 output (CLI-04, issue #5).
//!
//! `--format sarif` was withheld from `OutputFormat` in `src/main.rs` until a
//! writer existed — see the doc comment on that enum. This is the writer,
//! plus the document model it serializes.
//!
//! Deliberately its own file rather than an addition to `src/reporter.rs`:
//! `format_json` in that file *is* the contract `spec-ci-plugin` parses, and
//! keeping SARIF out of it means "did this change the JSON contract?" is
//! answerable by looking at a file whose diff is empty.
//!
//! ## D-1 — only `matches` become results
//!
//! [`format_sarif`] reads `ScanReport.matches` and nothing else. `suppressed`,
//! `low_confidence` and `baselined` are never in scope here — a SARIF result
//! means exactly what the exit code already acts on, so those three withheld
//! arrays can never resurface as a code-scanning alert.
//!
//! ## Severity
//!
//! CRITICAL/HIGH collapse to `error`, MEDIUM to `warning`, LOW to `note` —
//! SARIF's closed `level` set has no fourth slot. The native severity is
//! recoverable from `result.properties.severity`, and a rule's
//! `properties["security-severity"]` (plus the `security` tag) is what
//! GitHub's code-scanning UI actually reads to band an alert's displayed
//! severity — `rank` was considered and rejected; see ADR-003.
//!
//! ## Fingerprints
//!
//! `partialFingerprints` reuses [`crate::baseline::fingerprint`] — the same
//! sha256-over-`matched_text` digest the committed baseline uses — with a
//! `/<n>` occurrence ordinal appended, `n` being the 1-based position of this
//! match within its `(file, ruleId, digest)` group. The ordinal exists
//! because the bare digest alone collapses two identical payloads in one
//! file into a single `(ruleId, uri, partialFingerprint)` triple, which is
//! the tuple GitHub tracks an alert by — see ADR-003 and `baseline::fingerprint`'s
//! rustdoc.

use std::collections::HashMap;

use serde::Serialize;

use crate::baseline::fingerprint;
use crate::pattern::{ScanReport, Severity};
use crate::patterns::GradedRule;

/// The official SARIF 2.1.0 schema URL. Never fetched — this is a string
/// literal that identifies the schema version, not a network dependency.
const SARIF_SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/main/Schemata/sarif-schema-2.1.0.json";

const SARIF_VERSION: &str = "2.1.0";

/// Key under which `partialFingerprints` records the reused baseline digest.
const FINGERPRINT_KEY: &str = "matchedTextSha256/v1";

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

/// Identifies this scanner to a SARIF consumer, and carries the full rule
/// catalogue — every loaded pattern, not only the ones that fired. SARIF
/// permits rules with no results, and the metadata is useful on its own.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    pub information_uri: String,
    pub rules: Vec<SarifRule>,
}

/// One rule descriptor, built from a [`GradedRule`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifRule {
    pub id: String,
    pub name: String,
    pub short_description: SarifText,
    pub full_description: SarifText,
    pub help: SarifText,
    pub properties: SarifRuleProperties,
}

/// GitHub-specific rule metadata: the literal `security` tag (required
/// before GitHub honours `security-severity` at all) plus the pattern's
/// category, and `security-severity` — a `"0.0"`-`"10.0"` string GitHub bands
/// into the alert severity shown in its Security tab. This is what actually
/// recovers the native four-level severity where the collapsed `level` alone
/// would not: a reviewer sees four distinct severities, not two folded into
/// `error`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifRuleProperties {
    pub tags: Vec<String>,
    #[serde(rename = "security-severity")]
    pub security_severity: String,
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
    /// Index into `tool.driver.rules`. Omitted (never a dangling index)
    /// rather than defaulted to a sentinel when `pattern_id` has no matching
    /// rule — a case that cannot arise today because `Scanner::new_lenient`
    /// only ever *drops* patterns, so the rule set loaded into
    /// `tool.driver.rules` is always a superset of what can produce a
    /// `ScanMatch`. The result is still emitted either way; a finding is
    /// never dropped for want of a rule descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_index: Option<usize>,
    pub level: String,
    pub message: SarifText,
    pub locations: Vec<SarifLocation>,
    pub partial_fingerprints: HashMap<String, String>,
    pub properties: SarifResultProperties,
}

/// The native severity, preserved through the lossy `level` mapping.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifResultProperties {
    pub severity: String,
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

/// Maps this tool's four-level severity onto GitHub's `security-severity`
/// band, as the string GitHub's SARIF ingest expects.
///
/// Distinct from `level_for`: this is what actually keeps the four native
/// severities distinguishable in GitHub's Security tab, where `level` alone
/// would collapse CRITICAL and HIGH into one bucket. Bands: critical at 9.0
/// and above, high 7.0-8.9, medium 4.0-6.9, low below 4.0 — one representative
/// value per native severity, comfortably inside its band.
fn security_severity_for(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical => "9.0",
        Severity::High => "7.0",
        Severity::Medium => "5.0",
        Severity::Low => "2.0",
    }
}

/// Strips exactly one leading `./`, then percent-encodes every byte outside
/// `A-Za-z0-9-_.~/` as uppercase `%XX`.
///
/// One rule covers every hazard at once: a raw space (`check .`'s directory
/// paths), angle brackets, and non-ASCII bytes are all outside the unreserved
/// set and all get the same treatment, rather than a special case per
/// character class. The `<stdin>` sentinel needs no separate branch for the
/// same reason — its brackets are just two more bytes outside the set.
fn sanitize_uri(raw: &str) -> String {
    let stripped = raw.strip_prefix("./").unwrap_or(raw);
    let mut out = String::with_capacity(stripped.len());
    for byte in stripped.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// Builds `tool.driver.rules`, one descriptor per loaded pattern, and the
/// id-to-index map results use for `ruleIndex`.
///
/// The map is built from this SAME `rules` slice that becomes
/// `tool.driver.rules`, so the two cannot drift apart — there is no second,
/// independently-ordered source either could disagree with.
fn build_rules(rules: &[GradedRule]) -> (Vec<SarifRule>, HashMap<&str, usize>) {
    let mut index = HashMap::with_capacity(rules.len());
    let descriptors = rules
        .iter()
        .enumerate()
        .map(|(i, rule)| {
            index.insert(rule.id.as_str(), i);
            // Descriptions are the one field this tool carries per pattern;
            // SARIF's short/full split has no second source to draw from, so
            // both mirror it, falling back to the name when a pattern's
            // description is empty rather than emitting a blank string.
            let text = if rule.description.is_empty() {
                rule.name.clone()
            } else {
                rule.description.clone()
            };
            SarifRule {
                id: rule.id.clone(),
                name: rule.name.clone(),
                short_description: SarifText { text: text.clone() },
                full_description: SarifText { text },
                help: SarifText {
                    text: rule.remediation.clone(),
                },
                properties: SarifRuleProperties {
                    tags: vec!["security".to_string(), rule.category.clone()],
                    security_severity: security_severity_for(rule.severity).to_string(),
                },
            }
        })
        .collect();
    (descriptors, index)
}

/// Builds a SARIF 2.1.0 document as pretty-printed JSON.
///
/// Reads `ScanReport.matches` and nothing else — see the module-level D-1
/// note. `rules` populates `tool.driver.rules` in full, independent of which
/// patterns fired; `ruleIndex` and `partialFingerprints` are derived from it
/// and from the match list respectively.
///
/// Mirrors [`crate::reporter::format_json`]'s signature — `Result<String,
/// serde_json::Error>` rather than `anyhow`, for the same reason documented
/// there: callers that want to distinguish a serialization failure from
/// everything else can.
pub fn format_sarif(
    reports: &[ScanReport],
    rules: &[GradedRule],
) -> Result<String, serde_json::Error> {
    let (rule_descriptors, rule_index) = build_rules(rules);

    // Occurrence ordinal within `(file, ruleId, digest)`, counted in one pass
    // over the matches in the order the scanner produced them — the same
    // grouping `Baseline::from_reports` counts, so the two schemes agree on
    // what "the same finding, again" means.
    let mut seen: HashMap<(String, String, String), usize> = HashMap::new();

    let results: Vec<SarifResult> = reports
        .iter()
        .flat_map(|report| report.matches.iter())
        .map(|m| {
            let digest = fingerprint(&m.matched_text);
            let key = (m.file.clone(), m.pattern_id.clone(), digest.clone());
            let ordinal = seen.entry(key).or_insert(0);
            *ordinal += 1;

            let mut partial_fingerprints = HashMap::with_capacity(1);
            partial_fingerprints.insert(FINGERPRINT_KEY.to_string(), format!("{digest}/{ordinal}"));

            SarifResult {
                rule_id: m.pattern_id.clone(),
                rule_index: rule_index.get(m.pattern_id.as_str()).copied(),
                level: level_for(m.severity).to_string(),
                message: SarifText {
                    text: m.message.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: sanitize_uri(&m.file),
                        },
                        region: SarifRegion { start_line: m.line },
                    },
                }],
                partial_fingerprints,
                properties: SarifResultProperties {
                    severity: m.severity.to_string(),
                },
            }
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
                    rules: rule_descriptors,
                },
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_uri_strips_one_leading_dot_slash() {
        assert_eq!(sanitize_uri("./docs/foo.md"), "docs/foo.md");
        assert_eq!(sanitize_uri("docs/foo.md"), "docs/foo.md");
        assert_eq!(sanitize_uri("././docs/foo.md"), "./docs/foo.md");
    }

    #[test]
    fn sanitize_uri_percent_encodes_spaces_and_angle_brackets() {
        let encoded = sanitize_uri("has space/<stdin>");
        assert!(!encoded.contains(' '));
        assert!(!encoded.contains('<'));
        assert!(!encoded.contains('>'));
        assert_eq!(encoded, "has%20space/%3Cstdin%3E");
    }

    #[test]
    fn sanitize_uri_leaves_unreserved_characters_untouched() {
        assert_eq!(
            sanitize_uri("a/b-c_d.e~f123"),
            "a/b-c_d.e~f123",
            "unreserved characters must round-trip unchanged"
        );
    }
}
