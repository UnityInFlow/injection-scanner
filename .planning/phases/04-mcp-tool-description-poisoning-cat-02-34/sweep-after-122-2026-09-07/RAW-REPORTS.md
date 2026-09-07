# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md`: the 32 per-directory JSON reports
`scripts/gate03-sweep.sh` wrote for this run inventory one developer machine (plugin caches,
editor state, temp directories) and carry local paths and usernames, so they are kept out of the
public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-after-122-2026-09-07/*.json`.
- This directory is one half of issue **#122**'s two-run GATE-03 pair. The other half is
  `sweep-mainbase-122-2026-09-07/`; see the "#122" section of `../04-SWEEP.md` for the two-directional adjudication.

## Paths in this directory are redacted (review #34-r1 finding 5)

`manifest.tsv` and `checksums.sha256` carry two placeholders in place of absolute paths:

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$SCRATCH` | the session scratchpad under the system temp directory |

Nothing else changed: the per-directory list, the file and finding counts, and every SHA-256 sum
are byte-for-byte the values the run produced, and both runs' directory lists are identical to one
another and to the 04-01 baseline's.
