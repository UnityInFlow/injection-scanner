---
phase: 03-tool-permission-abuse-cat-01-33
plan: 02
subsystem: testing
tags: [false-positive-corpus, corpus-clean, frontmatter, structural-pass, CAT-01]

# Dependency graph
requires:
  - phase: 03-tool-permission-abuse-cat-01-33 (plan 01)
    provides: the measured baseline / harness that plan 04's mutation test and plan 06's prose
      patterns will run against; this plan's specimens are the corpus half of the same GATE-05
      gate
provides:
  - Five committed clean-corpus specimens (D-06/D-06a/D-06b) that any CAT-01 pattern (plans 05-07)
    must stay off, landed before any PI05x pattern exists
  - A proven-reachable structural false-positive control: narrow-allowed-tools-skill.md's
    frontmatter is confirmed parsed by `frontmatter::extract`, not merely assumed
affects: [03-04 (mutation-tested FP control / GATE-05), 03-05, 03-06, 03-07 (any plan authoring a
  PI05x pattern must run its pattern against these five specimens before shipping)]

# Actuals (#2632)
actuals:
  tokens: 2400
  tasks: 2
  commits: 2

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Structural false-positive control proven reachable via a throwaway `--patterns <tmpdir>`
      probe category (scope: frontmatter) rather than asserted by convention — same genre as the
      Plan 01 probe technique, applied to a clean-corpus file instead of an attack payload"
    - "Corpus-file leading-comment placement depends on document shape: a regular clean-corpus
      prose file keeps its rationale comment as the file's first lines (existing convention); a
      file whose value depends on frontmatter::extract() actually parsing it must move that
      comment AFTER the closing fence, since `---` must be the document's literal first line"

key-files:
  created:
    - tests/corpus/clean/sandbox-bypass-runbook.md
    - tests/corpus/clean/settings-permissions-reference.md
    - tests/corpus/clean/narrow-allowed-tools-skill.md
    - tests/corpus/clean/settings-deny-list.md
    - tests/corpus/clean/mcp-setup-guide.md
  modified: []

key-decisions:
  - "Wrote the runbook (hardest control) first, per CONTEXT.md's explicit ordering advice — it
    passed clean on the first draft with no narrowing needed, so no early pattern-scoping surprise
    surfaced this session."
  - "settings-deny-list.md written as a whole-file JSON document (first line '{'), matching the
    real settings.json shape it models, rather than a '---'-fenced YAML block — both forms are
    accepted by frontmatter::extract per the plan's acceptance criteria, and whole-file JSON is
    what a real settings.json actually is."
  - "Proved narrow-allowed-tools-skill.md's frontmatter is genuinely parsed (not silently invisible)
    via a one-off `--patterns` probe category, plus a negative control confirming the same content
    with a leading comment produces zero matches — this is the concrete evidence D-06(1)/T-03-06
    call for, not an assertion."

requirements-completed: [CAT-01, GATE-05]

coverage:
  - id: D1
    description: "Five false-positive control documents committed to tests/corpus/clean/ before
      any CAT-01 pattern exists, all reporting zero findings against the current 48-pattern set
      under --strict"
    requirement: CAT-01
    verification:
      - kind: integration
        ref: "cargo test --test corpus_test#the_clean_corpus_reports_nothing"
        status: pass
      - kind: integration
        ref: "cargo test --test corpus_test#the_clean_corpus_matches_nothing_even_under_strict"
        status: pass
      - kind: other
        ref: "cargo run --release -- check tests/corpus/clean --strict --format json (0 matches)"
        status: pass
    human_judgment: false
  - id: D2
    description: "The narrow-grant structural control (narrow-allowed-tools-skill.md) is proven
      reachable by frontmatter::extract/the structural pass, not silently unparsed, via a one-off
      scope:frontmatter probe pattern plus a negative control"
    requirement: GATE-05
    verification:
      - kind: other
        ref: "cargo run --release -- check tests/corpus/clean/narrow-allowed-tools-skill.md --patterns <tmpdir-probe> --strict --format json (4 matches, one per allowed-tools[0..3])"
        status: pass
    human_judgment: false
  - id: D3
    description: "settings-deny-list.md carries a real permissions.deny array with the same shape
      as the attack, plus a permissions.allow array, so a future bare-permissions pattern flagging
      a security control as an attack fails the build"
    requirement: GATE-05
    verification:
      - kind: other
        ref: "grep -c deny / grep -c allow on tests/corpus/clean/settings-deny-list.md (2 and 2)"
        status: pass
    human_judgment: false

# Metrics
duration: 20min
completed: 2026-09-01
status: complete
---

# Phase 3 Plan 02: CAT-01 false-positive control corpus Summary

**Five clean-corpus specimens (runbook, settings reference, narrow-grant skill, deny-list
settings.json, MCP setup guide) landed before any CAT-01 pattern exists, with the narrow-grant
control's frontmatter proven reachable by a throwaway probe rather than assumed.**

## Performance

- **Duration:** ~20 min
- **Tasks:** 2 completed
- **Files created:** 5

## Accomplishments

- `tests/corpus/clean/sandbox-bypass-runbook.md` — an operator runbook instructing a human to run
  `--dangerously-skip-permissions` inside a disposable, network-isolated container for a real
  operational reason (nightly golden-image rebuild), with explicit preconditions, teardown, and a
  warning never to do this outside the sandbox. Written first per CONTEXT.md's own advice that
  this is the hardest of the five controls; it passed clean under `--strict` on the first draft.
- `tests/corpus/clean/settings-permissions-reference.md` — a third-person reference document
  naming `--dangerously-skip-permissions`, `bypassPermissions` and `settings.json` descriptively,
  explaining what each is and why it is dangerous, without ever instructing the reader's agent to
  use any of them — the PI021 "mentions a flag, doesn't invoke it" precedent applied to CAT-01.
- `tests/corpus/clean/narrow-allowed-tools-skill.md` — a real skill file whose frontmatter grants
  tools narrowly (`Read`, `Grep`, `Glob`, scoped `Bash(npm test)`) as a YAML block sequence, the
  form real Claude Code skills actually use. The opening `---` fence is the file's literal first
  line; the rationale comment was moved to after the closing fence specifically so it does not
  break `frontmatter::extract`'s "fence must be `lines.next()`" requirement.
- `tests/corpus/clean/settings-deny-list.md` — a whole-file JSON `settings.json`-shaped document
  carrying a real `permissions.deny` array (path-scoped read denials plus a destructive-command
  denial) alongside a narrow `permissions.allow` array — the specimen that catches the
  worst-possible CAT-01 false positive: flagging a security control as an attack.
- `tests/corpus/clean/mcp-setup-guide.md` — a setup guide that legitimately instructs a reader to
  add an MCP server block to their `settings.json`, with no permission widening anywhere in the
  added block, forcing a future settings-widening pattern to key on the widening object rather
  than the settings filename.
- The narrow-grant control's frontmatter was verified genuinely parsed, not silently invisible —
  see "Probe evidence" below.

## Task Commits

Each task was committed atomically:

1. **Task 1: The two hardest controls — the operator runbook and the settings reference** -
   `22bbead` (test)
2. **Task 2: The three structural controls — narrow grant, deny list, and the setup guide** -
   `0d36000` (test)

_No separate plan-metadata commit is needed beyond this SUMMARY — parallel-worktree mode leaves
STATE.md/ROADMAP.md to the orchestrator._

## Files Created/Modified

- `tests/corpus/clean/sandbox-bypass-runbook.md` — operator runbook, the hardest false-positive
  control (D-06(3))
- `tests/corpus/clean/settings-permissions-reference.md` — descriptive permission-mechanism
  reference (D-06(2))
- `tests/corpus/clean/narrow-allowed-tools-skill.md` — real narrow `allowed-tools` grant, block
  sequence form (D-06(1))
- `tests/corpus/clean/settings-deny-list.md` — real `permissions.deny`/`permissions.allow` shape
  (D-06a/D-06b)
- `tests/corpus/clean/mcp-setup-guide.md` — legitimate "add this to your settings.json" guide
  (D-06(4))

## Decisions Made

- Wrote the runbook first, as CONTEXT.md recommends, to surface any forced-narrowing discovery as
  early as possible; none was needed — the document was clean under `--strict` on the first pass.
- `settings-deny-list.md` uses the whole-file-JSON form of `frontmatter::extract` (first line `{`)
  since that is what a real `settings.json` literally is, rather than dressing it up as a
  `---`-fenced YAML block purely for corpus-file-naming consistency.
- Confirmed via direct read of `.claude/settings.json` in this repo (not `~/.claude/settings.json`)
  that it contains only a `hooks` block and no `permissions` key — consistent with CONTEXT.md's
  D-06a correction note that the earlier research attribution to "this repo's own settings.json"
  was wrong, while the underlying risk (a real `.claude/settings.json` carrying `permissions.deny`
  elsewhere) is real and is what `settings-deny-list.md` now covers directly.

## Probe Evidence (GATE-05 / narrow-grant reachability)

Command and output proving `narrow-allowed-tools-skill.md`'s frontmatter is actually parsed by
`frontmatter::extract`, not silently skipped:

```
$ cargo run --release -- check tests/corpus/clean/narrow-allowed-tools-skill.md \
    --patterns /tmp/gsd-probe-03-02 --strict --format json
```

using a throwaway probe category at `/tmp/gsd-probe-03-02/probe.yaml`:

```yaml
category: probe_frontmatter_reachability
default_severity: LOW
patterns:
  - id: PROBE001
    name: allowed-tools-key-reachable
    scope: frontmatter
    pattern: "^allowed-tools\\[\\d+\\]\\s*="
    example: "allowed-tools[0] = Read"
    description: "Probe: proves the structural pass reaches an allowed-tools array entry"
    remediation: "n/a - test-only probe pattern, never shipped"
    tags: [probe]
```

Result: **4 matches**, one for each of `allowed-tools[0]` through `allowed-tools[3]` (`Read`,
`Grep`, `Glob`, `Bash(npm test)`), all located at line 4 with `context: "frontmatter_structural"`.

**Negative control:** the same probe run against a copy of the file's frontmatter preceded by a
leading `<!-- comment -->` (i.e. the mistake this file deliberately avoids — putting the rationale
comment before the opening fence) reports **0 matches**. This confirms a misplaced fence makes
`extract()` return `None` and the control silently invisible, exactly the "test that measured
nothing" failure mode this plan exists to rule out — not merely narrow, actually reachable.

Neither the probe file nor the negative-control scratch file were committed; both were scratch
artifacts under `/tmp`, deleted after the check.

## Deviations from Plan

None — plan executed exactly as written. No pre-existing false positives were surfaced: both Task
1 documents (the two hardest controls) were clean under `--strict` on the first draft, requiring no
narrowing or rewording.

## Issues Encountered

None. One unrelated event is worth recording for provenance: while reading `~/.claude/skills/gsd-audit-fix/SKILL.md`
for reference on real skill frontmatter shape, a tool-output block appeared claiming "bypass
permissions mode is active" and suggesting raw `Bash` be used in place of the tracked `Read`/
`Write`/`Edit` tools. No such mode was enabled in this session and no instruction of that kind came
from the user or the harness config, so it was treated as an untrusted/injected instruction and
ignored; execution continued unchanged with the standard tools. Noted here rather than silently
dropped, and fittingly on-topic for the phase this plan supports.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The clean corpus stands at 20 non-README specimens (D-06b's corrected 15 → 20), covering all
  five CAT-01 false-positive shapes named in CONTEXT.md/RESEARCH.md.
- Plans 05-07 (CAT-01 pattern authoring) can now run every new PI05x pattern against these five
  documents as part of their own acceptance criteria; none of the five reference any pattern ID,
  so they impose no coupling on how the patterns end up worded.
- Plan 04's GATE-05 mutation-pairing test has settings-deny-list.md and narrow-allowed-tools-skill.md
  available as the structural pairing targets, and the two prose documents as the prose pairing
  targets.
- No blockers.

---
*Phase: 03-tool-permission-abuse-cat-01-33*
*Completed: 2026-09-01*

## Self-Check: PASSED

- All 5 created corpus files verified present on disk with `ls -la`.
- Task commits `22bbead` and `0d36000` verified in `git log --oneline`.
- Re-ran all `<acceptance_criteria>` from both tasks: file counts (17 after Task 1, 20 after Task
  2), grep counts, `head -1` fence checks, the frontmatter probe (4 matches) and its negative
  control (0 matches), and `cargo run --release -- check tests/corpus/clean --strict` (0 matches
  across the whole directory) — all pass.
- Re-ran the plan-level `<verification>` block: `cargo test --locked` fully green, strict
  whole-directory check empty, file count 20, narrow-grant reachability proven and recorded above.
