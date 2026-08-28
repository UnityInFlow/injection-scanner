## What
<!-- One sentence: what this PR does -->

## Why
<!-- Link to milestone, or brief rationale -->

## Checklist

### Code
- [ ] No `unwrap()` in production code
- [ ] No `println!` debug output
- [ ] Pattern matches exhaustive (no catch-all `_` unless justified)
- [ ] `///` rustdoc on public items

### Tests
- [ ] New tests added for new functionality
- [ ] `cargo test` passes
- [ ] True positives AND non-matches tested per pattern

### Docs
- [ ] ADR written if a non-obvious decision was made
- [ ] README updated (if milestone boundary)

### Verification
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo fmt --check` clean
- [ ] CI green on this branch

### Self-Review
- [ ] I re-read the diff top-to-bottom
- [ ] No unrelated changes bundled in
- [ ] Commit messages follow convention

### Smoke Test Evidence
```
<!-- paste the command you ran and its output -->
```

## Pattern library

Only if this PR touches `patterns/core/`, `src/pattern.rs` or `src/scanner.rs`:

- [ ] `docs/PATTERN-CATALOGUE.md` regenerated — `cargo run --release -- rules --format markdown > docs/PATTERN-CATALOGUE.md`
- [ ] Every new pattern has an `example`, and a `counter_example` unless there is no plausible near-miss
- [ ] At least 3 positive and 2 negative test cases per new pattern (`PATTERNS.md` policy)
- [ ] No clean-corpus document was edited to make a pattern pass — or, if one was, the PR says why the tension is inherent
- [ ] Code-scanning baseline regenerated if `examples/` or `patterns/` changed

See `.claude/skills/pattern-library/SKILL.md` for the full loop.
