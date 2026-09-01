//! Enforces the `PATTERNS.md` per-pattern test policy in CI instead of by hand
//! (#70, split out of #27).
//!
//! The policy — at least 3 match cases and 2 non-match cases per pattern — was
//! a convention a reviewer had to remember. PR #66 is the evidence for why
//! that is not enough: its patterns each shipped with positives and negatives,
//! and the negatives still did not catch that PI048's `[A-Za-z0-9+/]{48,}`
//! matched any file path over 48 characters, for 3,494 false positives on our
//! own documentation.
//!
//! So this counts cases; it cannot judge whether they are *good* cases. A
//! negative that fails on length rather than on shape proves nothing. The
//! false-positive corpus in `tests/corpus/clean/` is the gate that catches
//! that, and `PATTERNS.md` asks for near-misses rather than obvious non-matches.
//!
//! ## The ratchet
//!
//! 30 of the original patterns predated the helpers and had no cases at all.
//! The four category widenings (#80, #95, #97, #99) took that to 11 as a side
//! effect -- rewriting a pattern is the natural moment to give it cases.
//! Rather than block on backfilling them (#89) or leave the policy unenforced,
//! they are listed in [`LEGACY_UNTESTED`]. That list is a debt register with
//! three properties, each pinned by a test below: a pattern not on it must
//! comply, the list may not grow, and an entry that *starts* complying must be
//! removed from it. The last one is what makes this tighten on its own rather
//! than becoming a permanent excuse.

use injection_scanner::patterns::load_embedded_patterns;
use std::collections::BTreeMap;

/// Minimums from `PATTERNS.md`.
const MIN_POSITIVES: usize = 3;
const MIN_NEGATIVES: usize = 2;

/// Patterns that shipped before `assert_positives`/`assert_negatives` existed.
///
/// **Do not add to this list.** A new pattern without cases must gain cases,
/// not an exemption. Backfilling is tracked in #89; remove ids from here as
/// they are covered — a test below fails if you leave a compliant id behind.
const LEGACY_UNTESTED: &[&str] = &[
    "PI003", "PI011", "PI012", "PI013", "PI025", "PI030", "PI033", "PI037", "PI040", "PI041",
    "PI042",
];

/// Counts of `(positive, negative)` cases per pattern id across `tests/`.
///
/// Counts *cases*, not call sites: three ids each with one case is not the
/// same as one id with three, and the policy is about the latter.
fn case_counts() -> BTreeMap<String, (usize, usize)> {
    let mut counts: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");

    for entry in std::fs::read_dir(&dir).expect("tests/ must be readable") {
        let path = entry.expect("readable dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("test source must be readable");
        for (helper, slot) in [("assert_positives", 0usize), ("assert_negatives", 1usize)] {
            for (id, cases) in calls(&source, helper) {
                let entry = counts.entry(id).or_insert((0, 0));
                if slot == 0 {
                    entry.0 += cases;
                } else {
                    entry.1 += cases;
                }
            }
        }
    }
    counts
}

/// Finds `helper("PI0XX", &[ ... ])` calls and counts the string literals in
/// each. Deliberately simple: a hand-rolled scan beats a regex dependency for
/// a shape this fixed, and anything it cannot parse counts as zero, which
/// fails safe — the policy test complains rather than passing silently.
fn calls(source: &str, helper: &str) -> Vec<(String, usize)> {
    let needle = format!("{helper}(");
    let mut out = Vec::new();
    let bytes = source.as_bytes();
    let mut from = 0;

    while let Some(rel) = source[from..].find(&needle) {
        let start = from + rel + needle.len();
        from = start;

        let Some(open) = source[start..].find('"') else {
            continue;
        };
        let id_start = start + open + 1;
        let Some(close) = source[id_start..].find('"') else {
            continue;
        };
        let id = &source[id_start..id_start + close];
        if !(id.len() == 5 && id.starts_with("PI") && id[2..].bytes().all(|b| b.is_ascii_digit())) {
            continue;
        }

        // Count literals between the `&[` and its matching `]`.
        let Some(bracket) = source[id_start + close..].find('[') else {
            continue;
        };
        let body_start = id_start + close + bracket + 1;
        let mut depth = 1usize;
        let mut i = body_start;
        let mut cases = 0usize;
        let mut in_string = false;
        let mut escaped = false;

        while i < bytes.len() && depth > 0 {
            let c = bytes[i] as char;
            if in_string {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    in_string = false;
                }
            } else {
                match c {
                    '"' => {
                        in_string = true;
                        cases += 1;
                    }
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => {}
                }
            }
            i += 1;
        }
        out.push((id.to_string(), cases));
    }
    out
}

fn all_ids() -> Vec<String> {
    load_embedded_patterns()
        .expect("patterns must load")
        .iter()
        .flat_map(|c| c.patterns.iter())
        .map(|p| p.id.clone())
        .collect()
}

#[test]
fn every_pattern_meets_the_test_case_policy() {
    let counts = case_counts();
    let mut failures = Vec::new();

    for id in all_ids() {
        if LEGACY_UNTESTED.contains(&id.as_str()) {
            continue;
        }
        let (pos, neg) = counts.get(&id).copied().unwrap_or((0, 0));
        if pos < MIN_POSITIVES || neg < MIN_NEGATIVES {
            failures.push(format!("{id}: {pos} positive(s), {neg} negative(s)"));
        }
    }

    assert!(
        failures.is_empty(),
        "PATTERNS.md requires at least {MIN_POSITIVES} match cases and {MIN_NEGATIVES} \
         non-match cases per pattern, added with assert_positives / assert_negatives in \
         tests/pattern_test.rs.\n\nBelow the bar:\n  {}\n\nMake the negatives near-misses \
         — legitimate text the pattern could plausibly fire on. A negative that fails for \
         a trivial reason proves nothing.",
        failures.join("\n  ")
    );
}

#[test]
fn the_legacy_exemption_list_does_not_grow() {
    assert!(
        LEGACY_UNTESTED.len() <= 30,
        "LEGACY_UNTESTED is a debt register, not an escape hatch — it had 30 entries when \
         #70 landed and must only ever shrink. A new pattern needs test cases, not an \
         exemption."
    );
    let ids = all_ids();
    let stale: Vec<&str> = LEGACY_UNTESTED
        .iter()
        .copied()
        .filter(|id| !ids.contains(&id.to_string()))
        .collect();
    assert!(
        stale.is_empty(),
        "LEGACY_UNTESTED names patterns that no longer exist: {stale:?}. Remove them."
    );
}

#[test]
fn a_legacy_pattern_that_now_complies_must_leave_the_list() {
    // Without this the list would be a ratchet that never tightens: someone
    // backfills the cases, the exemption stays, and the next change to that
    // pattern is unguarded again.
    let counts = case_counts();
    let graduated: Vec<String> = LEGACY_UNTESTED
        .iter()
        .filter(|id| {
            let (pos, neg) = counts.get(**id).copied().unwrap_or((0, 0));
            pos >= MIN_POSITIVES && neg >= MIN_NEGATIVES
        })
        .map(|id| (*id).to_string())
        .collect();

    assert!(
        graduated.is_empty(),
        "these patterns now meet the policy and must be removed from LEGACY_UNTESTED \
         so they stay enforced: {graduated:?}"
    );
}

// -------------------------------------- D-09: PI050+ relaxed_pattern ratchet
//
// GATE-05 (issue #33) requires new patterns in the PI050 range and above to
// carry `relaxed_pattern`, so their false-positive control is mutation-tested
// by `tests/pattern_relaxed_control_test.rs` rather than merely asserted in a
// PR description. The existing 48 patterns (PI001-PI049) stay exempt — D-09
// decided a 48-file retrofit migration does not belong inside a single
// category PR (GATE-04 discipline). Unlike `LEGACY_UNTESTED` above, there is
// no exemption list here: every id from PI050 up is new, so the rule is
// unconditional.
//
// This test is deliberately vacuous until Plan 05 ships the first pattern in
// the PI050 range — at this commit `all_ids()` contains nothing in that
// range, so the filter below matches zero patterns and the assertion passes
// trivially. What stops that vacuity from hiding a broken check is
// `tests/pattern_relaxed_control_test.rs`'s own mechanism self-test, which
// proves the relaxed_pattern -> Scanner pairing works via a synthetic
// fixture independent of any shipped pattern. CAT-02 and CAT-03 inherit this
// requirement automatically, since it keys on id range rather than category.
#[test]
fn every_pi05x_pattern_carries_a_relaxed_pattern() {
    let categories = load_embedded_patterns().expect("patterns must load");
    let missing: Vec<&str> = categories
        .iter()
        .flat_map(|c| c.patterns.iter())
        .filter(|p| {
            let Some(num) = p.id.strip_prefix("PI").and_then(|n| n.parse::<u32>().ok()) else {
                return false;
            };
            (50..=59).contains(&num)
        })
        .filter(|p| p.relaxed_pattern.as_deref().unwrap_or("").trim().is_empty())
        .map(|p| p.id.as_str())
        .collect();

    assert!(
        missing.is_empty(),
        "every pattern from PI050 up needs a `relaxed_pattern:` — GATE-05 requires its \
         false-positive control to be mutation-tested, not merely asserted. Missing: {missing:?}"
    );
}
