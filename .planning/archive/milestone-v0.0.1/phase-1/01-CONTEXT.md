# Phase 1 Context: Core Scanner

**Phase:** 1
**Date:** 2026-04-02
**Status:** Ready for planning

## Decisions

### SCAN-01: Pattern matching engine — regex for v0.0.1
Use `regex` crate for pattern matching. 30 patterns on typical spec files (<50kb) runs in <10ms. Aho-Corasick planned for v0.1.0 when pattern count grows beyond 75+. Create a GitHub issue for the upgrade when shipping v0.0.1.

### SCAN-02: Pattern loading — embedded core + optional external directory
Core patterns are embedded at compile time via `include_str!`. Single binary works standalone with zero file dependencies. If a `patterns/` directory exists next to the binary (or at a configured path), those patterns are loaded too — extending or overriding the built-in set. Community contributions go in external YAML files.

### SCAN-03: Encoding/obfuscation scope — Unicode + zero-width only in v0.0.1
Category E (encoding attacks) ships with Unicode direction overrides (U+202E, U+202D, U+200F) and zero-width character detection (U+200B, U+FEFF, U+200C, U+200D) only. HTML entity decoding and base64 detection deferred to v0.1.0. These are byte-scanning patterns, no decoding logic needed.

### CLI-04: SARIF output — deferred to Phase 2
Phase 1 ships text + JSON output only. SARIF is added in Phase 2 alongside the pre-commit hook and CI integration — they're both "CI/integration" features that belong together.

### SCAN-04: Severity assignment — category default + per-pattern override
Each category defines a default severity (e.g. role_override = CRITICAL). Individual patterns can override with their own severity. YAML stays clean — most patterns inherit from category, exceptions are explicit.

```yaml
category: role_override
default_severity: CRITICAL
patterns:
  - id: PI001
    pattern: "ignore previous instructions"
    # inherits CRITICAL from category
  - id: PI002
    pattern: "you are now"
    severity: HIGH  # override
```

## Deferred Ideas

- Aho-Corasick multi-pattern matching — v0.1.0 GitHub issue
- HTML entity decoding detection — v0.1.0
- Base64-encoded instruction detection — v0.1.0
- SARIF output format — Phase 2
- Runtime filter mode — v0.1.0 (for agent-sandbox integration)

## Downstream Notes

- **Planner:** Pattern engine needs two sources: embedded (compile-time) + external (runtime YAML loader)
- **Planner:** Only text + JSON output in Phase 1, no SARIF
- **Planner:** Severity has two levels: category default + per-pattern override
- **Planner:** Category E is Unicode + zero-width only, no decoding logic
- **Planner:** regex crate, not aho-corasick

---
*Created: 2026-04-02 after discuss-phase 1*
