use std::collections::HashMap;

use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::scan_content;

fn fixture_path(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

fn read_fixture(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).unwrap()
}

#[test]
fn test_clean_file_no_matches() {
    let content = read_fixture("clean-skill.md");
    let categories = load_embedded_patterns().unwrap();
    let report = scan_content(
        "tests/fixtures/clean-skill.md",
        &content,
        &categories,
        &HashMap::new(),
    );
    assert!(!report.has_findings());
}

#[test]
fn test_injected_file_has_matches() {
    let content = read_fixture("injected-skill.md");
    let categories = load_embedded_patterns().unwrap();
    let report = scan_content(
        "tests/fixtures/injected-skill.md",
        &content,
        &categories,
        &HashMap::new(),
    );
    assert!(report.has_findings());
    assert!(
        report.matches.len() >= 4,
        "Expected at least 4 matches, got {}",
        report.matches.len()
    );
}

#[test]
fn test_reports_correct_line_numbers() {
    let content = read_fixture("injected-skill.md");
    let categories = load_embedded_patterns().unwrap();
    let report = scan_content(
        "tests/fixtures/injected-skill.md",
        &content,
        &categories,
        &HashMap::new(),
    );
    for m in &report.matches {
        assert!(m.line > 0, "Line number should be > 0");
    }
}

#[test]
fn test_severity_counts() {
    let content = read_fixture("injected-skill.md");
    let categories = load_embedded_patterns().unwrap();
    let report = scan_content(
        "tests/fixtures/injected-skill.md",
        &content,
        &categories,
        &HashMap::new(),
    );
    assert!(
        report.critical_count > 0,
        "Expected at least 1 CRITICAL match"
    );
}

#[test]
fn test_scan_empty_content() {
    let categories = load_embedded_patterns().unwrap();
    let report = scan_content("empty.md", "", &categories, &HashMap::new());
    assert!(!report.has_findings());
}

#[test]
fn test_scan_content_with_only_benign_text() {
    let categories = load_embedded_patterns().unwrap();
    let report = scan_content(
        "test.md",
        "Just a normal README with nothing suspicious.",
        &categories,
        &HashMap::new(),
    );
    assert!(!report.has_findings());
}
