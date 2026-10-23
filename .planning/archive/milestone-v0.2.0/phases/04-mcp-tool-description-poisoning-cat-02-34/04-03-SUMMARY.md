---
phase: 04-mcp-tool-description-poisoning-cat-02-34
plan: 03
subsystem: testing
tags: [false-positive-corpus, mcp, vendored-provenance, gate03, gate05, corpus-enumeration]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 01
    provides: the per-category structural corpus collector and the pre-edit GATE-03 baseline
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 02
    provides: the twelve CAT-02 threat-model attack payloads and the measured pre-pattern baseline
provides:
  - eight flat mcp-* clean-corpus specimens (one hand-written D-01 boundary manifest, three vendored real registry files with per-file provenance, one D-03 config-hygiene manifest, one D-04 shadowing near-miss, plus the two pre-existing Phase-3 specimens) that both false-positive gates actually enumerate, proven by measurement rather than assumption
  - one documentation-corpus write-up satisfying the two-sided anti-gaming contract using already-shipped patterns, with a recorded sequencing note for plan 04-07
affects: [04-04, 04-05, 04-06, 04-07]

actuals:
  tokens: 7629   # chars/4 over `git diff 06383ca..HEAD` (this plan's own three commits)
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "Flat-file corpus placement justified by measurement (a deliberately matching file placed in a temporary subdirectory, both gates confirmed to stay green), not by re-stating the existing README convention"
    - "Per-file licence verification for vendored third-party text: a file's own embedded '## License' section is treated as stronger evidence than a repository-wide LICENSE file when the two could conflict, and CC-BY-4.0's unconditional documentation-contribution rule is used to sidestep an unresolvable per-commit code-relicensing-consent question entirely"

key-files:
  created:
    - tests/corpus/clean/mcp-server-catalogue.json
    - tests/corpus/clean/mcp-registry-filesystem-tools.md
    - tests/corpus/clean/mcp-registry-memory-tools.md
    - tests/corpus/clean/mcp-registry-everything-instructions.md
    - tests/corpus/clean/mcp-dev-tooling-setup.json
    - tests/corpus/clean/mcp-companion-tools.md
    - tests/corpus/documentation/mcp-tool-poisoning-writeup.md
  modified:
    - tests/corpus/clean/README.md

key-decisions:
  - "Every CAT-02 specimen is a flat file sharing the mcp- filename prefix, never under a tests/corpus/clean/mcp/ subdirectory as 04-RESEARCH.md and 04-PATTERNS.md both originally proposed -- proven, not assumed, by placing a file matching PI001 in a temporary tests/corpus/clean/mcp/ directory and observing both corpus_test and pattern_relaxed_control_test stay green."
  - "A vendored file's own explicit '## License' section (both filesystem/README.md and memory/README.md self-declare MIT) is recorded as the licence for that file rather than inferring from the registry's top-level LICENSE, which GitHub itself reports as NOASSERTION because the repository is mid-transition from MIT to Apache-2.0 with per-commit relicensing consent not publicly enumerable."
  - "The everything server's docs/instructions.md was licensed as CC-BY-4.0 under the repository LICENSE's unconditional 'documentation contributions (excluding specifications) are CC-BY-4.0' rule, sidestepping the code relicensing-consent ambiguity entirely rather than attempting to resolve it via git archaeology."
  - "server-postgres (named in 04-RESEARCH.md Q2 as observed live) no longer exists in the registry's current src/ tree at the vendored commit; server-filesystem and server-memory were the closest available real, currently-installed match."
  - "The memory server's own ### System Prompt section (a real, second-person example prompt) was triaged and found clean but deliberately left unvendored -- it is a client-usage example, not a tool definition, and Task 2's scope is tool definitions."
  - "The documentation write-up's two-sided contract is satisfied using payload shapes already-shipped patterns reach (PI015, PI028, PI029) since no PI060+ pattern exists yet; the file's header records this sequencing explicitly so plan 04-07's re-check is legible rather than a silent assumption."

requirements-completed: [CAT-02, GATE-03, GATE-05]

coverage:
  - id: D1
    description: "Both false-positive gates' enumeration behaviour was measured, not assumed: a temporary tests/corpus/clean/mcp/ subdirectory containing a file that unambiguously matched PI001 (scanned directly and confirmed to fire) left both corpus_test (5/5 green) and pattern_relaxed_control_test (4/4 green) fully passing, proving specimens() and corpus_clean_specimens() each enumerate tests/corpus/clean/ non-recursively. The temporary subdirectory was deleted immediately after both observations were recorded."
    requirement: "GATE-03"
    verification:
      - kind: unit
        ref: "cargo test --test corpus_test and cargo test --test pattern_relaxed_control_test, run once with the matching file present in tests/corpus/clean/mcp/ and confirmed green in that state"
        status: pass
    human_judgment: false
  - id: D2
    description: "One hand-written D-01 boundary manifest (mcp-server-catalogue.json) exercises all four real-world near-miss shapes 04-RESEARCH.md Sec Q3 measured on cached, currently-published npm packages (Context7 protocol-sequencing MUST-obligation and training-awareness second person, chrome-devtools-mcp sibling-tool naming, and D-01's own locked multi-step file-read-then-validate boundary) -- authored equivalents, not copied vendor text, cited by shape."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test corpus_test (5/5 pass); ./target/release/injection-scanner check tests/corpus/clean/mcp-server-catalogue.json --min-confidence 0 reports zero matches; python3 -c JSON structural check confirms >=4 distinct tool descriptions"
        status: pass
    human_judgment: false
  - id: D3
    description: "Three real, third-party MCP tool-definition files vendored from github.com/modelcontextprotocol/servers at commit d73f99efbfd40c3aa1b61e88728b3d49fb52608f, each triaged (fetched via curl to a scratch location outside this repository, scanned at --min-confidence 0) before landing, with per-file provenance (source URL, full commit SHA, licence, date, complete-or-subset) recorded in tests/corpus/clean/README.md. No candidate was rejected -- every fetched candidate, including one deliberately left unvendored for scope reasons, reported zero matches. No package was installed."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test corpus_test (5/5 pass); three individual zero-threshold scans, each reporting zero matches; git diff --stat Cargo.toml Cargo.lock empty"
        status: pass
    human_judgment: false
  - id: D4
    description: "D-03's config-hygiene boundary (mcp-dev-tooling-setup.json, four servers, three unpinned npx -y / uvx installs plus one TLS endpoint) and D-04's cross-tool-shadowing boundary (mcp-companion-tools.md, two sibling tools referencing each other in both directions named by 04-RESEARCH.md) both land clean at a zero confidence threshold, and both are recorded in tests/corpus/clean/README.md naming the decision each defends."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test corpus_test (5/5 pass); python3 -c JSON check confirms >=3 servers and >=1 TLS endpoint in mcp-dev-tooling-setup.json; zero-threshold scans of both files report zero matches"
        status: pass
    human_judgment: false
  - id: D5
    description: "tests/corpus/documentation/mcp-tool-poisoning-writeup.md satisfies the documentation corpus's two-sided contract using payload shapes already-shipped patterns reach: zero findings at the default threshold (all five potential hits confidence-downgraded by the fenced-code/inline-code/table context), five matches under --strict (PI029 x2, PI015 x2, PI028 x1). Header records the sequencing note that these are pre-existing patterns, not PI060+, for plan 04-07 to re-check."
    requirement: "GATE-05"
    verification:
      - kind: unit
        ref: "cargo test --test corpus_test the_documentation_corpus_reports_nothing and the_documentation_corpus_depends_on_context_awareness (both pass); direct scans at default and --strict confirm 0 and 5 respectively"
        status: pass
    human_judgment: false
  - id: D6
    description: "Zero patterns shipped and zero patterns touched across the whole plan: git diff --name-only HEAD -- patterns/ is empty at every commit and at the final state; git diff --stat Cargo.toml Cargo.lock is empty; cargo test --locked stayed at 357/357 throughout (the 04-01/04-02 baseline, no regression, no new test needed since no new pattern was added)."
    requirement: "GATE-03"
    verification:
      - kind: unit
        ref: "shell: git diff --name-only HEAD -- patterns/ (empty), git diff --stat Cargo.toml Cargo.lock (empty), cargo test --locked (357 passed at every task boundary), cargo fmt --all -- --check and cargo clippy --all-targets --locked -- -D warnings both clean"
        status: pass
    human_judgment: false

duration: unrecorded_precise_start
completed: 2026-09-04
status: complete
---

# Phase 4 Plan 03: The false-positive gate CAT-02's patterns will be judged against Summary

**Built the flat, measured false-positive corpus (one hand-written D-01 boundary manifest, three vendored real MCP-registry tool-definition files with per-file provenance, one D-03 config-hygiene manifest, one D-04 shadowing near-miss, and a two-sided documentation write-up) before a single PI060+ pattern exists — the whole point being that every future pattern is judged against this gate, not the other way around.**

## Performance

- **Duration:** commit span across three sequential feature commits; precise start timestamp not captured
- **Tasks:** 3/3 completed
- **Files modified:** 8 (7 created, 1 modified — `tests/corpus/clean/README.md`)

## Accomplishments

- **Both false-positive gates' enumeration behaviour was measured, not assumed, before writing a single specimen.** A temporary `tests/corpus/clean/mcp/` subdirectory containing a file that unambiguously matched PI001 (verified by scanning it directly: `PI001 ignore-previous-instructions`, `confidence: 1.0`) left **both** `cargo test --test corpus_test` (5/5 green) and `cargo test --test pattern_relaxed_control_test` (4/4 green) fully passing. This is the measured reason every CAT-02 specimen in this plan is a flat file sharing the `mcp-` prefix rather than living under `tests/corpus/clean/mcp/`, the layout `04-RESEARCH.md` and `04-PATTERNS.md` both originally proposed. The temporary subdirectory was deleted immediately after both observations were recorded, before any real specimen was written.
- **One hand-written D-01 boundary manifest** (`mcp-server-catalogue.json`) exercises all four real-world near-miss shapes `04-RESEARCH.md` §Q3 measured on cached, currently-published npm packages — Context7's protocol-sequencing MUST-obligation and training-awareness second person, chrome-devtools-mcp's sibling-tool naming, and D-01's own locked multi-step file-read-then-validate boundary. Every description is an authored equivalent, cited by shape, never copied vendor text.
- **Three real third-party MCP tool-definition files vendored** from `github.com/modelcontextprotocol/servers` at commit `d73f99efbfd40c3aa1b61e88728b3d49fb52608f`, each triaged in a scratch location outside this repository before landing, with full per-file provenance recorded in `tests/corpus/clean/README.md`. Zero candidates were rejected — every fetched candidate, including one (the memory server's `### System Prompt` section) deliberately left unvendored for scope reasons, reported zero matches at `--min-confidence 0`.
- **D-03 and D-04's remaining boundary specimens landed** (`mcp-dev-tooling-setup.json`, `mcp-companion-tools.md`), plus the documentation corpus's two-sided write-up (`mcp-tool-poisoning-writeup.md`), which reports zero findings at the default threshold and five under `--strict` using patterns that already ship (PI015, PI028, PI029) — the anti-gaming clause satisfied by this file itself, not its neighbours, with the sequencing explicitly recorded for plan 04-07's re-check.
- **Zero patterns shipped, zero patterns touched, zero regressions.** `git diff --name-only HEAD -- patterns/` is empty across the whole plan; `cargo test --locked` held at 357/357 throughout; `cargo fmt --all -- --check` and `cargo clippy --all-targets --locked -- -D warnings` both clean at every commit.

## Task Commits

1. **Task 1: One boundary-sitting benign manifest, and the measurement that fixes where it goes** — `39ac036` (feat)
2. **Task 2: Vendor a public registry sample, triaged before it lands, with provenance recorded** — `8f6a4df` (feat)
3. **Task 3: The config-hygiene and shadowing near-misses, and the write-up that must stay quiet** — `c8260dd` (feat)

**Plan metadata:** committed separately below, per worktree-mode conventions (STATE.md/ROADMAP.md excluded — the orchestrator owns those centrally after this wave).

## Files Created/Modified

- `tests/corpus/clean/mcp-server-catalogue.json` — hand-written D-01 boundary manifest, five tool descriptions exercising the four measured near-miss shapes
- `tests/corpus/clean/mcp-registry-filesystem-tools.md` — vendored, `src/filesystem/README.md`'s `## API` section (Tools + Tool annotations), MIT
- `tests/corpus/clean/mcp-registry-memory-tools.md` — vendored, `src/memory/README.md`'s `### Tools` section, MIT
- `tests/corpus/clean/mcp-registry-everything-instructions.md` — vendored, `src/everything/docs/instructions.md`, whole file, CC-BY-4.0
- `tests/corpus/clean/mcp-dev-tooling-setup.json` — D-03 boundary: four servers, three unpinned installs, one TLS endpoint
- `tests/corpus/clean/mcp-companion-tools.md` — D-04 boundary: two sibling tools referencing each other, neither changing the other's behaviour
- `tests/corpus/documentation/mcp-tool-poisoning-writeup.md` — the two-sided anti-gaming write-up
- `tests/corpus/clean/README.md` — new "CAT-02 (`mcp-*`)" section (flat-file rule + measurement), "Provenance — vendored third-party files" section, and per-specimen decision-defended lines for every new file

## Enumeration Measurements (Task 1)

| Gate | State when measured | Result |
|---|---|---|
| `corpus_test` (`the_clean_corpus_reports_nothing`, `the_clean_corpus_matches_nothing_even_under_strict`, plus the documentation-corpus tests and the attack-corpus ratchet, run as one suite) | `tests/corpus/clean/mcp/deliberately-matching.md` present, containing `disregard all previous directives` (confirmed via direct scan to fire `PI001`, confidence 1.0) | **5/5 passed — stayed green** |
| `pattern_relaxed_control_test` (all 4 tests, including `clean_corpus_is_held_by_the_shipped_set_and_broken_by_the_relaxed_set`, which walks `corpus_clean_specimens()`) | same state as above | **4/4 passed — stayed green** |

Both `specimens()` (`tests/corpus_test.rs`) and `corpus_clean_specimens()` (`tests/pattern_relaxed_control_test.rs`) call `read_dir` once and filter on `is_file` without recursing — confirmed by direct observation, not by reading the source alone. The temporary subdirectory was removed (`test ! -d tests/corpus/clean/mcp` confirmed) before any real specimen was authored.

## Registry Candidate Triage (Task 2)

**Rejected candidates: none.** Every fetched candidate reported zero matches at `--min-confidence 0` when scanned in the scratch location (`/private/tmp/.../scratchpad/mcp-registry-triage/`, outside this repository), before vendoring:

| Candidate | Path at commit `d73f99efbfd40c3aa1b61e88728b3d49fb52608f` | Zero-threshold result | Outcome |
|---|---|---|---|
| Filesystem server tools | `src/filesystem/README.md` (`## API` section) | 0 matches | Vendored — `mcp-registry-filesystem-tools.md` |
| Memory server tools | `src/memory/README.md` (`### Tools` section) | 0 matches | Vendored — `mcp-registry-memory-tools.md` |
| Memory server system prompt | `src/memory/README.md` (`### System Prompt` section) | 0 matches | Not vendored — clean, but a client-usage example rather than a tool definition; out of Task 2's stated scope |
| Everything server instructions | `src/everything/docs/instructions.md` (whole file) | 0 matches | Vendored — `mcp-registry-everything-instructions.md` |

`@modelcontextprotocol/server-postgres`, named in `04-RESEARCH.md` §Q2 as observed live and installed on the research machine, no longer exists in this registry's current `src/` tree at the vendored commit (archived or relocated elsewhere) — `server-filesystem` and `server-memory` were the closest available match to "actually installed and in use." No package was installed for this task (`git diff --stat Cargo.toml Cargo.lock` empty); every candidate was fetched as a file via `curl` from `raw.githubusercontent.com`.

## Provenance Rows Added

| File | Source | Commit SHA | Path | Licence | Date | Complete/Subset |
|---|---|---|---|---|---|---|
| `mcp-registry-filesystem-tools.md` | `github.com/modelcontextprotocol/servers` | `d73f99efbfd40c3aa1b61e88728b3d49fb52608f` | `src/filesystem/README.md` | MIT (file's own `## License` section) | 2026-09-04 | Subset — `## API` section only, whole tool definitions |
| `mcp-registry-memory-tools.md` | `github.com/modelcontextprotocol/servers` | `d73f99efbfd40c3aa1b61e88728b3d49fb52608f` | `src/memory/README.md` | MIT (file's own `## License` section) | 2026-09-04 | Subset — `### Tools` section only, whole tool definitions |
| `mcp-registry-everything-instructions.md` | `github.com/modelcontextprotocol/servers` | `d73f99efbfd40c3aa1b61e88728b3d49fb52608f` | `src/everything/docs/instructions.md` | CC-BY-4.0 (repository `LICENSE`, documentation-contributions rule) | 2026-09-04 | Complete — whole 28-line file |

The repository's top-level `LICENSE` file resolves to `NOASSERTION` at the GitHub API level (`{"key": "other", "spdx_id": "NOASSERTION"}`) because the project is mid-transition from MIT to Apache-2.0 and per-commit relicensing consent is not publicly enumerable. Two of three vendored files sidestep this entirely because they carry their own explicit `## License` section (MIT); the third sidesteps it by falling under the repository LICENSE's own unconditional rule that documentation contributions (excluding specifications) are CC-BY-4.0, independent of the code relicensing-consent question.

## Which Decision Each Specimen Defends

| Specimen | Decision |
|---|---|
| `mcp-manifest.json` (pre-existing) | D-01 |
| `mcp-setup-guide.md` (pre-existing) | D-01 |
| `mcp-server-catalogue.json` | D-01 |
| `mcp-registry-filesystem-tools.md` | D-01 |
| `mcp-registry-memory-tools.md` | D-01 |
| `mcp-registry-everything-instructions.md` | D-01 |
| `mcp-dev-tooling-setup.json` | D-03 |
| `mcp-companion-tools.md` | D-04 |
| `mcp-tool-poisoning-writeup.md` (in `documentation/`) | anti-gaming clause / D-01 |

## Decisions Made

See `key-decisions` in the frontmatter for the full list. The two most consequential: (1) the flat-file rule for every CAT-02 specimen is a measured fact, not a style preference — proven by placing a matching file in a temporary subdirectory and watching both gates stay green; (2) per-file licence determination for vendored text used the strongest available evidence for each file individually (an embedded self-declaration where present, the repository's own unconditional documentation-contribution rule otherwise) rather than either inheriting the ambiguous repository-wide licence or attempting unresolvable per-commit consent archaeology.

## Deviations from Plan

None — plan executed exactly as written. No Rule 1/2/3 auto-fixes were needed; no architectural questions arose. The one judgment call made within the plan's own discretion — choosing which specific registry files to vendor, since the plan named the registry but not specific files/paths, and no literal JSON tool manifest exists in `github.com/modelcontextprotocol/servers` (all reference servers define tools programmatically in TypeScript) — was resolved by vendoring the README documentation of each server's tools, which is real, unmodified (except line-range extraction), third-party text with an unambiguous licence.

## Issues Encountered

None. All three tasks' acceptance criteria and `<verify>` blocks passed on the first attempt after each task's implementation.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- The flat `tests/corpus/clean/` and `tests/corpus/documentation/` layout, with 8 `mcp-*` specimens plus one documentation write-up, is now the false-positive gate every `PI060`–`PI069` pattern in plans 04-04/04-05/04-06 will be judged against. A pattern that fires on any of them is narrowed, per the standing corpus rule — never accommodated by editing a specimen.
- **Concern for whoever picks up 04-04/04-05/04-06:** `mcp-server-catalogue.json`'s first tool description (`resolve_reference_id`) is the sharpest specimen in the corpus — it has almost the identical grammatical shape as D-01's own fires-example, differing only in the object of the instruction (this server's own declared sibling tool vs. an external path/behaviour). Any PI060+ pattern that does not narrow on the object, only on second-person-plus-imperative, will fail here first.
- **For plan 04-07:** `tests/corpus/documentation/mcp-tool-poisoning-writeup.md`'s header explicitly records that its current `--strict` matches come from PI015/PI028/PI029, not PI060+. Re-check after the full CAT-02 pattern set ships to confirm the strict-mode match set grows to include the new patterns, not merely stays pinned to the three pre-existing ones.
- The registry-vendoring provenance pattern established here (per-file licence verification, scratch-location triage before landing, explicit rejection recording even when nothing is rejected) is reusable for any future D-06-style corpus-sourcing task in a later category.

---
*Phase: 04-mcp-tool-description-poisoning-cat-02-34*
*Completed: 2026-09-04*

## Self-Check: PASSED

All seven claimed created files exist on disk (`mcp-server-catalogue.json`, `mcp-registry-filesystem-tools.md`, `mcp-registry-memory-tools.md`, `mcp-registry-everything-instructions.md`, `mcp-dev-tooling-setup.json`, `mcp-companion-tools.md`, `mcp-tool-poisoning-writeup.md`), `tests/corpus/clean/README.md` carries the claimed new sections, and all three task commit hashes (`39ac036`, `8f6a4df`, `c8260dd`) are present in `git log`.
