//! `docs/PATTERN-CATALOGUE.md` must match the pattern library it documents.
//!
//! Without this the catalogue rots on the first pattern PR that forgets to
//! regenerate, and a rotted catalogue for a security tool is worse than none:
//! it states, in the repo's own voice, that the scanner catches something it
//! may no longer catch. The failure message names the exact command to fix it,
//! because a contributor hitting this has no reason to know it.

use injection_scanner::catalogue::{render, GENERATED_MARKER};
use injection_scanner::patterns::load_embedded_patterns;

const CATALOGUE: &str = "docs/PATTERN-CATALOGUE.md";

fn committed() -> String {
    std::fs::read_to_string(CATALOGUE)
        .unwrap_or_else(|e| panic!("{CATALOGUE} must exist and be readable: {e}"))
}

#[test]
fn the_catalogue_is_not_stale() {
    let categories = load_embedded_patterns().expect("patterns must load");
    let expected = render(&categories);
    let actual = committed();

    if actual != expected {
        // Point at the first differing line rather than dumping 1,400 lines.
        let first_diff = expected
            .lines()
            .zip(actual.lines())
            .position(|(a, b)| a != b)
            .map(|i| {
                format!(
                    "first difference at line {}:\n  expected: {:?}\n  committed: {:?}",
                    i + 1,
                    expected.lines().nth(i).unwrap_or(""),
                    actual.lines().nth(i).unwrap_or("")
                )
            })
            .unwrap_or_else(|| {
                format!(
                    "no line differs, but length does: expected {} lines, committed {}",
                    expected.lines().count(),
                    actual.lines().count()
                )
            });

        panic!(
            "{CATALOGUE} is out of date with the pattern library.\n\n\
             Regenerate it:\n\n  \
             cargo run --release -- rules --format markdown > {CATALOGUE}\n\n\
             {first_diff}"
        );
    }
}

#[test]
fn the_catalogue_is_marked_generated() {
    assert!(
        committed().contains(GENERATED_MARKER),
        "{CATALOGUE} must carry the generated-by marker, so nobody hand-edits it \
         and loses the change on the next regeneration"
    );
}

#[test]
fn every_pattern_id_appears_in_the_catalogue() {
    // Belt and braces against a renderer bug that silently drops a category:
    // the staleness test above would still pass, because it compares the
    // renderer against itself.
    let categories = load_embedded_patterns().expect("patterns must load");
    let doc = committed();
    let missing: Vec<&str> = categories
        .iter()
        .flat_map(|c| c.patterns.iter())
        .filter(|p| !doc.contains(&format!("### {} —", p.id)))
        .map(|p| p.id.as_str())
        .collect();
    assert!(
        missing.is_empty(),
        "these patterns ship but are not documented in {CATALOGUE}: {missing:?}"
    );
}
