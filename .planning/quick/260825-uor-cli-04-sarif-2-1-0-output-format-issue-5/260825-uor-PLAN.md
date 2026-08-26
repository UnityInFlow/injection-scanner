---
phase: quick-260825-uor
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - src/lib.rs
  - src/sarif.rs
  - src/baseline.rs
  - src/patterns/mod.rs
  - src/main.rs
  - tests/sarif_test.rs
  - tests/json_contract_test.rs
  - .github/workflows/code-scanning.yml
  - .github/code-scanning-baseline.json
  - README.md
  - docs/adr/ADR-003-sarif-output.md
  - TODO.md
autonomous: true
requirements: [CLI-04]
user_setup: []

estimate:
  tokens: 95000
  raw_tokens: 95000
  tasks: 3
  confidence: low

must_haves:
  truths:
    - "`check <path> --format sarif` writes a SARIF 2.1.0 document to stdout that parses as JSON and carries the required `$schema`, `version`, `runs[0].tool.driver` and `runs[0].results` structure"
    - "Exactly one SARIF result per entry in `ScanReport.matches`; `suppressed`, `low_confidence` and `baselined` produce ZERO results (D-1)"
    - "Every result's `ruleId` resolves to a rule in `runs[0].tool.driver.rules`, and its `ruleIndex` points at that same rule"
    - "Every finding severity maps to a `level` drawn from the closed SARIF set — CRITICAL and HIGH to `error`, MEDIUM to `warning`, LOW to `note`"
    - "The native four-level severity survives the lossy 4-to-3 mapping: it is recoverable from `result.properties.severity` and is distinguishable in GitHub's UI via the rule's `properties.security-severity`"
    - "`partialFingerprints` are line-number independent, and two identical payloads in one file get two DISTINCT fingerprints"
    - "`artifactLocation.uri` is a valid relative URI reference — no leading `./`, no raw spaces or angle brackets"
    - "Exit codes under `--format sarif` are identical to the codes under `--format text`, for the same scan"
    - "`--format json` output is unchanged: top level is a bare array of report objects, and the key set of each report and each match is exactly what v0.0.3 emitted"
    - "`rules --format sarif` is rejected by clap at parse time, because a rules-only document has an empty `results` array and uploading one closes every open alert"
    - "The code-scanning upload runs on a trigger a fork cannot fire, and `ci.yml` gains NO write scope"
    - "The work stays stacked on `feat/cli-08-baseline` and its PR is retargeted to `main` before #79's branch is deleted (D-2)"
  artifacts:
    - "src/sarif.rs — the SARIF document model and `format_sarif`"
    - "src/main.rs — `OutputFormat::Sarif`, and a separate `RulesFormat` for the `rules` subcommand"
    - "src/patterns/mod.rs — `GradedRule` and `grade`, moved out of main.rs so `rules`, `explain` and SARIF share one source of truth"
    - "src/baseline.rs — `fingerprint` promoted to `pub`"
    - tests/sarif_test.rs
    - tests/json_contract_test.rs
    - .github/workflows/code-scanning.yml
    - .github/code-scanning-baseline.json
    - docs/adr/ADR-003-sarif-output.md
    - "README.md — a `### SARIF output` section under `## Usage`"
  key_links:
    - "`ScanReport.matches` is the ONLY array the writer reads — D-1 is enforced at the one place the results array is built, not by filtering downstream"
    - "`baseline::fingerprint` now has two consumers: the committed baseline and the code-scanning alert identity. Changing it moves both, which is the property that makes the reuse worth having"
    - "the id-to-index map used for `ruleIndex` is built from the SAME `grade(&categories)` output that fills `tool.driver.rules`, so the two cannot drift"
    - "`.github/workflows/code-scanning.yml` is `push: main` / `schedule` / `workflow_dispatch` only — never `pull_request`, because that is the trigger that would put a `security-events: write` token in a job running fork-authored build scripts"
    - "the committed `.github/code-scanning-baseline.json` is what keeps `patterns/` and `examples/` IN scope for code scanning instead of blanket-excluded — a poisoned pattern PR must still raise an alert"
---

<objective>
Implement CLI-04 — a real `--format sarif` writer, plus the GitHub code-scanning upload that is the
second half of the requirement. Closes issue #5.

Purpose: this is the last unmet *original* v0.0.1 promise in Phase 4's success criteria, and
`--format sarif` silently returning human-readable text is one of the audit findings that opened
this milestone. The `OutputFormat` doc comment in `src/main.rs` forbids adding the variant before
the writer exists; this plan adds both in the same change.

Output: `src/sarif.rs`, the `OutputFormat::Sarif` wiring, two new test files,
`.github/workflows/code-scanning.yml` with its committed baseline, README section, and
`docs/adr/ADR-003-sarif-output.md` (required by the `pr-artifacts` skill — "output format
contracts (JSON, SARIF)" is an explicit ADR trigger).
</objective>

<execution_context>
Branch: `feat/cli-04-sarif`, already stacked on `feat/cli-08-baseline` (verified: `cli-08` is an
ancestor, zero commits ahead). Per D-2 this stays stacked. **Orchestrator hazard, not an
implementation one:** this repo rebase-merges, and merging PR #79 with `--delete-branch`
auto-closes the stacked child, after which GitHub refuses both `gh pr reopen` and retargeting.
Retarget this PR to `main` BEFORE #79's branch is deleted.

Read before writing code:
@.claude/skills/code-review/SKILL.md
@.claude/skills/pr-artifacts/SKILL.md
</execution_context>

<context>
@.planning/quick/260825-uor-cli-04-sarif-2-1-0-output-format-issue-5/260825-uor-CONTEXT.md
@CLAUDE.md
@src/main.rs
@src/reporter.rs
@src/pattern.rs
@src/baseline.rs
@src/patterns/mod.rs
@.github/workflows/ci.yml
@docs/adr/ADR-002-baseline-fingerprints.md
</context>

<design_decisions>

These resolve the CONTEXT's "Design guidance (not locked)" section. Three of them differ from what
the guidance proposed; each says why.

**1. The writer lives in `src/sarif.rs`, not `src/reporter.rs`.**
`reporter.rs` is 123 lines of string formatting with no data model. SARIF needs a serializable
document model — roughly eight structs plus four mapping helpers — which would more than double
that file and mix two levels of abstraction in it. The decisive reason is narrower: `format_json`
in `reporter.rs` *is* the contract `spec-ci-plugin` parses. Keeping SARIF out of that file means
"did this change the JSON contract?" is answerable by looking at a file whose diff is empty.
`format_sarif` mirrors `format_json`'s signature, returning `Result<String, serde_json::Error>`
rather than `anyhow`, for the reason already documented on `format_json`.

**2. Severity mapping: `level` as proposed; `security-severity` instead of `rank`.**
CRITICAL and HIGH to `error`, MEDIUM to `warning`, LOW to `note` — agreed, unchanged.

The CONTEXT proposed carrying the native severity in `properties` **and/or `rank`**. Decline
`rank`. GitHub code scanning does not read it; what it reads is `properties["security-severity"]`
on the **rule descriptor**, a 0.0-10.0 string it bands into the alert severity shown in the
Security tab (critical at 9.0 and above, high 7.0-8.9, medium 4.0-6.9, low below 4.0). So `rank`
would be a second, differently-scaled severity number that nothing consumes and that can silently
drift out of step with `level` — a maintenance trap in exchange for nothing.

Use instead:
- rule `properties["security-severity"]`: `"9.0"` CRITICAL, `"7.0"` HIGH, `"5.0"` MEDIUM,
  `"2.0"` LOW (strings). This is what actually recovers the lossy mapping where it matters — a
  reviewer looking at the Security tab sees four distinct severities, not two collapsed into
  `error`.
- rule `properties["tags"]`: `["security", "<category>"]`. The literal `security` tag is required
  before GitHub will honour `security-severity` at all; the category comes from the pattern file.
- result `properties["severity"]`: the native string (`CRITICAL` / `HIGH` / `MEDIUM` / `LOW`), for
  every consumer that is not GitHub.

**3. `partialFingerprints` reuse `baseline::fingerprint`, with an occurrence ordinal.**
Agreed on the reuse and on why it is more than a convenience. Two corrections:

(a) **`fingerprint` is NOT public.** `src/baseline.rs:84` reads `fn fingerprint(...)`. Both the
CONTEXT and the codebase orientation state it is already public; it is not. Promoting it to `pub`
is part of Task 1.

(b) **The bare digest is not a sufficient fingerprint.** It hashes `matched_text` alone — not the
file, not the pattern id, not the line. Two occurrences of the same payload in the same file
therefore produce byte-identical `(ruleId, uri, partialFingerprint)` triples, which is the tuple
GitHub tracks an alert by. The two collapse into one alert, and fixing one of them then closes an
alert whose payload is still in the file. For a security scanner that is a missed alert, which is
the failure direction that matters.

So: key `matchedTextSha256/v1`, value `<fingerprint>/<n>` where `n` is the 1-based ordinal of this
occurrence within its `(file, ruleId, digest)` group — the same grouping `Baseline::from_reports`
already counts with. Always suffixed, including the first, so there is one rule and no special
case. Line-number independence is preserved, which is the whole point. Accepted cost: deleting the
first of two identical payloads renumbers the survivor, so GitHub closes one alert and opens
another. That is cosmetic churn, traded against a silently missed alert.

**4. Rules come from a shared `grade()`, moved into the library.**
Agreed with the guidance that `explain` already assembles this material — but `GradedRule` and
`load_graded` are private to `src/main.rs`, and the writer belongs in the library so it can be
unit-tested. Move the struct and a **pure** `grade(&[PatternCategory]) -> Vec<GradedRule>` into
`src/patterns/mod.rs`. `load_graded` stays in `main.rs` as the thin wrapper that prints the
per-file load warnings — a library function must not write to stderr.

`GradedRule` moves verbatim, field order and `Serialize` derive included, because
`rules --format json` serializes it and that output must not change either.

Emit **all** loaded rules, not only those that fired. SARIF permits rules with no results, and the
rule metadata is useful on its own. The reverse — a result whose `ruleId` has no rule — cannot
happen: `Scanner::new_lenient` only ever drops patterns, so the rule set is a superset of what can
match.

**5. Validation: structural assertions plus the upload. No vendored schema, no `jsonschema` crate.**
Rejected `jsonschema` as a dev-dependency: it pulls a large tree (fancy-regex, url, referencing and
friends) into the `Cargo.lock` of a security tool that ships SLSA-attested binaries, SHA-pins its
actions, and has kept itself to twelve dependencies. One test assertion does not buy that.

Rejected vendoring `sarif-schema-2.1.0.json`: it is roughly half a megabyte, `json` is in
`DEFAULT_EXTENSIONS`, and this repo scans itself — so every `check .` and every pre-commit hook run
would walk it forever, for a check that is necessary but not sufficient anyway. A document can be
schema-valid and still be rejected or mis-rendered by GitHub's ingest.

What we do instead:
- the writer is built from `#[derive(Serialize)]` structs, so the document shape is fixed at
  compile time rather than assembled ad hoc from `json!` macros;
- `tests/sarif_test.rs` asserts SARIF 2.1.0's required properties explicitly, enumerated in the
  test — `$schema`, `version`, `runs`, `runs[].tool.driver.name`, `results[].message.text`,
  `results[].locations[].physicalLocation.artifactLocation.uri`, `region.startLine >= 1`, and the
  closed `level` set;
- the real end-to-end proof is `.github/workflows/code-scanning.yml`. GitHub's ingest validates
  against 2.1.0 and rejects invalid documents, which is where "validates against the schema" is
  actually settled — the CONTEXT says exactly this and it is right.

**6. `ci.yml` is not touched. `security-events: write` goes in a separate workflow.**
Answering the question posed directly: **no, `security-events: write` is not safe in `ci.yml`.**
That workflow is `on: pull_request`, so it runs fork-authored code — `cargo test` executes
build scripts and proc macros from a fork-modified `Cargo.toml`. Granting a write scope in a job
that runs attacker-supplied code is the escalation its own RUNNER POLICY header forbids in as many
words ("Never add write scopes or secrets to this workflow"). GitHub does hand fork PRs a
read-only token regardless, but relying on that leaves the same-repo-branch and `push: main` runs
holding the write scope for no reason, and it makes the file's stated invariant untrue.

New `.github/workflows/code-scanning.yml`, GitHub-hosted (self-hosted remains impossible on this
public repo), `on: push: branches: [main]` + `schedule` + `workflow_dispatch`, never
`pull_request`. This is the same shape the repo already uses for `release.yml`: elevated
permissions only behind a trigger a fork cannot fire, over code that has already been reviewed onto
`main`.

No SARIF smoke step is added to `ci.yml` either. `cargo test` in that same job already runs the
integration tests against `CARGO_BIN_EXE_injection-scanner`, so a step would be redundant — and the
strongest answer to "is the fork-facing workflow still safe" is that its diff is empty.

**7. Self-scan noise: a committed baseline, not blanket excludes.**
Measured on this checkout: `check .` reports 52 findings across `examples/` (5 attack corpora),
`patterns/core/*.yaml` (the regexes are payload-shaped by construction), `tests/fixtures/`,
`tools/injection-lab/corpus/` and `.planning/phases/phase-2/PLAN.md`. Uploaded unfiltered, that is
52 alerts of pure noise on a public repo's Security tab on day one.

Blanket `--exclude 'examples/**' --exclude 'patterns/**'` would fix the noise and build a blind
spot into exactly the two directories a malicious community pattern PR would land in — invisible to
anyone reading the Security tab. Instead, commit `.github/code-scanning-baseline.json` and scan
with `--baseline`. Verified live on this checkout: 51 entries written, rescan with the baseline
reports zero and exits 0, and the baseline file itself scans clean (ADR-002's hashing is what makes
that true). D-1 then does the rest — baselined findings produce no SARIF results, so a clean `main`
uploads an empty results array and any genuinely new payload anywhere in the repo becomes an alert.

`.planning/**` is excluded by glob rather than baselined: it is planning prose, not part of the
shipped or agent-facing surface, and it grows a new payload-describing document every quick task —
a permanent source of regeneration churn.

This also exercises the `--baseline` + `--format sarif` seam, which is precisely the class of gap
STATE.md's 2026-08-25 review note says keeps slipping through ("the gap was at a feature seam, and
green tests hid it").

**8. `rules` gets its own format enum.**
Adding `Sarif` to the shared `OutputFormat` would silently make `rules --format sarif` a valid
invocation. The document it would produce is a run with `tool.driver.rules` populated and
`results: []` — which is not "here is the rule catalogue" to a code-scanning consumer, it is "this
analysis found nothing", and uploading it closes every open alert for the category. Give
`Commands::Rules` a `RulesFormat { Text, Json }` and clap keeps rejecting `sarif` at parse time
with the valid list, exactly as it does today. That is the property the `OutputFormat` doc comment
was introduced to protect, preserved rather than traded away for one shared type.

</design_decisions>

<tasks>

<task type="tracer">
  <name>Task 1: One finding, end to end, as SARIF</name>
  <files>src/sarif.rs, src/lib.rs, src/main.rs, tests/sarif_test.rs</files>
  <action>
Wire the thinnest complete path from a scanned finding to a SARIF document on stdout. Deliberately
minimal — Task 2 expands it, and its tests must be able to fail first.

Add `pub mod sarif;` to `src/lib.rs`.

Create `src/sarif.rs` with `#[derive(Serialize)]` structs modelling only what a minimal valid
document needs: a top-level report carrying `$schema` (rename the field; the value is the official
`raw.githubusercontent.com` 2.1.0 schema URL, a string that is never fetched) and `version` set to
`2.1.0`; `runs`, each with `tool.driver` (`name`, `version` from `env!("CARGO_PKG_VERSION")`,
`information_uri` renamed to `informationUri` and pointing at the repository) and `results`.

Each result carries `ruleId`, `level`, `message.text`, and one location with
`physicalLocation.artifactLocation.uri` and `physicalLocation.region.startLine`. Use
`#[serde(rename_all = "camelCase")]` on the structs so Rust field names stay idiomatic. Message
text is the finding's `message`.

Add `pub fn format_sarif(reports: &[ScanReport]) -> Result<String, serde_json::Error>` using
`to_string_pretty`, mirroring `reporter::format_json`. It reads `ScanReport.matches` and nothing
else — this single call site is where D-1 is enforced, so the other three arrays are never
in scope here at all. Say that in the rustdoc.

Severity to level: a private helper mapping CRITICAL and HIGH to `error`, MEDIUM to `warning`, LOW
to `note`, matched exhaustively with no catch-all arm.

In `src/main.rs`: add the `Sarif` variant to `OutputFormat` with a doc comment, and **rewrite the
existing "SARIF is deliberately absent" paragraph** on the enum so it records that the writer now
exists and that the invariant it protects is unchanged — do not leave a comment contradicting the
code. Add the `Display` arm. Add the `Sarif` arm to BOTH `match format` sites in the `Check` arm
(the `--write-baseline` branch around line 497 and the main output branch around line 538); in the
`--write-baseline` branch SARIF shows the pre-baseline run, consistent with what text and JSON
already do there.

`Commands::Rules` still holds `OutputFormat` at this point and will not compile once the variant
exists — split it to `RulesFormat` now (per design decision 8): a `ValueEnum` with `Text` and
`Json`, its own `Display`, and the `Rules` arm matching on it. `--format` help text and the `text`
default are unchanged.

Start `tests/sarif_test.rs` with a single end-to-end test, spawned via
`env!("CARGO_BIN_EXE_injection-scanner")` (see `tests/test_harness_contract_test.rs` — any other
spelling fails the build): run `check tests/fixtures/injected-skill.md --format sarif`, assert
stdout parses as JSON, `version` is `2.1.0`, `runs` has one entry, and `results` is non-empty with
a `ruleId`, a `level`, a `message.text` and a `startLine` of at least 1.

No `unwrap()` anywhere — `#![deny(clippy::unwrap_used)]` is on both crates. Integration tests may
use `expect` with a message, matching the existing test files.
  </action>
  <verify>
    <automated>cargo test --test sarif_test 2>&amp;1 | tail -5</automated>
    <automated>cargo run --quiet -- check tests/fixtures/injected-skill.md --format sarif | python3 -c "import json,sys; d=json.load(sys.stdin); assert d['version']=='2.1.0'; assert len(d['runs'][0]['results'])>0; print('ok', len(d['runs'][0]['results']), 'results')"</automated>
    <automated>cargo fmt --all -- --check &amp;&amp; cargo clippy --all-targets --locked -- -D warnings</automated>
  </verify>
  <done>
`check <file> --format sarif` emits a document that parses as JSON, declares `2.1.0`, and carries
one result per reported finding with a rule id, a level, a message and a start line. `rules
--format sarif` is refused by clap. `--format text` and `--format json` are untouched. Committed.
  </done>
</task>

<task type="auto" tdd="true">
  <name>Task 2: The SARIF contract, driven by tests that fail first</name>
  <files>src/sarif.rs, src/baseline.rs, src/patterns/mod.rs, src/main.rs, tests/sarif_test.rs, tests/json_contract_test.rs</files>
  <behavior>
Write these as failing tests against Task 1's minimal writer BEFORE implementing. Each one fails
for a stated reason at the start — record the observed failure, do not assume it.

In `tests/sarif_test.rs` (integration, `CARGO_BIN_EXE_injection-scanner`; build inputs from
`tests/fixtures/` or from temp files whose payloads are read out of those fixtures — no verbatim
payloads in new repository files, this repo scans itself):

- **Level mapping is total and closed.** A scan producing all four native severities yields results
  whose `level` values are drawn only from `error` / `warning` / `note` / `none`, and the
  CRITICAL and HIGH findings both land on `error`. Fails now: `level` exists but nothing pins the
  set.
- **The native severity survives the lossy mapping.** Every result carries
  `properties.severity` equal to its native level, and the rule for a CRITICAL pattern carries a
  `properties.security-severity` strictly greater than the one for a HIGH pattern — the pair that
  collapses to a single `level`. Fails now: no `properties` are emitted.
- **Every `ruleId` resolves.** Collect `runs[0].tool.driver.rules` ids; assert every result's
  `ruleId` is in that set AND that `rules[result.ruleIndex].id == result.ruleId`. Fails now: there
  is no rules array.
- **Rules carry usable metadata.** Each rule has `id`, `name`, `shortDescription.text`,
  `fullDescription.text`, `help.text` containing the remediation, `properties.tags` including the
  literal security tag, and a `security-severity`.
- **The three withheld arrays produce no results — three separate cases.** (1) a file whose finding
  is suppressed by an in-file directive; (2) a payload inside a fenced code block, withheld as
  low-confidence; (3) a finding accepted by a `--baseline` written in the same test. Each asserts
  the corresponding `--format json` array is non-empty AND that `results` is empty — the array must
  be non-empty, or the test proves nothing. Fails now for the baseline case only if Task 1 leaked;
  expect all three to pass immediately and say so in the report rather than claiming they failed
  first.
- **SARIF 2.1.0 required structure.** One test enumerating the required properties by name:
  `$schema`, `version` exactly `2.1.0`, `runs` a non-empty array, `runs[].tool.driver.name`
  present, and for each result `ruleId`, `level`, `message.text`, and
  `locations[].physicalLocation.artifactLocation.uri` with `region.startLine >= 1`.
- **URI hygiene.** Scan a directory (so paths arrive with the `./` prefix `check .` produces) and a
  temp file whose name contains a space. Assert no `uri` starts with `./` and none contains a raw
  space, `<` or `>`. Fails now: the writer passes `ScanMatch.file` through verbatim.
- **Fingerprints are line-independent and non-colliding.** A file with the same payload twice
  yields two results with DIFFERENT `partialFingerprints` values; prepending ten blank lines to
  that file leaves both values unchanged. Fails now: there are no `partialFingerprints`.
- **Exit codes are format-independent.** For a CRITICAL fixture, a MEDIUM-only document under
  `--fail-on critical`, and a clean file, `--format sarif` exits with the same code
  `--format text` does (1, 2, 0). Borrow the matrix shape from `tests/cli_surface_test.rs`.
- **`--quiet --format sarif` writes nothing to stdout** and still exits on the finding.
- **`rules --format sarif` is rejected at parse time**: non-zero exit, stderr names the valid
  values, and stdout is not a SARIF document.

In `tests/json_contract_test.rs` — the "byte-identical `--format json`" guard. No golden file:
committing one would put verbatim payloads in a new repository file and the scanner would flag its
own fixture. Assert the contract explicitly instead, which is what a golden file would have been
protecting:
- `check <fixture> --format json` stdout parses as a JSON **array** at the top level (this is the
  `JSON.parse(output) as Array<...>` that `spec-ci-plugin` does);
- each report object's key set is EXACTLY `file`, `matches`, `suppressed`, `low_confidence`,
  `baselined`, `critical_count`, `high_count`, `medium_count`, `low_count` — no more, no fewer, so
  a field added for SARIF's benefit fails here;
- each match object's key set is EXACTLY `pattern_id`, `pattern_name`, `severity`, `message`,
  `remediation`, `file`, `line`, `matched_text`, `context`, `confidence`;
- output is pretty-printed, not compact;
- `rules --format json` still parses and each entry's key set is exactly `id`, `name`, `severity`,
  `category`, `description`, `remediation`, `pattern`, `tags` — this one guards the `GradedRule`
  move below.
  </behavior>
  <action>
Implement only what the tests above demand.

`src/baseline.rs`: promote `fingerprint` to `pub`. Extend its rustdoc with a note that it now has
two consumers — the committed baseline and the SARIF alert identity — so a change to it moves both,
and that this shared definition of "the same finding" is the reason for the reuse rather than a
convenience.

`src/patterns/mod.rs`: move `GradedRule` from `src/main.rs` verbatim (field order, names and the
`Serialize` derive unchanged — `rules --format json` serializes it) and make it and its fields
`pub`, with `///` docs on the type. Add `pub fn grade(categories: &[PatternCategory]) ->
Vec<GradedRule>`, holding the existing severity-resolution and sort-by-id logic, and nothing else —
no stderr output from a library function. In `src/main.rs`, `load_graded` becomes the thin wrapper
that loads, prints the per-file warnings it already prints, and returns `grade(&loaded.categories)`.

`src/sarif.rs`: extend the model and change the signature to
`format_sarif(reports: &[ScanReport], rules: &[GradedRule]) -> Result<String, serde_json::Error>`.

- `tool.driver.rules`: one descriptor per `GradedRule` — `id`, `name`, `shortDescription.text` from
  the description (fall back to the name when a pattern's description is empty),
  `fullDescription.text`, `help.text` carrying the remediation, and `properties` with `tags`
  (the security tag plus the category) and `security-severity` as a string: `9.0` CRITICAL,
  `7.0` HIGH, `5.0` MEDIUM, `2.0` LOW. Build the id-to-index map from this same slice so
  `ruleIndex` cannot drift from the array.
- results: add `ruleIndex`, `partialFingerprints`, and `properties.severity` carrying the native
  level. A finding whose `pattern_id` is somehow absent from `rules` must not be dropped and must
  not emit a dangling `ruleIndex` — omit the index, keep the result. Note in the rustdoc why that
  case cannot arise today (lenient loading only ever drops patterns).
- `partialFingerprints`: key `matchedTextSha256/v1`, value `baseline::fingerprint(&m.matched_text)`
  with `/<n>` appended, `n` being the 1-based ordinal within the `(file, ruleId, digest)` group.
  Count the groups in one pass over the report's matches.
- URI: strip exactly one leading `./`, then percent-encode every byte outside
  `A-Za-z0-9`, `-`, `_`, `.`, `~` and `/` as uppercase `%XX`. That is ~15 lines and no new
  dependency; it makes the space, angle-bracket and non-ASCII cases correct with one rule rather
  than a special case per character. The stdin sentinel is handled by the same rule and needs no
  branch.

`src/main.rs`: pass `&grade(&categories)` into `format_sarif` at both call sites, computing it only
on the SARIF arm so the text and JSON paths pay nothing.

Keep the checklist in `.claude/skills/code-review/SKILL.md` in view while writing: no `unwrap()`,
no stray `println!` for debugging, `///` on every public item, exhaustive matches with no catch-all.
  </action>
  <verify>
    <automated>cargo test --test sarif_test --test json_contract_test 2>&amp;1 | tail -20</automated>
    <automated>cargo test --locked 2>&amp;1 | tail -30</automated>
    <automated>cargo fmt --all -- --check &amp;&amp; cargo clippy --all-targets --locked -- -D warnings</automated>
  </verify>
  <done>
All the behaviours listed above are asserted and green; the full suite (25+ test binaries) is green;
fmt and clippy are clean. Every result resolves to a rule, the four native severities are recoverable
from the document, the three withheld arrays contribute nothing, and the `--format json` contract is
pinned by an explicit key-set test. Committed.
  </done>
</task>

<task type="auto">
  <name>Task 3: The upload leg, plus ADR and docs</name>
  <files>.github/workflows/code-scanning.yml, .github/code-scanning-baseline.json, README.md, docs/adr/ADR-003-sarif-output.md, TODO.md</files>
  <precondition>`gh auth status` succeeds — resolving the action SHA pins queries the GitHub API.</precondition>
  <action>
**Do not modify `.github/workflows/ci.yml`.** Its diff staying empty is the argument that the
fork-facing workflow is still safe (design decision 6).

Generate the baseline that keeps the repo's own attack corpora out of the Security tab without
excluding them from scanning:

    cargo run --quiet --release -- check . --exclude '.planning/**' \
      --write-baseline .github/code-scanning-baseline.json

Commit it. Verified on this checkout: 51 entries, and `check .` over the committed file finds
nothing because ADR-002 stores `sha256:<hex>` and never the payload.

Create `.github/workflows/code-scanning.yml`:
- a header comment in the same voice as `ci.yml`'s RUNNER POLICY block, stating that this workflow
  holds `security-events: write` and is therefore restricted to triggers a fork cannot fire, that
  it must never gain `pull_request`, and that self-hosted runners remain impossible on this public
  repo under `allows_public_repositories: false`;
- `on: push: { branches: [main] }`, a weekly `schedule`, and `workflow_dispatch`;
- `permissions: { contents: read, security-events: write }`;
- `runs-on: ubuntu-latest`;
- SHA-pin every action with a trailing `# vX.Y.Z` comment, matching `ci.yml`'s style. Reuse the
  pins already in `ci.yml` for `actions/checkout` and `dtolnay/rust-toolchain`; resolve
  `github/codeql-action/upload-sarif` yourself, e.g.
  `gh api repos/github/codeql-action/git/ref/tags/v3 --jq .object.sha`, and pin what you get;
- build with `cargo build --release --locked`, then run
  `check . --exclude '.planning/**' --baseline .github/code-scanning-baseline.json --format sarif`
  redirected to `results.sarif`;
- handle the exit code explicitly, with a comment saying why. `0`, `1` and `2` are all normal scan
  outcomes and must proceed to the upload — the alert IS the output here, and the gate that blocks
  code lives in the pre-commit hook and `ci.yml`. Anything else is a crash and must fail the job
  with a `::error::` line. Do NOT reach for `continue-on-error: true`: PR #59's review found
  exactly that turning a gate decorative, and it would swallow a panic here;
- upload with `github/codeql-action/upload-sarif`, `sarif_file: results.sarif` and
  `category: injection-scanner` so the analysis does not collide with any other tool's;
- a comment recording how to regenerate the baseline when `examples/` or `patterns/` change, and
  that stale entries surface as notes on stderr rather than failures.

`docs/adr/ADR-003-sarif-output.md`, using `.claude/skills/pr-artifacts/adr_template.md` and
following ADR-002's register. Required by the skill: "output format contracts (JSON, SARIF)" is an
explicit trigger. Record, with the reasoning and not just the conclusion: D-1 and why the three
withheld arrays are not results; the 4-to-3 severity mapping and `security-severity` rather than
`rank`; the fingerprint reuse and the occurrence ordinal that makes it non-colliding; why
validation is structural plus the upload rather than a vendored schema or a new dev-dependency; why
`ci.yml` does not get `security-events: write`; and why the self-scan is baselined rather than
excluded. Alternatives Considered should carry the ones actually rejected — SARIF `suppressions`
for baselined findings (per D-1), `rank`, `jsonschema`, blanket excludes, and a shared
`OutputFormat` for `rules`.

`README.md`: add `### SARIF output` under `## Usage`, next to `### JSON output`. Show the command,
say what a result is and what it deliberately is not (one result per reported finding; suppressed,
low-confidence and baselined findings stay in `--format json`), give the severity mapping table
including `security-severity`, and show the code-scanning upload as a copyable workflow fragment
with the fork-trigger warning attached. Mention that `rules` has no SARIF form and why. Keep the
existing `## Output Examples` section consistent if it enumerates formats.

`TODO.md`: tick `#5` in the Phase 4 list.

Leave `.planning/REQUIREMENTS.md` and `.planning/STATE.md` to the orchestrator.

Then run the `pr-artifacts` checklist end to end and put its verification report in the PR body,
with `Closes #5`.
  </action>
  <verify>
    <automated>python3 -c "import json;d=json.load(open('.github/code-scanning-baseline.json'));print('entries',len(d['entries']));assert len(d['entries'])>0"</automated>
    <automated>cargo run --quiet -- check .github/code-scanning-baseline.json; test $? -eq 0 &amp;&amp; echo "baseline file is inert"</automated>
    <automated>python3 -c "
import re
raw = open('.github/workflows/code-scanning.yml').read().splitlines()
# Comment lines are stripped before the negative checks: the header comment has
# to be free to NAME the triggers and runner labels it forbids, or the guard
# invalidates itself the moment the rationale is written down.
code = '\n'.join(l for l in raw if not l.lstrip().startswith('#'))
assert 'security-events: write' in code, 'upload needs the write scope'
for bad in ('pull_request', 'self-hosted', 'orangepi', 'arc-runner'):
    assert bad not in code, 'forbidden in executable YAML: ' + bad
print('trigger and runner policy ok')
"</automated>
    <automated>git diff --exit-code .github/workflows/ci.yml &amp;&amp; echo "ci.yml unchanged — the fork-facing workflow gained no scope"</automated>
    <automated>test -f docs/adr/ADR-003-sarif-output.md &amp;&amp; grep -q 'SARIF output' README.md &amp;&amp; echo "ADR and README present"</automated>
  </verify>
  <done>
`.github/workflows/code-scanning.yml` exists, holds `security-events: write`, and is reachable only
from `push: main`, `schedule` and `workflow_dispatch`. `.github/code-scanning-baseline.json` is
committed and scans clean. `ci.yml` has an empty diff. ADR-003, the README section and the TODO
tick are in place, and the PR body carries the `pr-artifacts` verification report with
`Closes #5`. Committed.
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| scanned document → SARIF document | The adversary authors `matched_text`, which reaches the SARIF `message`, `partialFingerprints` input and (via the file name) the `artifactLocation.uri` |
| repository → GitHub code scanning | The uploaded document decides which alerts open, stay open, or close |
| fork pull request → CI token | A fork can run `cargo test`, and therefore fork-authored build scripts, inside `ci.yml` |

## STRIDE Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation Plan |
|-----------|----------|-----------|----------|-------------|-----------------|
| T-uor-01 | Tampering | `partialFingerprints` | medium | mitigate | Occurrence ordinal within `(file, ruleId, digest)`, so two identical payloads in one file cannot collapse into a single alert that closes while one payload remains (design decision 3b) |
| T-uor-02 | Elevation of Privilege | `ci.yml` token scope | high | mitigate | `security-events: write` goes in a separate workflow with no fork-firable trigger; `ci.yml` is not modified, and a verify gate asserts its diff is empty |
| T-uor-03 | Denial of Service (of signal) | code-scanning alert list | medium | mitigate | The self-scan is baselined, not blanket-excluded, so `patterns/` and `examples/` stay in scope and a poisoned pattern PR still raises an alert |
| T-uor-04 | Repudiation | rules-only SARIF upload | medium | mitigate | `rules` gets its own format enum; a run with an empty `results` array cannot be produced from the rule listing and so cannot silently close open alerts |
| T-uor-05 | Information Disclosure | committed baseline | low | accept | ADR-002 already stores `sha256:<hex>` and never the payload; verified inert on this checkout |
| T-uor-06 | Tampering | `artifactLocation.uri` | low | mitigate | One percent-encoding rule over the whole path, so a crafted filename cannot inject URI structure |
| T-uor-SC | Tampering | npm/pip/cargo installs | high | mitigate | No new runtime or dev dependency is added by this plan — `jsonschema` and a vendored schema were both rejected (design decision 5), so the package-legitimacy surface is unchanged |
</threat_model>

<verification>
- `cargo test --locked` green across all test binaries, including the two new files.
- `cargo fmt --all -- --check` and `cargo clippy --all-targets --locked -- -D warnings` clean.
- `check <fixture> --format sarif` parses as JSON, declares `2.1.0`, and every result resolves to a
  rule by both `ruleId` and `ruleIndex`.
- The three withheld arrays each proven non-empty in a run whose `results` array is empty.
- `--format json` and `rules --format json` key sets pinned; `src/reporter.rs` has an empty diff.
- `.github/workflows/ci.yml` has an empty diff.
- History stays strictly linear — this repo rebase-merges. If any part of this runs in an isolated
  worktree, check for a merge commit before committing and flatten it (STATE.md, 2026-08-25).
</verification>

<success_criteria>
- CLI-04 is met on both halves: a SARIF 2.1.0 document that GitHub's ingest accepts, and a workflow
  that uploads it to code scanning.
- D-1 holds at the one place results are built: `ScanReport.matches` only.
- D-2 holds: the work stays stacked on `feat/cli-08-baseline`, and the PR is retargeted to `main`
  before #79's branch is deleted.
- No new dependency, no `unwrap()`, no self-hosted runner, no write scope on a fork-firable trigger.
</success_criteria>

<output>
Commit per task. Open the PR with `Closes #5` and the `pr-artifacts` verification report.
</output>