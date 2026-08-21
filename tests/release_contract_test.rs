//! Enforces the cross-repo asset contract with `spec-ci-plugin` (issue #18).
//!
//! `04-spec-ci-plugin/src/injection-scanner.ts` downloads, `chmod +x`es and
//! executes, at a pinned tag and with no file extension:
//!
//! ```text
//! injection-scanner-<arch>-unknown-linux-musl        arch = x86_64 | aarch64
//! ```
//!
//! That coupling was documented in a comment at the top of `release.yml` and
//! enforced nowhere. A release that switched to tarballs, renamed an asset,
//! changed a target triple, or simply failed to cut the musl pair would break
//! another repository's CI — the hardest place to diagnose it.
//!
//! These tests read `release.yml` as data, not as text, and check that the
//! workflow still *produces* the exact names the consumer *requests*. They run
//! in the ordinary `cargo test` gate, so the failure lands on the pull request
//! that changes the workflow rather than on whoever next upgrades tool 04.
//!
//! The complementary half runs at release time: `verify-published-assets` in
//! `release.yml` fetches the published URLs and checks they are raw ELF binaries
//! of the right architecture. This file cannot see that — a workflow can be
//! perfectly specified and still fail to upload — and that job cannot see this,
//! since it only runs on a tag. Both are needed.

use serde_yaml::Value;

/// The asset names `spec-ci-plugin` builds its download URL from.
///
/// Changing either of these is a breaking change for tool 04, whatever this
/// repository's own version number says.
const CONSUMER_ASSETS: [&str; 2] = [
    "injection-scanner-x86_64-unknown-linux-musl",
    "injection-scanner-aarch64-unknown-linux-musl",
];

const CHECKSUM_MANIFEST: &str = "SHA256SUMS.txt";

fn release_workflow() -> Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/.github/workflows/release.yml");
    let text = std::fs::read_to_string(path).expect("release.yml must exist");
    serde_yaml::from_str(&text).expect("release.yml must be valid YAML")
}

fn job<'a>(workflow: &'a Value, name: &str) -> &'a Value {
    workflow
        .get("jobs")
        .and_then(|jobs| jobs.get(name))
        .unwrap_or_else(|| panic!("release.yml must define a `{name}` job"))
}

fn steps(job: &Value) -> &[Value] {
    job.get("steps")
        .and_then(Value::as_sequence)
        .expect("job must have steps")
        .as_slice()
}

/// Find a step whose `name` contains `needle`.
fn step_named<'a>(steps: &'a [Value], needle: &str) -> &'a Value {
    steps
        .iter()
        .find(|s| {
            s.get("name")
                .and_then(Value::as_str)
                .is_some_and(|n| n.contains(needle))
        })
        .unwrap_or_else(|| panic!("no step whose name contains {needle:?}"))
}

/// Match a shell-style glob containing `*` against a literal name.
///
/// Deliberately minimal: the point is to answer "does the workflow's upload
/// pattern still select this exact asset", and the patterns in question use
/// nothing more exotic than `*`.
fn glob_matches(pattern: &str, name: &str) -> bool {
    let mut remaining = name;
    let mut segments = pattern.split('*');

    let first = segments.next().unwrap_or("");
    match remaining.strip_prefix(first) {
        Some(rest) => remaining = rest,
        None => return false,
    }

    let mut last_was_wildcard = true;
    let mut trailing = "";
    for segment in segments {
        trailing = segment;
        last_was_wildcard = true;
        if segment.is_empty() {
            continue;
        }
        match remaining.find(segment) {
            Some(at) => {
                remaining = &remaining[at + segment.len()..];
                last_was_wildcard = false;
            }
            None => return false,
        }
    }

    // A pattern not ending in `*` must consume the whole name.
    if !trailing.is_empty() && !last_was_wildcard {
        return remaining.is_empty();
    }
    if pattern.ends_with('*') {
        return true;
    }
    remaining.is_empty()
}

fn build_matrix_targets(workflow: &Value) -> Vec<(String, bool)> {
    job(workflow, "build-binaries")
        .get("strategy")
        .and_then(|s| s.get("matrix"))
        .and_then(|m| m.get("include"))
        .and_then(Value::as_sequence)
        .expect("build-binaries must use a matrix include list")
        .iter()
        .map(|entry| {
            let target = entry
                .get("target")
                .and_then(Value::as_str)
                .expect("each matrix entry needs a target")
                .to_string();
            let experimental = entry
                .get("experimental")
                .and_then(Value::as_bool)
                .expect("each matrix entry needs an experimental flag");
            (target, experimental)
        })
        .collect()
}

fn patterns_from(step: &Value, key: &str) -> Vec<String> {
    step.get("with")
        .and_then(|w| w.get(key))
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("step must declare `{key}`"))
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

fn release_file_patterns(workflow: &Value) -> Vec<String> {
    let steps = steps(job(workflow, "release"));
    patterns_from(step_named(steps, "Create GitHub Release"), "files")
}

fn attestation_subject_patterns(workflow: &Value) -> Vec<String> {
    let steps = steps(job(workflow, "release"));
    patterns_from(step_named(steps, "Attest build provenance"), "subject-path")
}

fn selects(patterns: &[String], asset: &str) -> bool {
    patterns.iter().any(|pattern| {
        let file_part = pattern.rsplit('/').next().unwrap_or(pattern);
        glob_matches(file_part, asset)
    })
}

#[test]
fn musl_targets_are_built_and_are_not_experimental() {
    let workflow = release_workflow();
    let targets = build_matrix_targets(&workflow);

    for asset in CONSUMER_ASSETS {
        let triple = asset
            .strip_prefix("injection-scanner-")
            .expect("consumer assets are prefixed with the binary name");
        let entry = targets
            .iter()
            .find(|(t, _)| t == triple)
            .unwrap_or_else(|| {
                panic!(
                    "release.yml no longer builds `{triple}`, but spec-ci-plugin downloads \
                 `{asset}` and executes it. See issue #18."
                )
            });
        assert!(
            !entry.1,
            "`{triple}` is marked experimental, so a failure would be tolerated by \
             continue-on-error and the release would ship without an asset spec-ci-plugin \
             requires. The musl pair is hard-required. See issue #18."
        );
    }
}

#[test]
fn published_globs_still_select_the_consumer_assets() {
    let workflow = release_workflow();
    let patterns = release_file_patterns(&workflow);

    for asset in CONSUMER_ASSETS.iter().chain([&CHECKSUM_MANIFEST]) {
        assert!(
            selects(&patterns, asset),
            "no upload pattern in release.yml selects `{asset}`. Patterns are {patterns:?}. \
             spec-ci-plugin downloads this exact name from the release. See issue #18."
        );
    }
}

/// The consumer assets must also be covered by the provenance attestation.
///
/// This is a separate glob list from the upload one, and nothing keeps the two
/// in step. An asset dropped from `subject-path` still ships — silently without
/// the signed provenance the release notes tell people to verify.
#[test]
fn attestation_covers_the_consumer_assets() {
    let workflow = release_workflow();
    let patterns = attestation_subject_patterns(&workflow);

    for asset in CONSUMER_ASSETS {
        assert!(
            selects(&patterns, asset),
            "no attestation subject-path in release.yml covers `{asset}`. Patterns are \
             {patterns:?}. It would ship without the provenance the release notes instruct \
             consumers to verify. See issue #18."
        );
    }
}

#[test]
fn assets_are_raw_binaries_named_by_target_triple() {
    let workflow = release_workflow();
    let package = step_named(steps(job(&workflow, "build-binaries")), "Package");
    let script = package
        .get("run")
        .and_then(Value::as_str)
        .expect("the packaging step must be a script");

    assert!(
        script.contains(r#"OUT="injection-scanner-${{ matrix.target }}""#),
        "the packaging step no longer names the asset `injection-scanner-<target>` verbatim. \
         spec-ci-plugin builds that name itself and appends no extension. See issue #18."
    );

    for archiver in ["tar ", "tar\n", "zip ", "gzip "] {
        assert!(
            !script.contains(archiver),
            "the packaging step appears to archive the binary (`{}`). The Action chmods and \
             executes the downloaded file directly, so it must stay a raw executable. \
             See issue #18.",
            archiver.trim()
        );
    }
}

#[test]
fn a_published_asset_check_runs_after_the_release() {
    let workflow = release_workflow();
    let verify = job(&workflow, "verify-published-assets");
    let needs = verify.get("needs").expect("verify job must declare needs");

    let depends_on_release = match needs {
        Value::String(s) => s == "release",
        Value::Sequence(items) => items.iter().any(|i| i.as_str() == Some("release")),
        other => panic!("unexpected `needs` shape: {other:?}"),
    };
    assert!(
        depends_on_release,
        "verify-published-assets must run after `release`, or it checks URLs that do not \
         exist yet. See issue #18."
    );

    let script = steps(verify)
        .iter()
        .filter_map(|s| s.get("run").and_then(Value::as_str))
        .collect::<String>();
    for asset in CONSUMER_ASSETS {
        let triple = asset.strip_prefix("injection-scanner-").unwrap();
        let arch = triple.split('-').next().unwrap();
        assert!(
            script.contains(arch),
            "the published-asset check does not mention `{arch}`, so `{asset}` is not being \
             verified. See issue #18."
        );
    }
    assert!(
        script.contains(CHECKSUM_MANIFEST),
        "the published-asset check must also verify {CHECKSUM_MANIFEST}: spec-ci-plugin now \
         refuses to execute a binary it cannot check against that manifest."
    );
}

#[test]
fn glob_matcher_behaves() {
    assert!(glob_matches(
        "injection-scanner-*-unknown-linux-musl",
        "injection-scanner-x86_64-unknown-linux-musl"
    ));
    assert!(glob_matches(
        "injection-scanner-*-unknown-linux-musl",
        "injection-scanner-aarch64-unknown-linux-musl"
    ));
    assert!(!glob_matches(
        "injection-scanner-*-unknown-linux-gnu",
        "injection-scanner-x86_64-unknown-linux-musl"
    ));
    assert!(!glob_matches(
        "injection-scanner-*-unknown-linux-musl.tar.gz",
        "injection-scanner-x86_64-unknown-linux-musl"
    ));
    assert!(glob_matches("SHA256SUMS.txt", "SHA256SUMS.txt"));
    assert!(!glob_matches("SHA256SUMS.txt", "SHA256SUMS.txt.asc"));
}
