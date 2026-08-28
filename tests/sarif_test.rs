//! `--format sarif` (CLI-04, issue #5).
//!
//! Task 1 wired the thinnest complete path from a scanned finding to a SARIF
//! document on stdout (`check_format_sarif_emits_a_minimal_valid_document`).
//! This file's remaining tests cover the full contract added in Task 2: the
//! rule catalogue, `ruleIndex`, `partialFingerprints`, the D-1 guarantee that
//! `suppressed`/`low_confidence`/`baselined` never become SARIF results, URI
//! hygiene, and format-independent exit codes.
//!
//! No test in this file types an injection payload as a Rust string literal.
//! Every payload used to build a temporary scanned document is extracted at
//! runtime from an existing fixture (via [`matched_text`]) or from an
//! existing repository file's own committed content (via
//! [`matched_text_in_file`]) — this repo scans itself, and a literal payload
//! typed here would be a new, un-baselined finding in a brand new file.
//!
//! Spawns the real binary via `env!("CARGO_BIN_EXE_injection-scanner")` — never
//! a hand-built target-directory path; `test_harness_contract_test.rs` fails
//! the build otherwise.

use std::collections::HashSet;
use std::io::Write;
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

/// A temp file that cleans itself up. Named uniquely per call so parallel
/// tests in this file never collide on the same path.
struct Doc(PathBuf);

impl Doc {
    fn new(name: &str, content: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "injscan-sarif-{}-{}-{name}",
            std::process::id(),
            unique_suffix()
        ));
        let mut file = std::fs::File::create(&path).expect("create temp doc");
        file.write_all(content.as_bytes()).expect("write temp doc");
        Self(path)
    }
    fn path(&self) -> &str {
        self.0.to_str().expect("utf-8 path")
    }
}

impl Drop for Doc {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn unique_suffix() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(binary()).args(args).output().expect("run");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn scan_sarif(path: &str, extra: &[&str]) -> Value {
    let mut args = vec!["check", path, "--format", "sarif"];
    args.extend_from_slice(extra);
    let (_, stdout, stderr) = run(&args);
    serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("sarif stdout must be JSON: {e}\nstdout: {stdout}\nstderr: {stderr}")
    })
}

fn scan_json(path: &str, extra: &[&str]) -> Value {
    let mut args = vec!["check", path, "--format", "json"];
    args.extend_from_slice(extra);
    let (_, stdout, stderr) = run(&args);
    serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("json stdout must be JSON: {e}\nstdout: {stdout}\nstderr: {stderr}")
    })
}

/// The `matched_text` the scanner itself reported for `pattern_id` when
/// scanning `path` — extracted by running the tool, never typed as a literal.
fn matched_text_in_file(path: &Path, pattern_id: &str, all_files: bool) -> String {
    let path_str = path.to_str().expect("utf-8 path");
    let mut args = vec!["check", path_str, "--format", "json", "--strict"];
    if all_files {
        args.push("--all-files");
    }
    let (_, stdout, stderr) = run(&args);
    let doc: Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("json stdout must be JSON: {e}\nstdout: {stdout}\nstderr: {stderr}")
    });
    for report in doc.as_array().expect("top level must be an array") {
        for m in report["matches"]
            .as_array()
            .expect("matches must be an array")
        {
            if m["pattern_id"] == pattern_id {
                return m["matched_text"]
                    .as_str()
                    .expect("matched_text must be a string")
                    .to_string();
            }
        }
    }
    panic!(
        "pattern {pattern_id} did not fire scanning {}",
        path.display()
    );
}

fn matched_text(fixture_name: &str, pattern_id: &str) -> String {
    matched_text_in_file(&fixture(fixture_name), pattern_id, false)
}

/// A file containing a genuine LOW finding, so the level mapping is exercised
/// against a real scan rather than a literal typed here.
///
/// Was `tests/corpus_test.rs`, which named PI035 in a source comment back when
/// PI035 matched the bare phrase "jailbreak prompt". #99 made that pattern
/// require the request the research framing is excusing — and raised it to
/// MEDIUM — so it no longer fires on prose that merely names the technique,
/// which was the entire point of the change. PI041 (zero-width characters) is
/// the LOW that remains, and the generated catalogue quotes zero-width payloads
/// in its examples. That file is regenerated by the same loop that changes
/// patterns, so it stays current.
fn low_finding_source() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/PATTERN-CATALOGUE.md")
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
    let doc: Value = serde_json::from_str(&stdout).expect("stdout must parse as JSON");

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

#[test]
fn level_mapping_is_total_and_closed() {
    let critical = matched_text("injected-skill.md", "PI001");
    let high = matched_text("injected-skill.md", "PI030");
    let medium = matched_text("injected-skill.md", "PI003");
    let low = matched_text_in_file(&low_finding_source(), "PI041", true);

    let doc = Doc::new(
        "levels.md",
        &format!("{critical}\n\n{high}\n\n{medium}\n\n{low}\n"),
    );

    let sarif = scan_sarif(doc.path(), &["--strict"]);
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(!results.is_empty());

    let allowed = ["error", "warning", "note", "none"];
    for result in results {
        let level = result["level"].as_str().expect("level must be a string");
        assert!(
            allowed.contains(&level),
            "unexpected level {level}: {result}"
        );
    }

    let level_for = |rule_id: &str| -> String {
        results
            .iter()
            .find(|r| r["ruleId"] == rule_id)
            .unwrap_or_else(|| panic!("no result for {rule_id} in {results:?}"))["level"]
            .as_str()
            .expect("level must be a string")
            .to_string()
    };
    assert_eq!(level_for("PI001"), "error", "CRITICAL must map to error");
    assert_eq!(level_for("PI030"), "error", "HIGH must map to error");
}

#[test]
fn native_severity_survives_the_lossy_mapping() {
    let critical = matched_text("injected-skill.md", "PI001");
    let high = matched_text("injected-skill.md", "PI030");
    let doc = Doc::new("severity-props.md", &format!("{critical}\n\n{high}\n"));

    let sarif = scan_sarif(doc.path(), &["--strict"]);
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(!results.is_empty());
    for result in results {
        let native = result["properties"]["severity"]
            .as_str()
            .unwrap_or_else(|| panic!("missing properties.severity: {result}"));
        assert!(
            ["CRITICAL", "HIGH", "MEDIUM", "LOW"].contains(&native),
            "unexpected native severity {native}: {result}"
        );
    }

    let rules = sarif["runs"][0]["tool"]["driver"]["rules"]
        .as_array()
        .expect("rules must be an array");
    let security_severity = |rule_id: &str| -> f64 {
        rules
            .iter()
            .find(|r| r["id"] == rule_id)
            .unwrap_or_else(|| panic!("no rule {rule_id} in {rules:?}"))["properties"]
            ["security-severity"]
            .as_str()
            .expect("security-severity must be a string")
            .parse::<f64>()
            .expect("security-severity must parse as a float")
    };
    assert!(
        security_severity("PI001") > security_severity("PI030"),
        "PI001 (CRITICAL) security-severity must exceed PI030 (HIGH)'s — the pair that \
         collapses to a single `level` must stay distinguishable through this field"
    );
}

#[test]
fn every_result_rule_id_resolves_by_id_and_index() {
    let critical = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new("resolve.md", &format!("{critical}\n"));
    let sarif = scan_sarif(doc.path(), &["--strict"]);

    let rules = sarif["runs"][0]["tool"]["driver"]["rules"]
        .as_array()
        .expect("rules must be an array");
    assert!(
        !rules.is_empty(),
        "rules must list every loaded pattern, not only the ones that fired"
    );

    let rule_ids: HashSet<&str> = rules
        .iter()
        .map(|r| r["id"].as_str().expect("rule id must be a string"))
        .collect();

    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(!results.is_empty());
    for result in results {
        let rule_id = result["ruleId"].as_str().expect("ruleId must be a string");
        assert!(
            rule_ids.contains(rule_id),
            "ruleId {rule_id} has no matching rule descriptor"
        );
        let index = result["ruleIndex"]
            .as_u64()
            .unwrap_or_else(|| panic!("missing ruleIndex: {result}")) as usize;
        assert_eq!(
            rules[index]["id"], rule_id,
            "rules[ruleIndex].id must match result.ruleId"
        );
    }
}

#[test]
fn rules_carry_usable_metadata() {
    let critical = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new("metadata.md", &format!("{critical}\n"));
    let sarif = scan_sarif(doc.path(), &["--strict"]);
    let rules = sarif["runs"][0]["tool"]["driver"]["rules"]
        .as_array()
        .expect("rules must be an array");
    assert!(!rules.is_empty());

    for rule in rules {
        assert!(rule["id"].is_string(), "missing id: {rule}");
        assert!(rule["name"].is_string(), "missing name: {rule}");
        assert!(
            !rule["shortDescription"]["text"]
                .as_str()
                .unwrap_or("")
                .is_empty(),
            "missing shortDescription.text: {rule}"
        );
        assert!(
            !rule["fullDescription"]["text"]
                .as_str()
                .unwrap_or("")
                .is_empty(),
            "missing fullDescription.text: {rule}"
        );
        assert!(
            !rule["help"]["text"].as_str().unwrap_or("").is_empty(),
            "missing help.text (remediation): {rule}"
        );
        let tags = rule["properties"]["tags"]
            .as_array()
            .unwrap_or_else(|| panic!("missing properties.tags: {rule}"));
        assert!(
            tags.iter().any(|t| t == "security"),
            "properties.tags must include the literal security tag: {rule}"
        );
        assert!(
            rule["properties"]["security-severity"].is_string(),
            "missing properties.security-severity: {rule}"
        );
    }
}

#[test]
fn suppressed_findings_produce_no_sarif_results() {
    let critical = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new(
        "suppressed.md",
        &format!("{critical}  <!-- injection-scanner:ignore PI001 -->\n"),
    );

    let json = scan_json(doc.path(), &[]);
    let report = &json.as_array().expect("json top level must be an array")[0];
    assert!(
        !report["suppressed"]
            .as_array()
            .expect("suppressed must be an array")
            .is_empty(),
        "the suppressed array must be non-empty, or this test proves nothing: {report}"
    );

    let sarif = scan_sarif(doc.path(), &[]);
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(
        results.is_empty(),
        "a suppressed finding must not become a SARIF result: {results:?}"
    );
}

#[test]
fn low_confidence_findings_produce_no_sarif_results() {
    let critical = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new("fenced.md", &format!("```\n{critical}\n```\n"));

    let json = scan_json(doc.path(), &[]);
    let report = &json.as_array().expect("json top level must be an array")[0];
    assert!(
        !report["low_confidence"]
            .as_array()
            .expect("low_confidence must be an array")
            .is_empty(),
        "the low_confidence array must be non-empty, or this test proves nothing: {report}"
    );

    let sarif = scan_sarif(doc.path(), &[]);
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(
        results.is_empty(),
        "a low-confidence finding must not become a SARIF result: {results:?}"
    );
}

#[test]
fn baselined_findings_produce_no_sarif_results() {
    let critical = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new("baselined.md", &format!("{critical}\n"));
    let baseline_path = std::env::temp_dir().join(format!(
        "injscan-sarif-baseline-{}-{}.json",
        std::process::id(),
        unique_suffix()
    ));
    let baseline_str = baseline_path.to_str().expect("utf-8 path");

    // Accept the current finding.
    let (write_code, _, write_stderr) = run(&[
        "check",
        doc.path(),
        "--write-baseline",
        baseline_str,
        "--quiet",
    ]);
    assert_eq!(
        write_code, 0,
        "--write-baseline always exits 0: {write_stderr}"
    );

    let json = scan_json(doc.path(), &["--baseline", baseline_str]);
    let report = &json.as_array().expect("json top level must be an array")[0];
    assert!(
        !report["baselined"]
            .as_array()
            .expect("baselined must be an array")
            .is_empty(),
        "the baselined array must be non-empty, or this test proves nothing: {report}"
    );

    let sarif = scan_sarif(doc.path(), &["--baseline", baseline_str]);
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(
        results.is_empty(),
        "a baselined finding must not become a SARIF result: {results:?}"
    );

    let _ = std::fs::remove_file(&baseline_path);
}

#[test]
fn sarif_2_1_0_required_structure_is_present() {
    let critical = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new("structure.md", &format!("{critical}\n"));
    let sarif = scan_sarif(doc.path(), &["--strict"]);

    assert!(
        sarif.get("$schema").and_then(|v| v.as_str()).is_some(),
        "missing $schema: {sarif}"
    );
    assert_eq!(sarif["version"], "2.1.0");
    let runs = sarif["runs"].as_array().expect("runs must be an array");
    assert!(!runs.is_empty());
    assert!(
        runs[0]["tool"]["driver"]["name"].as_str().is_some(),
        "missing runs[].tool.driver.name"
    );

    let results = runs[0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(!results.is_empty());
    for result in results {
        assert!(result["ruleId"].is_string());
        assert!(result["level"].is_string());
        assert!(result["message"]["text"].is_string());
        let uri = result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"]
            .as_str()
            .expect("uri must be a string");
        assert!(!uri.is_empty());
        let start_line = result["locations"][0]["physicalLocation"]["region"]["startLine"]
            .as_u64()
            .expect("startLine must be numeric");
        assert!(start_line >= 1);
    }
}

#[test]
fn uri_hygiene_no_dot_slash_no_raw_special_characters() {
    let dir = std::env::temp_dir().join(format!(
        "injscan-sarif-dir-{}-{}",
        std::process::id(),
        unique_suffix()
    ));
    std::fs::create_dir_all(&dir).expect("mkdir temp dir");
    let critical = matched_text("injected-skill.md", "PI001");
    std::fs::write(dir.join("finding.md"), format!("{critical}\n")).expect("write finding");
    std::fs::write(dir.join("has space.md"), format!("{critical}\n")).expect("write spaced file");

    let out = Command::new(binary())
        .current_dir(&dir)
        .args(["check", ".", "--format", "sarif", "--strict"])
        .output()
        .expect("run");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let sarif: Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("sarif stdout must be JSON: {e}\nstdout: {stdout}"));
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    assert!(!results.is_empty());

    for result in results {
        let uri = result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"]
            .as_str()
            .expect("uri must be a string");
        assert!(!uri.starts_with("./"), "uri must not start with ./: {uri}");
        assert!(
            !uri.contains(' '),
            "uri must not contain a raw space: {uri}"
        );
        assert!(
            !uri.contains('<') && !uri.contains('>'),
            "uri must not contain angle brackets: {uri}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fingerprints_are_line_independent_and_non_colliding() {
    let critical = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new(
        "dup.md",
        &format!("{critical}\nordinary prose line\n{critical}\n"),
    );
    let sarif = scan_sarif(doc.path(), &["--strict"]);
    let results = sarif["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    let matching: Vec<&Value> = results.iter().filter(|r| r["ruleId"] == "PI001").collect();
    assert_eq!(
        matching.len(),
        2,
        "expected two PI001 findings from two occurrences: {results:?}"
    );

    let fingerprint = |r: &Value| -> String {
        r["partialFingerprints"]["matchedTextSha256/v1"]
            .as_str()
            .unwrap_or_else(|| panic!("missing partialFingerprints.matchedTextSha256/v1: {r}"))
            .to_string()
    };
    let fp1 = fingerprint(matching[0]);
    let fp2 = fingerprint(matching[1]);
    assert_ne!(
        fp1, fp2,
        "two identical payloads in one file must get DISTINCT fingerprints, or GitHub \
         collapses them into a single alert that closes while one payload remains"
    );

    let padded = Doc::new(
        "dup-padded.md",
        &format!(
            "{}{critical}\nordinary prose line\n{critical}\n",
            "\n".repeat(10)
        ),
    );
    let sarif2 = scan_sarif(padded.path(), &["--strict"]);
    let results2 = sarif2["runs"][0]["results"]
        .as_array()
        .expect("results must be an array");
    let matching2: Vec<&Value> = results2.iter().filter(|r| r["ruleId"] == "PI001").collect();
    assert_eq!(matching2.len(), 2);
    assert_eq!(
        fingerprint(matching2[0]),
        fp1,
        "prepending blank lines must not change the fingerprint"
    );
    assert_eq!(
        fingerprint(matching2[1]),
        fp2,
        "prepending blank lines must not change the fingerprint"
    );
}

#[test]
fn exit_codes_are_format_independent() {
    let critical_text = matched_text("injected-skill.md", "PI001");
    let medium_text = matched_text("injected-skill.md", "PI003");

    let critical_doc = Doc::new("exit-critical.md", &format!("{critical_text}\n"));
    let medium_doc = Doc::new("exit-medium.md", &format!("{medium_text}\n"));
    let clean_doc = Doc::new("exit-clean.md", "an ordinary paragraph about the weather\n");

    let cases: [(&str, &str, i32); 3] = [
        (critical_doc.path(), "critical", 1),
        (medium_doc.path(), "critical", 2),
        (clean_doc.path(), "critical", 0),
    ];

    for (path, bar, want) in cases {
        for format in ["text", "sarif"] {
            let (code, _, _) = run(&[
                "check",
                path,
                "--fail-on",
                bar,
                "--format",
                format,
                "--quiet",
            ]);
            assert_eq!(
                code, want,
                "check {path} --fail-on {bar} --format {format} exited {code}, expected {want}"
            );
        }
    }
}

#[test]
fn quiet_format_sarif_writes_nothing_to_stdout() {
    let critical_text = matched_text("injected-skill.md", "PI001");
    let doc = Doc::new("quiet.md", &format!("{critical_text}\n"));
    let (code, stdout, stderr) = run(&["check", doc.path(), "--format", "sarif", "--quiet"]);
    assert_eq!(code, 1);
    assert!(stdout.is_empty(), "--quiet must print nothing: {stdout:?}");
    assert!(
        stderr.is_empty(),
        "--quiet covers stderr notes too: {stderr:?}"
    );
}

#[test]
fn rules_format_sarif_is_rejected_at_parse_time() {
    let (code, stdout, stderr) = run(&["rules", "--format", "sarif"]);
    assert_ne!(code, 0, "rules --format sarif must be rejected");
    assert!(
        stderr.contains("text") && stderr.contains("json"),
        "stderr must name the valid values: {stderr}"
    );
    assert!(
        serde_json::from_str::<Value>(&stdout).is_err() || stdout.trim().is_empty(),
        "stdout must not be a SARIF document: {stdout}"
    );
}
