---
gsd_state_version: 1.0
milestone: v0.2.0
milestone_name: Agent-shaped attacks
status: in_progress
stopped_at: "Phase 5 plan 05-07 (close-out) complete: 9 patterns, development recall 103/115, held-out recall 2/12; BLOCKING finding 183 (PI070 will-sentences) awaits a maintainer decision before the PR"
last_updated: "2026-10-09T15:00:00.000Z"
state_head: 1d8642e
progress:
  total_phases: 5
  completed_phases: 4
  total_plans: 23
  completed_plans: 23
  percent: 100
---

# State: injection-scanner

## Project Reference

See: `.planning/PROJECT.md`
**Core value:** Catch prompt injection attacks before they reach production
**Milestone:** **v0.2.0 — Agent-shaped attacks** (opened 2026-08-30)

> Full history for the previous milestone — 412 lines of session notes — is archived at
> `.planning/archive/milestone-v0.1.0/STATE.md`, alongside its REQUIREMENTS, ROADMAP and phases.

## Current Phase

**Phase 5 — Persistence & lifecycle hijack (CAT-03, #35)** · **7/7 plans executed** (closed out 2026-10-09 by plan 05-07).
The phase-complete marker and the pull request are the orchestrator's; the last phase of this milestone. Phases 1-4 are
complete (Phase 4: PR #120 + PR #136).

**What shipped:** 9 patterns in `PI070`-`PI079` — `PI070` (already on `main` from PR #110, widened here) plus eight new
(`PI071`-`PI077` and `PI079`). **`PI078` is deliberately unallocated** (D-07: a remote-lifecycle-hook-endpoint rule fires on the
attack payload and on the clean corporate-audit specimen alike). The range had **9** free slots, not 10, because `PI070` had already
shipped. Six are HIGH (`PI070`, `PI071`, `PI072`, `PI075`, `PI076`, `PI077`) and three MEDIUM (`PI073`, `PI074`, `PI079`);
**nothing is CRITICAL** (D-06, recorded on #35). Library: **79 patterns**.

**The headline number is the held-out one, and it is much lower.** The category scores **7/12 (58.3%)** on its development corpus
(prose 4/7, structural 3/5) but **2/12 (16.7%)** on a sealed, blind-authored held-out set (prose 2/8, structural 0/4) that was hashed
before any `PI071`+ pattern existed and opened once, after the patterns were frozen. Neither held-out detection is a pattern this phase
wrote (`PI025`, `PI070`). Per `heldout-set.md` rule 4 the ten misses were reported, not tuned away. Library-wide development recall is
**103/115 (89.6%)**; the held-out rows are totalled separately and never summed into it.

**Measured at close-out (HEAD `1d8642e`, 2026-10-09):** full suite `cargo test --locked` **467 passed, 0 failed (38 test-result lines, exit 0, redirected to a file and the exit code read; never piped)**; `cargo fmt --check` and
`cargo clippy --all-targets -D warnings` clean; whole-category GATE-03 delta **0 additions, 0 removals** over 26,407 files in both
directions (`05-SWEEP.md`); GATE-04 proven by a quoted diff (one pattern file); `Cargo.toml`, `Cargo.lock` and `src/` untouched, so no ADR.
Self-scan outside `examples/ patterns/ tests/ tools/` is exactly the two standing `PATTERN-CATALOGUE.md` findings (`PI001`, `PI031`) and
nothing new.

**One BLOCKING finding is open and needs a maintainer decision before the PR: #183.** The pre-PR code-review gate found that `PI070`'s
widened objects and verbs make third-person `will` vendor-feature sentences fire HIGH (13 of 17 probes are new relative to the pre-phase
pattern; the sweep is silent because no such sentence is in a real file on this machine). The 9b remedy does not discriminate; the options
are a deictic requirement, dropping `will` from the HIGH modal set, a MEDIUM grade, or reverting the widening. See `05-REVIEW.md` BL-01.

**Filed at close-out:** #164-#183 (all milestone v0.3.0 except #183, which is v0.2.0), full accounting in
`.planning/phases/05-persistence-lifecycle-hijack-cat-03-35/deferred-items.md`. The #35 close-out comment (the severity deviation and the
slot arithmetic) is https://github.com/UnityInFlow/injection-scanner/issues/35#issuecomment-6080268901, with an addendum for #183.

> **This section has drifted before — verify it before planning from it.** It once named a merged PR as the open blocker for closing
> Phase 4, and carried a test count and two pinned line numbers that had all moved. Settle such claims with `gh pr view <N> --json state`
> (is a PR actually open), `git log --merges --oneline origin/main` (the merge-commit count) and a full `cargo test` (the test count). The
> live answer beats this file whenever the two disagree. State at close-out: Phase 5 is on branch `feat/35-persistence-lifecycle-hijack`,
> **no PR is open yet**, and `main` is where Phase 4 left it. Last verified: 2026-10-09.

### Four things from Phase 5 worth not rediscovering

**(1) A development corpus overstates a detector; ship a sealed set before the patterns exist.** The category looked like 7/12 and is
2/12 on payloads nobody tuned against. The development payloads cleared the derivation check *lexically*, but their shapes had been restated
to the pattern authors, so the check could not see structural dependence. The held-out set (authored blind, hashed first, opened once)
is the only number here that is independent. `tools/corpus-derivation-check.py` proves "no wording was lifted", never "GATE-01 satisfied" (#166).

**(2) "A modal keeps vendor prose out" is false for `will`, and a clean sweep is absence of evidence.** Every HIGH prose arm that accepts a
product name as a declarative subject fired on vendor documentation: `PI071` twice, `PI076` once, and now `PI070` (#183). The 26,407-file
sweep was empty every time because no such sentence existed in a real file on this machine. The only thing that found each one was an
adversarial probe with synthesized vendor-voice sentences. Run one (the one in `05-REVIEW.md`) before trusting a green sweep on a HIGH
prose arm. The discriminator for `PI071`/`PI076` was the second-person possessive, not the deictic word; `PI070`'s bare-object-path case
needs a different one.

**(3) The both-directions specimen test beat the sweep.** `PI078` was dropped because swapping the bound event, the host's registrable
domain and the URL between the attack payload and the audit specimen moved nothing. The dotted-host draft produced 0 hits on the 263-file
hooks sweep, so the sweep half of the old criterion alone would not have disqualified it. A pattern that cannot separate the attack from a
compliance control is not shipped, however clean the sweep.

**(4) The structural pass matches one projected line at a time, and `--compare` is path-keyed.** No structural regex can require two leaves
to co-occur; `locate()` collapses repeated keys onto the first line (#165); a download-then-run hook is outside `PI077`'s pipe-to-interpreter
arm and all four held-out hook files use it (#176). For the sweep: capture the baseline from the checkout later plans run in, sweep repo-local
inputs by the main checkout's literal absolute paths, point `--compare` only at `.planning/local/`, and plant a deletion to prove the
comparison is not vacuous (#182).

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
| 4 | CAT-02 MCP & tool-description poisoning `PI060-069` | #34 | **Done** — PR #120 + PR #136 |
| 5 | CAT-03 persistence & lifecycle hijack `PI070-079` | #35 | 7/7 plans executed — 9 patterns, held-out recall 2/12; blocked on a decision for #183, then phase-complete marker and PR (orchestrator) |

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
| GATE-04 | No reviewable unit widens more than one category — widening four at once is an unreviewable FP blast radius |
| GATE-05 | The false-positive control is mutation-tested — two of four v0.1.0 widenings had a control the corpus was not holding |

Also standing: `main` stays strictly linear. A pattern's `name` is a **consumer contract** —
`pattern_name` ships in the JSON `spec-ci-plugin` reads, so widen the `description`, never rename.

## Detection recall — the published number

Re-synced with `tests/recall_test.rs` and the README on 2026-10-09 (plan 05-07, HEAD `1d8642e`). The authority is `EXPECTED` in
`tests/recall_test.rs`; if the two disagree, that array wins. The held-out rows are pinned in `EXPECTED` too but are **totalled
separately** (the test report prints both totals) and are never part of the 115.

| Category | Detected | Recall |
|---|---|---|
| Data Exfiltration | 13/13 | 100% |
| Instruction Injection | 15/15 | 100% |
| Jailbreaks | 12/12 | 100% |
| Tool & Permission Abuse | 17/17 | 100% |
| Role Override | 11/12 | 92% |
| Encoding/Obfuscation | 11/12 | 91.7% |
| Multilingual (Czech; German misses) | 8/10 | 80% |
| MCP & Tool-Description Poisoning | 9/12 | 75% (all ten CAT-02 patterns shipped; the remaining structural rug-pull misses need D-05's deferred structural cross-reference work, issue #131) |
| Persistence & Lifecycle Hijack (development corpus) | 7/12 | 58.3% (prose 4/7, structural 3/5; 3 deliberate misses, 2 recorded gaps) |
| **Total (development corpus)** | **103/115** | **89.6%** |
| **Persistence & Lifecycle Hijack, held-out set** (the published v0.2.0 CAT-03 number) | **2/12** | **16.7%** (prose 2/8, structural 0/4) |

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

- **Filed at Phase 4 close-out (04-07, #34):** #129 (JSONC parse gap, silent — **resolved**: fixed
  by quick task `260915-spt` / `4cddc99`, issue closed), #130 (decoded-layer
  pass skips structural patterns), #131 (D-05 structural cross-reference for tool shadowing),
  #132 (`docs/DETECTION-BACKLOG.md` self-match, PR #110-origin), #133 (WR-02, structural corpus
  README backfill). Full accounting in
  `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/deferred-items.md`.

- **Already tracked, not duplicated:** #115 (Phase 3 planning artifacts — restored to `main` at
  this close-out from commit `752ac98`/`db2a575` rather than left referenced-only; re-verify #115
  is still needed for anything beyond that), #116 (stale planning counters — this close-out
  reconciled ROADMAP/STATE's library and recall projections but did not audit every counter in
  every file), #117 (GATE-04 contradiction between STATE.md and REQUIREMENTS.md — this phase's
  own GATE-04 diff check passed cleanly, but the requirement-text contradiction #117 names is
  unresolved by that; still open).

## Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260902-jhy | Fix CR-01 negation blindness in PI053/PI056/PI057; fold in WR-01 `PATTERNS.md` category row | 2026-09-02 | `db2a575` | [260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi](./quick/260902-jhy-fix-cr-01-negation-blindness-in-pi053-pi/) |
| 260903-fast | Fix char-boundary panic in frontmatter projection (detection bypass via oversized multi-byte scalar) | 2026-09-03 | `d28dfd0` | — |
| 260915-spt | Fix #129 — JSONC-commented config silently skipped the structural pass; adds the stderr diagnostic *and* offset-preserving JSONC tolerance | 2026-09-15 | `4cddc99` | [260915-spt-fix-129-jsonc-commented-config-silently-](./quick/260915-spt-fix-129-jsonc-commented-config-silently-/) |
| 260915-u3j | Fix #128 — the manufactured-boundary gate: a match whose edge falls inside a separator-joined compound token (`sh-lint`, `on-call`) is withheld as an artefact, library-wide across all passes | 2026-09-16 | `c47194e` | [260915-u3j-fix-128-separator-normalizer-folds-sh-li](./quick/260915-u3j-fix-128-separator-normalizer-folds-sh-li/) |
| 260915-u3j | Fix #128 — a pattern-agnostic manufactured-boundary gate stops `sh-lint`/`on-call`/`DAN-mode-switch` from firing PI028/PI030/PI031 as artefacts, across all five scanner passes; adds ADR-006 | 2026-09-16 | `66351c8`..`b31b110` (+ Task 4) | [260915-u3j-fix-128-separator-normalizer-folds-sh-li](./quick/260915-u3j-fix-128-separator-normalizer-folds-sh-li/) |
| 260916-sz4 | Amend GATE-04 to a review-unit rule — "No reviewable unit widens more than one category"; the old "its own PR" wording was never once satisfiable as written (#117) | 2026-09-16 | `7c8e99e` | [260916-sz4-amend-gate-04-to-no-reviewable-unit-wide](./quick/260916-sz4-amend-gate-04-to-no-reviewable-unit-wide/) |
| 260916-sz4b | Fold the GATE-04 amendment into `.continue-here.md` — a fourth, tracked copy of the standing-gates table still read "One category per PR" (#117) | 2026-09-17 | `20ae586` | — |
| 260922-st9 | Put the `LEGACY_UNTESTED` ratchet note in past tense — #138 emptied the list, so the present-tense "they are listed in" doc read as a disabled check rather than the ratchet at full strictness (#89) | 2026-09-22 | `b85a536` | [260922-st9-update-stale-legacy-untested-module-doc-](./quick/260922-st9-update-stale-legacy-untested-module-doc-/) |
| 260923-dbv | Reconcile the stale `CLAUDE.md` Status block — it named a release two versions behind, presented the archived Production Readiness milestone as current, and asserted "CI has been dead since 2026-06-24. Nothing merges" while five PRs were merging through green CI | 2026-09-23 | `4ffbee7` | [260923-dbv-reconcile-the-stale-claude-md-status-blo](./quick/260923-dbv-reconcile-the-stale-claude-md-status-blo/) |
| 261006-tq8 | Reconcile five stale claims in this file — merged PR #110 still named as the open blocker for closing Phase 4, `0 merge commits`, `357 tests`, two drifted catalogue line numbers, and #129 shown as open; adds the anti-drift blockquote | 2026-10-06 | `465526a` | [261006-tq8-reconcile-stale-claims-in-state-md-110-i](./quick/261006-tq8-reconcile-stale-claims-in-state-md-110-i/) |

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

- 2026-09-07 (Phase 4 close-out, 04-07): **CAT-02 closed — see the "Current Phase" section above
  for the full detail (kept there, alongside 04-04/04-05/04-06's own entries, rather than
  duplicated here).** Four measurements Phase 5 inherits: (1) the `relaxed_pattern` ratchet's
  `id >= 50` predicate already covers `PI070`+, already fixed in 04-01; (2) the structural corpus
  collector and both clean-corpus enumerations are non-recursive (`fs::read_dir` one level deep);
  (3) the decoded-layer pass runs only prose-scoped patterns, so a `scope: frontmatter` pattern
  never sees a decoded value (#130); (4) `scripts/gate03-sweep.sh --compare` silently loads an
  empty baseline against this repository's own committed (JSON-less) sweep directories — always
  point it at `.planning/local/`. Four issues filed (#129-#132) plus one for a carried-over Phase
  3 gap (#133, WR-02, waived in `.planning/WINDOWS.md`). The Phase 3 planning directory, lost to
  the rebase-merge that closed #33, was restored to `main` from `752ac98`/`db2a575`.

- 2026-09-16 (quick task 260915-u3j): **#128 closed — the research's locked mechanism did
  not survive measurement, worth not rediscovering.** The plan was originally built on
  "gate the normalized pass": measured against `27e4d49`, that fixes at most 2 of the 6
  false positives. `-` is a non-word character, so `sh\b`/`on\b` are satisfied by a hyphen
  directly, with **no fold involved** — `PI028` (all four `sh-lint` variants) and `PI030`
  (`on-call`) are **raw-pass** findings, and the raw-pass-runs-first dedup means a
  normalized-pass-only gate never gets a say on them. Only `PI031` (`DAN-mode-switch`) is
  genuinely normalized-pass. The shipped gate is therefore **pass-independent**:
  `normalize::span_edge_is_manufactured(text, start, end)`, called at all five scanner
  passes against each pass's own haystack (the normalized pass via a new `original_span`
  helper so the gated span and the quoted text can never disagree). Withheld artefacts are
  filed to a new `manufactured_boundary` array on `ScanReport` — recorded, not discarded,
  same principle as `suppressed`/`low_confidence`/`baselined` — but deliberately carries
  **no promotion flag**, because a flag that restored a known artefact would re-enable the
  bug. Gate set is `-`/`_` only (a strict subset of the fold set, pinned by a test); `.`/`/`
  excluded because they join paths/domains where a match edge before the separator is
  routinely legitimate. Eleven patterns audited as exposed to the same short-token-`\b`
  shape (acceptance criterion 4); full writeup in
  `docs/adr/ADR-006-manufactured-boundary-gate.md`. GATE-02 pin unmoved
  (`tests/recall_test.rs::EXPECTED` byte-identical to `27e4d49`); GATE-03 swept 23,037
  real files before and after — manifest/summary byte-identical, `--compare` empty in both
  directions (see `260915-u3j-SWEEP.md`). Issue #128's acceptance criterion 3 was corrected
  in the issue itself: `ig-nore pre-vious in-structions` was never detected on `main`,
  before or after this fix — intra-word hyphenation is a separate, still-open evasion.

## Session Continuity

Last session: 2026-10-09
Stopped at: Completed 05-07-PLAN.md (Phase 5 close-out) — #183 awaits a maintainer decision before the PR
Resume file: None

---
*Last updated: 2026-10-09*

## Performance Metrics

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
| Phase 04 P01 | ~15min (commit span) | 3 tasks | 44 files |
| Phase 04 P05 | 69min | 3 tasks | 16 files |
| Phase 04 P06 | 6h 40min (includes ~50min of judged predecessor draft) | 3 tasks | 9 files |
| Phase 04 P07 | ~27min (commit span) | 3 tasks | 15 files |

## Decisions

- [Phase 04]: GATE-03 sweep excludes ~/.cursor/extensions and ~/.vscode/extensions (vendored bundle noise, direct cause of a reproduced src/frontmatter.rs:219 panic); their config-relevant subdirectories are swept instead
- [Phase 04]: relaxed_pattern's PI050+ GATE-05 requirement is now an open-ended predicate (id >= 50), fixing a closed 50-59 range that would have exempted every CAT-02/PI060+ pattern
- [Phase ?]: PI063's external-object set is closed and enumerated (filesystem path, environment variable, concealment framing), chained onto the subject with no gap the verb could be reached across -- keeps all four measured Q3 near-misses silent without special-casing any of them
- [Phase ?]: GATE-03 comparisons target a fresh binary built from the branch's own fork point in a separate git worktree (never git stash) once main has moved multiple generations past the last committed baseline
- [Phase 04]: PI066/PI067's heuristic discriminators require a tool-shaped object (backtick code span, snake_case identifier, or identifier+`()`) on both sides of the trigger grammar, not trigger words alone -- vocabulary-only heuristics on common English contrasts (never/always, when/if) produce real false positives at scale (26 hits across ~23,900 files before the fix)
- [Phase 04]: PI068/PI069's rug-pull bound is stated in the pattern file's own header comment, not only in plan/state records -- a catalogue or README claiming these arms mitigate the rug-pull class would be a false security claim in the repo's own voice
- [Phase 04]: `scripts/gate03-sweep.sh --compare` must be pointed at `.planning/local/<sweep-dir>/` (the gitignored raw JSON), never at this repository's own committed sweep directories -- the committed copies have zero `*.json` files by design (`dad56d1`/`e54be72`), so `--compare` silently loads an empty baseline and reports every real finding as a false addition
- [Phase 04]: the Phase 3 planning directory (`.planning/phases/03-tool-permission-abuse-cat-01-33/`) and the CR-01 quick-task directory (`.planning/quick/260902-jhy-.../`), both lost to the rebase-merge that closed #33, are restored to `main` at 04-07's close-out from commits `752ac98`/`db2a575` on the still-live `feat/cat-01-tool-permission-abuse` branch -- future phase-closing rebase-merges must not repeat this loss
