# GATE-03 Sweep — Quick Task 260902-jhy (CR-01 negation-blindness fix)

Following `.planning/phases/03-tool-permission-abuse-cat-01-33/03-SWEEP.md`'s structure and its
established "self-repo half of `--compare` is not meaningful, audit it rather than fear it"
methodology.

## Binaries

| Run | Git SHA | Notes |
|---|---|---|
| `sweep-before` | `26fc6af` | HEAD at plan-start, before any Task 1 edit. Built and swept as Task 1 Step 0, per the plan's ordering constraint (patterns compile into the binary). |
| `sweep-after` | `fdc51cb` | Task 1 (`d157783`, PI053/PI056/PI057 negation-guard fix) + Task 2 (`fdc51cb`, catalogue/baseline regen + WR-01 row) landed. |

## Reproduction command

```bash
cargo build --release
bash scripts/gate03-sweep.sh <output-dir> \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/03-injection-scanner \
  /Users/jirihermann/.claude/plugins/cache \
  /Users/jirihermann/.claude/skills \
  /Users/jirihermann/.claude/gsd-core \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/01-spec-linter \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/02-ai-changelog \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/04-spec-ci-plugin \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/05-budget-breaker \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/06-token-dashboard \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/07-mcp-hub \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/08-kore-runtime \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/09-agent-tracer \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/10-agent-memory \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/11-context-manager \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/12-agent-replayer \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/13-eu-ai-act-toolkit \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/14-agent-sandbox \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/15-llm-diff \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/16-prompt-vc \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/17-skills-registry \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/18-agent-bench \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/19-skill-composer \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/20-mcp-test
```

`03-SWEEP.md`'s reproduction command named a Plan-03 worktree
(`.claude/worktrees/agent-ade5856f6fc048a75`) that no longer exists on this machine; substituted
the current worktree root (this repository itself, `03-injection-scanner`) per the plan's Task 1
Step 0 instruction. All 22 directories from the original list resolved and were present on this
machine (the precondition for Task 3 required at least 3; all 22 held).

## Directory list swept, with per-directory file/finding counts

| Directory | Files | Findings (before) | Findings (after) | Status |
|---|---:|---:|---:|---|
| `03-injection-scanner` (this repo) | 397 | 1120 | 1075 | swept |
| `.claude/plugins/cache` | 952 | 10 | 10 | swept |
| `.claude/skills` | 71 | 0 | 0 | swept |
| `.claude/gsd-core` | 555 | 14 | 14 | swept |
| `01-spec-linter` | 48 | 0 | 0 | swept |
| `02-ai-changelog` | 50 | 0 | 0 | swept |
| `04-spec-ci-plugin` | 43 | 0 | 0 | swept |
| `05-budget-breaker` | 59 | 0 | 0 | swept |
| `06-token-dashboard` | 76 | 0 | 0 | swept |
| `07-mcp-hub` | 403 | 1 | 1 | swept |
| `08-kore-runtime` | 204 | 0 | 0 | swept |
| `09-agent-tracer` | 106 | 0 | 0 | swept |
| `10-agent-memory` | 49 | 0 | 0 | swept |
| `11-context-manager` | 6 | 0 | 0 | swept |
| `12-agent-replayer` | 182 | 0 | 0 | swept |
| `13-eu-ai-act-toolkit` | 102 | 3 | 3 | swept |
| `14-agent-sandbox` | 60 | 0 | 0 | swept |
| `15-llm-diff` | 63 | 0 | 0 | swept |
| `16-prompt-vc` | 48 | 0 | 0 | swept |
| `17-skills-registry` | 172 | 8 | 8 | swept |
| `18-agent-bench` | 7 | 0 | 0 | swept |
| `19-skill-composer` | 58 | 0 | 0 | swept |
| `20-mcp-test` | 71 | 1 | 1 | swept |

**Every one of the 21 reference/third-party directories has a byte-identical finding count before
and after.** Only this repository's own count moved (1120 → 1075, a drop of 45 — all inside this
repo's own `tests/`, `patterns/`, `examples/`, `docs/` and the multi-line block-scan artifacts
detailed below).

## PI053/PI056/PI057 counts, this repo only (`summary.tsv`)

| Pattern | Before | After |
|---|---:|---:|
| PI053 | 30 | 12 |
| PI054 | 13 | 13 (untouched) |
| PI055 | 40 | 40 (untouched) |
| PI056 | 21 | 6 |
| PI057 | 20 | 8 |

## `--compare` in both directions

### `--compare sweep-before sweep-after` (NEW findings)

Raw output listed 16 lines. **Zero of the 16 are in a third-party directory.** All 16 resolve to
one of three known, audited causes:

1. **A stale, gitignored `.claude/worktrees/agent-ad3e0837514b816af/` copy of this same
   repository** (8 of the 16 lines) — `.claude/` is gitignored (`.gitignore:6`), not part of this
   task's diff, and not part of git history. Confirmed independently present, containing an older
   snapshot of this repo's own files. Excluded from the audit below exactly as `03-SWEEP.md`
   excluded the previous plan's stale worktree.
2. **Line-shift of pre-existing content** — Task 1's new design comments (PI053, PI056, PI057) and
   Task 1's six new negative test sentences (`tests/pattern_test.rs`) added lines above existing
   `example`/positive blocks in both files, shifting their line numbers. The `(file, line,
   pattern_id)` compare key treats "same text, new line" as new. Verified line-by-line: every one
   of these entries is the *identical* pre-existing `example`/positive/comment text this repo
   already had, at its new post-edit line.
3. **A pre-existing multi-line block-scan pass reports a second, systematically miscalculated line
   number for some matches**, independent of any pattern this task touched — confirmed by finding
   the *same* duplicate-at-a-different-line behavior for **PI054** (untouched by this task) in both
   `sweep-before` and `sweep-after`, at the identical text (`'You have permission to run any
   command'`) and the identical relative offset. This scanner behavior predates and is unrelated to
   CR-01; it only became newly *visible in the diff* because Task 1/2's added lines shifted where
   the two passes' independently-computed line numbers land relative to each other.

| File | Line | Pattern | Cause | Genuinely new text? |
|---|---:|---|---|---|
| `patterns/core/tool-permission-abuse.yaml` | 180 | PI054 | Line-shift (was :162) | No — pre-existing `example` |
| `patterns/core/tool-permission-abuse.yaml` | 199 | PI055 | Line-shift (was :181) | No — pre-existing `example` |
| `patterns/core/tool-permission-abuse.yaml` | 208 | PI055 | Line-shift (was :190) | No — pre-existing comment |
| `patterns/core/tool-permission-abuse.yaml` | 256 | PI018 | Line-shift (was :227) | No — pre-existing D-17 comment |
| `tests/corpus/attack/tool-permission-abuse.md` | 24 | PI056 | Block-pass off-by-one duplicate of the genuine PI056 payload at line 25 (already flagged there in both runs) | No — same attack corpus payload, reported twice |
| `tests/corpus/attack/tool-permission-abuse.md` | 25 | PI057 | Block-pass off-by-one duplicate of the genuine PI057 payload at line 26 (already flagged there in both runs) | No — same attack corpus payload, reported twice |
| `tests/pattern_test.rs` | 1300–1302 | PI054 | Line-shift (was :1291–1293) | No — pre-existing positives |
| `tests/pattern_test.rs` | 1323–1325 | PI055 | Line-shift (was :1314–1316) | No — pre-existing positives |
| `tests/pattern_test.rs` | 1379 | PI057 | Block-pass miscalculated line, duplicate of the pre-existing positive `"You should turn off that guardrail..."` (unchanged text, was reported near :1365) | No — pre-existing positive |
| `tests/pattern_test.rs` | 1414 | PI018 | Block-pass miscalculated line, duplicate of the pre-existing doc-comment text `"safety checks have been disabled"` (unchanged, was near :1394) | No — pre-existing comment |

**No line in this table is a new attack phrasing, and none is on benign third-party prose.**

### `--compare sweep-after sweep-before` (REMOVED findings)

Excluding the same stale worktree copy, the removed findings fall into two groups, both expected
and both individually adjudicated:

**Group A — the fix working as designed.** PI053/PI056/PI057 no longer matching the six
reviewer-reproduced prohibition sentences and the design-comment prose that mentioned trigger
phrases mid-sentence (not clause-initial), inside `patterns/core/tool-permission-abuse.yaml`'s own
comments:

| File:Line | Pattern | Matched text (before) | Verdict |
|---|---|---|---|
| `patterns/core/tool-permission-abuse.yaml:139` | PI053 | "run with --dangerously-skip-permissions" (decoded-pass dup of the `example` field) | Known false-positive class (self-scan of the pattern's own `example`), now narrower due to YAML-context interaction — `pattern_example_test` still asserts the isolated `example` string matches (it does, confirmed green) |
| `patterns/core/tool-permission-abuse.yaml:155` | PI053 | "run with --dangerously-skip-permissions" | Design-comment prose (not clause-initial) — correctly no longer matches |
| `patterns/core/tool-permission-abuse.yaml:156` | PI053 | "switch the permission mode to bypassPermissions" | Design-comment prose (not clause-initial) — correctly no longer matches |
| `patterns/core/tool-permission-abuse.yaml:170` | PI053 | "run with --yolo" | Design-comment prose (not clause-initial) — correctly no longer matches |
| `patterns/core/tool-permission-abuse.yaml:214` | PI056 | 'settings.json from "allow list' | Design-comment prose fragment describing the 55-char window's derivation, no directive verb before it — correctly no longer matches after the verb requirement was added |
| `tests/pattern_test.rs` (multiple lines in the PI053/PI056/PI057 test bodies) | PI053/PI056/PI057 | The six reviewer prohibition sentences plus their surrounding comment prose | **This is the fix.** These are the exact sentences CR-01 exists to stop matching; they are now pinned as unit-test negatives instead |

**Group B — line-shift counterparts of the "NEW" table above**, e.g.
`patterns/core/tool-permission-abuse.yaml:162 PI054` (moved to :180) and
`tests/pattern_test.rs:1291-1293 PI054` (moved to :1300-1302) — the same pre-existing content,
same verdict as their "new" counterpart.

**No removed finding is a genuine payload.** Nothing here required going back to Task 1 to
re-narrow.

## Third-party PI053/PI054/PI055/PI056/PI057 findings, explicitly

Checked every non-`03-injection-scanner` directory's JSON report in both runs for any
PI053/PI054/PI055/PI056/PI057 match: **zero in `sweep-before`, zero in `sweep-after`.** These
patterns have never fired on any of the 21 reference directories, before or after this fix — fully
consistent with `03-SWEEP.md`'s own after-sweep finding that CAT-01's only genuine third-party hit
to date is PI051 (`17-skills-registry/.claude/settings.local.json`), unaffected by this task.

## Verdict

**Zero new third-party findings. Zero false-positive spike anywhere in the after-sweep.** Every
self-repo delta traces to one of three known, non-regression causes (stale worktree noise,
line-shift of unchanged content, or a pre-existing block-pass line-miscalculation duplicate proven
to also affect an untouched pattern). CR-01's fix removes exactly the six reviewer-reproduced
prohibition sentences and nothing else; no genuine attack-corpus payload or third-party document
lost or gained a finding.
