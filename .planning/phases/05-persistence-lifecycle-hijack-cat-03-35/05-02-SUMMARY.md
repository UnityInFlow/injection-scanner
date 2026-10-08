---
phase: 05-persistence-lifecycle-hijack-cat-03-35
plan: 02
subsystem: testing
tags: [gate-05, false-positive-corpus, clean-corpus, lifecycle-hooks, documentation-corpus, rust]

requires:
  - phase: 05-persistence-lifecycle-hijack-cat-03-35
    provides: plan 05-01 corpus (12 blind payloads), the recall pin, the two-wrapper projection probe
provides:
  - six prose false-positive specimens (flat persistence-*.md) in tests/corpus/clean/
  - three legitimate agent hooks configurations (flat persistence-*.json) across three wrapper conventions
  - a two-sided documentation write-up with its strict-mode matches attributed to PI070
  - two recorded mutation proofs (cue-less / determiner-blind prose reading; binding-only structural reading)
affects: [05-03, 05-04, 05-05, 05-06, 05-07]

actuals:
  tokens: 14000
  tasks: 3
  commits: 3

key-files:
  created:
    - tests/corpus/clean/persistence-shell-install-prose.md
    - tests/corpus/clean/persistence-git-hook-docs.md
    - tests/corpus/clean/persistence-memory-feature-docs.md
    - tests/corpus/clean/persistence-instruction-file-writes.md
    - tests/corpus/clean/persistence-file-template-conventions.md
    - tests/corpus/clean/persistence-hook-setup-docs.md
    - tests/corpus/clean/persistence-legitimate-hooks-config.json
    - tests/corpus/clean/persistence-local-hook-endpoint.json
    - tests/corpus/clean/persistence-root-form-hooks-config.json
    - tests/corpus/documentation/persistence-lifecycle-hijack-writeup.md
  modified:
    - tests/corpus/clean/README.md
    - tests/corpus/documentation/README.md

key-decisions:
  - "Added two synthesized sections to the instruction-file specimen because the verbatim-sourced sentences carry no deictic content and so could not exercise a deictic conjunct at all"
  - "Did not put the 'save these instructions ... so they persist across sessions' vendor sentence in clean/: 05-RESEARCH.md Q3 says it would make PI073 unshippable"

requirements-completed: [CAT-03, GATE-05]

duration: ~35min
completed: 2026-10-08
status: complete
---

# Phase 5 Plan 02: CAT-03 false-positive gate, built before the patterns Summary

**Nine flat `persistence-*` clean specimens (six prose, three hooks configs across nested / flat / mixed-root wrapper shapes) plus a two-sided write-up are in the tree before any PI071+ pattern; a cue-less prose probe fires on the instruction-file specimen on 2 lines and a binding-only structural probe fires on all three hooks configs (8, 3 and 4 projected lines); `git diff --stat patterns/` is empty.**

## Commits

| Task | Hash | What |
|---|---|---|
| 1 | `6c54bf7` | six prose specimens + CAT-03 section with six `Decision it defends` rows |
| 2 | `0aad363` | three hooks-config specimens + three README rows |
| 3 | `36515f1` | documentation write-up + documentation README section |

## Specimens: decision defended and provenance

"Sourced" means copied as quoted in `05-RESEARCH.md` §Q1 (or verified on disk, where stated). Third-party web pages were **not re-fetched** in this plan (no fetch tool was available to me); each header says so. Synthesized lines are labelled in the file.

| File | Decision | Sourced | Synthesized |
|---|---|---|---|
| `persistence-shell-install-prose.md` | D-02 / PI070 / PI071 | Homebrew and nvm install sentences (as quoted in §Q1) | SSH `authorized_keys` and `crontab` sentences |
| `persistence-git-hook-docs.md` | Q4 / Pitfall 5 | Pro Git sentence (as quoted in §Q1); `core.hooksPath=/dev/null` clone, verified on disk (`suggest-patches.md:66`) | one team-convention sentence |
| `persistence-memory-feature-docs.md` | D-04 / PI074 grading | three Claude memory-page fragments (as quoted in §Q1; the elided middle of the third is not reconstructed) | none |
| `persistence-instruction-file-writes.md` | D-04 | `spike-wrap-up.md:215`, `claude-md-improver/SKILL.md:11`, `reflect.md:60`, `revise-claude-md.md:2` all verified on disk | the deictic routing-line variant; two vendor-README lines (`your CLAUDE.md`, `.cursorrules in your project root`) |
| `persistence-file-template-conventions.md` | D-02 | none | all of it (the licence-header research rows are search summaries, not fetched) |
| `persistence-hook-setup-docs.md` | Q4 | two sentences from this repo's `CLAUDE.md:7` and `.planning/PROJECT.md:4,42` | the hooks-guide-register sentences and the `install-hook` usage sentence |
| `persistence-legitimate-hooks-config.json` | D-03 AMENDMENT | the inline commands from the hooks guide, as quoted in §Q1 | the `python3` script-path and plugin-root commands, matchers, nesting |
| `persistence-local-hook-endpoint.json` | D-03 AMENDMENT | none | loopback port, webhook command (credential path comes from an env var) |
| `persistence-root-form-hooks-config.json` | D-03 AMENDMENT | the `cargo fmt` guard command, as quoted in §Q2 | the mixed shape itself |

The write-up `tests/corpus/documentation/persistence-lifecycle-hijack-writeup.md` is synthesized prose; its payloads are shapes already in `05-CONTEXT.md` and `05-01`.

## `--strict` trips against the current pattern set

None. Every candidate was scanned with the release binary before commit. After all nine were added:

```
files 42 findings 0        # tests/corpus/clean at --strict --format json; matches + low_confidence + suppressed
```

(`tests/corpus/clean` held 39 entries before the three JSON files, 42 after; the six `.md` entries were counted in the 39.) No pre-existing false positive was found, so nothing is reported for GATE-04.

Baseline for plan 05-05 to re-check, `--strict`:

```
check CLAUDE.md (repo)                                           -> No injection patterns detected.
check .planning/PROJECT.md                                       -> No injection patterns detected.
check <ecosystem>/unity-in-flow-ai/CLAUDE.md                     -> No injection patterns detected.
```

## Mutation proof 1: cue-less, determiner-blind prose reading (D-04)

Probe: one prose regex requiring only a write verb, deictic content (`these|the following|the above|the below|this|those` + an instruction noun) and an agent-loaded instruction-file name, no determiner restriction, no future-reader cue. Loaded with `--patterns <scratch dir outside patterns/>`; positive control: `echo "Write these rules to CLAUDE.md so future sessions follow them." | check - --strict` printed `[('PI999', 'Write these rules to CLAUDE.md')]`, so the probe loads and fires.

**First run, with only the sourced sentences: the probe fired on nothing.** The four verbatim-sourced sentences carry no deictic content ("an auto-load routing line", "learnings from this session"), so as first written the specimen did not exercise the conjunct. I added the two synthesized sections described above. Second run over `tests/corpus/clean --strict`:

```
PROBE tests/corpus/clean/persistence-instruction-file-writes.md
  ('PI999', 57, 'prose', "Add the following line to the project's CLAUDE.md")
  ('PI999', 61, 'prose', 'Add the following rules to your CLAUDE.md')
```

Line 57 (the project's determiner, no future-reader cue) proves the durability-cue requirement is load-bearing; line 61 (`your`) proves the `your` exclusion is. Probe directory deleted afterwards (`ls` of the scratch dir, `grep -c probe-` printed `0`).

Caveat: the probe's noun slot needs the noun directly after the deictic word, so "Copy the rules below into .cursorrules" (postposed form) is not covered by this probe. That sentence is in the specimen for a pattern with a postposed arm and was not shown to be load-bearing here.

## Mutation proof 2: binding-only structural reading (D-03 AMENDMENT)

Probe: one `scope: frontmatter` regex, `(?:^hooks\.[^=\s]+|^[A-Za-z]+\[\d+\]\.hooks\[\d+\])\.(?:command|bash|powershell|url)\s*=\s*\S`, i.e. a lifecycle binding in the path and any non-empty value. Per-specimen projected-line hits over `tests/corpus/clean --strict`:

```
persistence-legitimate-hooks-config.json   8   [lines 10, 10, 10, 10, 10, 10, 10, 10]
persistence-local-hook-endpoint.json       3   [lines 13, 8, 20]
persistence-root-form-hooks-config.json    4   [lines 8, 8, 8, 8]
```

All three fire, every count greater than zero. Positive control (probe armed): over `tests/corpus/attack/structural/persistence-lifecycle-hijack` it hit all five plan 05-01 payloads (3, 1, 1, 1, 1). Probe directory deleted (`grep -c probe-` printed `0`). A binding-only `PI077` draft in plan 05-06 therefore fails `corpus_test` instead of reaching review.

**Observation, not fixed:** in the first file all eight hits report line 10, the line of the first `command` key. The projection's `locate()` maps repeated leaf keys to their first occurrence, so suppression directives and baseline fingerprints cannot tell hook commands apart in a file with repeated `command` keys. `05-RESEARCH.md` Pitfall 10 records the compact-object variant of this; the repeated-key variant is the same mechanism and I did not find it recorded.

Other acceptance checks:

```
python3 t2check.py -> distinct shapes 3
  persistence-legitimate-hooks-config.json   wrapper-nested
  persistence-local-hook-endpoint.json       wrapper-flat
  persistence-root-form-hooks-config.json    mixed(root+wrapper)   root-event-keys= ['SessionStart', 'SubagentStart']
  writes into ~/.claude: ['jq -r .tool_input.command >> ~/.claude/bash.log']
find tests/corpus/clean -mindepth 1 -type d | wc -l   -> 0
```

## Documentation specimen: both directions

Default threshold:

```
No injection patterns reported.
2 findings withheld as documentation (code blocks, inline spans, tables). Re-run with --strict to see them.
```

`--strict`:

```
:37 HIGH ... (PI070)  [fenced code · confidence 0.2]
:58 HIGH ... (PI070)  [table · confidence 0.3]
2 finding(s): 0 critical, 2 high, 0 medium, 0 low
```

Both strict matches are `PI070`, which already ships; the file header names it and states plan 05-07 re-checks. A script that strips comments, fences, table rows and inline spans left no payload token (`authorized_keys`, `CLAUDE.md`, `.zshrc`, `~/.ssh`, `cp ~/`) in the remaining prose.

## Verification

| Gate | Result |
|---|---|
| `cargo test --test corpus_test --locked` | 5 passed, including `the_clean_corpus_matches_nothing_even_under_strict` and both documentation directions (run after Task 2 and after Task 3) |
| `cargo test --test pattern_relaxed_control_test --locked` | 4 passed |
| `cargo test --test frontmatter_test --locked` | 41 passed |
| `cargo test --test markdown_context_test --locked` | 31 passed |
| `cargo test --test markdown_context_test --locked the_projects_own_documentation_is_clean` | `running 1 test`, 1 passed, 30 filtered out |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --all-targets --locked -- -D warnings` | clean |
| `git diff --stat e350f2f HEAD -- patterns Cargo.toml Cargo.lock \| wc -l` | `0` |
| `git diff --name-only e350f2f HEAD` | the 12 plan files only; no CAT-01/CAT-02 specimen, no `STATE.md`, no `ROADMAP.md` |

I did **not** run the full `cargo test` (about six minutes; scoped to the named binaries above, per the brief).

## Deviations from Plan

**1. [Rule 2 - correctness] Two synthesized sections added to `persistence-instruction-file-writes.md`.** The plan's task 1 mutation proof requires the cue-less probe to fire on this specimen. With the four sourced sentences alone it fired on nothing (recorded above). A specimen that does not trip the mutation it exists to catch is decorative, so I added a deictic variant of the routing line and two vendor-README lines, each labelled synthesized in the header. Commit `6c54bf7`.

**2. [Plan wording] The documentation README has no specimen table, only the two-direction contract table.** The plan says "add the row to the table". I added a short `## CAT-03` section with a one-row table rather than invent rows for the existing specimens. Commit `36515f1`.

**3. Task 1 acceptance "exactly six rows" vs. nine at close.** After Task 2 the CAT-03 section has nine rows, as the plan's Task 2 criterion requires.

**4. Slip caught before commit.** My first README edit replaced the `## Provenance` heading line instead of preceding it, and one comment close was typed `->` instead of `-->`, which made the rest of a specimen read as an HTML comment. Both were found by re-reading `grep -n '^## '` output and by the probe reporting `context: html_comment`, and fixed before any commit.

## Known Stubs

None.

## Threat Flags

None. Test fixtures and README text only; no endpoint, auth path or parser change.

## Self-Check: PASSED

- All nine `tests/corpus/clean/persistence-*` specimens and `tests/corpus/documentation/persistence-lifecycle-hijack-writeup.md` exist (present in `git diff --name-only e350f2f HEAD`).
- Commits `6c54bf7`, `0aad363`, `36515f1` are in `git log`.
- `find tests/corpus/clean -mindepth 1 -type d | wc -l` printed `0`; `patterns/`, `Cargo.toml`, `Cargo.lock` unchanged.
- Not self-checked: the SUMMARY commit itself.
