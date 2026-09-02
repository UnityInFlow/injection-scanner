# ADR-004: `relaxed_pattern` — the false-positive control as a test, not a ritual

Date: 2026-09-02
Status: Accepted

## Context

Every pattern in this library is a claim about what the scanner catches. A widened
pattern is simultaneously a claim about what it does *not* catch, and that second
claim has historically gone unverified.

`PATTERNS.md` has long asked contributors to "name the thing keeping it narrow, then
break it and confirm the corpus goes red". Two v0.1.0 reviews showed that instruction
being followed in form but not in substance:

- **#95.** PI021's disclosure arms depend on requiring the possessive — `your system
  prompt`, not `the system prompt`. Relaxing it to `(?:your|the)` left the entire clean
  corpus green. The one document that should have caught it, `mcp-manifest.json`,
  survived only because "Returns the system prompt currently configured" uses a plural
  verb that fails to match `return`. The control held by accident, not by design.
- **#97.** PI018's precedence arm produced six HIGH findings on ordinary configuration
  prose in its first draft. HIGH is the severity `install-hook` blocks commits at.

In both cases the narrowing was real but nothing in CI proved it. The evidence lived in
a PR description, which no future change re-runs. Meanwhile the clean corpus is fifteen
files — a sample far too small to be the sole false-positive gate for a pattern library
that now spans six categories.

This ADR is required on the `pr-artifacts` trigger for a change to the YAML pattern
format: `Pattern` gains a field, and `pattern_policy_test.rs` gains a rule about when
that field is mandatory.

## Decision

A pattern may declare `relaxed_pattern`: a deliberately widened variant of its own
`pattern` with the narrowing removed.

It is never loaded into the live scanner. Both `load_embedded_patterns` and
`load_external_patterns` build the `Scanner` from `pattern` only. Its sole consumer is
`tests/pattern_relaxed_control_test.rs`, which swaps it in for `pattern` and asserts a
two-sided property:

- the **shipped** `pattern` must NOT match this pattern's `counter_example`;
- the **relaxed** variant MUST match it.

Together those turn "the narrowing is load-bearing" from an assertion into a failing
test whenever it stops being true. That is GATE-05.

It is **required for `PI050` and above** and optional below (D-09). The 48 pre-existing
patterns stay exempt: retrofitting them is worthwhile but is not a precondition for
shipping a new category, and `pattern_policy_test::every_pi05x_pattern_carries_a_relaxed_pattern`
enforces the boundary so the exemption cannot silently widen.

It is a first-class schema field rather than a tag or a comment, so that
`deny_unknown_fields` rejects a typo in a community pattern file instead of silently
dropping the control — a silently-dropped control is precisely the failure this ADR
exists to end.

It is **not** rendered into `docs/PATTERN-CATALOGUE.md` (D-08a). The shipped regex is
already published there in the Regex details block, so withholding the relaxed form
discloses nothing new; the field describes what the scanner deliberately does not
detect, which is test scaffolding rather than a detection to document.

## Consequences

### Positive

- The false-positive control is re-verified on every CI run, by machine, forever —
  rather than at review time, by a human, once.
- A later "harmless" widening that quietly dissolves the narrowing fails the build
  instead of shipping. This is the property #95 and #97 both lacked.
- The `counter_example` field gains teeth. It was already binding via
  `pattern_example_test` (the counter-example must not match), but nothing previously
  proved the counter-example was *near* the boundary rather than trivially far from it.
  A `relaxed_pattern` that catches it proves the specimen sits exactly on the line.
- It scales past the fifteen-file clean corpus. The control travels with the pattern
  instead of depending on some corpus document happening to contain the right sentence.

### Negative / Trade-offs

- Every `PI050+` pattern now carries a second regex to write and maintain. Author cost
  is real, and a careless `relaxed_pattern` is a test that passes without proving
  anything.
- The field is dead weight at runtime: parsed, held in memory, and never matched
  against. The cost is small and the alternative (a parallel test-only file) was worse.
- It proves the narrowing against **one** specimen, the `counter_example`. It is a
  boundary probe, not a proof of general correctness — GATE-03's ~1,300-file third-party
  sweep remains the breadth gate, and CR-01 in this very phase is the reminder: all
  three affected patterns had passing GATE-05 pairings while still firing on
  prohibitions no `counter_example` had thought to encode.

## Alternatives Considered

**Keep the manual discipline, write it more firmly in `PATTERNS.md`.** Rejected: it was
already written there, and #95 and #97 are what following it produced. An instruction
that CI cannot check degrades into a PR-description ritual.

**Grow the clean corpus instead.** Rejected as a substitute, adopted as a complement.
Adding a specimen that catches a specific over-widening is the correct move and was done
for both #95 and #97 (`clean/prompt-tooling-docs.md`, `clean/config-precedence.md`). But
a corpus document only defends the sentences someone thought to write down, and it
defends them globally rather than pinning the property to the pattern that depends on
it. The two mechanisms answer different questions.

**Express the widening as a test-only fixture file keyed by pattern ID.** Rejected: it
puts the control at arm's length from the thing it controls. A contributor editing a
regex sees the `relaxed_pattern` on the adjacent line; they would not reliably find a
fixture in another directory, and the pairing would rot.

**Make it mandatory for all 79 patterns immediately.** Rejected as scope. Retrofitting
48 patterns is a separate piece of work with its own review burden, and blocking a new
category on it would have delayed CAT-01 indefinitely. The policy test pins the
`PI050+` boundary so the exemption is explicit and cannot drift.
