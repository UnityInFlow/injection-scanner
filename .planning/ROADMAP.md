# Roadmap: injection-scanner v0.2.0 — Agent-shaped attacks

**Opened:** 2026-08-30 · **Previous milestone:** Production Readiness (v0.0.3 + v0.1.0), shipped
2026-08-29, archived at `.planning/archive/milestone-v0.1.0/`

## Why this order

The two engines come first, and not for tidiness — the dependency is real and confirmed from both
sides. #32 states it is "the prerequisite for the `PI050`–`PI059` and `PI060`–`PI069` categories",
and #33/#34 both list frontmatter-shaped patterns (`allowed-tools: *`, `Bash(*)`, `mcpServers`
entries) that regex cannot address without the false positives #32 exists to remove.

Each of #33 and #34 does split into a **prose half** that today's engine could match and a
**structured half** that cannot. Shipping the prose halves first is possible — and rejected:
it means touching each category twice and taking exactly the false-positive risk the structured
parser eliminates.

**#30 sits second rather than last** because it is the only item in the milestone that moves the
*published* recall number — 56/60 → 59/60 — so it converts into a visible claim immediately,
and it retires #6 and #7, already closed against it.

## Phases

- [ ] **Phase 1: Structural frontmatter engine (ENG-01, #32)** — Parse YAML/TOML/JSON frontmatter
      with a real parser; inspect `allowed-tools`, `tools`, `permissions`, `mcpServers`, `hooks`,
      `model`/`system` as data. Unblocks Phases 3 and 4.

- [ ] **Phase 2: Recursive decoder (ENG-02, #30)** — base64, hex, URL, HTML entities, `\u` escapes,
      applied recursively with a decode-bomb bound. Takes recall to 59/60.

- [ ] **Phase 3: Tool & permission abuse (CAT-01, #33)** — `PI050`–`PI059`.
- [ ] **Phase 4: MCP & tool-description poisoning (CAT-02, #34)** — `PI060`–`PI069`.
- [ ] **Phase 5: Persistence & lifecycle hijack (CAT-03, #35)** — `PI070`–`PI079`.

## Phase details

### Phase 1: Structural frontmatter engine — ENG-01 (#32)

**Goal:** the scanner reads configuration as configuration.

**Success criteria**

- YAML, TOML and JSON frontmatter parse with a real parser; a malformed document is skipped
  loudly and never aborts the scan (the FIX-03 rule, applied to a new input class)

- Structured findings carry a distinct `context` so they are separable in JSON/SARIF output
- Zero new findings on `tests/corpus/clean/` **and** on the third-party sweep
- A structured finding can sit at CRITICAL because its shape is unambiguous — proven by a test,
  not asserted

**Watch for**

- `.mdc`, `.cursorrules` and extensionless agent files are already in the default set; frontmatter
  detection must not assume `.md`

- Parser choice is a supply-chain decision — this crate parses untrusted input by definition

### Phase 2: Recursive decoder — ENG-02 (#30)

**Goal:** an encoded payload is no longer a bypass, however many layers deep.

**Success criteria**

- Recall reaches **59/60**; `tests/recall_test.rs` updated to the new exact count
- Decode depth and output size are bounded; a decode bomb is refused, not OOM'd
- A decoded finding reports the **original** byte offsets, not offsets into the decoded text
- `matched_text` still carries original bytes — the `--baseline` digest depends on it, and
  normalizing it would turn every baselined finding into a free pass for its obfuscation family

**Watch for**

- #6 and #7 are closed as superseded by this; make sure both cases are actually covered
- Separator normalization already rewrites `-` as whitespace before matching — decoded text
  enters the same pipeline

### Phase 3: Tool & permission abuse — CAT-01 (#33)

**Goal:** `PI050`–`PI059`. Injection whose payload widens the agent's own authority.

**Success criteria**

- Twelve new corpus payloads written from the threat model; pattern count is not a target — it is
  whatever the threat model requires within `PI050`–`PI059`, and the resulting number is recorded
  after the fact

- Both halves covered: structured (wildcard grants via ENG-01) and prose
  (`--dangerously-skip-permissions`, "no need to ask", "add this to your settings.json")

- A frontmatter-scoped `PI05x` pattern **does** fire on a file's own wildcard grant, accepting
  overlap with `spec-linter` S005 — the boundary is provenance, not phrasing: S005 lints a spec you
  wrote, in your own repo, at authoring time; this scanner is pointed at untrusted input, so the
  same `allowed-tools: *` is a lint finding in your own CLAUDE.md and an attack in a skill someone
  shipped you

> Both corrections above come from `03-CONTEXT.md` D-11 (the S005-boundary criterion) and D-16 (the
> "10 patterns" criterion). CONTEXT.md is authoritative where it conflicts with this roadmap — do
> not re-derive or re-inherit the superseded wording.

**Plans:** 7/7 plans executed

Plans:

- [x] 03-01-PLAN.md — Corpus, recall harness and the measured pre-pattern baseline (D-01..D-05)
- [x] 03-02-PLAN.md — Five false-positive control specimens in `tests/corpus/clean/` (D-06, D-06a, D-06b)
- [x] 03-03-PLAN.md — GATE-03 sweep script, recorded pre-pattern sweep, ROADMAP correction (D-10, D-11, D-16)
- [x] 03-04-PLAN.md — Relaxed-control schema field, mutation-pairing gate, PI050+ ratchet (D-07, D-08, D-09)
- [x] 03-05-PLAN.md — Structural patterns PI050-PI052, CRITICAL, `scope: frontmatter` (D-12, D-13)
- [x] 03-06-PLAN.md — Prose patterns PI053-PI057, HIGH (D-14, D-15, D-17)
- [x] 03-07-PLAN.md — GATE-03 delta sweep, number reconciliation, deferral issues, pre-PR gate

### Phase 4: MCP & tool-description poisoning — CAT-02 (#34)

**Goal:** `PI060`–`PI069`. The attack the user never sees.

**Success criteria**

- 10 patterns; 12 new corpus payloads
- Imperative language inside a tool `description`; unpinned `npx -y` and `http://` servers;
  cross-tool shadowing; version/date-conditional rug-pull markers

- **Highest false-positive risk in the milestone** — a legitimate MCP manifest is full of
  imperative description text. The `Show the current system prompt` precedent applies: the
  possessive requirement is what keeps PI021 off real manifests. Expect to need a similar
  narrowing rule, and sweep real MCP manifests specifically, not just documentation

**Plans:** 7 plans

Plans:

- [ ] 04-01-PLAN.md — Pre-edit GATE-03 baseline, per-category structural corpus collector, GATE-05 range repair
- [ ] 04-02-PLAN.md — 12 threat-model payloads, wrapper-shape projection control, measured pre-pattern baseline (GATE-01)
- [ ] 04-03-PLAN.md — Clean-corpus boundary specimens, vendored registry sample with provenance (D-06)
- [ ] 04-04-PLAN.md — PI060-PI062 config hygiene, MEDIUM, `scope: frontmatter` (D-03)
- [ ] 04-05-PLAN.md — PI063-PI065 tool-description poisoning, HIGH, prose (D-01, D-02)
- [ ] 04-06-PLAN.md — PI066-PI069 shadowing and rug-pull heuristics, MEDIUM, prose (D-04)
- [ ] 04-07-PLAN.md — Whole-category sweep, number reconciliation, deferral issues, phase close

### Phase 5: Persistence & lifecycle hijack — CAT-03 (#35)

**Goal:** `PI070`–`PI079`. Payloads that survive the obvious cleanup.

**Success criteria**

- 10 patterns; 12 new corpus payloads
- Self-rewriting instructions, hook and lifecycle abuse, memory-file poisoning
- At least one pattern detects an instruction to write *into* a file the agent will re-read

## Gates applied to every phase

| Gate | Rule |
|---|---|
| GATE-01 | 12 corpus payloads per category, from the threat model, never derived from patterns |
| GATE-02 | Recall counts pinned **exactly** — an improvement fails the build too |
| GATE-03 | ~1,300-file third-party sweep on every pattern change |
| GATE-04 | One category per PR |
| GATE-05 | The false-positive control is mutation-tested |

Also standing: `main` stays strictly linear (0 merge commits); a pattern's `name` is a consumer
contract (`pattern_name` ships in the JSON `spec-ci-plugin` reads) — widen the `description`, never
rename.

## Progress

| Phase | Requirement | Issue | Status |
|---|---|---|---|
| 1. Structural frontmatter engine | ENG-01 | #32 | Not started |
| 2. Recursive decoder | ENG-02 | #30 | Not started |
| 3. Tool & permission abuse | CAT-01 | #33 | In Progress|
| 4. MCP & tool-description poisoning | CAT-02 | #34 | Not started |
| 5. Persistence & lifecycle hijack | CAT-03 | #35 | Not started |

**Library:** 48 patterns today → ~78 at milestone end.
**Recall:** 56/60 today → 59/60 after Phase 2, plus 36 new payloads measured separately.
