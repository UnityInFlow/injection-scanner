# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `install-hook` — installs a git pre-commit hook (`.pre-commit-hooks.yaml`) that blocks a commit
  when staged files contain findings at or above a threshold (#8)
- `--fail-on <severity>`, `--quiet`, exit code `2` for findings that exist but sit below the
  `--fail-on` bar, and the `rules` / `explain` subcommands (#25)
- Unicode normalization (separator, spacing, homoglyph, fullwidth, zero-width) — obfuscating a
  payload is no longer a bypass (#26)
- Multi-line matching across paragraph joins — a newline is no longer a bypass (#24)
- Markdown context awareness: a payload quoted inside a fenced code block, an inline code span, or
  a documentation example scores below the confidence threshold by default and is no longer
  reported as an attack; below-threshold findings are recorded, never dropped (#23, #20)
- A false-positive corpus (`tests/corpus/clean/`, `tests/corpus/documentation/`) that asserts zero
  findings on legitimate documents by default, and non-zero on the documentation corpus only under
  `--strict` (QUAL-03)
- 18 new patterns filling the reserved ID gaps — PI008-PI009, PI015-PI019, PI026-PI029, PI039,
  PI043-PI047, PI049 — growing the library from 30 to 48 patterns across all five categories (#27)

### Changed

- The file walker now uses the `ignore` crate, so `.gitignore` is honoured by default; broadened
  the set of scanned file types and added `--all-files` for scanning everything; narrowed PI011 to
  cut a false-positive source (#22, #23)
- Severity rebalanced across the full CRITICAL/HIGH/MEDIUM/LOW range instead of concentrating
  almost everything in CRITICAL or HIGH (#21)
- `raw_only` is now an explicit field in the pattern schema (previously implicit), documented in
  `PATTERNS.md`

### Fixed

- PI017 no longer fires on ordinary CSS — `font-size: 0.8rem` was matching an unterminated
  `font-size\s*:\s*0` regex; PI045 no longer matches ordinary scientific notation (`Δt`, `kΩ`,
  `250µs`) — its confusable-character list is now limited to glyphs that actually substitute for a
  Latin letter

### Security

- `src/` now denies `clippy::unwrap_used`, closing the gap left when #19 shipped only its cleanup
  half

## [0.0.3] - 2026-08-22

### Added

- `--no-suppress` — ignore all in-file suppression directives, for scanning a document you did not
  write
- Duplicate pattern ID detection, unknown-field rejection, and `--strict-patterns` for external
  pattern files (#28)
- `verify-published-assets`, a release-time gate that walks the exact download path
  `spec-ci-plugin` uses and fails the run if the published asset contract breaks (#18)

### Changed

- Matching is case-insensitive by default, and the pattern set is compiled once per scan instead of
  once per file (#12, #13)
- A suppressed finding is now recorded with the same detail as a visible one (severity, message,
  matched text), not just a bare pattern ID (#15, #16, #19)
- Unknown `--format` values are rejected instead of silently falling through to text output (#42)
- CI restored via the D-02 public/fork split, and the release pipeline moved to GitHub-hosted
  runners (#45)
- GitHub Actions are SHA-pinned, with Dependabot keeping the pins current

### Fixed

- A read error on one file no longer aborts the whole scan — errors are now isolated per file (#14)

### Security

- Every release binary now carries a signed SLSA build-provenance attestation (#45)
- `src/main.rs` and `src/lib.rs` deny `clippy::unwrap_used`

## [0.0.2] - 2026-06-24

### Changed

- Release assets renamed from `injection-scanner`, `injection-scanner-darwin-arm64`,
  `injection-scanner-linux-x86_64` to the target-triple form `injection-scanner-<target-triple>` —
  six binaries plus `SHA256SUMS.txt`. This is the shape `spec-ci-plugin` first consumed, and remains
  this repository's release asset contract.

## [0.0.1] - 2026-04-02

### Added

- Initial release: pattern library across five attack categories (role override, instruction
  injection, data exfiltration, jailbreaks, encoding/obfuscation)
- Text and JSON output modes
- Inline suppression
- Stdin mode (`check -`)

[Unreleased]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.3...HEAD
[0.0.3]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/UnityInFlow/injection-scanner/releases/tag/v0.0.1
