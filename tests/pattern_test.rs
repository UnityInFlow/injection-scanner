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
    // Asserts the MECHANISM, not a grade. This pinned `PI003 == High` and broke
    // when #21 regraded it to MEDIUM — the override was working perfectly. What
    // must hold is that a per-pattern severity is parsed and differs from the
    // category default; which pattern demonstrates that is a detail of the
    // library, not of the loader.
    let overridden: Vec<_> = role_override
        .patterns
        .iter()
        .filter_map(|p| p.severity.map(|s| (p.id.as_str(), s)))
        .filter(|(_, s)| *s != role_override.default_severity)
        .collect();
    assert!(
        !overridden.is_empty(),
        "at least one role_override pattern must override the category default \
         of {:?}, or this test proves nothing",
        role_override.default_severity
    );
}

#[test]
fn test_external_patterns_empty_dir() {
    let dir = std::path::Path::new("/nonexistent");
    let result = injection_scanner::patterns::load_external_patterns(dir).categories;
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

/// The whole severity range stays populated (#21, QUAL-02).
///
/// The library shipped 30 patterns graded 14 CRITICAL / 16 HIGH, with no MEDIUM
/// and no LOW anywhere. Everything was an emergency, so nothing was, and
/// `--fail-on <severity>` had no meaningful threshold to offer.
///
/// The pressure runs one way — a new pattern's author believes in it, and the
/// category defaults are CRITICAL and HIGH — so without this the distribution
/// drifts back on its own.
#[test]
fn every_severity_level_is_populated() {
    use std::collections::BTreeMap;

    let categories = injection_scanner::patterns::load_embedded_patterns().expect("patterns");
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for category in &categories {
        for pattern in &category.patterns {
            let severity = pattern.severity.unwrap_or(category.default_severity);
            *counts.entry(format!("{severity:?}")).or_default() += 1;
        }
    }

    for level in ["Critical", "High", "Medium", "Low"] {
        assert!(
            counts.get(level).copied().unwrap_or(0) > 0,
            "no {level} patterns exist. A scanner whose every finding is an \
             emergency has no findings — see the grading criteria in \
             PATTERNS.md. Distribution: {counts:?}"
        );
    }

    // Nothing here is a target; the bound only catches a wholesale slide back to
    // "everything is CRITICAL", which is the failure this guards.
    let total: usize = counts.values().sum();
    let critical = counts.get("Critical").copied().unwrap_or(0);
    assert!(
        critical * 2 <= total,
        "{critical} of {total} patterns are CRITICAL. Grade against how much \
         benign text shares the phrasing, not against how bad the attack would \
         be — every pattern here describes a bad outcome. See PATTERNS.md."
    );
}
