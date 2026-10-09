# Phase 5 Plan 07 — the whole-category GATE-03 delta (2026-10-09)

This is the one measurement that compares **every pattern commit of Phase 5** against plan 05-01's pre-edit
baseline, in a single `--compare`. Each pattern plan (05-03 through 05-06) measured its own delta; a category is
reviewed as one unit (GATE-04), and its false-positive blast radius is a property of the whole range. The baseline
cannot be recaptured after a pattern edit, so it is not: it is the directory plan 05-01 wrote before any CAT-03
pattern existed, and `--compare` is one-directional, so both directions are run.

## Provenance of the two sides

| | Baseline | Candidate |
|---|---|---|
| Directory | `.planning/local/sweep-baseline-05-01-2026-10-08/` | `.planning/local/sweep-final-05-2026-10-08/` |
| Binary built from | `1d734937410c8025c9d484834a505d2154511199` | `d46df12d4164c52d961a5c9c9e9284e0a767070e` plus uncommitted corpus and test files (committed afterwards as `c77a490`) |
| Pattern/engine state | the 71-pattern set; `PI070` is the only `PI07x` pattern | the 79-pattern set; `PI070` widened, `PI071`-`PI077` and `PI079` added, `PI078` unallocated |
| Precondition | `src`, `patterns`, `Cargo.toml`, `Cargo.lock` clean | `git diff d46df12 HEAD -- patterns src Cargo.toml Cargo.lock` is empty, so the binary carries exactly the committed pattern set. Binary SHA-256 `29455af24708c94d...` |
| Build | `cargo build --release --locked` | `cargo build --release --locked`, `injection-scanner 0.1.0` |
| Rows | 33 | 33 |
| Files scanned | 26,417 | 26,407 |
| Findings (manifest column 3, reported only) | 1,306 | 1,306 |
| Findings including low-confidence | 1,571 | 1,571 |

The eleven pattern commits between the two binaries (`git log 1d73493..HEAD -- patterns/`): `be30f62`, `b0e70d7`,
`5233a97`, `4aa7d78`, `f87bf6a`, `f9059dc`, `c9bfdcb`, `6d0f6a5`, `6f6f34a`, `dd4d726` (and the comment-only `d82a13f`).
`git diff --stat 1d73493 HEAD -- patterns/` lists exactly one file, `patterns/core/persistence-lifecycle-hijack.yaml`.

## Directory list

The same 33 rows as the baseline, taken from its `manifest.tsv` column 1. The thirty `$HOME` rows are swept by their
`$HOME` paths; the three repo-local rows by the **main checkout's literal absolute paths**
(`$REPO/.planning/local/sweep-inputs-ext-05/{cursor,vscode}` and `$REPO/.planning/local/sweep-inputs-hooks-05`),
because `--compare` keys on the absolute path in each report and this plan ran in a disposable worktree. No input set
was copied and the baseline was not re-captured. A row-key comparison of the two committed manifests
(`diff <(cut -f1 baseline/manifest.tsv) <(cut -f1 final/manifest.tsv)`) is empty, exit 0.

`manifest.tsv` differs from the baseline's on exactly one line: `$HOME/.claude/plugins/cache`, **952** files against the
baseline's 962 and plan 05-06's 950, with the same single finding. It is the plugin host garbage-collecting and
refreshing orphaned cache entries between runs (plans 05-05 and 05-06 recorded the same row moving), not a scanner
effect. `summary.tsv` is `diff`-identical to the baseline's. `checksums.sha256` differs from plan 05-06's on that one
row only (32 of 33 raw reports are byte-identical).

## `--compare`, both directions, quoted verbatim

Both arguments are `.planning/local/` directories in the main checkout (33 raw reports each, counted before the run).

```
json reports: baseline=33 candidate=33

$ bash scripts/gate03-sweep.sh --compare <main>/.planning/local/sweep-baseline-05-01-2026-10-08 <main>/.planning/local/sweep-final-05-2026-10-08
<no output>
rc=0

$ bash scripts/gate03-sweep.sh --compare <main>/.planning/local/sweep-final-05-2026-10-08 <main>/.planning/local/sweep-baseline-05-01-2026-10-08
<no output>
rc=0
```

## Non-vacuity: a planted deletion is reported

A copy of the candidate's reports with one finding removed from its largest report, compared against the intact
candidate, made `--compare` print that finding and exit `1`:

```
planted deletion: <largest report> PI026 line 95
$ bash scripts/gate03-sweep.sh --compare <copy-with-one-finding-removed> <full-candidate>
$REPO/.planning/local/sweep-inputs-ext-05/vscode/berublan.vscode-log-viewer-0.14.1/README.md:95	PI026
rc=1
```

## Adjudication

**Additions: 0. Removals: 0. Nothing to adjudicate**, in either direction, over the whole range. Per the plan's three
possible outcomes: an empty delta in both directions closes the gate; there is no addition already adjudicated in a
plan SUMMARY to carry forward; and there is no addition that appears only now, so no pattern was narrowed and no clean
specimen added by this plan.

## What this sweep does and does not establish

- **Zero findings from any `PI070`-`PI079` pattern, at any confidence, on any of the 26,407 files**, in the baseline and
  in the candidate: a census of `matches` plus `low_confidence` over the 33 reports returns `{}` for both. The highest
  pattern id that fires anywhere is `PI061`. That is the false-positive result: the nine patterns add no finding to a
  corpus of real agent skills, plugins, settings and hooks.
- It is also **uninformative about recall in the wild**. Zero findings means zero false positives and zero true positives
  on these inputs. The frozen 263-file hooks input (`$REPO/.planning/local/sweep-inputs-hooks-05`, 59 findings in both
  runs, none from `PI077`) is the population a binding-only rule was measured hitting 254 of 328 files on; zero there is
  the discriminator holding, not evidence that it detects anything in the wild.
- The result agrees with plan 05-06's own delta (also empty, 26,405 files) and extends it across all nine patterns and
  the `PI070` widening in one comparison, which is the interaction a per-plan sweep cannot see.

## Standing note

`--compare` keys on the absolute path in each report, and loads findings by globbing `*.json` in each argument. This
repository commits no raw reports by design, so **always point `--compare` at `.planning/local/` directories**. Against a
committed sweep directory it loads an empty baseline and reports every real finding as an addition, which already cost
Phase 4 plan 04-07 a spurious 500-line diff.
