# injection-scanner

## What This Is
A Rust CLI tool that statically scans files for prompt injection patterns — role overrides, instruction injection, data exfiltration, jailbreaks, and encoding attacks. Runs as a pre-commit hook in <200ms. Tool #03 in the UnityInFlow ecosystem.

## Core Value
Catch prompt injection attacks before they reach production — in skill files, CLAUDE.md, RAG documents, and user inputs.

## Requirements

> Requirements live in each milestone's archived `REQUIREMENTS.md` under `.planning/archive/`.
> There is no active milestone: maintenance-only since 2026-10-23. The list further below is
> the **original v0.0.1 set, retained for history**.

### Validated — shipped and confirmed

- ✓ **Production Readiness (v0.0.3 + v0.1.0)** — shipped 2026-08-29. Recall **10/60 → 56/60**
  on a corpus written from the threat model. CI restored, SARIF output, `install-hook`,
  `--baseline`, `--fail-on`, Unicode normalization, multi-line matching, 48 patterns, SLSA
  provenance over six target-triple binaries. `.planning/archive/milestone-v0.1.0/`.
- ✓ **v0.2.0 — Agent-shaped attacks** — released 2026-10-10. The scanner reads agent
  configuration, not just prose: structural frontmatter engine, recursive decoder, and three
  agentic categories (tool & permission abuse, MCP tool-description poisoning, persistence &
  lifecycle hijack). 48 → 79 patterns. CAT-03 held-out recall **2/12** is the published number;
  development recall 103/115. `.planning/archive/milestone-v0.2.0/`.

### Active

None — maintenance-only. Since release: #197 added `PI114`-`PI118` (84 patterns, development
recall 105/115). Close-out gaps (#164-#184) wait on the unplanned GitHub milestone v0.3.0.

### Original v0.0.1 set (historical)
- [ ] **SCAN-01**: 30+ patterns across 5 categories (role override, instruction injection, exfiltration, jailbreaks, encoding)
- [ ] **SCAN-02**: YAML pattern loader from `patterns/` directory
- [ ] **SCAN-03**: Severity classifier (CRITICAL / HIGH / MEDIUM / LOW)
- [ ] **SCAN-04**: Remediation hints per pattern
- [ ] **CLI-01**: File scanner for text/markdown/YAML files
- [ ] **CLI-02**: Stdin mode (`cat file | injection-scanner check -`)
- [ ] **CLI-03**: JSON output mode
- [ ] **CLI-04**: SARIF output mode (GitHub Advanced Security)
- [ ] **CLI-05**: Allowlist (`# injection-scanner:ignore PI001` inline suppression)
- [ ] **HOOK-01**: `injection-scanner install-hook` installs pre-commit hook
- [ ] **PERF-01**: Hook runs in <200ms for typical project
- [ ] **DIST-01**: Pre-built binaries for macOS (arm64/x86), Linux (x86/arm64), Windows
- [ ] **DOCS-01**: Community pattern contribution guide (PATTERNS.md)
- [ ] **REL-01**: Published to GitHub Releases with SHA256 checksums

### Out of Scope (original set)
- Runtime filtering (v0.1.0 — for agent-sandbox integration)
- LLM-based semantic injection detection — v1.0.0
- Homebrew formula — v0.1.0 (manual binary install first)

## Context
- First Rust tool in the UnityInFlow ecosystem
- Feeds into: spec-ci-plugin (#04), skills-registry (#17), agent-sandbox (#14), kore-runtime (#08)
- No existing OSS prompt injection scanner with maintained pattern library
- OpenClaw has 5,700+ community skills with zero security scanning
- Superpowers has 129k stars of community skills — attack surface is real

## Constraints
- Rust stable, edition 2021
- clap (derive) for CLI, serde + serde_yaml for patterns, regex for matching
- anyhow for binary error handling, thiserror for library errors
- cargo fmt + cargo clippy -- -D warnings before every commit
- No unwrap() in production code
- CI and release run on GitHub-hosted runners (public repo; the org runner group refuses self-hosted jobs on it — see CLAUDE.md)

## Key Decisions
| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust over TypeScript | <200ms performance requirement for pre-commit hook | ✅ Held — matching is fast; the perf problem was per-file regex compilation, not the language |
| YAML patterns over embedded | Community contributions via PR, no recompilation needed | ✅ Held — embedded core + external overlay works |
| SARIF output | GitHub Advanced Security integration, industry standard | ✅ Shipped in v0.1.0 |
| regex over Aho-Corasick | Simpler for v0.0.1, Aho-Corasick for v0.1.0 if perf needed | ✅ Held — compile-once mattered more; the PERF-01 budget moved 200 → 400 ms for the 79-pattern library (#188) |
| Public/fork CI on a GitHub-hosted runner (2026-08-21) | Org runner group blocks all self-hosted jobs on public repos; the private-window alternative denies CI to fork PRs and kills community pattern contributions. Scoped exception mirroring the spec-ci-plugin D-02 split. | ✅ Held — fork PRs (#197, #200) received CI and #197 merged |
| "Production ready" = v0.0.3 + v0.1.0 (2026-08-21) | Correctness and signal quality before differentiation. Agentic categories deferred to v0.2.0. | ✅ Shipped 2026-08-29 |
| v0.2.0: engines before categories, three categories not eight (2026-08-30) | #32 is a stated prerequisite for `PI050`-`PI069`; three categories kept each PR reviewable (GATE-04) | ✅ Shipped 2026-10-10 |
| Publish the held-out CAT-03 number, not the development one (2026-10-09) | A development corpus restated to the pattern authors overstates a detector; only a sealed blind set is independent | ✅ Held — 2/12 published beside 7/12 development |
| Scope freeze after v0.2.0 (2026-10) | Ecosystem focus moves to agent-memory v0.1.0 | ✅ Maintenance-only from 2026-10-23 |

---
*Last updated: 2026-10-23 after v0.2.0 milestone archive*
