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

### Lints

`cargo clippy -- -D warnings` is the gate, and it covers less than it looks like
it covers. Clippy's **restriction** group is off by default, and `-D warnings`
does not turn it on — it only promotes lints that are already firing. So a lint
in that group catches nothing until something opts into it explicitly.

`clippy::unwrap_used` is the one that bit us. CLAUDE.md has said "no `unwrap()`
in production code" since the project started, and issue #19 asked for the lint;
only the cleanup half shipped, so for an entire milestone a new `unwrap()` in
`src/` passed CI silently. `src/lib.rs` and `src/main.rs` now each carry
`#![deny(clippy::unwrap_used)]` — both, because they are separate crates and a
crate-level attribute does not cross that boundary.

Two things to know if you go looking:

- **Testing the gate is easy to get wrong.** `Some(1).unwrap()` is caught even
  without the attribute, because clippy can see statically that it succeeds.
  Probe with a value it cannot reason about — `raw.parse::<usize>().unwrap()` —
  or you will conclude the gate works when it does not.
- **The same gap applies to every other restriction lint** (`indexing_slicing`,
  `panic`, `expect_used`, `float_arithmetic`, …). If a rule matters, deny it
  explicitly; do not assume `-D warnings` is enforcing it.

`expect()` is deliberately *not* denied. It carries a message, and the one use in
`allowlist.rs` is on a compile-time-constant regex covered by a test — denying it
there would push toward a silent fallback, which is worse than a documented panic
on an unreachable branch. If a second `expect()` shows up in `src/`, that is the
moment to revisit.

Integration tests are separate crates and are unaffected. `unwrap()` in a test is
fine.

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

## The release asset contract

`spec-ci-plugin` (tool 04) downloads and executes two of this repository's release
assets directly, at a pinned tag, with **no file extension**:

```
injection-scanner-x86_64-unknown-linux-musl
injection-scanner-aarch64-unknown-linux-musl
```

It `chmod +x`es what it downloads and runs it, after verifying the bytes against
`SHA256SUMS.txt` from the same release. So these names, and the fact that they are
raw executables rather than archives, are a **public API of this repository**.
Renaming one, switching to tarballs, changing a target triple, or dropping a musl
leg is a breaking change for another repository's CI, whatever this repo's version
number says — and it surfaces there, not here, which is the hardest place to
diagnose it.

Two things enforce it, and they see different failures:

- `tests/release_contract_test.rs` runs in the ordinary `cargo test` gate. It parses
  `.github/workflows/release.yml` as YAML and checks the workflow still *produces*
  the names the consumer *requests*: both musl targets are in the build matrix and
  not `experimental`, the upload globs still select them, the attestation
  `subject-path` still covers them, and the packaging step still emits a raw
  target-triple-named binary. Break the contract in a pull request and it fails
  there.
- `verify-published-assets` in `release.yml` runs after the release is published. It
  walks the consumer's exact path — anonymous `curl` of the public download URL at
  the new tag — and asserts HTTP 200, ELF magic, the right architecture in the ELF
  header, presence in `SHA256SUMS.txt`, a passing checksum, and a working
  `--version`. A workflow can be perfectly specified and still fail to upload; this
  is what catches that.

Neither can see what the other sees. If you touch `release.yml`, expect both.

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
- PI050-PI059: Tool and permission abuse (Category F)
- PI070-PI079: Persistence and lifecycle hijack (Category H)
- PI110-PI119: Multilingual evasion (Category G; Czech first, one language per slice)

## Commit Convention

```
feat: add new pattern category
fix: reduce false positives in PI001
test: add non-match cases for exfiltration
docs: update PATTERNS.md
```
