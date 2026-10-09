# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-05-01-2026-10-08/` and the `sweep-after-05-0N-*` directories: the 33 per-directory
JSON reports this run wrote inventory one developer machine and carry local paths, so they are kept out of the
public history.

- Committed here: `manifest.tsv`, `summary.tsv`, `checksums.sha256` (SHA-256 of every raw report, filenames
  redacted), and this file. Zero `*.json` (checked: `find` over the phase directory prints `0`).
- Kept locally, gitignored: `.planning/local/sweep-final-05-2026-10-08/*.json` (33 files) in the main checkout.
- **Never point `--compare` at this directory.** It has no `*.json`, so it loads a silently empty set and reports
  every real finding as an addition. Both `--compare` runs recorded in `../05-SWEEP.md` used the
  `.planning/local/` directories.

`$HOME` and `$REPO` stand for the developer's home directory and the main checkout root. Counts are untouched.
The full record, both `--compare` outputs and the adjudication are in `../05-SWEEP.md`.
