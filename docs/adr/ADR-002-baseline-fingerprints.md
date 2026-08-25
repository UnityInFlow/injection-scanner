# ADR-002: Baseline Fingerprints

Date: 2026-08-25
Status: Accepted

## Context

An existing repository cannot adopt this scanner if day one is a wall of findings —
issue #25 calls `--baseline` "the single highest-value adoption feature here". A
baseline that lets a repository accept its current findings once and gate only new
ones is the standard answer. The interesting question is what an accepted entry may
contain.

The scanner's own `DEFAULT_EXTENSIONS` list (`src/walk.rs`) includes `json`, and a
baseline is itself a committed JSON file living in the repository it gates. Any
design that stores the payload verbatim turns the adoption artifact into a finding
source on the very next scan — the tool that is supposed to make the repository
adoptable would fail its own gate.

This ADR is required on two independent `pr-artifacts` triggers: a new crate
dependency (`sha2`) and a change to an output format contract (`ScanReport` gains a
fourth array, `baselined`).

## Decision

An accepted finding is identified by the triple `(file, pattern_id, digest)`, where
`digest` is `sha256:<hex>` computed over the bytes of `matched_text`, plus an
occurrence `count` recording how many identical fingerprints were accepted. Line
number (`first_seen_line`) is carried for information only and is deliberately
excluded from identity.

`--write-baseline <FILE>` scans, writes this file, and exits `0` — writing the
baseline IS the accept decision, so it cannot also be a failure. `--baseline <FILE>`
moves every finding whose fingerprint is recorded, up to its `count`, out of
`matches` and into a new `baselined: Vec<ScanMatch>` array on `ScanReport`,
recomputed through `ScanReport::with_baselined` so the severity tallies never count
an accepted finding.

New dependency: `sha2 = "0.10"` (RustCrypto, `github.com/RustCrypto/hashes`;
provenance verified against the locked decision before being added, per the
`<precondition>` on Task 1 of the executing plan — see threat T-QT-SC).

## Consequences

### Positive

- The committed baseline is inert under a scanner whose default extension set
  includes `json`: hashing means the adoption artifact can never become a finding
  source under its own gate.
- A weak or non-cryptographic digest would let an adversary who authors the scanned
  text — the same adversary this whole tool defends against — tune a *new* payload
  to collide with an already-accepted fingerprint (T-QT-02). sha256 closes that.
- `count` closes "more of the same is free": baselining two occurrences accepts two,
  not an unlimited number. A third identical occurrence is still reported (T-QT-03).
- Excluding the line number from identity means editing anything above a finding
  does not force the baseline to be regenerated — the file stays a record of a
  decision made once, rather than churn on every commit.
- Nothing is dropped: `baselined` follows the same rationale as `suppressed` and
  `low_confidence` — a finding withheld is filed under the reason it was withheld,
  never discarded, and additive with `#[serde(default)]` so `spec-ci-plugin`'s
  `JSON.parse(output) as Array<...>` contract is unaffected.

### Negative / Trade-offs

- A reviewer reading the baseline in a pull request sees `PI001 in docs/foo.md ×2`,
  not the matched text. That is a real loss of review signal, and the trade the
  user consciously chose (D-1) rather than the alternative below.
- A stale entry — one that matches nothing this run — is a live licence to
  re-introduce the finding it once accepted. It cannot be prevented structurally,
  only surfaced: a stale-entry note on stderr names the pattern and file so it can
  be pruned (T-QT-01).
- Identity is the path string exactly as the scanner reports it (minus one leading
  `./`), so a baseline must be generated and consumed by comparable invocations —
  `check .` and `check docs/foo.md` key differently unless the normalisation is
  applied on both sides, which is why it is applied on both `from_reports` and
  `apply` rather than once.

## Alternatives Considered

- **Store the payload verbatim.** Rejected — the artifact becomes a finding source
  under the scanner's own default extension set, and leaks payload text into every
  PR review that touches the baseline.
- **A non-cryptographic hash (e.g. FNV, CRC32).** Rejected — the adversary authors
  the scanned input, and a fast, non-cryptographic hash gives them a tractable
  target to tune a new payload onto an already-accepted fingerprint.
- **Include the line number in identity.** Rejected — regenerating the baseline on
  every commit that shifts lines above a finding would make the file pure churn,
  defeating its purpose as a durable record of a decision.
- **Unbounded acceptance per fingerprint (no `count`).** Rejected — accepting one
  occurrence of a pattern would silently accept an unlimited number of future
  occurrences of the identical text, which is a much larger grant than "we saw this
  once and decided it was fine."
