# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-05-01-2026-10-08/`: the 33 per-directory JSON reports this run wrote
inventory one developer machine and carry local paths, so they are kept out of the public history.

- Committed here: `manifest.tsv`, `summary.tsv`, `checksums.sha256` (SHA-256 of every raw report,
  filenames redacted), and this file. Zero `*.json` (checked: `find` over the phase directory prints `0`).
- Kept locally, gitignored: `.planning/local/sweep-after-05-03-2026-10-08/*.json` (33 files) in the
  main checkout.
- **Never point `--compare` at this directory.** It has no `*.json`, so it loads a silently empty
  set and reports every real finding as an addition. Both `--compare` runs below used the
  `.planning/local/` directories.

## What this capture is

Phase 5 plan 05-03, GATE-03 candidate sweep for the first two CAT-03 pattern commits: `PI070`
widened (object and verb sets) and `PI071` added. Compared against plan 05-01's pre-edit baseline.

| Field | Value |
|---|---|
| Binary built from | HEAD `b0e70d72e7f78c249a27de6c193446fe32d13212` (`feat(05-03): add PI071 agent-persistence-nonmodal`), preceded by `be30f62` (the `PI070` widening) |
| Build | `cargo build --release --locked`; `git status` over `src patterns Cargo.toml Cargo.lock` was empty after the commit, and the binary was copied before that commit and is byte-identical (SHA-256 `6f8b6ebdaa9ded02...`) to the release binary of the committed tree |
| Pattern set | 72 patterns (baseline: 71); `PI070` widened, `PI071` new |
| Rows | 33, identical row list to the baseline (taken from its `manifest.tsv`, column 1) |
| Files scanned | 26,417 |
| Findings | 1,306 |
| Input paths | the three repo-local rows by the main checkout's literal absolute paths (`$REPO/.planning/local/sweep-inputs-ext-05/{cursor,vscode}`, `$REPO/.planning/local/sweep-inputs-hooks-05`); the other thirty by their `$HOME` paths. No input set was copied into the executor's worktree and the baseline was not re-captured. |

`$HOME` and `$REPO` stand for the developer's home directory and the main checkout root. Counts are untouched.

## `--compare`, both directions, quoted verbatim

Both arguments are `.planning/local/` directories in the main checkout (33 raw reports each).

```
$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-baseline-05-01-2026-10-08 $REPO/.planning/local/sweep-after-05-03-2026-10-08
<no output>
rc=0

$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-after-05-03-2026-10-08 $REPO/.planning/local/sweep-baseline-05-01-2026-10-08
<no output>
rc=0
```

Additions: 0. Removals: 0. Nothing to adjudicate. An interim sweep taken with the `PI070`-only
binary (`be30f62`, before `PI071` existed) produced the same two empty outputs.

Supporting checks, run on the same two directories:

- `manifest.tsv` columns 1-3 (directory, files, findings) hash identically for baseline and candidate
  (`0c2215eab66671efc4ddeb16050a8d98` on both), and the redacted `manifest.tsv` committed here is
  `diff`-identical to the baseline's.
- `summary.tsv` is `diff`-identical to the baseline's, and contains no `PI07x` row: neither `PI070`
  nor `PI071` fires on any of the 26,417 files.
- `checksums.sha256`: the 30 `$HOME` rows are byte-identical to the baseline's. The three `$REPO`
  rows differ, as the baseline's own path-keyed caveat predicts (those reports embed the checkout's
  absolute path, and the baseline's rows were re-captured by the orchestrator under the main checkout).
- **The comparison is not vacuous.** A scratch pair built from one real report, with a single
  finding deleted from the second copy, made `--compare` print that finding and exit `1`:
  `.../McpServerUnderTest.kt:175	PI015`, `rc=1`.

## Adjudication

Empty: there are no additions to adjudicate. The result is also uninformative about recall in the
wild: the widened `PI070` and `PI071` produce no finding on any of these 26,417 third-party files, so
the sweep measures false-positive exposure only and says nothing about true positives.
