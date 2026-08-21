//! SCAN-08 (issue #28): pattern validation.
//!
//! Invalid regexes used to be warned about on stderr and silently dropped, and
//! duplicate ids were not detected at all — so a community pattern could shadow
//! a core one and emit contradictory findings under the same `pattern_id`.

use std::fs;
use std::process::Command;

use injection_scanner::pattern::{Pattern, PatternCategory, Severity};
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::Scanner;

fn binary_path() -> String {
    format!(
        "{}/target/debug/injection-scanner",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("injscan-validation-{name}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir must be creatable");
    dir
}

fn category(name: &str, id: &str, pattern: &str) -> PatternCategory {
    PatternCategory {
        category: name.to_string(),
        default_severity: Severity::Low,
        patterns: vec![Pattern {
            id: id.to_string(),
            name: "probe".to_string(),
            pattern: pattern.to_string(),
            severity: None,
            case_sensitive: None,
            description: "probe".to_string(),
            remediation: String::new(),
            tags: vec![],
        }],
    }
}

#[test]
fn duplicate_ids_are_rejected_not_silently_merged() {
    let cats = vec![
        category("core", "PI001", "alpha"),
        category("community", "PI001", "beta"),
    ];
    let (scanner, errors) = Scanner::new_lenient(&cats);

    assert_eq!(
        errors.len(),
        1,
        "the duplicate must be reported: {errors:?}"
    );
    assert!(
        format!("{}", errors[0]).contains("PI001"),
        "the error must name the id: {}",
        errors[0]
    );
    assert_eq!(
        scanner.pattern_count(),
        1,
        "only the first claim of an id is kept"
    );
}

#[test]
fn duplicate_ids_cannot_produce_contradictory_findings() {
    // The real damage: two findings, same pattern_id, different severity and
    // message, in one report — corrupting any consumer keying on pattern_id.
    let cats = vec![
        category("core", "PI001", "alpha"),
        category("community", "PI001", "beta"),
    ];
    let (scanner, _) = Scanner::new_lenient(&cats);
    let report = scanner.scan(
        "t.md",
        "alpha\nbeta",
        &injection_scanner::allowlist::Suppressions::default(),
    );
    assert!(
        report
            .matches
            .iter()
            .filter(|m| m.pattern_id == "PI001")
            .count()
            <= 1,
        "one id must not yield findings from two different definitions: {:?}",
        report.matches
    );
}

#[test]
fn embedded_patterns_have_no_duplicate_ids() {
    let categories = load_embedded_patterns().expect("patterns must load");
    let (_, errors) = Scanner::new_lenient(&categories);
    assert!(
        errors.is_empty(),
        "the shipped pattern library must be clean: {errors:?}"
    );
}

#[test]
fn unknown_yaml_fields_are_rejected() {
    // `#[serde(default)]` on every optional field meant a misspelled key such as
    // `severty:` was silently ignored, so the pattern shipped with the wrong
    // severity and nobody was told.
    let yaml = "category: t\ndefault_severity: LOW\npatterns:\n  - id: T1\n    name: t\n    pattern: x\n    severty: CRITICAL\n";
    let parsed: Result<PatternCategory, _> = serde_yaml::from_str(yaml);
    assert!(parsed.is_err(), "a misspelled field must be rejected");
}

#[test]
fn strict_patterns_flag_turns_warnings_into_failure() {
    let dir = temp_dir("strict");
    let pat_dir = dir.join("patterns");
    fs::create_dir_all(&pat_dir).unwrap();
    fs::write(
        pat_dir.join("bad.yaml"),
        "category: bad\ndefault_severity: CRITICAL\npatterns:\n  - id: PI999\n    name: bad\n    pattern: \"[invalid(\"\n    description: bad\n    remediation: x\n",
    )
    .unwrap();
    fs::write(dir.join("doc.md"), "nothing here\n").unwrap();
    let doc = dir.join("doc.md");

    let lenient = Command::new(binary_path())
        .args([
            "check",
            doc.to_str().unwrap(),
            "--patterns",
            pat_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute binary");
    assert!(
        lenient.status.success(),
        "without the flag a bad external pattern must warn, not fail"
    );

    let strict = Command::new(binary_path())
        .args([
            "check",
            doc.to_str().unwrap(),
            "--patterns",
            pat_dir.to_str().unwrap(),
            "--strict-patterns",
        ])
        .output()
        .expect("Failed to execute binary");
    assert!(
        !strict.status.success(),
        "--strict-patterns must fail on an invalid pattern"
    );
}
