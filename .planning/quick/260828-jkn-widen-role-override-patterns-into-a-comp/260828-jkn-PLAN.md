---
task_id: 260828-jkn
slug: widen-role-override-patterns-into-a-comp
issue: 80
branch: feat/pi80-role-override-matrix
status: in-progress
---

# Quick task 260828-jkn — role-override as a matrix, not seven literals (#80)

## Problem

`role_override` is the flagship category and it is seven near-literal phrases. Each
pins a specific verb *and* object *and* modifier, so a synonym walks past all of them:
`disregard all previous instructions` misses PI007 (which needs the literal `your`),
`forget everything you were told before` misses PI006 (which needs `you know`).

The recall corpus merged in #92 puts a number on it: **1/12** on
`tests/corpus/attack/role-override.md`. That corpus is written from the threat model,
not from the regexes, so it is a fair target.

## Constraint that shapes the work

`PI001`–`PI009` is the whole reserved block for this category and all nine IDs are in
use. Every other block is reserved for a category that does not exist yet (#33–#40).
So this is a **widening in place**, not an addition: each ID keeps its concept and
grows from a literal into a composition of verb × modifier × object.

## Approach

Building blocks, applied per ID rather than as one mega-regex:

- **nullify verbs** — ignore, disregard, forget, discard, override, bypass, set/put aside, abandon
- **priorness** — previous, prior, earlier, preceding, foregoing, original, initial, existing, current, above
- **instruction objects** — instructions, directives, directions, guidelines, guidance, rules, constraints, restrictions, context, prompt, programming, training, persona, system prompt/message

Deliberately **not** in the priorness list: `old`, `legacy`. `tests/corpus/clean/agent-spec.md`
says "Ignore the legacy `v1/` package" and "Forget the old naming convention" — ordinary
maintenance prose in the single most common document this scanner is pointed at.

The object noun stays **required** wherever the verb is an ordinary English one. That is
what keeps `hard-wrapped-prose.md` clean: "Reviewers should ignore all previous" sits on
its own line above a `## Instructions for reviewers` heading, and the payload only exists
if the join crosses the heading.

## Tasks

1. **Widen the nine patterns** in `patterns/core/role-override.yaml`; keep each `id`,
   `name` concept and severity. Update `example`/`counter_example` where the concept moved.
2. **Test cases** — ≥3 positive, ≥2 negative per changed pattern in `tests/pattern_test.rs`,
   per the `PATTERNS.md` policy the CI ratchet enforces.
3. **Re-measure and re-publish** — `EXPECTED` in `tests/recall_test.rs`, the recall table in
   `README.md`, and `docs/PATTERN-CATALOGUE.md` regenerated, all in the same commit.
4. **Re-baseline** `.github/code-scanning-baseline.json` — widened patterns match more of
   this repo's own `patterns/`, `examples/` and `tests/fixtures/`.

## Gates (both directions)

| Gate | Requirement |
|---|---|
| `recall_test` | role-override rises from 1/12; number pinned exactly and published |
| `corpus_test` | `tests/corpus/clean/` stays at **0** findings, including under `--strict` |
| `corpus_test` | `tests/corpus/documentation/` stays at 0 by default, >0 under `--strict` |
| `pattern_example_test` | every `example` matches, every `counter_example` does not |
| `pattern_policy_test` | per-pattern case counts hold |
| `catalogue_test` | catalogue regenerated |

**The corpus rule:** if a widened pattern fires on `tests/corpus/clean/`, narrow the
pattern. Editing the corpus to fit inverts the only false-positive gate this repo has.

## Out of scope

`exfiltration`, `instruction-injection` and `jailbreak` are also at 0/12 and have the same
defect. They are the same fix in three more files and belong in their own PRs — one
category per PR keeps the false-positive blast radius reviewable.
