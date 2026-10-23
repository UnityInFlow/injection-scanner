# Phase 3: Tool & permission abuse (CAT-01, #33) - Research

**Researched:** 2026-09-01
**Domain:** Prompt-injection pattern authoring (regex + structural projection) in a Rust static scanner; no LLM involved
**Confidence:** MEDIUM-HIGH — code-level findings are VERIFIED against the repo; threat-model phrasing is a mix of CITED (cross-checked, independent secondary sources) and ASSUMED (no primary-source confirmation found)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Corpus shape for the structured half**
- **D-01:** The recall harness gains a **second collection mode**: `tests/corpus/attack/structural/`
  where each *file* is one whole payload, `---` fenced and realistic. The existing line-oriented
  files are unchanged. Required because `tests/recall_test.rs::payloads()` splits on `\n` and scans
  each line as its own document, so a frontmatter payload cannot be expressed at all today.
  — **Reversibility:** costly.
- **D-02:** `EXPECTED` gains **two rows**, `("tool-permission-abuse", n, m)` and
  `("tool-permission-abuse-structural", n, m)`. A single combined row can stay correct while the
  structural half silently goes to zero. The README may still present one combined figure per
  category.
- **D-03:** **No prose/structural ratio is set in advance.** Write all 12 payloads from the threat
  model, then record whatever split results.
- **D-04:** **Corpus lands first, with a measured baseline.** Commit the 12 payloads alone and run
  them against the current 48-pattern binary *before any PI05x exists*; commit that number. The
  patterns land as a second commit and the delta is the evidence.
- **D-05:** `categories()` currently filters `p.is_file()`, so a new `structural/` subdirectory
  would be **silently skipped** — total stays 60 and the suite still passes. The harness change
  must assert the structural payloads were actually collected.

**False-positive control**
- **D-06:** Four new documents join `tests/corpus/clean/` **before** the patterns land: (1) a skill
  file with real, narrow `allowed-tools`; (2) a settings/permissions reference doc naming
  `--dangerously-skip-permissions`, `bypassPermissions`, `settings.json` descriptively; (3) an
  operator runbook instructing a human to bypass inside a disposable container; (4) an MCP/agent
  setup guide legitimately saying "add this to your settings.json".
- **D-07:** GATE-05 is enforced by an **automated pairing test in CI**. For each new pattern: assert
  the shipped pattern stays **off** its control document, *and* that a documented **relaxed** form
  of the same pattern **does** match it.
- **D-08:** The relaxed form lives in a **new optional field on the `Pattern` schema**, beside
  `example` and `counter_example`, following the `raw_only` precedent.
  — **Reversibility:** one-way — `patterns/` is a documented community-contribution surface and
  `deny_unknown_fields` is enforced.
- **D-09:** The field is **required for new patterns (PI050+)**, enforced by extending the
  per-pattern test policy ratchet. The existing 48 stay exempt.

**The spec-linter S005 boundary**
- **D-10:** A frontmatter-scoped `PI05x` **does** fire on a file's own wildcard grant, accepting
  overlap with `spec-linter` S005. Justification: **provenance, not phrasing**.
- **D-11:** **`ROADMAP.md`'s Phase 3 criterion is wrong and must not be inherited.** Its current text
  ("Complements spec-linter S005... this detects a document persuading someone to widen them")
  would delete the structural half and leave ENG-01 inert; correct it to D-10's wording as part of
  this phase.
- **D-12:** The new CRITICAL **ships** and is documented as a deliberate behaviour change; consumer
  CI (`spec-ci-plugin`) will go green→red on upgrade for a wildcard grant. `--baseline` remains the
  escape hatch but is not a gate on this decision.
  — **Reversibility:** one-way.
- **D-13:** CAT-01 claims **`allowed-tools`, `tools`, `permissions`** from ENG-01's projected key
  set. `mcpServers` → CAT-02; `hooks` → CAT-03. `model` and `system` stay unclaimed.

**Dangerous-command items**
- **D-14:** `sudo`, `rm -rf`, `chmod 777`, `curl … | sudo sh` are **dropped from CAT-01** — payload
  *execution*, not authority *widening*.
- **D-15:** `PI028 pipe-to-shell` already exists (`patterns/core/exfiltration.yaml`) and has a
  measured gap (`curl x.sh | sudo sh` does not match). Own issue, own sweep, **not this PR**.
- **D-16:** **"10 patterns" is no longer the target.** `PI050`–`PI059` is an ID range, not a quota.
  ROADMAP criterion becomes "12 new corpus payloads; pattern count as the threat model requires,
  within `PI050`–`PI059`".
- **D-17:** "Instructions to disable a hook or a guardrail" **stays in CAT-01** as prose (removing a
  control vs. CAT-03's installing/maintaining presence).

### Claude's Discretion

None — every question in the discussion was answered explicitly. Both offered "You decide" options
(EXPECTED row granularity, CAT-01/CAT-03 hook split) were declined in favour of an explicit choice.

### Deferred Ideas (OUT OF SCOPE)

- `PI028` `| sudo sh` gap — own issue, own sweep, under GATE-04.
- Bare dangerous-command detector (`sudo`, `rm -rf`, `chmod 777` in a skill body) — v0.3.0 backlog
  candidate.
- Backfill the mutation-tested FP-control field across the existing 48 patterns — own issue.
- `model:` / `system:` frontmatter overrides — unclaimed by any category.
- The two pre-existing `docs/PATTERN-CATALOGUE.md` sweep findings (lines 73, 902) — standing,
  predates this milestone, no issue yet.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CAT-01 (#33) | `PI050`–`PI059` tool & permission abuse: wildcard tool grants, `--dangerously-skip-permissions`/`bypassPermissions`, edit-settings.json / disable-a-hook prose. Depends on ENG-01 for the frontmatter half. | §Threat Model Catalogue (attack shapes + real phrasing sources); §ENG-01 Arming (exact projection shapes to match); §Real Agent-Config Shapes (what the four new clean-corpus docs must contain) |
| GATE-01 | 12 corpus payloads, from the threat model, never derived from patterns | §Threat Model Catalogue supplies the raw phrasings a planner can turn into payloads without consulting the regex |
| GATE-02 | Recall pinned exactly | §Harness Change specifies the exact `EXPECTED` row shape (D-02) and the sequencing with D-04's baseline commit |
| GATE-03 | ~1,300-file third-party sweep | §GATE-03 Sweep Reproducibility — verified NOT reproducible from this repo; documented as a manual, machine-local procedure |
| GATE-04 | One category per PR | Confirmed scope boundary: D-13's claimed-key set, D-14/D-15 exclusions keep this PR to CAT-01 only |
| GATE-05 | Mutation-tested FP control | §Schema Field (D-08 `relaxed_pattern`) + §Harness Change give the mechanism: a second regex field and a CI test pairing it with `counter_example`/clean-corpus control docs |
</phase_requirements>

## Summary

CAT-01 is two independent detection problems sharing one PR: a **structural** half that arms
ENG-01 for the first time (a `scope: frontmatter` pattern matching a wildcard tool grant in parsed
YAML/TOML/JSON) and a **prose** half that is ordinary regex work in this codebase's established
idiom (verb × object matrices, false-positive control proven by mutation, not assertion).

The structural half is higher-risk than it looks. ENG-01's projection renders parsed configuration
as `path = value` text, and the *shape* of that text differs by how the source YAML was written:
`allowed-tools: "*"` projects as `allowed-tools = *`, but the far more common block-sequence form
this project's own skills actually use —
```yaml
allowed-tools:
  - Read
  - Bash(*)
```
— projects as **two lines**, `allowed-tools[0] = Read` and `allowed-tools[1] = Bash(*)`. A pattern
written only against the scalar form misses the array form, which is the form real Claude Code
skills use [VERIFIED: ~/.claude/skills/gsd-audit-fix/SKILL.md:4-5 — `allowed-tools:\n  - Read\n  -
Write\n  - Edit\n  - Bash\n  - Grep`]. A second, sharper trap: this project's own
`.claude/settings.json` carries a real `permissions.deny` array containing entries that look exactly
like the shape a naive `permissions.*` regex would flag as dangerous
[VERIFIED: .claude/settings.json:9-13 — `"deny": ["Read(.env)", "Read(.env.*)", "Read(.secrets)"]`].
A pattern scoped to the bare `permissions` key without requiring the `.allow` path segment will
fire on a security *control* being described, not an attack — the single most important
false-positive risk this phase carries, and it is not mentioned anywhere in CONTEXT.md.

Two further code-verified findings should change how the plan sequences work: (1) `EXPECTED`'s exact
count enforced in D-05 already gives most of the "assert structural payloads were actually collected"
protection D-05 asks for — *if* the new collection function ever fails to walk the directory it
returns 0, which the mismatch check reports loudly — but a second, independent test is still needed
because `categories()`'s current `p.is_file()` filter means the directory is invisible to *every*
existing test, including `every_claimed_category_has_a_corpus_file`, until new code explicitly reads
it. (2) The `---`-fence requirement in D-01 is not stylistic: `frontmatter::extract()` requires the
fence to be the document's literal first line — no leading comment, no blank line — so the
explanatory-comment convention every other corpus file in this repo uses would silently break
parsing for a structural payload and produce a false "miss" that reads as detection failure rather
than corpus-authoring error.

GATE-03's ~1,300-file sweep is **not reproducible from this repository**. It is a manual,
machine-local procedure (`~/.claude/plugins/cache`, "the GSD workflow reference set", "seven sibling
repositories") run once by the maintainer for PR #103/#102 [VERIFIED: gh pr view 103, commit
a89feec]; there is no script, vendored corpus, or CI job. The planner should treat it as a manual
step to reconstruct, not a command to invoke.

**Primary recommendation:** Write the 12 threat-model payloads first (§Threat Model Catalogue gives
sourced phrasings for every shape except two, which are flagged as honest gaps), land the corpus +
harness-collection change + measured baseline in one commit (per D-04), then land patterns +
`relaxed_pattern` schema field + the four new clean-corpus controls + the extended policy ratchet in
a second commit. Design the first frontmatter pattern to require the `.allow[` path segment (or
equivalent) before matching a wildcard under `permissions`, not the bare key.

## Architectural Responsibility Map

This is a single-process Rust CLI, not a multi-tier application — the standard web-tier table does
not apply directly. Mapped onto this tool's own architectural layers instead:

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Wildcard grant detection (`allowed-tools`, `tools`, `permissions.allow`) | Structural engine (`src/frontmatter.rs` projection + `scope: frontmatter` pattern) | — | Shape is unambiguous once parsed; CRITICAL is only defensible here, not as prose regex (D-10) |
| Prose persuasion detection (`--dangerously-skip-permissions`, "no need to ask", "add this to your settings.json") | Prose engine (`src/scanner.rs` line/multiline/normalized/decoded passes) | — | Natural-language phrasing, no parser involved; HIGH per issue #33 |
| Pattern data (regex + metadata) | `patterns/core/tool-permission-abuse.yaml` (new file) | — | Follows the existing per-category YAML convention; must be registered in `load_embedded_patterns()` |
| Schema for the FP-control mutation guard | `src/pattern.rs` (`Pattern` struct, new field) | — | D-08: additive, optional, `deny_unknown_fields`-safe |
| Recall measurement | `tests/recall_test.rs` | `tests/corpus/attack/{tool-permission-abuse.md, structural/}` | New collection mode owned entirely by the test harness, not production code |
| False-positive proof | `tests/corpus/clean/` (4 new docs) + a new pairing test (D-07) | `tests/pattern_example_test.rs` (existing genre) | The corpus is production-adjacent test data; the pairing test is new CI surface |
| Consumer contract | `pattern_id` / `pattern_name` in the JSON `spec-ci-plugin` reads | `--baseline` escape hatch (`src/baseline.rs`) | Unaffected by this phase except for the new CRITICAL findings consumers will start seeing (D-12) |

## Standard Stack

### Core

No new external dependencies are required. This phase reuses the crates ENG-01/ENG-02 already
introduced.

| Library | Version (Cargo.toml) | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `serde_yaml` | 0.9 [VERIFIED: Cargo.toml:16] | Parses YAML frontmatter for the structural pass | Already load-bearing for pattern-file loading and ENG-01; unmaintained upstream (noted in `.continue-here.md` as issue #105, not a blocker for this phase) |
| `toml` | 0.8 [VERIFIED: Cargo.toml:17] | Parses TOML (`+++`) frontmatter | Already wired into `frontmatter.rs::parse` |
| `serde_json` | 1 [VERIFIED: Cargo.toml:15] | Parses whole-file JSON (`settings.json`, `.mcp.json` shape) and the common `Value` tree ENG-01 projects | Already the projection's internal representation |
| `regex` | 1 [VERIFIED: Cargo.toml:18] | Both prose and structural pattern matching | The whole scanner is regex-over-text by design (ENG-01's stated reason for a projection instead of a rule DSL) |

### Supporting

None new.

### Alternatives Considered

Not applicable — CONTEXT.md already locks the mechanism (ENG-01 projection + ordinary regex,
D-08's schema-field approach for the relaxed form). No alternative libraries or approaches are in
scope for this research.

**Installation:** none required.

## Package Legitimacy Audit

Not applicable — this phase adds zero new external packages. All work is pattern YAML, test files,
and a schema-field addition using crates already vendored and verified in prior phases.

## Architecture Patterns

### System Architecture Diagram

```
                    ┌─────────────────────────────────────────┐
                    │   Document under scan (skill file,       │
                    │   CLAUDE.md, settings.json, .mcp.json)   │
                    └───────────────┬───────────────────────────┘
                                    │
                    ┌───────────────▼───────────────┐
                    │ Scanner::scan_with_confidence  │  (src/scanner.rs)
                    └───────────────┬────────────────┘
                                    │
        ┌───────────┬──────────┬───┴──────┬──────────┬─────────────────┐
        ▼           ▼          ▼          ▼          ▼                 ▼
   line pass   multiline   normalized   decoded   [4th pass]    structural pass
   (Prose)     pass        pass         pass      unused here  (Frontmatter)
                                                                       │
                                                        ┌──────────────▼──────────────┐
                                                        │ frontmatter::analyze()       │
                                                        │  extract() -> parse() ->     │
                                                        │  project() = Vec<path=value> │
                                                        └──────────────┬──────────────┘
                                                                       │
                                                        only runs if >= 1 pattern has
                                                        scope: frontmatter loaded
                                                        (else short-circuited, scanner.rs:469)
                                                                       │
                                                        ┌──────────────▼──────────────┐
                                                        │ scope:frontmatter patterns   │
                                                        │ regex over "path = value"    │
                                                        │ confidence = 1.0 always      │
                                                        └──────────────┬──────────────┘
                                                                       │
                    ┌──────────────────────────────────────────────────▼
                    │  ScanMatch { pattern_id, severity, matched_text, context, ... }
                    └──────────────────────────────────────────────────
```

The prose passes (line/multiline/normalized/decoded) are the CAT-01 prose half's home; the
structural pass at the bottom is what D-01/D-13 arm for the first time in this milestone.

### Recommended Project Structure

```
patterns/core/
  tool-permission-abuse.yaml        # new — category: tool_permission_abuse, prose (HIGH) + scope:frontmatter (CRITICAL) patterns in one file
tests/corpus/attack/
  tool-permission-abuse.md          # new — line-oriented prose payloads
  structural/                       # new — D-01's second collection mode
    <n>-descriptive-name.md         # each file = ONE whole payload, --- fenced, first line literally "---"
tests/corpus/clean/
  narrow-allowed-tools-skill.md     # new — D-06 (1)
  settings-permissions-reference.md # new — D-06 (2)
  sandbox-bypass-runbook.md         # new — D-06 (3), hardest case, write first per CONTEXT.md
  mcp-setup-guide.md                # new — D-06 (4)
src/pattern.rs                      # +1 optional field (D-08) on Pattern
tests/pattern_policy_test.rs        # extended: new ratchet requiring the field for PI050+
tests/<new>_pairing_test.rs         # new — D-07's mutation-pairing test, "same genre as pattern_example_test.rs"
tests/recall_test.rs                # categories() gains a second collection mode; EXPECTED +2 rows
docs/PATTERN-CATALOGUE.md           # regenerated
.planning/ROADMAP.md                # Phase 3 criterion corrected per D-11
README.md                           # recall table + pattern-count table updated
```

### Pattern 1: Structural rule = ordinary regex over a canonical projection

**What:** A `scope: frontmatter` pattern is not a new matching language — it is the same
`RegexBuilder` used by every prose pattern, run against `ProjectedLine::render()`'s `"path = value"`
string instead of a raw line.

**When to use:** Any CAT-01 finding whose signal is a *parsed key/value*, not a sentence — wildcard
tool grants, `permissions.defaultMode: bypassPermissions`.

**Example (illustrative, not a proposed final regex):**
```yaml
# Source: src/frontmatter.rs render_scalar() — a JSON/YAML string value renders WITHOUT quotes,
# so "Bash(*)" in the source projects as literally Bash(*), not "Bash(*)".
  - id: PI050
    name: wildcard-tool-grant
    scope: frontmatter
    severity: CRITICAL
    # Illustrative only — must handle BOTH the scalar form (allowed-tools = *)
    # AND the array-indexed form (allowed-tools[0] = Bash(*)), because real
    # Claude Code skills write allowed-tools as a YAML block sequence
    # [VERIFIED: ~/.claude/skills/gsd-audit-fix/SKILL.md:4-5].
    pattern: "^(?:allowed-tools|tools)(?:\\[\\d+\\])?\\s*=\\s*(?:\\*|.*\\bBash\\(\\*\\))"
    example: "allowed-tools = *"
    counter_example: "allowed-tools[0] = Read"
```

### Pattern 2: Prose FP control proven by mutation, not assertion (established, not new)

**What:** Every widening in this repo's history ships the counter_example AND a demonstration that
removing the narrowing turns the corpus red (see `.claude/skills/pattern-library/SKILL.md` §"Prove
the false-positive control, do not assert it" — cites #95 and #97 as the two PRs where an
un-mutated control passed despite not actually holding). D-07/D-08 formalise this into a schema
field + CI test instead of a PR-description ritual.

**When to use:** Every CAT-01 pattern (D-09 makes it mandatory for PI050+).

### Anti-Patterns to Avoid

- **Scoping a wildcard pattern to the bare `permissions` key.** This project's own
  `.claude/settings.json` [VERIFIED: read this session, lines 9-13] proves `permissions.deny` is a
  real, common, *protective* array containing entries a naive regex over `permissions\..*=.*` would
  flag. Require the `.allow` path segment (or an equivalent semantic check) before matching.
- **Writing a structural corpus payload with a leading rationale comment.** Every other corpus file
  in this repo opens with an HTML or `#` comment explaining sourcing (see `clean/security-runbook.md`,
  `clean/config-precedence.md`). For a `structural/` payload this breaks parsing: `extract_delimited`
  [VERIFIED: src/frontmatter.rs:137-159] requires the fence to be `lines.next()` — the document's
  literal first line — with no tolerance for a preceding comment or blank line. A comment before
  `---` makes `extract()` return `None`, the structural pass finds nothing, and the payload silently
  reads as an undetected miss rather than a corpus bug.
- **Treating `EXPECTED`'s exact-pin as sufficient proof the structural directory is collected.** It
  is necessary but not sufficient — see §Harness Change.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Matching structured config keys | A path/rule DSL in the pattern schema | `scope: frontmatter` + the existing regex engine over `ProjectedLine::render()` | ENG-01 already made this decision and documented why (`src/frontmatter.rs:9-13`) — a second matching language is out of scope and contradicts the phase's own dependency |
| Proving a false positive control holds | A PR-description assertion ("verified: X stays green") | The D-07/D-08 mechanism: a `relaxed_pattern` field + an automated pairing test | This repo's own history (#95, #97 per the pattern-library skill) shows asserted-but-unmutated controls shipped real over-widenings twice already |
| Detecting real vs. relaxed pattern behaviour | Hand-testing "does this regex still avoid the counter_example" per PR | The pairing test compiles both `pattern` and `relaxed_pattern` and asserts opposite outcomes against the same control text, in CI, for every pattern that has the field | Automates exactly what GATE-05 requires and cannot regress silently |

**Key insight:** every mechanism this phase needs (projection, mutation-proof pairing, exact-pin
recall) already exists as an established idiom in this codebase. The research risk is not "what
library to use" — it is "what exact text shape reaches the regex," which is why this document
concentrates on projection edge cases and the harness's file-collection trap.

## Common Pitfalls

### Pitfall 1: The array-form projection is the common case, not the edge case

**What goes wrong:** A pattern author writes and tests against `allowed-tools: "*"` (scalar), ships
it, and it fails to fire on the far more common block-sequence form real skills use.

**Why it happens:** `frontmatter::walk()` [VERIFIED: src/frontmatter.rs:183-228] recurses into
`Value::Array` and appends `[index]` to the path for each element, producing one projected line per
array element rather than one line for the whole array. A YAML block sequence —
```yaml
allowed-tools:
  - Read
  - Bash(*)
```
— parses to `Value::Array([String("Read"), String("Bash(*)")])` and projects as two lines:
`allowed-tools[0] = Read` and `allowed-tools[1] = Bash(*)`. There is no single line reading
`allowed-tools = [Read, Bash(*)]` or similar.

**How to avoid:** Every CAT-01 structural pattern's regex must match the value at any array index,
i.e. anchor on the *value* content after `=`, with the key/index prefix flexible:
`(?:allowed-tools|tools)(?:\[\d+\])?\s*=\s*...`, not `^allowed-tools\s*=\s*\*$`.

**Warning signs:** A pattern's `example:` field uses the scalar form only, and `counter_example:`
never exercises the array form — the pairing test (D-07) would not catch this because it tests
*narrowing*, not *coverage*; only a corpus payload written in block-sequence form (matching the real
skill-file convention) catches it, which is why GATE-01's threat-model corpus matters here
specifically.

### Pitfall 2: `permissions` is claimed at the key level, but `.allow` vs `.deny` is a semantic split the projection does not make for you

**What goes wrong:** A pattern scoped to `permissions` and matching any wildcard character fires on
`permissions.deny[0] = Bash(rm -rf *)` — a security control being *described*, which this repo's own
`.claude/settings.json` contains [VERIFIED: read this session] — turning a defensive document into a
false CRITICAL.

**Why it happens:** ENG-01's projection is path-shape-agnostic; it does not know `allow` and `deny`
are semantic opposites. D-13 claims the `permissions` key generically.

**How to avoid:** Any `permissions`-scoped pattern must require the path contain `.allow` (or
similar) before considering the value, and separately, `permissions.defaultMode = bypassPermissions`
(a real, dangerous structural shape — see §ENG-01 Arming) needs its own arm, since it is not an
array element at all.

**Warning signs:** the clean-corpus control document #1 (D-06, narrow `allowed-tools`) and the
settings/permissions reference doc (D-06 #2) will not by themselves catch this — neither has to
contain a `permissions.deny` block written in structured (not prose) form. Recommend the narrow
skill-file control (D-06 #1) or a fifth ad-hoc specimen also carry a real
`.claude/settings.json`-shaped `permissions.deny` array, since that is the exact shape this repo's
own file already demonstrates is realistic and common.

### Pitfall 3: GATE-03's sweep cannot be run by a fresh contributor or CI

**What goes wrong:** A plan step says "run the GATE-03 sweep" as if it were a command.

**Why it happens:** The ~1,300-file sweep for PR #103/#102 [VERIFIED: `gh pr view 103`, commit
`a89feec`] ran against `~/.claude/plugins/cache`, "the GSD workflow reference set," and "seven
sibling repositories in this ecosystem" — none vendored into this repo, none pinned by version, none
referenced by a script or CI job. `grep -rn "1,300"` across the whole repo (including `docs/`,
`.github/`, `scripts/` — no `scripts/` directory exists) returns only prose mentions in
`CHANGELOG.md`/`.planning/`, never a command.

**How to avoid:** Plan this as a manual, human-run step against whatever `~/.claude/plugins`,
sibling UnityInFlow repos, and GSD reference docs happen to be present on the machine doing the PR —
document the exact directories swept in the PR description (as #103 did), not as an automated gate.
Do not block the plan on scripting this; it was never scripted for v0.1.0 either.

### Pitfall 4: The perf-headroom number cited in REQUIREMENTS.md is stale

**What goes wrong:** REQUIREMENTS.md's "Out of Scope" table cites "41ms against 200ms" (~5×
headroom) as the reason the Aho-Corasick prefilter (#4) is deferred. The measured, current number in
README.md is **60ms on a 40-file repository, against the 200ms budget** [VERIFIED: README.md:597] —
~3.3× headroom, not ~5×.

**Why it happens:** 41ms predates ENG-02's recursive decoder, which STATE.md documents cost "137ms
of a 143ms regression" before a function-word gate cut it to "3.3% overhead" — the 60ms figure is
post-fix and post-ENG-02; 41ms was measured before ENG-02 landed at all.

**How to avoid:** If CAT-01's new patterns (prose + structural) measurably move the number again,
re-measure and report the new headroom rather than reasoning from the stale 41ms. 12 new prose
patterns plus a projection pass (only paid when a `scope: frontmatter` pattern exists — see
scanner.rs:469-473) is unlikely to threaten the 200ms budget, but the actual number should be stated
in the PR, following this repo's own established practice (every prior category widening measured
and reported its cost).

## Code Examples

### The exact projection shapes a CAT-01 structural pattern must handle

```text
# Source: src/frontmatter.rs::render_scalar / walk — verified by reading the projection code and a
# real skill file this session.

# Scalar wildcard (rare in practice for allowed-tools, common for a hand-authored settings override)
allowed-tools = *

# Array-indexed wildcard — the REAL shape real Claude Code skills use
# (VERIFIED: ~/.claude/skills/gsd-audit-fix/SKILL.md:4-5 uses a YAML block sequence)
allowed-tools[0] = Read
allowed-tools[1] = Write
allowed-tools[2] = Edit
allowed-tools[3] = Bash
allowed-tools[4] = Grep

# JSON array form (settings.json / .mcp.json shape)
tools[0] = *

# Nested settings.json permissions shape — VERIFIED against this project's own global
# ~/.claude/settings.json this session:
#   "permissions": { "allow": [...], "deny": [...], "defaultMode": "auto" }
permissions.allow[0] = Bash(npx gsd-core *)
permissions.deny[0] = Read(.env)
permissions.defaultMode = auto

# The dangerous structural shape a widening attacker would actually write —
# not an array element at all:
permissions.defaultMode = bypassPermissions
```

### The recall harness's collection-mode trap (D-05), stated precisely

```rust
// Source: tests/recall_test.rs:102-108 (read this session, verbatim)
fn categories() -> Vec<(String, PathBuf)> {
    let dir = attack_dir();
    let mut out: Vec<(String, PathBuf)> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("attack corpus must be readable: {e}"))
        .map(|e| e.expect("directory entry").path())
        .filter(|p| p.is_file())   // <-- a `structural/` subdirectory is silently dropped HERE
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("README.md"))
        .map(|p| { /* name = file_stem, e.g. "tool-permission-abuse" */ })
        .collect();
    // ...
}
```

If a plan step simply drops files into `tests/corpus/attack/structural/` without also changing this
function, the suite stays green at 60/60 and the new payloads are never scanned — exactly the
"test that measured nothing" anti-pattern `.continue-here.md` names as blocking.

## State of the Art

Not meaningfully applicable — this is pattern-authoring work in an established, actively-maintained
in-repo idiom (verb×object matrices, mutation-tested controls), not a library/framework choice with
an "old way vs. new way." The one relevant shift is external: `--dangerously-skip-permissions` /
`bypassPermissions` / `--yolo` are all mid-2025–2026 additions across the agent-CLI ecosystem
(Claude Code, Gemini CLI) that did not exist when the original injection-scanner spec was written —
this is precisely why CAT-01 is a new category rather than a widening of an old one.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The exact regex/wording for OWASP's *current* (2025/2026) LLM06 "Excessive Agency" entry — I could not fetch `genai.owasp.org`'s live text; only the archived 2023 v1.1 wording ("Granting LLMs unchecked autonomy...") was directly readable, plus secondary summaries of the 2025 edition | §Threat Model Catalogue | Low — used only as category-framing context, not as a payload source; no corpus text is derived from it |
| A2 | The "update .vscode/settings.json with the recommended configuration" quote attributed to CVE-2025-53773's attack chain came from a third-party arxiv paper's paraphrase of the CVE, not the primary GHSA/CVE.org advisory text (which I could not fetch directly — 404) | §Threat Model Catalogue | Medium — if used verbatim as a corpus payload, verify against a primary advisory first; the *shape* (persuading an agent to write an auto-approve flag into a settings file) is corroborated by three independent secondary write-ups (Wiz, embracethered.com, multiple CVE aggregators) |
| A3 | "you have permission to run any command" / "allowlist this command" have no directly-sourced published phrasing found this session (searches returned generic prompt-injection examples, not this specific phrasing) — see the explicit gap note below | §Threat Model Catalogue | Low-Medium — these are straightforward paraphrases of the shape issue #33 already specifies; a planner writing a corpus payload from this shape is not inventing a *category*, only wording within one already locked by CONTEXT.md/issue #33 |
| A4 | `Bash` (unscoped, no parens) as an `allowed-tools` entry grants unrestricted shell access, same severity class as `Bash(*)` — based on training knowledge of Claude Code's permission syntax, not verified against a fetched primary doc this session | §ENG-01 Arming / Pitfall 1 | Medium — if wrong, a pattern that also flags bare `Bash` would be an unjustified widening; verify against `code.claude.com/docs/en/permission-modes` or equivalent before shipping a pattern that treats bare `Bash` as equivalent to `Bash(*)` |

## Open Questions

1. **Should `relaxed_pattern` (or whatever D-08's field is named) render into the public,
   generated `docs/PATTERN-CATALOGUE.md`?**
   - What we know: `example` and `counter_example` are both rendered there today
     [VERIFIED: `.claude/skills/pattern-library/SKILL.md` — "They are rendered into
     docs/PATTERN-CATALOGUE.md"], and D-08 places the new field "beside" them.
   - What's unclear: publishing the exact regex mutation that defeats each pattern's narrowing is a
     different disclosure profile than publishing an attack example or a benign counter-example —
     it is closer to publishing a bypass. CONTEXT.md does not address this.
   - Recommendation: the planner/pattern-library-skill update should decide explicitly whether
     `catalogue::render()` includes this field, rather than defaulting to "same treatment as the
     other two" by inertia. Given D-08 is recorded as one-way, get this right before the first
     PI050 pattern ships.

2. **Where does `structural/`'s per-payload rationale live, given the fence-must-be-line-1
   constraint?**
   - What we know: every existing corpus file opens with an explanatory comment; a structural
     payload file cannot, because `---` must be `lines.next()`.
   - What's unclear: whether the plan wants per-file trailing comments (after the closing `---`,
     which `extract()` ignores and is therefore safe), a shared `structural/README.md` (matching the
     top-level `attack/README.md` convention), or filenames descriptive enough to not need prose.
   - Recommendation: a shared `structural/README.md` following the existing `attack/README.md`
     pattern is the lowest-risk choice — it cannot accidentally break a payload the way a
     per-file leading comment would.

3. **Does the harness need a THIRD collection mode later, or does `structural/` generalize?**
   - What we know: CAT-02 (#34, next phase) also has a structural half (`mcpServers`), per the
     traceability table.
   - What's unclear: whether `tests/corpus/attack/structural/` should be flat (this phase's files
     only, category hardcoded as `"tool-permission-abuse-structural"`) or nested by category
     (`structural/<category>/`) for CAT-02 to reuse without touching `categories()`'s logic again.
   - Recommendation: build it flat for this phase per D-01's literal path, but leave a comment in
     the new collection code flagging that CAT-02 will need to either add its own subdirectory or
     generalize this function — do not silently paint the design into a corner GATE-04 will then
     make expensive to unwind (D-01 already tags this collection mode "costly" to change again).

## Environment Availability

Not applicable — this phase has no runtime dependencies beyond crates already vendored (`serde_yaml`,
`toml`, `serde_json`, `regex`), and the toolchain already used (`cargo test`, `cargo clippy`,
`cargo fmt`). No new CLI tools, services, or databases are introduced.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust's built-in `#[test]` harness via `cargo test`; `criterion` 0.8 for benches only (not part of the CI gate) [VERIFIED: Cargo.toml:26-34] |
| Config file | none — `[dev-dependencies]` in `Cargo.toml`; no separate test-runner config |
| Quick run command | `cargo test --test recall_test` (recall only) / `cargo test --test pattern_test` (new pattern cases) |
| Full suite command | `cargo test --locked` (331 tests as of the last STATE.md measurement, pre-phase) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GATE-01 | 12 threat-model payloads collected and counted | integration | `cargo test --test recall_test recall_matches_the_recorded_numbers` | ❌ Wave 0 — needs new collection mode |
| GATE-02 | Exact recall pin, both rows | integration | `cargo test --test recall_test` | ✅ exists, needs `EXPECTED` rows added |
| GATE-03 | Third-party sweep, zero regressions | manual | `cargo run --release -- check <local-dirs> --exclude '.planning/**'` (see Pitfall 3 — not scriptable from this repo) | N/A — manual by design |
| GATE-04 | Scope stays to CAT-01 only | review | n/a — a code-review/plan-boundary check, not a test | N/A |
| GATE-05 | Mutation-tested FP control per pattern | unit | new test, "same genre as `tests/pattern_example_test.rs`" per CONTEXT.md canonical refs | ❌ Wave 0 |
| D-09 (field required for PI050+) | ratchet | unit | extend `tests/pattern_policy_test.rs`'s pattern | ❌ Wave 0 — needs a parallel `LEGACY_UNTESTED`-style (but inverted: PI050+ ids must all comply, no exemption list needed since they're new) |
| Clean corpus, 4 new docs | zero findings | integration | `cargo test --test corpus_test` | ✅ exists, needs 4 new fixture files |
| Whole-repo self-scan | zero non-payload findings | manual/CI | the `check .` one-liner in `.claude/skills/pattern-library/SKILL.md` | ✅ exists as documented command |

### Sampling Rate

- **Per task commit:** `cargo test --test recall_test` + `cargo test --test corpus_test` (fast,
  targeted)
- **Per wave merge:** `cargo test --locked` (full suite)
- **Phase gate:** Full suite green, `cargo clippy --all-targets --locked -- -D warnings`,
  `cargo fmt --all -- --check`, catalogue regenerated, whole-repo self-scan clean outside
  `examples/`, `patterns/`, `tests/`, `tools/` before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `tests/recall_test.rs` — new collection mode for `structural/` (D-01), plus an explicit test
  proving the directory is actually walked (D-05) — not fully covered by the exact-pin alone; see
  §Common Pitfalls / §Harness Change reasoning above
- [ ] A new pairing test file (name TBD — e.g. `tests/pattern_relaxed_control_test.rs`) implementing
  D-07/GATE-05
- [ ] `tests/pattern_policy_test.rs` extended for D-09's PI050+ requirement
- [ ] Four new `tests/corpus/clean/` fixtures (D-06)
- [ ] `tests/corpus/attack/tool-permission-abuse.md` and `tests/corpus/attack/structural/*`

## Security Domain

This tool IS a security control (a static injection scanner), not an application with its own
auth/session surface — most ASVS categories are not applicable in the usual sense. What applies:

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | No auth surface in this tool |
| V3 Session Management | No | Stateless CLI |
| V4 Access Control | No | No multi-user access model |
| V5 Input Validation | **Yes** | The scanner parses untrusted YAML/TOML/JSON by definition (ENG-01). Already bounded: `MAX_DEPTH`=12, `MAX_NODES`=5,000, `MAX_VALUE_LEN`=2,048 bytes [VERIFIED: src/frontmatter.rs:48-54]. CAT-01 introduces no new parsing surface — it only adds regex patterns run against the existing bounded projection |
| V6 Cryptography | No | Not applicable to this phase |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| YAML "billion laughs" / alias-expansion decode bomb via a crafted frontmatter block | Denial of Service | Already mitigated by `frontmatter.rs`'s depth/node/length bounds; `serde_yaml` is unmaintained (issue #105, noted in `.continue-here.md`) — not this phase's problem to fix, but the bound is what currently protects against it, not the parser itself |
| Catastrophic backtracking (ReDoS) in a new hand-written CAT-01 regex, especially any pattern chaining `.{0,N}?` windows the way `PI021`/`PI022` do | Denial of Service | Follow this repo's established idiom: bounded, non-greedy windows (`[^.\n]{0,20}?`), never an unbounded `.*` before a variable-length alternation; the `pattern_example_test`/`corpus_test` suite runs every pattern against real, longer documents (the sweep), which is where a pathological regex would first show up as a timeout, not a correctness failure |
| A structural pattern accidentally matching a *protective* configuration value (e.g. `permissions.deny[0] = Bash(rm -rf *)`) as if it were an attack | Tampering (false report) | See Common Pitfall 2 — require the `.allow` path segment, do not match on the bare `permissions` key + wildcard character alone |

## Sources

### Primary (HIGH confidence)

- This repository, read directly this session: `.planning/phases/03-tool-permission-abuse-cat-01-33/03-CONTEXT.md`,
  `.planning/REQUIREMENTS.md`, `.planning/STATE.md`, `.planning/ROADMAP.md`, `.planning/.continue-here.md`,
  `tests/recall_test.rs`, `src/pattern.rs`, `src/frontmatter.rs`, `src/scanner.rs`,
  `src/patterns/mod.rs`, `src/walk.rs`, `.claude/skills/pattern-library/SKILL.md`,
  `tests/corpus/attack/README.md`, `tests/corpus/clean/*`, `patterns/core/exfiltration.yaml`,
  `patterns/core/instruction-injection.yaml`, `tests/pattern_example_test.rs`,
  `tests/pattern_policy_test.rs`, `tests/suppression_symmetry_test.rs`, `tests/corpus_test.rs`,
  `README.md`, `PATTERNS.md`, `Cargo.toml`, `~/.claude/skills/gsd-*/SKILL.md` (real skill
  frontmatter), `~/.claude/settings.json` and `.claude/settings.json` (real permissions shape),
  `gh issue view 33`, `gh pr view 103`, `git log`/`git show a89feec`, `docs/DETECTION-BACKLOG.md` §Part 2

### Secondary (MEDIUM confidence)

- CVE-2025-53773 (GitHub Copilot / VS Code `chat.tools.autoApprove` privilege escalation via prompt
  injection) — corroborated across Wiz (wiz.io/vulnerability-database/cve/cve-2025-53773),
  embracethered.com, CVE.org record listing, and an arxiv paper's paraphrase
  (arxiv.org/html/2601.17548v1). Independent sources agree on the mechanism (an injected prompt
  persuades the agent to write an auto-approve config flag into a settings file); exact quoted
  wording is from the arxiv paper's summary, not confirmed against a primary advisory text directly
  (WebFetch to the GHSA advisory 404'd).
- Claude Code permission-mode documentation (`bypassPermissions`, `acceptEdits`, `defaultMode`,
  `--dangerously-skip-permissions`) — cross-checked across multiple independent
  practitioner write-ups (claudelog.com, morphllm.com, developersdigest.tech,
  explainx.ai) converging on the same field names and CLI flag.
- Gemini CLI `--yolo` flag, Aider `--yes-always`, Cursor "YOLO mode" — cross-checked across
  addyosmani.com, inventivehq.com, cowork.ink.
- arxiv.org/html/2601.17548v1, "Prompt Injection Attacks on Agentic Coding Assistants" — taxonomy
  categories (permission escalation via configuration, tool poisoning, wildcard file access in
  skills) and the Claude Code skills observation "A skill with Read access can read any file, not
  just project files."

### Tertiary (LOW confidence)

- OWASP LLM Top 10 2025's exact current LLM06 "Excessive Agency" wording — only the archived 2023
  v1.1 text was directly fetchable; the 2025 framing (expanded scope, subsuming "Insecure Plugin
  Design") is from WebSearch summaries of secondary sources, not the primary `genai.owasp.org` page
  text.
- General prompt-injection phrasing examples (PayloadsAllTheThings, Palo Alto Unit42, Pillar
  Security blog posts) — useful for category framing, not for CAT-01-specific phrasing; none
  produced a directly quotable "skip confirmation" / "allowlist this command" example.

## Metadata

**Confidence breakdown:**
- Standard stack / mechanism: HIGH — everything needed is already implemented and read this session; no new libraries
- Architecture (projection edge cases, harness trap, permission.allow/deny split): HIGH — derived from reading the actual source, not inferred
- Threat-model phrasing (§ below, GATE-01 payload sourcing): MEDIUM — one strong, cross-corroborated CVE and several cross-checked terminology sources; two explicit phrasing gaps admitted rather than invented
- GATE-03 reproducibility finding: HIGH — verified by exhaustive repo grep plus reading the actual PR body that produced the cited number

**Research date:** 2026-09-01
**Valid until:** ~30 days for the code-level findings (stable unless `src/frontmatter.rs`/`src/scanner.rs` change again); ~7 days for the external CVE/terminology citations if verbatim payload wording will be drawn from them, since primary-source confirmation is still outstanding (see Assumptions Log A2)

---

## Appendix: Threat Model Catalogue (Research Question 1)

Sourced attack shapes for GATE-01's 12 threat-model payloads. Each entry names its source and
confidence tier. Where no published phrasing was found, the gap is stated explicitly rather than
filled with an invented example.

### Structural shapes (frontmatter half)

| Shape | Source | Tier |
|---|---|---|
| `allowed-tools: "*"` / `allowed-tools: [Bash(*)]` wildcard tool grant | Issue #33's own pattern list, corroborated as a general "wildcard grants on agent roles" best-practice-violation pattern by MCP-security literature (Aptible, Cloud Security Alliance labs) | CITED — the exact key names are from this project's issue tracker; the general risk framing is cross-checked externally |
| `"tools": ["*"]` (JSON manifest form) | Issue #33 | CITED (project-internal) |
| `permissions.allow: ["Bash(*)"]` in a `settings.json`-shaped document | Verified real syntax from this project's own `~/.claude/settings.json` [VERIFIED: read this session] combined with the attack framing from issue #33 | VERIFIED (syntax) + CITED (attack framing) |
| `permissions.defaultMode: "bypassPermissions"` set structurally rather than via a CLI flag | Derived from real Claude Code `permissionMode` field values (`default`, `acceptEdits`, `plan`, `bypassPermissions`), cross-checked across claudelog.com, explainx.ai, morphllm.com | CITED — not explicitly listed in issue #33, but a natural extension of D-13's claimed `permissions` key; flagged as a recommendation, not a locked requirement |

### Prose shapes — persuasion to widen authority

| Shape / real phrasing | Source | Tier |
|---|---|---|
| "update `.vscode/settings.json` with the recommended configuration" (leading the agent to write `"chat.tools.autoApprove": true`) | CVE-2025-53773 attack chain, corroborated across Wiz, embracethered.com, and an arxiv paper's paraphrase of the advisory | CITED (cross-corroborated, primary advisory text not directly fetched — see Assumption A2) |
| `--dangerously-skip-permissions` (CLI flag) | Claude Code CLI, directly named in issue #33 and cross-checked across multiple practitioner docs | CITED |
| `bypassPermissions` (permission-mode value) | Claude Code `permissionMode`/`defaultMode` field, cross-checked across claudelog.com, explainx.ai, morphllm.com | CITED |
| `--yolo` | Gemini CLI's actual flag name (also bound to Ctrl+Y in-session), cross-checked across addyosmani.com and inventivehq.com | CITED |
| `--yes-always` | Aider's equivalent flag, cross-checked via addyosmani.com's comparison table | CITED |
| "you have permission to run any command" | Paraphrase of issue #33's own listed shape; no independently published verbatim example located this session | **ASSUMED / honest gap** — a planner using this wording is instantiating a shape CONTEXT.md/issue #33 already locked, not inventing a new attack class, but the exact sentence is not sourced externally |
| "you may skip confirmation" / "no need to ask" | Same as above — issue #33's own phrasing; no external verbatim source found | **ASSUMED / honest gap** |
| "add this to your settings.json" / "update your permissions to include" / "allowlist this command" | Same shape as the CVE-2025-53773 chain above (structurally identical: persuade the agent to write a permission-widening entry into a config file the agent controls); the CVE gives an existing published near-verbatim instance ("update .vscode/settings.json with the recommended configuration") that can stand in for this shape with more confidence than the issue's own wording alone | CITED, via the CVE analogy |
| Instructions to disable a hook or guardrail ("turn off the pre-commit hook", "disable that guardrail") | Not independently sourced from a published incident this session; structurally the mirror image of PI018's existing "verification/safety checks have been disabled" arm in `patterns/core/instruction-injection.yaml` [VERIFIED: read this session, PI018 pattern], which the scanner ALREADY partially detects for *claims a control is off* — CAT-01's new arm per D-17 is specifically the *imperative* form ("turn off X") rather than PI018's *declarative* form ("X has been disabled") | **Partially ASSUMED** — the declarative sibling is verified in-repo; the imperative phrasing itself has no external citation found |

### Explicit gaps (stated per the instruction to admit rather than invent)

No published, directly-quotable phrasing was located this session for: "you have permission to run
any command," "you may skip confirmation," "no need to ask" (as a standalone payload rather than
attached to a disclosure request), and the imperative "disable/turn off this guardrail/hook" framing.
These are all straightforward instantiations of shapes issue #33 and CONTEXT.md already lock, so the
gap is in *external verbatim sourcing*, not in category legitimacy — a planner can write these
payloads directly from the locked issue text without contradicting GATE-01's "not derived from the
patterns" rule, since the *patterns don't exist yet* and the *issue* (not the regex) is the source.
