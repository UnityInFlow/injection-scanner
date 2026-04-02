# injection-scanner

## What This Is
A Rust CLI tool that statically scans files for prompt injection patterns — role overrides, instruction injection, data exfiltration, jailbreaks, and encoding attacks. Runs as a pre-commit hook in <200ms. Tool #03 in the UnityInFlow ecosystem.

## Core Value
Catch prompt injection attacks before they reach production — in skill files, CLAUDE.md, RAG documents, and user inputs.

## Requirements

### Active
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

### Out of Scope
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
- Self-hosted CI runners (arc-runner-unityinflow + orangepi)

## Key Decisions
| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust over TypeScript | <200ms performance requirement for pre-commit hook | — Pending |
| YAML patterns over embedded | Community contributions via PR, no recompilation needed | — Pending |
| SARIF output | GitHub Advanced Security integration, industry standard | — Pending |
| regex over Aho-Corasick | Simpler for v0.0.1, Aho-Corasick for v0.1.0 if perf needed | — Pending |

---
*Last updated: 2026-04-02*
