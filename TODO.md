# TODO — injection-scanner

Actionable checklist derived from the 2026-08-21 deep-dive audit. Every item maps to a GitHub issue
and a milestone. Full reasoning lives in `docs/AUDIT-2026-08.md`, `docs/DETECTION-BACKLOG.md` and
`docs/ROADMAP-v0.1.0.md`.

**Current state:** v0.0.2 · 30 patterns · 5 categories · **93 tests** · CI green · **all v0.0.3 code merged to `main` (2026-08-22); only the tag remains**

> **Revised 2026-08-21** after an independent verification pass (`scratchpad/opencode-verification-report.md`).
> Every original claim reproduced. Two new defects added (#42, #43), CI promoted to a hard gate,
> and #16 downgraded. See the revision note at the end of `docs/AUDIT-2026-08.md`.

---

## Milestone v0.0.3 — Make it actually work

No new features. Every claim already in the README becomes true.

- [x] **#17** Restore CI — D-02 public/fork split; drop the zero-runner `arc-runner-unityinflow` matrix leg `P0` **← hard gate: nothing else in this milestone merges until this is green**
- [x] **#12** Case-insensitive matching by default + `case_sensitive` opt-out field `P0`
- [x] **#13** `Scanner` struct — compile the pattern set once, not once per file `P0`
- [x] **#14** Per-file error isolation so one non-UTF-8 file cannot abort the scan `P0`
- [x] **#15** Suppression: `ignore` / `ignore-next-line` / `ignore-file`; fix the README `P0`
- [x] **#16** `find_iter` with a per-line cap — report every match, not just the first `P2`
- [x] **#42** `--format` as a clap `ValueEnum` — `--format sarif` currently returns text, not an error `P0`
- [x] **#43** Fix the `spec-ci-plugin` consumer — unverified download, unversioned cache, version default mismatch `P0`
      *(spec-ci-plugin PR #9, CI green, awaiting merge — also closes #56)*
- [x] **#18** Release smoke test enforcing the `spec-ci-plugin` musl asset contract `P1`
      *(PR #59, CI green — plus a repo-level contract test and CONTRIBUTING docs)*
- [x] **#19** Replace `unwrap()` in `allowlist.rs`; deny `unwrap_used` in `src/` `P2` *good first issue*

**Exit criteria**
- `Ignore all previous instructions` (sentence case) is detected
- 500-file scan completes under 200ms
- A binary file in the tree does not abort the run
- `--format bogus` errors instead of silently printing text
- CI is green on a pull request from a fork

---

## Milestone v0.1.0 — Complete the original spec, halve the false positives

Closes the three unmet v0.0.1 requirements (CLI-04, HOOK-01, PERF-01) and makes the tool usable on
documentation.

### Correctness and usability
- [ ] **#20** Markdown context awareness + `confidence` — the scanner currently reports 13 findings on its own README `P1`
- [ ] **#21** Rebalance severity — there is not one MEDIUM or LOW pattern today `P1`
- [ ] **#22** Replace the hand-rolled walker with the `ignore` crate — excludes, `.gitignore`, size caps, parallelism `P1`
- [ ] **#23** Broaden scanned file types — `.mdx`, `.json`, `.html`, `.cursorrules`, extensionless skill files `P1`
- [ ] **#24** Multi-line window matching — a newline currently defeats every pattern `P1`
- [ ] **#25** CLI surface — `--fail-on`, `--quiet`, exit 2, `rules`, `explain`, `--baseline` `P1`
- [ ] **#26** Unicode normalization pass — defeats homoglyph, spacing and separator evasion `P1`
- [ ] **#28** Pattern validation — invalid regexes are silently dropped; duplicate IDs undetected `P1`

### Requirements being closed
- [ ] **#5** SARIF 2.1.0 output — CLI-04 `P1`
- [ ] **#8** `install-hook` + `.pre-commit-hooks.yaml` — HOOK-01 `P1`
- [ ] **#4** Aho-Corasick prefilter — land *after* #13 `P1`
- [~] **#29** Measure what the docs assert: coverage gate, criterion benchmarks, false-positive corpus, fuzzing `P1`
      *Benchmarks + perf regression guard done (PR #58). Coverage now measurable and honest (PR #61
      fixed `cli_test.rs` running a stale binary): 72.18% total, ~78.8% on core logic — under the
      documented 80%, all of the gap in `load_external_patterns`. Coverage gate, FP corpus
      (QUAL-03, Phase 3) and fuzzing still open.*

### Patterns
- [ ] **#27** Fill the reserved ID gaps — `PI008-009`, `PI015-019`, `PI026-029`, `PI039`, `PI043-049` (~18 patterns) `P1` *good first issue*

**Exit criteria**
- `injection-scanner check README.md` and `check docs/` return zero findings
- `examples/*-attack.md` still return their full expected counts
- SARIF output validates against the 2.1.0 schema
- `install-hook` completes a real commit in under 200ms
- False-positive rate is measured against a corpus, not asserted

---

## Milestone v0.2.0 — Agentic attack surface

What an *agent* can be made to do, not just what a chatbot can be made to say.

- [ ] **#32** Structural frontmatter analysis — real YAML/TOML/JSON parsing `P2`
- [ ] **#33** `PI050-PI059` tool and permission abuse `P2`
- [ ] **#34** `PI060-PI069` MCP and tool-description poisoning `P2`
- [ ] **#35** `PI070-PI079` persistence and lifecycle hijack `P2`
- [ ] **#30** Recursive decoder — base64, hex, URL, HTML entities, `\u` escapes (supersedes #6 and #7) `P2`
- [ ] **#31** Invisible-character density heuristic `P2`
- [ ] **#11** Runtime filter mode for `agent-sandbox` `P2`
- [ ] **#6** HTML entity decoding — close when #30 lands
- [ ] **#7** Base64 detection — close when #30 lands

---

## Milestone v0.3.0 — Reach

- [ ] **#36** `PI080-PI089` indirect / RAG-borne injection `P2`
- [ ] **#37** `PI090-PI099` credential harvesting instructions `P2`
- [ ] **#38** `PI100-PI109` output-format hijack `P2`
- [ ] **#39** `PI110-PI119` multilingual evasion `P2` *help wanted*
- [ ] **#40** `PI120-PI129` delimiter and context-boundary spoofing `P2`
- [ ] **#41** Distribution — crates.io, library/CLI split, binstall, `.pre-commit-hooks.yaml`, GitHub Action `P2`
- [ ] **#10** Homebrew formula `P2`

---

## Housekeeping

- [ ] **#9** Cross-compilation CI — appears delivered by v0.0.2 (6 targets + SHA256SUMS); confirm the asset list and close
- [ ] Reconcile `.planning/REQUIREMENTS.md` and `.planning/STATE.md` with reality — Phase 1 is recorded COMPLETE while CLI-04, HOOK-01 and PERF-01 are unmet
- [ ] Add the severity-grading criteria to `PATTERNS.md` so community contributions grade themselves
- [ ] Enforce the existing `PATTERNS.md` policy (≥3 true positives, ≥2 near-miss negatives per pattern) in CI
- [ ] Record a decision-log row for the CI policy fix, mirroring the `prompt-vc` entry in the root CLAUDE.md

---

## Pattern library trajectory

| Milestone | Patterns | Categories | Detection engines |
|---|---|---|---|
| v0.0.2 (today) | 30 | 5 | 1 (line regex) |
| v0.1.0 | ~48 | 5 | 4 (+ normalization, windows, Aho-Corasick) |
| v0.2.0 | ~78 | 8 | 7 (+ frontmatter, decoder, heuristic) |
| v0.3.0 | ~128 | 13 | 7 |

The original spec's month-3 target was 75 patterns.

---

*Generated 2026-08-21 from `docs/AUDIT-2026-08.md`.*
