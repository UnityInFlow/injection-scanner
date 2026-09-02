---
phase: quick-260902-jhy
plan: 01
subsystem: pattern-library
tags: [regex, injection-scanner, tool-permission-abuse, gate-03, gate-02]

requires:
  - phase: 03-tool-permission-abuse-cat-01-33
    provides: PI050-PI059 (CAT-01 tool-permission-abuse category), the GATE-03 sweep methodology and script, the PATTERNS.md relaxed_pattern/counter_example contract
provides:
  - Negation-blind PI053/PI056/PI057 arms tightened to clause-initial / enumerated-filler guards
  - Six pinned unit-test regression negatives for the reviewer-reproduced prohibition sentences
  - PATTERNS.md Categories table row for tool_permission_abuse (WR-01)
affects: [pattern-library, spec-ci-plugin (consumes PI050-PI059 findings via install-hook)]

actuals:
  tokens: 8300
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "Negation guard via clause-initial anchoring: (?:^|\\n|[.;:!?]\\s+)\\s*(?:[-*+]\\s+)? immediately before a directive verb, when the negator sits before the whole span"
    - "Negation guard via enumerated filler set: replace an open [^.\\n]{0,N} gap with a closed set of modal/hedge words the negator is not a member of, when the negator sits inside the span"

key-files:
  created: []
  modified:
    - patterns/core/tool-permission-abuse.yaml
    - tests/pattern_test.rs
    - docs/PATTERN-CATALOGUE.md
    - PATTERNS.md
    - .github/code-scanning-baseline.json

key-decisions:
  - "Structural tightening (option b), not an engine-side negation guard (option a) — locked in the plan's decision_record, re-verified rather than revisited"
  - "counter_example tracks the most recently added narrowing: each pattern's counter_example was promoted to its own negation-guard specimen; the displaced counter_examples were kept as pinned tests/pattern_test.rs negatives rather than discarded"
  - "GATE-03's literal --compare exit code (1) was investigated rather than treated as a blocking regression, following the same self-repo compare-key methodology 03-SWEEP.md already established for this phase — full audit trail in 260902-jhy-SWEEP.md"

requirements-completed: [CR-01, WR-01]

duration: ~55min
completed: 2026-09-02
status: complete
---

# Quick Task 260902-jhy: Fix CR-01 negation blindness in PI053/PI056/PI057

**Closed a HIGH-severity false-positive class where PI053, PI056 and PI057 fired on prohibitions ("Never run with --dangerously-skip-permissions...") by requiring the directive verb to be clause-initial or bridged only by an enumerated non-negating filler set, instead of an open unguarded gap.**

## Performance

- **Duration:** ~55 min
- **Started:** 2026-09-02T12:05Z (approx, first Read)
- **Completed:** 2026-09-02T12:33Z
- **Tasks:** 3/3
- **Files modified:** 11 (across 3 commits)

## Accomplishments

- All six reviewer-reproduced prohibition sentences from `03-REVIEW.md`'s CR-01 now produce zero `PI053`/`PI056`/`PI057` findings, pinned as permanent unit-test negatives.
- GATE-02 recall unchanged and exact: 7/7 prose, 5/5 structural, `EXPECTED` unedited.
- GATE-05 holds for all three patterns: each shipped pattern misses its (promoted) `counter_example`, its `relaxed_pattern` catches it.
- `docs/PATTERN-CATALOGUE.md` and `.github/code-scanning-baseline.json` regenerated in step with the pattern change.
- `PATTERNS.md`'s Categories table now claims `PI050-PI059` (WR-01), closing the gap the phase-03 review flagged.
- GATE-03 re-swept over the same 22-directory list (21 third-party/reference + this repo): zero third-party findings for any PI05x pattern, in either run, confirmed by direct enumeration — full audit in `260902-jhy-SWEEP.md`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Close the negation gap in PI053, PI056 and PI057** - `d157783` (fix)
2. **Task 2: Re-derive the coupled artifacts, run GATE-02 and GATE-05, add the WR-01 row** - `fdc51cb` (docs)
3. **Task 3: GATE-03 — re-sweep the third-party corpus and record the delta** - `f81906f` (docs)

_Task 1's Step 0 (pre-change GATE-03 baseline sweep) is folded into the `d157783` commit — the sweep was captured from unmodified HEAD (`26fc6af`) before the pattern edit, per the plan's ordering constraint._

## Files Created/Modified

- `patterns/core/tool-permission-abuse.yaml` - Tightened PI053 (both arms), PI056, and PI057 (both arms) with the negation guard; promoted each `counter_example` to the reviewer's prohibition sentence
- `tests/pattern_test.rs` - Six new pinned negatives (the reviewer sentences plus the displaced old counter_examples)
- `docs/PATTERN-CATALOGUE.md` - Regenerated from the tightened patterns
- `PATTERNS.md` - Added the `tool_permission_abuse` / `PI050-PI059` row to the Categories table (WR-01)
- `.github/code-scanning-baseline.json` - Regenerated (digests and line numbers moved with the pattern edit)
- `.planning/quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/260902-jhy-SWEEP.md` - GATE-03 before/after sweep record and audit
- `.planning/quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/sweep-before/{manifest,summary}.tsv`, `sweep-after/{manifest,summary}.tsv` - Committed sweep evidence (per-directory JSON reports kept on disk, not committed, per the `sweep-2026-09-02/` precedent)
- `.planning/quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/deferred-items.md` - Two pre-existing, unrelated whole-repo self-scan findings logged out of scope

## The three regex changes, final form

**PI053** — both arms' directive verb made clause-initial (the negator in the reviewer's sentences sits before the whole span in both arms):

```regex
(?:(?:^|\n|[.;:!?]\s+)\s*(?:[-*+]\s+)?(?:run|restart|re-?launch)\b\s+(?:with|in|using)\s+[^.\n]{0,10}?(?:--dangerously-skip-permissions|--yolo|--yes-always)\b|(?:^|\n|[.;:!?]\s+)\s*(?:[-*+]\s+)?(?:set|switch|change|configure)\b[^.\n]{0,25}?\bpermission\s*mode\b[^.\n]{0,20}?\bto\b[^.\n]{0,15}?\bbypass\s*permissions?\b)
```

**PI056** — a required directive verb prepended before the settings-file noun (the shipped pattern previously had no verb requirement at all):

```regex
(?:^|\n|[.;:!?]\s+)\s*(?:[-*+]\s+)?(?:update|edit|modify|change|add\s+to|open|write)\b[^.\n]{0,15}?\b(?:settings\.json|settings\s+file|your\s+settings)\b[^.\n]{0,55}?\b(?:permissions?\s*key|allow\s*list|auto-?approve|bypass\s*mode|unrestricted\s*(?:access|grant|permission)|no\s+longer\s+require\s+approval|without\s+(?:further\s+)?approval|skip\s+approval)\b
```

**PI057** — arm 1 (negator before the span) made clause-initial; arm 2 (negator inside the span, between "you" and the verb) had its open gap replaced with an enumerated modal/hedge filler set that "never"/"not" are deliberately not members of:

```regex
(?:^|\n|[.;:!?]\s+)\s*(?:[-*+]\s+)?(?:turn\s+off|disable)\b[^.\n]{0,20}?\b(?:hook|guardrail|check|scanner|gate)\b[^.\n]{0,25}\byou\b|\byou\b\s+(?:can|may|should|must|will|could|might|need\s+to)?\s*(?:just\s+|simply\s+|please\s+)?(?:turn\s+off|disable)\b[^.\n]{0,20}?\b(?:hook|guardrail|check|scanner|gate)\b
```

## Decisions Made

- **Approach locked at plan time, re-verified rather than revisited**: structural tightening (option b), not an engine-side post-match negation check (option a). The decision record's rationale (GATE-02 recall risk, 48-pattern blast radius, no lookaround in the Rust `regex` crate, decoded-layer pass using `is_match` not `find_iter`) held; all six sentences plus all existing positives/negatives behaved exactly as the plan's Python-verified design predicted once translated to Rust.
- **`counter_example` tracks the most recently added narrowing**: each pattern's `counter_example` was replaced with the negation-guard specimen (the reviewer's own sentence), and the old counter_example moved to `tests/pattern_test.rs` as a pinned negative rather than being discarded — nothing lost, GATE-05 now mutation-tests the negation guard specifically instead of the prior narrowing.
- **GATE-03's literal exit code was investigated, not treated as an automatic stop**: `scripts/gate03-sweep.sh --compare` exits 1 due to self-repo artifacts (a stale gitignored worktree, line-shift of pre-existing content, and a pre-existing multi-line block-scan line-miscalculation duplicate — proven present for PI054, a pattern this task never touched, in both before and after runs). Following the exact self-repo compare-key methodology `03-SWEEP.md` already documented for this phase, every one of the 16 "new" and all "removed" lines was individually traced to a non-regression cause. Zero third-party findings for any PI05x pattern in either run, confirmed by direct enumeration of all 21 reference directories' JSON reports. Full evidence in `260902-jhy-SWEEP.md`.

## Deviations from Plan

### Auto-fixed / logged issues

**1. [Scope boundary — logged, not fixed] Two pre-existing whole-repo self-scan findings in `docs/PATTERN-CATALOGUE.md`**
- **Found during:** Task 2's whole-repo self-scan step
- **Issue:** `docs/PATTERN-CATALOGUE.md:74` (PI001 matching PI002's own `description` text) and `:903` (PI031 matching PI031's own catalogued example prose) are outside `examples/`, `patterns/`, `tests/`, `tools/` and are not clean
- **Why not fixed:** Confirmed present verbatim at the same lines in the catalogue committed at `26fc6af` — before this task's first edit. Unrelated to PI053/PI056/PI057/WR-01; fixing them means editing PI001/PI002/PI031, out of scope per the executor's scope-boundary rule
- **Logged:** `.planning/quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/deferred-items.md`

**2. [Investigation, not a fix] GATE-03 `--compare` literal exit code**
- **Found during:** Task 3
- **Issue:** The plan's `<done>` criterion reads "compare exits 0"; the actual exit code is 1
- **Resolution:** Not a code change — a full audit proving the non-zero exit is entirely self-repo compare-key noise (stale worktree, line-shift, block-pass duplicate) with zero third-party impact, documented in `260902-jhy-SWEEP.md` per the precedent `03-SWEEP.md` already set for this exact situation in this exact phase
- **Committed in:** `f81906f`

---

**Total deviations:** 2 (1 logged/deferred, 1 investigated-and-documented). **Impact on plan:** No scope creep; both are transparency items, not corrections to CR-01/WR-01's actual scope.

## Issues Encountered

The GATE-03 after-sweep initially looked like a stop signal (`--compare` returned 16 "new" lines, non-zero exit). Full investigation (detailed in `260902-jhy-SWEEP.md`) traced every line to: (a) a stale, gitignored `.claude/worktrees/` copy of this repository left over from an unrelated prior session, (b) line-number shifts of pre-existing `example`/positive/comment text caused by this task's own added lines, and (c) a pre-existing multi-line block-scan pass that reports a second, independently-miscalculated line number for some matches — proven pre-existing and unrelated to this task's regex changes by finding the identical duplicate-at-shifted-line behavior for PI054, a pattern this task never touched. Zero third-party findings changed in either direction, confirmed by directly enumerating every one of the 21 reference directories' JSON reports for PI053/PI054/PI055/PI056/PI057 in both runs.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

CR-01 and WR-01 are closed. The negation-guard mechanism (clause-initial anchoring / enumerated filler set) is stated as a reusable design rule in the plan's decision record for future CAT-02 (`PI060-069`) and CAT-03 (`PI070-079`) work. The accepted recall narrowing (clause-initial anchoring is blind to mid-sentence directives like "Please run with --yolo") is recorded there and bounded by GATE-02's exact 7/7 recall confirmation — no concrete evasion of this shape was found during execution, so no `docs/DETECTION-BACKLOG.md` entry was added for it.

---
*Quick task: 260902-jhy*
*Completed: 2026-09-02*

## Self-Check: PASSED

All 7 files (patterns/core/tool-permission-abuse.yaml, tests/pattern_test.rs, docs/PATTERN-CATALOGUE.md, PATTERNS.md, .github/code-scanning-baseline.json, 260902-jhy-SWEEP.md, deferred-items.md) confirmed present on disk. All 3 commit hashes (d157783, fdc51cb, f81906f) confirmed present in `git log`.
