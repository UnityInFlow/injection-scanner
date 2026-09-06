# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md`: the 32 per-directory JSON reports
`scripts/gate03-sweep.sh` wrote for this run inventory one developer machine (plugin caches,
editor state, temp directories) and carry local paths and usernames, so they are kept out of the
public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-after-04-04-2026-09-06/*.json`.
- The sibling run `sweep-mainbase-04-04-2026-09-06/` follows the same rule. It is the run built
  from `0d50e92` — the tree this branch started from, PR #110 merged, zero CAT-02 patterns — and
  it is the correct `--compare` reference for plan 04-04's own delta. See the plan-04 section of
  `04-SWEEP.md` for why a third run was needed and for the full two-directional adjudication.

## Paths in this directory are redacted (review #34-r1 finding 5)

`manifest.tsv` and `checksums.sha256` originally carried the same absolute paths the raw JSON
reports were excluded for — the developer's username, home layout, editor and plugin locations,
unrelated private project names and a temporary session UUID — which defeated the stated purpose
of the rule above. Two placeholders are now substituted throughout, in the `.md`, the `.tsv` and
the checksum file names alike:

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$SCRATCH` | the session scratchpad under the system temp directory |

Nothing else changed: the per-directory list, the file and finding counts, and every SHA-256 sum
are byte-for-byte the values the run produced, and the three runs' directory lists remain
identical to one another. To verify a local copy of the raw reports, expand the placeholders back
before running `shasum -c` — e.g. `sed "s#^\\(.*  \\./\\)HOME_#\\1#"` against the local file names,
or compare sums directly, which the substitution does not touch.
