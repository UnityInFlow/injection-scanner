# Raw JSON sweep reports are not in the repository

Same rule as every Phase 4 sweep directory (`.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-*`):
the 33 per-directory JSON reports `scripts/gate03-sweep.sh` wrote for this run inventory one
developer machine and carry local paths, so they are kept out of the public history.

- Committed here: `manifest.tsv`, `summary.tsv`, `checksums.sha256` (SHA-256 of every raw
  report, filenames redacted), and this file.
- Kept locally, gitignored: `.planning/local/sweep-baseline-05-01-2026-10-08/*.json` (33 files).
- This directory is **zero `*.json` by construction**. Pointing `--compare` at it loads an
  *empty* baseline and reports every real finding in the candidate as a false "addition" — the
  failure that cost Phase 4 plan 04-07 a spurious 500-line diff. **Always point `--compare` at
  `.planning/local/sweep-baseline-05-01-2026-10-08/`, never at this directory.**

## What this capture is

Phase 5 plan 05-01 Task 1, commit A — the GATE-03 **pre-edit** baseline for CAT-03
(`PI070`–`PI079`, #35). Plans 05-03 through 05-07 each `--compare` against it, so every later
delta measures that plan's patterns and nothing else.

| Field | Value |
|---|---|
| Binary built from | `git rev-parse HEAD` = **`1d734937410c8025c9d484834a505d2154511199`** |
| Build | `cargo build --release --locked` (38.60s), `injection-scanner 0.1.0` |
| Precondition | `git status --porcelain -- src patterns tests examples Cargo.toml Cargo.lock` printed nothing before the build |
| Rows | 33 (Phase 4's 32 + the frozen hooks-config row), 0 dropped |
| Files scanned | 26,417 |
| Findings | 1,306 |
| Sweep wall time | 207s (baseline run), 221s (stability run) |
| Pattern set | the 71-pattern set at that sha; `PI070` is the only `PI07x` pattern |

`HEAD` was taken before any file in this plan was created. `git log -1 --format=%H 1d734937410c8025c9d484834a505d2154511199`
resolves.

## Paths in this directory are redacted

| Placeholder | Stands for |
|---|---|
| `$HOME` | the developer's home directory |
| `$REPO` | the repository root the three repo-local input sets were created under |

Counts are untouched.

## Row substitutions against Phase 4's 32-row list

Phase 4's final manifest (`sweep-final-2026-09-03/manifest.tsv`) is the base. Two rows changed
and one row was added.

### 1. `$SCRATCH/cursor-safe` and `$SCRATCH/vscode-safe` -> frozen doc-file copies (a fallback the plan allows, for a different reason than its trigger)

The plan said to substitute the real `$HOME/.cursor/extensions` and `$HOME/.vscode/extensions`
and to fall back to a narrowed copy only if the binary exited above 2. **That substitution was
attempted and abandoned.** No crash: the binary spent more than **9 minutes at 100% CPU on the
single `~/.cursor/extensions` row** (39,738 files, 1.4 GB) without finishing, so a two-run
stability probe over it was infeasible. I killed that run (nothing from it is in this baseline).

The cause is not a crash and not the char-boundary panic `d28dfd0` fixed. A probe over the 60
largest files under `~/.cursor/extensions` (`--all-files --no-ignore`, one file at a time, 25s
cap each) found **24 of 60 timed out**; 23 of the 24 timeouts were `.js` bundles and one was a `.js.map` (the smallest was a 4.0 MB
`server.js`), while
non-text blobs of 19-52 MB were rejected as binary in 0.1s. `--all-files` is what lets `.js`
bundles in at all. Logged as a deferral in `deferred-items.md` — it is not this plan's job to fix.

So both rows became **frozen, doc-type-only copies**: every `*.md`, `*.markdown`, `*.json`,
`*.jsonc`, `*.yaml`, `*.yml`, `*.toml`, `*.txt` file, at most 1,000,000 bytes, symlinks skipped,
copied preserving the relative path into `.planning/local/sweep-inputs-ext-05/{cursor,vscode}/`.

| Root | Doc files copied | Skipped (>1MB / link / error) | Files the scanner counted |
|---|---:|---:|---:|
| `~/.cursor/extensions` | 4,655 | 4 | 1,234 |
| `~/.vscode/extensions` | 5,970 | 8 | 1,593 |

The scanner counts fewer files than were copied because its walker declines some paths even
under `--all-files --no-ignore`; the counted figures are the ones in `manifest.tsv`.

### 2. New row: the frozen hooks-config input set (`$REPO/.planning/local/sweep-inputs-hooks-05`)

263 files, 59 findings (all `PI051` wildcard-permission-allow, which is what settings files
legitimately full of `Bash(*)` produce).

**The live agent-configuration roots were rejected as sweep inputs.** `$HOME/.claude`,
`$HOME/.codex`, `$HOME/.cursor` and `$HOME/.gemini` carry session transcripts, shell snapshots,
todo state and telemetry that are rewritten *while these plans execute*, and whose content quotes
the payloads and patterns this phase is building. A sweep over them observes itself: every later
`--compare` would report additions and removals caused by the executor's own transcripts. No
whole-root row exists: `awk -F'\t' '$1 ~ /^\$HOME\/\.(claude|codex|cursor|gemini)$/' manifest.tsv | wc -l`
prints `0`.

**Collection rule** (rebuild by this rule if the directory is lost):

- Match filenames `hooks.json`, `settings.json`, `settings.local.json`, `hooks.yaml`.
- Search roots: `~/.claude`, `~/.codex`, `~/.cursor`, `~/.gemini` (34 files) **plus**
  `~/.config` and `~/Documents/workspace-1-ideas` (229 files).
- Prune any directory named: `node_modules`, `target`, `projects`, `sessions`, `history`,
  `shell-snapshots`, `shell_snapshots`, `todos`, `telemetry`, `statsig`, `session-env`,
  `file-history`, `paste-cache`, `debug`, `backups`, `tmp`, `.tmp`, `thread-writer-locks`,
  `log`, `logs`, `statistics`, `stats`.
- For the two additional roots also prune `worktrees`, `.git`, `03-injection-scanner` (the repo
  under change), `local`, `build`, `dist`, `.gradle`.
- Copy each match preserving its path relative to `$HOME`.

**Deviation from the plan, recorded:** the plan asks for >=100 collected files from the four
named roots. That is not achievable on this machine — the four roots yield **34** files after the
exclusions. The researcher's own 328-file count (05-RESEARCH.md §Q2) came from the wider
`~/.config` + `~/Documents/workspace-1-ideas` search, so I applied the *same* collection rule to
those two roots to reach 263. The exclusion list and the frozen-copy property are unchanged.

A frozen snapshot is not a weaker input than the live roots: the gate's own header says the
meaningful signal is the delta between two runs over the **same** input with only the binary
changed.

## Two-run stability probe (run on the unmodified tree, before trusting the list)

The sweep was run twice back-to-back over the identical 33-row list into
`.planning/local/sweep-stab-a/` and `.planning/local/sweep-baseline-05-01-2026-10-08/`. The second
is kept as the baseline.

```
COMPARE sweep-stab-a -> sweep-baseline-05-01-2026-10-08 rc=0
<empty>
COMPARE sweep-baseline-05-01-2026-10-08 -> sweep-stab-a rc=0
<empty>
```

Both directions empty. `diff` of the two `manifest.tsv` and the two `summary.tsv`: identical.
No row was dropped for instability.

## Standing warnings from the run (not findings)

`620` files in the `Library/Application Support/Code/User` row were skipped by the structural
pass as "invalid JSON document" (VS Code `.log` files that start with `{`), `3` binaries in
`cannaryandbg` and `1` `.wasm` in `pattern-atlas` as "looks like binary content". Identical in
both runs.

## PATH-KEYED CAVEAT — read before running any later `--compare`

`--compare` keys findings on `(file, line, pattern id)` where `file` is the **absolute path
recorded in the JSON report**. The thirty rows under `$HOME/...` are stable across worktrees.
**The three rows under `$REPO/.planning/local/` are not**: the baseline was captured while the
repo root was a disposable agent worktree, so their `file` keys embed that worktree's absolute
path. A later plan running in a different checkout will see those three rows as wholesale
additions and removals.

Two ways to make the comparison honest:

1. Move `.planning/local/sweep-inputs-hooks-05/` and `.planning/local/sweep-inputs-ext-05/` into
   the checkout every later plan runs in, then **re-run the baseline from that checkout** (below)
   so the three rows' keys match; or
2. Run the later comparison with those three rows excluded from both sides and compare the
   other thirty.

### RESOLVED 2026-10-08 by the orchestrator — option 1 was taken

`$MAIN` below is the **main checkout** root (the working tree whose branch is not an
`agent-*` / `worktree-agent-*` namespace). It is the root every later plan compares from.

- The frozen input sets were copied out of the agent worktree into `$MAIN/.planning/local/`
  (`sweep-inputs-ext-05/{cursor,vscode}/`, `sweep-inputs-hooks-05/`) **before** that worktree
  was removed, together with the 33 raw reports.
- A release binary was built in `$MAIN`. Its `src/`, `patterns/`, `Cargo.toml` and `Cargo.lock`
  are unchanged from the recorded baseline sha `1d734937410c8025c9d484834a505d2154511199`
  (`git diff --stat 1d73493..HEAD -- src patterns Cargo.toml Cargo.lock` is empty), so it
  carries the same 71-pattern set and the re-capture is comparable.
- Only the three `$REPO` rows were re-run. The thirty `$HOME` rows were left exactly as
  captured, because their keys never depended on which checkout ran the sweep.
- Accepted by this file's own criterion: the three re-captured rows match the committed
  `manifest.tsv` **row for row** — `1234 / 271`, `1593 / 431`, `263 / 59`.
- `$MAIN/.planning/local/sweep-baseline-05-01-2026-10-08/` now holds 33 reports, none containing
  the retired worktree id, and its own `manifest.tsv` was corrected to the `$MAIN` paths.
  `--compare` of that directory against itself is empty, exit 0.

**BINDING RULE for plans 05-03 through 05-07.** Each executor runs in its own worktree, so a
`$REPO`-relative input path would re-introduce precisely this defect — silently, as a pile of
additions and removals that look like a regression. Sweep the input sets and the baseline by
their **`$MAIN`-absolute** paths, never by a path relative to your own checkout:

```
$MAIN/.planning/local/sweep-inputs-ext-05/cursor
$MAIN/.planning/local/sweep-inputs-ext-05/vscode
$MAIN/.planning/local/sweep-inputs-hooks-05

bash scripts/gate03-sweep.sh --compare \
  $MAIN/.planning/local/sweep-baseline-05-01-2026-10-08 \
  <your candidate output dir>
```

Do not copy the input sets into your own worktree, and do not re-capture the baseline. Both
re-create the path skew this section exists to close.

## RECOVERY — the raw JSON is not in git

If `.planning/local/sweep-baseline-05-01-2026-10-08/` is lost, rebuild it. Never use `git stash`.

```bash
# 1. A separate worktree at the recorded sha, so the phase's own work is untouched.
git worktree add --detach "$TMPDIR/is-baseline-05" 1d734937410c8025c9d484834a505d2154511199

# 2. Build the release binary from that sha.
(cd "$TMPDIR/is-baseline-05" && cargo build --release --locked)

# 3. Rebuild the frozen input sets by the collection rules above, into the checkout that
#    will run the later comparisons:  .planning/local/sweep-inputs-hooks-05/
#                                      .planning/local/sweep-inputs-ext-05/{cursor,vscode}/

# 4. Re-run the sweep over the row list in manifest.tsv (column 1; expand $HOME and $REPO),
#    from the checkout that owns the frozen inputs, into the SAME output path.
INJECTION_SCANNER_BIN="$TMPDIR/is-baseline-05/target/release/injection-scanner" \
  bash scripts/gate03-sweep.sh .planning/local/sweep-baseline-05-01-2026-10-08 \
  <the 33 directories from manifest.tsv>

# 5. Remove the helper worktree.
git worktree remove "$TMPDIR/is-baseline-05"
```

A rebuilt baseline is only valid if its `manifest.tsv` matches the committed one **row for row**
(same directory column, same file count, same finding count) and `summary.tsv` is identical. Do
not expect `checksums.sha256` to match for the three `$REPO` rows (their reports embed absolute
paths); the thirty `$HOME` rows' checksums must match unless the underlying third-party files
changed on disk.
