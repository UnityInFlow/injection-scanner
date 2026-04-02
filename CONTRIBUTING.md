# Contributing to injection-scanner

## Development

```bash
cargo build          # compile
cargo test           # run tests
cargo clippy -- -D warnings  # lint
cargo fmt            # format
cargo build --release # optimized binary
```

## Adding a New Pattern

1. Choose the appropriate category YAML file in `patterns/core/`
2. Add your pattern following the existing format:
```yaml
  - id: PI0XX
    name: descriptive-name
    pattern: "your\\s+regex\\s+pattern"
    description: "What this pattern detects"
    remediation: "How to fix it"
    tags: [category]
```
3. Optionally override severity: `severity: CRITICAL` (otherwise inherits from category)
4. Add test cases in the appropriate test file
5. Run `cargo test` -- all green
6. Submit a PR

## Pattern ID Numbering

- PI001-PI009: Role override (Category A)
- PI010-PI019: Instruction injection (Category B)
- PI020-PI029: Data exfiltration (Category C)
- PI030-PI039: Jailbreaks (Category D)
- PI040-PI049: Encoding/obfuscation (Category E)

## Commit Convention

```
feat: add new pattern category
fix: reduce false positives in PI001
test: add non-match cases for exfiltration
docs: update PATTERNS.md
```
