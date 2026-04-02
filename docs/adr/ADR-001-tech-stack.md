# ADR-001: Tech Stack for injection-scanner

**Status:** Accepted
**Date:** 2026-04-02

## Context
injection-scanner needs to scan files for prompt injection patterns in <200ms (pre-commit hook). Must support YAML pattern definitions, multiple output formats, and cross-platform distribution.

## Decision
- **Rust** -- performance requirement (<200ms), single binary distribution, cross-compilation
- **clap (derive)** -- CLI framework, generates help/version/completions
- **regex** -- pattern matching engine. Sufficient for 30 patterns on <50kb files. Aho-Corasick planned for v0.1.0 when pattern count grows.
- **serde + serde_yaml** -- YAML pattern file parsing
- **serde_json** -- JSON output format
- **anyhow** -- error handling in main binary
- **thiserror** -- typed errors in library code
- **Embedded core patterns + optional external YAML** -- single binary works standalone, community patterns via external directory

## Alternatives Considered
- **TypeScript** -- too slow for pre-commit hook, no single binary
- **aho-corasick** -- deferred to v0.1.0, regex is fast enough for 30 patterns
- **tree-sitter** -- overkill for line-by-line pattern scanning
- **External patterns only** -- binary wouldn't work standalone

## Consequences
- Single binary, zero runtime dependencies
- <200ms is achievable (regex on 30 patterns)
- Community can contribute patterns via YAML PRs without touching Rust code
- Cross-compilation required for 5 platform targets
