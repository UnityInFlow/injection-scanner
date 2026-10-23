---
phase: 04-mcp-tool-description-poisoning-cat-02-34
plan: 07
subsystem: patterns
tags: [cat-02, mcp, tool-description-poisoning, gate03, gate04, close-out, number-reconciliation, issue-filing]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 06
    provides: CAT-02's full ten-pattern set (PI060-PI069), the sweep-after-04-06-2026-09-07 evidence, and the D-01 third-person blind spot's partial narrowing via PI066
provides:
  - a whole-branch (whole-PR) GATE-03 delta for PI060-PI069, verified two independent ways (the orchestrator-directed sweep-mainbase-04-05-2026-09-07 baseline, and a freshly rebuilt 66bf53c binary in a same-session disposable git worktree) — both clean in both directions
  - a number reconciliation table in 04-SWEEP.md covering every number CAT-02 publishes (pattern count, recall, corpus totals, test count, severity description), all already agreeing with the finished tree before this task began
  - five filed GitHub issues for measured limitations (#129 JSONC parse gap, #130 decoded-layer pass skips structural patterns, #131 D-05 structural cross-reference, #132 docs/DETECTION-BACKLOG.md self-match, #133 WR-02 corpus README backfill) and deferred-items.md as the single accounting document
  - docs/DETECTION-BACKLOG.md's CAT-02 bullets marked shipped, distinguishing fully-answered from partially-answered (config-hygiene's accepted npx-y gap; the rug-pull bullet's language-only bound)
  - a full CHANGELOG.md entry for CAT-02: all ten pattern ids, the severity banding and its install-hook consequence, four Changed entries mirroring every README behaviour-change callout, and a Security note
  - GATE-04 verified from the branch diff (one patterns/core/ file touched, no CAT-03 path, empty Cargo.toml/Cargo.lock diff against origin/main)
  - the restored Phase 3 planning directory and CR-01 quick-task directory (lost to the rebase-merge that closed #33), recovered from the still-live feat/cat-01-tool-permission-abuse branch
  - a discovered-and-fixed methodological trap: scripts/gate03-sweep.sh --compare silently loads an empty baseline against this repository's own committed (JSON-less) sweep directories — always point it at .planning/local/
affects: [05-persistence-lifecycle-hijack-cat-03-35]

actuals:
  tokens: 131688   # chars/4 over `git diff 62a5a27..HEAD` (full diff, all three tasks).
                    # Of this, ~114600 tokens (454508 chars/4) is the restored Phase 3
                    # planning directory + CR-01 quick-task directory (24+8 files, a
                    # one-time historical recovery, not new authored content) — the
                    # genuine new-work diff (everything else) is ~18100 tokens
                    # (72244 chars/4). Compare the plan's 80000-token estimate against
                    # the ~18100 genuine-new-work figure, not the inflated total.
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "Always point `scripts/gate03-sweep.sh --compare` at `.planning/local/<sweep-dir>/`, never at this repository's own committed sweep directory — the committed copy has zero raw JSON by design (dad56d1/e54be72), so `--compare` silently loads an empty baseline and reports every real finding as a false addition"
    - "Build an independent, same-session fresh pre-edit binary (disposable git worktree) as a corroborating cross-check even when a suitable committed baseline already exists — two independently-built binaries agreeing exactly is the strongest form of GATE-03 evidence available"
    - "Where a plan's literal acceptance-criteria wording (a stale exact count, an environment-specific shell idiom) conflicts with a measured, already-adjudicated reality, verify the underlying invariant directly and document the wording gap as a deviation rather than force a document to match stale text"

key-files:
  created:
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-final-2026-09-03/ (redacted GATE-03 sweep output: manifest.tsv, summary.tsv, checksums.sha256, RAW-REPORTS.md)
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/deferred-items.md
    - .planning/phases/03-tool-permission-abuse-cat-01-33/ (restored, 24 files, from commit 752ac98)
    - .planning/quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/ (restored, 8 files, from commit db2a575)
  modified:
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md (new "Phase 4 Plan 07" section: runs, both comparison pairs, full number reconciliation table, the --compare-against-an-empty-baseline trap and its fix)
    - docs/DETECTION-BACKLOG.md (CAT-02 bullets marked shipped, four bullets distinguished fully/partially answered)
    - CHANGELOG.md (full CAT-02 Added entry, four Changed entries, one Security entry)
    - .planning/WINDOWS.md (entry #1 fixed — src/frontmatter.rs:219 panic, quick task 260903-fast; entry #2 waived — WR-02, filed as #133)
    - .planning/.continue-here.md (new anti-pattern table row: comparison silently substituting an empty baseline)
    - .planning/ROADMAP.md (Phase 4 plan checklist 7/7 ticked; library/recall projections corrected to measured values)
    - .planning/REQUIREMENTS.md (CAT-02 checkbox ticked; traceability table row Complete, plus ENG-01/ENG-02/CAT-01's stale Pending rows corrected in the same table)
    - .planning/STATE.md (Current Phase status, Phases table, Session Notes entry naming all four Phase-5 inheritances, Tracking section listing newly-filed issues and cross-referencing #115/#116/#117, three new Decisions entries)

key-decisions:
  - "The orchestrator-directed sweep-mainbase-04-05-2026-09-07 baseline is the whole-category (whole-PR) delta this task's verdict rests on, not the plan's originally-named sweep-baseline-2026-09-03 — three-plus pattern-set generations of unrelated history (PR #110, #122, #124-#127, 04-04 itself) sit between the plan's named baseline and this branch's actual fork point"
  - "Comparing directly against this repository's own committed sweep directories is invalid — they have zero raw JSON on disk by design, so scripts/gate03-sweep.sh --compare silently loads an empty baseline. Always re-point at .planning/local/<sweep-dir>/, which still holds the real, gitignored JSON"
  - "A second, independent pre-edit binary was built fresh from 66bf53c in a disposable git worktree, in the same session as the final candidate, purely to corroborate the committed baseline's own result — not because the committed baseline was suspected of being wrong, but because two independently-built binaries agreeing exactly is stronger evidence than one"
  - "The plan's self-scan verify command (`sys.exit(0 if len(rows)==2 else 1)`) is not honored literally — it embeds the stale 04-01-era assumption of two self-matches, already superseded by the PR #110-origin ten-entry docs/DETECTION-BACKLOG.md self-match plans 04-04/04-05/04-06 already found and recorded. Verified the real invariant instead: the 12-pair set is unchanged before and after every edit in this plan"
  - "The Phase 3 planning directory and the CR-01 quick-task directory are restored to main from their still-live source branch (feat/cat-01-tool-permission-abuse, confirmed present on origin, not at risk of garbage collection) rather than left as file-path references — reversible, documentation-only, and stops 04-CONTEXT.md's two dangling citations from dangling without any citation-text edit"
  - "ROADMAP's Phase 4 top-level checkbox, its Progress-table Status column, and STATE's completed_phases counter are deliberately NOT flipped to complete/Done — per the orchestrator's explicit instruction, those phase-complete markers are the orchestrator's to set after its own verification runs. Only plan-level progress (7/7 ticked) and factual number corrections (library/recall projections) were made"
  - "ENG-01/ENG-02/CAT-01's stale 'Pending' rows in REQUIREMENTS.md's traceability table were corrected to 'Complete' alongside CAT-02's own row — the same document, the same column, and leaving three already-shipped requirements marked Pending right next to a freshly-ticked CAT-02 would be a glaring, easily-fixed inaccuracy in the exact table this task was already editing (Rule 1)"
  - "state.record-metric zeroed STATE.md's completed_phases (3->0) and percent (94->0) fields on this run — the same regression class 04-05's own SUMMARY already documented for a different field pair. Corrected by hand immediately; not re-attempted via the tool a second time in this plan"

requirements-completed: [CAT-02, GATE-01, GATE-02, GATE-03, GATE-04, GATE-05]

coverage:
  - id: D1
    description: "The whole-category (whole-branch) GATE-03 delta for PI060-PI069 is measured on the finished tree, both directions, against the correct isolating baseline, and independently corroborated with a second freshly-built binary"
    requirement: "GATE-03"
    verification:
      - kind: integration
        ref: "scripts/gate03-sweep.sh --compare (sweep-mainbase-04-05-2026-09-07 <-> sweep-final-2026-09-03, both directions: 0/0) plus an independent fresh-66bf53c-worktree rebuild, both directions: 0/0"
        status: pass
    human_judgment: false
  - id: D2
    description: "Every published CAT-02 number (71 patterns, 102/109 recall, 12-payload corpus, 406 tests, PATTERNS.md severity row) was measured against the finished tree and agrees with README/PATTERNS.md/tests/recall_test.rs — no correction required"
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "tests/pattern_test.rs::test_total_pattern_count (71); tests/recall_test.rs::recall_matches_the_recorded_numbers (102/109); cargo test --test catalogue_test (passes without regenerating)"
        status: pass
      - kind: other
        ref: "04-SWEEP.md's Plan 07 'Number reconciliation table' — one row per published number, each with its measured value and the documents checked against"
        status: pass
    human_judgment: false
  - id: D3
    description: "Every measured limitation this phase found is either a filed issue or an explicit, referenced existing item — none survives only as tacit knowledge"
    requirement: "CAT-02"
    verification:
      - kind: other
        ref: "deferred-items.md — 5 newly filed issues (#129-#133), 2 existing-and-accepted items (D-01 blind spot, rug-pull bound), 1 measured-not-applicable item (04-03's zero rejected candidates)"
        status: pass
    human_judgment: false
  - id: D4
    description: "GATE-04 is verified from the branch diff against origin/main (== 66bf53c): exactly one patterns/core/ file touched, no CAT-03 path, no dependency-manifest change"
    requirement: "GATE-04"
    verification:
      - kind: integration
        ref: "git diff --name-only origin/main..HEAD | grep '^patterns/core/' (mcp-tool-poisoning.yaml only); git diff --stat origin/main..HEAD -- Cargo.toml Cargo.lock (empty)"
        status: pass
    human_judgment: false
  - id: D5
    description: "The dangling 04-CONTEXT.md citations to Phase 3 artifacts resolve — the Phase 3 planning directory and the CR-01 quick-task directory are restored to main"
    requirement: "CAT-02"
    verification:
      - kind: other
        ref: "git checkout 752ac98 -- .planning/phases/03-tool-permission-abuse-cat-01-33/ (24 files); git checkout db2a575 -- .planning/quick/260902-jhy-.../ (8 files); both paths now exist at HEAD"
        status: pass
    human_judgment: false
  - id: D6
    description: "The developer review packet (recall delta, pattern count, GATE-03 adjudication, reconciliation corrections, filed issue numbers, GATE-04 result, and the two accepted costs) is assembled for the end-of-phase human review"
    requirement: "CAT-02"
    verification: []
    human_judgment: true
    rationale: "The packet's three review questions (severity banding, D-01/rug-pull accepted trades, whether to keep the Phase 3 restoration on main) require a human's judgment call, not an automated pass/fail — that is the explicit purpose of this task's <human-check> block, harvested at end-of-phase per workflow.human_verify_mode: end-of-phase."

duration: commit span ~27min (18:09-18:36 UTC / 20:09-20:36 CEST, 2026-09-07) across three task commits; total session time including required reading, the sweep methodology investigation, and issue filing was longer (~58min from 19:38 CEST)
completed: 2026-09-07
status: complete
---

# Phase 4 Plan 07: CAT-02 close-out — whole-category sweep, number reconciliation, five filed issues Summary

**A whole-branch GATE-03 delta for CAT-02's ten patterns comes back clean in both directions, verified two independent ways after a same-machine sweep-comparison trap (comparing against an empty, JSON-less committed baseline) produced and then resolved a spurious 500-line "diff"; every published number already agreed with the finished tree; five measured limitations are now filed issues; and the Phase 3 planning directory lost to a rebase-merge is restored to `main`.**

## Performance

- **Duration:** commit span ~27min (18:09-18:36 UTC / 20:09-20:36 CEST) across three task commits; total session time longer (~58min from 19:38 CEST) including required reading and the sweep methodology investigation
- **Tasks:** 3/3 completed
- **Files touched:** 46 (4 created — one sweep-record directory, `deferred-items.md`, and two restored planning directories of 24+8 files each; 9 modified outside the restored directories)
- **Test count:** 406 passing throughout (unchanged — no source code, pattern, or test file was touched by this plan)

## Accomplishments

- Whole-branch (PI060-PI069) GATE-03 delta: 0 additions, 0 removals against the orchestrator-directed `sweep-mainbase-04-05-2026-09-07` baseline, independently corroborated with a freshly rebuilt `66bf53c` binary in a same-session disposable `git worktree` (also 0/0)
- Discovered and fixed a real methodological trap along the way: `scripts/gate03-sweep.sh --compare` against this repository's own committed (JSON-less) sweep directories silently loads an empty baseline and reports every real finding as a false addition — a spurious 500-line "diff" on the first attempt, resolved by re-pointing at `.planning/local/`, which still holds the real, gitignored JSON
- Every published number (71 patterns, 102/109 recall, 12-payload CAT-02 corpus, 406 tests, `PATTERNS.md`'s severity description) was measured against the finished tree and already agreed with the README, `PATTERNS.md` and `tests/recall_test.rs` — plans 04-04/04-05/04-06 had each already reconciled their own numbers in the commit that changed them, so this task needed zero corrections, only verification
- Five issues filed for measured limitations: the JSONC parse gap (#129), the decoded-layer pass's inability to reach a structural pattern (#130), D-05's deferred structural cross-reference (#131), `docs/DETECTION-BACKLOG.md`'s ten-pattern self-match arriving with the PR #110 merge (#132), and WR-02's carried-over corpus README gap (#133, with its `.planning/WINDOWS.md` entry waived)
- GATE-04 verified from the branch diff: exactly one `patterns/core/` file touched, no CAT-03 path, empty `Cargo.toml`/`Cargo.lock` diff against `origin/main`
- Restored the Phase 3 planning directory (7 plans, 7 summaries, the review, the sweep evidence — 24 files) and the CR-01 quick-task directory (8 files), both lost to the rebase-merge that closed #33, from their still-live source branch (`feat/cat-01-tool-permission-abuse`, confirmed present on `origin`) — `04-CONTEXT.md`'s two citations now resolve without any citation-text edit

## Task Commits

Each task was committed atomically:

1. **Task 1: Whole-category sweep and number reconciliation** - `16f2135` (docs)
2. **Task 2: File issues for measured limitations, write the consumer record** - `e74f7a6` (docs)
3. **Task 3: Tracking close-out, GATE-04 verification, Phase 5 handoff** - `e4b46a1` (docs)

_Note: all three commits are `docs` type — this plan is pure documentation, sweep evidence, and tracking work. No production source, pattern, or test file was touched._

## Files Created/Modified

- `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-final-2026-09-03/` - final whole-category sweep output (redacted; raw JSON at `.planning/local/`)
- `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md` - new "Phase 4 Plan 07" section: runs, both comparison pairs, the empty-baseline trap and its fix, the full number reconciliation table
- `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/deferred-items.md` - every deferral, newly filed or existing, with its issue number or explicit accepted-cost status
- `docs/DETECTION-BACKLOG.md` - CAT-02's four bullets marked shipped, two fully-answered and two partially-answered (with the bound named)
- `CHANGELOG.md` - full CAT-02 `[Unreleased]` entry: Added (all ten pattern ids, severity table), four Changed entries, one Security entry
- `.planning/WINDOWS.md` - entry #1 (frontmatter panic) marked fixed; entry #2 (WR-02) waived with issue #133's reference
- `.planning/.continue-here.md` - new anti-pattern table row for the empty-baseline comparison trap
- `.planning/ROADMAP.md` - Phase 4 plan checklist 7/7; library/recall projections corrected to measured values (phase-complete markers left untouched per orchestrator instruction)
- `.planning/REQUIREMENTS.md` - CAT-02 ticked and its traceability row set Complete; ENG-01/ENG-02/CAT-01's stale Pending rows corrected in the same table
- `.planning/STATE.md` - Current Phase status, Phases table, Session Notes entry, Tracking section, three new Decisions entries
- `.planning/phases/03-tool-permission-abuse-cat-01-33/` - restored, 24 files, from commit `752ac98`
- `.planning/quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/` - restored, 8 files, from commit `db2a575`

## Decisions Made

See `key-decisions` in the frontmatter. The most consequential: (1) treating the orchestrator-directed `sweep-mainbase-04-05-2026-09-07` as this task's real verdict-bearing baseline, with `sweep-baseline-2026-09-03` demoted to a continuity-only check; (2) discovering and fixing the empty-baseline `--compare` trap rather than reporting the spurious 500-line diff as a real finding; (3) building an independent, same-session fresh pre-edit binary purely as corroboration, even though a suitable committed baseline already existed; (4) restoring the Phase 3 directory to `main` from its still-live source branch rather than leaving it as a dangling reference; (5) deliberately NOT flipping Phase 4's phase-complete markers, per the orchestrator's explicit scope instruction.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `state.record-metric` zeroed STATE.md's `completed_phases` and `percent` frontmatter fields**
- **Found during:** Task 3, immediately after running the tool
- **Issue:** `gsd_run query state.record-metric` correctly appended the Performance Metrics table row but also rewrote `completed_phases: 3 -> 0` and `percent: 94 -> 0` — the same regression class 04-05's own SUMMARY already documented for a different field pair (`total_plans`/`completed_plans`) on this same file, whose narrative `STATE.md` format predates the fields these tools expect.
- **Fix:** Restored `completed_phases: 3` and `percent: 94` by hand immediately after the tool ran, before any other edit. Not re-attempted via the tool a second time in this plan.
- **Files modified:** `.planning/STATE.md`
- **Verification:** `git diff` confirmed the frontmatter's `completed_phases`/`percent` values matched their pre-tool state before proceeding
- **Committed in:** `e4b46a1` (Task 3 commit)

### Auto-fixed / Investigated Issues (deviations from the plan's literal text, not from correctness)

**2. [Rule 3 - Blocking, self-diagnosed] A `--compare` invocation against this repository's own committed sweep directories silently loads an empty baseline**
- **Found during:** Task 1, on the first attempt to run the whole-category comparison
- **Issue:** `scripts/gate03-sweep.sh --compare` globs `*.json` in each argument directory. This repository deliberately does not commit the raw per-directory JSON reports (`dad56d1`, `e54be72`) — only `manifest.tsv`, `summary.tsv` and `checksums.sha256` are public. Comparing against a committed (JSON-less) copy of `sweep-baseline-2026-09-03` or `sweep-mainbase-04-05-2026-09-07` therefore loads an **empty baseline**, and every one of this run's 500 real findings read as a false "new" one — pattern ids `PI001` through `PI049`, none of them CAT-02's.
- **Fix:** Re-pointed `--compare` at `.planning/local/<sweep-dir>/`, which still holds the real, gitignored JSON on this machine. Both directions against the real `sweep-mainbase-04-05-2026-09-07` data: empty (0/0). Independently corroborated with a freshly rebuilt `66bf53c` binary swept in the same session: also 0/0.
- **Files modified:** none (diagnostic-only; the fix was in the command invocation, not any file)
- **Verification:** `04-SWEEP.md`'s Plan 07 section records the full before/after, both compare commands, and the root-cause explanation
- **Committed in:** `16f2135` (Task 1 commit)

**3. [Rule 3 - Blocking, documented] Two of the plan's literal `<verify>`/acceptance-criteria commands do not pass verbatim on this machine, though the invariant they check holds**
- **Found during:** Task 2 (the self-scan `len(rows)==2` check) and Task 3 (the `wc -l | grep -qx '0'` check)
- **Issue:** (a) The plan's Task 2 automated self-scan check asserts exactly 2 self-matching findings outside `examples/`, `patterns/`, `tests/`, `tools/` — a stale 04-01-era assumption. The real, already-adjudicated set (04-04 onward) is 12 unique `(file, pattern_id)` pairs (10 in `docs/DETECTION-BACKLOG.md` from the PR #110 merge, 2 pre-existing in `docs/PATTERN-CATALOGUE.md`), unattributable to this phase and filed as #132. (b) The plan's Task 3 automated GATE-04 check pipes `wc -l` into `grep -qx '0'` — this machine's BSD `wc -l` pads its count with leading whitespace (`"       0"`), which does not exact-match `-x '0'`, so the literal command reports FAIL even though the underlying `Cargo.toml`/`Cargo.lock` diff is genuinely empty.
- **Fix:** Neither is "fixed" in the sense of changing behavior — both are pre-existing environment/wording mismatches between the plan text and reality. Verified the real invariant directly in both cases: (a) the 12-pair self-scan set is byte-identical before and after every edit in Task 2; (b) `git diff --stat origin/main..HEAD -- Cargo.toml Cargo.lock | wc -l | tr -d ' '` prints `0`.
- **Files modified:** none
- **Verification:** documented inline in this SUMMARY and in `04-SWEEP.md`'s Plan 07 section
- **Committed in:** `e74f7a6` (Task 2), `e4b46a1` (Task 3)

**4. [Rule 3 - Blocking, documented] The plan's Task 3 GATE-04 acceptance criterion expects an "added" `patterns/core/` file; the real diff shows it "modified"**
- **Found during:** Task 3's GATE-04 diff check
- **Issue:** The plan's acceptance criterion reads "`git diff --name-only <base>..HEAD` contains exactly one **added** file under `patterns/core/`." `patterns/core/mcp-tool-poisoning.yaml` already existed at this branch's fork point (`66bf53c`), because plan 04-04 (`PI060`-`PI062`) shipped on a separate, already-merged branch before this branch (`feat/34-mcp-tool-poisoning-pi063`, carrying plans 04-05/04-06/04-07) was created. The diff therefore shows the file as `M` (modified), not `A` (added).
- **Fix:** Verified the substantive invariant the criterion exists to protect — exactly one file under `patterns/core/` is touched at all, and no other file under `patterns/core/` is modified — which holds regardless of the A/M distinction. Not a scope violation: GATE-04's purpose (one category, one reviewable diff) is satisfied either way.
- **Files modified:** none
- **Verification:** `git diff --name-status origin/main..HEAD -- patterns/core/` shows exactly one row, `M patterns/core/mcp-tool-poisoning.yaml`
- **Committed in:** `e4b46a1` (Task 3 commit)

---

**Total deviations:** 4 (1 auto-fixed bug in a state-management tool; 3 documented literal-wording/environment gaps between the plan's acceptance criteria and measured reality, none requiring a code or content change). **Impact on plan:** None represent scope creep or a real correctness gap. The state-metric fix restored data integrity in `STATE.md`'s frontmatter. The three wording gaps are all cases where the underlying invariant was verified directly and found to hold — the literal command or exact-count assertion in the plan text was simply stale or environment-specific.

## Issues Encountered

None beyond the deviations above.

## Developer Review Packet

Assembled here per Task 3's instruction, for the end-of-phase human review this plan's `<human-check>` block defers to (per `workflow.human_verify_mode: end-of-phase`, harvested into the phase's UAT flow rather than a mid-flight checkpoint).

### 1. Final recall numbers against the pre-phase numbers

| | Pre-phase (04-01, zero CAT-02 patterns) | Post-phase (04-07, all ten shipped) |
|---|---:|---:|
| CAT-02 combined | 6/12 (50%) | **9/12 (75%)** |
| CAT-02 prose sub-row | 2/4 (50%) | **4/4 (100%)** |
| CAT-02 structural sub-row | 4/8 (50%) | **5/8 (62.5%)** |
| Library-wide total | 76/84 (90.5%, measured 04-02, right after CAT-02's corpus landed) | **102/109 (93.6%)** |

The three remaining CAT-02 misses: two rug-pull structural payloads (D-05's deferred structural cross-reference, issue #131) and one deliberately-undetected unpinned-registry-install payload (D-03's accepted cost, the ecosystem-default measurement).

### 2. Total pattern count

**71 patterns** (measured from the loader, `injection-scanner rules --format json`), matching `tests/pattern_test.rs::test_total_pattern_count`, the README's pattern-count sentence, and the README Pattern Categories table's per-category counts summed. CAT-02 (`mcp_tool_poisoning`) holds exactly 10.

### 3. Whole-category GATE-03 adjudication summary

Whole-branch delta (PI060-PI069) against the correct isolating baseline: **0 additions, 0 removals**, verified two independent ways (the orchestrator-directed `sweep-mainbase-04-05-2026-09-07`, and a freshly rebuilt `66bf53c` binary in a same-session disposable worktree — both agree exactly). Continuity comparison against the stale 04-01 baseline (`sweep-baseline-2026-09-03`, recorded for completeness, not the verdict-bearing pair): 2 additions (1 pre-existing true positive, 1 scratch-path-churn artifact) and 67 removals (66 of PR #110's pre-existing delta already adjudicated in 04-04, 1 the path-churn counterpart) — identical to 04-05's and 04-06's own continuity numbers, nothing attributable to this plan. Full detail in `04-SWEEP.md`'s "Phase 4 Plan 07" section.

### 4. Reconciliation corrections

**None.** Every number this phase publishes already agreed with the finished tree before this task began — plans 04-04, 04-05 and 04-06 each reconciled their own numbers in the same commit that changed them. The one wording gap found (the self-scan set's literal "two known standing self-matches" language, stale since 04-04) was adjudicated as an existing, already-documented condition rather than a new correction — see `04-SWEEP.md`'s "The self-scan wording gap" section.

### 5. Filed issue numbers

#129 (JSONC parse gap), #130 (decoded-layer pass skips structural patterns), #131 (D-05 structural cross-reference), #132 (`docs/DETECTION-BACKLOG.md` self-match), #133 (WR-02 corpus README gap). Full accounting, including two existing-and-accepted items that were deliberately NOT filed as new issues (D-01's third-person blind spot, the rug-pull bound), in `deferred-items.md`.

### 6. GATE-04 diff check result

**Pass.** `git diff --name-only origin/main..HEAD | grep '^patterns/core/'` returns exactly one file (`mcp-tool-poisoning.yaml`, modified — see Deviation 4 above for why "modified" rather than "added" is correct here). No path relating to CAT-03 (`PI070`-`PI079`, persistence & lifecycle hijack) appears anywhere in the diff. `git diff --stat origin/main..HEAD -- Cargo.toml Cargo.lock` is empty.

### The two accepted costs this phase records in shipped files

1. **D-01's third-person blind spot.** `PI063`-`PI065` require second-person address; a bare third-person payload aimed directly at the model, with no other tool referenced, is outside their reach by design. `PI066` (04-06) narrows this to the specific cross-tool-shadowing shape but does not close it generally. Stated in `patterns/core/mcp-tool-poisoning.yaml`'s header, the README's behaviour-change callouts, and `deferred-items.md`.
2. **The rug-pull bound.** `PI068`/`PI069` detect version-, date-, approval- and call-count-conditional directive LANGUAGE only — they cannot mitigate the rug-pull class itself, since a single static scan cannot prove a server will not silently republish a poisoned description after a gating condition is met. Stated in the same three places.

### Three questions for the developer (per Task 3's `<human-check>`)

1. **Is the severity banding right** — three arms (`PI063`-`PI065`) at HIGH, the severity `install-hook` blocks commits at, and seven (`PI060`-`PI062`, `PI066`-`PI069`) at MEDIUM?
2. **Are D-01's third-person blind spot and the rug-pull bound the right trades**, given the false-positive evidence recorded behind them (the four measured Q3 near-misses for D-01; the 26-hit trigger-words-only false-positive class 04-06's sweep found and fixed for the heuristic arms)?
3. **Should the Phase 3 planning artifacts stay restored on `main`** (this plan's default choice, made because the source branch is reversible and the restoration is documentation-only), **or should they be reverted and referenced where they live** at commit `752ac98`/`db2a575` on `feat/cat-01-tool-permission-abuse` instead?

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- CAT-02 is fully closed: ten patterns shipped, recall measured and reconciled, GATE-03/GATE-04/GATE-05 all verified from evidence, five issues filed for every measured limitation, and tracking updated to the real position.
- Phase 5 (CAT-03, `PI070`-`PI079`, #35) inherits: the `relaxed_pattern` obligation (`id >= 50`, already covers `PI070`+), the CR-01 negation rule, the per-category structural corpus layout (one level of nesting, non-recursive collectors — a third-level nesting need would require a new dedicated walker), and the open follow-ups in `deferred-items.md`.
- Three questions await the developer's sign-off (above) before a PR is opened — this SUMMARY is what the end-of-phase verification step reads to assemble that review.
- No blockers.

---
*Phase: 04-mcp-tool-description-poisoning-cat-02-34*
*Completed: 2026-09-07*

## Self-Check: PASSED

All claimed files verified present on disk (sweep output, `deferred-items.md`, the restored Phase
3 planning directory and CR-01 quick-task directory, `04-SWEEP.md`, `docs/DETECTION-BACKLOG.md`,
`CHANGELOG.md`, `.planning/.continue-here.md`, `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`,
`.planning/STATE.md`, `.planning/WINDOWS.md`) and all three task commit hashes (`16f2135`,
`e74f7a6`, `e4b46a1`) confirmed present in `git log`. Re-ran the full gate immediately before this
check: `cargo fmt --all -- --check` clean, `cargo clippy --all-targets --locked -- -D warnings`
clean, `grep -c 'PI060' CHANGELOG.md` returns 3 (>= 1, all ten ids present), and
`git diff --stat origin/main..HEAD -- Cargo.toml Cargo.lock` is empty (0 lines after trimming —
see Deviation 3 for why the literal `wc -l | grep -qx '0'` form fails on this machine's BSD
`wc -l`). `cargo test --locked` was last run green (406/406) immediately before Task 3's commit,
and no source, pattern, or test file has changed since.
