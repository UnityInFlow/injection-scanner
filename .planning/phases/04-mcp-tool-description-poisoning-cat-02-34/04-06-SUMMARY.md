---
phase: 04-mcp-tool-description-poisoning-cat-02-34
plan: 06
subsystem: patterns
tags: [cat-02, mcp, tool-description-poisoning, cross-tool-shadowing, tool-override, rug-pull, gate03, gate05, negation-handling, false-positive-re-narrowing]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 05
    provides: PI063-PI065 (the second-person description-poisoning arms), the D-01 third-person blind spot this plan's PI066 closes, and the sweep-after-04-05-2026-09-07 pre-edit baseline this plan reuses
provides:
  - PI066 cross-tool-shadowing (MEDIUM, prose) -- fires on a description that names a DIFFERENT tool's invocation (or output) as the trigger for a directive, in the third person, without requiring second-person address; closes D-01's accepted blind spot
  - PI067 tool-override-directive (MEDIUM, prose) -- fires on a description that removes the reader's choice between two named tools (instead-of-using/never-use/always-instead), distinct from ordinary recommendation
  - PI068 version-conditional-directive (MEDIUM, prose) -- fires on a version-gated directive consequent; the first of the two rug-pull-language arms
  - PI069 deferred-activation-directive (MEDIUM, prose) -- fires on date-, approval- or call-count-gated directive consequents; completes the rug-pull-language pair with PI068
  - CAT-02's full planned ten-pattern set (PI060-PI069), 71 total patterns in the repository
  - the rug-pull bound stated in the pattern file's own header comment -- PI068/PI069 detect version/date/approval/call-count-conditional LANGUAGE only, never a mitigation for a server that republishes a poisoned description after a gating condition is met
  - a re-narrowing precedent for PI066/PI067's first drafted arms, which keyed only on trigger words and produced 26 real false positives (1 PI066, 25 PI067) in the GATE-03 sweep before being fixed with a tool-shaped-object requirement
  - re-pinned recall (102/109, 93.6%; mcp-tool-poisoning prose sub-row 4/4, 100%), the regenerated catalogue and code-scanning baseline, and three new README behaviour-change callouts
  - a fully adjudicated two-directional GATE-03 sweep isolating this plan's delta against sweep-after-04-05-2026-09-07 (zero in both directions) plus a continuity comparison against the stale 04-01 baseline (69 total lines, all pre-existing/already-adjudicated)
affects: [04-07]

actuals:
  tokens: 17700
  tasks: 3
  commits: 4

tech-stack:
  added: []
  patterns:
    - "Tool-shaped-object discriminator: require a backtick-quoted code span, a snake_case identifier, or an identifier immediately followed by `()` as the object of a verb, to separate a real tool reference from an ordinary English noun in heuristic prose patterns (PI066 Arm A, all three PI067 arms)"
    - "Directive-vs-fact narrowing: require the consequent of a conditional trigger to be an enumerated directive verb (skip/bypass/ignore/disable/proceed/stop asking/no longer ask/automatically-X), never a statement of fact about the software, to separate an attack payload from ordinary compatibility/rate-limit/deprecation documentation (PI068, PI069)"
    - "GATE-05 counter_example selection: when the discriminator is directive-vs-fact, the counter_example must keep the identical antecedent trigger and differ ONLY in the consequent, or the relaxed_pattern mutation control cannot prove the narrowing is load-bearing"

key-files:
  created:
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-after-04-06-2026-09-07/ (redacted GATE-03 sweep output: manifest.tsv, summary.tsv, checksums.sha256, RAW-REPORTS.md)
  modified:
    - patterns/core/mcp-tool-poisoning.yaml (PI066-PI069 added; PI066/PI067 re-narrowed; rug-pull bound added to header comment)
    - tests/pattern_test.rs (test_pi066_cross_tool_shadowing, test_pi067_tool_override_directive, test_pi068_version_conditional_directive, test_pi069_deferred_activation_directive; total pattern count 67 -> 71)
    - tests/recall_test.rs (mcp-tool-poisoning prose row 3/4 -> 4/4)
    - examples/mcp-tool-poisoning-attack.md (PI066-PI069 worked payloads)
    - README.md (category/pattern counts, recall table, three behaviour-change callouts)
    - docs/PATTERN-CATALOGUE.md (regenerated)
    - .github/code-scanning-baseline.json (regenerated)
    - tests/corpus/documentation/mcp-tool-poisoning-writeup.md (blockquote -> fenced code block, confidence downgrade fix; see Deviations)
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md (new "Phase 4 Plan 06" section)

key-decisions:
  - "PI066 is deliberately person-agnostic: the discriminator is that the sentence's subject is another tool's invocation, not second-person address, closing D-01's accepted blind spot rather than restating it"
  - "PI067's discriminator is choice-removal (always/never/instead-of), not recommendation vocabulary (prefer/over), specifically to survive the harder direction of tests/corpus/clean/mcp-companion-tools.md's boundary specimen"
  - "PI068 and PI069 detect version/date/approval/call-count-conditional LANGUAGE, explicitly not a rug-pull mitigation -- stated in the pattern file's own header comment, not only in this plan record, because a single scan cannot prove absence of a future republish"
  - "PI066/PI067's first drafted arms (trigger-words-only) were re-narrowed to require a tool-shaped object (backtick/snake_case/paren-suffixed) after the GATE-03 sweep found 26 real false positives -- ordinary imperative style-guide prose, not tool-substitution content"
  - "PI067 Arm B requires 'always' literally paired with its own verb+object, never a bare trailing 'instead', because a bare-word trigger let a same-clause backtick-quoted term (e.g. a TypeScript style guide's 'never use `any`... use `unknown`... instead') slip past the object requirement alone"
  - "Stale plan-authored pattern-count targets (63/65/66) were corrected to the real tree's counts (68/70/71) at each task, same deviation class 04-05's own SUMMARY already documented, per the plan's stated intent that the count move by exactly the number of patterns added"

requirements-completed: [CAT-02, GATE-02, GATE-03, GATE-05]

coverage:
  - id: D1
    description: "PI066 cross-tool-shadowing detects the documented third-person Elastic payload shape without requiring second-person address"
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "tests/pattern_test.rs#test_pi066_cross_tool_shadowing"
        status: pass
      - kind: integration
        ref: "cargo test --test corpus_test (no_new_pattern_escapes_the_attack_corpus, the_clean_corpus_reports_nothing)"
        status: pass
    human_judgment: false
  - id: D2
    description: "PI067 tool-override-directive distinguishes substitution from recommendation, surviving the companion-tools clean-corpus boundary specimen"
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "tests/pattern_test.rs#test_pi067_tool_override_directive"
        status: pass
      - kind: integration
        ref: "cargo test --test pattern_relaxed_control_test (shipped_pattern_misses_counter_example_but_relaxed_form_catches_it)"
        status: pass
    human_judgment: false
  - id: D3
    description: "PI068/PI069 detect version-, date-, approval- and call-count-conditional directive language, with the rug-pull bound stated in the pattern file header"
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "tests/pattern_test.rs#test_pi068_version_conditional_directive, tests/pattern_test.rs#test_pi069_deferred_activation_directive"
        status: pass
    human_judgment: false
  - id: D4
    description: "The GATE-03 two-directional sweep isolates this plan's delta against the reused sweep-after-04-05-2026-09-07 baseline (zero both directions) and reproduces 04-05's own continuity numbers against the stale 04-01 baseline"
    requirement: "GATE-03"
    verification:
      - kind: other
        ref: "scripts/gate03-sweep.sh --compare (four runs, recorded verbatim in 04-SWEEP.md's Phase 4 Plan 06 section)"
        status: pass
    human_judgment: true
    rationale: "The sweep's adjudication (which findings are pre-existing vs. attributable to this plan) required manual inspection of matched file content, not a scripted pass/fail; recorded here for the record, but the underlying comparison commands and their empty/non-empty results are independently reproducible."
  - id: D5
    description: "GATE-05 mutation-pairing control proves each new pattern's discriminator is load-bearing, not accidental"
    requirement: "GATE-05"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_relaxed_control_test (all four tests)"
        status: pass
    human_judgment: false

duration: 6h 40min
completed: 2026-09-07
status: complete
---

# Phase 4 Plan 06: CAT-02's Four Heuristic Arms Summary

**PI066-PI069 close out the ten-pattern CAT-02 category — cross-tool shadowing, tool-override, and the two rug-pull-language markers — after a GATE-03 sweep over 23,770 real third-party files caught and fixed a genuine over-widening in the first two arms.**

## Performance

- **Duration:** 6h 40min (including a stalled predecessor's ~50 minutes of uncommitted, judged-and-mostly-kept draft work for Task 1)
- **Started:** 2026-09-07 (predecessor session) / resumed same day
- **Completed:** 2026-09-07T19:35:00Z (approx.)
- **Tasks:** 3
- **Files modified:** 9 (plus one new sweep-output directory)

## Accomplishments

- PI066 cross-tool-shadowing: detects the third-person Elastic-quoted payload shape D-01's second-person discriminator cannot reach, with a mutation-tested negation guard against a prohibition sentence
- PI067 tool-override-directive: detects substitution ("instead of using X, always Y") as distinct from recommendation, surviving the harder direction of the companion-tools clean-corpus specimen
- PI068 version-conditional-directive and PI069 deferred-activation-directive: detect version-, date-, approval- and call-count-conditional directive language, with an explicit, header-comment-level statement that this is language detection only, never a rug-pull mitigation
- A real false-positive class caught by the GATE-03 sweep (26 hits: 1 PI066, 25 PI067 — ordinary imperative style-guide prose) was root-caused and fixed with a tool-shaped-object requirement, then the entire sweep was re-run clean
- Recall re-measured and re-pinned: 102/109 (93.6%), mcp-tool-poisoning's prose sub-row now 4/4 (100%)

## Task Commits

Each task was committed atomically, plus one fix commit discovered mid-Task-3:

1. **Task 1: PI066 cross-tool shadowing** - `3b57934` (feat)
2. **Task 2: PI067 tool override and PI068 version-conditional behaviour** - `06c7c21` (feat)
3. **Task 3: PI069 deferred activation, GATE-03 delta** - `1c0f80c` (feat)
4. **Re-narrowing fix found by Task 3's own sweep** - `a91c5e2` (fix)

_Note: commit 4 exists because Task 3's acceptance criteria require running the GATE-03 sweep on the finished tree and adjudicating every finding — the sweep itself surfaced a real defect in Tasks 1-2's shipped patterns, which was fixed and re-verified before the plan could close._

## Files Created/Modified

- `patterns/core/mcp-tool-poisoning.yaml` - PI066-PI069 added; PI066/PI067 re-narrowed after the sweep; rug-pull bound stated in the header comment
- `tests/pattern_test.rs` - four new test functions; total pattern count assertion 67 → 71
- `tests/recall_test.rs` - mcp-tool-poisoning prose row 3/4 → 4/4
- `examples/mcp-tool-poisoning-attack.md` - worked payloads for all four new patterns
- `README.md` - category/pattern counts, recall table, three behaviour-change callouts (two of which needed a follow-up fix after they self-triggered PI066/PI067 on their own quoted examples — see Deviations)
- `docs/PATTERN-CATALOGUE.md`, `.github/code-scanning-baseline.json` - regenerated after every pattern change
- `tests/corpus/documentation/mcp-tool-poisoning-writeup.md` - a pre-existing blockquote container downgraded to a fenced code block once PI066 became the first pattern to reach that quotation (see Deviations)
- `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md` - new "Phase 4 Plan 06" section with the full sweep, false-positive root-cause, and four-way adjudication
- `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-after-04-06-2026-09-07/` - redacted sweep output (raw JSON kept locally gitignored under `.planning/local/`)

## Decisions Made

See `key-decisions` in the frontmatter. The two most consequential: (1) PI066/PI067's re-narrowing to require a tool-shaped object (backtick/snake_case/paren-suffixed), which is what separates a real cross-tool/substitution directive from ordinary "never do X, always do Y" technical writing; (2) stating the rug-pull bound in the pattern file's own header comment rather than only in this plan record, so a future reader of the pattern source sees the limitation directly.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `tests/corpus/documentation/mcp-tool-poisoning-writeup.md`'s Elastic quotation sat in a low-signal-but-not-actually-low-confidence blockquote container**
- **Found during:** Task 1, while judging the predecessor's uncommitted draft
- **Issue:** The file's own documented contract (`tests/corpus_test.rs`'s docstring) requires quoted attack payloads in `tests/corpus/documentation/` to report ZERO findings at the default confidence threshold but be recoverable under `--strict` — proving context-awareness is load-bearing, not merely that the corpus was never exercised. The quotation sat in a `>` blockquote, whose `MatchContext::BlockQuote` carries confidence 0.9 (effectively prose-equivalent, above `DEFAULT_MIN_CONFIDENCE` 0.5). No prior pattern had reached inside that blockquote, so the defect was latent until PI066 became the first pattern whose grammar matched the exact quoted sentence.
- **Fix:** Moved the quotation from `> "..."` into a fenced ` ```text ` block (confidence 0.2, below the default threshold, still non-zero under `--strict`). Verified by evidence, not assumption: reverting the container change and re-running `cargo test --test corpus_test` reproduces `the_documentation_corpus_reports_nothing` FAILING with `mcp-tool-poisoning-writeup.md: 1 finding(s)`; restoring the fenced form and re-running returns it to green.
- **Files modified:** `tests/corpus/documentation/mcp-tool-poisoning-writeup.md`
- **Verification:** `cargo test --test corpus_test` (5/5 passing), with the mutation-and-revert check performed and reported above
- **Committed in:** `3b57934` (Task 1 commit)

**2. [Rule 1 - Bug] Two README behaviour-change callouts self-triggered the very patterns they were documenting**
- **Found during:** Task 1 and Task 2's whole-repo self-scan checks
- **Issue:** Quoting the attack sentences verbatim inside a `>` blockquote in `README.md` (once for PI066, once for PI067) made those quotations match their own new patterns — the same failure mode `docs/DETECTION-BACKLOG.md` already exhibits for other patterns, per the pattern-library skill's own documented history.
- **Fix:** Rewrote both callouts to point at the worked payload in `examples/mcp-tool-poisoning-attack.md` instead of re-quoting the attack sentence directly. Re-verified the whole-repo self-scan (`injection-scanner check . --exclude '.planning/**'`, findings outside `examples/`, `patterns/`, `tests/`, `tools/`) returns byte-identical output to the pre-edit baseline after each fix.
- **Files modified:** `README.md`
- **Verification:** Whole-repo self-scan re-run after each change, confirmed identical to `04-SWEEP.md`'s baseline
- **Committed in:** `3b57934` (Task 1), `06c7c21` (Task 2)

**3. [Rule 1 - Bug] PI066 and PI067's first drafted arms produced 26 real false positives in the GATE-03 sweep**
- **Found during:** Task 3's mandatory two-directional sweep over ~23,900 real third-party files
- **Issue:** Both patterns' original arms keyed only on trigger vocabulary (never/always/instead-of; when+calls/invokes/uses/runs) with no requirement on what was actually being used, called or invoked — a grammar common throughout ordinary technical-writing style guides, not specific to naming two tools. Representative real hits: `"Never use category names as scope IDs — always use the GUID."`, `"Never use \`any\` type, use \`unknown\` or generics instead"`, `"ALWAYS use a navigation stack title instead of a custom text element"`.
- **Fix:** Both patterns now require an explicit tool-shaped object (backtick-quoted code span, snake_case identifier, or identifier immediately followed by `()`) directly after the relevant verb. PI067's Arm B additionally requires "always" literally paired with its own verb+object, closing a residual gap where a bare trailing "instead" let a same-clause backtick-quoted term slip through the object check alone.
- **Files modified:** `patterns/core/mcp-tool-poisoning.yaml`
- **Verification:** Full re-run of `tests/pattern_test.rs`, `corpus_test`, `pattern_relaxed_control_test`, `cargo test --locked`, and the GATE-03 sweep — all green, sweep clean of all four CAT-02 heuristic patterns
- **Committed in:** `a91c5e2` (fix commit, discovered mid-Task-3)

**4. [Rule 3 - Blocking] Stale plan-authored pattern-count targets**
- **Found during:** Every task
- **Issue:** The plan's stated total-pattern targets (63/65/66) were written before plan 04-05 shipped, leaving them one generation stale — the same deviation class 04-05's own SUMMARY already documented for its own Task 1/2 targets.
- **Fix:** `tests/pattern_test.rs`'s `test_total_pattern_count` corrected to the real tree's counts at each task (68, 70, 71). The plan's stated intent — the count moves by exactly the number of patterns added — is honoured; only the literal numbers are corrected.
- **Files modified:** `tests/pattern_test.rs`
- **Verification:** `cargo test --test pattern_test test_total_pattern_count`
- **Committed in:** `3b57934`, `06c7c21`, `1c0f80c`

---

**Total deviations:** 4 auto-fixed (3 Rule 1 - Bug, 1 Rule 3 - Blocking)
**Impact on plan:** All four were necessary for correctness (a corpus contract violation, two self-scan failures, and a real false-positive class in the shipped patterns) or to keep a test assertion honest against the real tree. No scope creep — the false-positive fix in particular is exactly the outcome Task 3's own acceptance criteria anticipated and required.

## Issues Encountered

None beyond the deviations above. The predecessor executor's ~50-minute uncommitted draft for Task 1 (PI066) was treated as unverified input per the continuation brief: every claim in it was independently re-derived from evidence (regex behaviour tested against Python `re` before and after committing to the Rust `regex` crate escaping, the mutation test re-run and its exact failure message re-captured, the self-scan re-verified) rather than trusted, and the one genuinely load-bearing judgment call in it (moving the corpus specimen's container from blockquote to fenced code) was kept only after proving by reversion that it was necessary, with its inline justification moved out of the corpus file and into the commit message per the continuation brief's explicit instruction.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- CAT-02's full ten-pattern set (PI060-PI069) is shipped, tested, and swept clean against ~23,900 real third-party files
- Plan 04-07 (final reconciliation) can compare against `sweep-after-04-06-2026-09-07/` as its own pre-edit baseline, following the same reuse pattern this plan followed for 04-05's
- D-05 (structural cross-reference for cross-tool shadowing — verifying a referenced tool actually exists in the manifest) remains explicitly deferred to its own future issue, as locked by D-04/D-05 in `04-CONTEXT.md`
- No blockers

---
*Phase: 04-mcp-tool-description-poisoning-cat-02-34*
*Completed: 2026-09-07*

## Self-Check: PASSED

All key files verified present on disk (`patterns/core/mcp-tool-poisoning.yaml`, `tests/pattern_test.rs`, `tests/recall_test.rs`, `examples/mcp-tool-poisoning-attack.md`, the sweep output directory) and all four commit hashes (`3b57934`, `06c7c21`, `1c0f80c`, `a91c5e2`) confirmed present in `git log`.
