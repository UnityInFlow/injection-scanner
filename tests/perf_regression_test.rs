//! Guards the invariant that made the 500-file scan 47x faster (issue #29).
//!
//! The regression this defends against is specific: moving pattern compilation
//! back inside the per-file loop. That is not a subtle slowdown — before
//! `Scanner` existed, a 500-file scan cost 806ms against a 200ms budget while a
//! single 20,000-line file scanned in 19ms, because cost tracked file *count*
//! rather than content.
//!
//! These assertions are **ratios, not wall-clock bounds**. An absolute
//! millisecond threshold measured on a developer laptop is either so loose it
//! misses the regression (the regressed build was 806ms, comfortably inside a
//! "500 files under 1s" bound) or so tight it goes flaky on a loaded CI runner.
//! Comparing the cost of scanning N files against the cost of compiling the
//! pattern set once is self-calibrating: both sides move together with machine
//! speed, so the ratio holds on any hardware and in either build profile.
//!
//! The absolute PERF-01 budget is measured where it is meaningful — against the
//! release binary, in `benches/scan.rs` and in the CI performance gate.

use std::hint::black_box;
use std::time::{Duration, Instant};

use injection_scanner::allowlist::Suppressions;
use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::Scanner;

const FILE_COUNT: usize = 500;

/// How many times the cost of a single pattern-set compile a full 500-file scan
/// is allowed to reach.
///
/// Measured at ~1.8x. A build that compiles per file cannot come in under
/// `FILE_COUNT` by construction, so anything between the two separates them;
/// 50 leaves ~28x headroom for a slow or contended runner while still failing
/// 10x below the regressed value.
const MAX_SCAN_TO_COMPILE_RATIO: u32 = 50;

fn corpus(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| {
            format!(
                "# Doc {i}\n\n\
                 Some ordinary prose about the project.\n\n\
                 - a bullet\n\
                 - another bullet\n\n\
                 More prose, roughly the length of a typical spec paragraph.\n"
            )
        })
        .collect()
}

/// Median of three, so one descheduled sample cannot decide the test.
fn median_of_three(mut f: impl FnMut() -> Duration) -> Duration {
    let mut samples = [f(), f(), f()];
    samples.sort();
    samples[1]
}

fn time(mut f: impl FnMut()) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

#[test]
fn pattern_set_is_compiled_once_not_per_file() {
    let categories = load_embedded_patterns().expect("embedded patterns must load");
    let files = corpus(FILE_COUNT);

    let compile = median_of_three(|| {
        time(|| {
            black_box(Scanner::new(&categories).expect("patterns must compile"));
        })
    });

    let scanner = Scanner::new(&categories).expect("patterns must compile");
    let scan_all = median_of_three(|| {
        time(|| {
            for (i, content) in files.iter().enumerate() {
                black_box(scanner.scan(&format!("doc{i}.md"), content, &Suppressions::default()));
            }
        })
    });

    let budget = compile * MAX_SCAN_TO_COMPILE_RATIO;
    assert!(
        scan_all < budget,
        "scanning {FILE_COUNT} files took {scan_all:?}, which is more than \
         {MAX_SCAN_TO_COMPILE_RATIO}x the {compile:?} cost of compiling the pattern set once \
         (budget {budget:?}). That is the signature of pattern compilation having moved back \
         inside the per-file loop — see issue #29. Construct `Scanner` once and reuse it."
    );
}

#[test]
fn scan_cost_tracks_content_not_file_count() {
    let categories = load_embedded_patterns().expect("embedded patterns must load");
    let scanner = Scanner::new(&categories).expect("patterns must compile");

    // Same total content, split two ways. A scanner that recompiles per file
    // pays the compile cost 100 times on the right and once on the left, so the
    // two diverge; one that compiles up front sees roughly the same work.
    let one_big: String = (0..FILE_COUNT)
        .map(|i| format!("# Section {i}\n\nOrdinary prose in a long document.\n\n"))
        .collect();
    let many_small: Vec<String> = (0..100)
        .map(|i| format!("# Section {i}\n\nOrdinary prose in a long document.\n\n").repeat(5))
        .collect();

    let single = median_of_three(|| {
        time(|| {
            black_box(scanner.scan("big.md", &one_big, &Suppressions::default()));
        })
    });
    let split = median_of_three(|| {
        time(|| {
            for (i, content) in many_small.iter().enumerate() {
                black_box(scanner.scan(&format!("part{i}.md"), content, &Suppressions::default()));
            }
        })
    });

    assert!(
        split < single * 10,
        "the same content split across {} files took {split:?} versus {single:?} as one file. \
         Per-file overhead now dominates content cost — see issue #29.",
        many_small.len()
    );
}
