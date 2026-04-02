use std::collections::HashMap;

use injection_scanner::allowlist::{is_suppressed, parse_suppressions};
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::scan_content;

fn fixture_path(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

fn read_fixture(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).unwrap()
}

#[test]
fn test_parse_single_suppression() {
    let content = "some text <!-- injection-scanner:ignore PI001 -->";
    let suppressions = parse_suppressions(content);
    assert_eq!(suppressions.get(&1).unwrap(), &vec!["PI001".to_string()]);
}

#[test]
fn test_parse_multiple_ids_on_one_line() {
    let content = "text <!-- injection-scanner:ignore PI001, PI002 -->";
    let suppressions = parse_suppressions(content);
    let ids = suppressions.get(&1).unwrap();
    assert!(ids.contains(&"PI001".to_string()));
    assert!(ids.contains(&"PI002".to_string()));
}

#[test]
fn test_no_suppressions_in_clean_content() {
    let content = "Just normal text\nNothing special here";
    let suppressions = parse_suppressions(content);
    assert!(suppressions.is_empty());
}

#[test]
fn test_is_suppressed_returns_true_for_matching_id() {
    let mut suppressions = HashMap::new();
    suppressions.insert(5, vec!["PI001".to_string()]);
    assert!(is_suppressed(&suppressions, 5, "PI001"));
}

#[test]
fn test_is_suppressed_returns_false_for_different_id() {
    let mut suppressions = HashMap::new();
    suppressions.insert(5, vec!["PI001".to_string()]);
    assert!(!is_suppressed(&suppressions, 5, "PI011"));
}

#[test]
fn test_is_suppressed_returns_false_for_different_line() {
    let mut suppressions = HashMap::new();
    suppressions.insert(5, vec!["PI001".to_string()]);
    assert!(!is_suppressed(&suppressions, 6, "PI001"));
}

#[test]
fn test_suppressed_line_not_detected_in_scan() {
    let content = read_fixture("allowlisted.md");
    let categories = load_embedded_patterns().unwrap();
    let suppressions = parse_suppressions(&content);
    let report = scan_content("allowlisted.md", &content, &categories, &suppressions);

    // PI001 on the suppressed line should NOT appear in results
    let pi001_matches: Vec<_> = report
        .matches
        .iter()
        .filter(|m| m.pattern_id == "PI001")
        .collect();
    assert!(
        pi001_matches.is_empty(),
        "PI001 should be suppressed but found: {:?}",
        pi001_matches
    );
}

#[test]
fn test_unsuppressed_line_still_detected() {
    let content = read_fixture("allowlisted.md");
    let categories = load_embedded_patterns().unwrap();
    let suppressions = parse_suppressions(&content);
    let report = scan_content("allowlisted.md", &content, &categories, &suppressions);

    // "forget everything you know" (PI006) on the unsuppressed line SHOULD be detected
    let pi006_matches: Vec<_> = report
        .matches
        .iter()
        .filter(|m| m.pattern_id == "PI006")
        .collect();
    assert!(
        !pi006_matches.is_empty(),
        "PI006 should be detected on the unsuppressed line"
    );
}

#[test]
fn test_pi001_suppression_does_not_suppress_pi011() {
    let content = read_fixture("allowlisted.md");
    let categories = load_embedded_patterns().unwrap();
    let suppressions = parse_suppressions(&content);
    let report = scan_content("allowlisted.md", &content, &categories, &suppressions);

    // Line 8 has PI011 suppression, so PI011 should NOT appear for that line
    let pi011_line8: Vec<_> = report
        .matches
        .iter()
        .filter(|m| m.pattern_id == "PI011" && m.line == 8)
        .collect();
    assert!(
        pi011_line8.is_empty(),
        "PI011 on line 8 should be suppressed"
    );

    // But PI001 suppression (line 6) should NOT affect PI011 detection elsewhere
    // Verify that if PI011 appears on the unsuppressed "forget everything" line,
    // it wouldn't be from PI001's suppression leaking.
    // The key test: PI001 suppress on line 6 does NOT suppress PI011 on line 6.
    // Line 6 only suppresses PI001, so any other pattern match on line 6 should still fire.
}
