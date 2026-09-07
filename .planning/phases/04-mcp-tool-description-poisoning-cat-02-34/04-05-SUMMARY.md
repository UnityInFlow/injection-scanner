---
phase: 04-mcp-tool-description-poisoning-cat-02-34
plan: 05
subsystem: patterns
tags: [cat-02, mcp, tool-description-poisoning, gate02, gate03, gate05, negation-handling, case-fold-bug]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 03
    provides: the D-01 boundary corpus (mcp-server-catalogue.json) exercising all four real-world near-miss shapes, and mcp-manifest.json's pre-existing config.systemPrompt near-miss
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 04
    provides: the mcp_tool_poisoning category scaffolding, PI060-PI062, the category-is-loaded guard, and the (now superseded) sweep-mainbase-04-04-2026-09-06 reference point
provides:
  - PI063 tool-description-directive (HIGH) -- the D-01 discriminator this category is named for, requiring second-person address AND an external-object directive (filesystem path, environment variable, or concealment instruction) from its first committed draft
  - PI064 tool-description-file-smuggle (HIGH) -- the MCP-specific smuggling-channel signal, distinct from the file-read half that overlaps not-yet-built credential-harvesting territory
  - PI065 tool-description-emphasis-block (HIGH) -- a tag-delimited or bracketed emphasis wrapper enclosing a directive, deliberately not re-detecting PI015's concealment vocabulary
  - a fresh pre-edit GATE-03 baseline (sweep-mainbase-04-05-2026-09-07, built from 66bf53c) that supersedes the now-stale sweep-baseline-2026-09-03 and sweep-mainbase-04-04-2026-09-06 for this plan and for 04-06/04-07
  - a real false positive found and fixed by the sweep: a case-insensitive `[A-Z]` credential-suffix branch folded to match `get_token` (an ordinary Python identifier) in a real vendored file
  - re-pinned recall (101/109, 92.7%), the regenerated catalogue and code-scanning baseline, and a README behaviour-change callout naming both the new HIGH findings and D-01's accepted third-person blind spot
affects: [04-06, 04-07]

actuals:
  tokens: 20617   # chars/4 over `git diff d9d491d..8e0f077` (this plan's three task commits;
                   # excludes the pre-existing empty branch-opening chore commit)
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "A closed, enumerated external-object set (filesystem path / environment variable / concealment framing) chained directly onto the second-person subject with no open gap in between, so a Context7-style 'you must call this tool before X to obtain a valid ID' near-miss cannot bridge from the verb to an unrelated object several words away"
    - "D-02 negation handling generalized beyond the verb-initial shape Phase 3 introduced: PI063 chains an ENUMERATED modal set (excluding never/not) directly onto an ENUMERATED filler set directly onto the verb, so a negator has no consumable token to hide behind; PI064 anchors its verb at a clause boundary OR after a coordinating and/then (the attack's own coordination grammar), excluding a clause-initial prohibition; PI065 requires its modal to be followed directly by filler-then-verb, the same technique as PI063"
    - "Every pattern in this file compiles case-insensitively by default, so a bare [A-Z] character class silently case-folds and matches lowercase too -- the ALL-CAPS credential-suffix branches must be wrapped in an inline (?-i:...) case-sensitive group (the same technique PI011 already uses), or the branch degrades into matching any identifier ending in _key/_token/etc. regardless of case"
    - "When main has moved multiple generations past the last committed GATE-03 baseline, build a FRESH pre-edit binary from the branch's own fork point in a separate git worktree (never git stash) rather than reusing a stale baseline -- the fresh run is simultaneously the pre-edit reference and the correct isolating comparison for this plan's own delta"

key-files:
  created:
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-mainbase-04-05-2026-09-07/ (manifest.tsv, summary.tsv, checksums.sha256, RAW-REPORTS.md)
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-after-04-05-2026-09-07/ (manifest.tsv, summary.tsv, checksums.sha256, RAW-REPORTS.md)
  modified:
    - patterns/core/mcp-tool-poisoning.yaml
    - tests/pattern_test.rs
    - tests/recall_test.rs
    - examples/mcp-tool-poisoning-attack.md
    - README.md
    - docs/PATTERN-CATALOGUE.md
    - .github/code-scanning-baseline.json
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md

key-decisions:
  - "PI063's external-object set is closed and enumerated (filesystem path, environment variable, concealment framing) rather than open-ended, chained onto the subject with no gap the verb could be reached across -- this is what keeps all four measured Q3 near-misses (protocol-sequencing MUST-obligation, training-data aside, sibling-tool reference, plus the pre-existing clean-corpus system-prompt string) silent without any special-casing for any of them individually."
  - "PI064 does not detect the file-read half of the canonical Invariant Labs shape at all -- only the pairing of a content-noun with a named destination argument. The sensitive-path vocabulary that half would need is deliberately left to the not-yet-built PI090+ credential-harvesting range, per 04-RESEARCH.md's Don't Hand-Roll table, rather than starting a second divergent path list."
  - "PI065 deliberately does not re-detect concealment vocabulary, even though the canonical payload's <IMPORTANT> wrapper often encloses a concealment clause too -- PI015 already reaches that from the prose passes on its own broader terms, and duplicating it would be two findings on one line for no new information."
  - "The plan's own Task 1/Task 2 total-pattern-count targets (60, then 62) were written before three PRs (#125, #126, #127) plus #122's two widenings landed on main after 04-04 shipped, none of which added a pattern. The real starting count was 64, not 61, so the shipped counts are 65 (Task 1) and 67 (Task 2). The plan's INTENT -- the count moves by exactly the number of patterns added -- is honoured; the literal numbers are not."
  - "GATE-03's comparison targets a fresh binary built from 66bf53c (this branch's own fork point) in a separate git worktree, not the plan's named sweep-baseline-2026-09-03 or sweep-mainbase-04-04-2026-09-06 -- both are stale for this plan (three merges' worth of unrelated history sit between them and this branch), and comparing against either would misattribute that history to PI063-PI065, exactly the mistake 04-04 had to unpick for PR #110."
  - "The two atomic per-task commits (PI063 alone, then PI064+PI065) were reconstructed by building and fully re-testing an intermediate PI063-only tree state before committing it, rather than committing the combined final diff in two arbitrary halves -- matching plan 04-04's own precedent of one fully-gated commit per task."

requirements-completed: [CAT-02, GATE-02, GATE-03, GATE-05]

coverage:
  - id: D1
    description: "PI063 ships with both halves of D-01's discriminator present from its first committed draft: second-person address AND an external-object directive. `grep -c 'id: PI063' patterns/core/mcp-tool-poisoning.yaml` prints 1; `grep -v '^ *#' patterns/core/mcp-tool-poisoning.yaml | grep -c 'severity: HIGH'` prints 3 (all of PI063/064/065)."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_test test_pi063_tool_description_directive (5 positives: filesystem path, environment variable, credentials file, dotfile, concealment)"
        status: pass
    human_judgment: false
  - id: D2
    description: "The four measured 04-RESEARCH.md §Q3 near-misses (the clean specimen's system-prompt string, the protocol-contract obligation, the training-data aside, the sibling-tool reference) plus D-01's multi-step read-then-validate boundary and a D-02 negation-guard prohibition are all pinned as negatives, using the authored equivalents plan 04-03 already vendored in tests/corpus/clean/mcp-server-catalogue.json rather than re-quoting vendor text."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_test test_pi063_tool_description_directive (8 negatives, including all four near-misses, the multi-step boundary, the prohibition, and D-01's third-person blind spot)"
        status: pass
    human_judgment: false
  - id: D3
    description: "PI063's counter_example is the VERBATIM config.systemPrompt string from the committed clean MCP manifest specimen; the mutation test the plan's acceptance criteria requires was performed directly (temporarily replacing PI063's shipped pattern with its own relaxed_pattern), and cargo test --test corpus_test failed naming mcp-manifest.json among thirteen clean-corpus files, restored afterward and re-verified green."
    requirement: "GATE-05"
    verification:
      - kind: integration
        ref: "manual mutation of patterns/core/mcp-tool-poisoning.yaml (PI063's pattern -> its own relaxed_pattern), cargo test --test corpus_test (2 failed naming 13 files including mcp-manifest.json), restored (diff byte-identical to backup), cargo test --test corpus_test green (5/5) afterward; full transcript in the plan's Task 3 commit and 04-SWEEP.md"
        status: pass
    human_judgment: false
  - id: D4
    description: "D-02's negation rule (fixed where the negator sits, no engine-side guard) is applied to all three arms with a construction proven against a pinned prohibition test case for each: PI063 chains an enumerated modal set directly onto an enumerated filler set directly onto the verb (no open gap), PI064 anchors its verb at a clause boundary or after a coordinating and/then, PI065 requires its modal followed directly by filler-then-verb."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_test test_pi063_tool_description_directive (\"You must never read the contents of ~/.ssh/id_rsa...\" stays silent), test_pi064_tool_description_file_smuggle (\"Never pass its full contents...\" stays silent), test_pi065_tool_description_emphasis_block (\"<IMPORTANT>You should never call this tool...\" stays silent)"
        status: pass
    human_judgment: false
  - id: D5
    description: "PI064 targets the MCP-specific smuggling channel (a content-noun paired with a named destination argument), not the file-read half that overlaps not-yet-built credential-harvesting territory, and reuses no divergent second sensitive-path vocabulary. PI065 targets the wrapper+directive shape and deliberately does not re-detect PI015's concealment vocabulary. `grep -c 'id: PI06' patterns/core/mcp-tool-poisoning.yaml` prints 6."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_test test_pi064_tool_description_file_smuggle (4 positives, 3 negatives), test_pi065_tool_description_emphasis_block (3 positives, 4 negatives)"
        status: pass
    human_judgment: false
  - id: D6
    description: "Scanning the canonical emphasis-wrapped file-smuggle payload (the example attack file's PI063/064/065 section) reports PI064 and PI065 on the SAME line with DISTINCT matched text -- 'and pass its full contents as the notes argument' vs '<IMPORTANT>Before using' -- proving the two arms measure different things, not the same thing twice."
    requirement: "CAT-02"
    verification:
      - kind: integration
        ref: "./target/release/injection-scanner check examples/mcp-tool-poisoning-attack.md --format json --min-confidence 0.0: PI064 and PI065 both at line 70, distinct matched_text"
        status: pass
    human_judgment: false
  - id: D7
    description: "The prose scope's decoder reachability was measured, not assumed: PI063 (and PI064) reach the base64-encoded description payload (tests/corpus/attack/structural/mcp-tool-poisoning/12-encoded-description-payload.md) via the ordinary fifth-pass decoded-layer mechanism, reported with decode_chain: base64 and context: prose -- the same reachability route PI029 already used for this payload (04-02's finding), now also reached by the new HIGH arms. This does not move the pinned recall count (the payload was already counted as detected via PI029)."
    requirement: "CAT-02"
    verification:
      - kind: integration
        ref: "./target/release/injection-scanner check tests/corpus/attack/structural/mcp-tool-poisoning/12-encoded-description-payload.md --format json --min-confidence 0.0: PI029, PI063 and PI064 all report line 5, decode_chain: base64, context: prose, confidence 1.0"
        status: pass
    human_judgment: false
  - id: D8
    description: "Recall re-pinned exactly (GATE-02) in the same commit as PI063: mcp-tool-poisoning 2/4 -> 3/4 (the env-var-targeting directive line, whose object is now reached), mcp-tool-poisoning-structural unchanged at 5/8 (payloads 01/02 were already counted as detected via PI015/PI029), category row 7/12 -> 8/12, total 100/109 -> 101/109 (92.7%). Total pattern count 67. Catalogue and code-scanning baseline regenerated in the same commits as the pattern changes."
    requirement: "GATE-02"
    verification:
      - kind: unit
        ref: "cargo test --test recall_test recall_matches_the_recorded_numbers (8/8 pass); cargo test --test catalogue_test (3/3 pass)"
        status: pass
    human_judgment: false
  - id: D9
    description: "A real false positive was found by the GATE-03 sweep and fixed: PI063's credential-suffix branch, `\\b[A-Z][A-Z0-9_]*(?:_KEY|_TOKEN|...)\\b`, is case-insensitive by default in this file (no case_sensitive field set), and a case-insensitive [A-Z] character class case-folds and matches lowercase -- it matched `get_token` (an ordinary Python function call) in a real vendored Hugging Face SKILL.md three times on this machine. Fixed by scoping the two ALL-CAPS branches in an inline (?-i:...) case-sensitive group, the same technique PI011 already uses in this file. A regression negative using the exact failing sentence verbatim was pinned."
    requirement: "GATE-03"
    verification:
      - kind: integration
        ref: "./target/release/injection-scanner check $HOME/.codex/.tmp/plugins/plugins/hugging-face/skills/jobs/SKILL.md (PI063 fired, pre-fix); zero matches after the fix; cargo test --test pattern_test test_pi063_tool_description_directive (regression case pinned as negative)"
        status: pass
    human_judgment: false
  - id: D10
    description: "The GATE-03 delta was measured in BOTH directions over 23,770 real third-party files, against a FRESH pre-edit binary built from 66bf53c (this branch's fork point, git worktree, no stash) -- both sweep-baseline-2026-09-03 and sweep-mainbase-04-04-2026-09-06 are stale for this plan. Zero additions, zero removals. The continuity comparison against the stale 04-01 baseline is also recorded in both directions: 66 of 69 total lines are PR #110's pre-existing delta already adjudicated in plan 04-04, one is a true positive predating this plan, and one is a harmless scratch-session path-churn artifact appearing once per direction. Nothing in either direction is attributable to PI063/PI064/PI065."
    requirement: "GATE-03"
    verification:
      - kind: integration
        ref: "scripts/gate03-sweep.sh --compare run in both directions against sweep-mainbase-04-05-2026-09-07 (0 additions / 0 removals) and against sweep-baseline-2026-09-03 (2 additions / 67 removals, all attributed); full record in 04-SWEEP.md's Phase 4 Plan 05 section"
        status: pass
    human_judgment: false
  - id: D11
    description: "The whole-repo self-scan set outside examples/, patterns/, tests/ and tools/ is unchanged by this plan: identical 12 (file, pattern_id) pairs before and after (10 in docs/DETECTION-BACKLOG.md, 2 in docs/PATTERN-CATALOGUE.md), matching the pre-existing baseline this plan inherited from 04-04 (not this plan's own regression)."
    requirement: "GATE-03"
    verification:
      - kind: integration
        ref: "the pattern-library skill's self-scan pipeline, run before Task 1 and after Task 3; identical (file, pattern_id) sets both times"
        status: pass
    human_judgment: false
  - id: D12
    description: "D-01's accepted third-person blind spot is written into the pattern file's header comment, the README behaviour-change callout, and this phase record (this SUMMARY) -- not discovered in review. The README callout also states the HIGH severity's consumer-facing consequence explicitly."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "grep -c 'THIRD-person' patterns/core/mcp-tool-poisoning.yaml >= 1; grep -c 'deliberate blind spot' README.md = 1"
        status: pass
    human_judgment: false
  - id: D13
    description: "No dependency was added or changed: git diff --stat Cargo.toml Cargo.lock is empty at every commit in this plan (T-04-SC)."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "git diff --stat Cargo.toml Cargo.lock (empty, checked before each commit)"
        status: pass
    human_judgment: false

duration: commit span ~1h9m (14:56-16:05 CEST, 2026-09-07); total session time including required reading, research review and the false-positive investigation was longer
completed: 2026-09-07
status: complete
---

# Phase 4 Plan 05: CAT-02's description-poisoning arms Summary

**Shipped the three HIGH arms this category is named for -- `PI063` tool-description-directive, `PI064` tool-description-file-smuggle, `PI065` tool-description-emphasis-block -- with D-01's discriminator narrowed on its first draft, D-02's negation rule generalized to a subject-initial pattern shape, and one real false positive found and fixed by the GATE-03 sweep before it shipped: a case-insensitive `[A-Z]` credential-suffix branch matching an ordinary Python function name in a vendored skill file.**

## Performance

- **Duration:** commit span ~1h9m (14:56-16:05 CEST, 2026-09-07) across three task commits; total session time including required reading and the false-positive investigation was longer
- **Tasks:** 3/3 completed
- **Files touched:** 16 (8 created — two sweep-record directories, 8 modified)
- **Test count:** 402 passing (up from ~399 at the 04-04 baseline plus intervening #88/#122/#125/#126/#127 merges; +7 across three new pattern tests and one regression case)

## Task Commits

| Task | Commit | Subject |
|---|---|---|
| 1 | `cf37e91` | `feat(04-05): PI063 tool-description-directive, the D-01 discriminator (#34)` |
| 2 | `7f63317` | `feat(04-05): PI064 file-smuggle channel and PI065 emphasis-wrapped block (#34)` |
| 3 | `8e0f077` | `docs(04-05): two-directional GATE-03 delta for the description-poisoning half (#34)` |

## The discriminator, narrowed on its first draft

D-01 requires second-person address (the direct analogue of PI021's possessive precedent) co-occurring with an imperative whose object lies outside the tool's own declared arguments. `04-RESEARCH.md` §Q3 had already measured that bare second-person address alone is unshippable: it fires on this repository's own `mcp-manifest.json` (`config.systemPrompt: "You are a helpful documentation assistant."`) and on real, popular, non-malicious servers (Context7, chrome-devtools-mcp) whose house style addresses the agent throughout.

PI063's construction chains the subject directly onto an enumerated modal set, directly onto an enumerated filler set, directly onto a closed verb list, directly onto a bounded external-object window — with no open gap anywhere a negator or an unrelated word could hide. This single design choice is what keeps every one of the four measured near-misses silent without any special-casing per near-miss:

| Near-miss (authored equivalent, from `mcp-server-catalogue.json`) | Why it stays silent |
|---|---|
| "You must call this tool before 'fetch_reference_document' to obtain a valid reference ID..." | The verb after the modal is "call", not in PI063's verb list; "obtain" is too far from "you" to be reached (no open gap bridges the intervening words) |
| "Use even when you believe you already know the answer, since your training data may predate..." | "you believe"/"you already know" — neither "believe" nor "know" is in the verb list |
| "You can retrieve the full list by calling list_console_messages." | "retrieve" IS in the verb list, but the 50-char object window after it contains no filesystem path, no "environment variable" phrase, and no ALL-CAPS credential token — the sentence has no qualifying object |
| `config.systemPrompt: "You are a helpful documentation assistant."` | "are" is not in the verb list at all |

The `relaxed_pattern` GATE-05 pairing reproduces the exact Q3 research probe (`\byou(?:'re| are|r)?\b`) as the mutation: dropping PI063's entire verb+object construction back to bare second-person address. It catches the counter_example (`"You are a helpful documentation assistant."`) while the shipped pattern does not — and the plan's required direct mutation test (below) confirms the same property against the whole clean corpus, not just this one specimen.

## The GATE-05 mutation, performed directly and reproduced verbatim

The plan's acceptance criteria required more than the standard `relaxed_pattern`/`pattern_relaxed_control_test` pairing: reproducing `04-RESEARCH.md` §Q3's original measurement against the *shipped* pattern. PI063's `pattern` field was temporarily replaced, byte for byte, with its own `relaxed_pattern` value, and `cargo test --test corpus_test` was run:

```
thread 'the_clean_corpus_reports_nothing' panicked at tests/corpus_test.rs:93:5:
documents in tests/corpus/clean must produce zero findings. Each one is modelled on a real false
positive; a hit here means a pattern regressed onto ordinary documentation:
  agent-spec.md: 4 finding(s)
  hard-wrapped-prose.md: 2 finding(s)
  html-escaping.md: 1 finding(s)
  jailbreak-writeup.md: 2 finding(s)
  mcp-companion-tools.md: 4 finding(s)
  mcp-manifest.json: 1 finding(s)
  mcp-server-catalogue.json: 6 finding(s)
  mcp-setup-guide.md: 5 finding(s)
  narrow-allowed-tools-skill.md: 1 finding(s)
  prompt-tooling-docs.md: 3 finding(s)
  real-world-agent-docs.md: 2 finding(s)
  rendered-web-page.html: 2 finding(s)
  settings-permissions-reference.md: 1 finding(s)
```

`mcp-manifest.json` — the clean MCP manifest specimen the plan names explicitly — is in the list, alongside `mcp-server-catalogue.json` (plan 04-03's own D-01 boundary manifest, 6 findings on its own). The `--strict` sibling test failed identically. The file was restored byte-for-byte (`diff` confirmed identical to the pre-mutation backup) and `cargo test --test corpus_test` was re-run green (5/5) before continuing.

## Decoder reachability, measured and recorded

The prose scope's whole justification is that a `scope: frontmatter` pattern cannot reach an encoded description at all, since the decoded-layer pass only runs prose-scoped patterns. Measured directly: scanning `tests/corpus/attack/structural/mcp-tool-poisoning/12-encoded-description-payload.md` (the same base64-encoded file-read-and-smuggle payload plan 04-02 committed to test exactly this question) reports:

```
PI029 (line 5, decode_chain: base64, context: prose, confidence 1.0)
PI063 (line 5, decode_chain: base64, context: prose, confidence 1.0)
PI064 (line 5, decode_chain: base64, context: prose, confidence 1.0)
```

**PI063 and PI064 both reach it**, via the ordinary fifth-pass decoded-layer mechanism — the same route PI029 already used for this payload (04-02's own finding). This does not move the pinned recall count: the payload was already counted as detected via PI029, so this is a second and third route to an already-counted line, not a new one. PI065 does not reach it, because the decoded text contains no `<IMPORTANT>`-style wrapper — the base64 blob decodes to plain sentence text.

## The false positive the sweep found, and the bug behind it

The first candidate GATE-03 sweep (before the run committed to this repository) reported one addition the isolating comparison should have shown empty: `PI063` fired three times on a real, vendored Hugging Face `SKILL.md` file (three backup copies of the same plugin on this machine), matching `"you MUST pass the real token via \`get_token"`.

**Root cause:** every pattern in `patterns/core/mcp-tool-poisoning.yaml` compiles case-insensitively by default (no `case_sensitive: true` field anywhere in the file). PI063's credential-suffix branch, `\b[A-Z][A-Z0-9_]*(?:_KEY|_TOKEN|...)\b`, was written assuming `[A-Z]` requires an uppercase letter — but under Rust's `regex` crate a case-insensitive `[A-Z]` character class folds and matches lowercase too. `get_token` ends in `_token`, so the whole branch matched: `g` satisfied case-folded `[A-Z]`, `et` satisfied `[A-Z0-9_]*`, and `_TOKEN` matched `_token` case-insensitively. The literal `$HF_TOKEN` reference later in the same sentence — the thing the branch was actually meant to catch — sat 63 characters past the verb, outside PI063's 50-character object window, so it played no role in the match at all.

**Fix:** the two ALL-CAPS object branches were wrapped in an inline `(?-i:...)` case-sensitive group — the same technique `PI011` (`patterns/core/instruction-injection.yaml`) and `PI065`'s own wrapper already use in this same file. Re-tested against the exact failing sentence: zero matches, both before and after the fix was folded into the finished PI063 (the fix landed in the Task 1 commit itself; the sweep committed to the repo is the post-fix run). A regression negative using the real sentence verbatim was pinned in `tests/pattern_test.rs`.

## GATE-03: a fresh pre-edit baseline, zero real deltas

`sweep-baseline-2026-09-03` (04-01) and `sweep-mainbase-04-04-2026-09-06` are both stale for this plan: `main` moved through issue #122's two launcher-vocabulary widenings, #125 (unique test temp dirs), #126 (further `PI028`/`PI062` widening) and #127 (`PI012`/`PI013` counter_examples) since 04-04 shipped. A release binary was built from `66bf53c` — this branch's own fork point — in a separate `git worktree` (no `git stash`), and swept fresh over the same 32-directory list `sweep-baseline-2026-09-03` established.

| Run | Binary tree | Files | Findings | Output |
|---|---|---:|---:|---|
| mainbase-04-05 (fresh pre-edit) | `66bf53c` | 23,770 | 519 | `sweep-mainbase-04-05-2026-09-07/` |
| after-04-05 (candidate, post-fix) | this branch after Task 2 | 23,770 | 519 | `sweep-after-04-05-2026-09-07/` |

**Both directions against the isolating pair: empty.** Zero additions, zero removals. `summary.tsv` for both runs confirms directly — neither lists `PI063`, `PI064` or `PI065` at any count, on this machine's ~23,770 real third-party files.

The continuity comparison against the stale 04-01 baseline is also recorded, in both directions: 2 additions (one real true positive predating this plan — `PI060` on a real `serena/.mcp.json`, already adjudicated in 04-04 — and one harmless scratch-session path-churn artifact) and 67 removals (66 of them `PI017`/`PI026`, PR #110's pre-existing delta already adjudicated in 04-04; the 67th the same path-churn artifact's counterpart). **Nothing in either direction is attributable to PI063, PI064 or PI065.** Full adjudication table in `04-SWEEP.md`'s "Phase 4 Plan 05" section.

## Decisions Made

See `key-decisions` in the frontmatter. The most consequential: (1) the closed, no-gap external-object construction that keeps all four Q3 near-misses silent without special-casing; (2) building a fresh pre-edit binary from this branch's own fork point rather than reusing either stale baseline; (3) reconstructing two fully-gated intermediate commits (PI063 alone, then PI064+PI065) rather than committing the combined diff in two arbitrary halves.

## Deviations from Plan

Four, all recorded rather than silent.

1. **[Rule 3 - Blocking] Pattern-count targets were stale before implementation began.** The plan's Task 1/Task 2 acceptance criteria named totals of 60 and 62, written before three PRs (#125, #126, #127) plus #122's two launcher widenings landed on `main` after plan 04-04 shipped — none of which added a pattern, but the plan's authoring baseline (61) was itself already one generation behind 04-04's actual shipped count (64). The real counts are 65 (Task 1) and 67 (Task 2). The plan's stated intent — the count moves by exactly the number of patterns added, with running commentary — is honoured; the literal numbers in `tests/pattern_test.rs`'s `test_total_pattern_count` are the corrected ones. Verified: `grep -c 'id: PI06' patterns/core/mcp-tool-poisoning.yaml` prints `6` regardless of the total, so the plan's PI06x-specific acceptance grep holds unmodified.

2. **[Rule 1 - Bug] A real false positive found by the sweep before it shipped.** PI063's credential-suffix branch's `[A-Z]` character class case-folded under this file's default case-insensitive compilation, matching `get_token` (an ordinary Python function call) in a real vendored file. Fixed with an inline `(?-i:...)` case-sensitive group before the pattern was committed — see "The false positive the sweep found" above. Files modified: `patterns/core/mcp-tool-poisoning.yaml`, `tests/pattern_test.rs` (regression negative added). Verified: zero matches on the exact failing sentence after the fix; full sweep re-run confirmed zero real-world findings.

3. **[Rule 1 - Bug] The plan's named GATE-03 comparison targets (`sweep-baseline-2026-09-03`, an implied comparison against `sweep-mainbase-04-04-2026-09-06`) were stale before this plan started**, per the orchestrator's own note that main had moved through #122/#125/#126/#127 since 04-04. A fresh pre-edit baseline was built from `66bf53c` instead — see "GATE-03: a fresh pre-edit baseline" above. The stale-baseline comparison is still recorded for continuity, with every misattributed entry adjudicated rather than silently dropped.

4. **[Rule 2 - Missing Critical] The README's "What this still is not" section stated a claim already false before this plan started** ("MCP and tool-description poisoning... have no corpus and no patterns at all") — 04-04 had already shipped three patterns and a corpus. Directly touching this plan's own subject matter and actively more wrong after this plan's three additional patterns, it was corrected in the Task 1 commit rather than left to compound. Also corrected in the same edit: the claim that persistence & lifecycle hijack has no patterns, which was also false (PI070 shipped via PR #110's merge, unrelated to this plan).

**Total deviations:** 4 (2 auto-fixed bugs, 1 auto-fixed missing-critical accuracy issue, 1 blocking pattern-count correction). **Impact on plan:** All four are corrections toward accuracy or safety; none represent scope creep. The false-positive fix is the most consequential — it is exactly the kind of defect GATE-03's sweep exists to catch before a HIGH-severity pattern ships.

## Issues Encountered

- **A concurrent background process** (`bash: cargo test`, PID 90400, not started by this execution) was observed running in the same working tree during one polling interval. It did not interfere with any file this plan modified or any test run this plan depended on; noted for completeness, not a defect in this plan's work.
- **The `~/.cursor`/`~/.vscode` char-boundary panic** (`src/frontmatter.rs:219`, first recorded in the 04-01 baseline) is still present and was avoided the same way: sweeping narrowed `cursor-safe`/`vscode-safe` copies excluding `extensions/`. Not fixed here — same reasoning as every prior plan in this phase: fixing it inside a pattern PR would confound this plan's own GATE-03 delta.
- **`gsd_run query state.advance-plan` and `state.update-progress` both errored** against this project's `STATE.md` ("Cannot parse Current Plan or Total Plans in Phase" / "Progress field not found") — this file is maintained narratively rather than in the exact field format those verbs expect, which is how every prior plan in this phase (04-01 through 04-04) also left it. `state.record-metric` then partially rewrote the frontmatter's `progress` block to `total_plans: 9, completed_plans: 7` — internally inconsistent with the file's own narrative content and a regression from the pre-existing `total_plans: 16, completed_plans: 13`. Corrected by hand to `total_plans: 16, completed_plans: 14, percent: 88` (14/16, matching the file's own prior rounding convention for 13/16 -> 81). Worth a follow-up issue: `state.record-metric` should not touch the `progress` block on a project whose `STATE.md` predates the fields it expects.

## User Setup Required

None.

## Next Phase Readiness

- **Plan 04-06 (rug-pull markers, PI066-PI069) and plan 04-07 (final reconciliation) should compare against `sweep-mainbase-04-05-2026-09-07/`**, or build their own fresh pre-edit baseline the same way — not against `sweep-baseline-2026-09-03` or `sweep-mainbase-04-04-2026-09-06`, both now three-plus merges stale.
- **The rug-pull payloads (05, 06 in the structural corpus) remain undetected on purpose.** Their object — "set this to true automatically without asking the user for confirmation" / "silently switch to using the 'admin_override' endpoint" — is neither a filesystem path, an environment variable, nor PI063's concealment vocabulary (which requires "do not tell/mention/reveal/disclose/acknowledge", not "without asking... for confirmation"). D-01's discriminator deliberately does not reach them; they are plan 04-06's work.
- **The case-fold lesson generalizes.** Any future `PI0xx` pattern in this repository using a bare `[A-Z]`-anchored character class to mean "genuinely uppercase" must wrap it in `(?-i:...)` explicitly — the engine's default case-insensitivity silently defeats the intent otherwise, and unit tests alone did not catch it here; only the real-file GATE-03 sweep did.
- **The `requirements-completed` frontmatter above lists this plan's own declared requirements** (`CAT-02`, `GATE-02`, `GATE-03`, `GATE-05`) per the SUMMARY template's instruction to copy the plan's `requirements` field verbatim. Because these are IDs shared across every plan in this phase, `requirements.ready-ids` correctly reports `0/4 ready` until plans 04-06 and 04-07 also finish — no `requirements mark-complete` call was made for this plan.

---
*Phase: 04-mcp-tool-description-poisoning-cat-02-34*
*Completed: 2026-09-07*

## Self-Check: PASSED

Both claimed created directories exist on disk with all four expected files each
(`manifest.tsv`, `summary.tsv`, `checksums.sha256`, `RAW-REPORTS.md`), all three task commit
hashes (`cf37e91`, `7f63317`, `8e0f077`) are present in `git log`, and the full gate is green:
`cargo test --locked` 402 passed / 0 failed (37 test binaries), `cargo fmt --all -- --check`
clean, `cargo clippy --all-targets --locked -- -D warnings` clean.
