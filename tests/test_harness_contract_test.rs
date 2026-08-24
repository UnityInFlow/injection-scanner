//! Guards the invariant that let 29 tests pass against a binary they never built.
//!
//! `tests/cli_test.rs`, `pattern_validation_test.rs` and `scan_resilience_test.rs`
//! each independently grew a `binary_path()` that formatted
//! `CARGO_MANIFEST_DIR` + a hard-coded profile directory. Three copies, one bug,
//! and nothing connected them — so fixing one left the other two.
//!
//! The failure mode is silent in the direction that matters. Cargo builds the
//! binary under test into whatever target directory is in effect, but the tests
//! invoked a *different* path; if an artifact from some earlier build was
//! sitting there, they exercised it and reported green. Proven by building a
//! binary whose entire `main` was `std::process::exit(0)` into an alternate
//! `CARGO_TARGET_DIR`: all 29 tests passed.
//!
//! `env!("CARGO_BIN_EXE_<name>")` is the only correct spelling. Cargo sets it
//! per integration-test crate and points it at the artifact it just produced,
//! so it follows `CARGO_TARGET_DIR`, `--target` and the profile for free.

use std::fs;
use std::path::Path;

/// Assembled at runtime so this file does not match its own rule — the
/// self-match that bit PI012.
fn forbidden_fragments() -> Vec<String> {
    let target = "tar".to_string() + "get";
    vec![format!("{target}/debug"), format!("{target}/release")]
}

#[test]
fn no_integration_test_hard_codes_a_path_into_the_target_directory() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let forbidden = forbidden_fragments();
    // Assembled at runtime for the same reason as the fragments above.
    let binary_name: &str = &("injection".to_string() + "-scanner");
    let this_file = Path::new(file!())
        .file_name()
        .expect("file!() always has a final component")
        .to_owned();

    let mut offenders = Vec::new();
    let mut checked = 0usize;

    let entries = fs::read_dir(&tests_dir).expect("tests/ must be readable");
    for entry in entries {
        let path = entry.expect("directory entry must be readable").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        if path.file_name() == Some(this_file.as_ref()) {
            continue;
        }
        let source = fs::read_to_string(&path).expect("test source must be valid UTF-8");
        checked += 1;
        let spawns_correctly = source.contains("CARGO_BIN_EXE");

        for (index, line) in source.lines().enumerate() {
            // Prose describing this very rule has to be allowed to name it.
            // Only executable lines can actually mis-target the binary.
            if line.trim_start().starts_with("//") {
                continue;
            }
            let report = |offenders: &mut Vec<String>| {
                offenders.push(format!(
                    "{}:{}: {}",
                    path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                    index + 1,
                    line.trim()
                ));
            };

            // A target-directory path is only a hazard when it points at the
            // *executable*. A test that creates a fixture directory happening to
            // be named `target/debug` — to prove the walker refuses to descend
            // into one — is doing the opposite of misbehaving, and the first
            // version of this guard failed exactly that test.
            if forbidden.iter().any(|f| line.contains(f.as_str())) && line.contains(binary_name) {
                report(&mut offenders);
                continue;
            }

            // The direct form of the invariant, checked per FILE rather than
            // per line. Spawning through a local `binary_path()` helper is the
            // correct shape — demanding the literal on the same line as
            // `Command::new` would condemn the very fix this guard exists to
            // protect. What must not exist is a file that spawns the binary
            // without `CARGO_BIN_EXE` appearing anywhere in it, which is exactly
            // what the three broken files looked like.
            if line.contains("Command::new(") && !spawns_correctly {
                report(&mut offenders);
            }
        }
    }

    assert!(
        checked > 0,
        "found no integration tests to check — this guard would pass vacuously"
    );

    assert!(
        offenders.is_empty(),
        "{} integration test source(s) build a path into the target directory \
         instead of using env!(\"CARGO_BIN_EXE_injection-scanner\"). Such a test \
         runs whatever artifact is already on disk — it passes against a stale \
         binary and panics when none exists. Offending lines:\n  {}",
        offenders.len(),
        offenders.join("\n  ")
    );
}

/// Both crate roots must deny `clippy::unwrap_used`.
///
/// Issue #19 asked for this and was closed with only its first half done — the
/// `unwrap()` in `allowlist.rs` was replaced, but nothing was ever configured
/// to stop the next one. `cargo clippy -- -D warnings` does not cover it:
/// `unwrap_used` is a restriction lint, off by default, so a new `unwrap()` in
/// production code passed CI silently for the whole milestone.
///
/// The attribute is the enforcement, so removing it is the regression. `src/`
/// is two crates — the library and the binary — and a crate-level attribute
/// does not cross that boundary, which is why both are checked.
#[test]
fn both_crate_roots_deny_unwrap_used() {
    use std::fs;
    use std::path::Path;

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let needle = "deny(clippy::unwrap_used)";

    for root in ["lib.rs", "main.rs"] {
        let path = src.join(root);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} must be readable: {e}", path.display()));

        let declared = source.lines().any(|line| {
            let line = line.trim();
            line.starts_with("#![") && line.contains(needle)
        });

        assert!(
            declared,
            "src/{root} does not carry `#![{needle}]`. Without it a new `unwrap()` in \
             production code passes `cargo clippy -- -D warnings` silently — that lint is \
             a restriction lint and is off by default. See issue #19."
        );
    }
}
