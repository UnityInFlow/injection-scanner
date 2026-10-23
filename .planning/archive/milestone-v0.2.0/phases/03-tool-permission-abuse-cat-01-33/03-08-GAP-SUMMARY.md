---
phase: 03-tool-permission-abuse-cat-01-33
plan: 08-GAP
subsystem: pattern-library
tags: [regex, negation, false-positive, prose-detection, tool-permission-abuse]

# Dependency graph
requires:
  - phase: 03-tool-permission-abuse-cat-01-33
    provides: PI050-PI057 (Plans 01-07), the code review that found CR-01 (03-REVIEW.md)
provides:
  - Negation guard on PI053/PI056/PI057 so a prohibition sentence no longer matches
  - Negative-polarity clean-corpus specimen (tests/corpus/clean/security-policy-prohibitions.md)
  - PATTERNS.md and structural attack-corpus README documentation fixes (WR-01, WR-02)
  - Re-measured GATE-03 sweep delta confirming zero third-party regression
affects: [any future PI05x-range prose pattern, GATE-05 mutation-pairing gate]

# Actuals (#2632)
actuals:
  tokens: 11199
  tasks: 8
  commits: 7

tech-stack:
  added: []
  patterns:
    - "Line-initial anchor (`^\\s*`) as a lookaround-free negation guard for imperative prose patterns"

key-files:
  created:
    - tests/corpus/clean/security-policy-prohibitions.md
    - .planning/phases/03-tool-permission-abuse-cat-01-33/03-08-gate03-manifest.tsv
    - .planning/phases/03-tool-permission-abuse-cat-01-33/03-08-gate03-summary.tsv
  modified:
    - patterns/core/tool-permission-abuse.yaml
    - tests/pattern_test.rs
    - examples/tool-permission-abuse-attack.md
    - PATTERNS.md
    - tests/corpus/attack/structural/README.md
    - docs/PATTERN-CATALOGUE.md
    - .github/code-scanning-baseline.json
    - .planning/phases/03-tool-permission-abuse-cat-01-33/03-SWEEP.md

key-decisions:
  - "Chose the sentence/line-initial anchor (`^\\s*`) over a preceding-negator-token check or an explicit non-negated alternation, because it is expressible in the Rust `regex` crate without lookaround and every required positive is already the first thing on its own line."
  - "Simplified the anchor from `(?:^|[.!?]|\\n)\\s*` to `^\\s*` after discovering the punctuation/newline branch duplicated findings across `multiline.rs`'s paragraph-join pass — `^` is zero-width and cannot consume a character from a preceding physical line, eliminating the duplicate at the cost of not reaching a directive whose own words are hard-wrapped across a line break."
  - "Made the negation guard the GATE-05 narrowing under test for all three patterns (new counter_example = a CR-01 negated sentence, relaxed_pattern = the pre-fix regex), rather than trying to preserve the old counter_example pairing alongside it — the schema supports exactly one relaxed_pattern/counter_example per id, and the negation guard is the more safety-critical narrowing to keep gate-enforced."

requirements-completed: []

coverage:
  - id: D1
    description: "PI053/PI056/PI057 no longer fire on prohibition sentences (the four CR-01 reproductions)"
    verification:
      - kind: integration
        ref: "manual: injection-scanner check <CR-01 sentences> --strict, verified zero findings"
        status: pass
      - kind: unit
        ref: "tests/pattern_test.rs#test_pi053_skip_permissions_flag, test_pi056_widen_settings_directive, test_pi057_disable_guardrail_directive"
        status: pass
    human_judgment: false
  - id: D2
    description: "No detection regression: CAT-01 recall stays 12/12, library recall stays 70/72"
    verification:
      - kind: unit
        ref: "tests/recall_test.rs#recall_matches_the_recorded_numbers, the_cat_01_payload_totals_sum_to_twelve"
        status: pass
    human_judgment: false
  - id: D3
    description: "Negative-polarity clean-corpus specimen added; clean corpus stays 0 under --strict"
    verification:
      - kind: integration
        ref: "tests/corpus_test.rs#the_clean_corpus_matches_nothing_even_under_strict"
        status: pass
    human_judgment: false
  - id: D4
    description: "GATE-05/D-09 hold: every PI05x pattern carries a relaxed_pattern relaxing its real (now negation-guard) narrowing"
    verification:
      - kind: unit
        ref: "tests/pattern_relaxed_control_test.rs#shipped_pattern_misses_counter_example_but_relaxed_form_catches_it, tests/pattern_policy_test.rs#every_pi05x_pattern_carries_a_relaxed_pattern"
        status: pass
    human_judgment: false
  - id: D5
    description: "GATE-03 sweep re-run; delta recorded (zero third-party regression)"
    verification:
      - kind: manual_procedural
        ref: "scripts/gate03-sweep.sh over the pinned 22-directory list, results in 03-08-gate03-manifest.tsv/summary.tsv and 03-SWEEP.md's new section"
        status: pass
    human_judgment: true
    rationale: "The sweep's third-party corpus is machine-local and not reproducible by another agent without this exact machine's sibling directories; a human should spot-check 03-SWEEP.md's new section against the committed raw manifest/summary before trusting the zero-delta claim."
  - id: D6
    description: "WR-01 (PATTERNS.md categories table) and WR-02 (structural attack-corpus README) documentation drift fixed"
    verification:
      - kind: other
        ref: "PATTERNS.md Categories table now has the Tool & Permission Abuse row; tests/corpus/attack/structural/README.md documents all 5 payload files"
        status: pass
    human_judgment: false

duration: 55min
completed: 2026-09-02
status: complete
---

# Phase 3 Gap-Closure (03-08): CR-01 negation-blindness fix Summary

**PI053/PI056/PI057 no longer misread a prohibition ("Never run with --dangerously-skip-permissions...") as the directive it forbids, via a lookaround-free line-initial anchor — with zero detection regression on the pinned 12/12 CAT-01 / 70/72 library recall counts, and a self-caught duplicate-finding bug from the first draft of the fix.**

## Performance

- **Duration:** 55 min
- **Tasks:** 8/8 completed (the numbered items in `<required_work>`)
- **Files modified:** 11
- **Commits:** 7

## Accomplishments

- **CR-01 fixed.** All four review-reproduced sentences (`Never run with --dangerously-skip-permissions...`, `Do not run with --yolo in CI.`, `Do not disable the security scanner, you will be held responsible...`, `Do not update your settings.json to no longer require approval.`) now produce **zero findings** against the release binary, verified directly (`injection-scanner check ... --strict`), not just inferred from unit tests.
- **No detection regression.** `tests/recall_test.rs` — the exact pinned counts (CAT-01 12/12: 7/7 prose + 5/5 structural; library 70/72) — passes unchanged. Every prior positive (YAML `example` fields, `tests/pattern_test.rs` cases) still fires.
- **Self-caught regression during the fix itself.** The first draft's anchor, `(?:^|[.!?]|\n)\s*`, duplicated findings in `multiline.rs`'s cross-line paragraph-join pass whenever an attack sentence sat on its own physical line immediately after another complete sentence (exactly the corpus's own shape) — the punctuation branch consumed the *preceding* sentence's period into the match span, making it look like it crossed a line break. Caught by manually re-scanning `tests/corpus/attack/tool-permission-abuse.md` (14 matches instead of the expected 10) rather than by any existing automated gate — none of `corpus_test`, `recall_test`, `pattern_test`, or the GATE-05 tests are sensitive to duplicate-but-correct findings. Fixed by simplifying the anchor to `^\s*` (line-initial only, zero-width, cannot cross a line boundary), which cost one casualty: `examples/tool-permission-abuse-attack.md`'s hard-wrapped illustrative paragraph had to be reflowed one-sentence-per-line, since a directive whose own words are split across a line break is no longer reachable by design (reaching across would reintroduce the duplicate).
- **Negative-polarity clean-corpus specimen added.** `tests/corpus/clean/security-policy-prohibitions.md` — a CONTRIBUTING.md-style security policy containing the CR-01 sentences plus additional realistic prohibition variants for all three patterns, addressed to a human contributor. Scores zero findings under `--strict`, and (via the existing `relaxed_pattern`/`counter_example` mutation-pairing mechanism already proven for these three sentences) is exactly the class of specimen the relaxed pattern set is proven to catch a relaxation against.
- **GATE-05/D-09 honoured.** All three patterns' `relaxed_pattern` now targets the negation guard specifically (the new narrowing this plan added): `counter_example` is a CR-01 negated sentence, `relaxed_pattern` is the pre-fix regex with the anchor removed. `tests/pattern_relaxed_control_test.rs` and `tests/pattern_policy_test.rs` are green.
- **Both-polarity unit coverage added** for PI053, PI056 and PI057 in `tests/pattern_test.rs` — the four CR-01 sentences plus additional variants (PI053 arm 2, PI056's "Never edit...", PI057's "Never turn off...") as `assert_negatives`, alongside the pre-existing `assert_positives`.
- **GATE-03 sweep re-run** over the same pinned 22-directory list. Raw output committed (`03-08-gate03-manifest.tsv`/`summary.tsv`) before analysis, per the resilience protocol. **Zero third-party delta in both directions** — no new false positive, and the one confirmed true positive from Plan 07 (`17-skills-registry/.claude/settings.local.json:3`, PI051 CRITICAL) is unchanged. The −16 finding delta within this repository's own scan is entirely self-scan artifacts (changed `counter_example` literals, new negative test-case strings, the reflowed examples file) — documented in `03-SWEEP.md`'s new section.
- **WR-01 and WR-02 fixed.** `PATTERNS.md`'s Categories table now lists `Tool & Permission Abuse | PI050-PI059 | HIGH (CRITICAL structural override)`. `tests/corpus/attack/structural/README.md`'s payload table now documents all 5 structural attack-corpus files (was 1 of 5).
- **Catalogue and baseline regenerated** (`docs/PATTERN-CATALOGUE.md`, `.github/code-scanning-baseline.json`) per the pattern-library skill, reflecting the changed `pattern`/`counter_example`/`relaxed_pattern` fields.

## Task Commits

Each numbered item in `<required_work>` was committed atomically:

1. **Add negation guard to PI053/PI056/PI057** — `bac75c0` (fix)
2. **Both-polarity test cases + catalogue/baseline regen** — `8d1b37c` (test)
3. **Negative-polarity clean-corpus specimen** — `d908420` (test)
4. **WR-01/WR-02 documentation fixes** — `5726cb1` (docs)
5. **Self-caught duplicate-finding bug fix (anchor simplification)** — `d979a35` (fix)
6. **Raw GATE-03 sweep output, committed before analysis** — `32a66bd` (chore)
7. **GATE-03 sweep delta analysis** — `6fa43a7` (docs)

(No separate commit was needed for GATE-05/D-09 compliance or the recall re-measurement — both were verified as part of commits 1-2 above, and required no additional code changes beyond what those commits already made.)

## Files Created/Modified

- `patterns/core/tool-permission-abuse.yaml` — negation guard (line-initial anchor) added to PI053/PI056/PI057's `pattern`; `counter_example`/`relaxed_pattern` re-targeted to the negation narrowing; comments rewritten in PI057's existing style, including a full account of the duplicate-finding bug and why `^` alone (not `(?:^|[.!?]|\n)`) is correct.
- `tests/pattern_test.rs` — `assert_negatives` gained the CR-01 reproductions plus additional prohibition variants for PI053, PI056, PI057.
- `tests/corpus/clean/security-policy-prohibitions.md` (new) — negative-polarity clean specimen.
- `examples/tool-permission-abuse-attack.md` — reflowed the PI053-PI057 illustrative paragraph one-sentence-per-line (required by the line-initial-anchor fix), with a note explaining why.
- `PATTERNS.md` — added the missing Tool & Permission Abuse row to the Categories table (WR-01).
- `tests/corpus/attack/structural/README.md` — added rows for the 4 missing structural payload files (WR-02).
- `docs/PATTERN-CATALOGUE.md`, `.github/code-scanning-baseline.json` — regenerated per the pattern-library skill.
- `.planning/phases/03-tool-permission-abuse-cat-01-33/03-SWEEP.md` — new "Gap-closure sweep (03-08, CR-01)" section.
- `.planning/phases/03-tool-permission-abuse-cat-01-33/03-08-gate03-manifest.tsv`, `03-08-gate03-summary.tsv` (new) — raw sweep output, committed before analysis.

## Decisions Made

- **Line-initial anchor over other lookaround-free options.** Considered (per the task's own framing): (a) sentence/clause-initial anchoring, (b) consuming the sentence prefix and rejecting a preceding negation token, (c) an explicit alternation that cannot match after a negator. Option (b) is not expressible in the Rust `regex` crate without lookaround for PI056's shape specifically — the negator ("Do not") sits *before* the directive verb, which itself sits *before* the settings mention, so "reject if a negation token appears between sentence-start and the trigger" cannot be built from a bounded character-class gap the way the existing PI05x patterns build their object-window gaps. Option (a) (sentence/clause-initial anchoring of the directive verb, or of `you` for PI057's arm 2) works because every required positive already phrases its directive as the first thing on its own line, and a negator always occupies that leading slot instead — verified by hand for all three patterns before implementing.
- **`^\s*` instead of `(?:^|[.!?]|\n)\s*`.** The richer anchor (matching sentence-internal and cross-line boundaries too, not just the start of a line) seemed like a strict superset improvement at first, but it introduced a genuine duplicate-finding bug in the `multiline.rs` cross-line pass (see Accomplishments above) that none of the existing test suite caught — only a manual re-scan of the attack corpus surfaced it. `^\s*` gives up reaching a directive whose own words are split across a hard-wrapped line break, which is absent from every pinned positive and was only present in this milestone's own illustrative `examples/` file (fixed by reflowing that file, not by weakening the anchor).
- **The negation guard became the GATE-05 narrowing under test for all three patterns**, replacing the prior `counter_example`/`relaxed_pattern` pairing (which tested each pattern's *original* Plan 06/07 narrowing — directive framing for PI053, the settings-widening object for PI056, second-person address for PI057). The schema (`tests/pattern_relaxed_control_test.rs`) supports exactly one `relaxed_pattern`/`counter_example` pair per pattern id, and the negation guard is the more safety-critical property to keep gate-enforced going forward; the original narrowings remain enforced in `pattern` itself and are still covered by manual `assert_negatives` cases in `tests/pattern_test.rs`, just no longer by the automated mutation-pairing gate.
- **Corpus-safety reasoning by construction, not just by testing.** Every `pattern` change in this plan is a strict prefix-narrowing of the previous body (the anchor/verb-gap is prepended in front of the unchanged rest of the regex), which makes each new pattern's match set mathematically a subset of the old pattern's match set — so it cannot introduce a new clean-corpus match, only remove one. This was verified as a design property before implementation, then confirmed empirically by the test suite.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Anchor design introduced a duplicate cross-line finding**
- **Found during:** Self-verification after committing the first fix (before this task's Item 6 sweep)
- **Issue:** The initial anchor `(?:^|[.!?]|\n)\s*` consumed a preceding sentence's terminating punctuation into the match, which — because `multiline.rs`'s paragraph-join pass joins consecutive lines with a single space — made the match appear to span a line break when it did not conceptually do so, causing the cross-line pass to report a duplicate of a finding the per-line pass had already reported standalone. Confirmed via a manual scan of `tests/corpus/attack/tool-permission-abuse.md` (14 matches instead of 10) and via `no_new_pattern_escapes_the_attack_corpus`, which failed once the anchor was simplified to `^`-only without also reflowing `examples/tool-permission-abuse-attack.md` (a hard-wrapped sentence in that file became unreachable).
- **Fix:** Simplified the anchor to `^\s*` (zero-width, cannot consume a preceding line's content) and reflowed `examples/tool-permission-abuse-attack.md`'s illustrative paragraph one-sentence-per-line.
- **Files modified:** `patterns/core/tool-permission-abuse.yaml`, `examples/tool-permission-abuse-attack.md`
- **Verification:** Manual re-scan of `tests/corpus/attack/tool-permission-abuse.md` confirmed 10 matches (no duplicates); full `cargo test --locked` green including `no_new_pattern_escapes_the_attack_corpus`.
- **Commit:** `d979a35`

---

**Total deviations:** 1 auto-fixed (Rule 1 — bug introduced and caught within this same plan, before it could reach the sweep or a reviewer).
**Impact on plan:** No scope creep — the fix is a narrowing of the same anchor concept already planned, not a new mechanism. Caught by the executor's own manual verification discipline (re-scanning the attack corpus directly with the release binary) rather than by any existing automated gate, which is itself worth noting: none of `corpus_test`, `recall_test`, `pattern_test`, or the GATE-05 tests check for duplicate-but-otherwise-correct findings, so a future PI05x pattern using a similar anchor construction should re-run this same manual check.

## Issues Encountered

None beyond the self-caught deviation above.

## Deferred Items (explicitly left open, per instructions)

- **WR-03** (`scripts/gate03-sweep.sh`'s `sweep_one`/`build_summary` missing `local` declarations) — not touched. Still open.
- **IN-01** (PI050/PI051 pre-wildcard asymmetry in how much may precede the wildcard inside a tool call) — not touched. Still open.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

CR-01 is closed: PI053/PI056/PI057 no longer misclassify prohibitions as directives, with recall and clean-corpus properties unchanged and GATE-05 still enforcing the (now negation-specific) narrowing for all three. WR-01 and WR-02 are closed. WR-03 and IN-01 remain open, as instructed, for a future plan. This gap-closure plan does not mark any new `REQUIREMENTS.md` entries complete — CAT-01 and GATE-05 were already marked complete by prior plans in this phase, and this plan is a fix to a code-review finding against that already-completed work, not new scope.

## Self-Check: PASSED

- `patterns/core/tool-permission-abuse.yaml` — FOUND, contains updated PI053/PI056/PI057.
- `tests/corpus/clean/security-policy-prohibitions.md` — FOUND.
- `.planning/phases/03-tool-permission-abuse-cat-01-33/03-08-gate03-manifest.tsv` — FOUND.
- `.planning/phases/03-tool-permission-abuse-cat-01-33/03-08-gate03-summary.tsv` — FOUND.
- Commits `bac75c0`, `8d1b37c`, `d908420`, `5726cb1`, `d979a35`, `32a66bd`, `6fa43a7` — all FOUND in `git log --oneline`.
- `cargo test --locked` — all green (full run, no failures) as of the final commit.
- `cargo fmt --all -- --check` — clean.
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- Four CR-01 reproduction sentences — verified zero findings against the release binary.
- `tests/recall_test.rs` — CAT-01 12/12, library 70/72, unchanged.

---
*Phase: 03-tool-permission-abuse-cat-01-33*
*Completed: 2026-09-02*
