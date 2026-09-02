# Requirements: injection-scanner v0.2.0

**Defined:** 2026-08-30
**Milestone:** Agent-shaped attacks
**Core Value:** Catch prompt injection attacks before they reach production

## Why this milestone exists

v0.1.0 made the scanner detect the attacks its README already claimed — recall went from
**10/60 to 56/60** by rewriting four categories from literal phrase lists into
verb × modifier × object matrices. That closed the credibility gap. It did not add a single new
*kind* of attack.

Every one of the 48 patterns targets a payload aimed at a **chat model reading prose**. None target
the payloads aimed at an **agent with tools** — a wildcard permission grant in frontmatter, an
instruction hidden in an MCP tool `description` that the user never sees, a lifecycle hook that
re-installs the attacker's instructions after the file is cleaned.

That class is what makes this tool different from a grep for known phrases, and it is
the most active area of agent-security research. It is also the class that `spec-linter` S005
gestures at from the lint side and nothing checks from the content side.

**Thesis: the scanner learns to read agent configuration, not just agent prose.**

## v0.2.0 Requirements

Ordered by dependency. Each is one PR, with its own false-positive sweep.

### Engines

- [ ] **ENG-01** (#32): Frontmatter in YAML / TOML / JSON is parsed with a real parser and inspected
      as structured data — `allowed-tools`, `tools`, `permissions`, `mcpServers`, `hooks`, `model`
      and `system` overrides — rather than matched with regex over raw lines. Findings from
      structured data carry near-zero false-positive risk because the shape is unambiguous, which
      is what lets them sit at CRITICAL.

- [ ] **ENG-02** (#30): A recursive decoder walks base64, hex, URL-encoding, HTML entities and
      `\u` escapes, re-running detection on each decoded layer, bounded against decode bombs.
      Nested encodings (base64 inside an HTML entity inside a URL escape) are the real shape and
      are what three separate single-layer decoders would each miss. **Includes a reversal
      transform** — see the corrected recall attribution below.

### Categories

- [x] **CAT-01** (#33): `PI050`–`PI059` — tool & permission abuse. Payloads that widen the agent's
      own authority: wildcard tool grants, `--dangerously-skip-permissions` / `bypassPermissions`
      directives, instructions to edit `settings.json` or disable a hook. The agentic equivalent of
      privilege escalation. Depends on **ENG-01** for the frontmatter half.

- [ ] **CAT-02** (#34): `PI060`–`PI069` — MCP & tool-description poisoning. Instructions hidden in
      a tool `description`, read by the model on every call and never shown to the user; unpinned
      `npx -y` servers and `http://` endpoints; cross-tool shadowing; rug-pull markers that are
      version- or date-conditional. Depends on **ENG-01** for the `mcpServers` half.

- [ ] **CAT-03** (#35): `PI070`–`PI079` — persistence & lifecycle hijack. Payloads that survive
      the obvious cleanup: instructions that re-write themselves into a config, hook and lifecycle
      abuse, memory-file poisoning.

### Gates — non-negotiable, they are what made v0.1.0 trustworthy

- [x] **GATE-01**: Every new category adds **12 corpus payloads** to `tests/corpus/attack/`,
      written from the threat model and **never derived from the patterns** — a corpus built from
      each pattern's own `example` scores 100% by construction and measures nothing.

- [x] **GATE-02**: `tests/recall_test.rs` continues to pin counts **exactly**, not as a floor, so
      an improvement fails the build too and the published number cannot go stale.

- [x] **GATE-03**: Every pattern change is swept against ~1,300 files of **real third-party
      documentation**, not only the 18-file clean corpus. The 2026-08-29 sweep found 2 CRITICAL and
      25 HIGH false positives the corpus was silent about.

- [ ] **GATE-04**: Each category ships as **its own PR**. Widening four categories at once produces
      an unreviewable false-positive blast radius — this is a recorded lesson, not a preference.
      **Status (Phase 3, 2026-09-02):** CAT-01 shipped alone, so the one-category-at-a-time
      substance of this gate holds. Left unchecked because this milestone's GSD branching strategy
      is `none` — Phase 3 committed directly to `main` and no PR object exists to point at. Check
      it when the work is raised as a PR, or amend the gate to say "its own reviewable unit".

- [x] **GATE-05**: The false-positive control in each PR is **mutation-tested** — two of four
      v0.1.0 widenings had a control the corpus was not actually holding.

## Deferred to v0.3.0

Tracked, not in this roadmap.

| Req | Issue | Why deferred |
|---|---|---|
| `PI080`–`PI089` indirect / RAG-borne | #36 | Real, but the agentic three are the differentiator; this is the natural follow-on |
| `PI090`–`PI099` credential harvesting | #37 | Overlaps `spec-linter` S003; needs a scope boundary decision first |
| `PI100`–`PI109` output-format hijack | #38 | Lower severity ceiling than the agentic set |
| `PI110`–`PI119` multilingual evasion | #39 | Every pattern is English-only — a whole separate problem, and labelled `help wanted` |
| `PI120`–`PI129` delimiter spoofing | #40 | Narrow; sequence after the engines exist |
| Invisible-character density heuristic | #31 | Heuristic rather than pattern; needs its own FP story |

## Out of Scope

| Item | Reason |
|---|---|
| Library / CLI split (part of #41) | **Zero blocked consumers.** `kore-runtime` has no reference to this tool, `agent-sandbox` and `safe-scrape` have no repo. `spec-ci-plugin` shells out successfully in production. Do it when a real consumer is blocked, not before — the API shape will be wrong if guessed. |
| Aho-Corasick prefilter (#4) | Perf budget is met with 5× headroom — 41ms against 200ms. Headroom for a growing library, not a fix. Revisit if the library passes ~150 patterns. |
| Runtime filter mode (#11) | Depends on `agent-sandbox`, which has no repo. |
| Windows binary (#9) | Real unmet constraint, but distribution not detection. Check the `mcp-hub` HUB-V2-02 precedent first — `cfg(unix)` deps that do not link. |

## Traceability

| Requirement | Issue | Phase | Status |
|---|---|---|---|
| ENG-01 | #32 | Phase 1 | Pending |
| ENG-02 | #30 | Phase 2 | Pending |
| CAT-01 | #33 | Phase 3 | Pending |
| CAT-02 | #34 | Phase 4 | Pending |
| CAT-03 | #35 | Phase 5 | Pending |
| GATE-01..05 | — | All phases | Pending |

**Coverage:** 5 feature requirements, 5 mapped. 5 gates apply to every phase. Unmapped: 0 ✓

## Success criteria for the milestone

1. Recall reaches **58/60** on the existing corpus.

   **Corrected 2026-08-30.** This previously read 59/60 "because ENG-02 closes the three base64
   misses". Measured, that was wrong — only one of the three is base64:

   | Miss | Cause | Closed by ENG-02 |
   |---|---|---|
   | base64 payload | genuine decoder gap | **yes** |
   | reversed text | a reversal transform, not a decoding one | **yes** — added to scope |
   | fully despaced | the documented non-goal in `normalize.rs`: `i g n o r e a l l` collapses to `ignoreall`, and every pattern joins words with `\s+` | **no** — needs a pattern-set rewrite |
   | role-override precedence | not separable from ordinary config documentation | **no** — deliberate |

   The claim came from a comment in `tests/recall_test.rs` and was propagated into this file, into
   STATE.md and into the v0.1.0 release notes without being checked. Two misses stay, both for
   stated reasons.

2. Three new categories are measured on their **own** 36 new payloads, and that number is published
   in the README alongside the existing table.

3. The clean corpus and the third-party sweep both stay at **zero** regressions.
4. Library grows 48 → ~78 patterns with the per-pattern test policy ratchet still green.

---
*Requirements defined: 2026-08-30*
