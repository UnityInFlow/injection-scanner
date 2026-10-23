# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-05-01-2026-10-08/` and the later `sweep-after-05-0N-*` directories: the 33
per-directory JSON reports this run wrote inventory one developer machine and carry local paths, so they are
kept out of the public history.

- Committed here: `manifest.tsv`, `summary.tsv`, `checksums.sha256` (SHA-256 of every raw report, filenames
  redacted), and this file. Zero `*.json` (checked: `find` over the phase directory prints `0`).
- Kept locally, gitignored: `.planning/local/sweep-after-05-05-2026-10-08/*.json` (33 files) in the main
  checkout.
- **Never point `--compare` at this directory.** It has no `*.json`, so it loads a silently empty set and
  reports every real finding as an addition. Both `--compare` runs below used the `.planning/local/`
  directories.

## What this capture is

Phase 5 plan 05-05, GATE-03 candidate sweep for the two CAT-03 pattern commits of this plan: `PI073`
(`f9059dc`) and `PI076` (`c9bfdcb`). Compared against plan 05-01's pre-edit baseline, which remains the
comparison target for every pattern plan in this phase.

| Field | Value |
|---|---|
| Binary built from | HEAD `c9bfdcb763120540cf35f1aa819ca37e04a73768` (`feat(05-05): add PI076 agent-hook-registration-directive (AI addressee required)`), after `f9059dc` (`PI073`) |
| Build | `cargo build --release --locked`; `git status --porcelain -- src patterns Cargo.toml Cargo.lock` printed nothing; binary SHA-256 `c734037a003485f2...` |
| Pattern set | 77 patterns (baseline: 71): `PI070`/`PI071` from plan 05-03, `PI072`/`PI074`/`PI075` from plan 05-04, plus `PI073` and `PI076` |
| Rows | 33, identical row list to the baseline (taken from its `manifest.tsv`, column 1) |
| Files scanned | 26,405 |
| Findings | 1,306 |
| Input paths | the three repo-local rows by the main checkout's literal absolute paths (`$REPO/.planning/local/sweep-inputs-ext-05/{cursor,vscode}`, `$REPO/.planning/local/sweep-inputs-hooks-05`); the other thirty by their `$HOME` paths. No input set was copied into the executor's worktree and the baseline was not re-captured. |

`$HOME` and `$REPO` stand for the developer's home directory and the main checkout root. Counts are untouched.

## `--compare`, both directions, quoted verbatim

Both arguments are `.planning/local/` directories in the main checkout (33 raw reports each, counted before
the run).

```
$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-baseline-05-01-2026-10-08 $REPO/.planning/local/sweep-after-05-05-2026-10-08
<no output>
rc=0

$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-after-05-05-2026-10-08 $REPO/.planning/local/sweep-baseline-05-01-2026-10-08
<no output>
rc=0
```

Additions: 0. Removals: 0. Nothing to adjudicate.

Supporting checks, run on the same two directories:

- **One input row changed on disk, and it is not a scanner effect.** `manifest.tsv` columns 1-3 are identical to the
  baseline's on 32 of 33 rows. The 33rd, `$HOME/.claude/plugins/cache`, reads **950** files against the baseline's
  **962**, with the same single finding. The 12 files that left are exactly two orphaned `frontend-design` plugin cache
  versions (`.orphaned_at`, `LICENSE`, `README.md`, `plugin.json`, two `SKILL.md`-tree files each); none of the 12
  exists on disk any more, and no file appears in the candidate that was not in the baseline. The plugin host garbage-collects
  orphaned cache entries, which is the explanation. Row keys (column 1) are identical on all 33 rows.
- `summary.tsv` is `diff`-identical to the baseline's and contains no `PI07x` row: none of the seven CAT-03 patterns
  fires on any of the 26,405 files at the reported tier.
- Counting at every confidence (reported and low-confidence), `PI073` and `PI076` produce **0 and 0** findings across all 33
  reports.
- `checksums.sha256` here equals plan 05-04's on 32 of 33 rows; the one that differs is the `plugins/cache` report, for the
  reason above. The three repo-local rows' checksums are unchanged.
- **The comparison is not vacuous.** A scratch pair built from the largest real report, with a single finding deleted from
  one copy, made `--compare` print that finding and exit `1`
  (`$HOME/.codex/.tmp/plugins/plugins/catalyst-by-zoho/skills/catalyst-by-zoho/references/cli-reference.md:131	PI018`).

## Adjudication

Empty: there are no additions to adjudicate. This is the plan's named risk, so it is worth being plain about what the
result does and does not show. `PI073`'s target alternation names files that agent-tooling repositories discuss
constantly, and the sweep list includes the agent-configuration inputs plan 05-01 added (the Cursor and VS Code
extension trees and the hooks-configuration set) as well as the two largest plugin and tool trees
(`~/.claude/plugins/marketplaces`, 5,806 files, and `~/.codex/.tmp`, 10,741 files). `PI073` produces no finding,
reported or low-confidence, on any of the 26,405 files. `PI076` produces none either.

It is also uninformative about recall in the wild: zero findings means zero false positives on these inputs and also
zero true positives. The vendor-README sentence that `PI073` cannot separate from the attack by any regex (its named
blind spot) was not located in the sweep, which is consistent with the research's earlier measurement of 0 hits in
about 64,000 file-scans, and is not evidence that it does not exist.
