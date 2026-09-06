# Raw JSON sweep reports are not in the repository

The 32 per-directory JSON reports that `scripts/gate03-sweep.sh` wrote for this baseline are
inventories of one developer machine (plugin caches, editor state, temp directories). They carry
local paths and usernames, so they are kept out of the public history.

- Committed here: `manifest.tsv`, `summary.tsv`, `panic-cursor-extensions.txt`, and
  `checksums.sha256` (SHA-256 of every raw report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-baseline-2026-09-03/*.json` — the
  `--compare` runs in plans 04-04 to 04-07 read from that directory on the machine that
  recorded the baseline. On any other machine, re-record a baseline from the same binary
  (`b4f05ef`) before comparing.
- Plan 04-01's acceptance line "number of `*.json` files equals the `swept` rows in
  `manifest.tsv`" therefore holds against the local directory, not this one.

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
