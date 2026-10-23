---
phase: 04-mcp-tool-description-poisoning-cat-02-34
plan: 01
subsystem: testing
tags: [gate03, gate05, recall-harness, structural-corpus, relaxed-pattern, mcp]

requires:
  - phase: 03-tool-permission-abuse-cat-01-33
    provides: the flat structural corpus, the PI050+ relaxed_pattern field, GATE-05's mutation-control mechanism
provides:
  - a committed pre-edit GATE-03 sweep baseline (32 directories, 23,738 files, 584 findings) that every later Phase 4 plan's --compare run measures against
  - a per-category structural corpus collector in tests/recall_test.rs with a two-sided pinned-row/pinned-directory guard
  - an open-ended (PI050-and-above) relaxed_pattern ratchet in tests/pattern_policy_test.rs with a positive-control table test
affects: [04-02, 04-03, 04-04, 04-05, 04-06, 04-07]

actuals:
  tokens: 2129534   # chars/4 over the full realized diff (b4f05ef..HEAD); ~2.1M of this is the
                     # 32 committed sweep-evidence JSON files (machine-generated scan output, not
                     # authored content) — the authored diff (code/tests/docs/tsv/panic-doc,
                     # excluding those JSON blobs) is 53,530 chars / ~13,383 tokens on the same scale.
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "Per-category structural corpus subdirectories, row name derived by suffixing the directory name rather than restated as a literal"
    - "Open-ended id-range predicate (extracted, unit-tested) instead of an inlined closed range for a ratchet meant to apply to every future category"

key-files:
  created:
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-baseline-2026-09-03/ (32 JSON reports, manifest.tsv, summary.tsv, panic-cursor-extensions.txt)
  modified:
    - tests/recall_test.rs
    - tests/pattern_policy_test.rs
    - tests/corpus/attack/structural/README.md
    - tests/corpus/attack/structural/tool-permission-abuse/*.md (git mv, zero content change)

key-decisions:
  - "Excluded ~/.claude root and both ~/.cursor/extensions and ~/.vscode/extensions from the GATE-03 sweep — the former is fully represented by three already-swept subdirectories, the latter two are vendored third-party bundle trees (same noise class as node_modules) and the direct cause of a reproduced production panic"
  - "Narrowed ~/.cursor and ~/.vscode to their config-relevant subdirectories via scratch mirrors outside the repo, rather than modifying scripts/gate03-sweep.sh to tolerate a non-0/1/2 scanner exit"
  - "Structural corpus row names are derived (directory name + '-structural' suffix) rather than hand-written, so a directory rename cannot silently rename a pinned recall row"
  - "relaxed_pattern's PI050+ requirement is an open-ended predicate (>= 50), not the previous closed 50-59 range, so CAT-03 (#35) inherits it without another edit to this file"

requirements-completed: [CAT-02, GATE-02, GATE-03, GATE-05]

coverage:
  - id: D1
    description: "Pre-edit GATE-03 baseline captured from the unmodified, pre-CAT-02 release binary (b4f05ef, 56-pattern set) — 32 directories, 23,738 files, 584 findings, zero skipped-missing; whole-repo self-scan shows only the two known standing PATTERN-CATALOGUE.md self-matches; cargo test --locked at 353 tests, matching STATE.md"
    requirement: "GATE-03"
    verification:
      - kind: unit
        ref: "shell: manifest.tsv/summary.tsv non-empty, *.json count == swept-row count, git diff --quiet HEAD -- src patterns Cargo.toml Cargo.lock tests"
        status: pass
    human_judgment: false
  - id: D2
    description: "Structural attack corpus generalised from one flat CAT-01-only directory to one subdirectory per category (tool-permission-abuse/, CAT-01's five payloads moved via git mv with zero content change); recall_test.rs's collector, GATE-02 exact-pin, duplicate-check and frontmatter-parse tests all walk every category; two-sided guard (pinned row needs a directory, directory needs a pinned row) proven by two performed mutation checks"
    requirement: "GATE-02"
    verification:
      - kind: unit
        ref: "cargo test --test recall_test (7/7 pass, tool-permission-abuse 7/7, tool-permission-abuse-structural 5/5 — both unchanged from before this plan)"
        status: pass
    human_judgment: false
  - id: D3
    description: "relaxed_pattern's GATE-05 requirement fixed to actually be open-ended (id >= 50) instead of a closed 50-59 range that would have exempted every CAT-02/PI060+ pattern from the mutation-tested false-positive control; proven with a positive-control table test independent of any shipped pattern"
    requirement: "GATE-05"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_policy_test (5/5 pass, including requires_relaxed_pattern_covers_every_category_boundary)"
        status: pass
    human_judgment: false
  - id: D4
    description: "A real, reproduced production panic was found while capturing the GATE-03 baseline: scanning ~/.cursor/extensions crashes the release binary at src/frontmatter.rs:219 (assertion failed: self.is_char_boundary(new_len)) — same failure shape as the ENG-02 tail[..12] panic, on real third-party bytes. Documented (sweep-baseline-2026-09-03/panic-cursor-extensions.txt, 04-SWEEP.md, WINDOWS.md) but deliberately not fixed here — Task 1 required zero source diff."
    verification: []
    human_judgment: true
    rationale: "This is a live crash bug discovered mid-baseline-capture, not a deliverable this plan built. It needs a human decision on filing a follow-up issue and prioritizing the fix (src/frontmatter.rs's scalar-value truncation is exactly the code path CAT-02's own structural half will exercise most, in plans 04-04/04-05/04-06), which is outside a single executor's authority to decide unprompted."

duration: unrecorded_precise_start
completed: 2026-09-03
status: complete
---

# Phase 04 Plan 01: GATE-03 baseline + structural corpus + relaxed_pattern ratchet fix Summary

**Captured the pre-edit GATE-03 sweep baseline every later Phase 4 plan compares against, generalised the structural attack corpus from one flat CAT-01-only directory to one subdirectory per category, and fixed a GATE-05 ratchet that would have silently exempted every CAT-02 pattern from its mutation-tested false-positive control — while discovering (and documenting, not fixing) a real production panic on real third-party bytes along the way.**

## Performance

- **Duration:** commit span ~13 minutes (10:53–11:06); total session time including research review, sweep-directory investigation and the panic bisection was longer — precise start timestamp was not captured at session start
- **Tasks:** 3/3 completed
- **Files modified:** 44 (32 sweep-evidence JSON reports + manifest.tsv + summary.tsv + panic-cursor-extensions.txt + 04-SWEEP.md, 5 renamed corpus payloads, tests/corpus/attack/structural/README.md, tests/recall_test.rs, tests/pattern_policy_test.rs)

## Accomplishments

- **Pre-edit GATE-03 baseline** (`sweep-baseline-2026-09-03/`, `04-SWEEP.md`): swept the shipping 56-pattern release binary (git SHA `b4f05ef`) over the Phase 3 directory list plus every D-06/Q5 MCP-specific source measured as real on this machine — 32 directories, 23,738 files, 584 findings, zero directories missing. Whole-repo self-scan: only the two known, accepted `docs/PATTERN-CATALOGUE.md` self-matches (`PI001`@:74, `PI031`@:903), nothing new. `cargo test --locked`: 353 tests, matching `STATE.md`'s recorded baseline — confirms the tree was genuinely unmodified at capture time.
- **Structural corpus generalised** (`tests/recall_test.rs`, `tests/corpus/attack/structural/`): CAT-01's five payloads moved via `git mv` into `tool-permission-abuse/` (verified zero-content-change renames). The collector now walks one subdirectory per category, deriving each row's name by suffixing the directory name rather than hand-writing it — a rename mutation check and an unpinned-directory mutation check both performed and confirmed to FAIL loudly (see below), then restored.
- **GATE-05 ratchet fixed** (`tests/pattern_policy_test.rs`): the `relaxed_pattern` requirement's id-range check was a closed `50..=59` range while its own adjacent comment claimed open-ended inheritance by CAT-02/CAT-03 — false, and undetected because the check had been vacuous (zero PI050+ patterns existed) since it was written. Replaced with an extracted, open-ended `requires_relaxed_pattern(id)` predicate plus a positive-control table test that does not depend on any pattern shipping. Threshold-mutation check performed and confirmed to FAIL, then restored.

## Task Commits

1. **Task 1: Capture the pre-edit GATE-03 baseline and the pre-edit self-scan** — `fcba330` (chore)
2. **Task 2: Give the structural attack corpus a second category without dropping the first** — `d20b879` (refactor)
3. **Task 3: Make the relaxed_pattern ratchet actually cover PI060-PI069** — `1979469` (fix)

_No plan-metadata commit was made separately from this SUMMARY's own commit (below) — `commit_docs`/`.planning` gitignore status not checked ahead of time; see the final-commit step._

## Files Created/Modified

- `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md` — the recorded directory list, counts, self-scan baseline, test count, and the discovered panic
- `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-baseline-2026-09-03/` — 32 per-directory JSON reports, `manifest.tsv`, `summary.tsv`, `panic-cursor-extensions.txt`
- `tests/recall_test.rs` — `structural_categories()` (replacing `structural_payloads()`), `assert_pinned_row()` shared helper, two-sided guard in `the_structural_corpus_is_actually_collected`, every structural-aware test now walks all categories
- `tests/corpus/attack/structural/tool-permission-abuse/*.md` — CAT-01's five payloads, moved unchanged
- `tests/corpus/attack/structural/README.md` — rewritten for the subdirectory layout; WR-02 named explicitly as still open
- `tests/pattern_policy_test.rs` — `requires_relaxed_pattern(id)` predicate, `requires_relaxed_pattern_covers_every_category_boundary` table test, corrected section comment
- `.planning/WINDOWS.md` — created; two entries recorded (the discovered panic, and WR-02's carry-forward)

## Decisions Made

- **Directory list for the sweep** was assembled by measurement, not by copying every `find`-discovered manifest location verbatim: `~/.claude/plugins/marketplaces` was added as the task explicitly named it; other real MCP-manifest-holding directories (Codex, Cursor, VS Code — both workspace and global —, Kiro, Warp, GitHub Copilot IntelliJ, and the two non-ecosystem sibling repos) were added per 04-RESEARCH.md §Q5's explicit recommendation. `~/.claude` root (holding a single `claude_desktop_config.json`) was deliberately excluded as redundant with three already-swept subdirectories.
- **~/.cursor/extensions and ~/.vscode/extensions excluded from the sweep.** Sweeping `~/.cursor` whole reproduced a real panic (`src/frontmatter.rs:219`, `assertion failed: self.is_char_boundary(new_len)`) — the same failure shape as the ENG-02 `tail[..12]` panic, on real third-party vendored extension bundle files. Since `scripts/gate03-sweep.sh` treats any non-`0`/`1`/`2` scanner exit as fatal to the entire run (by design), and Task 1 forbids any source-code change, both directories were narrowed to their genuinely config-relevant subdirectories (copied into scratch mirrors outside the repo) rather than working around the crash by modifying the scanner or the sweep script.
- **The panic is documented, not fixed.** Recorded in `sweep-baseline-2026-09-03/panic-cursor-extensions.txt`, `04-SWEEP.md`, and `.planning/WINDOWS.md` (kind: todo). This is flagged for human attention in the coverage table (D4) because CAT-02's own structural patterns (plans 04-04/04-05/04-06) will exercise the exact same code path (`frontmatter.rs`'s scalar-value rendering) most heavily of anything in this milestone.
- **relaxed_pattern predicate is `>= 50`, open-ended**, matching ADR-004's literal wording, rather than re-deriving a new closed range for CAT-02 (`50..=69`) that would just repeat the same class of mistake for CAT-03 later.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking, worked around without a source change] Reproduced production panic blocked completing the sweep over `~/.cursor`/`~/.vscode`**
- **Found during:** Task 1
- **Issue:** `scripts/gate03-sweep.sh` aborts the entire sweep run on any non-`0`/`1`/`2` scanner exit; scanning `~/.cursor/extensions` (real, unmodified third-party files) panics the release binary at `src/frontmatter.rs:219`.
- **Fix:** Did not modify `src/` or the sweep script (forbidden by Task 1's own constraint). Instead excluded both directories' `extensions/` subtrees from the sweep, substituting their config-relevant subdirectories (copied into scratch mirrors outside the repo, verified not to panic before being folded into the full run).
- **Files modified:** none in `src/`; only the evidence directory and `04-SWEEP.md` document the decision.
- **Verification:** `git diff --stat HEAD -- src patterns Cargo.toml Cargo.lock tests` empty; full sweep completed with exit 0, all 32 rows `swept`.
- **Committed in:** `fcba330` (Task 1 commit)

**2. [Rule 3 - Blocking] Phase 3's recovered directory list used a stale sibling-repo path**
- **Found during:** Task 1
- **Issue:** The `git show`-recovered Phase 3 manifest lists sibling repos as `.../workspace-1-ideas/01-spec-linter` etc. (missing the `unity-in-flow-ai/` path segment); the first sweep attempt reported all 20 sibling repos `skipped-missing`.
- **Fix:** Corrected the paths to include `unity-in-flow-ai/` and re-ran the sweep.
- **Files modified:** none in `src/`; corrected before the committed sweep.
- **Verification:** Second sweep run shows all 20 sibling-repo rows `swept` with the same file/finding counts as Phase 3's original baseline.
- **Committed in:** `fcba330` (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (both Rule 3, both worked around without touching `src/`, matching Task 1's hard constraint)
**Impact on plan:** Both necessary to produce a complete, honest baseline. No scope creep — no pattern, engine, or corpus-payload work was added.

## Issues Encountered

A real production panic (`src/frontmatter.rs:219`, `assertion failed: self.is_char_boundary(new_len)`) was discovered while capturing the baseline — see "Decisions Made" and coverage item D4 above. Not an issue with this plan's own work; a pre-existing bug this plan's measurement happened to surface. Recorded, not resolved.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- `sweep-baseline-2026-09-03/` is ready to be the fixed reference point for every later Phase 4 plan's `scripts/gate03-sweep.sh --compare` run (04-04, 04-05, 04-06, 04-07). **It must not be regenerated** once a CAT-02 pattern exists.
- The structural corpus can now hold CAT-02's `mcpServers` structural half as its own subdirectory (e.g. `tests/corpus/attack/structural/mcp-tool-poisoning/`) without touching CAT-01's pinned row.
- `relaxed_pattern`'s GATE-05 requirement is now provably enforced for every `PI060`-`PI069` pattern plan 04-04/04-05/04-06 will author.
- **Concern for whoever picks up 04-04/04-05/04-06:** the discovered `src/frontmatter.rs:219` panic sits directly in the code path CAT-02's structural patterns exercise. Worth checking whether any CAT-02 corpus payload or clean specimen (especially anything with a long single-line string value) risks triggering the same class of crash before those plans' own GATE-03 sweeps run.

---
*Phase: 04-mcp-tool-description-poisoning-cat-02-34*
*Completed: 2026-09-03*

## Self-Check: PASSED

All claimed files exist (`04-01-SUMMARY.md`, `04-SWEEP.md`, `sweep-baseline-2026-09-03/`, the moved
`tool-permission-abuse/01-wildcard-allowed-tools-block-sequence.md`, `.planning/WINDOWS.md`) and all
three task commit hashes (`fcba330`, `d20b879`, `1979469`) are present in git log.
