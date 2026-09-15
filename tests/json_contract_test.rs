//! `--format json` and `rules --format json` byte-identical contract guard
//! (CLI-04, issue #5).
//!
//! Adding SARIF must not move `--format json`: `spec-ci-plugin` does
//! `JSON.parse(output) as Array<...>` against it. No golden file is
//! committed here — a fixed snapshot would put a verbatim finding (matched
//! text, line numbers) in a brand new repository file, and this repo scans
//! itself. The contract is asserted explicitly instead, which is what a
//! golden file would have been protecting: the top-level shape, the exact
//! key set of a report object, the exact key set of a match object, and that
//! the output stays pretty-printed.
//!
//! Spawns the real binary via `env!("CARGO_BIN_EXE_injection-scanner")` —
//! never a hand-built target-directory path; `test_harness_contract_test.rs`
//! fails the build otherwise.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_injection-scanner")
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(binary()).args(args).output().expect("run");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn keys(value: &Value) -> BTreeSet<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("expected a JSON object, got {value}"))
        .keys()
        .cloned()
        .collect()
}

fn set_of(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|s| s.to_string()).collect()
}

fn check_json_fixture() -> Value {
    let (_, stdout, stderr) = run(&[
        "check",
        fixture("injected-skill.md").to_str().expect("utf-8 path"),
        "--format",
        "json",
    ]);
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stdout must be JSON: {e}\nstdout: {stdout}\nstderr: {stderr}"))
}

#[test]
fn format_json_top_level_is_an_array() {
    let doc = check_json_fixture();
    assert!(
        doc.is_array(),
        "top level must be an array — spec-ci-plugin does \
         `JSON.parse(output) as Array<...>`: {doc}"
    );
}

#[test]
fn format_json_report_key_set_is_exactly_pinned() {
    let doc = check_json_fixture();
    let expected = set_of(&[
        "file",
        "matches",
        "suppressed",
        "low_confidence",
        "baselined",
        "critical_count",
        "high_count",
        "medium_count",
        "low_count",
    ]);
    let reports = doc.as_array().expect("top level must be an array");
    assert!(!reports.is_empty());
    for report in reports {
        assert_eq!(
            keys(report),
            expected,
            "report key set drifted — a field added for SARIF's benefit must not leak \
             into --format json: {report}"
        );
    }
}

#[test]
fn format_json_match_key_set_is_exactly_pinned() {
    let doc = check_json_fixture();
    let expected = set_of(&[
        "pattern_id",
        "pattern_name",
        "severity",
        "message",
        "remediation",
        "file",
        "line",
        "matched_text",
        "context",
        "confidence",
    ]);
    let mut checked = 0usize;
    for report in doc.as_array().expect("top level must be an array") {
        for m in report["matches"]
            .as_array()
            .expect("matches must be an array")
        {
            assert_eq!(keys(m), expected, "match key set drifted: {m}");
            checked += 1;
        }
    }
    assert!(
        checked > 0,
        "the fixture must produce at least one match, or this test proves nothing"
    );
}

#[test]
fn format_json_output_stays_pretty_printed() {
    let (_, stdout, _) = run(&[
        "check",
        fixture("injected-skill.md").to_str().expect("utf-8 path"),
        "--format",
        "json",
    ]);
    assert!(
        stdout.starts_with("[\n"),
        "output must stay pretty-printed, not compact: {stdout:?}"
    );
}

#[test]
fn rules_format_json_key_set_is_exactly_pinned() {
    let (_, stdout, stderr) = run(&["rules", "--format", "json"]);
    let doc: Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stdout must be JSON: {e}\nstdout: {stdout}\nstderr: {stderr}"));
    let expected = set_of(&[
        "id",
        "name",
        "severity",
        "category",
        "description",
        "remediation",
        "pattern",
        "tags",
    ]);
    let entries = doc
        .as_array()
        .expect("rules --format json top level must be an array");
    assert!(!entries.is_empty());
    for entry in entries {
        assert_eq!(
            keys(entry),
            expected,
            "GradedRule key set drifted — this guards the patterns::mod move: {entry}"
        );
    }
}

// -------------------------------------------------------------------------
// Issue #129: `config_parse_error` is additive and conditional — present
// only on a report whose config block could not be parsed, absent
// everywhere else, so the pinned key set above stays exact for every
// consumer on every report that parsed cleanly.
// -------------------------------------------------------------------------

/// New CLI fixtures never live under `tests/fixtures/` (a committed fixture
/// carrying a live payload would add a finding to this repo's own self-scan);
/// a fresh temp dir per call, matching `tests/scan_resilience_test.rs`.
fn broken_config_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "injscan-json-contract-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir must be creatable");
    std::fs::write(dir.join("broken.json"), "{ \"servers\": { \"x\": } }\n")
        .expect("fixture must be writable");
    dir
}

#[test]
fn format_json_carries_config_parse_error_on_a_broken_config_report() {
    let dir = broken_config_dir();
    let (_, stdout, stderr) = run(&[
        "check",
        dir.to_str().expect("utf-8 path"),
        "--format",
        "json",
    ]);
    let doc: Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stdout must be JSON: {e}\nstdout: {stdout}\nstderr: {stderr}"));

    assert!(
        doc.is_array(),
        "top level must still be an array with the new field present: {doc}"
    );
    let reports = doc.as_array().expect("top level must be an array");
    assert_eq!(reports.len(), 1, "one file scanned, one report: {doc}");
    let report = &reports[0];
    let error = report.get("config_parse_error").unwrap_or_else(|| {
        panic!("report for a broken config block must carry config_parse_error: {report}")
    });
    assert!(
        error.is_string(),
        "config_parse_error must be a string, not null or absent: {report}"
    );
}
