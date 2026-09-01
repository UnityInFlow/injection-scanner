---
phase: 03-tool-permission-abuse-cat-01-33
plan: 01
subsystem: testing
tags: [recall-harness, corpus, frontmatter, cat-01, gate-01, gate-02]

requires:
  - phase: 03-tool-permission-abuse-cat-01-33 (Phase 1, ENG-01)
    provides: "src/frontmatter.rs structural projection + PatternScope::Frontmatter, previously inert (no scope:frontmatter pattern existed)"
provides:
  - "12 CAT-01 threat-model payloads (5 structural, 7 prose), pinned as the pre-pattern baseline"
  - "tests/recall_test.rs second collection mode (structural_payloads()/measure_structural()) — arms the recall harness to measure a frontmatter-scoped category for the first time"
  - "Proof (mutation-checked) that the structural pass is reachable end-to-end from a corpus payload once a frontmatter-scoped pattern is loaded"
  - "README.md recall table updated to 58/72 with the CAT-01 pre-pattern baseline row"
affects: [03-tool-permission-abuse-cat-01-33/03-04-PLAN, 03-tool-permission-abuse-cat-01-33/03-05-PLAN, 03-tool-permission-abuse-cat-01-33/03-06-PLAN]

actuals:
  tokens: 7451
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "Second recall-harness collection mode for whole-file, non-line-splittable payloads (structural_payloads(), parallel to categories()/payloads())"
    - "Probe pattern categories parsed via serde_yaml::from_str::<PatternCategory>() as test scaffolding, never shipped to patterns/"

key-files:
  created:
    - tests/corpus/attack/structural/README.md
    - tests/corpus/attack/structural/01-wildcard-allowed-tools-block-sequence.md
    - tests/corpus/attack/structural/02-scalar-wildcard-tools-grant.md
    - tests/corpus/attack/structural/03-json-manifest-wildcard-tools.md
    - tests/corpus/attack/structural/04-permissions-allow-wildcard-settings.md
    - tests/corpus/attack/structural/05-bypass-permission-mode.md
    - tests/corpus/attack/tool-permission-abuse.md
  modified:
    - tests/recall_test.rs
    - tests/corpus/attack/README.md
    - README.md

key-decisions:
  - "D-03 (no ratio set in advance): the threat model produced a 5 structural / 7 prose split, recorded as measured rather than targeted."
  - "D-04's baseline is genuinely 0/12, confirmed twice — once via the recall harness (cargo test) and once by scanning all 12 payloads individually with the release binary in --format json --strict, so no pattern id needed spillover attribution."
  - "The structural/ collection mode is kept flat per D-01's literal path, with an explicit code comment flagging that CAT-02 (#34) will need either its own subdirectory or a generalisation of structural_payloads()."

requirements-completed: [CAT-01, GATE-01, GATE-02]

coverage:
  - id: D1
    description: "One frontmatter payload proven reachable end-to-end (corpus file -> whole-file collector -> frontmatter::analyze -> projection -> frontmatter-scoped probe -> reported match -> exactly-pinned recall row), with a prose-scoped negative control on the identical regex"
    requirement: GATE-01
    verification:
      - kind: integration
        ref: "tests/recall_test.rs#the_structural_pass_is_reachable_from_the_corpus"
        status: pass
      - kind: integration
        ref: "tests/recall_test.rs#the_structural_corpus_is_actually_collected (mutation-checked: renaming tests/corpus/attack/structural fails this test)"
        status: pass
      - kind: integration
        ref: "tests/recall_test.rs#every_structural_payload_parses_as_frontmatter (mutation-checked: a leading comment before the fence fails this test)"
        status: pass
    human_judgment: false
  - id: D2
    description: "12 threat-model payloads (5 structural, 7 prose) exist and are collected by the recall harness, exactly 12 per GATE-01"
    requirement: GATE-01
    verification:
      - kind: integration
        ref: "tests/recall_test.rs#the_cat_01_payload_totals_sum_to_twelve"
        status: pass
      - kind: integration
        ref: "tests/recall_test.rs#every_claimed_category_has_a_corpus_file"
        status: pass
      - kind: integration
        ref: "tests/recall_test.rs#no_payload_is_duplicated_across_the_corpus"
        status: pass
    human_judgment: false
  - id: D3
    description: "Pre-pattern baseline (0/12) measured and pinned exactly in EXPECTED, per GATE-02, with the README recall table updated in the same phase"
    requirement: GATE-02
    verification:
      - kind: integration
        ref: "tests/recall_test.rs#recall_matches_the_recorded_numbers"
        status: pass
      - kind: manual_procedural
        ref: "target/release/injection-scanner check <each of 12 payload files> --format json --strict, confirmed matches: [] for all 12"
        status: pass
    human_judgment: false
  - id: D4
    description: "Zero patterns ship in this plan (D-04's ordering requirement)"
    requirement: GATE-01
    verification:
      - kind: other
        ref: "grep -rn 'scope:' patterns/core/*.yaml (returns nothing)"
        status: pass
    human_judgment: false

duration: 45min
completed: 2026-09-01
status: complete
---

# Phase 3 Plan 1: CAT-01 corpus + harness + pre-pattern baseline Summary

**12 threat-model payloads for tool & permission abuse (5 structural, 7 prose) land with a second recall-harness collection mode and a measured 0/12 pre-pattern baseline — zero PI05x patterns exist yet, by design.**

## Performance

- **Duration:** 45 min
- **Started:** 2026-09-01
- **Completed:** 2026-09-01
- **Tasks:** 3
- **Files modified:** 10 (7 created, 3 modified)

## Accomplishments

- Wired one structural payload end-to-end through every layer this phase touches (corpus file ->
  `structural_payloads()` collector -> `frontmatter::analyze` -> projection -> a frontmatter-scoped
  probe pattern -> a reported match -> an exactly-pinned recall row), with a mutation-checked proof
  that the structural pass is actually reachable and a prose-scoped negative control on the
  identical regex proving the finding came from the structural pass, not raw-line matching.
- Landed the remaining 11 payloads: 4 more structural shapes (scalar wildcard `tools: "*"`, a
  whole-file JSON manifest with `"tools": ["*"]`, a whole-file JSON `settings.json` shape with
  `permissions.allow: ["Bash(*)"]`, and a structurally-set `permissions.defaultMode:
  bypassPermissions`) and 7 prose payloads (CLI/mode bypass flags, an unrestricted-authority claim,
  a skip-confirmation directive, a settings.json-widening persuasion, and the imperative
  disable-a-guardrail form D-17 keeps in this category).
- Measured the baseline against the shipping 48-pattern set twice — via the recall harness and via
  the release binary scanning each of the 12 payloads individually in `--format json --strict` —
  and confirmed zero spillover from any existing pattern.
- Updated `README.md`'s recall table (58/72, 80.6%) with the pre-pattern baseline row and corrected
  the "what this still is not" paragraph and `tests/corpus/attack/README.md`'s stale "deliberately
  absent" claim.

## Task Commits

Each task was committed atomically:

1. **Task 1: End-to-end "a frontmatter payload is measured" — one payload only** - `6518917` (feat)
2. **Task 2: The remaining 11 threat-model payloads and the corpus README correction** - `f78bab1` (feat)
3. **Task 3: Measure the baseline against the shipping 48-pattern set and pin it** - `e28a1c4` (docs)

**Plan metadata:** committed separately after this SUMMARY.

## Files Created/Modified

- `tests/corpus/attack/structural/README.md` - the structural corpus's own contract (fence-on-line-1 rule, sourcing rule, exclusion from `categories()`)
- `tests/corpus/attack/structural/01-wildcard-allowed-tools-block-sequence.md` - the tracer payload, YAML block-sequence `allowed-tools` grant including `Bash(*)`
- `tests/corpus/attack/structural/02-scalar-wildcard-tools-grant.md` - scalar `tools: "*"` grant
- `tests/corpus/attack/structural/03-json-manifest-wildcard-tools.md` - whole-file JSON manifest, `"tools": ["*"]`
- `tests/corpus/attack/structural/04-permissions-allow-wildcard-settings.md` - whole-file JSON `settings.json` shape, `permissions.allow: ["Bash(*)"]` alongside a realistic `permissions.deny` entry
- `tests/corpus/attack/structural/05-bypass-permission-mode.md` - structurally-set `permissions.defaultMode: bypassPermissions`
- `tests/corpus/attack/tool-permission-abuse.md` - 7 prose payloads, one per line
- `tests/recall_test.rs` - `structural_dir()`, `structural_payloads()`, `measure_structural()`, `STRUCTURAL_CATEGORY`, two new `EXPECTED` rows, `the_structural_corpus_is_actually_collected`, `every_structural_payload_parses_as_frontmatter`, `the_structural_pass_is_reachable_from_the_corpus`, `the_cat_01_payload_totals_sum_to_twelve`; extended `recall_matches_the_recorded_numbers`, `every_claimed_category_has_a_corpus_file`, `no_payload_is_duplicated_across_the_corpus`
- `tests/corpus/attack/README.md` - removed tool/permission abuse from the "deliberately not here" list, documented `structural/`
- `README.md` - recall table (58/72) and "what this still is not" paragraph corrected

## Decisions Made

- **D-03 honored as written:** no prose/structural ratio was chosen in advance. The threat model
  produced 5 structural shapes and 7 prose shapes; that split is recorded as measured, not
  engineered to hit a target.
- **The pre-pattern baseline is exactly 0/12**, confirmed by two independent methods (the recall
  harness's `Scanner::scan` and the release binary's `check --format json --strict` on each
  payload individually). No existing pattern id needed spillover attribution — the plan's action
  text anticipated this might not be zero and asked to "stop and investigate" if the structural
  row were non-zero; it was zero, confirming `scanner.rs`'s frontmatter short-circuit behaves as
  documented with no `scope: frontmatter` pattern loaded.
- **`structural_payloads()` is flat**, per D-01's literal path, with a code comment flagging CAT-02
  (#34) will need either its own subdirectory or a generalisation of this function.

## Deviations from Plan

None - plan executed exactly as written. All three tasks' acceptance criteria and `<verify>`
commands passed as specified, including both mutation checks (renaming `structural/` and inserting
a leading comment above a payload's fence), each confirmed to fail and then restored.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The corpus and harness are armed: `tests/recall_test.rs` now measures a `scope: frontmatter`
  category for the first time, with a mutation-tested proof the structural pass is reachable.
- Zero PI05x patterns exist yet, as required by D-04 — Plans 04-06 (per the phase's cross-plan
  artifact table) can now write patterns against a corpus whose baseline is a committed, measured
  fact rather than an assumption, and the delta they produce will be a readable diff against this
  commit's 0/12.
- No blockers.

---
*Phase: 03-tool-permission-abuse-cat-01-33*
*Completed: 2026-09-01*

## Self-Check: PASSED

- All created/modified files verified present on disk (10 files).
- All three task commits (`6518917`, `f78bab1`, `e28a1c4`) verified present in `git log --oneline --all`.
- All `<acceptance_criteria>` re-run and passing for Tasks 1-3.
- `cargo test --locked` fully green (full suite).
- `cargo fmt --all -- --check` and `cargo clippy --all-targets --locked -- -D warnings` both clean.
- `grep -rn 'scope:' patterns/core/*.yaml` returns nothing (zero patterns shipped, D-04 honored).
- Both mutation checks (directory rename, leading comment) confirmed to fail then restored.
