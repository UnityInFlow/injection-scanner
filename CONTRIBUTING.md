# Contributing to injection-scanner

## Development

```bash
cargo build          # compile
cargo test           # run tests
cargo clippy -- -D warnings  # lint
cargo fmt            # format
cargo build --release # optimized binary
cargo bench          # performance benchmarks (benches/scan.rs)
```

### Performance

The scanner has a budget of **200ms for a typical project** (500 files). Two things
defend it, and they do different jobs:

- `tests/perf_regression_test.rs` runs on every CI job and asserts that the pattern
  set is compiled **once**, not per file. It compares the cost of a 500-file scan
  against the cost of one pattern-set compile, so it is a ratio rather than a
  wall-clock bound and holds on any hardware. This is the guard that matters: the
  regression it catches once cost 806ms against a 200ms budget.
- `cargo bench` measures the four shapes of work in release mode — compile, one
  large file, 500 small files, and a pathological single line. Criterion keeps a
  baseline in `target/criterion`, so a second run reports the delta.

If you touch `Scanner`, run both.

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
