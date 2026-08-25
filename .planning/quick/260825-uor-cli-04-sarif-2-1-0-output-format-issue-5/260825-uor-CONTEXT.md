# Context — CLI-04, SARIF 2.1.0 output

**Locked by the user 2026-08-25. Plan against these; do not re-open them.**

Requirement **CLI-04** (`.planning/REQUIREMENTS.md`, Phase 4): "SARIF 2.1.0 output that validates
against the schema and uploads to GitHub code scanning". GitHub issue **#5**.

This is the last unmet *original* v0.0.1 promise. `--format sarif` returning human-readable text
with a findings exit code was one of the audit findings that opened this milestone — the variant
was then deliberately withheld from the `OutputFormat` enum until a writer existed. See the doc
comment on `OutputFormat` in `src/main.rs`; adding the variant without the writer is precisely
what that comment forbids.

## D-1 — Only `matches` become SARIF results

`suppressed`, `low_confidence` and `baselined` are **not** emitted as results. SARIF carries
exactly what the exit code acts on: one result per reported finding.

Rationale: a code-scanning alert means "act on this". The three withheld arrays each mean the
opposite, for three different reasons. They stay visible in `--format json`, which is where a
consumer that wants the full picture already looks.

SARIF 2.1.0's `suppressions` property was considered and rejected for this milestone — it would
put baselined findings in front of reviewers as dismissed alerts, which is noise in the exact
workflow `--baseline` exists to quiet.

## D-2 — Branch and merge order

Stacked on `feat/cli-08-baseline` (PR #79), because both touch `OutputFormat` in `src/main.rs`
and `src/reporter.rs`.

**Hazard, recorded from experience in this repo:** it rebase-merges with zero merge commits, and
merging a parent PR with `--delete-branch` **auto-closes the stacked child**; GitHub then refuses
both `gh pr reopen` and retargeting. The child must be retargeted to `main` *before* #79's branch
is deleted. This is an orchestrator concern, not an implementation one — noted here so it is not
lost.

## Design guidance (not locked — plan it, but start here)

**Severity mapping.** SARIF `level` is one of `error` / `warning` / `note` / `none`, which does
not match this tool's four-level severity. Proposed: CRITICAL and HIGH → `error`, MEDIUM →
`warning`, LOW → `note`. The full severity must survive the lossy mapping — carry it in
`properties` and/or `rank` so a consumer can recover it. Do not silently flatten.

**Reuse the CLI-08 fingerprint for `partialFingerprints`.** SARIF uses these to keep an alert
identified across runs, so GitHub does not close and reopen the same alert when a line moves.
`baseline::fingerprint` already computes exactly the right thing — sha256 over `matched_text`,
deliberately independent of line number — and it is already public. Reusing it means the baseline
and the code-scanning alert agree on what "the same finding" means, which is a real property, not
a convenience.

**Rules.** Emit `tool.driver.rules` from the loaded pattern set: `id`, `name`,
`shortDescription`, `fullDescription`, `help` carrying the remediation, and `properties` with the
category and the native severity. Each result references its rule by `ruleId`. `explain <PI0XX>`
already assembles this material — reuse rather than re-derive.

**Validation without a network.** Tests must not fetch the schema at runtime. Prefer structural
assertions plus, if a schema check is wanted, a vendored copy — decide and justify. The real
end-to-end proof is a CI job uploading to code scanning, which is where "validates against the
2.1.0 schema" is actually settled.

## Non-negotiable constraints

- `--format json` output must not change. Top level stays an array of report objects —
  `spec-ci-plugin` does `JSON.parse(output) as Array<...>`.
- SARIF is a **new** `OutputFormat` variant; the `match` in `main.rs` is exhaustive by
  construction, which is the point of the enum.
- `#![deny(clippy::unwrap_used)]` in both crates. No `unwrap()`.
- Integration tests MUST spawn via `env!("CARGO_BIN_EXE_injection-scanner")` —
  `tests/test_harness_contract_test.rs` fails the build on a hand-built target path.
- Do not write verbatim injection payloads into new repository files; reuse `tests/fixtures/`.
  This repo scans itself.
- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` clean; never commit failing
  tests.
