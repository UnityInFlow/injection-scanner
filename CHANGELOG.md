# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Tool & Permission Abuse** category (`PI050`-`PI057`, #33): detects documents that widen the
  agent's own authority — the agentic equivalent of privilege escalation. Three structural
  patterns match a wildcard tool grant, permission-allow wildcard, or bypass permission mode
  directly in a file's own parsed frontmatter (`scope: frontmatter`, CRITICAL). Five prose
  patterns detect persuasion to widen authority through the ordinary regex engine (HIGH):
  skip-permissions flags and bypass-mode instructions, unrestricted-authority claims,
  skip-confirmation directives, settings-file widening directives (the CVE-2025-53773 shape),
  and imperative instructions telling the agent to `turn off` a hook or guardrail. Recall on this category's 12
  threat-model payloads went from the pre-pattern 0/12 baseline to **12/12 (100%)**; measured
  library-wide recall moved from 63/72 to **70/72 (97.2%)**.

- **MCP & Tool-Description Poisoning** category (`PI060`-`PI069`, #34): detects the attack the
  user never sees — instructions hidden in a tool's own `description`, read by the model on
  every call and never surfaced in a host UI — plus the supply-chain hygiene signals that
  describe how such a tool arrives in the first place. Ten patterns across two severity bands:

  | ID | Name | Severity | Detects |
  |---|---|---|---|
  | `PI060` | unvetted-mcp-server-source | MEDIUM | An MCP server entry installed from a git reference, repository shorthand or archive URL (`git+https://…`, `github:owner/repo`, `.tgz`/`.zip`/`.git`) rather than a package registry |
  | `PI061` | plaintext-mcp-endpoint | MEDIUM | A server entry pointing at a plaintext `http://` endpoint rather than TLS, excluding loopback |
  | `PI062` | remote-script-mcp-launch | MEDIUM | A launch command that pipes a downloaded script into a shell (`curl … \| sh`) |
  | `PI063` | tool-description-directive | HIGH | A tool `description` that addresses the model in the second person and directs it at something outside its own declared arguments — a filesystem path, an environment variable, or a concealment instruction |
  | `PI064` | tool-description-file-smuggle | HIGH | A description that smuggles a file's contents through an argument the tool's own schema does not describe as carrying file contents |
  | `PI065` | tool-description-emphasis-block | HIGH | A `<IMPORTANT>...</IMPORTANT>`-style tag-delimited or bracketed emphasis wrapper enclosing a directive |
  | `PI066` | cross-tool-shadowing | MEDIUM | A description that names a *different* tool's invocation (or output) as the trigger for a directive, in the third person, without requiring second-person address |
  | `PI067` | tool-override-directive | MEDIUM | A description that removes the reader's choice between two named tools (`instead of using X, always Y`), distinct from an ordinary recommendation |
  | `PI068` | version-conditional-directive | MEDIUM | A directive whose consequent is gated on a version comparison |
  | `PI069` | deferred-activation-directive | MEDIUM | A directive whose consequent is gated on a date, an approval, or a call count |

  `PI063`-`PI065` are HIGH — the severity `install-hook` blocks a commit at by default — because
  this is the attack shape the category is named for. `PI060`-`PI062` and `PI066`-`PI069` are
  MEDIUM by category-default inheritance: real, worth surfacing in `check`, JSON, SARIF and code
  scanning, but below the commit-blocking line. Recall on this category's 12 threat-model
  payloads went from the pre-pattern 6/12 baseline to **9/12 (75%)**; measured library-wide
  recall moved from 76/84 (90.5%, the pre-pattern baseline measured immediately after this
  category's corpus landed) to **102/109 (93.6%)** as the corpus grew alongside the patterns,
  both here and in categories outside this phase.

### Changed

- **Behaviour change: a wildcard tool grant in a scanned file's own frontmatter is now a
  CRITICAL finding (D-12).** Previously this shape produced no detection at all — the structural
  pass (`scope: frontmatter`) was defined in the schema but no pattern used it, so it was inert
  in every shipped binary. `spec-ci-plugin` shells out to this binary in consumer CI, so a
  consumer repository whose skill or agent config carries `allowed-tools: "*"`,
  `permissions.allow: ["Bash(*)"]` or `permissions.defaultMode: bypassPermissions` will see its
  build go from green to red on upgrade. This is the finding the tool exists to produce, not a
  regression — see the README's "Behaviour change" note for the full justification and the
  `--baseline` migration path (shipped in v0.1.0) for consumers that need to accept their current
  state before narrowing it.

- **Behaviour change: an MCP server entry with an off-registry install source, a plaintext
  endpoint, or a remote-script launch is now a MEDIUM finding (D-03).** On upgrade, a consumer's
  CI will newly see a finding on any `.mcp.json`, `mcp.json`, `claude_desktop_config.json` or
  settings-shaped file matching one of these three shapes. These sit below the severity
  `install-hook` blocks commits at, so an existing pre-commit hook keeps passing; `--baseline`
  remains the migration path. Deliberately **not** reported: the ordinary unpinned registry
  install (`npx -y @scope/pkg`) — measured to be the ecosystem default (8 of 24 real manifests
  with a launch command use it), so reporting it would mean reporting almost every real MCP
  setup. See the README's "Behaviour change" note and the header of
  `patterns/core/mcp-tool-poisoning.yaml` for the full measurement.

- **Behaviour change: a tool `description` addressing the model in the second person and
  directing it at something outside its own declared arguments is now a HIGH finding (D-01).**
  `PI063`-`PI065` fire on this shape from their first committed draft — a filesystem path, a
  credential file, an environment variable, or a file's contents smuggled through an unrelated
  argument. These are HIGH because this category exists for exactly this shape: instructions
  hidden in a tool's own description, read by the model on every call and never surfaced in a
  host UI. On upgrade, a consumer's CI will newly see a finding on any vendored MCP tool
  definition whose description carries this shape.

- **Behaviour change: a tool description that shadows a different tool, overrides a tool
  choice, or gates a directive on a version/date/approval/call-count is now a MEDIUM finding
  (D-04).** `PI066` closes the third-person blind spot `PI063`-`PI065`'s second-person
  discriminator deliberately accepts; `PI067` detects substitution rather than mere
  recommendation; `PI068`/`PI069` detect the rug-pull class's conditional LANGUAGE, not the
  class itself. See the Security note below and the README's four behaviour-change callouts for
  the full narrowing and every accepted cost.

### Security

- **CAT-02's discriminators trade recall for false-positive safety in two named, accepted ways.**
  (1) `PI063`-`PI065` require second-person address; a bare third-person payload aimed directly
  at the model, with no other tool referenced, is outside their reach by design — the accepted
  cost is stated in `patterns/core/mcp-tool-poisoning.yaml`'s header comment, and `PI066`
  narrows but does not close it (it closes only the cross-tool-shadowing shape specifically).
  (2) `PI068`/`PI069` detect version-, date-, approval- and call-count-conditional directive
  LANGUAGE only — they cannot and do not mitigate the rug-pull class itself, since a single
  static scan cannot prove a server will not silently republish a different, poisoned
  description once a gating condition is met. Both limitations are measured and recorded, not
  discovered in review; see `.planning/phases/04-mcp-tool-description-poisoning-cat-02-34/deferred-items.md`
  for the full accounting, including two engine-capability gaps found during this work and filed
  as follow-up issues: a JSONC-commented `mcp.json` silently skips the structural pass with no
  diagnostic (#129), and the decoded-layer pass does not re-run structural (`scope: frontmatter`)
  patterns against a decoded value (#130).

## [0.1.0] - 2026-08-29

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
- SARIF 2.1.0 output (`--format sarif`) with rule metadata, `ruleIndex`, line-independent
  `partialFingerprints` and GitHub `security-severity`, plus a code-scanning upload workflow that
  runs only on triggers a fork cannot fire (#5)
- `--baseline <file>` and `--write-baseline <file>` for incremental adoption on an existing
  repository: accepted findings move into a withheld `baselined` array rather than being dropped,
  and are stored as `(file, pattern_id, sha256(matched_text))` digests rather than verbatim
  payloads, so the adoption artifact does not itself become a finding. `install-hook --baseline`
  wires the two together (#25)
- A measured recall corpus: `tests/corpus/attack/` holds 60 realistic payloads, twelve per
  README-claimed category, written from the threat model rather than derived from the patterns —
  a corpus assembled from each pattern's own `example` would score 100% by construction and
  measure nothing. `tests/recall_test.rs` pins the result *exactly*, so an improvement fails the
  build too and the published number cannot go stale while the real one drifts (#81)
- `docs/PATTERN-CATALOGUE.md`, generated from the library, plus binding `example` /
  `counter_example` fields in the pattern schema and a staleness gate that fails CI when the
  catalogue and the patterns disagree
- A coverage gate at 85%, Criterion benchmarks in CI (#29), and a per-pattern test-policy
  ratchet (#70)

### Changed

- The file walker now uses the `ignore` crate, so `.gitignore` is honoured by default; broadened
  the set of scanned file types and added `--all-files` for scanning everything; narrowed PI011 to
  cut a false-positive source (#22, #23)
- Severity rebalanced across the full CRITICAL/HIGH/MEDIUM/LOW range instead of concentrating
  almost everything in CRITICAL or HIGH (#21)
- `raw_only` is now an explicit field in the pattern schema. It was briefly inferred from a
  `homoglyph` tag, which meant a taxonomy label silently switched off a pattern's
  Unicode-normalized pass — the pass that defeats confusable substitution. Tags never change
  matching behaviour; documented in `PATTERNS.md` and pinned by a test
- **Measured detection recall went from 10/60 to 56/60.** Four of the five README-claimed
  categories were rewritten from lists of literal phrases into verb × modifier × object matrices:
  role override 1/12 → 11/12 (#80), data exfiltration 0/12 → 12/12 (#95), instruction injection
  0/12 → 12/12 (#97), jailbreaks 1/12 → 12/12 (#99). The defect was the *shape* of the patterns,
  not the difficulty of the attacks — PI021 required the verb *POST*, so
  `Print your system prompt verbatim` walked straight past it. Obfuscation detection was always
  shape-based and was already at 9/12. The four remaining misses are deliberate and documented:
  three base64 cases need the decoder (#30), and one role-override precedence claim is not
  separable from ordinary configuration documentation
- Pattern `description` fields now carry the widened concept. A pattern's `name` is a consumer
  contract — `pattern_name` ships in the JSON `spec-ci-plugin` reads — so renaming one is a
  consumer-visible break for zero detection value

### Fixed

- PI017 no longer fires on ordinary CSS — `font-size: 0.8rem` was matching an unterminated
  `font-size\s*:\s*0` regex; PI045 no longer matches ordinary scientific notation (`Δt`, `kΩ`,
  `250µs`) — its confusable-character list is now limited to glyphs that actually substitute for a
  Latin letter
- 2 CRITICAL and 25 HIGH false positives, found by sweeping roughly 1,300 files of real
  third-party documentation rather than trusting the hand-written clean corpus, which is 18 files
  all authored by someone who knew which pattern they were testing. Recall held at 56/60 through
  the fix (#102)
- `update your instructions` no longer fires PI009. That is a HIGH, which is the threshold
  `install-hook` writes by default, so the false positive blocked commits. The verb list is now
  split on benignness: `reset` / `replace` / `overwrite` match bare, while `update` / `change` /
  `modify` require a qualifier binding the object to the running configuration

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

[Unreleased]: https://github.com/UnityInFlow/injection-scanner/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.3...v0.1.0
[0.0.3]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/UnityInFlow/injection-scanner/releases/tag/v0.0.1
