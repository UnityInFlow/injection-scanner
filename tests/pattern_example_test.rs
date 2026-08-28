//! Every pattern carries a worked example, and the example is binding.
//!
//! `docs/PATTERN-CATALOGUE.md` is generated from these fields, so a stale
//! example would be a documentation lie about a security tool — the catalogue
//! would claim the scanner catches something it does not. These tests make the
//! example part of the pattern's contract instead of prose beside it.

use injection_scanner::allowlist::Suppressions;
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::Scanner;

/// Does `id` fire on `text`? Uses the full scanner, not a bare regex, so the
/// example is checked against the behaviour a user actually gets.
fn fires(id: &str, text: &str) -> bool {
    let categories = load_embedded_patterns().expect("patterns must load");
    let scanner = Scanner::new(&categories).expect("patterns must compile");
    let report = scanner.scan("example.md", text, &Suppressions::default());
    report
        .matches
        .iter()
        .chain(report.low_confidence.iter())
        .any(|m| m.pattern_id == id)
}

#[test]
fn every_pattern_carries_an_example() {
    let categories = load_embedded_patterns().expect("patterns must load");
    let missing: Vec<&str> = categories
        .iter()
        .flat_map(|c| c.patterns.iter())
        .filter(|p| p.example.as_deref().unwrap_or("").trim().is_empty())
        .map(|p| p.id.as_str())
        .collect();
    assert!(
        missing.is_empty(),
        "every pattern needs an `example:` — it is what the catalogue renders \
         and what proves the pattern does something. Missing: {missing:?}"
    );
}

#[test]
fn every_example_matches_its_own_pattern() {
    let categories = load_embedded_patterns().expect("patterns must load");
    let mut broken = Vec::new();
    for pattern in categories.iter().flat_map(|c| c.patterns.iter()) {
        if let Some(example) = pattern.example.as_deref() {
            if !fires(&pattern.id, example) {
                broken.push(format!("{} example={example:?}", pattern.id));
            }
        }
    }
    assert!(
        broken.is_empty(),
        "an example that does not match its own pattern is a documented lie. \
         Either the regex changed or the example did:\n  {}",
        broken.join("\n  ")
    );
}

#[test]
fn no_counter_example_matches_its_own_pattern() {
    let categories = load_embedded_patterns().expect("patterns must load");
    let mut broken = Vec::new();
    for pattern in categories.iter().flat_map(|c| c.patterns.iter()) {
        if let Some(counter) = pattern.counter_example.as_deref() {
            if fires(&pattern.id, counter) {
                broken.push(format!("{} counter_example={counter:?}", pattern.id));
            }
        }
    }
    assert!(
        broken.is_empty(),
        "a counter_example is the false positive this pattern must not cause. \
         These now match, so the pattern got wider:\n  {}",
        broken.join("\n  ")
    );
}
