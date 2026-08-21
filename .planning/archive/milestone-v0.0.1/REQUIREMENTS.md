# Requirements: injection-scanner

**Defined:** 2026-04-02
**Core Value:** Catch prompt injection attacks before they reach production

## v0.0.1 Requirements

### Scanning
- [ ] **SCAN-01**: 30+ patterns across 5 categories
- [ ] **SCAN-02**: YAML pattern loader
- [ ] **SCAN-03**: Severity classifier (CRITICAL/HIGH/MEDIUM/LOW)
- [ ] **SCAN-04**: Remediation hints per pattern

### CLI
- [ ] **CLI-01**: File scanner (text/markdown/YAML)
- [ ] **CLI-02**: Stdin mode
- [ ] **CLI-03**: JSON output
- [ ] **CLI-04**: SARIF output
- [ ] **CLI-05**: Inline allowlist suppression

### Pre-commit Hook
- [ ] **HOOK-01**: install-hook command
- [ ] **PERF-01**: <200ms for typical project

### Distribution
- [ ] **DIST-01**: Pre-built binaries (5 targets)
- [ ] **DOCS-01**: PATTERNS.md contribution guide
- [ ] **REL-01**: GitHub Release with checksums

## v0.1.0 Requirements
- **RT-01**: Runtime filter mode (stdin → filtered stdout)
- **PERF-02**: Aho-Corasick multi-pattern matching
- **DIST-02**: Homebrew formula
- **INT-01**: spec-ci-plugin integration

## Out of Scope
| Feature | Reason |
|---------|--------|
| LLM-based detection | v1.0.0 — regex sufficient for known patterns |
| Custom severity overrides | v0.1.0 |
| Auto-fix | Dangerous for security tool — flag only |

## Traceability
| Requirement | Phase | Status |
|-------------|-------|--------|
| SCAN-01 | Phase 1 | Pending |
| SCAN-02 | Phase 1 | Pending |
| SCAN-03 | Phase 1 | Pending |
| SCAN-04 | Phase 1 | Pending |
| CLI-01 | Phase 1 | Pending |
| CLI-02 | Phase 1 | Pending |
| CLI-03 | Phase 1 | Pending |
| CLI-04 | Phase 2 | Pending |
| CLI-05 | Phase 1 | Pending |
| HOOK-01 | Phase 2 | Pending |
| PERF-01 | Phase 2 | Pending |
| DIST-01 | Phase 2 | Pending |
| DOCS-01 | Phase 2 | Pending |
| REL-01 | Phase 2 | Pending |

**Coverage:** 14 requirements, all mapped.

---
*Last updated: 2026-04-02*
