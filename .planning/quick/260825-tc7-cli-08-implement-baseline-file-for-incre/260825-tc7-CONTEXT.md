# Context — CLI-08 `--baseline`

**Locked by the user 2026-08-25. Do not revisit these; plan against them.**

Requirement: `.planning/REQUIREMENTS.md` **CLI-08** — "`--baseline <file>` for incremental
adoption on an existing repository". GitHub issue **#25** (last open item on it; `--fail-on`,
`--quiet`, exit 2, `rules`, `explain` already shipped in PR #76). Phase 4, `.planning/ROADMAP.md`.

Issue #25 calls `--baseline` "the single highest-value adoption feature here". STATE.md agrees:
"an existing repository cannot adopt this without it, because day one is a wall of findings."

## D-1 — Fingerprint: hashed payload + occurrence count

An entry identifies an accepted finding by:

| field | role |
|---|---|
| `file` | repo-relative path as the scanner reports it |
| `pattern_id` | e.g. `PI001` |
| `digest` | `sha256:<hex>` over the **matched text** |
| `count` | how many occurrences of this exact fingerprint were accepted |
| `first_seen_line` | informational only — **not** part of identity |

**Line numbers are deliberately excluded from identity.** Editing anything above a finding must
not invalidate the baseline, or the baseline needs regenerating on every commit and stops being
a record of a decision.

**The payload is hashed, not stored verbatim, for a specific reason:** `json` is in
`DEFAULT_EXTENSIONS` (`src/walk.rs`), so a committed `baseline.json` full of verbatim payloads
would itself be flagged by the next scan — the adoption artifact would become a finding source.
Hashing makes the baseline inert. It also removes any crafted-collision surface: the adversary
authors the scanned text, and a weak/non-cryptographic digest would let a *new* payload be
tuned to match an already-accepted fingerprint.

`count` closes the "more of the same is free" hole: baselining two occurrences accepts two, not
an unlimited number. A third identical occurrence is reported.

Cost accepted: a reviewer reading the baseline in a PR sees `PI001 in docs/foo.md ×2`, not the
text. That is the trade the user chose.

New dependency: `sha2 = "0.10"`.

## D-2 — Creation and reporting

- `check --write-baseline <FILE>` — scan, write the baseline, exit `0` (`exit::CLEAN`). This is
  "accept the current state".
- `check --baseline <FILE>` — findings whose fingerprint is in the file are moved out of
  `matches` into a **new `baselined: Vec<ScanMatch>` array** on `ScanReport` and do not affect
  the exit code.
- `--baseline` and `--write-baseline` are **mutually exclusive** (clap `conflicts_with`).
- **Nothing is dropped.** `baselined` is a fourth array alongside the existing `matches`,
  `suppressed` and `low_confidence`, and follows their established rationale verbatim: a finding
  withheld is filed under *the reason* it is withheld, never discarded. Text output gets a
  one-line count, in the same voice as the existing suppression / low-confidence lines.
- **Stale entries are reported.** Baseline entries that matched nothing this run are surfaced as
  a note so the baseline can be pruned — a stale entry is a live licence to re-introduce the
  finding it once accepted.

## Non-negotiable constraints carried in from the repo

- `baselined` is an **additive** field with `#[serde(default)]`. The top-level JSON must stay an
  **array of report objects** — `spec-ci-plugin` does `JSON.parse(output) as Array<...>`. This is
  the same constraint that shaped `suppressed` and `low_confidence`; the audit's L-02 JSON
  envelope is still deferred.
- Severity tallies (`critical_count` etc.) count `matches` **only**, so a baselined finding must
  not be counted. Match the existing `with_withheld` construction rather than inventing a path.
- `#![deny(clippy::unwrap_used)]` is in force in both crates. No `unwrap()`.
- A malformed or missing baseline file is a **hard error**, not a silent no-op.
- `--write-baseline` against stdin (`check -`) has no meaningful file identity — reject it.
- Tests: integration tests MUST use `env!("CARGO_BIN_EXE_injection-scanner")`. Hand-building a
  path into the target directory is forbidden and `tests/test_harness_contract_test.rs` enforces
  it — see the 2026-08-22 session note in STATE.md for why.
