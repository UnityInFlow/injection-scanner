//! The D-07 mutation-pairing gate for `Pattern::relaxed_pattern` (GATE-05,
//! issue #33).
//!
//! Two of four v0.1.0 category widenings shipped with a false-positive
//! control the corpus was not actually holding:
//!
//! - **#95.** PI021's disclosure arms depend on requiring the possessive
//!   (`your system prompt`, not `the system prompt`). Relaxing it to
//!   `(?:your|the)` left the entire clean corpus green — the near-miss it was
//!   meant to catch survived by an unrelated accident of pluralisation, not
//!   because the corpus caught it.
//! - **#97.** PI018's precedence arm produced six HIGH findings on ordinary
//!   configuration prose in its first draft. HIGH is what `install-hook`
//!   blocks commits at.
//!
//! "Break it and confirm the corpus goes red" was a PR-description ritual in
//! both cases, not a test. This file turns it into one: a pattern's
//! `relaxed_pattern` is the widened form with the narrowing removed, run
//! through a real [`Scanner`] — never a bare [`regex::Regex`] — so the
//! mutation is evaluated under exactly the compile flags, scope routing,
//! normalization and decode passes the shipped pattern gets.
//!
//! At this commit no shipped pattern carries the field (D-09 makes it
//! required only for `PI050` and above; Plan 05 ships the first). The
//! mechanism self-test below is what stops that from making this file a gate
//! that passes by iterating an empty collection.

use std::path::{Path, PathBuf};

use injection_scanner::allowlist::Suppressions;
use injection_scanner::pattern::PatternCategory;
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::Scanner;

/// Does `id` fire on `text` under `scanner`? Matches on `pattern_id` (not
/// merely "did anything fire") so an unrelated pattern firing on the same
/// text can never be mistaken for the one under test.
fn fires(scanner: &Scanner, id: &str, text: &str) -> bool {
    let report = scanner.scan("relaxed-probe.md", text, &Suppressions::default());
    report
        .matches
        .iter()
        .chain(report.low_confidence.iter())
        .any(|m| m.pattern_id == id)
}

/// Builds the shipped scanner, straight from the embedded pattern set.
fn shipped_scanner() -> Scanner {
    Scanner::new(&load_embedded_patterns().expect("embedded patterns must load"))
        .expect("embedded patterns must compile")
}

/// Builds a scanner over the **relaxed** variant of every pattern that
/// carries one, leaving every other pattern untouched.
///
/// Reusing [`Scanner`] rather than compiling a bare `regex::Regex` is the
/// point: the mutation is then evaluated under exactly the compile flags
/// (case sensitivity), scope routing (prose vs. frontmatter) and
/// normalization/decode passes the shipped pattern gets. A hand-compiled
/// regex would not carry any of that, and the pairing would be comparing two
/// different things.
fn relaxed_scanner() -> Scanner {
    let mut categories = load_embedded_patterns().expect("embedded patterns must load");
    for category in &mut categories {
        for pattern in &mut category.patterns {
            if let Some(relaxed) = pattern.relaxed_pattern.clone() {
                pattern.pattern = relaxed;
            }
        }
    }
    Scanner::new(&categories).expect("relaxed pattern set must still compile")
}

/// Every pattern across the embedded set that carries a `relaxed_pattern`.
fn patterns_with_relaxed_form() -> Vec<(String, String, String)> {
    load_embedded_patterns()
        .expect("embedded patterns must load")
        .into_iter()
        .flat_map(|c| c.patterns.into_iter())
        .filter_map(|p| {
            p.relaxed_pattern
                .clone()
                .map(|relaxed| (p.id.clone(), p.pattern.clone(), relaxed))
        })
        .collect()
}

fn corpus_clean_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/clean")
}

fn corpus_clean_specimens() -> Vec<PathBuf> {
    let mut files: Vec<_> = std::fs::read_dir(corpus_clean_dir())
        .expect("tests/corpus/clean must be readable")
        .map(|entry| entry.expect("readable dir entry").path())
        .filter(|p| p.is_file())
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("README.md"))
        .collect();
    files.sort();
    files
}

// ------------------------------------------------------ mechanism self-test

/// Anti-vacuity control (T-03-11). At this commit zero shipped patterns carry
/// `relaxed_pattern`, so every test below that iterates
/// [`patterns_with_relaxed_form`] would pass by iterating an empty
/// collection — proving nothing. This test does not depend on any shipped
/// pattern: it parses two synthetic single-pattern categories from YAML
/// literals — one narrow, one relaxed — over a fixed control string, and
/// proves the mechanism itself (parse `relaxed_pattern`, swap it in, run it
/// through a `Scanner`) actually discriminates.
///
/// Inverting the two synthetic regexes below must make this FAIL — that
/// mutation check was performed and reverted as part of landing this file
/// (see the plan SUMMARY).
#[test]
fn the_relaxation_mechanism_itself_discriminates_narrow_from_relaxed() {
    let control_text = "the corpus keeps this string clean and unremarkable";

    let narrow_yaml = "category: probe\n\
                        default_severity: LOW\n\
                        patterns:\n  \
                          - id: PIPROBE1\n    \
                            name: narrow-probe\n    \
                            pattern: \"unremarkable and clearly malicious\"\n    \
                            relaxed_pattern: \"unremarkable\"\n    \
                            description: probe\n    \
                            remediation: none\n";
    let narrow: PatternCategory = serde_yaml::from_str(narrow_yaml).expect("probe YAML parses");
    assert_eq!(
        narrow.patterns[0].relaxed_pattern.as_deref(),
        Some("unremarkable"),
        "sanity: relaxed_pattern parsed from the synthetic fixture"
    );

    let narrow_scanner =
        Scanner::new(std::slice::from_ref(&narrow)).expect("narrow probe compiles");
    assert!(
        !fires(&narrow_scanner, "PIPROBE1", control_text),
        "the narrow synthetic pattern requires \"clearly malicious\", which is \
         absent from the control text — it must stay silent"
    );

    let mut relaxed = narrow;
    relaxed.patterns[0].pattern = relaxed.patterns[0]
        .relaxed_pattern
        .clone()
        .expect("fixture carries a relaxed_pattern");
    let relaxed_scanner = Scanner::new(&[relaxed]).expect("relaxed probe compiles");
    assert!(
        fires(&relaxed_scanner, "PIPROBE1", control_text),
        "the relaxed synthetic pattern drops the \"clearly malicious\" \
         requirement and must fire on the same control text"
    );
}

// -------------------------------------------------- counter_example pairing

/// For every shipped pattern carrying `relaxed_pattern`: the shipped scanner
/// must not fire on that pattern's own `counter_example`, and the relaxed
/// scanner must. A narrowing that is not actually load-bearing — one where
/// the relaxed form fires no more widely than the shipped one on the
/// documented near-miss — fails here instead of passing by accident (#95).
#[test]
fn shipped_pattern_misses_counter_example_but_relaxed_form_catches_it() {
    let pairs = patterns_with_relaxed_form();
    if pairs.is_empty() {
        eprintln!(
            "SKIP: no embedded pattern carries relaxed_pattern yet (D-09 requires it only for \
             PI050+; Plan 05 ships the first). The mechanism self-test above is what proves \
             this gate is not vacuous in the meantime."
        );
        return;
    }

    let shipped = shipped_scanner();
    let relaxed = relaxed_scanner();
    let categories = load_embedded_patterns().expect("embedded patterns must load");
    let mut failures = Vec::new();

    for pattern in categories.iter().flat_map(|c| c.patterns.iter()) {
        let Some(_) = pattern.relaxed_pattern.as_deref() else {
            continue;
        };
        let Some(counter) = pattern.counter_example.as_deref() else {
            failures.push(format!(
                "{}: carries relaxed_pattern but no counter_example to pair it against",
                pattern.id
            ));
            continue;
        };
        if fires(&shipped, &pattern.id, counter) {
            failures.push(format!(
                "{}: the SHIPPED pattern already matches its own counter_example \
                 {counter:?} — the narrowing is not doing anything",
                pattern.id
            ));
        }
        if !fires(&relaxed, &pattern.id, counter) {
            failures.push(format!(
                "{}: the RELAXED form does not match counter_example {counter:?} — \
                 the relaxation is not actually wider than the shipped pattern",
                pattern.id
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "GATE-05 pairing failed:\n  {}",
        failures.join("\n  ")
    );
}

// ---------------------------------------------------- clean-corpus pairing

/// Set-level pairing (T-03-11's other half). The shipped set must report
/// zero findings across `tests/corpus/clean/`, and — only once at least one
/// pattern carries `relaxed_pattern` — the relaxed set must report at least
/// one. Until then this half is skipped loudly, naming the reason, rather
/// than passing silently.
#[test]
fn clean_corpus_is_held_by_the_shipped_set_and_broken_by_the_relaxed_set() {
    let specimens = corpus_clean_specimens();
    assert!(
        !specimens.is_empty(),
        "tests/corpus/clean must contain specimens — this guard would pass vacuously"
    );

    let shipped = shipped_scanner();
    let mut shipped_hits = Vec::new();
    for path in &specimens {
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("{} must be valid UTF-8: {e}", path.display()));
        let report = shipped.scan(&path.to_string_lossy(), &content, &Suppressions::default());
        if !report.matches.is_empty() {
            shipped_hits.push(format!(
                "{}: {} finding(s)",
                path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                report.matches.len()
            ));
        }
    }
    assert!(
        shipped_hits.is_empty(),
        "the shipped set must hold tests/corpus/clean clean:\n  {}",
        shipped_hits.join("\n  ")
    );

    if patterns_with_relaxed_form().is_empty() {
        eprintln!(
            "SKIP: no embedded pattern carries relaxed_pattern yet, so there is nothing that \
             would widen the relaxed set's coverage of tests/corpus/clean. D-09 requires the \
             field only for PI050+; Plan 05 ships the first pattern in that range."
        );
        return;
    }

    let relaxed = relaxed_scanner();
    let mut relaxed_hits = 0usize;
    for path in &specimens {
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("{} must be valid UTF-8: {e}", path.display()));
        let report = relaxed.scan(&path.to_string_lossy(), &content, &Suppressions::default());
        relaxed_hits += report.matches.len();
    }
    assert!(
        relaxed_hits > 0,
        "at least one relaxed pattern exists, so the relaxed set must break the clean \
         corpus somewhere — otherwise the corpus is not actually holding the narrowing"
    );
}

// -------------------------------------------------------- well-formedness

/// Every `relaxed_pattern` present must compile as a regex and must differ
/// textually from the pattern it relaxes. A relaxed form identical to the
/// shipped form is a control that cannot fail (T-03-12) — proven here by
/// asserting the property, and by construction: temporarily adding such a
/// pair to a synthetic fixture and observing the failure was performed and
/// reverted as part of landing this file (see the plan SUMMARY).
#[test]
fn every_relaxed_pattern_compiles_and_differs_from_its_own_pattern() {
    let categories = load_embedded_patterns().expect("embedded patterns must load");
    let mut failures = Vec::new();

    for pattern in categories.iter().flat_map(|c| c.patterns.iter()) {
        let Some(relaxed) = pattern.relaxed_pattern.as_deref() else {
            continue;
        };
        if let Err(e) = regex::Regex::new(relaxed) {
            failures.push(format!(
                "{}: relaxed_pattern does not compile: {e}",
                pattern.id
            ));
        }
        if relaxed == pattern.pattern.as_str() {
            failures.push(format!(
                "{}: relaxed_pattern is textually identical to pattern — a control \
                 that cannot fail",
                pattern.id
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "well-formedness failures:\n  {}",
        failures.join("\n  ")
    );
}
