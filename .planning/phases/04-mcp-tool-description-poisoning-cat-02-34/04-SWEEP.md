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
