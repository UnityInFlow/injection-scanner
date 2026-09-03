---
gsd_state_version: 1.0
milestone: v0.2.0
milestone_name: Agent-shaped attacks
status: in_progress
stopped_at: Phase 3 shipped — PR #109 merged, #33 closed. Phase 4 (CAT-02) not started.
last_updated: "2026-09-03T07:16:56.000Z"
state_head: 465c7e0b1aa3bfc780127d488102fa4af8b75e64
progress:
  total_phases: 5
  completed_phases: 3
  total_plans: 9
  completed_plans: 9
  percent: 60
---

# State: injection-scanner

## Project Reference

See: `.planning/PROJECT.md`
**Core value:** Catch prompt injection attacks before they reach production
**Milestone:** **v0.2.0 — Agent-shaped attacks** (opened 2026-08-30)

> Full history for the previous milestone — 412 lines of session notes — is archived at
> `.planning/archive/milestone-v0.1.0/STATE.md`, alongside its REQUIREMENTS, ROADMAP and phases.

## Current Phase

**Phase 4 — MCP & tool-description poisoning (CAT-02, #34)** · status: **not started**

Phase 3 shipped 2026-09-02 as **PR #109** (rebase-merged, issue #33 auto-closed): `PI050`-`PI057`,
the `relaxed_pattern` schema field, and ADR-004. Its code review found one critical false positive
— **CR-01**, three prose patterns firing on *prohibitions* ("Never run with
`--dangerously-skip-permissions`") at HIGH, the severity `install-hook` blocks commits at. Fixed in
quick task `260902-jhy` before the PR, by structural tightening rather than a negation guard: real
payloads carry a negator inside the matched sentence, so a guard would have suppressed `PI053`'s
and `PI057`'s own `example` values and failed `pattern_example_test`.

Phases 1 and 2 shipped 2026-08-30; both engines are done. The two remaining phases are pattern
categories, one PR each (GATE-04). `main` clean, in sync, **353 tests**, CI green, zero open PRs,
still 0 merge commits.

**Carried into Phase 4:** `PI050+` patterns must ship a `relaxed_pattern` (GATE-05, ADR-004). The
generalizable CR-01 rule — *fix negation where the negator sits*: clause-initial anchoring when it
precedes the span, an enumerated filler set when it sits inside — applies directly to CAT-02's
prose arms. Open follow-ups: **WR-02** (structural corpus README documents 1 of 5 payloads),
**WR-03** (`scripts/gate03-sweep.sh` helpers declare no `local`), and two pre-existing
`docs/PATTERN-CATALOGUE.md` self-matches (`PI001` at :74, `PI031` at :903) that predate PR #109.

## The milestone in one paragraph

v0.1.0 made the scanner detect the attacks its README already claimed — recall **10/60 -> 56/60**.
It did not add a single new *kind* of attack. All 48 patterns target payloads aimed at a **chat
model reading prose**. None target payloads aimed at an **agent with tools**: a wildcard permission
grant in frontmatter, an instruction hidden in an MCP tool `description` the user never sees, a
lifecycle hook that reinstalls the attacker's instructions after the file is cleaned.
**v0.2.0 teaches the scanner to read agent configuration, not just agent prose.**

## Phases

| Phase | Requirement | Issue | Status |
|---|---|---|---|
| 1 | ENG-01 structural frontmatter engine | #32 | **Done** — PR #104 |
| 2 | ENG-02 recursive decoder | #30 | **Done** — PR #108, also closed #6 and #7 |
| 3 | CAT-01 tool & permission abuse `PI050-059` | #33 | **Done** — PR #109 |
| 4 | CAT-02 MCP & tool-description poisoning `PI060-069` | #34 | Not started |
| 5 | CAT-03 persistence & lifecycle hijack `PI070-079` | #35 | Not started |

Engines first, and the dependency is real rather than tidiness: #32 states it is the prerequisite
for `PI050-059` and `PI060-069`, and both categories carry frontmatter-shaped patterns
(`allowed-tools: *`, `Bash(*)`, `mcpServers`) that regex cannot address without the false positives
#32 exists to remove. #30 is second because it is the only item that moves the **published** recall
number, 56/60 -> 59/60.

## Standing gates — these are what made v0.1.0 trustworthy

| Gate | Rule |
|---|---|
| GATE-01 | 12 corpus payloads per new category, written **from the threat model**, never derived from the patterns |
| GATE-02 | Recall pinned **exactly**, not as a floor — an improvement fails the build too |
| GATE-03 | ~1,300-file third-party sweep on every pattern change; the 18-file clean corpus is not sufficient evidence |
| GATE-04 | One category per PR — widening four at once is an unreviewable FP blast radius |
| GATE-05 | The false-positive control is mutation-tested — two of four v0.1.0 widenings had a control the corpus was not holding |

Also standing: `main` stays strictly linear. A pattern's `name` is a **consumer contract** —
`pattern_name` ships in the JSON `spec-ci-plugin` reads, so widen the `description`, never rename.

## Detection recall — the published number

| Category | Detected | Recall |
|---|---|---|
| Data Exfiltration | 12/12 | 100% |
| Instruction Injection | 12/12 | 100% |
| Jailbreaks | 12/12 | 100% |
| Role Override | 11/12 | 92% |
| Encoding/Obfuscation | 11/12 | 91.7% |
| **Total** | **58/60** | **96.7%** |

Reached **58/60** on 2026-08-30 when ENG-02 landed.

**Corrected 2026-08-30.** This said 59/60, "closing the three base64 misses". Only ONE of the three
is base64. The second is reversed text (a different transform, now in ENG-02's scope); the third is
fully despaced text, which is the *documented non-goal* in `normalize.rs` — `i g n o r e a l l`
collapses to `ignoreall` and every pattern joins words with `\s+`, so closing it means rewriting
the pattern set rather than the input. Two misses therefore remain, both for stated reasons.

## Tracking

- **GitHub milestone:** `v0.2.0 — Agent-shaped attacks` (milestone #6) — 5 issues
- **Project board:** https://github.com/orgs/UnityInFlow/projects/4 — org-wide, 63 items, with
  `Phase` and `Priority` fields populated for this milestone

- **Deferred:** the `v0.3.0` milestone holds 10 issues — pattern categories #36-#40 plus #31, #41,
  #10, #11, #4

## Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260902-jhy | Fix CR-01 negation blindness in PI053/PI056/PI057; fold in WR-01 `PATTERNS.md` category row | 2026-09-02 | `db2a575` | [260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi](./quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/) |

## Milestone hygiene done 2026-08-30

Four milestones were open that should not have been. `v0.0.1`, `v0.0.3` and `v0.1.0` had all
shipped — and `v0.1.0` still held issue #4 (Aho-Corasick), which **shipped without it**. A bare
`v0.2.0` milestone already existed and a second was created before checking; the old one is now
renamed `v0.2.0 (early draft — superseded by milestone #6)` and closed. Issues #4, #31 and #11
moved to `v0.3.0`. All four shipped milestones are closed.

## Open decisions

**#41 library split — deliberately not in this milestone.** It claims three blocked consumers and
has **zero**: `kore-runtime` is public and shipped but contains no reference to this tool,
`agent-sandbox` has no repo, `safe-scrape` has no repo. `spec-ci-plugin` shells out to the verified
binary in production and that path is hardened. Do the split when a real consumer is blocked — the
API shape will be guessed wrong otherwise. The *other* half of #41 (crates.io, binstall metadata,
Homebrew) has real users and is cheap; the `.pre-commit-hooks.yaml` sub-item in it already shipped
and is stale.

**Windows binary (#9), narrowed 2026-08-30.** v0.1.0 ships 4 of the 5 targets #9 asked for; only
Windows x86_64 is missing, and it *is* a documented root-`CLAUDE.md` constraint. Read the `mcp-hub`
HUB-V2-02 precedent first — unguarded `cfg(unix)` deps that would not link.

## Session Notes

- 2026-08-30 (Phase 2): **ENG-02 shipped — recall 56/60 -> 58/60.** Three things found by
  measuring rather than assuming, and all three would have shipped silently.
  (1) **A panic in production code that 16 green unit tests missed.** `tail[..12]` sliced at a
  fixed byte offset, which crashes on any file with a multi-byte char near an `&` — a `·` in this
  repo's own source was enough. Found only by running the binary over the repo. **Unit tests do
  not substitute for the sweep**; the sweep is what exercises real bytes.
  (2) **Reversal is an involution**, so recursing on it produced `reversed -> reversed -> base64`
  for what is simply base64. Restricted to top level.
  (3) **Reversal was 137ms of a 143ms regression** — 84% of the cost for one payload in sixty,
  because every line's reversal was handed to all 48 patterns. A generic function-word gate on the
  reversed text cut the overhead from 28% to 3.3%. The gate uses **generic** words, never payload
  vocabulary: keying it on `ignore` would mean a new pattern silently needs a decoder change to be
  reachable.
  Also: `tests/decode_test.rs` needed `ignore-file` — the decoder makes its own fixtures visible
  for the first time, which is the tool correctly detecting its own test data.

- 2026-08-30 (Phase 1): **ENG-01 shipped.** The design worth carrying: rather than a rule DSL in
  the pattern schema, parsed config is **projected to canonical `path = value` text** and the
  existing regex engine runs against it, gated by a new `scope: frontmatter` field. One schema
  field instead of a second matching language, and structural rules earn confidence 1.0 because a
  parser resolved a real key — not because a sentence looked suspicious.
  **Three things worth not rediscovering.**
  (1) **The "silent on prose" test is vacuous without a control.** A scope test passes equally if
  the regex simply fails to match. `a_prose_scoped_rule_would_have_fired_on_that_same_prose` fires
  the *same sentence* through a prose-scoped rule to prove the silence is scope. This is GATE-05
  applied to a mechanism rather than a pattern.
  (2) **The structural pass is inert without a frontmatter-scoped pattern**, by design — the
  scanner skips parsing entirely. My first YAML-bomb test "passed" in 0.02s because of this and
  measured nothing. Any test of this pass must load a probe via `--patterns`.
  (3) **Behaviour-unchanged was proven, not asserted**: the published v0.1.0 binary and this build
  both report 728 findings on this repo, identical. That is the strongest form of GATE-03 — no
  pattern changed, so nothing could move.
  Also: a self-scan finding landed in `src/` for the first time (a doc-comment illustration of a
  pipe-to-shell payload). Suppressed inline with rationale rather than weakened, and `tests/`
  needed `ignore-next-line`, not `ignore` — the directive applies to the line it sits on.

- 2026-08-30: **v0.2.0 opened.** Scope decided on evidence, not instinct: the library is 48
  patterns, so three agentic categories is +30 (1.6x) against +80 (2.7x) for all eight, and the
  corpus cost is 36 new threat-model payloads against 96. GATE-04 (one category per PR) is what
  makes eight categories in one milestone unreviewable. #6 and #7 closed as genuinely superseded by
  #30. Previous milestone archived to `.planning/archive/milestone-v0.1.0/`.

- **Three lessons carried forward** from the archived milestone, because they will bite again:
  **(1)** A moving alias tag makes "merged", "released" and "reaching users" three different states,
  and only the third counts — `spec-ci-plugin`'s `v1` sat one commit behind and the fix reached
  nobody while every gate showed green.
  **(2)** `gh attestation verify` fails locally on `gh` 2.55.0 with
  `unsupported tlog public key type: PKIX_ED25519`. It is the client, not the release — proven by
  running it against a known-good v0.0.3 binary.
  **(3)** `cargo build --locked` cannot refresh `Cargo.lock` after a version bump; `--locked` exists
  to refuse exactly that. Run `cargo check --offline` first, then the locked build to verify.

## Session Continuity

Last session: 2026-09-01T10:48:34.004Z
Stopped at: Phase 3 planned — 7 plans, 4 waves, verification passed
`/gsd-discuss-phase 3` (CAT-01, #33) — context gathering before planning.
Resume file: .planning/phases/03-tool-permission-abuse-cat-01-33/03-01-PLAN.md

---
*Last updated: 2026-08-30*
