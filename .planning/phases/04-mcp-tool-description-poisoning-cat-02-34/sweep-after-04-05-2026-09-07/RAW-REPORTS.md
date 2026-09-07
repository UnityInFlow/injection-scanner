# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md`: the 32 per-directory JSON reports
`scripts/gate03-sweep.sh` wrote for this run inventory one developer machine (plugin caches,
editor state, temp directories) and carry local paths and usernames, so they are kept out of the
public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-after-04-05-2026-09-07/*.json`.
- This is the plan 04-05 candidate: this branch's tree after Task 2 (`PI063`
  tool-description-directive, `PI064` tool-description-file-smuggle, `PI065`
  tool-description-emphasis-block shipped), built with the release binary in the working tree.
  The sibling run `sweep-mainbase-04-05-2026-09-07/` is the fresh pre-edit baseline this run is
  compared against — see the "04-05" section of `04-SWEEP.md` for the full two-directional
  adjudication.
- This run was captured TWICE. The first capture found one true false positive — `PI063` fired
  on `get_token` (an ordinary Python function call) in a real vendored Hugging Face skill file,
  because the credential-suffix branch's `[A-Z]` character class folded case under this file's
  default case-insensitive compilation. The pattern was re-narrowed with an inline `(?-i:...)`
  case-sensitive group (the same technique `PI011` and `PI065`'s own wrapper already use), and
  this directory holds the re-narrowed run.

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
