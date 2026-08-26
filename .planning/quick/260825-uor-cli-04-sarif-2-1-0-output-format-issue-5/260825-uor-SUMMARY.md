---
phase: quick-260825-uor
plan: 01
subsystem: cli
tags: [rust, cli, sarif, code-scanning, clap, sha2]

requires:
  - phase: quick-260825-tc7
    provides: "baseline::fingerprint (sha256 over matched_text), the suppressed/low_confidence/baselined withheld-findings pattern on ScanReport"
provides:
  - "src/sarif.rs: SARIF 2.1.0 document model and format_sarif(reports, rules) — D-1 enforced by construction (reads ScanReport.matches only)"
  - "src/patterns/mod.rs: GradedRule and grade() moved from main.rs, pure and public, shared by rules/explain/SARIF"
  - "baseline::fingerprint promoted to pub — second consumer is SARIF partialFingerprints"
  - "OutputFormat::Sarif on `check`; Commands::Rules split onto its own RulesFormat so `rules --format sarif` stays rejected"
  - ".github/workflows/code-scanning.yml — GitHub-hosted, security-events:write, push:main/schedule/workflow_dispatch only"
  - ".github/code-scanning-baseline.json — 51 entries, keeps examples/patterns/fixtures in scope instead of excluded"
  - "docs/adr/ADR-003-sarif-output.md"
affects: [issue-5, CLI-04, code-scanning]

actuals:
  tokens: 23516
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "SARIF writer lives in its own module (src/sarif.rs), never reporter.rs — keeps 'did this change the JSON contract?' answerable by an empty diff on reporter.rs"
    - "D-1 enforced at the single construction site: format_sarif takes &[ScanReport] and reads only report.matches, so suppressed/low_confidence/baselined cannot leak in via a second code path"
    - "GradedRule/grade() pulled out of main.rs into the library (patterns::mod) so main.rs, sarif.rs and any future consumer share one source of truth instead of re-deriving effective severity"
    - "Occurrence-ordinal fingerprint: baseline::fingerprint(matched_text) + '/' + 1-based position within (file, ruleId, digest) — same grouping Baseline::from_reports already counts"
    - "security-severity + tags on the rule descriptor rather than SARIF rank, because GitHub's ingest reads the former and ignores the latter"

key-files:
  created:
    - src/sarif.rs
    - tests/sarif_test.rs
    - tests/json_contract_test.rs
    - .github/workflows/code-scanning.yml
    - .github/code-scanning-baseline.json
    - docs/adr/ADR-003-sarif-output.md
  modified:
    - src/lib.rs
    - src/main.rs
    - src/baseline.rs
    - src/patterns/mod.rs
    - README.md
    - TODO.md

key-decisions:
  - "D-1 implemented exactly, enforced at construction: format_sarif(&reports, &rules) iterates report.matches only; suppressed/low_confidence/baselined are never referenced in src/sarif.rs."
  - "Severity: CRITICAL/HIGH -> error, MEDIUM -> warning, LOW -> note (level_for, exhaustive). Native severity survives via result.properties.severity and rule properties['security-severity'] (9.0/7.0/5.0/2.0) plus the literal 'security' tag — rank rejected per the plan's design decision 2."
  - "partialFingerprints reuse baseline::fingerprint (promoted pub) with a 1-based occurrence ordinal appended, per design decision 3. Verified: two identical payloads in one file get two distinct fingerprints; prepending 10 blank lines changes neither."
  - "GradedRule and a new pure grade() moved into src/patterns/mod.rs verbatim (field order, Serialize derive unchanged); load_graded in main.rs becomes the thin load+warn wrapper around it, per design decision 4."
  - "rules gets its own RulesFormat{Text,Json} enum rather than sharing OutputFormat, per design decision 8 — 'rules --format sarif' stays rejected by clap at parse time."
  - "Validation is structural assertions (tests/sarif_test.rs) plus the real code-scanning upload — no jsonschema dependency, no vendored schema, per design decision 5."
  - "security-events:write lives only in the new code-scanning.yml (push:main / schedule / workflow_dispatch); ci.yml is untouched and its diff is asserted empty by an automated check, per design decision 6."
  - "Self-scan noise handled by a committed baseline (51 entries, examples/patterns/fixtures/corpus in scope) rather than blanket --exclude, per design decision 7. .planning/** excluded by glob as planning prose."

requirements-completed: [CLI-04]

coverage:
  - id: T1
    description: "check <path> --format sarif emits a document that parses as JSON, declares 2.1.0, and every result has ruleId/level/message.text/startLine>=1"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#check_format_sarif_emits_a_minimal_valid_document"
        status: pass
      - kind: integration
        ref: "tests/sarif_test.rs#sarif_2_1_0_required_structure_is_present"
        status: pass
    human_judgment: false
  - id: T2
    description: "Exactly one SARIF result per ScanReport.matches entry; suppressed/low_confidence/baselined produce zero results"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#suppressed_findings_produce_no_sarif_results"
        status: pass
      - kind: integration
        ref: "tests/sarif_test.rs#low_confidence_findings_produce_no_sarif_results"
        status: pass
      - kind: integration
        ref: "tests/sarif_test.rs#baselined_findings_produce_no_sarif_results"
        status: pass
    human_judgment: false
  - id: T3
    description: "Every ruleId resolves to tool.driver.rules by id and by ruleIndex"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#every_result_rule_id_resolves_by_id_and_index"
        status: pass
      - kind: integration
        ref: "tests/sarif_test.rs#rules_carry_usable_metadata"
        status: pass
    human_judgment: false
  - id: T4
    description: "Severity maps to the closed level set (CRITICAL/HIGH->error, MEDIUM->warning, LOW->note); native severity survives via properties.severity and rule security-severity"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#level_mapping_is_total_and_closed"
        status: pass
      - kind: integration
        ref: "tests/sarif_test.rs#native_severity_survives_the_lossy_mapping"
        status: pass
    human_judgment: false
  - id: T5
    description: "partialFingerprints are line-independent and distinct for two identical payloads in one file"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#fingerprints_are_line_independent_and_non_colliding"
        status: pass
    human_judgment: false
  - id: T6
    description: "artifactLocation.uri is a valid relative URI reference — no leading ./, no raw spaces or angle brackets"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#uri_hygiene_no_dot_slash_no_raw_special_characters"
        status: pass
      - kind: unit
        ref: "src/sarif.rs#tests::sanitize_uri_*"
        status: pass
    human_judgment: false
  - id: T7
    description: "Exit codes under --format sarif are identical to --format text for the same scan"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#exit_codes_are_format_independent"
        status: pass
      - kind: integration
        ref: "tests/sarif_test.rs#quiet_format_sarif_writes_nothing_to_stdout"
        status: pass
    human_judgment: false
  - id: T8
    description: "--format json output is byte-identical in shape: bare array top level, exact report/match key sets, pretty-printed"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/json_contract_test.rs#format_json_top_level_is_an_array"
        status: pass
      - kind: integration
        ref: "tests/json_contract_test.rs#format_json_report_key_set_is_exactly_pinned"
        status: pass
      - kind: integration
        ref: "tests/json_contract_test.rs#format_json_match_key_set_is_exactly_pinned"
        status: pass
      - kind: integration
        ref: "tests/json_contract_test.rs#format_json_output_stays_pretty_printed"
        status: pass
      - kind: integration
        ref: "tests/json_contract_test.rs#rules_format_json_key_set_is_exactly_pinned"
        status: pass
    human_judgment: false
  - id: T9
    description: "rules --format sarif rejected by clap at parse time"
    requirement: CLI-04
    verification:
      - kind: integration
        ref: "tests/sarif_test.rs#rules_format_sarif_is_rejected_at_parse_time"
        status: pass
    human_judgment: false
  - id: T10
    description: "Code-scanning upload runs only on a fork-unfirable trigger; ci.yml gains no write scope"
    requirement: CLI-04
    verification:
      - kind: other
        ref: "git diff --exit-code .github/workflows/ci.yml; grep for pull_request/self-hosted/orangepi/arc-runner in code-scanning.yml (excluding comments)"
        status: pass
    human_judgment: false

duration: ~70min
completed: 2026-08-25
status: complete
---

# Quick Task 260825-uor: CLI-04 SARIF 2.1.0 Output Summary

**A real `--format sarif` writer (rules, ruleIndex, occurrence-ordinal fingerprints, GitHub `security-severity`) plus a GitHub code-scanning upload workflow gated to fork-unfirable triggers, closing the last unmet original v0.0.1 promise (issue #5).**

## Performance

- **Duration:** ~70 min
- **Tasks:** 3/3 completed
- **Files:** 12 changed (6 created, 6 modified) — `git diff 79de54d..HEAD --stat`
- **Diff size:** 2145 insertions / 48 deletions, ~94KB → **23,516 tokens actual** (chars/4) against a 95,000-token estimate — the estimate was roughly 4x the realized cost
- **Commits:** 3 (feat, feat, feat) — plus this docs commit made separately by the orchestrator

## Accomplishments

- **Task 1 (tracer):** wired the thinnest complete path — `src/sarif.rs` with a minimal
  `#[derive(Serialize)]` document model (`$schema`, `version`, one `run`, `results` with
  `ruleId`/`level`/`message.text`/`startLine`), `pub fn format_sarif(reports: &[ScanReport])`,
  `OutputFormat::Sarif` wired at both `check` output sites, and `Commands::Rules` split onto its
  own `RulesFormat` so `rules --format sarif` stayed rejected by clap. One end-to-end integration
  test. Committed at `e992868`.
- **Task 2 (TDD):** wrote 14 tests in `tests/sarif_test.rs` and 5 in `tests/json_contract_test.rs`
  against Task 1's minimal writer *before* touching `src/sarif.rs` again, ran them, recorded the
  actual failures (5 failed for the stated structural reasons — see "TDD Gate Compliance" below —
  9 passed immediately), then implemented: promoted `baseline::fingerprint` to `pub`, moved
  `GradedRule`/`grade()` into `src/patterns/mod.rs`, extended the SARIF model with
  `tool.driver.rules`, `ruleIndex`, `partialFingerprints`, `properties.severity` and
  `properties["security-severity"]`, and a single percent-encoding rule for `artifactLocation.uri`.
  Re-ran: all 19 new tests green, full suite 227/227. Committed at `af9d0f8`.
- **Task 3:** generated `.github/code-scanning-baseline.json` (51 entries, verified inert and
  verified the baselined rescan reports zero matches), wrote
  `.github/workflows/code-scanning.yml` (GitHub-hosted, `security-events: write`, triggers
  `push:main`/weekly `schedule`/`workflow_dispatch` only, every action SHA-pinned —
  `github/codeql-action/upload-sarif` resolved to `v3.37.8` via `gh api`), wrote
  `docs/adr/ADR-003-sarif-output.md`, added a `### SARIF output` README section (usage, severity
  table, copyable upload fragment) plus a worked example under `## Output Examples`, and ticked
  `#5` in `TODO.md`. Committed at `8a5e94c`.

## TDD Gate Compliance (Task 2)

Per the plan's own prediction, the three withheld-array tests were expected to pass immediately
if Task 1 correctly withheld them — and they did: `suppressed_findings_produce_no_sarif_results`,
`low_confidence_findings_produce_no_sarif_results`, `baselined_findings_produce_no_sarif_results`
all passed on first run.

Two of the plan's other "fails now" predictions did **not** hold, and are recorded here rather
than silently claimed as failing-first, per the instruction to record the observed failure rather
than assume it:

- `level_mapping_is_total_and_closed` — the plan text says "Fails now: `level` exists but nothing
  pins the set." It actually **passed on first run**: Task 1's `level_for` was already exhaustive
  and correct over all four `Severity` variants, so a document exercising all four severities was
  already mapped correctly before Task 2 touched the writer. No code change was needed for this
  specific behavior.
- `sarif_2_1_0_required_structure_is_present` and `exit_codes_are_format_independent` and
  `quiet_format_sarif_writes_nothing_to_stdout` and `rules_format_sarif_is_rejected_at_parse_time`
  also passed immediately — each depends on structure or CLI plumbing Task 1 already built
  correctly (minimal-but-real, not stubbed).

The 5 tests that **did** fail first, for the stated reasons, before any Task 2 implementation:
`every_result_rule_id_resolves_by_id_and_index` and `rules_carry_usable_metadata` (no `rules`
array yet), `native_severity_survives_the_lossy_mapping` (no `properties.severity`),
`fingerprints_are_line_independent_and_non_colliding` (no `partialFingerprints`), and
`uri_hygiene_no_dot_slash_no_raw_special_characters` (`./` not stripped). These are the actual
new work in Task 2's implementation step.

This is consistent with the constraint's warning about the prior quick task's build-ahead
pattern: Task 1 here was still genuinely minimal (no rules array, no fingerprints, no properties,
no URI sanitization — all real gaps that Task 2 closed), it simply also happened to get the level
mapping right the first time because that logic was trivially exhaustive from the start.

## `--format json` Byte-Identical Proof

No golden file (per design decision 5 / the plan's own reasoning — a snapshot would put verbatim
finding text in a new repository file). Instead `tests/json_contract_test.rs` asserts the contract
explicitly: top-level array, report key set exactly
`{file, matches, suppressed, low_confidence, baselined, critical_count, high_count, medium_count,
low_count}`, match key set exactly
`{pattern_id, pattern_name, severity, message, remediation, file, line, matched_text, context,
confidence}`, output starts with `[\n` (pretty-printed), and `rules --format json` key set exactly
`{id, name, severity, category, description, remediation, pattern, tags}`. All 5 pass.
`src/reporter.rs` has an empty diff (`git diff 79de54d..HEAD -- src/reporter.rs` — empty) —
confirmed no changes were made to the JSON writer at all.

## Code-Scanning Workflow

- **Trigger:** `push: { branches: [main] }`, weekly `schedule` (`0 6 * * 1`), `workflow_dispatch`.
  Never `pull_request` — verified by an automated grep excluding comment lines.
- **Permissions:** `contents: read`, `security-events: write`.
- **Runner:** `ubuntu-latest` (GitHub-hosted) — self-hosted is impossible on this public repo
  under `allows_public_repositories: false`, same reasoning as `ci.yml`/`release.yml`.
- **Pins:** `actions/checkout@3d3c42e5...` (v7.0.1) and `dtolnay/rust-toolchain@4360b525...`
  (stable) reused verbatim from `ci.yml`; `actions/cache@55cc8345...` (v6.1.0) reused;
  `github/codeql-action/upload-sarif` resolved via `gh api repos/github/codeql-action/git/ref/tags/v3`
  → dereferenced the annotated tag object → commit `42947a340483f03ba47bb1a039b2c519aab3df85`
  (`v3.37.8`).
- **Exit-code handling:** 0/1/2 all proceed to upload (normal scan outcomes); anything else emits
  `::error::` and fails the job. No `continue-on-error: true` (PR #59's review finding, referenced
  in a comment).
- **`ci.yml` diff:** empty — `git diff --exit-code .github/workflows/ci.yml` passes.

## Task Commits

1. **Task 1: One finding, end to end, as SARIF** — `e992868` (feat)
2. **Task 2: The SARIF contract, driven by tests that fail first** — `af9d0f8` (feat)
3. **Task 3: The upload leg, plus ADR and docs** — `8a5e94c` (feat)

**Plan metadata:** commit deferred — the orchestrator handles the `.planning/` docs commit for
this quick task. Nothing under `.planning/` was staged or committed by this execution.

**PR not opened.** The branch instructions for this execution explicitly say "Do NOT push," and
`gh pr create` requires a pushed branch. Per D-2 in the plan and CONTEXT.md, this PR must also be
opened stacked on the still-unmerged `feat/cli-08-baseline` (PR #79) and retargeted to `main`
*before* #79's branch is deleted — an orchestrator-level sequencing concern outside this
execution's scope. The `pr-artifacts` verification report the plan asks to be placed in the PR
body is reproduced in this SUMMARY instead (Final Gate section below, plus the coverage table).

## Deviations from Plan

### Auto-fixed Issues

None — no Rule 1/2/3 auto-fixes were needed. The plan's own three explicit CONTEXT-overriding
decisions (private `fingerprint`, insufficient bare fingerprint, `rank` decline in favor of
`security-severity`) were followed exactly as specified in the plan body, not re-derived.

### Process deviations (not defects, documented per instructions)

**1. [Rule 4 territory, resolved without escalation] `shortDescription` and `fullDescription` use
the identical text.** The plan's action text says `shortDescription.text` comes "from the
description (fall back to the name when empty)" and separately lists `fullDescription.text`
without specifying its source. Since `GradedRule` carries only one `description` field (no
separate short/long text), both SARIF fields mirror the same value with the same fallback. This
is a plan-underspecified detail, not an architectural change, so it was resolved inline rather
than treated as a Rule 4 escalation — documented in `src/sarif.rs`'s `build_rules` rustdoc and in
ADR-003.

**2. LOW-severity test payload sourced from `tests/corpus_test.rs`, not a `tests/fixtures/` file.**
None of the 3 existing fixtures (`clean-skill.md`, `allowlisted.md`, `injected-skill.md`) trigger a
LOW-severity pattern. Rather than add a new fixture file containing an injection-shaped string
(forbidden by the "no verbatim payloads in new files" constraint) or hand-type a payload literal
in the new test source (same problem), `tests/sarif_test.rs::corpus_test_rs()` points at the
existing, already-committed `tests/corpus_test.rs`, which names PI035 unsuppressed in a source
comment (`"PI035", // jailbreak-prompt`) — scanning it like any other repository file yields a
genuine LOW finding without this quick task typing a payload anywhere. No test file in this
change contains an injection payload as a Rust string literal; every payload used to build a
temporary scanned document is extracted at runtime via `matched_text`/`matched_text_in_file`,
which run the real binary and read `matched_text` back out of its own JSON output.

## Authentication Gates

None encountered directly. The Task 3 `<precondition>` (`gh auth status` succeeds) was verified
before starting Task 3 — already authenticated as `hermanngeorge15` with `repo`/`workflow` scopes.

## Known Stubs

None. No hardcoded empty values, placeholder text, or unwired data sources were introduced.

## Threat Flags

None beyond what the plan's own `<threat_model>` already covers (T-uor-01 through T-uor-SC).
`.github/code-scanning-baseline.json` is a new committed artifact, but it is the exact
sha256-digest-only shape ADR-002 already established as inert, verified inert again here.

## Final Gate

```
cargo fmt --all -- --check                          PASS (clean; applied once mid-Task-1 and
                                                       once mid-Task-2 to fix formatting from
                                                       manual multi-line edits, both before commit)
cargo clippy --all-targets --locked -- -D warnings   PASS (zero warnings)
cargo test --locked                                  PASS — 227 passed, 0 failed, across 27 test
                                                       binaries (was 24 binaries / ~196 tests
                                                       before this quick task per the prior
                                                       SUMMARY; +2 new binaries — sarif_test.rs,
                                                       json_contract_test.rs — plus 3 new unit
                                                       tests in src/sarif.rs)
git diff --exit-code .github/workflows/ci.yml        PASS (empty diff)
git diff 79de54d..HEAD -- src/reporter.rs            PASS (empty diff)
python3 baseline-entries check                       PASS (51 entries)
check .github/code-scanning-baseline.json            PASS (No injection patterns detected, exit 0)
check . --baseline <the committed file>              PASS (0 matches across the repo)
workflow trigger/permission grep                     PASS (security-events: write present;
                                                       pull_request/self-hosted/orangepi/
                                                       arc-runner absent from executable YAML)
git log --merges feat/cli-08-baseline..HEAD          PASS (empty — history stayed linear)
```

## Self-Check: PASSED

- FOUND: src/sarif.rs
- FOUND: tests/sarif_test.rs
- FOUND: tests/json_contract_test.rs
- FOUND: .github/workflows/code-scanning.yml
- FOUND: .github/code-scanning-baseline.json
- FOUND: docs/adr/ADR-003-sarif-output.md
- FOUND: commit e992868 (feat: minimal end-to-end --format sarif writer (CLI-04))
- FOUND: commit af9d0f8 (feat: full SARIF contract — rules, ruleIndex, partialFingerprints (CLI-04))
- FOUND: commit 8a5e94c (feat: GitHub code-scanning upload for --format sarif (CLI-04, issue #5))
