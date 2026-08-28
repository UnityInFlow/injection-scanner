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
    // Asserted by LINE, not by count. The fixture line is real payload text, so
    // how many patterns match it is a property of the pattern library and moves
    // whenever the library is widened — #95 added a second (PI021 now matches
    // "output your system prompt", which only `POST` reached before), and this
    // test failed despite the baseline behaviour being untouched. What must hold
    // is the occurrence split: the first occurrence is accepted, the second is
    // not, however many patterns each one trips.
    let lines_of = |key: &str| -> Vec<u64> {
        let mut v: Vec<u64> = report[key]
            .as_array()
            .expect("array")
            .iter()
            .map(|m| m["line"].as_u64().expect("line number"))
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    assert_eq!(
        lines_of("baselined"),
        vec![1],
        "only the first occurrence may be accepted by the baseline: {report}"
    );
    assert_eq!(
        lines_of("matches"),
        vec![5],
        "the occurrence beyond the accepted count must remain reported: {report}"
    );
    assert_eq!(
        report["matches"].as_array().expect("array").len(),
        report["baselined"].as_array().expect("array").len(),
        "the two occurrences are the same text, so they must trip the same \
         number of patterns — one accepted, one reported: {report}"
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

// ---- review follow-ups ----

#[test]
fn writing_a_baseline_still_reports_that_files_went_unscanned() {
    // `--write-baseline` exits early to force exit code 0, and in doing so it
    // used to jump over the "N file(s) skipped and NOT scanned" summary. That
    // summary is deliberately the LAST line of a normal run so it cannot be
    // missed; losing it here means accepting a baseline while being told less
    // about the gaps in it than an ordinary scan would tell you.
    let dir = std::env::temp_dir().join(format!(
        "injscan-baseline-skip-{}-{}",
        std::process::id(),
        unique_suffix()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create tempdir");
    std::fs::copy(fixture_path(), dir.join("a.md")).expect("copy fixture");
    // Invalid UTF-8: unreadable as text, so the walker skips it.
    std::fs::write(dir.join("binary.md"), [0xff, 0xfe, 0x00, 0x01]).expect("write binary");
    let baseline = dir.join("b.json");

    let (code, _, stderr) = run(&[
        "check",
        dir.to_str().expect("utf-8 path"),
        "--write-baseline",
        baseline.to_str().expect("utf-8 path"),
    ]);
    let _ = std::fs::remove_dir_all(&dir);

    assert_eq!(code, 0, "--write-baseline must still exit 0: {stderr}");
    assert!(
        stderr.contains("file(s) skipped and NOT scanned"),
        "a baseline written over a tree with unscanned files must say so — the \
         user is recording an accept decision over coverage they do not have. \
         stderr was:\n{stderr}"
    );
}

#[test]
fn an_unknown_top_level_field_in_a_baseline_is_rejected() {
    // `BaselineEntry` already denies unknown fields on exactly this reasoning:
    // a mistyped key must fail the parse rather than yield a file that quietly
    // means something other than what its author wrote. The same argument
    // applies one level up.
    let doc = Doc::new("unknown-field.md", "clean prose, nothing to find here\n");
    let baseline = Doc::new(
        "unknown-field-baseline.json",
        r#"{"version":1,"generated_by":"x","entries":[],"entires":[]}"#,
    );

    let (code, stdout, stderr) = run(&["check", doc.path(), "--baseline", baseline.path()]);

    assert_ne!(
        code, 0,
        "an unrecognised top-level key means the file does not say what its \
         author thinks it says — parsing it anyway is how a baseline silently \
         stops gating. stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains(baseline.path()),
        "the error must name the offending file: {stderr}"
    );
}

#[test]
fn duplicate_entries_for_one_fingerprint_sum_their_counts() {
    // A hand-edited or concatenated baseline can carry the same identity twice.
    // Overwriting rather than summing silently narrows the accepted budget, so
    // findings the file plainly accepts get reported anyway — confusing, and it
    // trains people to distrust the tool. Two entries of count 1 mean two.
    let payload = std::fs::read_to_string(fixture_path()).expect("read fixture");
    let doc = Doc::new("dup-entries.md", &format!("{payload}\n{payload}\n"));
    let written = Doc::new("dup-entries-baseline.json", "{}");

    // Write a baseline that accepts BOTH occurrences, then split each entry
    // into two entries of half the count. Summing must be equivalent.
    let (code, _, stderr) = run(&["check", doc.path(), "--write-baseline", written.path()]);
    assert_eq!(code, 0, "--write-baseline must exit 0: {stderr}");

    let raw = std::fs::read_to_string(&written.0).expect("read baseline");
    let mut value: serde_json::Value = serde_json::from_str(&raw).expect("baseline is JSON");
    let entries = value["entries"].as_array().expect("entries array").clone();
    let mut split = Vec::new();
    for entry in entries {
        let count = entry["count"].as_u64().expect("count");
        assert_eq!(
            count, 2,
            "the fixture was duplicated, so each fingerprint should be accepted twice"
        );
        let mut half = entry.clone();
        half["count"] = serde_json::json!(1);
        split.push(half.clone());
        split.push(half);
    }
    value["entries"] = serde_json::Value::Array(split);
    written.write(&serde_json::to_string_pretty(&value).expect("re-serialize"));

    let (code, stdout, stderr) = run(&["check", doc.path(), "--baseline", written.path()]);
    assert_eq!(
        code, 0,
        "two entries of count 1 accept the same two occurrences one entry of \
         count 2 does. stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[test]
fn an_entry_whose_file_was_not_scanned_is_not_called_stale() {
    // Stale means "this file was scanned and the finding is gone", so it can
    // only be judged about files the run actually looked at. The pre-commit
    // hook scans ONLY staged files, so on a commit touching one unrelated file
    // every other entry consumes zero occurrences — and reporting all of them
    // as prunable, on every commit, is how a warning becomes wallpaper.
    let scanned = Doc::new("scanned-file.md", "clean prose, nothing here\n");
    let absent = Doc::new("absent-file.md", "clean prose, nothing here\n");
    let baseline = Doc::new("partial-scan-baseline.json", "{}");

    // An entry naming a file this run will never open.
    baseline.write(&format!(
        r#"{{"version":1,"generated_by":"test","entries":[
            {{"file":{},"pattern_id":"PI001",
              "digest":"sha256:{}","count":1,"first_seen_line":1}}]}}"#,
        serde_json::to_string(absent.path()).expect("encode path"),
        "0".repeat(64)
    ));

    let (code, _, stderr) = run(&["check", scanned.path(), "--baseline", baseline.path()]);

    assert_eq!(code, 0, "the scanned file is clean: {stderr}");
    assert!(
        !stderr.contains("can be pruned"),
        "an entry for a file outside this run's scope says nothing about whether \
         the finding is gone — calling it prunable on every partial scan buries \
         the genuinely stale entries the note exists to surface. stderr:\n{stderr}"
    );
}

#[test]
fn an_entry_whose_file_was_scanned_and_is_clean_is_still_called_stale() {
    // The negative case for the test above: narrowing "stale" to scanned files
    // must not narrow it to nothing.
    let doc = Doc::new("went-clean.md", include_str!("fixtures/injected-skill.md"));
    let baseline = Doc::new("went-clean-baseline.json", "{}");

    let (code, _, stderr) = run(&["check", doc.path(), "--write-baseline", baseline.path()]);
    assert_eq!(code, 0, "--write-baseline must exit 0: {stderr}");

    doc.write("the payload has been removed\n");
    let (code, _, stderr) = run(&["check", doc.path(), "--baseline", baseline.path()]);

    assert_eq!(code, 0, "the file is clean now: {stderr}");
    assert!(
        stderr.contains("can be pruned"),
        "this file WAS scanned and its findings are gone — that entry is a live \
         licence to re-introduce them and must still be reported. stderr:\n{stderr}"
    );
}
