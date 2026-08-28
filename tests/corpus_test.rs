//! The false-positive corpus (QUAL-03).
//!
//! Per-pattern negative tests prove a pattern rejects the cases its author
//! thought of. They do not prove it survives contact with documents nobody
//! wrote for it, and the difference is not academic: `PI048`
//! (`[A-Za-z0-9+/]{48,}`) was proposed with three negative tests and still
//! produced 3,494 false positives on this project's own documentation, because
//! `/` is a base64 character and every file path over 48 characters matched.
//! Its negatives — `shortToken123`, `abcd`, `not-base64-at-all!!!` — all failed
//! on *length*, so none of them could have caught a failure of *shape*.
//!
//! Two corpora, and the distinction between them is enforced in both
//! directions:
//!
//! | | default | `--strict` |
//! |---|---|---|
//! | `clean/` | 0 findings | 0 findings — nothing matches at all |
//! | `documentation/` | 0 findings | > 0 findings — context awareness is load-bearing |
//!
//! The second row is what stops the corpus from being gamed. Without it, a
//! false positive could be "fixed" by widening the context downgrade until it
//! swallowed real attacks, and every assertion here would still pass.

use std::fs;
use std::path::{Path, PathBuf};

use injection_scanner::allowlist::Suppressions;
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::Scanner;

fn scanner() -> Scanner {
    Scanner::new(&load_embedded_patterns().expect("embedded patterns must load"))
        .expect("patterns must compile")
}

fn corpus(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/corpus")
        .join(name)
}

/// Every specimen in a corpus directory, sorted, excluding its README.
///
/// Every file, not only `*.md`. The corpus has to cover the formats the walker
/// covers, and #23 widened that to JSON, HTML, MDX and the rest — a corpus that
/// silently globbed one extension would have gone quiet about exactly the file
/// types being added.
///
/// The README explains the corpus rather than being a specimen, and it quotes
/// the payloads it is describing.
fn specimens(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("corpus {} must be readable: {e}", dir.display()))
        .map(|entry| entry.expect("directory entry").path())
        .filter(|p| p.is_file())
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("README.md"))
        .collect();
    files.sort();
    assert!(
        !files.is_empty(),
        "corpus {} is empty — this guard would pass vacuously",
        dir.display()
    );
    files
}

/// `(reported, withheld)` counts for one file at the given threshold.
fn counts(path: &Path, min_confidence: f32) -> (usize, usize) {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{} must be valid UTF-8: {e}", path.display()));
    let report = scanner().scan_with_confidence(
        &path.to_string_lossy(),
        &content,
        &Suppressions::default(),
        min_confidence,
    );
    (report.matches.len(), report.low_confidence.len())
}

/// The headline property: realistic documents produce nothing to act on.
#[test]
fn the_clean_corpus_reports_nothing() {
    let mut failures = Vec::new();
    for path in specimens(&corpus("clean")) {
        let (reported, _) = counts(&path, injection_scanner::context::DEFAULT_MIN_CONFIDENCE);
        if reported > 0 {
            failures.push(format!(
                "  {}: {reported} finding(s)",
                path.file_name().and_then(|n| n.to_str()).unwrap_or("?")
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "documents in tests/corpus/clean must produce zero findings. Each one is \
         modelled on a real false positive; a hit here means a pattern regressed \
         onto ordinary documentation:\n{}",
        failures.join("\n")
    );
}

/// Stronger, and the one that would have caught PI048.
///
/// `clean/` must not merely be *withheld* — nothing in it may match at all. A
/// pattern that fires on a Kotlin import and is then silenced by a code-fence
/// downgrade is still a broken pattern; it will fire the moment the same path
/// appears in prose, which is where paths usually appear.
#[test]
fn the_clean_corpus_matches_nothing_even_under_strict() {
    let mut failures = Vec::new();
    for path in specimens(&corpus("clean")) {
        let (reported, withheld) = counts(&path, 0.0);
        if reported + withheld > 0 {
            failures.push(format!(
                "  {}: {} match(es) under --strict",
                path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                reported + withheld
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "documents in tests/corpus/clean must not match ANY pattern, not even one \
         that context awareness would withhold. Being saved by a code-fence \
         downgrade is not the same as being correct — the same text in prose \
         would fire:\n{}",
        failures.join("\n")
    );
}

/// Writing about injection must not be reported as injection.
#[test]
fn the_documentation_corpus_reports_nothing() {
    let mut failures = Vec::new();
    for path in specimens(&corpus("documentation")) {
        let (reported, _) = counts(&path, injection_scanner::context::DEFAULT_MIN_CONFIDENCE);
        if reported > 0 {
            failures.push(format!(
                "  {}: {reported} finding(s)",
                path.file_name().and_then(|n| n.to_str()).unwrap_or("?")
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "documents in tests/corpus/documentation quote payloads in code blocks, \
         inline spans and tables, and must report zero at the default \
         threshold:\n{}",
        failures.join("\n")
    );
}

/// The anti-gaming clause.
///
/// If a documentation specimen is also clean under `--strict`, context awareness
/// is not what is keeping it clean and the file has stopped testing anything.
/// That matters because the cheapest way to silence a false positive is to widen
/// the context downgrade — and this is the assertion that notices.
#[test]
fn the_documentation_corpus_depends_on_context_awareness() {
    let mut inert = Vec::new();
    for path in specimens(&corpus("documentation")) {
        let (reported, withheld) = counts(&path, 0.0);
        if reported + withheld == 0 {
            inert.push(
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string(),
            );
        }
    }
    assert!(
        inert.is_empty(),
        "these documentation specimens match nothing even under --strict, so they \
         no longer prove context awareness is doing anything. Either the payloads \
         they quote have drifted out of the pattern set, or a pattern was \
         narrowed. Give them real payloads or move them to tests/corpus/clean: {}",
        inert.join(", ")
    );
}

/// A ratchet on pattern coverage.
///
/// `examples/*-attack.md` is the corpus that proves patterns still *fire*, and
/// `tests/markdown_context_test.rs` pins its exact counts. This asserts the
/// complementary thing: which patterns that corpus reaches at all.
///
/// It matters because the false-positive corpus above creates a pressure. The
/// cheapest way to make a document clean is to narrow the pattern that flagged
/// it — and if that pattern was never exercised by an example, narrowing it into
/// uselessness costs nothing and no test notices. This is the test that notices.
///
/// A ratchet rather than a hard gate: seven patterns are already unexercised,
/// and fixing that is issue #29, not this change. New patterns may not join
/// them.
#[test]
fn no_new_pattern_escapes_the_attack_corpus() {
    /// Patterns no `examples/*-attack.md` currently reaches.
    ///
    /// Five of these — every one but `PI002` — additionally have no unit test
    /// naming them, so they have no coverage anywhere. All were verified by
    /// hand to fire on a payload matching their description at the time this
    /// list was written; they are untested, not broken. Shrinking this list is
    /// always welcome. Growing it requires deleting this comment, which is the
    /// point.
    const UNEXERCISED: &[&str] = &[
        "PI002", // ignore-prior-context
        // PI021 came off this list in #95: widening it from the single verb
        // `POST` to a verb x object matrix means `examples/exfiltration-attack.md`
        // now reaches it. The list shrinking is the ratchet working.
        "PI025", // fetch-url
        "PI035", // jailbreak-prompt
        "PI040", // unicode-rtl-override
        "PI041", // zero-width-chars
        "PI042", // zero-width-sequence
    ];

    let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let scanner = scanner();

    let mut fired = std::collections::BTreeSet::new();
    let mut checked = 0;
    for entry in fs::read_dir(&examples).expect("examples/ must be readable") {
        let path = entry.expect("directory entry").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.ends_with("-attack.md") {
            continue;
        }
        checked += 1;
        let content = fs::read_to_string(&path).expect("example must be readable");
        // Threshold 0.0: an example that quotes its payload in a fenced block is
        // still exercising the pattern. This asks whether the regex reaches the
        // text, not whether the finding would be reported.
        let report = scanner.scan_with_confidence(name, &content, &Suppressions::default(), 0.0);
        for m in report.matches.iter().chain(report.low_confidence.iter()) {
            fired.insert(m.pattern_id.clone());
        }
    }
    assert!(checked > 0, "found no attack examples — guard is vacuous");

    let all: std::collections::BTreeSet<String> = load_embedded_patterns()
        .expect("patterns")
        .iter()
        .flat_map(|c| c.patterns.iter().map(|p| p.id.clone()))
        .collect();

    let missing: Vec<_> = all
        .difference(&fired)
        .filter(|id| !UNEXERCISED.contains(&id.as_str()))
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "these patterns fire on no attack example, so nothing would notice if they          were narrowed into uselessness. Add a payload to the relevant          examples/*-attack.md: {}",
        missing.join(", ")
    );

    // The ratchet only ratchets if it tightens. A pattern that gained coverage
    // must leave the list, or the list slowly stops meaning anything.
    let stale: Vec<_> = UNEXERCISED
        .iter()
        .filter(|id| fired.contains(**id))
        .collect();
    assert!(
        stale.is_empty(),
        "these are listed as unexercised but an attack example now reaches them.          Remove them from UNEXERCISED: {stale:?}"
    );
}
