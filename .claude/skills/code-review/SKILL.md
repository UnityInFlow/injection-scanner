# Code Review Skill — Rust

## Review Checklist
- [ ] No `unwrap()` in production code — use `?` or handle the error
- [ ] No `println!` debug output — use `eprintln!` for errors only
- [ ] `anyhow` for error handling in main/CLI, `thiserror` for library errors
- [ ] Pattern match exhaustively — no catch-all `_` unless truly needed
- [ ] `///` rustdoc on all public items (structs, enums, functions)
- [ ] `serde` derives on all data structs (Serialize, Deserialize)
- [ ] `clap` derive for CLI args — no manual arg parsing
- [ ] Regex patterns compiled once (lazy_static or OnceLock), not per-call
- [ ] No `.clone()` where a reference would work
- [ ] Error messages are user-friendly, not raw Rust debug output
- [ ] YAML pattern files validate on load — bad patterns fail fast with clear message
- [ ] Tests: true positives AND non-matches (false positive prevention)
- [ ] `cargo fmt` clean
- [ ] `cargo clippy -- -D warnings` clean

## Output Format
Return: summary, blocking issues (must fix), suggestions (nice to have).
Cite each issue as `filepath:line — description`.
