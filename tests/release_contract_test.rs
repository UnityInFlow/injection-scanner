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
    // `split` always yields at least one element, so `parts` is never empty and
    // `first`/`last` are always valid.
    let parts: Vec<&str> = pattern.split('*').collect();
    let (Some(first), Some(last)) = (parts.first(), parts.last()) else {
        return false;
    };

    // No wildcard at all: the pattern is a literal filename.
    if parts.len() == 1 {
        return pattern == name;
    }

    // Anchor both ends before scanning the middle. Consuming the segments
    // left-to-right with `find` — as an earlier version did — lets a middle
    // search swallow the text the trailing segment needs, so `a*b` did not
    // match `abxb` and `*-unknown-linux-musl` did not match a name containing
    // that suffix twice. Those are false negatives, so the failure showed up as
    // "this asset is not in the upload list" for an asset that was.
    let Some(remaining) = name.strip_prefix(first) else {
        return false;
    };
    let Some(mut remaining) = remaining.strip_suffix(last) else {
        return false;
    };

    // Interior segments must appear in order in what is left between the
    // anchors. Greedy is safe here: the tail is already reserved.
    for segment in &parts[1..parts.len() - 1] {
        match remaining.find(*segment) {
            Some(at) => remaining = &remaining[at + segment.len()..],
            None => return false,
        }
    }

    true
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

    // A `continue-on-error: true` here would leave the job green whatever it
    // found, which is a decorative gate rather than a gate. Nothing else in the
    // workflow would notice.
    let tolerated = verify
        .get("continue-on-error")
        .map(|v| v.as_bool() != Some(false))
        .unwrap_or(false);
    assert!(
        !tolerated,
        "verify-published-assets sets `continue-on-error`, so a failed contract check would \
         not fail the workflow. The job exists to make a broken release impossible to miss; \
         tolerating its failure makes it decorative. See issue #18."
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
fn the_published_asset_check_is_read_only() {
    // This job downloads an unauthenticated artifact and executes it. That is
    // the whole point — it walks the consumer's exact path — but it means the
    // job runs code that is not gated on the repository's review process. It
    // must therefore hold nothing worth stealing: no write scope, no token
    // beyond the anonymous fetch, no secrets in its environment.
    //
    // The CI guidance in CLAUDE.md is specific about this split, and a
    // `permissions:` block is easy to widen by copying a neighbouring job.
    let workflow = release_workflow();
    let verify = job(&workflow, "verify-published-assets");

    let permissions = verify
        .get("permissions")
        .and_then(Value::as_mapping)
        .expect("verify-published-assets must declare an explicit `permissions` block");

    for (scope, level) in permissions {
        let scope = scope.as_str().unwrap_or("<non-string>");
        let level = level.as_str().unwrap_or("<non-string>");
        assert_eq!(
            level, "read",
            "verify-published-assets grants `{scope}: {level}`. The job fetches assets \
             anonymously and then executes one; it must not hold write scope on anything."
        );
    }

    let script = steps(verify)
        .iter()
        .filter_map(|s| s.get("run").and_then(Value::as_str))
        .collect::<String>();
    let env_blocks = steps(verify)
        .iter()
        .filter_map(|s| s.get("env"))
        .chain(verify.get("env"))
        .map(|v| serde_yaml::to_string(v).unwrap_or_default())
        .collect::<String>();

    for surface in [&script, &env_blocks] {
        assert!(
            !surface.contains("secrets."),
            "verify-published-assets references `secrets.`. It executes a downloaded \
             binary — nothing secret may be reachable from it."
        );
    }
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

    // A trailing literal that also occurs earlier in the name. The previous
    // matcher consumed segments left-to-right with `find`, so the middle search
    // ate the text the tail needed and these all returned false — an asset that
    // *is* covered by the upload glob reported as uncovered.
    assert!(glob_matches("a*b", "abxb"));
    assert!(glob_matches("*-musl", "a-musl-b-musl"));
    assert!(glob_matches(
        "injection-scanner-*-unknown-linux-musl",
        "injection-scanner-x86_64-unknown-linux-musl-unknown-linux-musl"
    ));

    // Anchors may not overlap: `a*a` needs at least two characters.
    assert!(!glob_matches("a*a", "a"));
    assert!(glob_matches("a*a", "aa"));

    // Degenerate patterns.
    assert!(glob_matches("*", "anything"));
    assert!(glob_matches("*", ""));
    assert!(glob_matches("prefix*", "prefix"));
    assert!(!glob_matches("prefix*", "prefi"));

    // Multiple wildcards, in order.
    assert!(glob_matches("a*b*c", "a-b-c"));
    assert!(!glob_matches("a*b*c", "a-c-b"));
}

#[test]
fn the_tag_is_checked_against_the_crate_version_before_anything_is_built() {
    // The git tag is what GitHub calls the release; `Cargo.toml` is what the
    // binary reports from `--version`. Nothing connected them. Pushing a tag
    // without the matching bump produced binaries reporting the PREVIOUS
    // version, published them under the new one, signed a provenance
    // attestation over them, and passed every gate — `verify-published-assets`
    // runs `--version` but never compared the output to anything.
    //
    // The check belongs in `test`, which every other job depends on, so a
    // mismatch costs one step rather than a full cross-compile of six targets
    // and a published release that has to be withdrawn.
    let workflow = release_workflow();
    let test_job = job(&workflow, "test");
    let guard = step_named(steps(test_job), "Tag must match the crate version");

    let script = guard
        .get("run")
        .and_then(Value::as_str)
        .expect("the tag/version guard must be a `run` step");

    assert!(
        script.contains("GITHUB_REF_NAME"),
        "the guard must read the pushed tag from GITHUB_REF_NAME"
    );
    assert!(
        script.contains("cargo metadata"),
        "the guard must read the version from cargo metadata rather than parsing Cargo.toml by hand"
    );
    assert!(
        script.contains("exit 1"),
        "the guard must fail the job on a mismatch, not merely log one"
    );

    // The package must be selected by name. `packages[0]` reads correctly with
    // one crate, but issue #41 plans a library split and cargo metadata orders
    // workspace packages alphabetically — `injection-scanner-core` would sort
    // ahead of `injection-scanner`, so the guard would compare the tag against
    // the library's version and block every correct release.
    assert!(
        !script.contains("[\"packages\"][0]") && !script.contains("packages[0]"),
        "the guard indexes into `packages`. Select by name instead: in a workspace \
         cargo metadata gives no ordering guarantee, and the first entry is not \
         necessarily the binary crate this tag is about."
    );
    assert!(
        script.contains("injection-scanner"),
        "the guard must name the package it is checking"
    );

    // It has to run before the build. `build-binaries` depends on `test`, so
    // placement inside `test` is what makes that true; assert the dependency
    // rather than trusting the job order in the file.
    let build = job(&workflow, "build-binaries");
    let needs = build
        .get("needs")
        .expect("build-binaries must declare needs");
    let gated = match needs {
        Value::String(s) => s == "test",
        Value::Sequence(items) => items.iter().any(|i| i.as_str() == Some("test")),
        other => panic!("unexpected `needs` shape: {other:?}"),
    };
    assert!(
        gated,
        "build-binaries must depend on `test`, or the tag/version guard does not gate the build"
    );
}
