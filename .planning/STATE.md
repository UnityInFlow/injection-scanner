---
gsd_state_version: 1.0
milestone: v0.2.0
milestone_name: Agent-shaped attacks
status: in_progress
stopped_at: Completed 04-07-PLAN.md
last_updated: "2026-09-07T18:36:33.000Z"
state_head: e4b46a1
progress:
  total_phases: 5
  completed_phases: 3
  total_plans: 16
  completed_plans: 16
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

**Phase 4 — MCP & tool-description poisoning (CAT-02, #34)** · status: **all 7 plans complete (04-01 through 04-07)** — phase-complete marker and the PR are the orchestrator's to set after its own verification runs

Phase 3 shipped 2026-09-02 as **PR #109** (rebase-merged, issue #33 auto-closed): `PI050`-`PI057`,
the `relaxed_pattern` schema field, and ADR-004. Its code review found one critical false positive
— **CR-01**, three prose patterns firing on *prohibitions* ("Never run with
`--dangerously-skip-permissions`") at HIGH, the severity `install-hook` blocks commits at. Fixed in
quick task `260902-jhy` before the PR, by structural tightening rather than a negation guard: real
payloads carry a negator inside the matched sentence, so a guard would have suppressed `PI053`'s
and `PI057`'s own `example` values and failed `pattern_example_test`.

Phases 1 and 2 shipped 2026-08-30; both engines are done. The two remaining phases are pattern
categories, one PR each (GATE-04). `main` clean, **357 tests**, CI green, still 0 merge commits.

**One open PR: #110** — an external contribution (+2,072 lines, 32 files, CI green) that
touches three categories at once: PI058 (CAT-01), PI070 (opens CAT-03), PI110-PI113
(multilingual, #39, a v0.3.0 issue), plus `src/context.rs` and a new integration. It
conflicts with GATE-04 (one category per PR), GATE-01 (its corpus is derived from its own
patterns), GATE-02 (moves the recall pin to 93/97, changing the denominator) and GATE-03
(evidence is 6 web pages, not the ~1,300-file sweep). It also repairs this repo's own dead
`.claude` hooks, which used non-existent event names so every guard was a silent no-op.
Triage before Phase 4 closes — it edits the same files Phase 4 does.

**Carried into Phase 4:** `PI050+` patterns must ship a `relaxed_pattern` (GATE-05, ADR-004). The
generalizable CR-01 rule — *fix negation where the negator sits*: clause-initial anchoring when it
precedes the span, an enumerated filler set when it sits inside — applies directly to CAT-02's
prose arms. Open follow-ups: **WR-02** (structural corpus README documents 1 of 5 payloads),
**WR-03** (`scripts/gate03-sweep.sh` helpers declare no `local`), and two pre-existing
`docs/PATTERN-CATALOGUE.md` self-matches (`PI001` at :74, `PI031` at :903) that predate PR #109.

**04-04 shipped 2026-09-06 (3 commits, branch `feat/34-mcp-tool-poisoning-pi060`).** D-03's three
config-hygiene signals are live: `PI060` unvetted-mcp-server-source, `PI061` plaintext-mcp-endpoint,
`PI062` remote-script-mcp-launch — all `scope: frontmatter`, all leaf-anchored across every real
wrapper convention, all MEDIUM by **category default** so no shipped pattern in
`patterns/core/mcp-tool-poisoning.yaml` carries a `severity` field at all. That is D-03's
below-the-commit-blocking-line requirement held by construction rather than one field at a time.
64 patterns, **373 tests**, recall **100/109 (91.7%)** with the CAT-02 structural row at 5/8.

Three things from it worth not rediscovering.

**(1) D-03's wording did not survive measurement, and the plan required measuring rather than
reading.** "Unpinned `npx -y <pkg>`" is the *ecosystem default*, not an outlier: a scratch probe
outside the repo over the 46 real manifests in the sweep list fired on **8** of them — 8 of the 24
that declare a launch command at all — against **1** using an off-registry source. This repo's own
false-positive gate already contains the shape (plan 04-03's `mcp-dev-tooling-setup.json` is built
from `npx -y @example/docs-search-mcp`), so a pin-based pattern fails `corpus_test` on day one.
`PI060` therefore discriminates on the install **source**. The accepted cost —
attack payload `07-unpinned-npx-install-mcpservers.md` stays undetected on purpose — is named in
the YAML header, the README callout, the `recall_test` comment and the SUMMARY.

**(2) The structural pass matches ONE projected line at a time.** `src/scanner.rs`'s fourth pass
renders each projected leaf separately, so **no structural regex can require two leaves to
co-occur**. `command = npx` and `args[0] = -y` are different lines; "unpinned `npx -y`" is not
expressible as one pattern regardless of the false-positive argument. Also: the `regex` crate has
**no lookahead**, so `PI061` excludes loopback structurally, by requiring a registrable-domain host.

**(3) `main` moved under the phase, and the 04-01 GATE-03 baseline is now two pattern-set
generations old.** Comparing straight to `sweep-baseline-2026-09-03` reports **66 removals that
belong to PR #110** (`PI017` retired into `MatchContext::HiddenHtml`, `PI026` made badge-safe), not
to the plan under test. 04-04 built a third sweep from the merge-base `0d50e92` to isolate its own
delta: **+1 finding, 0 removals** over 23,764 real files, the addition being a true positive on a
real `uvx --from git+https://…` manifest. **Plans 04-05/04-06/04-07 must compare against
`sweep-mainbase-04-04-2026-09-06/`**, not the 04-01 baseline.

**New open follow-up from 04-04:** `docs/DETECTION-BACKLOG.md` now self-matches **ten** times
(`PI011`, `PI014`, `PI019`, `PI027`, `PI028`, `PI029`, `PI039`, `PI045`, `PI054`, `PI055`). Verified
by stashing all of 04-04's work and re-running: it is identical with and without them, so it arrived
with the PR #110 merge and is **not** 04-04's. It is the exact "the scanner flags its own
documentation" failure the pattern-library skill warns about and the 2026-08 audit listed. Needs its
own issue or quick task; fixing it inside a pattern PR would confound that PR's GATE-03 delta.

**04-05 shipped 2026-09-07 (3 commits, same branch `feat/34-mcp-tool-poisoning-pi063`).** The
three HIGH prose arms this category is named for are live: `PI063` tool-description-directive
(D-01's discriminator — second-person address AND an external-object directive, from the first
committed draft), `PI064` tool-description-file-smuggle (the MCP-specific smuggling-channel
signal), `PI065` tool-description-emphasis-block (a tag-delimited/bracketed emphasis wrapper
enclosing a directive). 67 patterns, **402 tests**, recall **101/109 (92.7%)** with the CAT-02
combined row at 8/12.

Two things worth not rediscovering.

**(1) A real GATE-03 false positive, caused by case-folding, not by the discriminator being
wrong.** `PI063`'s credential-suffix branch (`\b[A-Z][A-Z0-9_]*_TOKEN\b`) matched `get_token` — an
ordinary Python function call — in a real vendored Hugging Face skill file, because every pattern
in this file compiles case-insensitively by default and a case-insensitive `[A-Z]` class folds and
matches lowercase too. Fixed with an inline `(?-i:...)` case-sensitive group around just the two
ALL-CAPS branches — `PI011` already uses this exact technique in the same file. **Any future
pattern using `[A-Z]` to mean "genuinely uppercase" must wrap it explicitly**, or the engine's
default case-insensitivity silently defeats the intent; unit tests alone did not catch this, only
the real-file sweep did.

**(2) `main` moved again under the phase.** `sweep-mainbase-04-04-2026-09-06` is now stale too:
#122 (two launcher widenings), #125, #126 and #127 landed after 04-04 shipped. 04-05 built a
*fresh* pre-edit binary from `66bf53c` (this branch's own fork point) in a separate `git worktree`
(no `git stash`) rather than reusing either prior baseline. **Plans 04-06/04-07 must compare
against `sweep-mainbase-04-05-2026-09-07/`** or build their own fresh pre-edit capture — not
against `sweep-baseline-2026-09-03` or `sweep-mainbase-04-04-2026-09-06`.

**04-06 shipped 2026-09-07 (4 commits, same branch `feat/34-mcp-tool-poisoning-pi063`).** CAT-02's
full ten-pattern set is now complete: `PI066` cross-tool-shadowing, `PI067` tool-override-directive,
`PI068` version-conditional-directive, `PI069` deferred-activation-directive — all MEDIUM by
category-default inheritance. 71 patterns, **cargo test --locked green (37 binaries)**, recall
**102/109 (93.6%)** with the CAT-02 prose sub-row at 4/4 (100%) and combined row at 9/12 (75%).

Two things worth not rediscovering.

**(1) A real GATE-03 false-positive class, caused by trigger-words-only discriminators, not by a
one-off bug.** `PI066`'s Arm A and all three of `PI067`'s arms originally keyed only on trigger
vocabulary (when+calls/invokes/uses/runs; never/always/instead-of) with no requirement on WHAT was
being used, called or invoked. The sweep over ~23,900 real files found **26 real additions** — 1
`PI066`, 25 `PI067` — every one an ordinary "never do X, always do Y" style-guide sentence with zero
tool-substitution content (`"Never use \`any\` type, use \`unknown\` or generics instead"` from a
TypeScript best-practices doc; `"ALWAYS use a navigation stack title instead of a custom text
element"` from an Expo skill). Fixed by requiring a tool-shaped object (backtick-quoted code span,
snake_case identifier, or identifier immediately followed by `()`) directly after the relevant verb
on BOTH sides of the substitution/shadowing grammar. **Any future heuristic keying on a common
English grammatical contrast (never/always, if/when) needs an explicit shape requirement on its
object, not just the trigger words** — vocabulary alone is not a tool-specific signal.

**(2) The reused pre-edit baseline pattern held cleanly across a third generation.** Following the
orchestrator's instruction, `sweep-after-04-05-2026-09-07` (the committed tree at `85cacde`) was
reused as-is as this plan's pre-edit baseline rather than capturing a fresh one — both directions
against it came back empty after the re-narrowing fix. **Plan 04-07 should reuse
`sweep-after-04-06-2026-09-07/`** the same way, or build its own fresh pre-edit capture — not
`sweep-baseline-2026-09-03` or either of the two prior `sweep-mainbase-*` directories, all now
stale.

**04-07 shipped 2026-09-07 (same branch `feat/34-mcp-tool-poisoning-pi063`).** CAT-02 close-out:
the whole-branch GATE-03 delta against `sweep-mainbase-04-05-2026-09-07` is clean in both
directions (0 additions, 0 removals), independently corroborated with a freshly rebuilt
`66bf53c` binary in the same session. Every published number (71 patterns, 102/109 recall,
12-payload CAT-02 corpus, 406 tests) was measured against the finished tree and already agreed
with the README/`PATTERNS.md`/`tests/recall_test.rs` — plans 04-04/04-05/04-06 had each already
reconciled their own numbers in the commit that changed them, so this close-out needed zero
corrections, only verification. Four measured limitations were filed as issues: the JSONC parse
gap (#129), the decoded-layer pass's inability to reach a structural pattern (#130), D-05's
structural cross-reference (#131), and the `docs/DETECTION-BACKLOG.md` self-match arriving with
the PR #110 merge (#132, not attributable to any Phase 4 plan). WR-02 (the `tool-permission-abuse/`
corpus README gap) is filed as #133 and its `.planning/WINDOWS.md` ledger entry waived; the
`src/frontmatter.rs:219` char-boundary panic window is marked fixed (already resolved by quick
task 260903-fast / `d28dfd0`).

Four things worth not rediscovering.

**(1) `scripts/gate03-sweep.sh --compare` is meaningless against this repository's own
committed sweep directories — always point it at `.planning/local/`.** This repo deliberately
does not commit the raw per-directory JSON reports (`dad56d1`, `e54be72`); only `manifest.tsv`,
`summary.tsv` and `checksums.sha256` are public. `--compare` loads findings by globbing `*.json`
in each argument directory, so comparing against a repository-committed (JSON-less) sweep
directory silently loads an **empty baseline**, and every real finding in the candidate reads
as a false "new" one — this produced a spurious 500-line "diff" on the first attempt in this
plan. The real, gitignored JSON is still on disk at `.planning/local/<sweep-dir>/*.json`;
re-pointing `--compare` there produced the correct, adjudicated result (0/0 whole-branch delta).
**Candidate for `.continue-here.md`'s anti-pattern table** — this is a variant of "the fixture
was green while the real thing was untested," except here the fixture (an empty baseline) was
silently substituted for the real one by a missing file, not an intentional shortcut.

**(2) The `relaxed_pattern` ratchet's id range did not cover this phase's ids despite an
adjacent comment claiming it did — already found and fixed in 04-01, re-confirmed here.**
`tests/pattern_policy_test.rs`'s `requires_relaxed_pattern` is `id >= 50` (open-ended), not the
originally-shipped closed `50..=59` range a comment nearby once claimed was sufficient — a
closed range would have silently exempted every `PI060`+ pattern in this phase from GATE-05's
mutation-tested false-positive control. Carried forward for Phase 5 (`PI070`+): the open-ended
predicate already covers it; no further repair needed, but the failure mode (inheriting a
documented range without checking it against the next category's ids) is exactly
`.continue-here.md`'s existing "inheriting a documented reason without measuring it" anti-pattern.

**(3) The structural corpus collector and both clean-corpus enumerations are non-recursive.**
`tests/recall_test.rs`'s `categories()` (`p.is_file()` filter) and `tests/corpus_test.rs`'s
clean-corpus enumeration both use `fs::read_dir` one level deep — a subdirectory is invisible to
either. This is why `structural_categories()` exists as a dedicated second collector walking one
level further into `tests/corpus/attack/structural/`. Phase 5 (CAT-03): if any future corpus
needs a third level of nesting, neither collector reaches it without a third dedicated walker.

**(4) The decoded-layer pass runs only prose-scoped patterns — a `scope: frontmatter` pattern
never sees a decoded value, even inside a document the structural pass otherwise projects.**
Measured, not assumed: `PI063`/`PI064` (prose-scoped) reach the base64-encoded description
payload via the ordinary decoded-layer pass over the raw JSON text; no CAT-02 *structural*
pattern needs to decode a projected value today, so this has not yet produced a measured miss.
Filed as #130. Phase 5: any future structural pattern whose target value could plausibly be
attacker-encoded inherits this same gap.

**What Phase 5 inherits, explicitly:** the `relaxed_pattern` obligation (`id >= 50`, already
covers `PI070`+), the CR-01 negation rule (fix negation where the negator sits — clause-initial
anchoring before the span, an enumerated filler set inside it), the per-category structural
corpus layout (`tests/corpus/attack/structural/<category>/`, one level of nesting, non-recursive
collectors), and the open follow-ups in `deferred-items.md` (#129-#133, D-01's accepted
third-person blind spot, the rug-pull bound).

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
| 4 | CAT-02 MCP & tool-description poisoning `PI060-069` | #34 | 7/7 plans complete (04-07 closed out the category) — phase-complete marker pending the orchestrator's own verification and PR |
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

Re-synced with `tests/recall_test.rs` and the README on 2026-09-06 — this table had drifted two
milestones behind (it still showed the 84-payload denominator and had no rows for the persistence
or multilingual ranges). The authority is `EXPECTED` in `tests/recall_test.rs`; if the two disagree,
that array wins.

| Category | Detected | Recall |
|---|---|---|
| Data Exfiltration | 13/13 | 100% |
| Instruction Injection | 15/15 | 100% |
| Jailbreaks | 12/12 | 100% |
| Tool & Permission Abuse | 17/17 | 100% |
| Persistence & Lifecycle Hijack | 6/6 | 100% |
| Role Override | 11/12 | 92% |
| Encoding/Obfuscation | 11/12 | 91.7% |
| Multilingual (Czech; German misses) | 8/10 | 80% |
| MCP & Tool-Description Poisoning | 9/12 | 75% (all ten CAT-02 patterns shipped; the remaining structural rug-pull misses need D-05's deferred structural cross-reference work, issue #131) |
| **Total** | **102/109** | **93.6%** |

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

- **Filed at Phase 4 close-out (04-07, #34):** #129 (JSONC parse gap, silent), #130 (decoded-layer
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

## Session Continuity

Last session: 2026-09-07T18:36:33.000Z
Stopped at: Completed 04-07-PLAN.md
Resume file: None

---
*Last updated: 2026-09-07*

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
