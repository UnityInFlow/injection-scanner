# Phase 3: Tool & permission abuse — CAT-01 (#33) - Context

**Gathered:** 2026-08-31
**Status:** Ready for planning

<domain>
## Phase Boundary

`PI050`–`PI059`: detection of documents that widen the **agent's own authority** — the agentic
equivalent of privilege escalation. Two halves:

- **Structural** — wildcard and over-broad grants in parsed frontmatter (`allowed-tools`, `tools`,
  `permissions`), matched through ENG-01's `scope: frontmatter` projection at CRITICAL.
- **Prose** — persuasion to widen: `--dangerously-skip-permissions` / `bypassPermissions` /
  `--yolo` directives, "no need to ask", "you may skip confirmation", "add this to your
  settings.json", "allowlist this command", and instructions to disable a guardrail. Ordinary
  regex engine at HIGH.

**This is the phase that first arms ENG-01.** `grep "scope:" patterns/core/*.yaml` returns nothing
today, so the structural pass is inert in the shipping binary. Phases 1 and 2 were provably
behaviour-neutral or purely additive; **Phase 3 is the first to carry real false-positive risk.**

Not in this phase: `mcpServers` (CAT-02, #34), `hooks` configuration (CAT-03, #35), bare
dangerous-command detection (`sudo`, `rm -rf`, `chmod 777` — deferred, see below).

</domain>

<decisions>
## Implementation Decisions

### Corpus shape for the structured half

- **D-01:** The recall harness gains a **second collection mode**: `tests/corpus/attack/structural/`
  where each *file* is one whole payload, `---` fenced and realistic. The existing line-oriented
  files are unchanged. Required because `tests/recall_test.rs::payloads()` splits on `\n` and scans
  each line as its own document, so a frontmatter payload cannot be expressed at all today.
  — **Reversibility:** costly — `recall_test.rs` is the harness behind the published recall number
  and GATE-02's exact pin; changing the collection contract again means re-baselining every count
  and the README table with it.
- **D-02:** `EXPECTED` gains **two rows**, `("tool-permission-abuse", n, m)` and
  `("tool-permission-abuse-structural", n, m)`. A single combined row can stay correct while the
  structural half silently goes to zero — which is exactly what an inert ENG-01 pass looks like.
  The README may still present one combined figure per category.
- **D-03:** **No prose/structural ratio is set in advance.** Write all 12 payloads from the threat
  model, then record whatever split results. A target chosen up front shapes the corpus by the
  implementation plan — the same defect as deriving it from the patterns, which GATE-01 exists to
  prevent.
- **D-04:** **Corpus lands first, with a measured baseline.** Commit the 12 payloads alone and run
  them against the current 48-pattern binary *before any PI05x exists*; commit that number. The
  patterns land as a second commit and the delta is the evidence. This makes GATE-01's ordering
  visible in git history rather than asserted, and surfaces incidental spillover from existing
  categories by name instead of leaving it to be assumed.
- **D-05:** `categories()` currently filters `p.is_file()`, so a new `structural/` subdirectory
  would be **silently skipped** — the total would stay 60 and the suite would still pass. The
  harness change must assert the structural payloads were actually collected. This is the
  "test that measured nothing" anti-pattern; do not let it recur in the harness itself.

### False-positive control

- **D-06:** Four new documents join `tests/corpus/clean/` (**15 → 19** non-README specimens) **before** the patterns land.
  Today `grep -rniE "allowed-tools|dangerously-skip|bypassPermissions|settings\.json|permission"
  tests/corpus/clean/` returns **nothing**, so every CAT-01 pattern would otherwise ship with a
  control that cannot fail:
  1. a skill file with **real, narrow** `allowed-tools` (`[Read, Grep]`, scoped `Bash(npm test)`) —
     proves a structural rule fires on `*` because the *value* is `*`, not because the *key* is
     `allowed-tools`;
  2. a settings/permissions **reference doc** that names `--dangerously-skip-permissions`,
     `bypassPermissions` and `settings.json` descriptively — a doc that mentions a flag must not
     read as a doc that tells the agent to use it (the `Show the current system prompt` / PI021
     precedent);
  3. an operator **runbook instructing a human** to bypass inside a disposable container — the
     hardest case in the category, differing from the payload only by audience and provenance. If a
     pattern cannot stay off this, the pattern needs narrowing before it ships;
  4. an **MCP/agent setup guide** that legitimately says "add this to your settings.json".
  **Corrected 2026-09-01** (research pass): the count above originally read "16 → 20", which counted
  `README.md`. Measured: `ls tests/corpus/clean/ | grep -v README` is **15**, and `corpus_test.rs`'s
  own `specimens()` filter excludes the README. Recorded because the milestone has a *blocking*
  anti-pattern about inheriting a documented number without measuring it.
- **D-06a (added from research):** A `permissions`-scoped pattern **must require the `.allow` path
  segment**. A real `settings.json` carries a `permissions.deny` array whose entries
  (`Read(.env)`, `Read(.secrets)`) have the same shape as the attack, so a pattern keyed on
  `permissions` alone would flag a **security control** as an attack — the worst possible false
  positive for this tool. Verified in `~/.claude/settings.json` on this machine. **Note the research
  report attributed this to "this repo's own `.claude/settings.json`" — that is wrong; this repo's
  file contains only a `hooks` block and no `permissions` key at all. The risk is real, the
  attribution was not.** The deny-list document is worth adding as a fifth clean-corpus specimen.
- **D-07:** GATE-05 is enforced by an **automated pairing test in CI**, not a manual step. For each
  new pattern: assert the shipped pattern stays **off** its control document, *and* that a
  documented **relaxed** form of the same pattern **does** match it. A control that would pass even
  with the narrowing removed fails the build. Same genre as the existing
  `pattern_example_test.rs` and `suppression_symmetry_test.rs`.
- **D-08:** The relaxed form lives in a **new optional field on the `Pattern` schema**, beside
  `example` and `counter_example` — following the recorded `raw_only` precedent: "deliberately a
  schema field rather than a tag so that `deny_unknown_fields` catches typos and the choice is
  visible in review". Keeps the guard adjacent to what it guards.
  — **Reversibility:** one-way — the pattern schema is a published contract. `patterns/` is a
  documented community-contribution surface and `deny_unknown_fields` is enforced, so once the
  field ships, external pattern files may carry it and removing it breaks their load. Field naming
  and semantics get one chance.
- **D-08a (resolved 2026-09-01, from research + pattern mapping):** The relaxed form is **not**
  rendered into `docs/PATTERN-CATALOGUE.md`. Both agents raised this as a disclosure question —
  "publishing a bypass mutation is a different disclosure profile than `example`/`counter_example`".
  **That premise is false and was verified false:** `src/catalogue.rs:152` already emits every
  shipped regex verbatim inside a `<details><summary>Regex</summary>` block, so the narrowing — and
  therefore the bypass region — is already fully public. The actual reason to keep it out is
  editorial: the catalogue documents *what the scanner detects*, and the relaxed form is test
  scaffolding describing what it deliberately does **not**. Recorded with the correction so this is
  not later "fixed" on the disclosure grounds that do not apply.
- **D-06b (resolved 2026-09-01):** D-06a's deny-list document is a **distinct fifth specimen**, not
  folded into control #1. A `settings.json` carrying `permissions.deny` is a different document
  shape from a skill file with a narrow `allowed-tools`, and it is the one that catches the
  flag-a-security-control failure. Clean corpus therefore goes **15 → 20** non-README specimens;
  D-06's "15 → 19" counted only the original four.
- **D-09:** The field is **required for new patterns (PI050+)** and enforced by extending the
  per-pattern test policy ratchet that `REQUIREMENTS.md` already tracks. The existing 48 stay
  exempt — a 48-file migration inside a category PR is against GATE-04. CAT-02 and CAT-03 inherit
  the requirement automatically.

### The spec-linter S005 boundary

- **D-10:** A frontmatter-scoped `PI05x` **does** fire on a file's own wildcard grant, accepting
  overlap with `spec-linter` S005. The justification is **provenance, not phrasing**: S005 lints a
  spec you wrote, in your repo, at authoring time; injection-scanner is pointed at untrusted input —
  a skill from a registry, a third-party MCP config, a RAG document. The same `allowed-tools: *` is
  a lint finding in your own CLAUDE.md and an attack in a skill someone shipped you. The consumers
  never coincide.
- **D-11:** **`ROADMAP.md`'s Phase 3 criterion is wrong and must not be inherited.** It reads
  "Complements `spec-linter` S005 rather than duplicating it — S005 lints the spec's own
  permissions; this detects a *document persuading someone* to widen them." Taken literally that
  deletes the structural half of the category and leaves ENG-01 inert, and it contradicts issue
  #33's own pattern list ("Wildcard tool grants in frontmatter: `allowed-tools: *`, `Bash(*)`,
  `"tools": ["*"]`"). Correct the criterion to D-10's wording as part of this phase. Recorded
  explicitly because inheriting a documented reason without measuring it is a **blocking**
  anti-pattern on this milestone — it is how 59/60 survived three documents.
- **D-12:** The new CRITICAL **ships**, and is documented as a deliberate behaviour change in the
  release notes and README. `spec-ci-plugin` shells out to this binary in consumer CI, so consumer
  builds carrying a wildcard grant will go from green to red on upgrade. That is the finding the
  tool exists for. (`--baseline` shipped in v0.1.0 and remains the escape hatch; it was not made a
  gate on this decision.)
  — **Reversibility:** one-way — once released, downstream CI has been broken and re-narrowing
  cannot un-break it; a later softening is itself a second published behaviour change.
- **D-13:** CAT-01 claims **`allowed-tools`, `tools`, `permissions`** from ENG-01's projected key
  set. `mcpServers` belongs to CAT-02 (#34); `hooks` belongs to CAT-03 (#35). `model` and `system`
  overrides stay **unclaimed** — decide when a category actually needs them rather than squatting
  now. Keeps GATE-04's blast radius honest and stops the first category absorbing the other two.

### Dangerous-command items

- **D-14:** `sudo`, `rm -rf`, `chmod 777` and `curl … | sudo sh` are **dropped from CAT-01**. They
  are payload *execution*, not authority *widening* — a different threat model with a different
  false-positive story, and the highest-FP items in #33's list (every install guide and runbook
  contains them).
- **D-15:** **`PI028 pipe-to-shell` already exists** in `patterns/core/exfiltration.yaml` and covers
  most of #33's `curl | sh` item — but it has a **measured gap**: the regex requires
  `\|\s*(?:ba)?sh\b`, so `curl x.sh | sudo sh` does **not** match, because `sudo` sits between the
  pipe and `sh`. This is a one-line widening of an existing *exfiltration* pattern. It gets its own
  issue and its own sweep — widening a second category inside the CAT-01 PR would violate GATE-04.
- **D-16:** **"10 patterns" is no longer the target.** `PI050`–`PI059` is an ID range, not a quota;
  ten was a planning convenience. Hitting a count by splitting one behaviour across two regexes is
  padding that the FP sweep then has to carry. The ROADMAP criterion becomes "12 new corpus
  payloads; pattern count as the threat model requires, within `PI050`–`PI059`", and the resulting
  number is recorded after the fact.
- **D-17:** "Instructions to disable a hook or a guardrail" **stays in CAT-01** as prose. The split
  is on intent, not on the word "hook": CAT-01 is **removing a control** ("turn off the pre-commit
  hook", "disable that guardrail"); CAT-03 is **installing or maintaining attacker presence** (a
  `hooks` entry that re-writes the payload back). Consistent with D-13's structural key split.

### Claude's Discretion

None — every question in this discussion was answered explicitly. The two "You decide" options
offered (EXPECTED row granularity, and the CAT-01/CAT-03 hook split) were both declined in favour
of an explicit choice.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase requirements and gates
- `.planning/REQUIREMENTS.md` — CAT-01 definition; GATE-01…GATE-05 in full. Note GATE-01's
  "never derived from the patterns" rule, which D-04 operationalises.
- `.planning/ROADMAP.md` §"Phase 3: Tool & permission abuse — CAT-01 (#33)" — **contains two
  criteria this context supersedes** (see D-11 and D-16). Read it, but treat D-11/D-16 as
  authoritative where they conflict.
- `.planning/.continue-here.md` — the three **blocking** anti-patterns carried into this phase, and
  the "ask what the nearest legitimate document looks like" instruction that produced D-06.
- GitHub issue #33 (`gh issue view 33 --repo UnityInFlow/injection-scanner`) — the original pattern
  list and the CRITICAL/HIGH severity split, which is carried forward unchanged.

### The measurement harness — read before touching recall
- `tests/corpus/attack/README.md` — the sourcing rule, the line-oriented format, and the explicit
  note that tool/permission abuse is *deliberately absent* pending v0.2.0. That paragraph must be
  updated in the same commit that adds the payloads.
- `tests/recall_test.rs` — `EXPECTED`, the exact-pin rationale, `payloads()` (the line splitter that
  forces D-01) and `categories()` (the `is_file()` filter behind D-05).
- `tests/corpus/clean/agent-spec.md` — the nearest legitimate document, and its own comment
  explaining why: "full of imperatives addressed to a model, which is exactly the shape an
  injection has; the difference is provenance, not phrasing."

### The engine this phase arms
- `src/frontmatter.rs` — ENG-01's projection contract: the canonical `path = value` form, the
  MAX_DEPTH / MAX_NODES / MAX_VALUE_LEN bounds, and why a projection was chosen over a rule DSL.
- `src/pattern.rs` — the `Pattern` schema; `PatternScope` (D-13's mechanism), and the `raw_only`
  doc comment that sets the precedent D-08 follows.
- `src/patterns/mod.rs` — `include_str!` embedding; a new category file must be registered in
  `load_embedded_patterns()` or it ships as dead YAML.

### Pattern authoring contract
- `.claude/skills/pattern-library/SKILL.md` — required schema fields, the example/counter_example
  contract, catalogue regeneration, and the false-positive gates that fail CI. **Invoke this skill
  (do not just read it) when actually editing `patterns/core/*.yaml`.** It needs updating for D-08's
  new field.
- `docs/PATTERN-CATALOGUE.md` — generated; regenerate with the pattern change. Note the two
  pre-existing sweep findings at lines 73 and 902, identical under v0.1.0 and not introduced here.
- `docs/DETECTION-BACKLOG.md` §Part 2 — cited by #33 as the source for this category's shapes.

### Ecosystem boundary
- `../CLAUDE.md` §"spec-linter — Lint Rules" — S005 `no-wildcard-permissions`, the rule D-10
  deliberately overlaps.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **ENG-01 structural projection** (`src/frontmatter.rs`): parses YAML/TOML/JSON frontmatter to
  `path = value` text. A `scope: frontmatter` pattern is an ordinary regex over that projection — no
  new matching language, no schema change needed for the structural half.
- **`PatternScope`** (`src/pattern.rs`, wired through `src/scanner.rs` at 6 sites): already gates
  prose vs frontmatter passes. `scanner.rs:472` short-circuits the whole structural pass when no
  frontmatter-scoped pattern is loaded — which is why the pass is currently inert.
- **`--baseline`** (`src/baseline.rs`, shipped v0.1.0): the migration path for D-12's behaviour
  change, should consumers need one.
- **Markdown context awareness** (`src/context.rs`) and the `low_confidence` disclosure: relevant to
  the prose half, since a permission directive inside a fenced block still reaches a flattened agent
  context — the finding from `tools/injection-lab`.

### Established Patterns
- **Exact-pin recall** (`tests/recall_test.rs`): counts are pinned exactly, so an *improvement*
  fails the build too. Every recall change is a deliberate two-place edit — the test and the README.
- **Every widening ships the specimen that proves its own FP control** — v0.1.0 practice; D-07
  automates it.
- **A pattern's `name` is a consumer contract.** `pattern_name` ships in the JSON `spec-ci-plugin`
  reads. Widen the `description`, never rename.
- **Self-scan cleanliness:** a CRITICAL finding in `src/` has never been legitimate; `frontmatter.rs`
  carries an inline `injection-scanner:ignore PI028` on its illustrative payload. Note that
  suppression directives apply to the line they sit on — `ignore-next-line` and `ignore-file` are
  distinct, and both have been needed this milestone.

### Integration Points
- New category file `patterns/core/<name>.yaml` → must be added to `load_embedded_patterns()`.
- New clean-corpus documents → picked up automatically by `tests/corpus_test.rs`.
- New attack payloads → `EXPECTED` in `recall_test.rs` **and** the README recall table.
- New schema field → `src/pattern.rs`, the pattern-library skill, `PATTERNS.md`, and the policy
  ratchet.

</code_context>

<specifics>
## Specific Ideas

- The **runbook telling a human to bypass permissions inside a disposable container** is the
  document to write first among the four controls. It is the one most likely to force a narrowing
  before merge, and finding that out early is cheaper than finding it out in the sweep.
- The pre-pattern baseline run (D-04) should **name** any incidental hits from the existing 48
  patterns rather than reporting a bare count — spillover attributed by pattern id is what makes the
  before/after delta interpretable.
- GATE-03's sweep is the gate that caught the ENG-02 panic. Run `check .` with the **release**
  binary, not just `cargo test`.

</specifics>

<deferred>
## Deferred Ideas

- **`PI028` `| sudo sh` gap** — one-line widening of an existing exfiltration pattern
  (`patterns/core/exfiltration.yaml:144`). Own issue, own sweep; out of this PR under GATE-04.
- **Bare dangerous-command detector** (`sudo`, `rm -rf`, `chmod 777` in a skill body) — a distinct
  threat model needing its own false-positive story. Candidate for the v0.3.0 backlog /
  `docs/DETECTION-BACKLOG.md`.
- **Backfill the mutation-tested FP-control field across the existing 48 patterns**, including the
  two known-slack v0.1.0 controls. Deliberately outside CAT-01's PR under GATE-04; wants its own
  issue.
- **`model:` and `system:` frontmatter overrides** — unclaimed by any category. `system:` is
  arguably role-override territory; decide when a category needs it.
- **The two pre-existing `docs/PATTERN-CATALOGUE.md` sweep findings** (lines 73, 902) — a standing
  violation of the "expect `[]`" rule with no issue of its own. Predates this milestone.

</deferred>

---

*Phase: 3-tool-permission-abuse-cat-01-33*
*Context gathered: 2026-08-31*
