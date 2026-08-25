//! `--format sarif` (CLI-04, issue #5).
//!
//! Task 1 wires the thinnest complete path from a scanned finding to a SARIF
//! document on stdout: this file starts with a single end-to-end test against
//! that minimal writer. Task 2 expands both the writer and this file to cover
//! the full SARIF contract — rule catalogue, `ruleIndex`, `partialFingerprints`,
//! the withheld-arrays guarantee, URI hygiene, and the exit-code matrix.
//!
//! Spawns the real binary via `env!("CARGO_BIN_EXE_injection-scanner")` — never
//! a hand-built target-directory path; `test_harness_contract_test.rs` fails
//! the build otherwise.

use std::process::Command;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_injection-scanner")
}

fn fixture(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn check_format_sarif_emits_a_minimal_valid_document() {
    let out = Command::new(binary())
        .args([
            "check",
            fixture("injected-skill.md").to_str().expect("utf-8 path"),
            "--format",
            "sarif",
        ])
        .output()
        .expect("run injection-scanner");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let doc: serde_json::Value = serde_json::from_str(&stdout).expect("stdout must parse as JSON");

    assert_eq!(
        doc["version"], "2.1.0",
        "SARIF documents must declare version 2.1.0: {doc}"
    );

    let runs = doc["runs"].as_array().expect("runs must be an array");
    assert_eq!(runs.len(), 1, "this tool always emits exactly one run");

    let results = runs[0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(
        !results.is_empty(),
        "the fixture has findings, so results must not be empty: {doc}"
    );

    for result in results {
        assert!(
            result["ruleId"].is_string(),
            "every result needs a ruleId: {result}"
        );
        assert!(
            result["level"].is_string(),
            "every result needs a level: {result}"
        );
        assert!(
            result["message"]["text"].is_string(),
            "every result needs message.text: {result}"
        );
        let start_line = result["locations"][0]["physicalLocation"]["region"]["startLine"]
            .as_u64()
            .unwrap_or_else(|| panic!("every result needs a numeric startLine: {result}"));
        assert!(start_line >= 1, "startLine must be 1-based: {result}");
    }
}
