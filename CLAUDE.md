# injection-scanner — Prompt Injection Static Scanner

## Project Overview

**Tool 03** in the [UnityInFlow](https://github.com/UnityInFlow) ecosystem.

Static scanner for prompt injection attacks in skill files, CLAUDE.md, RAG documents, and user inputs. Detects role overrides, instruction injection, data exfiltration, jailbreaks, and encoding/obfuscation attacks with a maintained pattern library. Runs as a pre-commit hook in <200ms.

**Phase:** 1 | **Stack:** Rust | **Distribution:** pre-built binaries + Homebrew

## Status

Ready to build — `spec-linter` (Tool 01) and `ai-changelog` (Tool 02) are shipped.

## Reference Documents

- `03-injection-scanner.md` — Feature spec, pattern library (5 attack categories), architecture, YAML pattern format, implementation todos (Weeks 5-6)
- `claude-code-harness-engineering-guide-v2.md` — Harness engineering patterns and best practices

Read these before making architectural or scope decisions.

## Tooling

| Tool | Status | Usage |
|---|---|---|
| **GSD** | Installed (global) | `/gsd:new-project` to scaffold when ready. `/gsd:plan-phase` and `/gsd:execute-phase` for structured development. |
| **RTK** | Active (v0.34.2) | Automatic via hooks. Compresses cargo, git output. ~80% token savings. |
| **Superpowers** | Active (v5.0.5) | Auto-triggers brainstorming, TDD, planning, code review, debugging skills. |

## Constraints

### Rust (inherited from ecosystem CLAUDE.md)
- Rust stable, edition 2021
- `clap` for CLI argument parsing (derive feature)
- `serde` + `serde_json` for serialisation
- `tokio` for async where needed
- `anyhow` for error handling in binaries, `thiserror` for libraries
- Format: `cargo fmt` before every commit
- Lint: `cargo clippy -- -D warnings` must pass
- Distribution: pre-built binaries for macOS (arm64/x86_64), Linux (x86_64/aarch64), Windows
- No `unwrap()` in production code — use `?` or handle the error
- Pattern match exhaustively — no catch-all `_` unless truly needed

### General
- Test coverage >80% on core logic before release
- No secrets committed — all credentials via environment variables
- No `console.log` or `println!` debug output left in committed code

## Acceptance Criteria — v0.0.1

- [ ] 30+ patterns across 5 categories: role override, instruction injection, exfiltration, jailbreaks, encoding attacks
- [ ] YAML pattern loader: load from `patterns/` directory
- [ ] Severity classifier: CRITICAL / HIGH / MEDIUM / LOW
- [ ] Remediation hints per pattern
- [ ] File scanner: scan any text/markdown/YAML file
- [ ] Stdin mode: `cat skill.md | injection-scanner check -`
- [ ] JSON and SARIF output modes
- [ ] Allowlist: `# injection-scanner:ignore PI001` inline suppression
- [ ] `injection-scanner install-hook` — installs pre-commit hook
- [ ] Hook runs in <200ms for a typical project
- [ ] Pre-built binaries for macOS, Linux, Windows
- [ ] Community pattern contribution guide: `PATTERNS.md`

## Development Workflow

When ready to build:

1. `/gsd:new-project` — describe injection-scanner, feed existing spec. Generates `.planning/PROJECT.md`, `REQUIREMENTS.md`, `ROADMAP.md`, `STATE.md`
2. `/gsd:discuss-phase 1` — lock in decisions for Week 5 (core scanner: pattern engine, YAML loader, severity classifier)
3. `/gsd:plan-phase 1` — atomic task plans with file paths
4. `/gsd:execute-phase 1` — parallel execution with fresh context windows
5. `/gsd:discuss-phase 2` — lock in decisions for Week 6 (pre-commit hook, cross-compilation, release)
6. `/gsd:plan-phase 2` — atomic task plans
7. `/gsd:execute-phase 2` — build and ship

Superpowers skills (TDD, code review, debugging) activate automatically during execution.

## Key Dependencies (for reference, not installed yet)

- `clap` — CLI argument parsing (derive)
- `serde` + `serde_yaml` — YAML pattern file parsing
- `regex` — pattern matching engine
- `serde_json` — JSON output

---

## CI / Self-Hosted Runners

Use UnityInFlow org-level self-hosted runners. Never use `ubuntu-latest`.

```yaml
# Default (X64)
runs-on: [arc-runner-unityinflow]

# ARM64 cross-compilation
runs-on: [orangepi]

# Matrix for both architectures
strategy:
  matrix:
    runner: [arc-runner-unityinflow, orangepi]
runs-on: ${{ matrix.runner }}
```

Available runners: `hetzner-runner-1/2/3` (X64), `orangepi-runner` (ARM64).

---

## Do Not

- Do not start implementation until spec-linter v0.0.1 is published on npm (DONE)
- Do not use `unwrap()` in production code
- Do not commit secrets or API keys
- Do not skip writing tests
- Do not inline the reference docs into this file — read them by path
