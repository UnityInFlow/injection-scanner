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

// ---------------------------------------------------------------------------
// Task 2 — the adversarial edges: count, staleness, malformed input, and the
// two rejections.
// ---------------------------------------------------------------------------

#[test]
fn an_occurrence_beyond_count_is_still_reported() {
    let line = fixture_payload_substring();
    let doc = Doc::new("count.md", &format!("{line}\n"));
    let baseline = baseline_path("count");

    let (code, _, _) = run(&[
        "check",
        doc.path(),
        "--write-baseline",
        baseline.to_str().expect("utf-8 path"),
        "--quiet",
    ]);
    assert_eq!(code, 0, "write-baseline always exits clean");

    // The same file now contains the payload twice.
    doc.write(&format!(
        "{line}\n\nSome unrelated clean prose sits here.\n\n{line}\n"
    ));

    let (code, stdout, _) = run(&[
        "check",
        doc.path(),
        "--baseline",
        baseline.to_str().expect("utf-8 path"),
        "--format",
        "json",
    ]);
    assert_eq!(
        code, 1,
        "baselining one occurrence accepts one, not an unlimited number (D-1) — the second \
         occurrence must still fail the build"
    );

    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let reports = parsed.as_array().expect("array");
    let report = reports
        .iter()
        .find(|r| r["file"].as_str() == Some(doc.path()))
        .expect("report present");
    assert_eq!(
        report["matches"].as_array().expect("array").len(),
        1,
        "exactly one occurrence beyond the accepted count must remain reported: {report}"
    );
    assert_eq!(
        report["baselined"].as_array().expect("array").len(),
        1,
        "exactly one occurrence must have been baselined, matching the recorded count: {report}"
    );

    let _ = std::fs::remove_file(&baseline);
}

#[test]
fn a_stale_entry_is_surfaced_and_only_when_stale() {
    let line = fixture_payload_substring();
    let doc = Doc::new("stale.md", &format!("{line}\n"));
    let baseline = baseline_path("stale");

    run(&[
        "check",
        doc.path(),
        "--write-baseline",
        baseline.to_str().expect("utf-8 path"),
        "--quiet",
    ]);

    // Control: the entry still matches, so the note must be ABSENT — otherwise
    // this test could pass vacuously against a note printed unconditionally.
    let (code, _, stderr) = run(&[
        "check",
        doc.path(),
        "--baseline",
        baseline.to_str().expect("utf-8 path"),
    ]);
    assert_eq!(code, 0);
    assert!(
        !stderr.contains("can be pruned"),
        "an entry that matched must NOT be reported stale: {stderr}"
    );

    // Now the document goes clean — the baseline entry matches nothing.
    doc.write("nothing suspicious here at all\n");
    let (code, _, stderr) = run(&[
        "check",
        doc.path(),
        "--baseline",
        baseline.to_str().expect("utf-8 path"),
    ]);
    assert_eq!(code, 0, "a stale baseline entry does not fail the build");
    assert!(
        stderr.contains("PI001"),
        "the stale note must name the pattern_id: {stderr}"
    );
    assert!(
        stderr.contains(doc.path()),
        "the stale note must name the file: {stderr}"
    );
    assert!(
        stderr.contains("pruned"),
        "the note must say the entry can be pruned, not just that it exists: {stderr}"
    );

    let _ = std::fs::remove_file(&baseline);
}

#[test]
fn editing_lines_above_a_finding_does_not_invalidate_its_baseline_entry() {
    let line = fixture_payload_substring();
    let doc = Doc::new("lines-move.md", &format!("{line}\n"));
    let baseline = baseline_path("lines-move");

    run(&[
        "check",
        doc.path(),
        "--write-baseline",
        baseline.to_str().expect("utf-8 path"),
        "--quiet",
    ]);

    doc.write(&format!(
        "Some clean prose.\nMore clean prose.\nEven more of it.\n\n{line}\n"
    ));

    let (code, stdout, _) = run(&[
        "check",
        doc.path(),
        "--baseline",
        baseline.to_str().expect("utf-8 path"),
        "--format",
        "json",
    ]);
    assert_eq!(
        code, 0,
        "editing lines above a finding must not invalidate its baseline entry (D-1) — line \
         number is not part of identity"
    );

    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let reports = parsed.as_array().expect("array");
    let report = reports
        .iter()
        .find(|r| r["file"].as_str() == Some(doc.path()))
        .expect("report present");
    assert!(
        !report["baselined"].as_array().expect("array").is_empty(),
        "the finding must still be baselined after prose was inserted above it: {report}"
    );

    let _ = std::fs::remove_file(&baseline);
}

#[test]
fn a_baseline_that_is_not_json_is_a_hard_error() {
    let bad = baseline_path("not-json");
    std::fs::write(&bad, "this is not json at all\n").expect("write");
    let doc = Doc::new("target-a.md", "nothing suspicious here\n");

    let (code, stdout, stderr) = run(&[
        "check",
        doc.path(),
        "--baseline",
        bad.to_str().expect("utf-8 path"),
    ]);
    assert_ne!(
        code, 0,
        "a malformed baseline must be a hard error — a repository must not believe it is \
         gated when it is not"
    );
    assert!(
        stdout.is_empty(),
        "no scan report should print when the baseline itself failed to load: {stdout:?}"
    );
    assert!(
        stderr.contains(bad.to_str().expect("utf-8 path")),
        "stderr must name the offending path: {stderr}"
    );

    let _ = std::fs::remove_file(&bad);
}

#[test]
fn a_baseline_entry_missing_a_required_field_is_a_hard_error() {
    let bad = baseline_path("missing-digest");
    std::fs::write(
        &bad,
        r#"{"version":1,"entries":[{"file":"x.md","pattern_id":"PI001","count":1}]}"#,
    )
    .expect("write");
    let doc = Doc::new("target-b.md", "nothing suspicious here\n");

    let (code, stdout, stderr) = run(&[
        "check",
        doc.path(),
        "--baseline",
        bad.to_str().expect("utf-8 path"),
    ]);
    assert_ne!(
        code, 0,
        "an entry missing `digest` must fail the parse rather than silently match nothing"
    );
    assert!(stdout.is_empty(), "{stdout:?}");
    assert!(
        stderr.contains(bad.to_str().expect("utf-8 path")),
        "{stderr}"
    );

    let _ = std::fs::remove_file(&bad);
}

#[test]
fn a_baseline_with_an_unrecognised_version_is_a_hard_error() {
    let bad = baseline_path("bad-version");
    std::fs::write(&bad, r#"{"version":99,"entries":[]}"#).expect("write");
    let doc = Doc::new("target-c.md", "nothing suspicious here\n");

    let (code, stdout, stderr) = run(&[
        "check",
        doc.path(),
        "--baseline",
        bad.to_str().expect("utf-8 path"),
    ]);
    assert_ne!(
        code, 0,
        "an unrecognised baseline version must be a hard error"
    );
    assert!(stdout.is_empty(), "{stdout:?}");
    assert!(
        stderr.contains("99") && stderr.to_lowercase().contains("version"),
        "stderr must name both the found and expected version: {stderr}"
    );

    let _ = std::fs::remove_file(&bad);
}

#[test]
fn a_nonexistent_baseline_path_is_a_hard_error() {
    let missing = baseline_path("does-not-exist");
    let doc = Doc::new("target-d.md", "nothing suspicious here\n");

    let (code, stdout, stderr) = run(&[
        "check",
        doc.path(),
        "--baseline",
        missing.to_str().expect("utf-8 path"),
    ]);
    assert_ne!(
        code, 0,
        "a missing baseline file must be a hard error, never a silent no-op"
    );
    assert!(stdout.is_empty(), "{stdout:?}");
    assert!(
        stderr.contains(missing.to_str().expect("utf-8 path")),
        "stderr must name the missing path: {stderr}"
    );
}

#[test]
fn baseline_and_write_baseline_together_are_rejected() {
    let doc = Doc::new("both-flags.md", "nothing suspicious here\n");
    let a = baseline_path("both-a");
    let b = baseline_path("both-b");
    let (code, _, stderr) = run(&[
        "check",
        doc.path(),
        "--baseline",
        a.to_str().expect("utf-8 path"),
        "--write-baseline",
        b.to_str().expect("utf-8 path"),
    ]);
    // Deliberately NOT asserting the specific exit code: clap's usage-error
    // exit is 2, which would be indistinguishable from exit::BELOW_THRESHOLD.
    assert_ne!(code, 0, "the two flags must be mutually exclusive");
    assert!(
        stderr.to_lowercase().contains("cannot be used with"),
        "clap's conflict wording should explain WHY, not just fail silently: {stderr}"
    );
}

#[test]
fn write_baseline_with_stdin_is_rejected() {
    let baseline = baseline_path("stdin-rejected");
    let mut child = Command::new(binary())
        .args([
            "check",
            "-",
            "--write-baseline",
            baseline.to_str().expect("utf-8 path"),
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn");
    // Nothing is written to stdin — the rejection must happen before stdin is
    // ever read, so a piped producer sees the error rather than a closed pipe.
    // Taking (and dropping) the write half closes the pipe on this end.
    let _ = child.stdin.take();
    let out = child.wait_with_output().expect("wait");

    assert!(
        !out.status.success(),
        "stdin has no stable file identity to record a baseline against"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.to_lowercase().contains("stdin"),
        "stderr must name stdin as the reason: {stderr}"
    );
    assert!(
        !baseline.exists(),
        "the baseline file must not be created when the flag combination is rejected"
    );
}

#[test]
fn the_written_baseline_is_inert() {
    let fixture = fixture_path();
    let fixture = fixture.to_str().expect("utf-8 path");
    let baseline = baseline_path("inert");
    let payload = fixture_payload_substring();

    run(&[
        "check",
        fixture,
        "--write-baseline",
        baseline.to_str().expect("utf-8 path"),
        "--quiet",
    ]);

    let baseline_text = std::fs::read_to_string(&baseline).expect("baseline must be readable");
    assert!(
        !baseline_text.contains(&payload),
        "the baseline must store a hash, never the payload verbatim — json is a scanned \
         extension by default, and a committed baseline full of payloads would itself become \
         a finding source: {baseline_text}"
    );

    let (code, _, _) = run(&[
        "check",
        baseline.to_str().expect("utf-8 path"),
        "--include",
        "**/*.json",
        "--quiet",
    ]);
    assert_eq!(
        code, 0,
        "scanning the baseline file itself must find nothing — this is D-1's core \
         justification, tested rather than asserted in prose"
    );

    let _ = std::fs::remove_file(&baseline);
}

#[test]
fn the_path_key_survives_the_leading_dot_slash_prefix_from_check_dot() {
    let tmp = std::env::temp_dir().join(format!(
        "injscan-baseline-dotdir-{}-{}",
        std::process::id(),
        unique_suffix()
    ));
    std::fs::create_dir_all(&tmp).expect("create tempdir");
    let line = fixture_payload_substring();
    std::fs::write(tmp.join("skill.md"), format!("{line}\n")).expect("write");

    // `check .` from inside the temp directory — exactly the shape the
    // installed pre-commit hook uses.
    let out = Command::new(binary())
        .current_dir(&tmp)
        .args(["check", ".", "--write-baseline", "baseline.json", "--quiet"])
        .output()
        .expect("run");
    assert_eq!(out.status.code(), Some(0));

    let out = Command::new(binary())
        .current_dir(&tmp)
        .args([
            "check",
            ".",
            "--baseline",
            "baseline.json",
            "--format",
            "json",
        ])
        .output()
        .expect("run");
    assert_eq!(
        out.status.code(),
        Some(0),
        "the baseline written by `check .` must be usable by a later `check .`"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let reports = parsed.as_array().expect("array");
    let skill_report = reports
        .iter()
        .find(|r| {
            r["file"]
                .as_str()
                .map(|f| f.ends_with("skill.md"))
                .unwrap_or(false)
        })
        .expect("skill.md report present");
    assert!(
        !skill_report["baselined"]
            .as_array()
            .expect("array")
            .is_empty(),
        "the ./-prefixed path that `check .` reports must match the entry a baseline written \
         the same way recorded: {skill_report}"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn baselined_findings_carry_full_evidence_and_the_top_level_stays_an_array() {
    let fixture = fixture_path();
    let fixture = fixture.to_str().expect("utf-8 path");
    let baseline = baseline_path("evidence");

    run(&[
        "check",
        fixture,
        "--write-baseline",
        baseline.to_str().expect("utf-8 path"),
        "--quiet",
    ]);

    let (code, stdout, _) = run(&[
        "check",
        fixture,
        "--baseline",
        baseline.to_str().expect("utf-8 path"),
        "--format",
        "json",
    ]);
    assert_eq!(code, 0);

    let parsed: Vec<injection_scanner::pattern::ScanReport> = serde_json::from_str(&stdout)
        .expect("the top level must parse as Vec<ScanReport>, not a bare Value");
    let report = parsed
        .iter()
        .find(|r| r.file == fixture)
        .expect("report present");
    assert!(report.matches.is_empty());
    assert!(!report.baselined.is_empty());
    let evidence = &report.baselined[0];
    assert!(
        !evidence.matched_text.is_empty(),
        "a baselined record must keep the evidence, not just an id"
    );
    assert!(!evidence.message.is_empty());
    assert!(!evidence.pattern_name.is_empty());
    assert!(!evidence.remediation.is_empty());
    assert_eq!(report.critical_count, 0);

    let _ = std::fs::remove_file(&baseline);
}
