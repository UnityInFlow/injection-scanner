# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-2026-09-03/RAW-REPORTS.md`, `sweep-after-04-05-2026-09-07/RAW-REPORTS.md`
and `sweep-after-04-06-2026-09-07/RAW-REPORTS.md`: the 32 per-directory JSON reports
`scripts/gate03-sweep.sh` wrote for this run inventory one developer machine (plugin caches,
editor state, temp directories) and carry local paths and usernames, so they are kept out of the
public history.

- Committed here: `manifest.tsv`, `summary.tsv` and `checksums.sha256` (SHA-256 of every raw
  report, so a local copy can be verified).
- Kept locally, gitignored: `.planning/local/sweep-after-04-review-fixes-2026-09-07/*.json`.
- This is the `/gsd-code-review --fix` candidate: this branch's tree after every CR-* and WR-*
  finding in `04-REVIEW.md` was applied (`PI063` comment fix, `PI064` documented cost, `PI065`
  before-using/calling narrowing, `PI066` Arm B/C tool-shape narrowing, `PI067` Arm C
  clause-boundary anchor and documented residual gap, `PI068`/`PI069` enumerated
  "automatically <verb>" set), built with the release binary in the working tree.
- **Pre-fix baseline for this comparison:** `sweep-after-04-06-2026-09-07/` — the last sweep
  captured on this branch before the review-fix pass, per the orchestrator's instruction that a
  reused baseline isolates exactly this pass's delta from everything that came before.
- **Result: zero delta in both directions.** `scripts/gate03-sweep.sh --compare` from
  `sweep-after-04-06-2026-09-07` to this directory, and the reverse, both returned empty (exit 0)
  — 519 findings in both runs, byte-for-byte identical file/line/pattern-id triples. The
  `04-REVIEW.md` findings were all constructed near-miss sentences the reviewer wrote to
  demonstrate a gap in the regex; none of the six real false-positive/negation-guard shapes fixed
  in this pass (CR-01, WR-01, WR-02, WR-03) happened to occur naturally anywhere in this
  32-directory, 23,774-file real-world sweep. This is the expected, ideal outcome for a
  review-fix pass: no regression introduced, and no observed real-world finding was gained or
  lost by the narrowing.
- File count drifted slightly from `sweep-after-04-06-2026-09-07`'s 23,770 to this run's 23,774 —
  ordinary machine churn between the two capture dates in `~/.claude/plugins/cache` (984 → 986)
  and the re-created `$SCRATCH/cursor-safe` specimen (755 → 757 files, both excluding
  `extensions/`), within the same drift band `04-SWEEP.md` already names for this directory pair.
  Every other directory's file count is byte-for-byte identical to `sweep-after-04-06-2026-09-07`.

## Paths in this directory are redacted

Same convention as the prior sweep directories. `manifest.tsv` and `checksums.sha256` carry two
placeholders in place of absolute paths:

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$SCRATCH` | the session scratchpad under the system temp directory |

Nothing else changed: the per-directory list, the file and finding counts, and every SHA-256 sum
are byte-for-byte the values the run produced, and this run's directory list is identical to
`sweep-after-04-06-2026-09-07`'s (same 32 rows, same order, reproduced from that run's own
`manifest.tsv`).
