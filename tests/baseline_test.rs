//! `--baseline` / `--write-baseline` (issue #25, CLI-08).
//!
//! An existing repository cannot adopt this scanner if day one is a wall of
//! findings. A baseline is the standard answer: accept the current state once,
//! then only new findings fail the build. This file guards the two-command
//! adoption flow end to end, plus the adversarial edges that would make the
//! feature unsafe — an unbounded accept, a fingerprint an attacker can retune,
//! a silently-ignored malformed baseline, and a stale entry nobody notices.
//!
//! Spawns the real binary via `env!("CARGO_BIN_EXE_injection-scanner")` —
//! never a hand-built target-directory path; `test_harness_contract_test.rs`
//! fails the build otherwise.

use std::io::Write;
use std::process::Command;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_injection-scanner")
}

/// A temp file that cleans itself up.
struct Doc(std::path::PathBuf);

impl Doc {
    fn new(name: &str, content: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "injscan-baseline-{}-{}-{name}",
            std::process::id(),
            unique_suffix()
        ));
        let mut file = std::fs::File::create(&path).expect("create");
        file.write_all(content.as_bytes()).expect("write");
        Self(path)
    }
    fn path(&self) -> &str {
        self.0.to_str().expect("utf-8 path")
    }
    fn write(&self, content: &str) {
        std::fs::write(&self.0, content).expect("overwrite");
    }
}

impl Drop for Doc {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Cheap per-call uniqueness so parallel tests in this file never collide on
/// the same temp path.
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

fn fixture_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/injected-skill.md")
}

/// The substring that makes the fixture a finding, read once from the fixture
/// itself so this file never re-types an injection payload.
fn fixture_payload_substring() -> String {
    let content = std::fs::read_to_string(fixture_path()).expect("fixture must be readable");
    content
        .lines()
        .find(|line| {
            line.to_lowercase()
                .contains("ignore all previous instructions")
        })
        .expect("fixture must contain the line this test keys on")
        .trim()
        .to_string()
}

fn baseline_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "injscan-baseline-file-{}-{}-{name}.json",
        std::process::id(),
        unique_suffix()
    ))
}

// ---------------------------------------------------------------------------
// Task 1 — the tracer: write it, then re-scan clean.
// ---------------------------------------------------------------------------

#[test]
fn the_two_command_adoption_flow_works_end_to_end() {
    let fixture = fixture_path();
    let fixture = fixture.to_str().expect("utf-8 path");
    let baseline = baseline_path("tracer");

    // Step 0: unbaselined, the fixture fails the build.
    let (code, _, _) = run(&["check", fixture, "--quiet"]);
    assert_eq!(
        code, 1,
        "the fixture must contain at least one finding for this test to mean anything"
    );

    // Step 1: --write-baseline accepts the current state and exits CLEAN.
    let (code, _, _) = run(&[
        "check",
        fixture,
        "--write-baseline",
        baseline.to_str().expect("utf-8 path"),
        "--quiet",
    ]);
    assert_eq!(
        code, 0,
        "--write-baseline means \"accept the current state\" and must exit 0 even though \
         the scan found CRITICAL findings — that is the entire point of D-2"
    );
    assert!(
        baseline.exists(),
        "--write-baseline must create the file at the given path"
    );

    // Step 2: re-scanning with --baseline moves the findings out of `matches`.
    let (code, stdout, _) = run(&[
        "check",
        fixture,
        "--baseline",
        baseline.to_str().expect("utf-8 path"),
        "--format",
        "json",
    ]);
    assert_eq!(
        code, 0,
        "every finding was baselined, so the exit code must read as clean"
    );

    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let reports = parsed.as_array().expect("top level must be an array");
    let report = reports
        .iter()
        .find(|r| r["file"].as_str() == Some(fixture))
        .expect("the fixture's report must be present");

    assert!(
        report["matches"]
            .as_array()
            .expect("matches is array")
            .is_empty(),
        "a fully-baselined file must have an empty `matches`: {report}"
    );
    assert!(
        !report["baselined"]
            .as_array()
            .expect("baselined is array — the array must exist and be populated")
            .is_empty(),
        "the baselined findings must be recorded somewhere, not silently dropped: {report}"
    );
    assert_eq!(
        report["critical_count"], 0,
        "a baselined CRITICAL must not be counted in the severity tallies: {report}"
    );

    let _ = std::fs::remove_file(&baseline);
}
