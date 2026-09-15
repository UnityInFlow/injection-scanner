---
phase: quick-260915-spt
plan: 01
type: execute
wave: 1
depends_on: []
autonomous: true
requirements: [ISSUE-129]
files_modified:
  - src/frontmatter.rs
  - src/scanner.rs
  - src/pattern.rs
  - src/baseline.rs
  - src/main.rs
  - tests/frontmatter_test.rs
  - tests/scan_resilience_test.rs
  - tests/json_contract_test.rs
  - tests/report_roundtrip_test.rs
  - README.md
  - docs/adr/ADR-005-jsonc-tolerant-config-parsing.md

estimate:
  tokens: 90000
  raw_tokens: 60000
  tasks: 3
  confidence: low

must_haves:
  truths:
    - "A config block that cannot be parsed produces a `warning:` on stderr naming the file and the parse error — the silence in #129 is gone."
    - "The warning is emitted even under `--quiet`: a coverage gap is not silenceable by a formatting flag."
    - "The warning does NOT change the exit code. A parse failure is not a finding; `spec-ci-plugin` keys on exit codes."
    - "A `.jsonl`/`.ndjson` file — a scanned type that is a line-delimited stream, never a single config document — produces no such warning."
    - "A `//`-commented `mcp.json` in the VS Code / Copilot-IntelliJ house style is parsed, projected, and produces the PI060/PI061/PI062 findings it would produce without the comments."
    - "A `//` or `/*` sequence INSIDE a JSON string literal survives verbatim — `\"url\": \"http://metrics.internal.example.com/mcp\"` still projects its full value and still fires PI061."
    - "A JSONC trailing comma before `}` or `]` is tolerated."
    - "A config file whose first non-whitespace content is a comment, with `{` after it, is still recognised as a config block instead of vanishing as `Ok(None)`."
    - "Relaxation is byte-offset and line-count preserving, so every reported line number is unchanged from the strict path."
    - "A file that is still unparseable AFTER relaxation still produces the visible warning."
    - "`--baseline` does not erase the diagnostic: the rebuilt report still carries the parse error."
  artifacts:
    - src/frontmatter.rs
    - src/scanner.rs
    - src/pattern.rs
    - src/baseline.rs
    - src/main.rs
    - tests/frontmatter_test.rs
    - tests/scan_resilience_test.rs
    - docs/adr/ADR-005-jsonc-tolerant-config-parsing.md
  key_links:
    - "`frontmatter::analyze` Err -> `Scanner::scan_with_confidence` (src/scanner.rs:562, the swallow site) -> `ScanReport::config_parse_error` -> `main.rs` eprintln. Break any link and the bug returns."
    - "`Baseline::apply` (src/baseline.rs:270) rebuilds the report via `with_baselined` — it must carry `config_parse_error` across, or `--baseline` silently restores the #129 behaviour."
    - "`relax_jsonc` byte-length invariant -> `frontmatter::locate` line arithmetic -> `ProjectedLine.line` -> every reported line number."
    - "`ScanReport` key set -> tests/json_contract_test.rs pinned assertions -> `spec-ci-plugin`'s `JSON.parse(output) as Array<...>`."
---

<objective>
Close GitHub issue #129: a JSONC-commented `mcp.json` / `.mcp.json` / `claude_desktop_config.json`
is rejected by `serde_json::from_str`, `analyze()` returns `Err`, the structural pass is skipped
under the FIX-03 skip-do-not-abort rule, and the scan reports zero matches with **no diagnostic on
stdout or stderr**. Nothing distinguishes "this file is clean" from "this file could not be read".

Purpose: two separate failures live in that one sentence. The **silence** is the reported bug — a
skipped pass that looks identical to a clean result is worse than no scan at all. The **gap** is
that a real, currently-published host family's config file (the VS Code / GitHub Copilot IntelliJ
`mcp.json` house style, which ships `//` comments inside otherwise-valid JSON) is invisible to
PI060/PI061/PI062, which is why the issue carries the `detection-gap` label. Fixing only the first
leaves a live detection gap behind a warning label.

Output: a JSONC-tolerant fallback on the `ConfigSyntax::Json` parse path, a `config_parse_error`
carried out of the scan on `ScanReport` and printed by `main.rs`, and tests pinning both — plus the
one case a naive implementation gets wrong (comment markers inside string literals).
</objective>

<decision_record>
## Scope decision: ship BOTH the diagnostic and the JSONC-tolerant parse

The issue offers a minimum (warn) and a fuller fix (tolerate). This plan ships both, with the
diagnostic first.

**Why not the warning alone.** A warning converts an invisible miss into a visible miss. The file
in the reproduction is not malformed — it is the documented house style of a shipping host, and the
scanner's own walker already lists `jsonc` and `json5` in `DEFAULT_EXTENSIONS` (src/walk.rs:79-80).
The tool therefore already claims to scan a file class its parser cannot read. Warning about it
every time is not closing the gap; it is annotating it.

**Why a comment-blanking preprocessor and NOT the `json5` crate.** `json5` changes what counts as
valid JSON far past comments — unquoted keys, trailing commas, single-quoted strings, hex numbers,
leading `+` — which widens the accepted grammar of every config file this scanner reads, and adds a
dependency (which is itself an ADR-triggering change under the pr-artifacts skill). A preprocessor
that **overwrites comment bytes with spaces** is strictly narrower and has one property that matters
more than its size: it preserves byte offsets and line breaks, so `locate()`'s `block.body.lines()`
arithmetic, `block.start_line`, and every reported line number stay exactly as they are on the
strict path. `ProjectedLine` does not change. No new dependency, no new grammar.

**Trailing commas are in scope.** "JSONC" as VS Code defines it is comments *and* trailing commas,
and its own config files routinely carry both. Half the tolerance leaves the named host family's
real files unscanned, which is the thing this issue is about. The marginal code is one extra pass
over the already-string-aware walker, and it gets its own test.

**A leading comment before `{` is in scope.** `extract()` requires the trimmed content to start with
`{` (src/frontmatter.rs:123). A JSONC file that opens with a `// header` comment therefore returns
`Ok(None)` — "no configuration here" — which is *even quieter* than the reported bug: not even the
Task 1 warning would fire, because no block was ever found. The guard stays O(1) for ordinary files:
the tolerant path is attempted only when the first non-whitespace byte is `/`.

**The diagnostic ships regardless and must survive the tolerance.** Anything still unparseable after
relaxation — a genuine syntax error, a truncated file — must still produce the visible `warning:`.
That is the actual reported bug and Task 1's tests must still pass unchanged after Task 2.

## Where the diagnostic is printed, and why not in the library

`scanner.rs` cannot print: the library never writes to stdout or stderr, and `main.rs` owns every
user-facing `warning:` line (src/main.rs:390, :477, :508). The parse failure is therefore carried
**out** of the scan as data on `ScanReport` and printed by the CLI.

## JSON contract: additive, and absent unless it happened

`config_parse_error: Option<String>` carries `#[serde(default, skip_serializing_if = "Option::is_none")]`.
`skip_serializing_if` rather than a bare `default` is deliberate: `tests/json_contract_test.rs`
pins the report key set **exactly**, and the failure it guards is a field leaking into
`--format json` for every consumer on every file. With `skip_serializing_if` the key set
`spec-ci-plugin` sees today is byte-for-byte unchanged on every report that parsed, and the key
appears only on the rare report that did not. Both halves get pinned by tests (absent when clean,
present when broken) so the conditional shape is a contract, not an accident.

## Knowingly accepted, NOT an oversight

`locate()` searches the **raw** `block.body`, comments included. A commented-out `"url": ...` line
sitting above the real one can therefore attract the reported line number. `locate` is documented
best-effort ("a slightly wrong line is far better than a finding that cannot be located at all",
src/frontmatter.rs:90-94) and the finding still points at the right region. Out of scope; recorded
here so a reviewer reads it as a decision.
</decision_record>

<execution_context>
@$HOME/.claude/gsd-core/workflows/execute-plan.md
@$HOME/.claude/gsd-core/templates/summary.md
</execution_context>

<context>
@.planning/STATE.md
@CLAUDE.md
@.claude/skills/code-review/SKILL.md
@.claude/skills/pr-artifacts/SKILL.md

@src/frontmatter.rs
@src/scanner.rs
@src/pattern.rs
@src/main.rs
@src/baseline.rs
</context>

<execution_rules>
**Branch.** Work on the already-checked-out `fix/129-jsonc-config-diagnostic` (forked from
origin/main at `f0b316b`). Do NOT create another branch. Verify with `git branch --show-current`
before the first edit.

**Do not touch `patterns/`.** This is an engine fix, not a pattern change. No pattern YAML is
edited, so `docs/PATTERN-CATALOGUE.md` and `.github/code-scanning-baseline.json` are NOT
regenerated and the pattern-library skill's catalogue loop does not apply. If you find yourself
editing a file under `patterns/`, stop — the plan is wrong and needs revisiting.

**New CLI fixtures go in a temp dir, never in `tests/fixtures/`.** A committed fixture carrying a
live `http://metrics.internal.example.com/mcp` payload adds a finding to this repo's own self-scan
and would force a `.github/code-scanning-baseline.json` regeneration — which the line above puts
out of scope. Use the `temp_dir()` / `unique_suffix()` helpers already at the top of
`tests/scan_resilience_test.rs`. Unit-level fixtures embedded as Rust string literals in
`tests/*.rs` are fine: `rs` is not in `DEFAULT_EXTENSIONS`.

**Long-running commands.** The full `cargo test` suite takes ~5 minutes and has previously tripped
the 600-second subagent watchdog. Every full-suite run MUST be backgrounded to a log file and
polled — use the Bash tool's `run_in_background: true`, or
`nohup cargo test > /tmp/injscan-129-test.log 2>&1 &`, then poll with
`tail -5 /tmp/injscan-129-test.log`. Never run the full suite in the foreground. Targeted runs
(`cargo test --test frontmatter_test`, `cargo test --test scan_resilience_test`) are fast and are
fine in the foreground.

**Rust constraints (CLAUDE.md + the code-review skill, non-negotiable).** No `unwrap()` — the crate
carries `#![deny(clippy::unwrap_used)]`. No `println!` in library or debug paths; user-facing
diagnostics are `eprintln!` from `main.rs` only. `///` rustdoc on every new public item. Match
exhaustively — no catch-all `_` arm. `cargo fmt` and `cargo clippy --all-targets -- -D warnings`
clean before each commit.

**Commit per task**, conventional style, referencing the issue: e.g.
`fix(frontmatter): surface a config parse failure instead of skipping it silently (#129)`.
</execution_rules>

<tasks>

<task type="tracer" tdd="true">
  <name>Task 1: Carry the config parse failure out of the scan and print it — end to end</name>
  <precondition>The working tree is on branch `fix/129-jsonc-config-diagnostic`; `git branch --show-current` confirms it before any edit.</precondition>
  <files>src/scanner.rs, src/pattern.rs, src/baseline.rs, src/main.rs, tests/scan_resilience_test.rs, tests/json_contract_test.rs, tests/report_roundtrip_test.rs</files>
  <read_first>src/scanner.rs:548-600 (the fourth structural pass and the swallow site at :562), src/pattern.rs:212-370 (ScanReport and its four constructors), src/baseline.rs:243-271 (the report rebuild), src/main.rs:415-515 (report collection) and :560-600 (output + the unconditional `skipped` warning), tests/scan_resilience_test.rs:1-45 (the temp_dir/unique_suffix helpers), tests/json_contract_test.rs:74-100 (the pinned report key set).</read_first>
  <behavior>
    Write these tests FIRST and watch them fail, then implement.

    In `tests/scan_resilience_test.rs` (temp-dir fixtures, real binary via `binary_path()`):
    - Test 1 — the reported bug: a file `broken.json` containing `{ "servers": { "x": } }` (invalid
      under any tolerance) scanned with `check <dir>` prints a stderr line containing
      `structural config pass skipped`, the file name, and text from the parse error. Assert the
      line is on **stderr**, not stdout.
    - Test 2 — exit code unchanged: that same scan exits 0. A parse failure is not a finding.
    - Test 3 — not silenceable: the same scan with `--quiet` still prints the warning.
    - Test 4 — no noise on line-delimited streams: a `stream.jsonl` whose first line is
      `{"event":"a"}` and second line is `{"event":"b"}` produces NO
      `structural config pass skipped` line, on stdout or stderr.
    - Test 5 — `--baseline` does not erase it: the broken.json scan run with
      `--baseline <an empty-entries baseline file written by --write-baseline>` still prints the
      warning. This is the `src/baseline.rs:270` rebuild link.
    - Test 6 — a clean, well-formed `.mcp.json` with no findings prints no warning at all.

    In `tests/json_contract_test.rs`:
    - Test 7 — the pinned report key set for a parseable file is UNCHANGED (the existing
      `format_json_report_key_set_is_exactly_pinned` must still pass untouched; do not edit it).
    - Test 8 — for a file with a broken config block, `--format json` carries a
      `config_parse_error` string key on that report object, and the top level is still an array.

    In `tests/report_roundtrip_test.rs`:
    - Test 9 — a report carrying `config_parse_error: Some(..)` survives a JSON round-trip, and a
      report JSON written WITHOUT the key still deserializes (the `#[serde(default)]` guarantee the
      module header already explains for `suppressed`).
  </behavior>
  <action>
    Add `config_parse_error: Option<String>` to `ScanReport` (src/pattern.rs) as the last field
    before the severity counts, carrying `#[serde(default, skip_serializing_if = "Option::is_none")]`
    and a `///` doc comment that states: why it exists (a skipped structural pass must not look like
    a clean file), that it is additive so the top-level JSON stays an array of report objects for
    `spec-ci-plugin`, and that it is absent rather than null when nothing failed so the pinned key
    set does not move for consumers. Default it to `None` in `with_baselined` (the only constructor
    that builds the struct literal) and add a chainable
    `pub fn with_config_parse_error(mut self, error: Option<String>) -> Self` so the scanner can
    attach it without a fifth positional constructor.

    In `src/scanner.rs`, replace the `} else if let Ok(Some((_, projected))) = analyze(content) {`
    swallow at :562 with an exhaustive `match analyze(content)` over `Ok(Some(..))` / `Ok(None)` /
    `Err(e)`, binding the error into a `config_parse_error: Option<String>` declared before the
    pass. `Ok(None)` is the ordinary "no configuration here" case and stays silent. Attach the error
    to the report built at :599 via `.with_config_parse_error(config_parse_error)`. Extend the
    existing block comment above the pass to record that skip-do-not-abort (FIX-03) does not mean
    skip-silently, and that the `if let` which discarded the `Err` is what issue #129 reported.

    In `src/baseline.rs`, take `config_parse_error` off the report before the `with_baselined`
    rebuild at :270 and restore it onto the new value, with a one-line comment naming why: the
    rebuild is a fresh struct, so a field not carried across is a field `--baseline` silently drops.

    In `src/main.rs`, after the report-collection block and BEFORE the `--write-baseline` handling
    (so both output paths are covered, and so the reassuring "wrote N entries" is never the last
    word over a file the scanner could not read), loop over `reports` and for each
    `Some(error)` emit exactly:
    `eprintln!("warning: structural config pass skipped for {file} — {error}")`.
    Emit it unconditionally, NOT inside an `if !quiet` — matching the existing unconditional
    coverage-gap warning at :606, and for the same reason. Skip the emission for line-delimited
    stream extensions via a small documented private helper (case-insensitive `jsonl` / `ndjson`):
    those are in `DEFAULT_EXTENSIONS` but are a sequence of documents, never one config file, so a
    per-file warning on them would be pure noise and would train users to filter the very line this
    task exists to add. The error is still recorded on the report either way — the library records
    facts, the CLI decides what to print.

    Exit codes must not move: the diagnostic is not a finding and touches neither the tallies nor
    the `fail_on` walk.
  </action>
  <verify>
    <automated>cargo test --test scan_resilience_test --test json_contract_test --test report_roundtrip_test --test cli_test</automated>
  </verify>
  <done>All nine tests above pass; the existing `format_json_report_key_set_is_exactly_pinned` passes unedited; `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` are clean; committed.</done>
  <reversibility rating="costly">Adding a field to `ScanReport` touches the JSON contract `spec-ci-plugin` consumes. Additive and conditionally serialized, so removing it later is safe — but a consumer may start keying on it, which is why the shape is pinned by tests now.</reversibility>
</task>

<task type="auto" tdd="true">
  <name>Task 2: JSONC tolerance on the ConfigSyntax::Json path — offset-preserving, string-safe</name>
  <files>src/frontmatter.rs, tests/frontmatter_test.rs, tests/scan_resilience_test.rs</files>
  <read_first>src/frontmatter.rs in full — particularly `extract` (:115-133), `parse` (:165-174), `locate` (:258-279) and the `ProjectedLine.line` doc comment (:89-95), which is what the byte-offset invariant protects. tests/frontmatter_test.rs:1-60 for the `rendered()` helper and the naming convention. patterns/core/mcp-tool-poisoning.yaml PI060/PI061/PI062 for realistic payload shapes.</read_first>
  <behavior>
    Write these tests FIRST and watch them fail, then implement.

    In `tests/frontmatter_test.rs`:
    - Test 1 — line comments: a document with a `//` comment line above `"servers"` parses and
      projects the same lines as the identical document without the comment.
    - Test 2 — trailing comment on a value line: `"type": "stdio", // a note` parses.
    - Test 3 — block comments: a `/* ... */` comment, including a multi-line one, parses; and the
      projected line numbers for keys AFTER the block comment are the SAME as they are in a document
      where the comment is replaced by blank lines. This is the line-preservation property.
    - Test 4 — THE CASE A NAIVE IMPLEMENTATION GETS WRONG: `"url": "http://metrics.internal.example.com/mcp"`
      projects with its value intact, character for character, including the `//`. Add a second
      specimen for the block marker: a value containing `/*` (e.g. an argument
      `"--glob=/*.json"`) survives verbatim. A comment marker inside a string literal is not a
      comment.
    - Test 5 — escaped quote: a value containing `\"` followed later by `//` on the same line
      (e.g. `"note": "he said \"hi\"", // trailing`) parses, proving the string walker honours
      backslash escapes rather than closing the string early.
    - Test 6 — trailing commas: `{"a": 1,}` and `{"a": [1, 2,],}` parse.
    - Test 7 — leading comment before `{`: a document starting with `// header\n{ ... }` is
      recognised by `extract()` (returns `Some`, syntax `Json`) and `analyze()` returns `Ok(Some(..))`
      rather than `Ok(None)`, with `start_line` pointing at the `{` line.
    - Test 8 — the invariant, asserted directly: for every fixture string in this test module,
      `relax_jsonc(s).len() == s.len()` and
      `relax_jsonc(s).lines().count() == s.lines().count()`.
    - Test 9 — still an error when it really is one: `{ "a": }` still returns `Err` from `analyze`
      after relaxation, and the error text is non-empty. (This is what keeps Task 1 honest.)
    - Test 10 — a plain prose document is still `Ok(None)`, and a document starting with `/` that is
      not JSON after comment blanking (e.g. `/usr/local/bin/foo\n`) is still `Ok(None)`, not an
      error.

    In `tests/scan_resilience_test.rs`, the end-to-end proof the detection gap is closed:
    - Test 11 — a temp-dir `mcp.json` in the reported house style (a `//` comment line, plus a
      `"url": "http://metrics.internal.example.com/mcp"` value) scanned with `check <dir>` reports
      a PI061 finding, and prints NO `structural config pass skipped` warning. Assert on the
      pattern id so the test names what it proves.
  </behavior>
  <action>
    Add `pub fn relax_jsonc(body: &str) -> String` to `src/frontmatter.rs` with `///` rustdoc
    stating the contract in one sentence: it returns a string of the SAME byte length and the SAME
    number of line breaks, with JSONC comment bytes and trailing-comma bytes overwritten by ASCII
    spaces, and nothing inside a string literal touched — which is what lets `locate()` and every
    reported line number stay valid against the original document.

    Implement as a single forward pass over `body.as_bytes()` into a `Vec<u8>` of equal length,
    tracking exactly three states — outside, inside a string literal, inside a comment — with
    backslash escape handling inside strings. Inside a comment emit `b' '` for every byte EXCEPT
    `b'\n'`, which is emitted unchanged (blanking a newline would shrink the line count and move
    every subsequent reported line). Multi-byte UTF-8 inside a comment is replaced byte-by-byte with
    ASCII spaces, which keeps the result valid UTF-8 because only whole comment regions are touched.
    Then a second pass over the blanked bytes, string-aware in the same way: a `,` outside a string
    whose next non-whitespace byte is `}` or `]` is overwritten with a space. Rebuild with
    `String::from_utf8(..)` and, on the unreachable error arm, fall back to returning `body.to_string()`
    rather than panicking — `unwrap()` is denied crate-wide and this input is untrusted by definition.

    Wire it into `parse()` as a FALLBACK on the `ConfigSyntax::Json` arm only: try
    `serde_json::from_str` first and return on success, so a well-formed document pays nothing; on
    failure, run `relax_jsonc` and retry. Report the RELAXED parser's error when the retry also
    fails — because offsets are preserved, its line and column still point at the right place in the
    original file, whereas the strict error would point at the comment. Keep the
    `invalid JSON document: {e}` message shape. YAML and TOML arms are untouched: both syntaxes have
    native comments and their parsers already accept them.

    Extend `extract()` so a document whose first non-whitespace byte is `/` gets the tolerant path:
    blank comments over the content, and if the blanked text's first non-whitespace byte is `{`,
    build the `ConfigBlock` with `body` = the ORIGINAL text from that `{` onward (raw, comments
    intact — `locate` searches it) and `start_line` = 1 + the number of `\n` before that offset.
    Gate on that single leading `/` byte so an ordinary markdown file pays one byte comparison and
    nothing else. Document the gate.

    Update the module header (src/frontmatter.rs:1-39) to record that the Json path is JSONC-tolerant
    and why, and update `parse()`'s doc comment.

    Do NOT change `ProjectedLine`, `project`, `walk`, `render_scalar`, or the `MAX_DEPTH` /
    `MAX_NODES` / `MAX_VALUE_LEN` bounds — relaxation happens before parsing, so every existing
    bound still applies to the parsed tree unchanged.
  </action>
  <verify>
    <automated>cargo test --test frontmatter_test --test scan_resilience_test --test perf_regression_test</automated>
  </verify>
  <done>All eleven tests above pass; every test written in Task 1 still passes unchanged, including the "still unparseable after relaxation still warns" case; `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` clean; committed.</done>
  <reversibility rating="reversible">A fallback on one parse arm plus a pure function. Deleting `relax_jsonc` and the fallback restores the strict behaviour exactly.</reversibility>
</task>

<task type="auto">
  <name>Task 3: ADR, docs, and the full-suite + self-scan gate</name>
  <files>docs/adr/ADR-005-jsonc-tolerant-config-parsing.md, README.md</files>
  <read_first>.claude/skills/pr-artifacts/adr_template.md and SKILL.md, docs/adr/ADR-004-relaxed-pattern-false-positive-control.md (house voice and depth), README.md:499-533 (the JSON output section and the paragraph listing the additive arrays), README.md:129-156 ("What gets scanned").</read_first>
  <action>
    Write `docs/adr/ADR-005-jsonc-tolerant-config-parsing.md` from the skill template. The
    pr-artifacts skill makes an ADR mandatory here on two counts: this changes the config parsing
    engine, and it changes the JSON output contract. Record the alternatives actually weighed and
    why each lost — warning-only (annotates the gap instead of closing it), the `json5` crate (a new
    dependency, and it widens the accepted grammar far past comments: unquoted keys, single quotes,
    hex numbers), a span-preserving JSON parser (solves `locate`'s best-effort line mapping too, but
    is a rewrite this issue does not justify) — and state the byte-offset invariant as the property
    that made the preprocessor cheap. Record the `skip_serializing_if` choice against the pinned key
    set, and the `.jsonl`/`.ndjson` print suppression, including its cost: a genuinely broken
    `.jsonl` config gets no warning, accepted because such a file is not a config document.

    Update `README.md`:
    - In the paragraph after the `### JSON output` example (~:525), add `config_parse_error` — a
      string that appears only when a config block was found and could not be parsed, additive, and
      not a finding, so it does not affect `critical_count` and friends or the exit code.
    - In `### What gets scanned`, state that `//` and `/* */` comments and trailing commas in JSON
      config files are tolerated (the VS Code / Copilot house style), and that a config block that
      still cannot be parsed is reported on stderr rather than skipped silently.
    Keep both edits short — README is already long and `S004 file-size` warns past 30kb.

    Then run the gates. Run the FULL suite in the background per the execution rules, poll it to
    completion, and paste the final summary line into the commit body. Then run
    `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`. Finally run the self-scan
    the code-scanning workflow runs —
    `cargo run --release -- check . --baseline .github/code-scanning-baseline.json` — and confirm
    it still exits CLEAN with no new findings. If it does not, STOP and report: a new finding means
    a JSONC file in this repo became parseable and now projects a real payload, which is a result
    worth a human decision, not a baseline regeneration.

    Do not edit `patterns/`, `docs/PATTERN-CATALOGUE.md`, or `.github/code-scanning-baseline.json`.
  </action>
  <verify>
    <automated>cargo clippy --all-targets -- -D warnings &amp;&amp; cargo fmt --check &amp;&amp; test -f docs/adr/ADR-005-jsonc-tolerant-config-parsing.md &amp;&amp; grep -q config_parse_error README.md &amp;&amp; git diff --name-only origin/main | grep -qv '^patterns/'</automated>
    <human-check>Full `cargo test` suite green — run backgrounded to a log and polled, never in the foreground; record the final "test result:" line for every test binary in the SUMMARY.</human-check>
  </verify>
  <done>ADR-005 exists and names the rejected alternatives; README documents the new field and the tolerance; the full suite is green with the counts recorded; clippy and fmt clean; the self-scan exits CLEAN against the unmodified baseline; nothing under `patterns/` is touched; committed.</done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| scanned file → `frontmatter::relax_jsonc` | The document is authored by the adversary by definition; the relaxer is now the first code to touch its bytes. |
| `ScanReport` → `--format json` → `spec-ci-plugin` | A new field crosses into a downstream consumer's parse. |
| `ScanReport` → stderr | Parse-error text derived from attacker-controlled input is printed to a terminal and to CI logs. |

## STRIDE Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation Plan |
|-----------|----------|-----------|----------|-------------|-----------------|
| T-129-01 | Tampering | `relax_jsonc` blanking inside a string literal | high | mitigate | String-literal state with backslash-escape handling; Task 2 Test 4 and Test 5 pin that a `//` in a URL value and a `/*` in an argument survive verbatim and that PI061 still fires end to end (Test 11). Blanking inside a string would silently delete a payload — the exact failure this tool exists to prevent. |
| T-129-02 | Denial of Service | `relax_jsonc` over an adversarial document | medium | mitigate | Single forward pass, O(n), no backtracking, one allocation of the input's own length. Runs only as a fallback after a strict parse has already failed, and only on the `Json` arm. `perf_regression_test` is in Task 2's verify command. |
| T-129-03 | Tampering | the relaxer accepting more than the host does | medium | mitigate | Tolerance is limited to comments and trailing commas — what VS Code's own JSONC parser accepts. The `json5` grammar widening (unquoted keys, single quotes) is explicitly rejected in the decision record, so the scanner does not read a document the host would refuse. |
| T-129-04 | Information Disclosure | the new `warning:` line and the `config_parse_error` field | low | accept | `serde_json` error text carries line/column and a token class, not file content; the path printed is one the user supplied. Same exposure as the existing `warning: skipped {path} — {reason}` lines. |
| T-129-05 | Denial of Service | alarm fatigue from a per-file warning on `.jsonl`/`.ndjson` | medium | mitigate | Print suppressed for line-delimited stream extensions (Task 1, Test 4). A warning that fires on every dataset file trains users to filter the one line that matters. |
| T-129-06 | Repudiation | `--baseline` rebuilding the report and dropping the field | high | mitigate | `src/baseline.rs:270` carries the field across the rebuild; Task 1 Test 5 pins it. Otherwise the exact flag CI runs with silently restores the #129 behaviour. |
| T-129-SC | Tampering | npm/pip/cargo installs | high | mitigate | N/A this task — no dependency is added. The decision record rejects the `json5` crate for exactly this reason; if that changes, the package-legitimacy gate applies before any install. |
</threat_model>

<verification>
1. `cargo test` (backgrounded, polled) — full suite green, no test count regression against the 357 recorded in STATE.md plus the new tests.
2. `cargo clippy --all-targets -- -D warnings` clean; `cargo fmt --check` clean.
3. Manual reproduction from the issue, both formats, both streams captured:
   `target/release/injection-scanner check /tmp/mcp-jsonc/mcp.json` reports the PI061 finding and no warning;
   `target/release/injection-scanner check /tmp/mcp-jsonc/broken.json` reports the `warning: structural config pass skipped for ...` line on stderr and exits 0.
4. `cargo run --release -- check . --baseline .github/code-scanning-baseline.json` exits CLEAN.
5. `git diff --name-only origin/main` lists nothing under `patterns/`, `docs/PATTERN-CATALOGUE.md`, or `.github/code-scanning-baseline.json`.
</verification>

<success_criteria>
- Every truth in `must_haves.truths` is pinned by a named test, not by inspection.
- The reported bug is fixed: a config file that cannot be parsed is visible on stderr, under
  `--quiet`, and under `--baseline`.
- The detection gap is closed: a `//`-commented `mcp.json` in the reported house style produces the
  PI06x finding it should.
- A `//` inside a string literal is proven intact by a test that would have failed against a naive
  line-comment strip.
- The `--format json` key set for a parseable report is byte-for-byte what `spec-ci-plugin` sees
  today.
- ADR-005 and the README updates satisfy the pr-artifacts skill before a PR is opened.
</success_criteria>

<output>
Create `.planning/quick/260915-spt-fix-129-jsonc-commented-config-silently-/260915-spt-SUMMARY.md` when done,
recording: the final test counts per binary, the self-scan result, the manual reproduction output for
both the commented and the broken file, and anything deferred (with a reason).
</output>
