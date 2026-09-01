---
phase: 03-tool-permission-abuse-cat-01-33
plan: 04
subsystem: pattern-schema
tags: [rust, serde, pattern-library, gate-05, mutation-testing, ci]

# Dependency graph
requires:
  - phase: 03-tool-permission-abuse-cat-01-33
    provides: the `Pattern` struct (`raw_only`, `example`/`counter_example` precedent) and the
      pattern-policy ratchet machinery this plan extends
provides:
  - "`Pattern::relaxed_pattern: Option<String>` — the D-08 one-way schema field, additive and
    typo-protected via `deny_unknown_fields`"
  - "`tests/pattern_relaxed_control_test.rs` — the D-07 GATE-05 mutation-pairing gate, proven
    non-vacuous by a synthetic mechanism self-test"
  - "`every_pi05x_pattern_carries_a_relaxed_pattern` — the D-09 ratchet requiring the field for
    PI050 and above"
  - authoring-document updates (PATTERNS.md, pattern-library SKILL.md) describing the contract
affects: [03-05, 03-06, 03-07, CAT-02, CAT-03]

# Actuals (#2632)
actuals:
  tokens: 7522
  tasks: 4
  commits: 3

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "One-way schema field decisions (deny_unknown_fields means a field name/semantics choice
       cannot be revised after a community pattern file ships with it)"
    - "Mutation-pairing test: reuse the production `Scanner` over a mutated pattern set rather
       than a bare `regex::Regex`, so a false-positive control is proven under the same compile
       flags, scope routing, normalization and decode passes the shipped pattern gets"
    - "Anti-vacuity mechanism self-test: a synthetic fixture proves a gate's mechanism works even
       while zero real data exercises it yet"

key-files:
  created:
    - tests/pattern_relaxed_control_test.rs
  modified:
    - src/pattern.rs
    - tests/case_sensitivity_test.rs
    - tests/raw_only_test.rs
    - tests/pattern_validation_test.rs
    - tests/frontmatter_test.rs
    - tests/pattern_policy_test.rs
    - PATTERNS.md
    - .claude/skills/pattern-library/SKILL.md

key-decisions:
  - "D-08 (resolved by the developer, Option A, 2026-09-01): the field is named `relaxed_pattern`,
     typed `Option<String>` with `#[serde(default)]`, positioned immediately after
     `counter_example`. This is a one-way naming decision — `deny_unknown_fields` means a
     community pattern file that ships with this field cannot have it renamed later without
     breaking that file's load."
  - "D-08a (already resolved, reaffirmed here): `relaxed_pattern` is never rendered into
     docs/PATTERN-CATALOGUE.md. The shipped regex is already public there in the Regex details
     block, so no new disclosure exists; the field is test scaffolding describing what the
     scanner deliberately does not detect."
  - "D-09 (already resolved, enforced here): `relaxed_pattern` is required for PI050 and above;
     the existing 48 patterns stay exempt because a 48-file migration inside a category PR
     violates GATE-04."

patterns-established:
  - "Mutation-pairing gate: a schema field pairs a shipped value with a deliberately-broken
     variant, and CI asserts the two diverge on a specific probe (here: counter_example)."

requirements-completed: [CAT-01, GATE-05]

coverage:
  - id: D1
    description: "Pattern::relaxed_pattern schema field — additive, deny_unknown_fields-protected,
      absent from the generated catalogue"
    requirement: GATE-05
    verification:
      - kind: unit
        ref: "tests/pattern_validation_test.rs#relaxed_pattern_field_loads_as_some_when_present"
        status: pass
      - kind: unit
        ref: "tests/pattern_validation_test.rs#relaxed_pattern_field_loads_as_none_when_absent"
        status: pass
      - kind: unit
        ref: "tests/pattern_validation_test.rs#a_misspelled_relaxed_pattern_field_is_rejected"
        status: pass
      - kind: other
        ref: "cargo run --release -- rules --format markdown | diff - docs/PATTERN-CATALOGUE.md (no output)"
        status: pass
    human_judgment: false
  - id: D2
    description: "The D-07 mutation-pairing gate: pattern_relaxed_control_test.rs proves a
      pattern's counter_example is missed by the shipped regex and caught by relaxed_pattern,
      via a real Scanner, and is non-vacuous via a synthetic mechanism self-test"
    requirement: GATE-05
    verification:
      - kind: unit
        ref: "tests/pattern_relaxed_control_test.rs#the_relaxation_mechanism_itself_discriminates_narrow_from_relaxed"
        status: pass
      - kind: unit
        ref: "tests/pattern_relaxed_control_test.rs#shipped_pattern_misses_counter_example_but_relaxed_form_catches_it"
        status: pass
      - kind: unit
        ref: "tests/pattern_relaxed_control_test.rs#clean_corpus_is_held_by_the_shipped_set_and_broken_by_the_relaxed_set"
        status: pass
      - kind: unit
        ref: "tests/pattern_relaxed_control_test.rs#every_relaxed_pattern_compiles_and_differs_from_its_own_pattern"
        status: pass
    human_judgment: false
  - id: D3
    description: "D-09 ratchet: every PI050+ pattern must carry relaxed_pattern, no exemption list"
    requirement: GATE-05
    verification:
      - kind: unit
        ref: "tests/pattern_policy_test.rs#every_pi05x_pattern_carries_a_relaxed_pattern"
        status: pass
    human_judgment: false
  - id: D4
    description: "PATTERNS.md and the pattern-library SKILL.md describe the relaxed_pattern
      contract, its PI050+ requirement, and its deliberate absence from the catalogue"
    human_judgment: true
    rationale: "Documentation clarity and completeness is a judgment call a human reviewer is
      better positioned to assess than an automated check; the grep-count acceptance criteria
      (>=2 mentions in each file) were verified mechanically but do not establish quality of
      exposition."

duration: 55min
completed: 2026-09-01
status: complete
---

# Phase 03 Plan 04: Pattern relaxed_pattern schema field + GATE-05 mutation-pairing gate Summary

**Added `Pattern::relaxed_pattern`, the one-way D-08 schema field, and turned "break it and
confirm the corpus goes red" into an enforced CI gate (`tests/pattern_relaxed_control_test.rs`)
proven non-vacuous by a synthetic self-test even while zero shipped patterns use the field yet.**

## Performance

- **Duration:** 55 min
- **Started:** 2026-09-01T12:56:00Z (approx, prior executor round reached Task 1's checkpoint)
- **Completed:** 2026-09-01T13:51:02Z
- **Tasks:** 4 (Task 1 resolved via developer decision, Tasks 2-4 executed)
- **Files modified:** 9 (1 created, 8 modified)

## Accomplishments

- `Pattern::relaxed_pattern: Option<String>` added to `src/pattern.rs`, positioned immediately
  after `counter_example`, additive and protected by the struct's `deny_unknown_fields`. Doc
  comment states what it is, why it exists, that it is never loaded into the live scanner, that
  CI pairs it against `counter_example`, its PI050+ requirement, and its deliberate absence from
  the catalogue.
- Three schema tests added to `tests/pattern_validation_test.rs`: loads as `Some` when present,
  `None` when absent, and a misspelled field name is rejected — proving `deny_unknown_fields`
  extends to the new field, the entire reason D-08 chose a schema field over a tag.
- `tests/pattern_relaxed_control_test.rs` created with four tests implementing GATE-05: a
  synthetic mechanism self-test (anti-vacuity control, T-03-11), the `counter_example` pairing
  across every pattern that carries the field, a set-level clean-corpus pairing, and a
  well-formedness check (T-03-12) rejecting a relaxed form identical to its own pattern. All four
  build scanners via `Scanner::new`, never a bare `regex::Regex`.
- `every_pi05x_pattern_carries_a_relaxed_pattern` added to `tests/pattern_policy_test.rs`: the
  D-09 ratchet requiring `relaxed_pattern` for every pattern id in PI050-PI059, unconditionally
  (no exemption list, since PI050+ is entirely new).
- `PATTERNS.md` and `.claude/skills/pattern-library/SKILL.md` updated: schema blocks, a new
  `relaxed_pattern` section citing #95/#97, a gates-table row, the automated (no longer manual)
  mutation-testing checklist item, and the corrected corpus file count (fifteen, not fourteen).
- `docs/PATTERN-CATALOGUE.md` is byte-identical before and after (`git diff --stat
  src/catalogue.rs` is empty) — D-08a held.

## Task Commits

Each task was committed atomically:

1. **Task 1: Lock the field name and semantics (D-08)** — resolved by the developer as **Option
   A** on 2026-09-01, recorded verbatim below. No file changes; the decision governs Tasks 2-4.
2. **Task 2: Add the schema field and repair every struct-literal construction** - `14168b1`
   (feat)
3. **Task 3: The D-07 mutation-pairing gate** - `10dfbf2` (test)
4. **Task 4: The D-09 ratchet and the two authoring documents** - `f70a0aa` (docs)

_No plan metadata commit yet — that is this commit, made immediately after this file per the
worktree parallel-executor protocol._

## Locked Decision: D-08

**Field name:** `relaxed_pattern`
**Chosen by:** the developer (Option A, as proposed)
**Date:** 2026-09-01

```rust
pub struct Pattern {
    // ...
    pub counter_example: Option<String>,

    /// Deliberately widened variant of `pattern` with the
    /// narrowing removed. Never loaded into the live scanner;
    /// CI asserts `pattern` does NOT match `counter_example`
    /// while `relaxed_pattern` DOES.
    #[serde(default)]
    pub relaxed_pattern: Option<String>,
}
```

Semantics: `Option<String>` with `#[serde(default)]`, immediately after `counter_example`; a
deliberately widened variant of the pattern's own regex with the narrowing removed; never loaded
into the live scanner; CI asserts the shipped `pattern` does NOT match `counter_example` while
`relaxed_pattern` DOES; required for `PI050` and above (the existing 48 patterns stay exempt); NOT
rendered into `docs/PATTERN-CATALOGUE.md` (D-08a). Rejected alternatives: `relaxed_form`,
`fp_control_mutation`, `counter_pattern`, and the option-C list-of-variants shape — none revisited.

## Files Created/Modified

- `src/pattern.rs` - the `relaxed_pattern: Option<String>` field, doc comment
- `tests/pattern_relaxed_control_test.rs` - new: the four-test GATE-05 mutation-pairing gate
- `tests/pattern_validation_test.rs` - three new schema tests for `relaxed_pattern`; struct literal
  updated
- `tests/pattern_policy_test.rs` - the D-09 `every_pi05x_pattern_carries_a_relaxed_pattern` ratchet
- `tests/case_sensitivity_test.rs`, `tests/raw_only_test.rs`, `tests/frontmatter_test.rs` - struct
  literals updated so the crate compiles with the new field
- `PATTERNS.md` - schema block + new `relaxed_pattern` section
- `.claude/skills/pattern-library/SKILL.md` - schema block, gates table row, automated checklist
  item, corrected corpus count

## Decisions Made

- D-08 locked as `relaxed_pattern` (see above) — the developer's explicit choice, recorded before
  any code was written.
- Kept `#[serde(default)]` on `relaxed_pattern` for consistency with sibling optional fields
  (`raw_only`, `case_sensitive`) even though the mutation check below showed it is not load-bearing
  for this particular field's missing-field behaviour — see Deviations.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed a clippy `cloned_ref_to_slice_refs` lint in the new test file**
- **Found during:** Task 3, running the plan-level `cargo clippy --all-targets --locked -- -D
  warnings` gate
- **Issue:** `Scanner::new(&[narrow.clone()])` cloned a value only to build a one-element slice;
  clippy's `cloned_ref_to_slice_refs` lint (new in this toolchain) flags this as unnecessary.
- **Fix:** Changed to `Scanner::new(std::slice::from_ref(&narrow))`, matching the idiom already
  used elsewhere in the test suite (e.g. `tests/case_sensitivity_test.rs`).
- **Files modified:** `tests/pattern_relaxed_control_test.rs`
- **Verification:** `cargo clippy --all-targets --locked -- -D warnings` clean; `cargo test
  --test pattern_relaxed_control_test --locked` still 4/4 green.
- **Committed in:** `10dfbf2` (Task 3 commit)

### Noteworthy Finding (not a deviation, but a plan-assumption correction)

**Task 2's `#[serde(default)]` mutation check did not fail as the plan predicted.** The
acceptance criterion said "Temporarily deleting `#[serde(default)]` from the new field makes the
absent-field test FAIL." Performing the check (delete the attribute, run
`relaxed_pattern_field_loads_as_none_when_absent`, observe, then restore) showed the test still
**passed**. Root cause: `serde_derive`'s generated `Deserialize` impl special-cases any field of
type `Option<T>` at the missing-field code path (`missing_field()`'s `MissingFieldDeserializer`
overrides `deserialize_option` to call `visitor.visit_none()`), independent of whether
`#[serde(default)]` is present. This is documented serde behaviour, not a bug in this crate.
`#[serde(default)]` was restored and kept on `relaxed_pattern` anyway, for consistency with the
sibling optional fields (`raw_only`, `case_sensitive`) that carry it explicitly even though the
attribute is likewise not strictly load-bearing for their `Option<T>` types either — the crate's
existing convention is to state `#[serde(default)]` explicitly on every optional field regardless
of whether serde's `Option<T>` special case alone would suffice, and this field follows it.
This does not affect `deny_unknown_fields` (which does still reject typos, verified by a separate
passing test) or the field's `Some`/`None` round-trip (both verified). No code change resulted;
this is recorded because the plan's stated expectation for that specific mutation check was
incorrect for this Rust/serde combination, and a future planner should not rely on that
particular mutation check to prove `Option<T>` field correctness in this codebase.

---

**Total deviations:** 1 auto-fixed (1 bug — clippy lint), plus 1 documented plan-assumption
correction (no code change).
**Impact on plan:** Neither affects GATE-05's actual guarantees. The clippy fix is cosmetic idiom
alignment. The serde finding narrows what the `#[serde(default)]` mutation check can prove for
`Option<T>` fields specifically, but the field's real behavioural contract (loads as `Some`/`None`
correctly, rejects typos under `deny_unknown_fields`) is independently verified by the three
schema tests, which all pass.

## Issues Encountered

None beyond the deviation above.

## Mutation Checks Performed (all reverted, no lasting diff)

1. **Task 2 — `#[serde(default)]` removal on `relaxed_pattern`.** Removed, ran
   `relaxed_pattern_field_loads_as_none_when_absent`, observed **PASS** (not the predicted FAIL —
   see Deviations), restored. `git diff` on `src/pattern.rs` after restore showed no leftover
   change.
2. **Task 3 — mechanism self-test regex inversion.** Swapped the narrow/relaxed synthetic regex
   values in `the_relaxation_mechanism_itself_discriminates_narrow_from_relaxed`, ran the test,
   observed **FAIL** (`assert_eq!` on the sanity check tripped first, then would have tripped the
   real assertions), restored from a `/tmp` backup; `diff` against the backup after restore was
   empty.
3. **Task 3 — well-formedness identical-pair check.** Temporarily appended a `PITEMPCHECK`
   pattern with `pattern` and `relaxed_pattern` set to the same literal string to
   `patterns/core/encoding.yaml`, ran `every_relaxed_pattern_compiles_and_differs_from_its_own_pattern`,
   observed **FAIL** naming `PITEMPCHECK`, restored from a `/tmp` backup; `git status --short` on
   the file after restore was empty.
4. **Task 4 — D-09 ratchet failure message.** Temporarily appended a `PI050` pattern with no
   `relaxed_pattern` to `patterns/core/encoding.yaml`, ran
   `every_pi05x_pattern_carries_a_relaxed_pattern`, observed **FAIL** with the message naming
   `PI050` exactly, restored from a `/tmp` backup; `git status --short` on the file after restore
   was empty.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `relaxed_pattern` is available for use by any subsequent plan in this phase that introduces a
  PI050+ pattern (03-05, 03-06, 03-07) — those plans MUST populate the field or
  `every_pi05x_pattern_carries_a_relaxed_pattern` will fail their build.
- `tests/pattern_relaxed_control_test.rs`'s two set-level tests currently print an explicit SKIP
  notice for their relaxed-half assertions, since no shipped pattern carries the field yet. The
  first PI050+ pattern (Plan 05, per the plan's own comments) will exercise those paths for real;
  no code change is needed for that transition to happen automatically.
- No blockers.

---
*Phase: 03-tool-permission-abuse-cat-01-33*
*Completed: 2026-09-01*
