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
