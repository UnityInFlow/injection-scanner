---
phase: quick-260915-spt
plan: 01
subsystem: pattern-library
tags: [frontmatter, jsonc, config-parsing, injection-scanner, mcp-json, diagnostics]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    provides: PI060-PI069 (CAT-02 MCP & tool-description poisoning), the frontmatter structural engine (ENG-01) that this fix extends
provides:
  - "config_parse_error: Option<String> on ScanReport — additive, skip_serializing_if-absent, carried through --baseline"
  - "warning: structural config pass skipped for <file> — <error> — unconditional stderr diagnostic, print-suppressed only for .jsonl/.ndjson"
  - "frontmatter::relax_jsonc — a byte-offset and line-count preserving JSONC comment/trailing-comma preprocessor, string-literal safe"
  - "extract() recognises a leading-comment JSONC header (first non-whitespace byte '/') instead of silently returning Ok(None)"
  - ADR-005 recording the diagnostic + tolerance decision and the alternatives rejected (warning-only, json5 crate, span-preserving parser, naive non-string-aware strip)
affects: [pattern-library, spec-ci-plugin (consumes --format json reports; config_parse_error is additive and does not move the pinned key set)]

actuals:
  tokens: 13214
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "Byte-offset/line-count preserving preprocessor as a parse fallback: try strict first, relax and retry only on failure, so a well-formed document pays nothing and every reported line number stays valid against the original text"
    - "String-literal-aware forward-pass state machine (Outside/InString/StringEscape/LineComment/BlockComment) for text relaxation, so a comment marker inside a string survives verbatim"
    - "Library records facts, CLI decides what to print: Scanner attaches config_parse_error to ScanReport unconditionally; main.rs owns the stderr warning and the .jsonl/.ndjson print-suppression policy"

key-files:
  created:
    - docs/adr/ADR-005-jsonc-tolerant-config-parsing.md
  modified:
    - src/pattern.rs
    - src/scanner.rs
    - src/baseline.rs
    - src/main.rs
    - src/frontmatter.rs
    - tests/scan_resilience_test.rs
    - tests/json_contract_test.rs
    - tests/report_roundtrip_test.rs
    - tests/frontmatter_test.rs
    - README.md

key-decisions:
  - "Shipped both the diagnostic (Task 1) and JSONC tolerance (Task 2), per the plan's locked decision_record — a warning alone would only annotate the detection gap the issue's `detection-gap` label named, not close it"
  - "config_parse_error uses skip_serializing_if = \"Option::is_none\" rather than a bare #[serde(default)], so the --format json key set tests/json_contract_test.rs pins stays byte-for-byte unchanged for every report that parsed cleanly"
  - "relax_jsonc is tried only as a fallback after a strict serde_json parse fails, and the RELAXED parser's error is what gets reported (not the strict one) — because offsets are preserved, its line/column still point at the right place in the original file"
  - ".jsonl/.ndjson files are exempted from the printed warning (never from the underlying record) via a documented main.rs helper — they are streams of independent documents, never one config file"
  - "Self-scan gate run with --exclude '.planning/**', matching .github/workflows/code-scanning.yml exactly, rather than the plan's Task 3 prose literal (which omits the flag and would false-fail on pre-existing, already-baselined .planning/ documentation) — see Deviations"

requirements-completed: [ISSUE-129]

duration: ~25min (commit span, plan already authored)
completed: 2026-09-15
status: complete
---

# Quick Task 260915-spt: Fix #129 — JSONC-commented config silently skipped

**A `//`/`/* */`-commented `mcp.json` (VS Code / GitHub Copilot IntelliJ house style) is now parsed and scanned instead of silently producing zero findings, and any config block that still cannot be parsed prints a visible stderr warning that survives `--quiet` and `--baseline`.**

## Performance

- **Duration:** ~25 min (commit span: 20:50 plan → 21:12 final commit)
- **Tasks:** 3/3
- **Files modified:** 11 (10 modified + 1 created), across 3 commits

## Accomplishments

- Closed the reported **silence**: `ScanReport` carries a new `config_parse_error: Option<String>` field; `Scanner::scan_with_confidence`'s fourth (structural) pass records `analyze()`'s `Err` arm instead of discarding it via a bare `if let`; `Baseline::apply` carries the field across its report rebuild; `main.rs` prints an unconditional `warning: structural config pass skipped for <file> — <error>` on stderr, before `--write-baseline` handling and outside `if !quiet`.
- Closed the reported **detection gap**: `frontmatter::relax_jsonc` tolerates `//`/`/* */` comments and trailing commas on the `ConfigSyntax::Json` path — a byte-offset and line-count preserving, string-literal-safe preprocessor tried only as a fallback after a strict parse fails. A JSONC-commented `mcp.json` now produces the `PI060`/`PI061`/`PI062` findings it would produce without the comments.
- `extract()` now recognises a document whose first non-whitespace byte is `/` (a leading comment before the opening `{`) instead of silently returning `Ok(None)` — which was even quieter than the reported bug.
- `.jsonl`/`.ndjson` files are exempted from the printed warning (never from the recorded fact) because they are streams of independent documents, never one config file.
- 22 new tests (10 in Task 1, 12 in Task 2) pin every `must_haves.truths` line item in the plan; all pass, and Task 1's tests pass unchanged after Task 2 landed.
- ADR-005 records the decision and the four rejected alternatives; README documents the new `config_parse_error` field and the JSONC tolerance.

## Task Commits

Each task was committed atomically:

1. **Task 1: Carry the config parse failure out of the scan and print it — end to end** - `a6f2f37` (fix)
2. **Task 2: JSONC tolerance on the ConfigSyntax::Json path — offset-preserving, string-safe** - `72f1cc7` (feat)
3. **Task 3: ADR, docs, and the full-suite + self-scan gate** - `4cddc99` (docs)

_No separate TDD-cycle commits — tests and implementation for each task were written together and committed once per task, consistent with this repo's existing quick-task commit pattern (each task's tests were run RED before implementation was written, per-task, as described under Deviations)._

## Files Created/Modified

- `src/pattern.rs` - `ScanReport.config_parse_error: Option<String>` (additive, `skip_serializing_if`), `with_config_parse_error` chainable setter
- `src/scanner.rs` - Fourth pass now matches exhaustively on `analyze()`'s `Ok(Some)`/`Ok(None)`/`Err`; the `Err` arm is recorded instead of discarded
- `src/baseline.rs` - `Baseline::apply` carries `config_parse_error` across its `with_baselined` report rebuild
- `src/main.rs` - Unconditional stderr warning loop (before `--write-baseline` handling); `is_line_delimited_stream` helper suppresses it for `.jsonl`/`.ndjson`
- `src/frontmatter.rs` - `relax_jsonc` (comment + trailing-comma blanking, string-literal safe), `parse()`'s `Json` arm falls back to it on strict-parse failure, `extract()` recognises a leading-comment JSONC header
- `tests/scan_resilience_test.rs` - 6 new CLI-level tests (Task 1) + 1 end-to-end JSONC/PI061 test (Task 2)
- `tests/json_contract_test.rs` - 1 new test: `config_parse_error` present and a string on a broken-config report
- `tests/report_roundtrip_test.rs` - 3 new tests: round-trip, key-omitted-when-None, legacy-report-still-loads
- `tests/frontmatter_test.rs` - 11 new unit tests covering comments, trailing commas, string-literal safety, the byte-length/line-count invariant, the leading-comment header case, and the still-an-error-after-relaxation case
- `README.md` - Documents `config_parse_error` in the JSON output section and JSONC tolerance in "What gets scanned"
- `docs/adr/ADR-005-jsonc-tolerant-config-parsing.md` - New. Records the decision and rejected alternatives

## Decisions Made

- Shipped both halves (diagnostic + tolerance) per the plan's locked `decision_record` — not revisited.
- `config_parse_error` placed as the last field before the severity counts on `ScanReport`, defaulted to `None` in `with_baselined` (the only struct-literal constructor), attached via a chainable `with_config_parse_error` rather than a sixth positional constructor parameter.
- `relax_jsonc`'s comment-blanking never touches `\n` (even inside a block comment), which is what keeps the byte-length AND line-count invariant — a multi-line block comment collapses to blank lines, never to nothing.
- The RELAXED parser's error text is what gets surfaced on a still-broken document, not the strict parser's — offsets are preserved through relaxation, so the relaxed error's line/column point at the same place in the original file, whereas the strict error would point at the comment.

## Deviations from Plan

### Auto-fixed / clarified during execution

**1. [Rule 3 - Blocking] Task 3's self-scan verification command needed `--exclude '.planning/**'` to match the actual CI gate**
- **Found during:** Task 3 (the self-scan gate)
- **Issue:** The plan's Task 3 prose gives the literal command
  `cargo run --release -- check . --baseline .github/code-scanning-baseline.json`
  (no `--exclude`). Run exactly as written, it exits 1 with 35 findings — all
  pre-existing, already-accepted-by-baseline-adjacent `.planning/` documentation
  (deferred-items.md, SWEEP.md, etc.), none related to this change. The task's own
  framing — "run the self-scan the code-scanning workflow runs" — points at
  `.github/workflows/code-scanning.yml`, whose actual invocation includes
  `--exclude '.planning/**'` (that repo file's own comment explains `.planning/**`
  is deliberately excluded because it is planning prose, not shipped/agent-facing
  surface, and would be pure baseline churn otherwise).
- **Fix:** Ran the scan with `--exclude '.planning/**'` added, matching the workflow
  file byte-for-byte. Exits 0, "No injection patterns reported.", 380 findings
  accepted by the unmodified baseline.
- **Files modified:** None (verification-only; no source change).
- **Verification:** `target/release/injection-scanner check . --exclude '.planning/**' --baseline .github/code-scanning-baseline.json` → exit 0.
- **Committed in:** N/A (verification step, documented in the Task 3 commit message).

**2. [Rule 2 - Missing critical] Task 1 shipped one extra test beyond the plan's nine**
- **Found during:** Task 1 (writing the RED tests)
- **Issue:** The plan's `must_haves.truths` distinguishes "absent when clean" and
  "present when broken" as two halves of one contract, but the plan's Task 1 test
  list (Tests 1-9) only pins the "present when broken" half in
  `tests/json_contract_test.rs` (Test 8) and never explicitly pins "the key is
  entirely absent (not null) on a clean report" as its own assertion.
- **Fix:** Added `a_report_with_no_config_parse_error_omits_the_key_entirely` in
  `tests/report_roundtrip_test.rs`, asserting the serialized JSON string does not
  contain `config_parse_error` at all when the field is `None`.
- **Files modified:** `tests/report_roundtrip_test.rs`.
- **Verification:** `cargo test --test report_roundtrip_test` — 10 passed, 0 failed.
- **Committed in:** `a6f2f37` (Task 1 commit).

---

**Total deviations:** 2 (1 verification-command clarification, 1 additional test for symmetry with the plan's own stated contract). No scope creep — both stay inside Task 1/Task 3's stated intent.

## Issues Encountered

None beyond the deviations above. `cargo fmt` reformatted several of the new test/doc blocks after each task's initial write (long assertion lines, one struct-literal-chain call); re-verified with `cargo fmt --check` clean and all tests re-run green after each reformat.

## Test Results

- **Task 1 targeted:** `cargo test --test scan_resilience_test --test json_contract_test --test report_roundtrip_test --test cli_test` → 14 + 6 + 10 + 14 = all passed, 0 failed (cli_test unaffected, included as the plan specified).
- **Task 2 targeted:** `cargo test --test frontmatter_test --test scan_resilience_test --test perf_regression_test` → 41 + 15 + 2 = all passed, 0 failed; all of Task 1's tests confirmed still passing unchanged.
- **Task 3 full suite (backgrounded, polled, log at `/tmp/it-129-test.log`):** `cargo test` → exit=0, every `test result:` line `ok`, **428 tests passed, 0 failed** across all 37 test binaries + doctests (up from the 406 recorded in STATE.md before this task — the +22 delta matches the 10 + 12 new tests across the two feature commits exactly).
- **clippy:** `cargo clippy --all-targets -- -D warnings` clean at every commit.
- **fmt:** `cargo fmt --check` clean at every commit (after one `cargo fmt` pass per task to absorb reformatting of newly-written lines).
- **Self-scan** (matching `.github/workflows/code-scanning.yml` exactly): `injection-scanner check . --exclude '.planning/**' --baseline .github/code-scanning-baseline.json` → exit 0, "No injection patterns reported.", 380 findings accepted by the unmodified baseline. `git diff --name-only origin/main` confirms nothing under `patterns/`, `docs/PATTERN-CATALOGUE.md`, or `.github/code-scanning-baseline.json` was touched.

### Manual reproduction (plan `<verification>` item 3)

Fixtures written to the session scratchpad (not `/tmp` directly, and not committed — the plan's own execution rules forbid new payload-carrying fixtures under `tests/fixtures/`, and this repo scans itself):

```
$ injection-scanner check .../mcp.json          # // house-style comment, then a real url
.../mcp.json
  :6 MEDIUM  An MCP server endpoint is reached over plaintext HTTP, ...  (PI061)  [frontmatter (structural) · confidence 1.0]
1 finding(s): 0 critical, 0 high, 1 medium, 0 low
$ echo $?
1                                                # findings exist — correct, not a coverage gap

$ injection-scanner check .../broken.json       # { "servers": { "x": } }
warning: structural config pass skipped for .../broken.json — invalid JSON document: expected value at line 1 column 21
No injection patterns detected.
$ echo $?
0                                                # parse failure is not a finding — exit code unchanged
```

Both match the plan's stated expectations exactly.

## Known Stubs

None.

## Threat Flags

None. The threat model's own register (T-129-01 through T-129-06, T-129-SC) is fully addressed by the implementation: string-literal-aware blanking (T-129-01, tests `a_url_value_containing_double_slash_survives_relaxation_verbatim` and `a_value_containing_a_block_comment_marker_survives_relaxation_verbatim`), single-pass O(n) relaxation with no backtracking (T-129-02, `perf_regression_test` included in Task 2's verify), tolerance limited to comments + trailing commas rather than the wider `json5` grammar (T-129-03), `.jsonl`/`.ndjson` print suppression against alarm fatigue (T-129-05, `a_line_delimited_json_stream_produces_no_config_parse_warning`), `--baseline` carry-through (T-129-06, `baseline_does_not_erase_the_config_parse_warning`), and no new dependency was added (T-129-SC).

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Issue #129 is closed by this task's three commits; no follow-on work identified within its scope.
- The `config_parse_error` field and `relax_jsonc` are both narrowly scoped and reversible per the plan's `reversibility` ratings — future work is not blocked on this.
- Not investigated as part of this task: whether other syntaxes (`json5` files specifically, which `DEFAULT_EXTENSIONS` also lists) have their own comment-tolerance gaps — `.json5`'s own native comment support was not audited here, since `ConfigSyntax` only distinguishes Yaml/Toml/Json and a `.json5`-extension file is still routed through the `Json` syntax arm this task already fixed.

## Self-Check: PASSED

All 11 claimed source/test/doc files and the SUMMARY.md itself confirmed present on
disk; all 3 claimed commit hashes (`a6f2f37`, `72f1cc7`, `4cddc99`) confirmed present
in `git log`. No missing items.

---
*Phase: quick-260915-spt*
*Completed: 2026-09-15*
