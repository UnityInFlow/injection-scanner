# State: injection-scanner

## Project Reference
See: `.planning/PROJECT.md`
**Core value:** Catch prompt injection attacks before they reach production
**Milestone:** **v0.2.0 — Agent-shaped attacks** (opened 2026-08-30)

> Full history for the previous milestone — 412 lines of session notes — is archived at
> `.planning/archive/milestone-v0.1.0/STATE.md`, alongside its REQUIREMENTS, ROADMAP and phases.

## Current Phase
**Phase 2 — Recursive decoder (ENG-02, #30)** · status: **not started, ready to plan**

Phase 1 shipped 2026-08-30 (PR #104, #32 closed; PR #106 added the alias-bomb test).
`main` clean, in sync, **313 tests**, CI green, zero open PRs, still 0 merge commits.

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
| 2 | ENG-02 recursive decoder | #30 | Not started |
| 3 | CAT-01 tool & permission abuse `PI050-059` | #33 | Not started |
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
| Encoding/Obfuscation | 9/12 | 75% |
| **Total** | **56/60** | **93%** |

Phase 2 (#30) takes this to **58/60**.

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

---
*Last updated: 2026-08-30*
