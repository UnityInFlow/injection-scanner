---
phase: 2
requirement: ENG-02
issue: 30
status: complete
---

# Phase 2 — Recursive decoder (ENG-02, #30)

> Written retroactively at pause time. Phase 2 was executed directly from the roadmap and the
> issue; this file exists so the phase record is complete and `/gsd-resume-work` does not read a
> missing PLAN as an anomaly.

## Scope, as corrected before implementation

#30 inherited a wrong premise: `recall_test.rs` said the three encoding misses were "the base64
family". Measured, only one was. Corrected in **PR #107** before any code was written.

| Miss | Cause | In scope |
|---|---|---|
| base64 payload | genuine decoder gap | yes |
| reversed text | a reversal transform, not decoding | yes — folded in |
| fully despaced | `normalize.rs`'s documented non-goal | no |

Target therefore **58/60**, not 59/60.

## Tasks

1. `src/decode.rs` — base64/base64url, hex, percent, HTML entities, `\u`/`\x`, reversal. Bounded
   depth 3, 4KB per candidate. Report only when the DECODED text matches a pattern.
2. `decode_chain` on `ScanMatch`, additive with a default.
3. Fifth scan pass, deduplicated against the earlier passes; `matched_text` keeps ORIGINAL bytes.
4. Update the pinned recall counts and publish the number in the README.
