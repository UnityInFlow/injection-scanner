# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-05-01-2026-10-08/` and `sweep-after-05-03-2026-10-08/`: the 33 per-directory
JSON reports this run wrote inventory one developer machine and carry local paths, so they are kept out of
the public history.

- Committed here: `manifest.tsv`, `summary.tsv`, `checksums.sha256` (SHA-256 of every raw report,
  filenames redacted), and this file. Zero `*.json` (checked: `find` over the phase directory prints `0`).
- Kept locally, gitignored: `.planning/local/sweep-after-05-04-2026-10-08/*.json` (33 files) in the main
  checkout.
- **Never point `--compare` at this directory.** It has no `*.json`, so it loads a silently empty set and
  reports every real finding as an addition. Both `--compare` runs below used the `.planning/local/`
  directories.

## What this capture is

Phase 5 plan 05-04, GATE-03 candidate sweep for the two CAT-03 pattern commits of this plan: `PI072`
(`4aa7d78`) and `PI074` + `PI075` (`f87bf6a`). Compared against plan 05-01's pre-edit baseline, which
remains the comparison target for every pattern plan in this phase.

| Field | Value |
|---|---|
| Binary built from | HEAD `f87bf6a4a76bee86e88b81abbffdf01b2b28ffa1` (`feat(05-04): add PI074 memory-write-directive and PI075 conditional-memory-write`), after `4aa7d78` (`PI072`) |
| Build | `cargo build --release --locked`; `git status --porcelain -- src patterns Cargo.toml Cargo.lock` printed nothing; binary SHA-256 `5f523a37095f4ee3...` |
| Pattern set | 75 patterns (baseline: 71): `PI070`/`PI071` from plan 05-03 plus `PI072`, `PI074`, `PI075` |
| Rows | 33, identical row list to the baseline (taken from its `manifest.tsv`, column 1) |
| Files scanned | 26,417 |
| Findings | 1,306 |
| Input paths | the three repo-local rows by the main checkout's literal absolute paths (`$REPO/.planning/local/sweep-inputs-ext-05/{cursor,vscode}`, `$REPO/.planning/local/sweep-inputs-hooks-05`); the other thirty by their `$HOME` paths. No input set was copied into the executor's worktree and the baseline was not re-captured. |

`$HOME` and `$REPO` stand for the developer's home directory and the main checkout root. Counts are untouched.

## `--compare`, both directions, quoted verbatim

Both arguments are `.planning/local/` directories in the main checkout (33 raw reports each, counted before
the run).

```
$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-baseline-05-01-2026-10-08 $REPO/.planning/local/sweep-after-05-04-2026-10-08
<no output>
rc=0

$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-after-05-04-2026-10-08 $REPO/.planning/local/sweep-baseline-05-01-2026-10-08
<no output>
rc=0
```

Additions: 0. Removals: 0. Nothing to adjudicate.

Supporting checks, run on the same two directories:

- `manifest.tsv` columns 1-3 (directory, files, findings) hash identically for baseline and candidate
  (`0c2215eab66671efc4ddeb16050a8d98` on both); the redacted `manifest.tsv` committed here is
  `diff`-identical to plan 05-03's, and its column-1 row keys are identical to the baseline's.
- `summary.tsv` is `diff`-identical to the baseline's and contains no `PI07x` row: none of the five
  CAT-03 patterns fires on any of the 26,417 files.
- Counting at every confidence, `PI072`, `PI074` and `PI075` produce 0 reported findings and 0
  low-confidence findings across all 33 reports.
- `checksums.sha256` here is `diff`-identical to plan 05-03's, which is what an unchanged finding set
  predicts.
- **The comparison is not vacuous.** A scratch pair built from the largest real report, with a single
  finding deleted from one copy, made `--compare` print that finding and exit `1`
  (`.../vscode/berublan.vscode-log-viewer-0.14.1/README.md:95	PI026`, `rc=1`).

## Adjudication

Empty: there are no additions to adjudicate. This is the plan's named risk, so it is worth being plain
about what the result does and does not show. The memory arm (`PI074`) is the one that was measured
firing on a document nobody wrote for it (this repository's own backlog bullet, fixed with a code span in
the same commit), and it produces no finding on any of the 26,417 third-party files, including the
5,806-file `~/.claude/plugins/marketplaces` row and the 10,741-file `~/.codex/.tmp` row, which are the
two largest rows and the ones most likely to hold agent and memory documentation. No third-party document tripped the memory arm, the
self-propagation arm or the conditional arm.

It is also uninformative about recall in the wild: zero findings means zero false positives on these
inputs and also zero true positives.
