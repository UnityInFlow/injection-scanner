# Roadmap: injection-scanner

## Phase 1: Core Scanner (Week 5)
**Goal:** Pattern matching engine with YAML loader, severity classifier, and basic CLI.
**Scope:** SCAN-01 through SCAN-04, CLI-01 through CLI-03, CLI-05
**Deliverables:**
- Pattern matching engine (regex-based)
- YAML pattern loader with validation
- 30+ patterns across 5 categories
- Severity classification (CRITICAL/HIGH/MEDIUM/LOW)
- Remediation hints
- File scanner + stdin mode
- JSON output
- Inline allowlist suppression
- Tests for all pattern categories

## Phase 2: Pre-commit Hook + Release (Week 6)
**Goal:** Pre-commit hook, SARIF output, cross-compilation, release.
**Scope:** CLI-04, HOOK-01, PERF-01, DIST-01, DOCS-01, REL-01
**Deliverables:**
- SARIF output format (GitHub Advanced Security)
- `install-hook` command
- Performance optimization (<200ms)
- Cross-compilation CI (5 targets)
- Pre-built binaries on GitHub Releases
- SHA256 checksums
- PATTERNS.md contribution guide
- README, CONTRIBUTING, LICENSE

---
*Last updated: 2026-04-02*
