//! The Aho-Corasick literal prefilter must not change detection (issue #4).
//!
//! The prefilter is an optimisation and nothing else. It decides which regexes
//! are *worth running* against a haystack; it never decides what is reported.
//! For a security scanner that distinction is the whole safety argument, so it
//! is asserted here rather than reasoned about in a PR description: every
//! assertion in this file compares the prefiltered scanner against the same
//! scanner with the prefilter switched off, over the same input, and requires
//! byte-identical reports.
//!
//! Why a whole file rather than a line in `scanner_test.rs`: the failure mode
//! being guarded against is *silent under-reporting*. A prefilter that is too
//! aggressive does not crash, does not fail a pattern's own example test, and
//! does not change any count the other suites pin — it quietly stops running a
//! regex on a document where that regex would have fired. The only thing that
//! catches it is running both paths over real, varied text and diffing the
//! output, which is what these tests do:
//!
//! - the full false-positive corpus, the attack corpus, and the documentation
//!   corpus, at both confidence thresholds and with suppressions parsed from
//!   the file exactly as the CLI parses them
//! - every embedded pattern's own `example` and `counter_example`, plus
//!   obfuscated and encoded rewrites of each, so the normalized and decode
//!   passes are exercised and not only the raw line pass
//!
//! Two further tests keep the optimisation honest in the other direction: one
//! asserts each pattern's own example survives its own literal set (a prefilter
//! that hid it would be wrong even if some other pass happened to catch it),
//! and one asserts the prefilter is not silently degenerating into a no-op.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use injection_scanner::allowlist::{parse_suppressions, Suppressions};
use injection_scanner::pattern::{Pattern, PatternCategory};
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::prefilter::required_prefixes;
use injection_scanner::scanner::Scanner;

fn categories() -> Vec<PatternCategory> {
    load_embedded_patterns().expect("embedded patterns must load")
}

fn patterns() -> Vec<Pattern> {
    categories()
        .into_iter()
        .flat_map(|c| c.patterns.into_iter())
        .collect()
}

/// The two scanners under comparison — prefiltered, and the reference path
/// with the prefilter switched off — built once for the whole process.
///
/// Hoisted deliberately. Compiling the pattern set is by far the most expensive
/// thing this crate does (`benches/scan.rs` measures ~28ms in release, and it
/// is an order of magnitude worse unoptimised), and these tests make thousands
/// of comparisons. Constructing a pair per assertion turned a ten-second suite
/// into a ten-minute one.
fn scanners() -> &'static (Scanner, Scanner) {
    static SCANNERS: OnceLock<(Scanner, Scanner)> = OnceLock::new();
    SCANNERS.get_or_init(|| {
        let categories = categories();
        let fast = Scanner::new(&categories).expect("patterns must compile");
        let reference = Scanner::new(&categories)
            .expect("patterns must compile")
            .without_prefilter();
        (fast, reference)
    })
}

/// Both scanners' verdicts on one document, rendered for comparison.
///
/// JSON rather than a hand-written comparison because `ScanMatch` grows fields
/// — `context`, `confidence` and `decode_chain` all arrived after v0.0.1 — and
/// a field-by-field assertion would go stale silently. Serialising the whole
/// report means a new field is covered the day it is added.
fn verdict(scanner: &Scanner, file: &str, content: &str, min_confidence: f32) -> String {
    let suppressions = parse_suppressions(content);
    let report = scanner.scan_with_confidence(file, content, &suppressions, min_confidence);
    serde_json::to_string(&report).expect("a report must serialise")
}

/// Assert both paths agree on `content`, at both thresholds that matter.
///
/// 0.0 is `--strict`: everything is reported regardless of markdown context, so
/// findings the default threshold files under `low_confidence` become ordinary
/// matches. Running both is not redundant — the destination arrays differ, and
/// a prefilter bug that dropped a fenced-code finding would be invisible at the
/// default threshold where that finding is withheld anyway.
fn assert_agrees(label: &str, content: &str) {
    let (fast, reference) = scanners();
    for min_confidence in [0.0_f32, 0.5, 1.0] {
        assert_eq!(
            verdict(fast, label, content, min_confidence),
            verdict(reference, label, content, min_confidence),
            "the prefiltered scanner and the unfiltered scanner disagree on {label} \
             at min_confidence {min_confidence}. The prefilter changed detection, \
             which is a defect and not a tuning choice — see issue #4."
        );
    }
}

/// Every file under `tests/corpus/`, recursively, excluding the READMEs that
/// describe the corpora rather than being specimens.
fn corpus_files() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) => panic!("corpus {} must be readable: {e}", dir.display()),
        };
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.file_name().and_then(|n| n.to_str()) != Some("README.md") {
                out.push(path);
            }
        }
    }

    let mut files = Vec::new();
    walk(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus"),
        &mut files,
    );
    walk(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"),
        &mut files,
    );
    files.sort();
    assert!(
        files.len() > 30,
        "expected the corpus and fixture directories to hold the specimens this \
         equivalence check depends on; found {}",
        files.len()
    );
    files
}

#[test]
fn prefiltered_and_unfiltered_scans_agree_on_the_whole_corpus() {
    for path in corpus_files() {
        let Ok(content) = fs::read_to_string(&path) else {
            // Binary or non-UTF-8 specimens are not this test's business; the
            // walker decides what reaches the scanner, and it hands it strings.
            continue;
        };
        assert_agrees(&path.display().to_string(), &content);
    }
}

#[test]
fn prefiltered_and_unfiltered_scans_agree_on_this_repositorys_own_documents() {
    // Real prose, written by hand, containing quoted payloads — the shape that
    // exercises the context machinery hardest. `docs/` and the root markdown
    // are the closest thing to a live corpus this repository has.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut checked = 0usize;
    for dir in [
        root.to_path_buf(),
        root.join("docs"),
        root.join("patterns/core"),
    ] {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if !path.is_file() {
                continue;
            }
            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };
            assert_agrees(&path.display().to_string(), &content);
            checked += 1;
        }
    }
    assert!(
        checked > 5,
        "expected to check this repository's own documents; checked {checked}"
    );
}

/// Rewrites that push a payload down each of the scanner's other passes.
///
/// The raw line pass is the easy one to get right. The passes that a prefilter
/// bug hides in are the ones whose haystack is *not* the source line — the
/// normalized text, a joined multi-line block, a base64 layer — because there
/// the prefilter is being asked about a string the user never wrote.
fn pass_variants(payload: &str) -> Vec<(String, String)> {
    let words: Vec<&str> = payload.split_whitespace().collect();
    let split_point = words.len() / 2;
    vec![
        ("raw".to_string(), payload.to_string()),
        // Normalized pass: confusable and zero-width obfuscation.
        (
            "obfuscated".to_string(),
            payload.replace(' ', "\u{200b} ").replace('o', "\u{03bf}"),
        ),
        // Multi-line pass: a payload wrapped across a line break.
        (
            "wrapped".to_string(),
            format!(
                "{}\n{}",
                words[..split_point].join(" "),
                words[split_point..].join(" ")
            ),
        ),
        // Decode pass: the payload only readable after base64.
        ("encoded".to_string(), base64_line(payload)),
        // Markdown context: fenced, inline and in a table cell.
        ("fenced".to_string(), format!("```\n{payload}\n```")),
        ("inline".to_string(), format!("Text `{payload}` more text.")),
        (
            "table".to_string(),
            format!("| a | b |\n|---|---|\n| {payload} | c |"),
        ),
        // A suppression directive, so the `suppressed` array is exercised too.
        (
            "suppressed".to_string(),
            format!("{payload} <!-- injection-scanner:ignore-file -->"),
        ),
    ]
}

/// Minimal standard base64, so the decode pass has something to decode.
///
/// Written out rather than pulled in as a dependency: this is eight lines and
/// the crate would be a test-only addition to the dependency tree of a tool
/// whose whole distribution story is a small static binary.
fn base64_line(input: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        let idx = [(n >> 18) & 63, (n >> 12) & 63, (n >> 6) & 63, n & 63];
        for (i, &v) in idx.iter().enumerate() {
            if i <= chunk.len() {
                out.push(ALPHABET[v as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[test]
fn prefiltered_and_unfiltered_scans_agree_on_every_pattern_example() {
    for pattern in patterns() {
        for text in [pattern.example.clone(), pattern.counter_example.clone()]
            .into_iter()
            .flatten()
        {
            for (variant, content) in pass_variants(&text) {
                assert_agrees(&format!("{}-{variant}.md", pattern.id), &content);
            }
        }
    }
}

#[test]
fn the_prefilter_never_hides_a_patterns_own_example() {
    // The equivalence tests above would catch this too, but only as a diff on
    // a serialised report. This one names the pattern and says what is wrong,
    // which is the difference between a five-minute fix and an afternoon.
    let (scanner, _) = scanners();
    for pattern in patterns() {
        let Some(example) = pattern.example.as_deref() else {
            continue;
        };
        let case_sensitive = pattern.case_sensitive.unwrap_or(false);
        let Some(literals) = required_prefixes(&pattern.pattern, case_sensitive) else {
            // No usable literal set — the pattern is never prefiltered out.
            continue;
        };
        let haystack = example.to_ascii_lowercase();
        assert!(
            literals.iter().any(|literal| haystack
                .as_bytes()
                .windows(literal.len())
                .any(|w| { w.eq_ignore_ascii_case(literal) })),
            "{} ({}): none of its {} required literal prefixes occur in its own example \
             {example:?}. Either the literal extraction is unsound or the example has \
             drifted from the pattern — see issue #4.",
            pattern.id,
            pattern.name,
            literals.len(),
        );
        // And end to end: the live scanner must still report it.
        let report =
            scanner.scan_with_confidence("example.md", example, &Suppressions::default(), 0.0);
        assert!(
            report
                .matches
                .iter()
                .chain(report.low_confidence.iter())
                .any(|m| m.pattern_id == pattern.id),
            "{} no longer fires on its own example under the prefilter: {example:?}",
            pattern.id
        );
    }
}

#[test]
fn the_prefilter_is_not_a_no_op() {
    // A prefilter that finds no literals for anything is perfectly *correct* —
    // it just runs every regex, exactly as before. It is also worthless, and it
    // would pass every other test in this file. This pins the coverage so a
    // change that silently stops extracting literals fails loudly instead of
    // quietly giving back the 200ms budget.
    let all = patterns();
    let filterable = all
        .iter()
        .filter(|p| required_prefixes(&p.pattern, p.case_sensitive.unwrap_or(false)).is_some())
        .count();
    assert!(
        filterable * 2 >= all.len(),
        "only {filterable} of {} patterns yield a usable literal prefix set. The prefilter \
         has stopped doing its job — see issue #4.",
        all.len()
    );
}
