use injection_scanner::allowlist::Suppressions;
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::Scanner;

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
    let report = Scanner::new(&categories)
        .expect("patterns must compile")
        .scan(
            "tests/fixtures/clean-skill.md",
            &content,
            &Suppressions::default(),
        );
    assert!(!report.has_findings());
}

#[test]
fn test_injected_file_has_matches() {
    let content = read_fixture("injected-skill.md");
    let categories = load_embedded_patterns().unwrap();
    let report = Scanner::new(&categories)
        .expect("patterns must compile")
        .scan(
            "tests/fixtures/injected-skill.md",
            &content,
            &Suppressions::default(),
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
    let report = Scanner::new(&categories)
        .expect("patterns must compile")
        .scan(
            "tests/fixtures/injected-skill.md",
            &content,
            &Suppressions::default(),
        );
    for m in &report.matches {
        assert!(m.line > 0, "Line number should be > 0");
    }
}

#[test]
fn test_severity_counts() {
    let content = read_fixture("injected-skill.md");
    let categories = load_embedded_patterns().unwrap();
    let report = Scanner::new(&categories)
        .expect("patterns must compile")
        .scan(
            "tests/fixtures/injected-skill.md",
            &content,
            &Suppressions::default(),
        );
    assert!(
        report.critical_count > 0,
        "Expected at least 1 CRITICAL match"
    );
}

#[test]
fn test_scan_empty_content() {
    let categories = load_embedded_patterns().unwrap();
    let report = Scanner::new(&categories)
        .expect("patterns must compile")
        .scan("empty.md", "", &Suppressions::default());
    assert!(!report.has_findings());
}

#[test]
fn test_scan_content_with_only_benign_text() {
    let categories = load_embedded_patterns().unwrap();
    let report = Scanner::new(&categories)
        .expect("patterns must compile")
        .scan(
            "test.md",
            "Just a normal README with nothing suspicious.",
            &Suppressions::default(),
        );
    assert!(!report.has_findings());
}
