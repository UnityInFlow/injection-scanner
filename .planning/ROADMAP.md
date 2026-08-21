# Roadmap: injection-scanner

**Milestone:** Production Readiness — v0.0.3 + v0.1.0
**Opened:** 2026-08-21 · **Supersedes:** the April 2026 Phase 1/2 roadmap (`.planning/archive/milestone-v0.0.1/`)
**Basis:** `docs/AUDIT-2026-08.md`, independently verified (`scratchpad/opencode-verification-report.md`)

## Why this milestone exists

v0.0.2 ships and is consumed downstream by `spec-ci-plugin`, but an audit found the tool does not do
what its README claims: sentence-case payloads are missed, one binary file aborts a scan, `--format
sarif` returns text, the scanner flags its own documentation, and CI has been dead since 2026-06-24.
This milestone makes every existing claim true, then closes the three requirements that were scoped
to v0.0.1 and never built.

**Definition of done:** a stranger can install injection-scanner, run it on a real repository, get
trustworthy findings, wire it into GitHub code scanning, and contribute a pattern via a fork PR that
receives CI.

---

## Phase 1: Restore the Gate

**Goal:** a working, trustworthy CI signal on every pull request, including from forks.
**Scope:** CI-01, CI-02
**Blocks:** everything. No other phase merges until Phase 1 is green.

**Deliverables**
- `ci.yml` rewritten as the D-02 split: fmt + clippy + test on a GitHub-hosted runner, secretless, `contents: read`
- `arc-runner-unityinflow` removed from all matrices (it matches zero registered runners)
- `release.yml` confirmed self-hosted on `[orangepi]`, triggered only by tag push / `release: published`
- A fork PR demonstrably receiving CI
- Decision row recorded in the root CLAUDE.md decisions log

**Success criteria**
- A pull request from a fork runs fmt, clippy and test to completion in under 10 minutes
- No self-hosted job is reachable from any fork-firable trigger
- No secret is exposed to a fork-triggered workflow

---

## Phase 2: Correctness — ship v0.0.3

**Goal:** every claim already in the README becomes true. No new features.
**Scope:** FIX-01 … FIX-06, INT-01, SCAN-08, PERF-01
**Depends on:** Phase 1

**Deliverables**
- Case-insensitive matching by default, with a `case_sensitive` opt-out field (FIX-01)
- `Scanner` struct owning compiled patterns — compiled once, not per file (FIX-02)
- Per-file error isolation so one non-UTF-8 file cannot abort a scan (FIX-03)
- Suppression: `ignore` / `ignore-next-line` / `ignore-file`, README corrected (FIX-04)
- `find_iter` with a per-line cap (FIX-05)
- `--format` as a clap `ValueEnum` (FIX-06)
- Pattern validation: duplicate-ID detection, `deny_unknown_fields`, `--strict-patterns` (SCAN-08)
- `spec-ci-plugin` consumer fixes: SHA256 verification, version-keyed cache, reconciled defaults (INT-01)
- Release-time smoke test asserting the musl asset contract

**Success criteria**
- `Ignore all previous instructions` is detected
- 500-file scan completes under 200ms (PERF-01, measured by benchmark not assertion)
- A binary file in the tree does not abort the run
- `--format bogus` exits non-zero with a usage error
- Both README suppression forms work as documented

---

## Phase 3: Signal Quality

**Goal:** findings you can trust — usable on documentation, resistant to trivial evasion.
**Scope:** QUAL-01, QUAL-02, QUAL-03, SCAN-05, SCAN-06, CLI-09, CLI-10
**Depends on:** Phase 2

**Deliverables**
- Markdown context classifier + `confidence` on every finding (QUAL-01, engine E7)
- Severity rebalanced across the full CRITICAL/HIGH/MEDIUM/LOW range, criteria written into `PATTERNS.md` (QUAL-02)
- False-positive corpus in CI — clean documents must stay at zero findings (QUAL-03)
- Unicode normalization pass (SCAN-05, engine E1)
- Multi-line sliding-window pass (SCAN-06, engine E5)
- Directory walking via the `ignore` crate: `.gitignore`, excludes, size caps, symlink cycles, parallelism (CLI-09)
- Broader file-type coverage including `.mdx`, `.json`, `.html`, `.cursorrules`, extensionless agent files (CLI-10)

**Success criteria**
- `injection-scanner check README.md` and `check docs/` return zero findings
- `examples/*-attack.md` still return their full expected counts
- Homoglyph, spacing, separator and newline-split variants of PI001 are all detected
- A symlink cycle produces exactly one finding per real file
- At least one MEDIUM and one LOW pattern exist and fire

---

## Phase 4: Integration — ship v0.1.0

**Goal:** close the three unmet original requirements and make the tool adoptable.
**Scope:** CLI-04, CLI-06, CLI-07, CLI-08, HOOK-01, PERF-02, SCAN-07, TEST-01, TEST-02, DOCS-02
**Depends on:** Phase 3

**Deliverables**
- SARIF 2.1.0 output (CLI-04) — added to the `--format` enum only now
- `install-hook` + `.pre-commit-hooks.yaml` (HOOK-01)
- Aho-Corasick prefilter (PERF-02, engine E6)
- `--fail-on`, `--quiet`, exit code 2 for warnings-only (CLI-06)
- `rules` and `explain <PI0XX>` subcommands (CLI-07)
- `--baseline` for incremental adoption on existing repos (CLI-08)
- Pattern gap-fill: `PI008-009`, `PI015-019`, `PI026-029`, `PI039`, `PI043-049` (SCAN-07, ~18 patterns)
- Coverage gate and criterion benchmarks wired into CI (TEST-01, TEST-02)
- Severity-grading criteria documented for contributors (DOCS-02)

**Success criteria**
- SARIF output validates against the 2.1.0 schema and uploads to GitHub code scanning
- `install-hook` completes a real commit in under 200ms
- Pattern library at ~48 patterns with the perf budget still met
- Coverage measured and above 80% on core logic
- False-positive rate measured against the corpus, not asserted

---

## Out of scope for this milestone

Deferred to v0.2.0 and beyond, tracked in `docs/DETECTION-BACKLOG.md`:
agentic categories (`PI050`–`PI079`), the recursive decoder (E2), invisible-character heuristics (E3),
structural frontmatter analysis (E4), runtime filter mode, crates.io publish, Homebrew, GitHub Action,
multilingual and delimiter categories, and any semantic/LLM detection.

---
*Last updated: 2026-08-21*
