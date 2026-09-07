# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md` and `sweep-after-04-05-2026-09-07/RAW-REPORTS.md`:
the 32 per-directory JSON reports `scripts/gate03-sweep.sh` wrote for this run inventory one
developer machine (plugin caches, editor state, temp directories) and carry local paths and
usernames, so they are kept out of the public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-after-04-06-2026-09-07/*.json`.
- This is the plan 04-06 candidate: this branch's tree after all three tasks (`PI066`
  cross-tool-shadowing, `PI067` tool-override-directive, `PI068` version-conditional-directive,
  `PI069` deferred-activation-directive shipped), built with the release binary in the working
  tree.
- This run was captured TWICE, same reason as 04-05's run. The first capture found 26 real
  additions relative to `sweep-after-04-05-2026-09-07` — one `PI066` and twenty-five `PI067`
  additions, ALL false positives, all ordinary "never do X, always do Y" style-guide advice with
  no tool-substitution or cross-tool-shadowing content whatsoever. Both patterns' first drafted
  form keyed only on trigger words (never/always/instead-of, when/calls) with no requirement on
  WHAT was being used/called or invoked. Both were re-narrowed to require a tool-shaped object
  (a backtick-quoted code span, a snake_case identifier, or an identifier immediately followed by
  `()`) directly after the relevant verb, and this directory holds the re-narrowed run — see the
  "04-06" section of `04-SWEEP.md` for the full before/after counts and adjudication.

## Paths in this directory are redacted

Same convention as `sweep-after-04-05-2026-09-07/RAW-REPORTS.md`. `manifest.tsv` and
`checksums.sha256` carry two placeholders in place of absolute paths:

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$SCRATCH` | the session scratchpad under the system temp directory |

Nothing else changed: the per-directory list, the file and finding counts, and every SHA-256 sum
are byte-for-byte the values the run produced, and this run's directory list is identical to
`sweep-after-04-05-2026-09-07`'s and to the 04-01 baseline's.
