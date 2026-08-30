---
phase: 2
requirement: ENG-02
issue: 30
status: complete
shipped: 2026-08-30
---

# Phase 2 complete — recursive decoder

**PR #108.** Closed #30, and #6 and #7 with it. Recall **56/60 → 58/60**, published.

## Verification

- 331 tests (was 313)
- Sweep: 12 new findings, **0 lost**; all in the corpus, the decoder's tests, or docs quoting
  payloads. The documented repo sweep still reports only the two pre-existing
  `PATTERN-CATALOGUE.md` findings, identical under v0.1.0.
- Clean corpus at zero. Perf +3.3% (516ms → 533ms).

## Three things found by measuring, all of which would have shipped silently

1. **A panic that 16 green unit tests missed.** `tail[..12]` sliced at a fixed byte offset in the
   HTML-entity scanner — a crash on any file with a multi-byte char near an `&`; a `·` in this
   repo's own source was enough. Found only by running the binary over the repo. **Unit tests do
   not substitute for the sweep.**
2. **Reversal is an involution.** Recursing on it produced `reversed -> reversed -> base64` for
   what is simply base64. Restricted to top level.
3. **Reversal was 137ms of a 143ms regression** — 84% of the cost for one payload in sixty, because
   every line's reversal was handed to all 48 patterns. A generic function-word gate on the
   reversed text cut the overhead from 28% to 3.3%. Generic words only, never payload vocabulary:
   keying it on `ignore` would mean a new pattern silently needs a decoder change to be reachable.

Also: `tests/decode_test.rs` needed `ignore-file` — the decoder makes its own fixtures visible for
the first time, which is the tool correctly detecting its own test data.
