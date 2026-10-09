# Raw JSON sweep reports are not in the repository

Same rule as `sweep-baseline-05-01-2026-10-08/` and the later `sweep-after-05-0N-*` directories: the 33
per-directory JSON reports this run wrote inventory one developer machine and carry local paths, so they are
kept out of the public history.

- Committed here: `manifest.tsv`, `summary.tsv`, `checksums.sha256` (SHA-256 of every raw report, filenames
  redacted), and this file. Zero `*.json` (checked: `find` over the phase directory prints `0`).
- Kept locally, gitignored: `.planning/local/sweep-after-05-06-2026-10-08/*.json` (33 files) in the main
  checkout.
- **Never point `--compare` at this directory.** It has no `*.json`, so it loads a silently empty set and
  reports every real finding as an addition. Both `--compare` runs below used the `.planning/local/`
  directories.

## What this capture is

Phase 5 plan 05-06, GATE-03 candidate sweep for the two CAT-03 pattern commits of this plan: `PI077`
(`6f6f34a`) and `PI079` (`dd4d726`, which also records `PI078` as dropped). Compared against plan 05-01's pre-edit
baseline, which remains the comparison target for every pattern plan in this phase.

| Field | Value |
|---|---|
| Binary built from | HEAD `dd4d72636b362d607127ce3fdb6c63679e281aa0` (`feat(05-06): add PI079, drop PI078 on evidence (D-07)`) |
| Build | `cargo build --release --locked`; `git status --porcelain -- src patterns Cargo.toml Cargo.lock` printed nothing; binary SHA-256 `5521331a35e5e9be...` |
| Pattern set | 79 patterns (baseline: 71): plans 05-03 to 05-05's seven, plus `PI077` and `PI079`. `PI078` is unallocated |
| Rows | 33, identical row list to the baseline (taken from its `manifest.tsv`, column 1) |
| Files scanned | 26,405 |
| Findings | 1,306 |
| Input paths | the three repo-local rows by the main checkout's literal absolute paths (`$REPO/.planning/local/sweep-inputs-ext-05/{cursor,vscode}`, `$REPO/.planning/local/sweep-inputs-hooks-05`); the other thirty by their `$HOME` paths. No input set was copied into the worktree and the baseline was not re-captured |

`$HOME` and `$REPO` stand for the developer's home directory and the main checkout root. Counts are untouched.
The release binary was built from the committed tree above; after the capture, the `PI077` header comment in the
YAML was reworded (comment only, no regex or field touched), which does not change what the binary detects.

## `--compare`, both directions, quoted verbatim

Both arguments are `.planning/local/` directories in the main checkout (33 raw reports each, counted before
the run).

```
json reports: baseline=33 candidate=33
$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-baseline-05-01-2026-10-08 $REPO/.planning/local/sweep-after-05-06-2026-10-08
<no output>
rc=0

$ bash scripts/gate03-sweep.sh --compare $REPO/.planning/local/sweep-after-05-06-2026-10-08 $REPO/.planning/local/sweep-baseline-05-01-2026-10-08
<no output>
rc=0
```

Additions: 0. Removals: 0. Nothing to adjudicate.

Supporting checks, run on the same two directories:

- **`PI077` and `PI079` produce no finding on any of the 26,405 files, at any confidence** (reported and
  low-confidence, counted from the 33 reports): `{'PI077': 0, 'PI079': 0}`.
- **The frozen hooks-configuration row is the one this plan was waiting for.** `$REPO/.planning/local/sweep-inputs-hooks-05`
  reads 263 files and 59 findings in both the baseline and the candidate, none from `PI077`. That input is
  exactly the population a binding-only rule was measured hitting on (254 of 328 files in research), so zero here
  is the discriminator holding on real hook configurations, not an empty set. It is also uninformative about
  recall in the wild: zero findings means zero false positives on these inputs and zero true positives.
- **One input row changed on disk, and it is not a scanner effect.** `manifest.tsv` differs from the baseline's on
  exactly one line, `$HOME/.claude/plugins/cache`: **950** files against the baseline's **962**, with the same single
  finding. It is the same row, with the same counts, that plan 05-05 recorded: the plugin host garbage-collects
  orphaned cache entries. Row keys (column 1) are identical on all 33 rows. `summary.tsv` is `diff`-identical to the
  baseline's.
- `checksums.sha256` here is byte-identical to plan 05-05's on all 33 rows.
- **The comparison is not vacuous.** A scratch pair built from the largest real report, with a single finding
  deleted from one copy, made `--compare` print that finding and exit `1`:

```
planted deletion: ('$HOME/.codex/.tmp/plugins/plugins/catalyst-by-zoho/skills/catalyst-by-zoho/references/cli-reference.md', 131, 'PI018')
$ bash scripts/gate03-sweep.sh --compare <copy-with-one-finding-deleted> <full-copy>
$HOME/.codex/.tmp/plugins/plugins/catalyst-by-zoho/skills/catalyst-by-zoho/references/cli-reference.md:131	PI018
rc=1
```

## Adjudication

Empty: there are no additions to adjudicate.

## `PI078`'s criterion outcome

`PI078` was **dropped in the previous commit, not by this sweep**, and its id is unallocated. The plan wrote its
criterion in two halves: the loopback specimen silent under `--strict`, and no sweep addition on a legitimate
audit endpoint. D-07's amendment replaced that with a stricter named test: a discriminator that fires on
structural payload 04 and stays silent on `persistence-corporate-audit-endpoint.json`, proven both ways. That test
failed (the dotted-host rule fires on both documents, and swapping the event, the registrable domain and the URL
between them moved nothing), so the pattern never reached this sweep. For completeness, the dotted-host draft was
also run over the 263-file hooks input and produced 0 hits there, which means **the sweep half of the original
criterion alone would not have disqualified it**. The payload-04-versus-audit-specimen equivalence is what did,
and a clean sweep is no evidence against that.
