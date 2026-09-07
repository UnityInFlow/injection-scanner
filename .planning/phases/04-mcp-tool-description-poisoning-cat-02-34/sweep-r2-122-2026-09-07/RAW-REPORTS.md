# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md`: the 32 per-directory JSON reports
`scripts/gate03-sweep.sh` wrote for this run inventory one developer machine (plugin caches,
editor state, temp directories) and carry local paths and usernames, so they are kept out of the
public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-r2-122-2026-09-07/*.json`.
- This directory is the **third** run in issue **#122**'s GATE-03 set, added by review r1's second
  widening. Its baseline is the unchanged `sweep-mainbase-122-2026-09-07/`; `sweep-after-122-2026-09-07/`
  is the first-review candidate that sits between them. See the "#122 review r1" section of
  `../04-SWEEP.md` for the two-directional adjudication.
- A third run (`sweep-r3-122-2026-09-07`, local only) was captured after the escaped-quote arm was
  added. Its 32 raw reports are **byte-identical** to this directory's, so it is not committed
  separately; see the "Third widening" subsection of `../04-SWEEP.md`.

## Paths in this directory are redacted (review #34-r1 finding 5)

`manifest.tsv` and `checksums.sha256` carry two placeholders in place of absolute paths:

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$SCRATCH` | the session scratchpad under the system temp directory |

Nothing else changed: the per-directory list, the file and finding counts, and every SHA-256 sum
are byte-for-byte the values the run produced, and this run's directory list is identical to the
other two #122 runs' and to the 04-01 baseline's.
