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

- [x] **Phase 1: Structural frontmatter engine (ENG-01, #32)** — Parse YAML/TOML/JSON frontmatter
      with a real parser; inspect `allowed-tools`, `tools`, `permissions`, `mcpServers`, `hooks`,
      `model`/`system` as data. Unblocks Phases 3 and 4.

- [x] **Phase 2: Recursive decoder (ENG-02, #30)** — base64, hex, URL, HTML entities, `\u` escapes,
      applied recursively with a decode-bomb bound. Takes recall to 59/60.

- [x] **Phase 3: Tool & permission abuse (CAT-01, #33)** — `PI050`–`PI059`.
- [x] **Phase 4: MCP & tool-description poisoning (CAT-02, #34)** — `PI060`–`PI069`.
- [x] **Phase 5: Persistence & lifecycle hijack (CAT-03, #35)** — `PI070`–`PI079`.

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

**Plans:** 7/7 plans executed

Plans:

- [x] 04-01-PLAN.md — Pre-edit GATE-03 baseline, per-category structural corpus collector, GATE-05 range repair
- [x] 04-02-PLAN.md — 12 threat-model payloads, wrapper-shape projection control, measured pre-pattern baseline (GATE-01)
- [x] 04-03-PLAN.md — Clean-corpus boundary specimens, vendored registry sample with provenance (D-06)
- [x] 04-04-PLAN.md — PI060-PI062 config hygiene, MEDIUM, `scope: frontmatter` (D-03)
- [x] 04-05-PLAN.md — PI063-PI065 tool-description poisoning, HIGH, prose (D-01, D-02)
- [x] 04-06-PLAN.md — PI066-PI069 shadowing and rug-pull heuristics, MEDIUM, prose (D-04)
- [x] 04-07-PLAN.md — Whole-category sweep, number reconciliation, deferral issues, phase close

### Phase 5: Persistence & lifecycle hijack — CAT-03 (#35)

**Goal:** `PI070`–`PI079`. Payloads that survive the obvious cleanup.

**Success criteria**

- 9 patterns in `PI070`-`PI079` (`PI078` deliberately unallocated, amended by D-07 in plan 05-06); 12 new corpus payloads
- Self-rewriting instructions, hook and lifecycle abuse, memory-file poisoning
- At least one pattern detects an instruction to write *into* a file the agent will re-read

> **`05-CONTEXT.md` is authoritative where it sharpens these criteria — do not re-derive the
> superseded readings.** Three things were settled there after measurement. (1) The range has **9**
> free slots, not 10: `PI070` already shipped via PR #110, so the phase plans `PI071`–`PI079` and
> the "10 patterns" line above counts the whole range rather than the phase's own additions (D-07);
> `PI078`/`PI079` are provisional and the criterion is amended in the same change if either is
> dropped. (2) The 12 corpus payloads **replace** the 6 inherited with PR #110, which were
> GATE-01-tainted — payload 1 was byte-identical to `PI070`'s own `example` — so the pinned recall
> row is expected to fall below its published 6/6 and that is the gate working (D-01). (3) Issue
> #35's "CRITICAL across the board" is **not** followed: `PATTERNS.md` rule 3 governs severity and
> nothing in this range ships in that tier (D-06).
>
> **Amended in plan 05-06 under D-07's authority.** `PI078` (remote-lifecycle-hook-endpoint) was
> measured and dropped, so the range ships 9 patterns (`PI070`-`PI077` and `PI079`) and the id
> `PI078` stays unallocated. The measurement: a dotted-host remote-endpoint rule fires on the
> structural attack payload 04 and on the clean `persistence-corporate-audit-endpoint.json`
> specimen alike, and swapping the bound event, the host's registrable domain and the URL between
> the two documents moved nothing. `PI079` was kept (MEDIUM) on its own measured criterion.

**Plans:** 7/7 plans executed

Plans:

- [x] 05-01-PLAN.md — GATE-03 pre-edit baseline, 12 blind-written payloads, measured pre-pattern pins (D-01, D-03, D-05)
- [x] 05-02-PLAN.md — Nine clean-corpus specimens and the two mutation proofs that make D-03's amendment and D-04's conjuncts load-bearing (D-02, D-03, D-04)
- [x] 05-03-PLAN.md — `PI070` object/verb widening and `PI071` agent-persistence-nonmodal, HIGH, prose (D-06)
- [x] 05-04-PLAN.md — `PI072` self-propagation, `PI074`/`PI075` memory arms, and the self-matching backlog bullets (D-02, D-05, D-06)
- [x] 05-05-PLAN.md — `PI073` instruction-file-write-directive (MEDIUM) and `PI076` agent-hook-registration-directive (D-04, D-06)
- [x] 05-06-PLAN.md — `PI077` structural lifecycle arm, plus `PI078`/`PI079` resolved on measured criteria (D-03, D-06, D-07)
- [x] 05-07-PLAN.md — Whole-category GATE-03 delta, number reconciliation, deferral issues, #35 close-out, pre-PR gates

## Gates applied to every phase

| Gate | Rule |
|---|---|
| GATE-01 | 12 corpus payloads per category, from the threat model, never derived from patterns |
| GATE-02 | Recall counts pinned **exactly** — an improvement fails the build too |
| GATE-03 | ~1,300-file third-party sweep on every pattern change |
| GATE-04 | No reviewable unit widens more than one category |
| GATE-05 | The false-positive control is mutation-tested |

Also standing: `main` stays strictly linear (0 merge commits); a pattern's `name` is a consumer
contract (`pattern_name` ships in the JSON `spec-ci-plugin` reads) — widen the `description`, never
rename.

## Progress

| Phase | Requirement | Issue | Status |
|---|---|---|---|
| 1. Structural frontmatter engine | ENG-01 | #32 | **Done** — PR #104 |
| 2. Recursive decoder | ENG-02 | #30 | **Done** — PR #108 |
| 3. Tool & permission abuse | CAT-01 | #33 | **Done** — PR #109 |
| 4. MCP & tool-description poisoning | CAT-02 | #34 | **Done** — PR #120 + PR #136 |
| 5. Persistence & lifecycle hijack | CAT-03 | #35 | **Complete.** 7/7 plans; 9 patterns `PI070`-`PI077` and `PI079`, `PI078` unallocated. Held-out recall 2/12 published as the CAT-03 number, development 7/12, library-wide 103/115. Merged as PR #185 (48 commits, rebased); #35 closed. The blocking review finding #183 was fixed before the PR, not deferred, and its own cost filed as #184 |

**Library (Phase 5 close, plan 05-07):** **79 patterns**, measured directly from the loader and asserted by
`test_total_pattern_count`: the 71 at Phase 4's close plus CAT-03's eight new patterns (`PI071`-`PI077` and `PI079`; `PI070`
had shipped from PR #110 and is not a new count). `PI078` is deliberately unallocated, so the range ships 9 patterns, not
the 10 its id span names.
**Recall (Phase 5 close, plan 05-07):** **103/115 (89.6%)** on the development corpus, up from 102/109 at Phase 4's close.
The denominator moved by +6, not by the 12 CAT-03 payloads alone, because the 6 inherited and GATE-01-tainted CAT-03
payloads were replaced (D-01) and the category's pre-pattern baseline was 97/115. CAT-03's own development rows are
**7/12 (58.3%)** (prose 4/7, structural 3/5). **The published CAT-03 number is the sealed held-out set's: 2/12 (16.7%)**
(prose 2/8, structural 0/4), reported beside the development score and **never summed into the 115**. The projection
this milestone opened with did not anticipate a held-out set, and the gap between 58.3% and 16.7% is the finding.

**Library:** **71 patterns** as of Phase 4's close (04-07), measured directly from the loader —
corrected from the ~78 projected when the milestone opened. CAT-02 (`PI060`-`PI069`) added
exactly its planned 10; the difference from the projection is other, un-milestoned pattern work
that landed on `main` during the same window (PR #110's `PI058`/`PI070`/`PI110`-`PI113`, plus
subsequent widenings), independent of this phase's own count. CAT-03 (`PI070`-`PI079`, Phase 5)
is already partially seeded by `PI070` from that same PR.
**Recall:** **102/109 (93.6%)** as of Phase 4's close (04-07), corrected from the 76/84 (90.5%)
projection recorded as of 04-02. The denominator grew to 109, not the originally projected 84 —
25 payloads beyond CAT-02's own 12 landed from other PRs during the same window (#110's
multilingual range, #122's widenings, and independent corpus growth in encoding/role-override).
CAT-02's own row is now **9/12 (75%)**, up from its measured pre-pattern baseline of 6/12 — the
remaining three misses are the two rug-pull structural payloads (D-05's deferred structural
cross-reference; see `deferred-items.md`) and one deliberately-undetected unpinned-registry-install
payload (D-03's accepted cost).
