use injection_scanner::pattern::{ScanMatch, ScanReport, Severity};
use injection_scanner::reporter::{format_json, format_text};

fn sample_report() -> ScanReport {
    ScanReport::new(
        "test.md".to_string(),
        vec![
            ScanMatch {
                pattern_id: "PI001".to_string(),
                pattern_name: "ignore-previous-instructions".to_string(),
                severity: Severity::Critical,
                message: "Attempts to override agent instructions".to_string(),
                remediation: "Remove instruction override text.".to_string(),
                file: "test.md".to_string(),
                line: 5,
                matched_text: "ignore all previous instructions".to_string(),
            },
            ScanMatch {
                pattern_id: "PI030".to_string(),
                pattern_name: "developer-mode".to_string(),
                severity: Severity::High,
                message: "Developer mode jailbreak".to_string(),
                remediation: "Remove developer mode activation.".to_string(),
                file: "test.md".to_string(),
                line: 10,
                matched_text: "developer mode enabled".to_string(),
            },
        ],
    )
}

fn empty_report() -> ScanReport {
    ScanReport::new("clean.md".to_string(), vec![])
}

#[test]
fn test_format_text_with_findings() {
    let report = sample_report();
    let output = format_text(&[report]);
    assert!(output.contains("test.md"));
    assert!(output.contains("PI001"));
    assert!(output.contains("PI030"));
    assert!(output.contains("CRITICAL"));
    assert!(output.contains("HIGH"));
    assert!(output.contains("2 finding(s)"));
}

#[test]
fn test_format_text_no_findings() {
    let report = empty_report();
    let output = format_text(&[report]);
    assert!(output.contains("No injection patterns detected."));
}

#[test]
fn test_format_text_shows_line_numbers() {
    let report = sample_report();
    let output = format_text(&[report]);
    assert!(output.contains(":5"));
    assert!(output.contains(":10"));
}

#[test]
fn test_format_json_returns_valid_json() {
    let report = sample_report();
    let json = format_json(&[report]).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);
}

#[test]
fn test_format_json_contains_pattern_ids() {
    let report = sample_report();
    let json = format_json(&[report]).unwrap();
    assert!(json.contains("PI001"));
    assert!(json.contains("PI030"));
}

#[test]
fn test_format_json_empty_reports() {
    let report = empty_report();
    let json = format_json(&[report]).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let arr = parsed.as_array().unwrap();
    let matches = arr[0]["matches"].as_array().unwrap();
    assert!(matches.is_empty());
}

#[test]
fn test_format_text_summary_counts() {
    let report = sample_report();
    let output = format_text(&[report]);
    assert!(output.contains("1 critical"));
    assert!(output.contains("1 high"));
    assert!(output.contains("0 medium"));
    assert!(output.contains("0 low"));
}
