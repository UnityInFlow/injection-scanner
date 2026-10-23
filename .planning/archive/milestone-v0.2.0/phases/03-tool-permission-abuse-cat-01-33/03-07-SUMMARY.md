---
phase: 03-tool-permission-abuse-cat-01-33
plan: 07
status: complete
completed: 2026-09-02
requirements: [CAT-01, GATE-02, GATE-03, GATE-04]
---

# 03-07 — Phase close: GATE-03 sweep delta, number reconciliation, deferrals

## The headline

GATE-03's after-sweep ran over the same 22 directories as Plan 03's baseline, on the same machine,
with the release binary. **Across ~3,200 files in 21 external directories, the eight new CAT-01
patterns produced exactly one new finding, and it is a true positive.**

| File | Line | Pattern | Severity | Verdict |
|---|---:|---|---|---|
| `17-skills-registry/.claude/settings.local.json` | 3 | PI051 | CRITICAL | **TRUE POSITIVE** |

That file's `permissions.allow` really does contain `Bash(rm -rf *)`, `Bash(rm -rf ./*)` and
`Bash(jar xf *)` — pre-approved unrestricted grants in a sibling UnityInFlow repository. Reported,
not suppressed; no pattern was widened or narrowed in response. **Zero new false positives in
third-party documentation.**

## GATE-03 earned its place — the first after-sweep failed

The after-sweep ran twice, and the first run is the whole justification for the gate.

`PI057` (disable-guardrail-directive), exactly as Plan 06 shipped it, produced **49 third-party hits
of which 48 were false positives**:

- **`skip`** — ubiquitous in ordinary engineering prose (`GSD_SKIP_SCHEMA_CHECK`,
  `--skip-git-repo-check`) and, worse, it matched **negated** directives backwards: *"DO NOT skip
  the config gate check"* instructs a reader to KEEP a control, and the pattern read it as an
  instruction to remove one.
- **`remove`** — two more, both about removing a *stale* hook reference rather than disabling a
  *live* one.
- Dropping both cleared 47 of 48. The 48th, *"Pass `--no-nli` to disable the contradiction check
  entirely"* (`15-llm-diff/README.md:57`), proved `turn off`/`disable` alone were insufficient:
  ordinary CLI-flag documentation uses the identical verb+object grammar as the attack.

`PI057` was re-narrowed to require second-person address (`you` co-occurring with the verb+object
pair in one sentence — the framing PI054/PI055 already require), its `counter_example` replaced with
that llm-diff sentence verbatim, and `tests/corpus/clean/cli-flag-reference.md` added to pin the
fix. Clean corpus is now **21 specimens**, still zero.

**PI057 passed every unit test, held all 20 clean specimens, and scored 12/12 on the threat-model
corpus while being wrong 98% of the time on real documentation.** No corpus-based gate would have
caught that. This is the ENG-02 lesson repeating: only real bytes surface this class of defect.

## Sweep totals

| | Files | Findings |
|---|---:|---:|
| Baseline (pre-pattern, 2026-09-01) | 3,356 | 518 |
| After-sweep (post-pattern, 2026-09-02) | 3,371 | 575 |
| Delta | +15 | +57 |

Raw output committed under `sweep-2026-09-02/` (`manifest.tsv`, `summary.tsv`,
`third-party-new-findings.tsv`).

**Recorded caveat:** `--compare` keys on `path:line:pattern_id`, and the baseline swept this repo
through Plan 03's worktree while the after-sweep swept Plan 07's. All 516 self-repo findings
therefore appear "new" purely from the path change. They were audited rather than dismissed: 49 are
PI05x, every one in a file that carries attack strings by construction (corpus payloads, the
examples doc, the pattern YAML's own `example:` fields, `pattern_test.rs` cases); the other 467 are
pre-existing non-CAT-01 findings under a different prefix. A future sweep should normalise the repo
root before keying — logged in 03-SWEEP.md, not fixed here (GATE-04 forbids unrelated tooling
changes in a category PR).

## Self-scan

Outside `examples/`, `patterns/`, `tests/`, `tools/`, `docs/` the self-scan shows exactly the
findings Plan 03's baseline enumerated, unchanged: `src/normalize.rs` (4), `src/baseline.rs` (3),
`src/scanner.rs` (2), `benches/scan.rs` (1) — all doc comments, test literals and a benchmark
fixture — plus the 2 `docs/PATTERN-CATALOGUE.md` findings that predate this milestone. **Nothing
new.** Note the plan's must_have says "the two findings that predate this milestone"; the baseline
itself documented **12**, of which those 2 are the pre-milestone pair and 10 are `.rs` files that
became visible only when the sweep started using `--all-files`. Recording the real number rather
than the one the must_have anticipated.

## Number reconciliation

Every published number agrees:

| Claim | README | CHANGELOG | Catalogue | YAML | recall_test |
|---|---|---|---|---|---|
| Pattern count | 56 | PI050–PI057 added | 56 entries | 56 ids | — |
| Categories | 6 | — | — | — | — |
| CAT-01 recall | 12/12 | 12/12 | — | — | 7/7 prose + 5/5 structural |
| Library recall | 70/72 (97.2%) | 70/72 (97.2%) | — | — | pinned exactly |
| Corpus size | 72 payloads | — | — | — | 72 |

## Requirements

Marked complete in `.planning/REQUIREMENTS.md`: **CAT-01**, **GATE-02**, **GATE-03**, **GATE-05**
(GATE-01 was already marked by Plan 01).

**GATE-04 deliberately left unchecked.** CAT-01 shipped alone, so the gate's one-category-at-a-time
substance holds, but this milestone's GSD branching strategy is `none` — Phase 3 committed directly
to `main` and there is no PR object to point at. An explicit status note recording this sits under
the gate in REQUIREMENTS.md. It should be checked when the work is raised as a PR, or the gate
amended to say "its own reviewable unit".

## Self-Check

- `cargo test --locked` — 353 passed, 0 failed
- `cargo clippy --all-targets --locked -- -D warnings` — clean
- `cargo fmt --all -- --check` — clean
- `check tests/corpus/clean --strict` — 0 findings across 21 specimens
- GATE-03 after-sweep — 1 new third-party finding, true positive

## Commits

- `7a3eb78` — fix(03-07): re-narrow PI057 after GATE-03 found 48/49 false positives
- `14fe56d` — docs(03-07): record the GATE-03 after-sweep and its raw output

## Issues encountered

The executor was killed by a **machine-sleep event** after diagnosing the PI057 false positives and
writing the fix, but before committing anything and before re-running the after-sweep with the
fixed pattern. It had been instructed to commit the raw sweep before analysing it and did not,
which would have cost the entire 3,371-file sweep had the orchestrator not been able to re-run it.

**The orchestrator committed the surviving fix, re-ran the after-sweep, computed the delta,
triaged the single third-party finding, reconciled the published numbers, marked the requirements
and wrote this SUMMARY** — verifying every gate against the built binary rather than inheriting any
claim from the terminated agent.

This is the fourth consecutive plan in this phase requiring orchestrator rescue (03-05 lost three
runs to sleep, network loss and a ~200 load average; 03-06 lost one to a session rate limit; this
plan lost one to sleep). The environment, not the work, was the binding constraint throughout.
