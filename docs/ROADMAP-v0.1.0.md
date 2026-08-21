# Upgrade Roadmap — v0.0.2 → v1.0.0

Sequenced so each release is shippable on its own and nothing lands without a test gate.
Derived from `AUDIT-2026-08.md` and `DETECTION-BACKLOG.md`.

---

## v0.0.3 — "Make it actually work" (patch, no new features)

**Goal:** every existing claim in the README becomes true. No new patterns, no new commands.

| # | Item | Audit ref |
|---|---|---|
| 1 | Restore CI — D-02 public/fork split on a GitHub-hosted runner. **Blocks every other item in this milestone; nothing else merges until it is green.** | H-07 |
| 2 | Case-insensitive matching by default + `case_sensitive` opt-out field | C-01 |
| 3 | `Scanner` struct — compile patterns once, not per file | C-02 |
| 4 | Per-file error isolation for non-UTF-8 / unreadable files | C-03 |
| 5 | Suppression: `ignore` (same line) + `ignore-next-line` + `ignore-file`; fix README | C-04 |
| 6 | `find_iter` with a per-line cap | C-05 |
| 7 | `expect()` with rationale instead of `unwrap()` in `allowlist.rs` | M-02 |
| 8 | Test asserting all embedded regexes compile; `--strict-patterns` mode | M-01 |
| 9 | `--format` as a clap `ValueEnum` — `--format sarif` currently returns text with exit 1 | C-06 |
| 10 | Fix the `spec-ci-plugin` consumer: version-keyed cache, SHA256 verification, reconcile the `v0.0.1`/`v0.0.2` default mismatch | L-02 |

**Exit criteria:** CI green on a PR from a fork (gate for everything else) · `Ignore all previous
instructions` is detected · 500-file scan under 200ms · a binary file in the tree does not abort the
scan · `--format bogus` errors instead of printing text.

**Do NOT ship v0.0.3 without keeping the musl asset contract** — `spec-ci-plugin` executes
`injection-scanner-<arch>-unknown-linux-musl`, raw and unextensioned, at a pinned tag (L-02).

---

## v0.1.0 — "Complete the original spec + halve the false positives"

**Goal:** close every unmet v0.0.1 requirement and make the tool usable on documentation.

### Requirements closed
- **CLI-04** SARIF 2.1.0 output (issue #5) — enables GitHub code scanning upload
- **HOOK-01** `install-hook` (issue #8) + `.pre-commit-hooks.yaml` for the pre-commit framework
- **PERF-01** verified by a criterion benchmark in CI, not by assertion
- **SCAN-03** severity rebalanced across the full CRITICAL/HIGH/MEDIUM/LOW range (H-02)

### Engine work
- **E7** markdown context classifier + `confidence` on every finding (H-01)
- **E6** Aho-Corasick prefilter (issue #4)
- **E5** multi-line window pass (H-05)
- **E1** normalization pass (defeats homoglyph/spacing/zero-width evasion)

### CLI surface
- `--fail-on critical|high|medium|low`, `--quiet`, exit 2 for warnings-only (spec-linter convention)
- `rules` / `explain <PI0XX>` subcommands
- `--exclude <glob>`, `.gitignore` respect, `--max-file-size`, skip `.git`/`target`/`node_modules` (H-03)
- Broader extension set + `--include` + extensionless agent files (H-04)
- `--baseline <file>` so existing repos can adopt incrementally

### Patterns
- Part 1 of `DETECTION-BACKLOG.md` — fill `PI008–PI009`, `PI015–PI019`, `PI026–PI029`, `PI039`, `PI043–PI049`
- False-positive corpus in CI: this repo's README plus real-world clean specs must stay at zero findings

**Exit criteria:** `injection-scanner check README.md` → 0 findings · SARIF validates against the
2.1.0 schema · `install-hook` completes a real commit in <200ms · FP rate measured, not claimed.

---

## v0.2.0 — "Agentic attack surface"

**Goal:** cover what an *agent* can be made to do, not just what a chatbot can be made to say.

- **E4** structural frontmatter analysis — **land this before** `PI050`–`PI069`, which depend on it
- `PI050–PI059` tool & permission abuse
- `PI060–PI069` MCP / tool-description poisoning
- `PI070–PI079` persistence & lifecycle hijack
- **E2** recursive decoder — closes issues #6 (HTML entities) and #7 (base64) properly
- **E3** invisible-character heuristic
- Library/CLI split: publish the crate so `kore` (08) and `agent-sandbox` (14) can embed the engine
- Runtime filter mode (issue #11) — `--mode filter`, stdin → annotated/redacted stdout

---

## v0.3.0 — "Reach"

- `PI080–PI099` indirect/RAG-borne + credential harvesting
- `PI100–PI129` output hijack, multilingual, delimiter spoofing
- Homebrew formula (issue #10) + `cargo-binstall` metadata + crates.io publish
- `UnityInFlow/injection-scanner-action` GitHub Action with automatic SARIF upload
- `.injection-scanner.toml` config file
- `--format github|junit|markdown`
- `cargo-fuzz` target on the pattern loader; coverage gate wired into CI

---

## v1.0.0 — "Stable"

- Frozen pattern-file schema (versioned, `schema_version` field) and stable library API
- **E8** optional semantic pass behind a non-default cargo feature
- Published false-positive and true-positive benchmark results
- Full ecosystem integration proven end-to-end: 04 spec-ci-plugin, 08 kore middleware,
  14 agent-sandbox runtime filter, 17 skills-registry submission gate

---

## Cross-cutting constraints

1. **Asset contract** (L-02) — the musl binaries must stay raw, unextensioned, and named by target
   triple on every release; add a release-time smoke test that fetches the published URL shape.
2. **CI policy** — the repo is public and the org runner group is `allows_public_repositories: false`.
   Self-hosted release work must complete in a private window, or run only on triggers forks cannot
   fire. See the root CLAUDE.md decisions log (OPS-01/OPS-02, D-02).
3. **No auto-fix, ever** — `REQUIREMENTS.md` rules it out for a security tool. Flag only.
4. **Severity discipline** — a growing library only stays usable if MEDIUM and LOW are real.
