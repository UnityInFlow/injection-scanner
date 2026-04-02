# PR Artifacts Skill — Rust

Produce ALL of the following before opening a PR for review.

## 1. GitHub Issue
- Verify the PR is linked to a GH issue: `gh issue view <N>`
- Or create one: `gh issue create --title "<summary>" --body "..."`
- Set the issue reference in the PR body: "Closes #N"

## 2. ADR (Architecture Decision Record)
Required when the PR introduces or changes:
- A new crate dependency
- The pattern matching engine or YAML format
- Output format contracts (JSON, SARIF)
- Distribution or cross-compilation strategy
- Pre-commit hook installation mechanism

Not required for: new patterns using existing format, bug fixes, test additions.

File as `docs/adr/NNNN-short-title.md` using the template in this skill directory.

## 3. Documentation Update
Update whichever applies:
- `README.md` — if new commands, flags, or patterns added
- `PATTERNS.md` — if pattern contribution process changed
- `CONTRIBUTING.md` — if contributing workflow changed
- `///` rustdoc on new public functions and structs

## 4. Tests
Verify before shipping:
- [ ] Unit tests for pattern matching (>=3 true positives, >=2 non-matches per pattern)
- [ ] Integration tests for CLI (exit codes, output formats)
- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes

## 5. Verification Report
In the PR description:
- [ ] All tests pass
- [ ] No `unwrap()` in production code
- [ ] No `println!` debug output
- [ ] Pattern match exhaustive — no catch-all `_` unless justified
- [ ] ADR written (or: not required because ...)
- [ ] Docs updated: [which files]
- [ ] Issue: Closes #N
- [ ] Smoke test: `injection-scanner check <file>` — describe output
