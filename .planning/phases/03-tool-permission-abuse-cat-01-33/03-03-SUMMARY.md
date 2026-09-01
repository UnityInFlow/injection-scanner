---
phase: 03-tool-permission-abuse-cat-01-33
plan: 03
subsystem: testing
tags: [gate-03, sweep, roadmap, bash, self-scan]

requires:
  - phase: 03-tool-permission-abuse-cat-01-33
    provides: "48-pattern release binary (ENG-01 + ENG-02 shipped), no scope:frontmatter pattern yet"
provides:
  - "scripts/gate03-sweep.sh — committed, self-tested GATE-03 sweep + --compare procedure"
  - "03-SWEEP.md — pre-pattern baseline (22 directories, 3,499 files, 518 findings) with a reserved after-sweep section"
  - "ROADMAP.md Phase 3 criteria corrected to match D-10/D-11/D-16"
affects: [03-07-PLAN.md]

actuals:
  tokens: 5410
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "GATE-03 sweep script: per-directory JSON reports + manifest.tsv/summary.tsv, --compare mode diffs two runs keyed on (file, line, pattern_id)"
    - "Machine-local corpus is enumerated and recorded per run rather than pinned to a fixed vendored list"

key-files:
  created:
    - scripts/gate03-sweep.sh
    - .planning/phases/03-tool-permission-abuse-cat-01-33/03-SWEEP.md
  modified:
    - .planning/ROADMAP.md

key-decisions:
  - "GATE-03's ~1,300-file historical sweep is not reproducible from this repo (no script, no vendored corpus, no CI job) — accepted as machine-local by design; the script's real output is a --compare delta over a named, counted directory list, not an absolute 'zero findings' claim"
  - "Candidate directories mapped to the historical sweep's three families: this repo itself, local agent-tooling caches (~/.claude/plugins/cache, ~/.claude/skills, ~/.claude/gsd-core), and 19 sibling UnityInFlow repos swept individually (not the parent workspace as one tree, to avoid double-counting this repo nested inside it)"
  - "ROADMAP's S005 criterion and '10 patterns' criterion corrected per D-10/D-11/D-16, with a one-line note recording the correction's source so it is not re-inherited"

requirements-completed: [GATE-03, CAT-01]

coverage:
  - id: D1
    description: "Committed, self-tested GATE-03 sweep script (sweep + --compare modes)"
    requirement: "GATE-03"
    verification:
      - kind: integration
        ref: "bash scripts/gate03-sweep.sh /tmp/sweep-selftest tests/corpus/clean (zero findings, matches corpus_test)"
        status: pass
      - kind: integration
        ref: "bash scripts/gate03-sweep.sh /tmp/sweep-selftest2 tests/corpus/attack (78 findings, non-zero)"
        status: pass
      - kind: integration
        ref: "bash scripts/gate03-sweep.sh --compare (self-compare exits 0; clean-vs-attack exits 1 and lists findings)"
        status: pass
      - kind: integration
        ref: "cargo run --release -- check scripts --strict --format json (and --all-files variant) both return an empty match list"
        status: pass
      - kind: other
        ref: "bash scripts/gate03-sweep.sh (no args) exits 2 with usage; missing-directory case exits 0 and records a skip"
        status: pass
    human_judgment: false
  - id: D2
    description: "Recorded pre-pattern sweep baseline over a named, counted directory set (22 directories, 3,499 files, 518 findings), including the 12 pre-existing self-scan findings inside this repo outside examples/patterns/tests/tools"
    requirement: "GATE-03"
    verification:
      - kind: manual_procedural
        ref: ".planning/phases/03-tool-permission-abuse-cat-01-33/03-SWEEP.md — directory list, manifest/summary tables, reproduction command, pre-existing findings table"
        status: pass
    human_judgment: false
  - id: D3
    description: "ROADMAP Phase 3 criteria corrected to match locked decisions D-10, D-11, D-16"
    requirement: "CAT-01"
    verification:
      - kind: unit
        ref: "grep -c '^### Phase' .planning/ROADMAP.md == 5; grep -q 'provenance'; git diff --stat confined to Phase 3 block + Progress row"
        status: pass
    human_judgment: false

duration: 25min
completed: 2026-09-01
status: complete
---

# Phase 3 Plan 03: GATE-03 Sweep Script, Recorded Baseline, ROADMAP Correction Summary

**A committed `scripts/gate03-sweep.sh` reconstructs GATE-03's manual sweep as a runnable, self-tested procedure whose real evidence is a `--compare` delta, plus a recorded 22-directory / 3,499-file pre-pattern baseline and a ROADMAP correction that stops the superseded S005/pattern-count criteria from being re-inherited.**

## Performance

- **Duration:** ~25 min
- **Tasks:** 3
- **Files:** 2 created, 1 modified

## Accomplishments

- **`scripts/gate03-sweep.sh`** — sweeps one or more directories with the release binary
  (`--format json --all-files --no-ignore --exclude '.planning/**'`), writing one JSON report per
  directory plus `manifest.tsv` (per-directory file/finding counts, including skipped-missing
  directories) and `summary.tsv` (findings by pattern id and severity). A `--compare
  <baseline> <candidate>` mode diffs two prior runs keyed on `(file, line, pattern_id)` and exits 1
  if the candidate introduced anything the baseline didn't have — that diff is the actual GATE-03
  evidence, since the swept corpus itself is not reproducible across machines.
- **Self-tested in both directions**, matching the plan's `<verify>` exactly: zero findings on
  `tests/corpus/clean` (agreeing with `corpus_test`), 78 findings on `tests/corpus/attack`,
  self-compare exits 0, clean-vs-attack compare exits 1 and lists every new finding. The script does
  not trip the tool's own self-scan (`check scripts --strict --format json` → `[]`, and the same with
  `--all-files`).
- **`03-SWEEP.md`** — a pre-pattern baseline run over 22 candidate directories (this repo, three
  local agent-tooling caches, and 19 sibling UnityInFlow repos), all present: 3,499 files scanned,
  518 findings, git SHA and pattern count (48, no `scope:` field) recorded, exact reproduction
  command line included, and an empty section reserved for Plan 07's after-sweep. Twelve pre-existing
  findings inside this repo outside `examples/`, `patterns/`, `tests/`, `tools/` are listed by
  file/line/pattern id — two already-known (`docs/PATTERN-CATALOGUE.md:73,902`) plus ten newly
  visible under `--all-files` scanning `.rs` doc comments, code comments and unit-test literals
  (`benches/scan.rs`, `src/baseline.rs`, `src/normalize.rs`, `src/scanner.rs`) — so Plan 07's delta
  is not confused by them.
- **ROADMAP Phase 3 criteria corrected**: the "10 patterns" criterion replaced with D-16's wording
  (twelve corpus payloads; pattern count recorded after the fact, not targeted), and the S005
  boundary criterion replaced with D-10's wording (a frontmatter-scoped `PI05x` pattern DOES fire on
  a file's own wildcard grant — the boundary is provenance, not phrasing). A one-line note records
  that both corrections come from `03-CONTEXT.md` D-11/D-16. Diff confined to the Phase 3 block and
  its Progress row (now "In progress"); Phases 1, 2, 4 and 5 are byte-identical.

## Task Commits

1. **Task 1: Build the committed sweep script** - `0c3c956` (feat)
2. **Task 2: Record the pre-pattern sweep over a named directory set** - `aa83292` (docs)
3. **Task 3: Correct the ROADMAP's Phase 3 criteria (D-10, D-11, D-16)** - `5a17e0c` (docs)

## Files Created/Modified

- `scripts/gate03-sweep.sh` - Committed GATE-03 sweep + `--compare` script (sweep mode and diff mode)
- `.planning/phases/03-tool-permission-abuse-cat-01-33/03-SWEEP.md` - Recorded pre-pattern baseline: directory list, manifest/summary tables, reproduction command, pre-existing findings, reserved after-sweep section
- `.planning/ROADMAP.md` - Phase 3 success criteria corrected per D-10/D-11/D-16; Progress row updated

## Decisions Made

- The historical ~1,300-file sweep is genuinely not reproducible from this repository (no script,
  no vendored corpus, no CI job existed for it) — accepted as machine-local by design rather than
  forced into a fake fixed corpus. The script's real output is a `--compare` delta, not an absolute
  finding count.
- Candidate directories were chosen to map onto the historical sweep's three families
  (`~/.claude/plugins/cache`, "the GSD workflow reference set", "seven sibling repositories") using
  what is actually present on this machine: `~/.claude/plugins/cache`, `~/.claude/skills`,
  `~/.claude/gsd-core`, and all 19 sibling UnityInFlow tool repositories (swept individually rather
  than as one parent-workspace tree, to avoid double-counting this repository, which is nested
  inside that same parent directory).
- The sweep script always passes `--all-files`, which reaches `.rs` source files the
  pattern-library skill's narrower default one-liner does not scan (`.rs` is not in
  `DEFAULT_EXTENSIONS`). This surfaced ten additional pre-existing self-scan findings beyond the two
  already documented in `03-CONTEXT.md` — all illustrative payload text in doc comments, code
  comments, or unit-test literals, none of them new attacks. Recorded in `03-SWEEP.md` rather than
  fixed, since fixing pattern-library conventions is out of this plan's scope under GATE-04.

## Deviations from Plan

None - plan executed exactly as written. The specific set of "sibling tool repositories" and
"local agent-tooling caches" swept was a discovery-time decision (the plan asked for discovery, not
a fixed list), documented above and in `03-SWEEP.md`'s "Candidates considered" table.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `scripts/gate03-sweep.sh` and the pre-pattern baseline in `03-SWEEP.md` are ready for Plan 07 to
  re-run (`--compare` against `/tmp/gate03-baseline` — note this is a machine-local `/tmp` path, not
  committed; Plan 07 must reproduce it with the recorded command line on the same machine, or record
  a fresh baseline if run elsewhere) once the CAT-01 patterns land.
- ROADMAP's Phase 3 criteria now match the locked decisions, so 03-04 through 03-07 can proceed
  without re-deriving or contradicting D-10/D-11/D-16.
- No blockers. This plan had no dependencies (`depends_on: []`) and ran independently of sibling
  plans 03-01/03-02 in this wave.

## Self-Check: PASSED

- `scripts/gate03-sweep.sh` exists and is executable (`test -x` succeeds)
- `.planning/phases/03-tool-permission-abuse-cat-01-33/03-SWEEP.md` exists and is non-empty
- All three task commits (`0c3c956`, `aa83292`, `5a17e0c`) found in `git log --oneline --all`
- Task 1 acceptance criteria re-verified: usage/exit 2 with no args; clean corpus → zero findings;
  attack corpus → non-zero findings; self-compare exits 0; clean-vs-attack compare exits 1 and lists
  findings; missing-directory case exits 0 and records the skip; self-scan (`--strict` and
  `--all-files` variants) returns an empty match list; `chmod +x` applied
- Task 2 acceptance criteria re-verified: `03-SWEEP.md` names 22 directories with counts, git SHA
  and reproduction command present, total file count (3,499) stated as a number, 12 pre-existing
  findings listed with file/line/pattern id, `/tmp/gate03-baseline/manifest.tsv` and `summary.tsv`
  match the tables in `03-SWEEP.md`, empty after-sweep section present, `grep -rn 'scope:'
  patterns/core/*.yaml` returns nothing
- Task 3 acceptance criteria re-verified: `git diff --stat .planning/ROADMAP.md` confined to Phase 3
  block + Progress row; `provenance` present; `rather than duplicating it` absent; `twelve` present;
  D-11/D-16 note present; `grep -c '^### Phase'` = 5
- Plan-level `<verification>` re-run: script self-tests pass in both directions; `03-SWEEP.md` names
  more than three directories with counts and a reproduction command; ROADMAP still has five phase
  blocks with the diff confined to Phase 3

---
*Phase: 03-tool-permission-abuse-cat-01-33*
*Completed: 2026-09-01*
