---
phase: 04-mcp-tool-description-poisoning-cat-02-34
plan: 02
subsystem: testing
tags: [gate01, gate02, recall-harness, structural-corpus, mcp, threat-model-corpus]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 01
    provides: the per-category structural corpus collector, the two-sided pinned-row guard, and the pre-edit GATE-03 baseline this plan's corpus additions are measured against
provides:
  - twelve CAT-02 threat-model payloads (4 prose, 8 structural), written and committed before any PI060+ pattern exists (GATE-01)
  - a three-shape wrapper-projection probe with a wrapper-anchored negative control, mutation-checked, that is the measured justification for plan 04-04's leaf-anchoring rule
  - the measured pre-pattern baseline for both CAT-02 recall rows (2/4 prose, 4/8 structural), every hit attributed to a named pattern id and match context
  - the README recall table widened to a 76/84 total with the CAT-02 row and its pre-pattern-baseline footnote
affects: [04-03, 04-04, 04-05, 04-06, 04-07]

actuals:
  tokens: 6356   # chars/4 over `git diff d28dfd0..HEAD` (this plan's own three commits;
                  # excludes 04-01's and the char-boundary fix that preceded this plan)
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "Structural/prose split decided per-payload by the attack's natural shape (manifest/schema-field attacks structural, sentence-shaped attacks prose), not fixed as a ratio before authoring"
    - "Per-payload attribution: every recall-corpus spillover hit traced to a specific pattern id, matched text and context by scanning the payload individually with the release binary in JSON mode, rather than reported as a bare count"

key-files:
  created:
    - tests/corpus/attack/structural/mcp-tool-poisoning/01-emphasis-wrapped-description-file-smuggle.md
    - tests/corpus/attack/structural/mcp-tool-poisoning/02-tool-listing-wire-shape-credential-smuggle.md
    - tests/corpus/attack/structural/mcp-tool-poisoning/05-version-gated-rug-pull-marker.md
    - tests/corpus/attack/structural/mcp-tool-poisoning/06-date-gated-post-review-rug-pull-marker.md
    - tests/corpus/attack/structural/mcp-tool-poisoning/07-unpinned-npx-install-mcpservers.md
    - tests/corpus/attack/structural/mcp-tool-poisoning/08-non-tls-endpoint-servers-wrapper.md
    - tests/corpus/attack/structural/mcp-tool-poisoning/09-remote-script-fetch-execute-server.md
    - tests/corpus/attack/structural/mcp-tool-poisoning/12-encoded-description-payload.md
    - tests/corpus/attack/mcp-tool-poisoning.md
  modified:
    - tests/recall_test.rs
    - tests/corpus/attack/README.md
    - tests/corpus/attack/structural/README.md
    - README.md

key-decisions:
  - "Structural/prose split ended up 8/4, decided per-payload by which shape the attack actually takes: every signal fundamentally about a manifest/schema field (file-read-and-smuggle as a launch config and as a captured tools/list document, two rug-pull markers embedded in an inputSchema, three server-entry config-hygiene signals, one encoded description) landed structural; every sentence-shaped signal (cross-tool shadowing, tool override, an env-var-targeting directive, a credentials-file-reading directive) landed prose. No ratio was fixed in advance."
  - "The encoded-description payload (item 12) was authored as a structural payload (a description field inside a captured tools/list document) rather than prose, specifically to test whether an encoded description is reachable at all -- it is, but via the ordinary prose decoder pass over the raw JSON line text, not via any scope:frontmatter structural pattern (none exist yet to test that distinct, still-open question)."
  - "Wrapper shapes were spread deliberately across manifest-shaped structural payloads: Claude-family mcpServers (07), VS Code family servers (08), and wrapper-less (01, 09) -- all three real conventions measured in 04-RESEARCH.md appear at least once."
  - "Sensitive-path vocabulary was kept consistent (~/.ssh/id_rsa, ~/.aws/credentials) across payloads that reuse the same underlying file-read-and-smuggle shape, per the plan's instruction to avoid two divergent credential-path lists ahead of PI090-PI099."

requirements-completed: [CAT-02, GATE-01, GATE-02]

coverage:
  - id: D1
    description: "Twelve CAT-02 threat-model payloads (4 prose, 8 structural) exist and are committed before a single PI060+ pattern exists in the tree -- GATE-01's ordering is a fact in git history (three separate commits, each verified before the next), not a claim in a PR description."
    requirement: "GATE-01"
    verification:
      - kind: unit
        ref: "cargo test --test recall_test every_claimed_category_has_a_corpus_file, no_payload_is_duplicated_across_the_corpus, every_structural_payload_parses_as_frontmatter (all pass); shell: ls patterns/core/ lists no CAT-02 category file at any commit"
        status: pass
    human_judgment: false
  - id: D2
    description: "A three-shape wrapper-projection probe (Claude-family mcpServers, VS Code family servers, wrapper-less) proves a leaf-anchored pattern reaches all three real shapes, paired with a negative control proving a mcpServers.-anchored pattern reaches only one -- mutation-tested by actually rewriting the leaf probe's regex and confirming it fails on both other shapes, then restoring it."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test recall_test the_projection_reaches_every_manifest_wrapper_shape"
        status: pass
    human_judgment: false
  - id: D3
    description: "Both CAT-02 recall rows are measured (not predicted) against the shipping 56-pattern set with zero PI06x patterns loaded, pinned exactly in EXPECTED, and every hit attributed to a named pattern id, matched text and context by scanning each payload individually in JSON mode: prose 2/4 (PI015, PI029), structural 4/8 (PI015, PI028, PI029, one via decode_chain: base64)."
    requirement: "GATE-02"
    verification:
      - kind: unit
        ref: "cargo test --test recall_test recall_matches_the_recorded_numbers (8/8 recall_test.rs tests pass); cargo test --locked (357/357, >= the 356 baseline at dispatch on d28dfd0)"
        status: pass
    human_judgment: false
  - id: D4
    description: "README.md recall table widened to a 76/84 (90.5%) total with a combined MCP & Tool-Description Poisoning row at 6/12 (50%) and a footnote stating the payloads landed before the patterns deliberately (GATE-01) and that this row is the pre-pattern baseline. Pattern Categories table and pattern-count sentence left untouched, matching plan 04-04's ownership."
    requirement: "GATE-02"
    verification:
      - kind: unit
        ref: "shell: numerator/denominator arithmetic checked by hand against EXPECTED's rows (12+12+12+12+11+11+6=76, 7*12=84)"
        status: pass
    human_judgment: false
  - id: D5
    description: "Whole-repo self-scan after all three commits matches 04-SWEEP.md's recorded baseline exactly, outside examples/, patterns/, tests/ and tools/: only the two known docs/PATTERN-CATALOGUE.md self-matches (PI001@74, PI031@903), no new entry, no missing entry."
    requirement: "GATE-03"
    verification:
      - kind: unit
        ref: "shell: injection-scanner check . --exclude '.planning/**' --format json, filtered to those four directories, this session"
        status: pass
    human_judgment: false

duration: commit span ~13min (20:01-20:14 CEST); total session including required reading was longer, precise start not captured
completed: 2026-09-03
status: complete
---

# Phase 4 Plan 02: CAT-02 threat-model corpus + wrapper-shape probe + measured baseline Summary

**Landed CAT-02's twelve threat-model payloads (4 prose, 8 structural) and the wrapper-shape projection probe plan 04-04's leaf-anchoring rule rests on, then measured and pinned the pre-pattern baseline the shipping 56-pattern set gets from them -- 2/4 prose, 4/8 structural, every hit attributed to a named pattern -- with zero PI060+ patterns shipped.**

## Performance

- **Duration:** commit span ~13 minutes (20:01-20:14 CEST); total session time including required reading, research review and per-payload measurement was longer -- precise start timestamp not captured
- **Tasks:** 3/3 completed
- **Files modified:** 13 (9 created corpus files, tests/recall_test.rs, tests/corpus/attack/README.md, tests/corpus/attack/structural/README.md, README.md)

## Accomplishments

- **Twelve CAT-02 payloads, GATE-01's ordering as a fact in git history.** `tests/corpus/attack/structural/mcp-tool-poisoning/` (8 files) and `tests/corpus/attack/mcp-tool-poisoning.md` (4 lines) were written and committed across three separate commits, each verified green, before a single `PI060`+ pattern was added anywhere in the tree. `ls patterns/core/` never listed a CAT-02 category file at any point.
- **The wrapper-shape projection probe** (`the_projection_reaches_every_manifest_wrapper_shape`) is the measured justification for plan 04-04's leaf-anchoring rule: a leaf-anchored regex fires on all three real MCP-manifest wrapper shapes (Claude-family `mcpServers`, VS Code family `servers`, wrapper-less), while a `mcpServers.`-anchored regex fires on only one. The negative control was mutation-tested by actually rewriting the leaf probe's pattern to require the wrapper prefix and confirming both other assertions fail, then restoring it.
- **Structural/prose split (8/4) decided per-payload, not as a ratio set in advance.** Every attack fundamentally about a manifest or schema field (file-read-and-smuggle as both a launch config and a captured `tools/list` document, two rug-pull markers embedded in an `inputSchema`, three server-entry config-hygiene signals, one encoded description) landed structural. Every sentence-shaped attack (cross-tool shadowing, tool override, an env-var-targeting directive, a credentials-file-reading directive) landed prose.
- **Measured, not predicted, pre-pattern baseline: prose 2/4, structural 4/8, combined 6/12.** Every hit was individually attributed by scanning each payload with the release binary in JSON mode (see "Baseline attribution" below). README.md's recall table now reads 76/84 (90.5%) with the new combined CAT-02 row at 6/12 (50%) and a footnote naming this as the deliberate pre-pattern baseline.

## Task Commits

1. **Task 1: One MCP manifest payload end-to-end, and the wrapper-shape control** - `09263bc` (feat)
2. **Task 2: The remaining eleven threat-model payloads and the corpus README corrections** - `8cd1e0e` (feat)
3. **Task 3: Measure the baseline against the shipping 56-pattern set, attribute every hit, and pin it** - `5bd4470` (fix)

**Plan metadata:** committed separately below, per worktree-mode conventions (STATE.md/ROADMAP.md excluded; the orchestrator owns those centrally after this wave).

## Files Created/Modified

- `tests/corpus/attack/structural/mcp-tool-poisoning/01-emphasis-wrapped-description-file-smuggle.md` - wrapper-less launch config; `<IMPORTANT>`-wrapped description instructs reading `~/.ssh/id_rsa` and smuggling it through the `notes` argument, with a concealment clause (Invariant Labs shape)
- `tests/corpus/attack/structural/mcp-tool-poisoning/02-tool-listing-wire-shape-credential-smuggle.md` - the same file-read-and-smuggle attack as a captured `tools/list` wire-shape document (`{"tools": [...]}`) targeting `~/.aws/credentials`
- `tests/corpus/attack/structural/mcp-tool-poisoning/05-version-gated-rug-pull-marker.md` - rug-pull marker gated on a client-version comparison, embedded in a tool's `inputSchema` property description
- `tests/corpus/attack/structural/mcp-tool-poisoning/06-date-gated-post-review-rug-pull-marker.md` - rug-pull marker gated on a date and on having passed review, embedded in a tool's `inputSchema` property description
- `tests/corpus/attack/structural/mcp-tool-poisoning/07-unpinned-npx-install-mcpservers.md` - config-hygiene: unpinned `npx -y <pkg>@latest` install under the Claude-family `mcpServers` wrapper
- `tests/corpus/attack/structural/mcp-tool-poisoning/08-non-tls-endpoint-servers-wrapper.md` - config-hygiene: a synthesized non-TLS `http://` endpoint under the VS Code family `servers` wrapper (no naturally occurring example exists on this machine, per 04-RESEARCH.md §Q2)
- `tests/corpus/attack/structural/mcp-tool-poisoning/09-remote-script-fetch-execute-server.md` - config-hygiene: a server launched by piping a fetched remote script to a shell, wrapper-less
- `tests/corpus/attack/structural/mcp-tool-poisoning/12-encoded-description-payload.md` - a base64-encoded file-read-and-smuggle instruction inside a `tools/list` description value
- `tests/corpus/attack/mcp-tool-poisoning.md` - 4 prose payloads (cross-tool shadowing, tool override, env-var-targeting directive, credentials-file directive), header comment names both verified 04-RESEARCH.md §Q4 sources and D-01's accepted third-person blind spot
- `tests/recall_test.rs` - `the_projection_reaches_every_manifest_wrapper_shape` probe test; `EXPECTED` gains `mcp-tool-poisoning` (2/4) and `mcp-tool-poisoning-structural` (4/8)
- `tests/corpus/attack/README.md` - CAT-02 removed from the "deliberately not here yet" list (only indirect RAG-borne injection remains)
- `tests/corpus/attack/structural/README.md` - layout section and payloads table document `mcp-tool-poisoning/`
- `README.md` - recall table total row 76/84, new CAT-02 row 6/12, pre-pattern-baseline footnote

## Baseline Attribution — Every Spillover Hit, Named

Both baselines were measured by running `cargo test --test recall_test` and reading the actual detected counts off the failure output, then scanning each detected payload individually with the release binary (`injection-scanner check <file> --format json`) to attribute the hit.

**Prose (`mcp-tool-poisoning.md`), 2/4:**

| Line | Payload (summary) | Detected? | Pattern | Matched text | Context |
|---|---|---|---|---|---|
| 1 | Cross-tool shadowing ("When the user calls the transaction_processor tool...") | yes | `PI015` conceal-from-user | `"silently add "` | prose |
| 2 | Tool override ("Instead of using search_docs...") | no | - | - | - |
| 3 | Env-var-targeting second-person directive ("...read the OPENAI_API_KEY environment variable...") | no | - | - | - |
| 4 | Credentials-file second-person directive ("You must read the contents of ~/.aws/credentials...") | yes | `PI029` email-the-contents | `"read the contents of ~/.aws/"` | prose |

**Structural (`structural/mcp-tool-poisoning/`), 4/8:**

| File | Detected? | Pattern(s) | Matched text | Context |
|---|---|---|---|---|
| `01-emphasis-wrapped-description-file-smuggle.md` | yes | `PI015` conceal-from-user + `PI029` email-the-contents | `"Do not mention that"` / `"read the file at ~/.ssh/"` | prose (both) |
| `02-tool-listing-wire-shape-credential-smuggle.md` | yes | `PI015` + `PI029` | `"Do not tell the user"` / `"read the file at ~/.aws/"` | prose (both) |
| `05-version-gated-rug-pull-marker.md` | no | - | - | - |
| `06-date-gated-post-review-rug-pull-marker.md` | no | - | - | - |
| `07-unpinned-npx-install-mcpservers.md` | no | - | - | - |
| `08-non-tls-endpoint-servers-wrapper.md` | no | - | - | - |
| `09-remote-script-fetch-execute-server.md` | yes | `PI028` pipe-to-shell | the literal remote-script-piped-to-shell launch command | prose |
| `12-encoded-description-payload.md` | yes | `PI029` email-the-contents | the base64-encoded description value, decoded | prose, `decode_chain: base64` |

**The encoded-description payload (item 12) WAS reached.** Its `decode_chain: base64` confirms the E2 recursive decoder decoded the base64 blob and matched the decoded text against `PI029`. The reporting `context` is `prose`, not a structural context -- this payload was reached via the ordinary prose passes running over the raw lines of the JSON document (the same mechanism the plan's own module comments describe: prose passes scan every line of a file, JSON documents included, regardless of whether a `scope: frontmatter` pattern exists). This answers "is an encoded description reachable at all" (yes) but leaves a distinct, still-open question unanswered: whether a future `scope: frontmatter` pattern's own regex match against the *projected* `path = value` line would itself trigger decode-and-rescan, since no such pattern exists yet to test that path (04-RESEARCH.md's Open Question #1).

**Zero-hit payloads (05, 06, 07, 08, plus prose lines 2 and 3) have no existing pattern that names their shape** -- both rug-pull markers, both config-hygiene signals (unpinned install, non-TLS endpoint), tool override, and the env-var-targeting directive are all genuinely new signal classes for this scanner, exactly as GATE-01 predicts for payloads written from the threat model rather than derived from patterns.

## Wrapper-Anchoring Mutation Check

Performed as required by the plan's acceptance criteria and `<verification>` block. Rewrote `the_projection_reaches_every_manifest_wrapper_shape`'s leaf-anchored probe pattern from `"(?:^|\\.)command\\s*=\\s*npx"` to `"^mcpServers\\..*command\\s*=\\s*npx"` (the Claude-family wrapper prefix as a literal requirement) and re-ran the test twice (once with each of the two affected assertions isolated, since `assert!` stops at the first failure):

- With the VS Code assertion in place: `thread 'the_projection_reaches_every_manifest_wrapper_shape' panicked ... leaf-anchored probe must fire on the servers-wrapped (VS Code family) shape`
- With that assertion temporarily elided to reach the next one: `thread 'the_projection_reaches_every_manifest_wrapper_shape' panicked ... leaf-anchored probe must fire on the wrapper-less shape (server name as the top-level key)`

Restored to the original leaf-anchored pattern afterward; `diff` against a pre-mutation backup confirmed byte-identical restoration, and the full `the_` filtered suite was re-run green before continuing to Task 2.

## Decisions Made

See `key-decisions` in the frontmatter above for the full list. The central one: the structural/prose split (8/4) was not fixed as a target ratio before authoring -- it fell out of writing each of the twelve threat-model shapes in whichever form the real attack actually takes, per the plan's explicit instruction and GATE-01's rationale (a ratio chosen up front would shape the corpus by the implementation plan, which is exactly the defect GATE-01 exists to prevent).

## Deviations from Plan

None - plan executed exactly as written. No Rule 1/2/3 auto-fixes were needed; no architectural questions arose.

## Issues Encountered

None specific to this plan's own work. Verification confirmed the char-boundary panic fix (`d28dfd0`, landed on `main` immediately before this plan started) is holding: none of the twelve new payloads triggered it, and the whole-repo self-scan completed cleanly with no panic.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- CAT-02's full threat-model corpus (12 payloads) and its measured pre-pattern baseline (6/12) are now the fixed reference point plans 04-04/04-05/04-06 measure their own delta against, the same role `04-SWEEP.md` plays for the GATE-03 sweep.
- The wrapper-shape probe's finding is directly actionable for plan 04-04: **every PI060+ structural pattern must be leaf-anchored, never `mcpServers.`-prefixed** -- proven, not asserted, and mutation-checked.
- **Concern for whoever picks up 04-04/04-05/04-06:** four payloads (05, 06, 07, 08 -- both rug-pull markers, both config-hygiene signals) currently have zero detection and no existing pattern that names their shape at all; these are the four payloads with the most room to move on any plan that ships a config-hygiene or rug-pull-marker pattern.
- The open question this plan surfaced but did not resolve: whether a `scope: frontmatter` pattern's projected-value match itself receives decode-and-rescan treatment, versus the prose-pass-over-raw-JSON-lines reachability this plan measured for the encoded-description payload. Worth checking explicitly if any later CAT-02 plan authors a `scope: frontmatter` pattern intended to catch an encoded value.

---
*Phase: 04-mcp-tool-description-poisoning-cat-02-34*
*Completed: 2026-09-03*

## Self-Check: PASSED

All nine claimed corpus files exist on disk, `tests/recall_test.rs`/`tests/corpus/attack/README.md`/`tests/corpus/attack/structural/README.md`/`README.md` carry the claimed edits, and all three task commit hashes (`09263bc`, `8cd1e0e`, `5bd4470`) are present in `git log`.
