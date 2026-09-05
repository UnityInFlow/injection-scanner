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
