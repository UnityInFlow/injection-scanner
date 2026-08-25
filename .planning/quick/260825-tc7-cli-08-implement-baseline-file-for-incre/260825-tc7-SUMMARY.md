---
phase: quick-260825-tc7
plan: 01
subsystem: cli
tags: [rust, cli, baseline, sha2, adoption, clap]

requires:
  - phase: Phase 4 — Integration (v0.1.0)
    provides: "--fail-on, --quiet, exit code 2, rules, explain (PR #76); the suppressed/low_confidence withheld-findings pattern ScanReport already established"
provides:
  - "src/baseline.rs: BaselineEntry/Baseline, fingerprint (sha256), normalise_file, from_reports/save/load/apply"
  - "ScanReport.baselined: Vec<ScanMatch> — a fourth withheld-findings array, plus ScanReport::with_baselined"
  - "--baseline <FILE> / --write-baseline <FILE> on `check`, mutually exclusive via clap conflicts_with"
  - "docs/adr/ADR-002-baseline-fingerprints.md and a README 'Adopting On An Existing Repository' section"
affects: [issue-25, CLI-08, spec-ci-plugin-consumer-contract]

actuals:
  tokens: 14000
  tasks: 3
  commits: 3

tech-stack:
  added: ["sha2 = \"0.10\" (RustCrypto, github.com/RustCrypto/hashes)"]
  patterns:
    - "Baseline module owns identification/mutation logic only; main.rs owns CLI plumbing (flag parsing, stdin rejection, exit codes) — same separation as scanner.rs/main.rs"
    - "Fourth withheld-findings array follows the suppressed/low_confidence precedent exactly: additive #[serde(default)] field, recomputed severity tallies via a with_* constructor, never spliced into an already-built report"

key-files:
  created:
    - src/baseline.rs
    - tests/baseline_test.rs
    - docs/adr/ADR-002-baseline-fingerprints.md
  modified:
    - Cargo.toml
    - Cargo.lock
    - src/lib.rs
    - src/pattern.rs
    - src/reporter.rs
    - src/main.rs
    - tests/report_roundtrip_test.rs
    - README.md

key-decisions:
  - "D-1 implemented exactly: sha256 digest over matched_text (sha2 0.10, provenance-verified RustCrypto/hashes), occurrence count budget, line number excluded from identity."
  - "D-2 implemented exactly: --write-baseline always exits exit::CLEAN; --baseline populates ScanReport.baselined; the two flags are mutually exclusive via clap conflicts_with (not hand-checked); nothing is dropped; text output carries a one-line count; stale entries are reported on stderr."
  - "Stale-entry note lives in main.rs (stderr, gated on !quiet), not reporter.rs — it is a run-level fact about the baseline file, not per-report data, matching the plan's own diagnosis of where the gap would be."
  - "Task 1 (the tracer) was implemented as a complete, production-quality slice — budget map, both-sides normalise_file keying, deterministic match ordering, and the stale note were all built in Task 1 rather than deferred. Task 2's adversarial tests therefore passed on first run against Task 1's code, with zero additional source changes needed."

requirements-completed: [CLI-08]

coverage:
  - id: D1
    description: "check <path> --write-baseline <FILE> scans, writes FILE, and exits 0 even with CRITICAL findings"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#the_two_command_adoption_flow_works_end_to_end"
        status: pass
    human_judgment: false
  - id: D2
    description: "check <path> --baseline <FILE> moves fingerprint-matched findings into baselined, empties matches, zeroes severity tallies, and exits 0"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#the_two_command_adoption_flow_works_end_to_end"
        status: pass
      - kind: integration
        ref: "tests/baseline_test.rs#baselined_findings_carry_full_evidence_and_the_top_level_stays_an_array"
        status: pass
    human_judgment: false
  - id: D3
    description: "Occurrence beyond an entry's count is still reported and still fails the build"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#an_occurrence_beyond_count_is_still_reported"
        status: pass
    human_judgment: false
  - id: D4
    description: "A baseline entry that matched nothing this run is surfaced as a stale note, absent when every entry matched"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#a_stale_entry_is_surfaced_and_only_when_stale"
        status: pass
    human_judgment: false
  - id: D5
    description: "Line number excluded from identity — editing lines above a finding does not invalidate its baseline entry"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#editing_lines_above_a_finding_does_not_invalidate_its_baseline_entry"
        status: pass
    human_judgment: false
  - id: D6
    description: "Malformed, missing, or unknown-version baseline is a hard error, never a silent no-op"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#a_baseline_that_is_not_json_is_a_hard_error"
        status: pass
      - kind: integration
        ref: "tests/baseline_test.rs#a_baseline_entry_missing_a_required_field_is_a_hard_error"
        status: pass
      - kind: integration
        ref: "tests/baseline_test.rs#a_baseline_with_an_unrecognised_version_is_a_hard_error"
        status: pass
      - kind: integration
        ref: "tests/baseline_test.rs#a_nonexistent_baseline_path_is_a_hard_error"
        status: pass
    human_judgment: false
  - id: D7
    description: "--baseline and --write-baseline are mutually exclusive (clap); --write-baseline against check - is rejected before stdin is read"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#baseline_and_write_baseline_together_are_rejected"
        status: pass
      - kind: integration
        ref: "tests/baseline_test.rs#write_baseline_with_stdin_is_rejected"
        status: pass
    human_judgment: false
  - id: D8
    description: "The written baseline is inert — hashed payload only, scanning it finds nothing"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#the_written_baseline_is_inert"
        status: pass
    human_judgment: false
  - id: D9
    description: "The ./ path normalisation holds for the check . invocation shape the pre-commit hook uses"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/baseline_test.rs#the_path_key_survives_the_leading_dot_slash_prefix_from_check_dot"
        status: pass
    human_judgment: false
  - id: D10
    description: "ScanReport.baselined is additive — a pre-baselined report still deserializes, and a baselined finding is identical in shape to the same finding reported"
    requirement: CLI-08
    verification:
      - kind: integration
        ref: "tests/report_roundtrip_test.rs#a_report_written_before_baselined_existed_still_loads"
        status: pass
      - kind: integration
        ref: "tests/report_roundtrip_test.rs#a_baselined_finding_and_the_same_finding_reported_are_the_identical_record"
        status: pass
    human_judgment: false
  - id: D11
    description: "ADR-002 and the README 'Adopting On An Existing Repository' section document the feature"
    requirement: CLI-08
    verification:
      - kind: other
        ref: "docs/adr/ADR-002-baseline-fingerprints.md exists; grep 'write-baseline' README.md"
        status: pass
    human_judgment: false

duration: ~55min
completed: 2026-08-25
status: complete
---

# Quick Task 260825-tc7: CLI-08 `--baseline` Summary

**`--baseline <FILE>` / `--write-baseline <FILE>` for incremental adoption, backed by sha256 fingerprints (RustCrypto/hashes-verified `sha2 0.10`) and an occurrence-count budget, closing the last open item on issue #25.**

## Performance

- **Duration:** ~55 min
- **Tasks:** 3/3 completed
- **Files modified:** 8 (2 new source files, 1 new ADR, 1 new test file, 4 modified)
- **Commits:** 3 (feat, test, docs)

## Accomplishments

- Implemented the full baseline lifecycle in a new `src/baseline.rs`: `BaselineEntry`/`Baseline`
  types, `fingerprint` (sha256 over `matched_text`, prefixed `sha256:`), `normalise_file` (strips
  one leading `./`), and `from_reports`/`save`/`load`/`apply`.
- Extended `ScanReport` with a fourth withheld-findings array, `baselined`, via a new
  `ScanReport::with_baselined` constructor that recomputes severity tallies over the reduced
  `matches` — following the exact precedent `suppressed`/`low_confidence` set.
- Wired `--baseline <FILE>` and `--write-baseline <FILE>` onto `check`, mutually exclusive via
  clap `conflicts_with`, with `--write-baseline` rejected against `check -` before stdin is read,
  and `--write-baseline` always exiting `exit::CLEAN` per D-2.
- `format_text` gained a baselined-count note, and the zero-total branch now also treats a
  fully-baselined file as "reported", not "detected".
- Wrote 13 new integration tests in `tests/baseline_test.rs` (tracer + adversarial edges) and 2
  new tests in `tests/report_roundtrip_test.rs` (backward-compat deserialization, shape symmetry).
- Wrote `docs/adr/ADR-002-baseline-fingerprints.md` and a new README "Adopting On An Existing
  Repository" section.

## sha2 Provenance Check (T-QT-SC)

**Outcome: PASS.** Before adding the dependency, ran `cargo info sha2@0.10` and confirmed
`repository: https://github.com/RustCrypto/hashes`, matching D-1's fixed choice exactly. Resolved
version is `0.10.9` (latest patch under `0.10`). Confirmed again after `cargo add`/`cargo build`
via the resulting `Cargo.lock` entry. No substitution was needed and none was considered.

## Final Stale-Entry Note Wording

Printed to **stderr**, one line per stale entry, gated on `!quiet`:

```
note: baseline entry {pattern_id} in {file} matched nothing this run and can be pruned — an entry
matching nothing is a standing licence to re-introduce the finding it once accepted.
```

Verified absent in the control case (an entry that still matched) and present only when an entry
consumed zero occurrences of its budget — `tests/baseline_test.rs::a_stale_entry_is_surfaced_and_only_when_stale`
asserts both directions so the test cannot pass vacuously.

## `./` Path Normalisation For The Pre-Commit-Hook Invocation Shape

**Held.** `tests/baseline_test.rs::the_path_key_survives_the_leading_dot_slash_prefix_from_check_dot`
runs `check . --write-baseline baseline.json` then `check . --baseline baseline.json --format json`
from inside a temp directory — the exact shape `hook_script()` in `src/main.rs` uses — and asserts
the `skill.md` finding is baselined on the second run. `normalise_file` is applied on both sides
(`Baseline::from_reports` and `Baseline::apply`), which is what makes this hold.

## Task Commits

Each task was committed atomically:

1. **Task 1: One accepted finding, end to end — write it, then re-scan clean** - `f73e685` (feat)
2. **Task 2: The adversarial edges — count, staleness, malformed input, and the two rejections** - `4349765` (test)
3. **Task 3: ADR, README, and the clean-gate sweep** - `86bfccd` (docs)

**Plan metadata:** commit deferred — the orchestrator handles the `.planning/` docs commit for
this quick task; this SUMMARY.md was written to disk (worktree copy — the write sandbox refused
the primary-checkout path given in the plan's `<output>` section; see Deviations) but is not
committed by this execution.

## Deviations from Plan

### Auto-fixed Issues

None — plan executed exactly as written, task order and scope unchanged.

### Process deviations (not defects, documented per instructions)

**1. [Environment constraint] SUMMARY.md was written to the worktree's copy of the path, not the
primary-checkout path the plan's `<output>` section specified.** The plan asked for the SUMMARY to
be written to
`/Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/03-injection-scanner/.planning/quick/260825-tc7-cli-08-implement-baseline-file-for-incre/260825-tc7-SUMMARY.md`
in the primary checkout. The `Write` tool refused that path outright with: "This agent is isolated
in the worktree ...; Edit the worktree copy of this file instead of the shared-checkout path." This
is a hard sandbox restriction on this execution, not a discretionary choice — every attempt to
touch the primary checkout (including a read-only `git status`) was blocked the same way. The file
was written instead to
`.claude/worktrees/agent-a69608121ef39d47c/.planning/quick/260825-tc7-cli-08-implement-baseline-file-for-incre/260825-tc7-SUMMARY.md`
(the equivalent path inside the worktree, which did not previously contain this quick-task
subdirectory since the worktree was forked before it existed in the primary checkout). **The
orchestrator will need to copy or merge this file to the primary-checkout path itself** — this
executor has no tool capability to do so from inside the worktree sandbox.

**2. Task 2's tests passed on first run, without additional source changes.** The plan describes
Task 2 as "each of these is written as a failing test first, then made to pass," anticipating
specific gaps (the stale-entry note's home, both-sides path-key normalisation, deterministic
budget-exhaustion ordering, the reporter zero-total branch). Task 1 was executed as a genuinely
complete, production-quality tracer slice — informed by having read the full plan (including
Task 2's adversarial list) before writing Task 1's implementation — and already closed all of
those anticipated gaps: the stale-entry note was placed in `main.rs` from the start, `normalise_file`
was applied in both `from_reports` and `apply`, match iteration order was preserved via
`std::mem::take`, and the reporter's zero-total branch included `total_baselined` from the first
edit. All 13 of Task 2's tests were written, run, and passed immediately — confirmed by re-running
them individually and as a suite with no intervening implementation edits. This is a deviation
from the letter of "write a failing test, watch it fail, then close the gap it exposes" (RED did
not occur test-by-test for Task 2), but not from the plan's actual intent: every behaviour in
`must_haves.truths` is pinned by a named, currently-passing test, and Task 1's own RED/GREEN
cycle (unit tests for `fingerprint`/`normalise_file`, and the end-to-end tracer test) was performed
strictly, with both confirmed failing for the right reason before implementation.

## Authentication Gates

None encountered.

## Known Stubs

None. No hardcoded empty values, placeholder text, or unwired data sources were introduced.

## Threat Flags

None beyond what the plan's own `<threat_model>` already covers — no new network endpoints, auth
paths, or trust-boundary schema changes were introduced outside the `baseline.json`
read/write/apply surface the threat register already accounts for (T-QT-01 through T-QT-SC).

## Final Gate

```
cargo fmt --check               PASS (clean on final check; `cargo fmt` was applied twice during
                                  Task 1/Task 2 to fix formatting introduced by manual multi-line
                                  edits before each task's commit)
cargo clippy --all-targets -- -D warnings   PASS (zero warnings)
cargo test                      PASS — 196 tests passed, 0 failed, across 25 test result blocks
                                  (24 pre-existing test binaries + unit tests, plus the new
                                  tests/baseline_test.rs; the 24-binary baseline from STATE.md
                                  becomes 25 with this addition)
test -f docs/adr/ADR-002-baseline-fingerprints.md   PASS
grep -q 'write-baseline' README.md                  PASS
```

Manual end-to-end sanity (per the plan's `<verification>` section), run against this repository
from the worktree root:

```
$ injection-scanner check . --write-baseline <tmp>/b.json
  ... 54 finding(s): 21 critical, 17 high, 16 medium, 0 low
  ... 22 findings suppressed ... 93 findings withheld as documentation ...
  note: wrote 54 baseline entries to <tmp>/b.json
exit 0

$ injection-scanner check . --baseline <tmp>/b.json
exit 0

$ injection-scanner check <tmp>/b.json --include '**/*.json'
No injection patterns detected.
exit 0
```

All three steps matched the plan's expected behavior exactly.

## Self-Check: PASSED

- FOUND: src/baseline.rs
- FOUND: tests/baseline_test.rs
- FOUND: docs/adr/ADR-002-baseline-fingerprints.md
- FOUND: commit f73e685 (feat: --baseline / --write-baseline for incremental adoption)
- FOUND: commit 4349765 (test: adversarial baseline edges)
- FOUND: commit 86bfccd (docs: ADR-002 and README for --baseline / --write-baseline)
