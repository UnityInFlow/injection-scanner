# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md`, `sweep-after-04-05-2026-09-07/RAW-REPORTS.md`
and `sweep-after-04-06-2026-09-07/RAW-REPORTS.md`: the 32 per-directory JSON reports
`scripts/gate03-sweep.sh` wrote for this run inventory one developer machine (plugin caches,
editor state, temp directories) and carry local paths and usernames, so they are kept out of
the public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-final-2026-09-03/*.json`.
- This is plan 04-07 Task 1's final, whole-category candidate: `HEAD` (`62a5a27`), whose
  `src/` and `patterns/` are byte-identical to 04-06's finished tree (`a91c5e2`) — the one
  commit between them is documentation-only. Built with the release binary in the working
  tree, over the SAME 32-directory list `sweep-mainbase-04-05-2026-09-07` and
  `sweep-after-04-06-2026-09-07` established (`manifest.tsv`'s directory column, file counts
  and finding counts are row-for-row identical to `sweep-after-04-06-2026-09-07`'s, and
  `checksums.sha256` is byte-identical set-for-set).
- **This run's own findings are identical to `sweep-after-04-06-2026-09-07`'s** — same 32
  directories, same 23,770 files, same 519 raw findings, same SHA-256 set. No source file
  changed between the two captures, so nothing could move.

## Paths in this directory are redacted

Same convention as the three prior sweep directories in this phase. `manifest.tsv` and
`checksums.sha256` carry two placeholders in place of absolute paths:

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$SCRATCH` | the session scratchpad under the system temp directory |

Nothing else changed: the per-directory list, the file and finding counts, and every SHA-256
sum are byte-for-byte the values the run produced.

## Whole-category (whole-PR) GATE-03 delta — the comparison this task's verdict rests on

The orchestrator's instruction for this task named `sweep-mainbase-04-05-2026-09-07` (built
from `66bf53c`, this branch's fork point, which already carries `PI060`-`PI062` merged to
`main` via plan 04-04's own branch) as the correct pre-phase baseline for the whole-category
delta — not the plan's originally-named `sweep-baseline-2026-09-03` (04-01, three
pattern-set generations stale: it predates PR #110, #122, #124-#127 and 04-04 itself, so a
straight comparison against it would attribute all of that unrelated history to this PR).

**Both directions against `sweep-mainbase-04-05-2026-09-07`, using its real per-directory
JSON preserved at `.planning/local/sweep-mainbase-04-05-2026-09-07/`: empty.** Zero
additions, zero removals. `PI063`-`PI069` (this branch's payload) and the unmodified
`PI060`-`PI062` add and remove nothing on ~23,770 real third-party files.

**Independently corroborated with a freshly-built pre-edit binary**, per the established
04-05 technique ("build a FRESH pre-edit binary from the branch's own fork point in a
separate git worktree when main has moved multiple generations past the last committed
baseline") — even though `sweep-mainbase-04-05-2026-09-07` was itself already a same-generation
baseline for this exact fork point, a second independent binary build from `66bf53c` in a
disposable `git worktree` was swept over the same 32-directory list, in the same session as
this candidate capture, to rule out any capture-time skew. Both directions against that
independent fresh capture: also empty. The two independent baselines (the committed one and
the freshly rebuilt one) agree exactly.

**A methodological note worth carrying forward.** An earlier attempt to compare directly
against the *repository-committed* copies of `sweep-baseline-2026-09-03` and
`sweep-mainbase-04-05-2026-09-07` produced a spurious 500-line "diff" — every finding in
this run showing up as a false "addition". Root cause: this repository deliberately does not
commit the raw per-directory JSON (`dad56d1`, `e54be72`) — only `manifest.tsv`,
`summary.tsv` and `checksums.sha256` are public. `scripts/gate03-sweep.sh --compare` loads
findings by globbing `*.json` in each directory; against the committed (JSON-less) copy, the
"baseline" side loads as an empty set, so every real finding in the candidate reads as a
"new" one. The real, gitignored JSON still lives at `.planning/local/<sweep-dir>/*.json` on
this machine (never deleted, only kept out of git) — re-running `--compare` against
`.planning/local/sweep-baseline-2026-09-03` and `.planning/local/sweep-mainbase-04-05-2026-09-07`
produced the correct, adjudicated results recorded in `04-SWEEP.md`. **Always point
`--compare` at a directory that still has its raw JSON present — `.planning/local/`, not the
repository's redacted copy** — or the result is meaningless by construction, not merely
noisy.

## Continuity comparison against `sweep-baseline-2026-09-03` (04-01), for the record

Recorded in full, with adjudication, in `04-SWEEP.md`'s "Phase 4 Plan 07" section — same
two entries in the additions direction (the pre-existing `PI060` true positive, the `PI011`
scratch-path-churn artifact) and the same 67 in the removals direction (`PI026` 39 + `PI017`
27, both PR #110's pre-existing delta; `PI011` 1, the path-churn counterpart) that 04-05's
and 04-06's own continuity sections already adjudicated. Nothing in either direction is
attributable to this plan.
