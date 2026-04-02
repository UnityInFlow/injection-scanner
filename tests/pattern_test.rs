use injection_scanner::pattern::Severity;

#[test]
fn test_load_embedded_patterns() {
    let categories = injection_scanner::patterns::load_embedded_patterns().unwrap();
    assert!(categories.len() >= 5, "Expected at least 5 categories");
}

#[test]
fn test_total_pattern_count() {
    let categories = injection_scanner::patterns::load_embedded_patterns().unwrap();
    let total: usize = categories.iter().map(|c| c.patterns.len()).sum();
    assert!(total >= 30, "Expected at least 30 patterns, got {}", total);
}

#[test]
fn test_severity_defaults() {
    let categories = injection_scanner::patterns::load_embedded_patterns().unwrap();
    let role_override = categories
        .iter()
        .find(|c| c.category == "role_override")
        .unwrap();
    assert_eq!(role_override.default_severity, Severity::Critical);
}

#[test]
fn test_severity_override() {
    let categories = injection_scanner::patterns::load_embedded_patterns().unwrap();
    let role_override = categories
        .iter()
        .find(|c| c.category == "role_override")
        .unwrap();
    let pi003 = role_override
        .patterns
        .iter()
        .find(|p| p.id == "PI003")
        .unwrap();
    assert_eq!(pi003.severity, Some(Severity::High));
}

#[test]
fn test_external_patterns_empty_dir() {
    let dir = std::path::Path::new("/nonexistent");
    let result = injection_scanner::patterns::load_external_patterns(dir).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_all_patterns_have_ids() {
    let categories = injection_scanner::patterns::load_embedded_patterns().unwrap();
    for cat in &categories {
        for pattern in &cat.patterns {
            assert!(
                !pattern.id.is_empty(),
                "Pattern missing ID in category {}",
                cat.category
            );
            assert!(
                pattern.id.starts_with("PI"),
                "Pattern ID should start with PI: {}",
                pattern.id
            );
        }
    }
}

#[test]
fn test_all_patterns_have_remediation() {
    let categories = injection_scanner::patterns::load_embedded_patterns().unwrap();
    for cat in &categories {
        for pattern in &cat.patterns {
            assert!(
                !pattern.remediation.is_empty(),
                "Pattern {} missing remediation",
                pattern.id
            );
        }
    }
}
