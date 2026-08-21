# Requirements: injection-scanner

**Milestone:** Production Readiness — v0.0.3 + v0.1.0
**Defined:** 2026-08-21 · **Core value:** Catch prompt injection attacks before they reach production
**Prior milestone:** `.planning/archive/milestone-v0.0.1/REQUIREMENTS.md`

> **Reconciliation note.** The archived requirements file grouped all 14 items under a "v0.0.1
> Requirements" heading while its own traceability table assigned CLI-04, HOOK-01 and PERF-01 to
> Phase 2 — and this repo's `CLAUDE.md` lists all three as v0.0.1 acceptance criteria. Three
> documents disagreed about what v0.0.1 contained. Resolved here: those three were **never delivered**
> and are carried into this milestone as Phase 4 (CLI-04, HOOK-01) and Phase 2 (PERF-01).

---

## Carried forward — delivered in v0.0.1/v0.0.2

| ID | Requirement | Status |
|---|---|---|
| SCAN-01 | 30+ patterns across 5 categories | ✅ 30 |
| SCAN-02 | YAML pattern loader (embedded + external) | ✅ |
| SCAN-03 | Severity classifier CRITICAL/HIGH/MEDIUM/LOW | ⚠️ range unused — see QUAL-02 |
| SCAN-04 | Remediation hints per pattern | ✅ |
| CLI-01 | File scanner | ⚠️ narrow — see CLI-09, CLI-10 |
| CLI-02 | Stdin mode | ✅ |
| CLI-03 | JSON output | ✅ |
| CLI-05 | Inline allowlist suppression | ⚠️ broken — see FIX-04 |
| DIST-01 | Pre-built binaries | ✅ 6 targets at v0.0.2 |
| DOCS-01 | PATTERNS.md contribution guide | ✅ |
| REL-01 | GitHub Release with checksums | ✅ |

---

## Phase 1 — Restore the Gate

- [ ] **CI-01**: Public/fork CI runs fmt + clippy + test to completion on a pull request from a fork, secretless, with `contents: read`
- [ ] **CI-02**: All secret-bearing and self-hosted work is reachable only from triggers a fork cannot fire (tag push / `release: published`)

## Phase 2 — Correctness (v0.0.3)

- [ ] **FIX-01**: Patterns match case-insensitively by default; `case_sensitive: true` available as a per-pattern opt-out
- [ ] **FIX-02**: The pattern set is compiled once per run, not once per file
- [ ] **FIX-03**: An unreadable or non-UTF-8 file is skipped and reported, never aborts the scan
- [ ] **FIX-04**: Suppression supports `ignore` (same line), `ignore-next-line`, and `ignore-file`; the README matches the implementation; the ID pattern accepts non-`PI` prefixes
- [ ] **FIX-05**: All matches per line are reported, bounded by a per-line cap
- [ ] **FIX-06**: `--format` rejects unknown values at parse time rather than falling through to text
- [ ] **SCAN-08**: Duplicate pattern IDs are detected; unknown YAML fields are rejected; `--strict-patterns` fails on an invalid pattern instead of warning
- [ ] **INT-01**: The `spec-ci-plugin` consumer verifies SHA256 before executing, keys its cache by version, and has a single source of truth for the default version; a release-time smoke test asserts the musl asset contract
- [ ] **PERF-01**: A 500-file scan completes in under 200ms, proven by a benchmark

## Phase 3 — Signal Quality

- [ ] **QUAL-01**: Every finding carries a lexical `context` and a `confidence`; findings inside fenced code and inline code are downgraded by default, restored by `--strict`
- [ ] **QUAL-02**: Severity is rebalanced across the full range; at least one MEDIUM and one LOW pattern exist; grading criteria are documented
- [ ] **QUAL-03**: A false-positive corpus of clean real-world documents returns zero findings in CI, and `examples/*-attack.md` return their full expected counts
- [ ] **SCAN-05**: A normalization pass defeats homoglyph, spacing, separator, fullwidth and zero-width-interleave evasion
- [ ] **SCAN-06**: A multi-line sliding window detects payloads split across a newline
- [ ] **CLI-09**: Directory walking respects `.gitignore`, supports `--exclude`/`--include`, caps file size, terminates cleanly on symlink cycles with no duplicate findings, and runs in parallel
- [ ] **CLI-10**: Default file coverage includes `.mdx`, `.json`, `.jsonl`, `.rst`, `.html`, `.csv`, `.mdc`, `.cursorrules` and extensionless agent files

## Phase 4 — Integration (v0.1.0)

- [ ] **CLI-04**: SARIF 2.1.0 output that validates against the schema and uploads to GitHub code scanning
- [ ] **CLI-06**: `--fail-on <severity>`, `--quiet`, and exit code 2 for warnings-only
- [ ] **CLI-07**: `rules` and `explain <PI0XX>` subcommands
- [ ] **CLI-08**: `--baseline <file>` for incremental adoption on an existing repository
- [ ] **HOOK-01**: `install-hook` installs a working pre-commit hook that completes a real commit in under 200ms; `.pre-commit-hooks.yaml` supports the pre-commit framework
- [ ] **PERF-02**: Aho-Corasick prefilter keeps the perf budget with a growing library
- [ ] **SCAN-07**: Reserved ID gaps filled — `PI008-009`, `PI015-019`, `PI026-029`, `PI039`, `PI043-049`
- [ ] **TEST-01**: Test coverage measured in CI with a gate above 80% on core logic
- [ ] **TEST-02**: Criterion benchmarks defend PERF-01 and fail CI on regression
- [ ] **DOCS-02**: Severity-grading criteria and the per-pattern test-case policy are documented and enforced in CI

---

## Out of Scope — this milestone

| Feature | Reason |
|---|---|
| Agentic categories `PI050`–`PI079` | v0.2.0 — depend on the frontmatter engine (E4) |
| Recursive decoder (E2) | v0.2.0 — supersedes issues #6 and #7 |
| Invisible-character heuristic (E3) | v0.2.0 — needs a real MEDIUM/LOW range first (QUAL-02) |
| Multilingual, delimiter, output-hijack categories | v0.3.0 |
| Runtime filter mode | v0.2.0 — for agent-sandbox |
| crates.io, Homebrew, binstall, GitHub Action | v0.3.0 |
| LLM/semantic detection | v1.0.0 — regex plus normalization is sufficient for known patterns |
| Auto-fix | Never — dangerous for a security tool. Flag only. |

---

## Traceability

| Requirement | Phase | Issues | Status |
|---|---|---|---|
| CI-01, CI-02 | 1 | #17 | Pending |
| FIX-01 | 2 | #12 | Pending |
| FIX-02 | 2 | #13 | Pending |
| FIX-03 | 2 | #14 | Pending |
| FIX-04 | 2 | #15 | Pending |
| FIX-05 | 2 | #16 | Pending |
| FIX-06 | 2 | #42 | Pending |
| SCAN-08 | 2 | #28 | Pending |
| INT-01 | 2 | #18, #43 | Pending |
| PERF-01 | 2 | #13, #29 | Pending |
| QUAL-01 | 3 | #20 | Pending |
| QUAL-02 | 3 | #21 | Pending |
| QUAL-03 | 3 | #29 | Pending |
| SCAN-05 | 3 | #26 | Pending |
| SCAN-06 | 3 | #24 | Pending |
| CLI-09 | 3 | #22 | Pending |
| CLI-10 | 3 | #23 | Pending |
| CLI-04 | 4 | #5 | Pending |
| CLI-06, CLI-07, CLI-08 | 4 | #25 | Pending |
| HOOK-01 | 4 | #8 | Pending |
| PERF-02 | 4 | #4 | Pending |
| SCAN-07 | 4 | #27 | Pending |
| TEST-01, TEST-02 | 4 | #29 | Pending |
| DOCS-02 | 4 | #21, #27 | Pending |

**Coverage:** 24 requirements, all mapped to a phase and at least one issue.

---
*Last updated: 2026-08-21*
