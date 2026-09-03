# Phase 4: MCP & Tool-Description Poisoning (CAT-02, PI060-PI069) — Pattern Map

**Mapped:** 2026-09-03
**Files analyzed:** 17 (files Phase 4 must create or touch)
**Analogs found:** 17 / 17 — every one has a direct Phase 3 counterpart; PR #109 (`4e447ed..0cc18f3`,
38 files, +2709/-55) is the template range for the whole phase.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog (Phase 3) | Match Quality |
|---|---|---|---|---|
| `patterns/core/mcp-tool-poisoning.yaml` | config (pattern library) | transform (regex match) | `patterns/core/tool-permission-abuse.yaml` | exact |
| `src/patterns/mod.rs` (2-line edit) | registration | CRUD (load) | same file, `tool-permission-abuse.yaml` entry | exact |
| `tests/corpus/attack/mcp-tool-poisoning.md` | test fixture | batch | `tests/corpus/attack/tool-permission-abuse.md` | exact |
| `tests/corpus/attack/structural/06-*.md` … (N files) | test fixture | batch | `tests/corpus/attack/structural/01-05-*.md` | exact |
| `tests/corpus/attack/structural/README.md` (edit) | docs | — | same file (also carries WR-02 debt) | exact |
| `tests/corpus/attack/README.md` (edit) | docs | — | same file, CAT-01 addition | exact |
| `tests/corpus/clean/mcp/*.md` (registry sample, D-06.3) | test fixture | batch | `tests/corpus/clean/mcp-setup-guide.md` | exact |
| `tests/corpus/clean/<boundary-specimen>.md` (hand-written, D-06.4) | test fixture | batch | `tests/corpus/clean/sandbox-bypass-runbook.md`, `settings-permissions-reference.md` | exact |
| `tests/pattern_test.rs` (append) | test | request-response | `test_pi050_wildcard_tool_grant` etc., lines ~1183-1245+ | exact |
| `tests/pattern_relaxed_control_test.rs` | test | event-driven (mutation gate) | same file, GATE-05 harness (`fires`, `shipped_scanner`) | exact — automatic |
| `tests/pattern_policy_test.rs` | test | batch | same file, `LEGACY_UNTESTED`/`case_counts()` | exact — automatic |
| `tests/recall_test.rs` (edit `EXPECTED`) | test | batch | same file, `EXPECTED` rows + `STRUCTURAL_CATEGORY` | exact |
| `tests/frontmatter_test.rs` (maybe edit) | test | request-response | same file, `wildcard_grant_probe` (lines 256-351) | conditional (see below) |
| `PATTERNS.md` (Categories table row) | docs | — | same file, PI050-PI059 row, line 123 | exact |
| `README.md` (Pattern Categories + recall table) | docs | — | same file, lines 254-306 | exact |
| `CHANGELOG.md` (Unreleased section) | docs | — | Phase 3's `[0.1.0]`/`[Unreleased]` entries | exact |
| `docs/PATTERN-CATALOGUE.md` | docs (generated) | transform | same file — regenerated, never hand-edited | exact |
| `.github/code-scanning-baseline.json` | config (generated) | transform | same file — regenerated via `--write-baseline` | exact |
| `examples/README.md` + `examples/mcp-tool-poisoning-attack.md` (if following the D-06 discipline) | docs/example | — | `examples/README.md` +1 line, `examples/tool-permission-abuse-attack.md` | exact |
| `scripts/gate03-sweep.sh` | script | batch | unchanged unless MCP-specific sweep paths are added (D-06.1/.2 sources) | possible edit only |

## Pattern Assignments

### 1. `patterns/core/mcp-tool-poisoning.yaml` (config)

**Analog:** `patterns/core/tool-permission-abuse.yaml` (full file — 309 lines added in PR #109;
read in full, it is the schema template).

**Header/category-comment pattern** (top of file, lines 1-32):
```yaml
category: tool_permission_abuse
default_severity: HIGH
# CAT-01 (#33): documents that widen the agent's OWN authority -- the agentic
# equivalent of privilege escalation. ...
```
Phase 4 copies this shape exactly:
```yaml
category: mcp_tool_poisoning
default_severity: HIGH   # D-03: config-hygiene signals override to MEDIUM per-pattern
# CAT-02 (#34): instructions hidden in an MCP tool's own `description` field,
# read by the model on every call and never surfaced in a UI. ...
```
A block comment must state the false-positive rationale up front the same way — this file's
comment explains the `path = value` projection semantics (unquoted strings, `[index]` suffixes on
arrays, `permissions.defaultMode` as a bare scalar path) because PI050-PI052 are `scope: frontmatter`.
CAT-02's structural patterns (if any target `mcpServers.*.command`/`args`/`url` fields) need the
same kind of up-front note; the injection patterns matching `description` prose need D-01's
second-person-address rationale stated instead, mirroring how the CAT-01 header explains the
`.allow`/`.deny` trap (D-06a) before the first pattern.

**Per-pattern structure** (e.g. PI050, lines 34-52):
```yaml
  - id: PI050
    example: |
      ---
      allowed-tools: "*"
      ---
    counter_example: |
      ---
      allowed-tools:
        - Read
        - Grep
      ---
    relaxed_pattern: "^(?:allowed-tools|tools)(?:\\[\\d+\\])?\\s*="
    name: wildcard-tool-grant
    severity: CRITICAL
    scope: frontmatter
    pattern: "^(?:allowed-tools|tools)(?:\\[\\d+\\])?\\s*=\\s*(?:\\*|[A-Za-z][A-Za-z0-9_]*\\(\\s*\\*\\s*\\))\\s*$"
    description: "A skill or manifest grants unrestricted tool access (a bare `*` or `Bash(*)`) in its own frontmatter"
    remediation: "Scope the grant to the specific tools, and specific arguments, the skill actually needs."
    tags: [tool-permission-abuse]
```
Every `PI060`+ pattern must carry the same 9-10 fields, in this same order, and **must** carry
`relaxed_pattern` (D-09/ADR-004 — mandatory for `PI050`+, so binding here too). `scope: frontmatter`
is used only for structural patterns (config-hygiene: unpinned `npx -y`, unknown host, `http://`
transport, `bypassPermissions`-style flags inside an MCP config). Prose patterns targeting a tool's
own `description` text carry **no** `scope` field (defaults to prose scanning) — same as
CAT-01's non-`PI050-052` arms.

**Severity override per D-03**: config-hygiene patterns need an explicit `severity: MEDIUM` field
(the file's `default_severity: HIGH` covers the injection arms only) — copy the per-pattern
`severity:` override mechanism straight from PI050's `severity: CRITICAL` line; same field, opposite
direction.

### 2. Registration — `src/patterns/mod.rs`

**Analog** (lines 21-22 for CAT-01; full const block lines 16-22):
```rust
const TOOL_PERMISSION_ABUSE_YAML: &str =
    include_str!("../../patterns/core/tool-permission-abuse.yaml");
```
plus its entry in the `load_embedded_patterns()` vec (not shown above but confirmed present —
grep matched `load_embedded_patterns` at line 28/131 with the six `*_YAML` consts feeding it).

Phase 4 copies the exact two-line shape:
```rust
const MCP_TOOL_POISONING_YAML: &str =
    include_str!("../../patterns/core/mcp-tool-poisoning.yaml");
```
plus the matching push/entry inside `load_embedded_patterns()`. `tests/pattern_test.rs`'s
`test_tool_permission_abuse_category_is_loaded` (line 1183) is the exact template for a
`test_mcp_tool_poisoning_category_is_loaded` — Trap 5 from 03-05-PLAN.md ("a category file absent
from `load_embedded_patterns()` ships as dead YAML and no other test notices") applies identically
here; this is a **required new test**, not automatic.

### 3. Attack corpus

**Flat prose analog:** `tests/corpus/attack/tool-permission-abuse.md` (26 lines) — one payload per
line, blank lines and `#`-prefixed lines ignored, header comment states sourcing discipline
("written from the threat model... never from a regex, since no PIxxx pattern exists yet").
`tests/corpus/attack/mcp-tool-poisoning.md` must open with the same disclaimer block, sourced from
`docs/DETECTION-BACKLOG.md` Part 2 and issue #34, and must exist **before** any `PI060`+ pattern is
written (D-04 precedent: 0/N pre-pattern baseline pinned first).

**Structural analog:** `tests/corpus/attack/structural/01-05-*.md` — one whole-file payload per
file, opening fence must be the file's literal first line (no leading `#` comment — `extract_delimited`
reads the fence via `lines.next()`), rationale goes in the README table or after the closing fence.
Phase 4's structural payloads (numbered continuing from `06-`) are for MCP manifests where the
whole document must parse as JSON/YAML for the projection to exist — e.g. an `.mcp.json` with a
poisoned `description` field inside a `scope: frontmatter`-projected structural pattern.

**Difference to report:** the flat file is for prose patterns scanned line-by-line via `payloads()`
(`tests/recall_test.rs`, splits on `\n`); the structural directory is for patterns that need a real
parseable document (`structural_payloads()` reads each file whole via `fs::read_to_string`, never
line-split). Use flat for the second-person-address injection arms (D-01); use structural for any
config-hygiene / frontmatter-scoped arm (D-03, D-04's heuristic shadowing patterns may go either way
depending on whether they need real JSON structure or fire on prose describing a tool).

**`structural/README.md` currently documents only 1 of 5 payloads (WR-02, open finding, not to be
silently fixed as a side effect — carry it forward or file it, per the CONTEXT.md carried-over
list).** Phase 4 adding new structural payloads makes this worse unless addressed; note in the
plan whether WR-02 is folded in or explicitly deferred again.

### 4. Clean corpus

**Analogs — six Phase 3 additions**, all under `tests/corpus/clean/`:
- `mcp-setup-guide.md` (42 lines) — **already MCP-shaped**; Phase 4 must re-run this specimen
  against every new `PI060`+ pattern before shipping (it is the nearest legitimate document per the
  SKILL.md table's method — "ask what the nearest legitimate document looks like"). Do not edit it
  to make a pattern pass; if it starts firing, narrow the pattern.
- `settings-permissions-reference.md`, `cli-flag-reference.md`, `narrow-allowed-tools-skill.md`,
  `sandbox-bypass-runbook.md` (69 lines — the largest, boundary-heavy specimen), `settings-deny-list.md`
  (17 lines — the D-06a `.deny` trap specimen).

Phase 4's D-06.4 obligation ("author benign manifests that deliberately sit on the boundary —
imperative, multi-step, file-reading descriptions that are entirely legitimate") is structurally
identical to how `sandbox-bypass-runbook.md` and `settings-permissions-reference.md` were authored
in Phase 3 — same role, same purpose, direct analog. D-06.3 (public registry sample) is new in kind
— no Phase 3 analog exists for a vendored third-party corpus; closest is still `mcp-setup-guide.md`
in spirit but plan for a new `tests/corpus/clean/mcp/` subdirectory with its own provenance note
(README or header comments per-file), since nothing in Phase 3 modeled a licensing/provenance
obligation.

### 5. Tests

| Test file | Plugs in automatically? | What Phase 4 must add |
|---|---|---|
| `tests/pattern_test.rs` | **No** | One `#[test] fn test_pi06x_<name>()` per pattern using `assert_positives`/`assert_negatives` (≥3/≥2, SKILL.md policy), plus the category-loaded guard test (see §2 above). Template: lines 1183-1245 (`test_tool_permission_abuse_category_is_loaded`, `test_pi050_wildcard_tool_grant`, `test_pi051_wildcard_permission_allow`). |
| `tests/pattern_relaxed_control_test.rs` (GATE-05) | **Yes, automatic** | The harness (`fires`, `shipped_scanner`, the relaxed-scanner builder) iterates every pattern carrying `relaxed_pattern`. No explicit per-category registration; a new `PI060`+ pattern with `relaxed_pattern` set is picked up mechanically. Confirm this by checking the mutation-pairing loop is over `load_embedded_patterns()` output, not a hardcoded id list (matches file's own self-description: "iterating an empty collection" is the failure mode it guards against). |
| `tests/pattern_policy_test.rs` (GATE for ≥3/≥2 case counts) | **Yes, automatic** | `case_counts()` scans all `tests/*.rs` source text for `assert_positives`/`assert_negatives` call sites and builds counts per id from source text — no explicit registration. `LEGACY_UNTESTED` must **not** gain new entries; every `PI060`+ id must comply from day one. |
| `tests/recall_test.rs` (GATE-02, exact-pinned) | **No** | `EXPECTED` (line 55) needs two new rows: `("mcp-tool-poisoning", hit, total)` for the flat corpus and a `STRUCTURAL_CATEGORY`-style row if Phase 4 introduces its own structural subdirectory naming (currently `tool-permission-abuse-structural` is the sole `STRUCTURAL_CATEGORY` const at line 31 — check whether Phase 4 reuses the same structural directory with mixed categories per-payload, or needs its own constant/row; measure, don't assume). `categories()` (line 143) auto-discovers `tests/corpus/attack/*.md` files, so the flat file itself needs no registration beyond existing, but its `EXPECTED` row is mandatory or `category_names_match_expected` (~line 365) fails loudly by design. Update the README recall table in the **same commit** (enforced by comment at line 352). |
| `tests/frontmatter_test.rs` (GATE for structural projection) | **Conditional** | Only needed if CAT-02 relies on `PatternScope::Frontmatter` for `mcpServers` JSON blocks or similar — per CONTEXT.md's open discretion item ("whether `PatternScope::Frontmatter`'s existing projection already covers standalone `.mcp.json`/`mcpServers` blocks... research must measure this, not assume"). If yes, add probes analogous to `wildcard_grant_probe` (lines 256-279) and the projection-agnostic tests (lines 64-142) for the new key shapes. If the existing projection already covers it, no edit needed — but this must be *measured*, not assumed, per D-05's discretion note. |

### 6. Docs

**`PATTERNS.md` Categories table** (line 123 is the CAT-01 row; table starts line 116):
```
| Tool and Permission Abuse | PI050-PI059 | HIGH (structural patterns PI050-PI052 override to CRITICAL) |
```
Phase 4 adds directly beneath it:
```
| MCP Tool-Description Poisoning | PI060-PI069 | HIGH (config-hygiene signals override to MEDIUM, D-03) |
```
**This row was the WR-01 miss in Phase 3 — the row was added late in a follow-up commit
(`0cc18f3 docs(patterns): regenerate catalogue/baseline and add WR-01 category row`), not in the
same commit as the patterns.** Phase 4 must add its Categories row in the *same* commit/plan as the
first `PI060` pattern ships, not as a cleanup afterthought.

**`README.md`** — two places, both from PR #109's diff (+48 lines):
- Pattern Categories table (lines 254-267 currently) — add an `MCP Tool-Description Poisoning` row
  with pattern count and default severity, same shape as the `Tool & Permission Abuse` row (line
  266).
- Recall table (lines 288-306) — add rows for the new corpus categories and update the **Total**
  row's aggregate count/percentage. Follow the italic footnote convention below the table (see
  lines ~298-306) documenting which plan closed which sub-row, mirroring "Plan 05 shipped the
  structural half... Plan 06 then shipped the prose half."
- If a behaviour-change note is warranted (Phase 3's D-12 callout at line ~298, "a wildcard tool
  grant... is now a CRITICAL finding, not merely absent detection"), Phase 4's D-03 MEDIUM-banding
  decision for config-hygiene signals is the natural candidate for the same kind of callout, since
  it also changes what a consumer's CI sees on upgrade.

**`CHANGELOG.md`** — analog is the `[0.1.0]`/CAT-01 entries structure (Added/Changed/Security
subsections under a version or `[Unreleased]` heading). Add pattern IDs and a one-line summary per
new capability, following the exact bullet style already in the file (`tail -60` above shows the
`[0.0.3]` entry as the closest template for phrasing).

**`docs/PATTERN-CATALOGUE.md`** — **generated only**, never hand-edited (SKILL.md is explicit).
Regenerate with:
```bash
cargo run --release -- rules --format markdown > docs/PATTERN-CATALOGUE.md
```
`tests/catalogue_test.rs` (not separately listed above but implied by SKILL.md step 3) fails CI if
this is stale — check it exists in this repo's test suite the same way `catalogue_test` is
referenced in SKILL.md line 16.

**`.github/code-scanning-baseline.json`** — regenerate whenever `examples/` or `patterns/` change:
```bash
cargo run --release -- check . --exclude '.planning/**' --write-baseline .github/code-scanning-baseline.json
```
PR #109 touched this file (+265 lines) as a direct consequence of adding `patterns/core/tool-permission-abuse.yaml` and `examples/tool-permission-abuse-attack.md` — expect the same for Phase 4.

## Shared Patterns

### The `relaxed_pattern` / GATE-05 mutation-pairing contract
**Source:** `docs/adr/ADR-004-relaxed-pattern-false-positive-control.md`, enforced by
`tests/pattern_relaxed_control_test.rs` + `tests/pattern_policy_test.rs::every_pi05x_pattern_carries_a_relaxed_pattern`.
**Apply to:** every `PI060`-`PI069` pattern, no exceptions (the rule scope is `PI050` and above, so
`PI060`+ inherits it unconditionally). This is automatic enforcement, not a task the planner assigns
per-file, but each new pattern's YAML entry must supply the field or CI fails.

### Second-person/agent-directed address discriminator (D-01)
**Source:** CONTEXT.md D-01, modeled explicitly on the PI021 possessive precedent
(`your system prompt` vs `the system prompt`) described in SKILL.md's "#95" entry and the
`exfiltration` row of its "nearest legitimate document" table.
**Apply to:** every prose-scanning `PI060`+ injection pattern. The regex construction technique
(require the addressed-to-the-model marker, not merely imperative mood) is the same technique that
made `exfiltration` and `instruction_injection` category rewrites survive their false-positive
gates — copy the *method*, not any specific regex.

### Negation handling (D-02 / CR-01)
**Source:** `.planning/phases/03-tool-permission-abuse-cat-01-33/03-REVIEW.md` §CR-01 and the fix
commit `1509091 fix(patterns): close CR-01 negation blindness in PI053/PI056/PI057`.
**Apply to:** any `PI060`+ pattern whose attack sentence could plausibly carry an in-clause negator
("do not report this to the user" is affirmative-attack shaped but so is its negated benign form).
Fix in the regex's clause anchoring / enumerated filler set, never as an engine-side guard.

### Self-scan cleanliness
**Source:** SKILL.md "Scan the whole repo, not just the corpus" — the exact `cargo run ... check .
--exclude '.planning/**'` command and expectation of `[]`.
**Apply to:** every new doc file this phase touches (`PATTERNS.md`, `README.md`, `CHANGELOG.md`,
`docs/PATTERN-CATALOGUE.md`) plus this very `04-PATTERNS.md` — no verbatim payload text outside
`examples/`, `patterns/`, `tests/`, `tools/`. This document itself complies (payloads referenced by
path/description only).

## No Analog Found

None. Every file class Phase 4 needs was already exercised end-to-end by Phase 3/PR #109. The one
genuinely new *kind* of artifact is the vendored public-registry clean corpus (D-06.3,
`tests/corpus/clean/mcp/`) — classified above as an extension of the existing clean-corpus pattern
rather than a gap, since the mechanism (files under `tests/corpus/clean/`) is unchanged; only the
provenance-tracking obligation is new.

## Report Also — Answers

**Automatic vs. explicit-registration test files:**
- Automatic (no per-category registration needed): `tests/pattern_relaxed_control_test.rs`,
  `tests/pattern_policy_test.rs`, `tests/recall_test.rs`'s `categories()` file-discovery (but its
  `EXPECTED` table is NOT automatic — see below).
- Explicit registration required: `src/patterns/mod.rs` (const + load list), the category-loaded
  guard test in `tests/pattern_test.rs`, `tests/recall_test.rs`'s `EXPECTED` array (line 55) and its
  paired README recall table, `tests/pattern_test.rs` positive/negative cases per pattern id,
  `PATTERNS.md` Categories table row (WR-01's exact miss — do not repeat), `tests/frontmatter_test.rs`
  probes (conditional on whether new structural key shapes are needed).

**Derived-artifact commands (exact):**
```bash
# Catalogue regeneration
cargo run --release -- rules --format markdown > docs/PATTERN-CATALOGUE.md

# Code-scanning baseline regeneration (after touching examples/ or patterns/)
cargo run --release -- check . --exclude '.planning/**' --write-baseline .github/code-scanning-baseline.json

# Recall re-measurement
cargo test --test recall_test

# Whole-repo self-scan check (expect [])
cargo run --release -- check . --exclude '.planning/**' --format json \
  | python3 -c "import json,sys; print([(r['file'],m['line'],m['pattern_id']) \
      for r in json.load(sys.stdin) for m in r['matches'] \
      if not any(k in r['file'] for k in ('examples/','patterns/','tests/','tools/'))])"
```

**Easy-to-forget files, inferred from PR #109's actual 38-file diff (`4e447ed..0cc18f3`):**
- `examples/README.md` (+1 line) and a new `examples/<category>-attack.md` file — easy to skip
  since it's not core to detection logic, but Phase 3 added both.
- `tests/case_sensitivity_test.rs` and `tests/raw_only_test.rs` each got a **1-line** edit — almost
  certainly a shared constant/list that enumerates all category YAML files or pattern counts;
  confirm whether Phase 4 needs the same 1-line touch (check what changed at those exact lines
  before assuming — likely a total-pattern-count assertion).
- `tests/pattern_validation_test.rs` (+46 lines) — schema/field validation tests for the new YAML;
  not mentioned in the read_first docs but present in the Phase 3 diff — likely required if
  `mcp-tool-poisoning.yaml` introduces any field usage not already covered (e.g., first use of a
  particular `scope` value, or first MEDIUM severity override in a structural pattern).
- `.claude/skills/pattern-library/SKILL.md` itself (+26/-? lines) — Phase 3 updated its own binding
  contract (the D-09 `relaxed_pattern` requirement was introduced there). Check the CONTEXT.md
  decisions for anything Phase 4 needs to add to this contract (e.g., D-01's second-person
  discriminator or D-03's MEDIUM banding might warrant a documented convention here too).
- `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md` — all three were touched
  in Phase 3's closing commits; expect the same tracking updates when Phase 4 closes.
- `docs/DETECTION-BACKLOG.md` (+/-4 lines in Phase 3) — likely struck-through/marked-done entries
  for the patterns just shipped; Part 2 (CAT-02 source material) will need the same treatment.
- `scripts/gate03-sweep.sh` (+218 lines in Phase 3 — this *was* the file's creation) — Phase 4 does
  **not** need to recreate this; it already exists and is reused. Only touch it if D-06's sweep
  sources (plugin caches, `07-mcp-hub`, registry sample, hand-written) need new directory arguments
  wired in; check current script content before assuming it needs edits (WR-03 debt: its helper
  functions lack `local` declarations — do not fix incidentally unless explicitly scoped in).

## Metadata

**Analog search scope:** `patterns/core/`, `src/patterns/mod.rs`, `tests/corpus/attack/`,
`tests/corpus/attack/structural/`, `tests/corpus/clean/`, `tests/*.rs`, `PATTERNS.md`, `README.md`,
`CHANGELOG.md`, `docs/`, `examples/`, `.github/code-scanning-baseline.json`,
`.claude/skills/pattern-library/SKILL.md`. Cross-referenced against `git diff --stat 4e447ed..0cc18f3`
(the full Phase 3 PR #109 range, 38 files).
**Files scanned:** ~20 read/greped directly; 38-file diff stat cross-checked for completeness.
**Pattern extraction date:** 2026-09-03.
