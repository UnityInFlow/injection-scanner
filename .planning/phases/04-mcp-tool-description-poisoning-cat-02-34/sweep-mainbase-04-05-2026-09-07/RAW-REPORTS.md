# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md`: the 32 per-directory JSON reports
`scripts/gate03-sweep.sh` wrote for this run inventory one developer machine (plugin caches,
editor state, temp directories) and carry local paths and usernames, so they are kept out of the
public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-mainbase-04-05-2026-09-07/*.json`.
- This is the FRESH pre-edit baseline for plan 04-05, built from a release binary compiled at
  `66bf53c` (`git worktree add` into a scratch directory, no `git stash` used) — the commit this
  branch forked from, PR #88's counter_example addition merged, zero `PI063`-`PI065` patterns.
  `sweep-baseline-2026-09-03` (04-01, `b4f05ef`) and `sweep-mainbase-04-04-2026-09-06` (`0d50e92`)
  are both now stale for this plan's purposes: `main` moved through issue #122's two launcher-
  vocabulary widenings, #125 (unique test temp dirs), #126 (PI028/PI062 widened further,
  `security-runbook.md` grew) and #127 (PI012/PI013 counter_examples, catalogue/baseline
  regenerated) since 04-04 shipped. Comparing straight to either earlier baseline would attribute
  all of that intervening history to this plan, exactly the misattribution 04-04 itself had to
  unpick for PR #110. This run is the correct isolating reference: identical pattern set to the
  candidate (`sweep-after-04-05-2026-09-07/`) except for the three prose arms this plan adds.
- The sibling run `sweep-after-04-05-2026-09-07/` is the candidate — this branch's tree after
  Task 2, using the release binary in the working tree. See the "04-05" section of `04-SWEEP.md`
  for the full two-directional adjudication.

## Paths in this directory are redacted

Same convention as `sweep-mainbase-122-2026-09-07/RAW-REPORTS.md`. `manifest.tsv` and
`checksums.sha256` carry two placeholders in place of absolute paths:

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$SCRATCH` | the session scratchpad under the system temp directory |

Nothing else changed: the per-directory list, the file and finding counts, and every SHA-256 sum
are byte-for-byte the values the run produced, and both runs' directory lists are identical to
one another and to the 04-01 baseline's.
