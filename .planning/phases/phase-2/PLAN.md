# Phase 2 Plan: Correctness — ship v0.0.3

**Milestone:** Production Readiness · **Requirements:** FIX-01…FIX-06, SCAN-08, INT-01, PERF-01
**Status:** in progress · **Branch:** `chore/milestone-production-readiness`

## Goal

Every claim already in the README becomes true. No new features.

## Tasks

- [x] **T1 — FIX-01 + FIX-02 (issues #12, #13), landed as ONE refactor.**
      Both rewrite the same `compile_patterns` → `scan_content` path, so splitting them would mean
      writing the same code twice. Introduced a `Scanner` type that owns the compiled pattern set;
      added `case_sensitive` to the pattern schema, defaulting to case-**insensitive**.
      *Result: sentence case and ALL CAPS now detected; 500-file scan 806ms → 17ms (47x), PERF-01 met.*
- [x] **T2 — FIX-06 (#42).** `--format` is now a clap `ValueEnum` with `ignore_case`.
      *Result: `--format sarif` errors with `[possible values: text, json]` instead of printing text;
      `--format JSON` produces JSON instead of silently degrading. SARIF deliberately absent from the
      enum until #5 implements a writer.*
- [ ] **T3 — FIX-03 (#14).** Per-file error isolation; report skipped files rather than aborting.
- [ ] **T4 — FIX-04 (#15).** `ignore` / `ignore-next-line` / `ignore-file`; relax the `PI\d+` ID regex; fix the README.
- [ ] **T5 — FIX-05 (#16).** `find_iter` with a per-line cap.
- [ ] **T6 — SCAN-08 (#28).** Duplicate-ID detection, `deny_unknown_fields`, `--strict-patterns`.
- [ ] **T7 — INT-01 (#18, #43).** `spec-ci-plugin` consumer fixes + release-time asset-contract smoke test.
- [ ] **T8 — PERF-01 (#29).** Criterion benchmark so the 17ms result is defended, not just observed.

## Success criteria

- `Ignore all previous instructions` is detected ✅
- 500-file scan completes under 200ms ✅ (17ms)
- A binary file in the tree does not abort the run
- `--format bogus` exits non-zero with a usage error ✅
- Both README suppression forms work as documented

## Decisions taken during execution

**Case-insensitive is the default, opt-out per pattern.** Prompt-injection payloads are natural
language; an attacker capitalising a sentence must not defeat detection. `case_sensitive: true`
exists for patterns where casing itself is the signal.

**`Scanner::new` is fallible and strict.** A pattern whose regex fails to compile now fails the run
instead of being warned about on stderr and silently dropped. A security scanner that quietly
reduces its own coverage while exiting green is worse than one that refuses to start. This closes
the "silent detection loss" half of M-01 ahead of T6.

**The free `scan_content` function was removed rather than kept as a convenience wrapper.** Keeping
it would have preserved a trap: any caller using it in a loop reintroduces the per-file compilation
bug. Nine test call sites were migrated instead.

**Exit-code collision discovered (affects Phase 4 / CLI-06, issue #25).** Clap emits **exit 2** for a
usage error, which is now what `--format sarif` returns. CLI-06 plans to use **exit 2 for
"warnings only"**, following the `spec-linter` convention. Those two meanings cannot share a code —
a CI job could not distinguish "you passed a bad flag" from "findings exist but only below the
failure threshold". This must be resolved when CLI-06 is designed, not after. Options: use exit 3
for warnings-only here, or keep 2 for warnings and remap clap's usage error. Recorded on #25.

**Known follow-up:** five patterns still carry a now-redundant inline `(?i)`. Harmless, but they
should be stripped when the pattern files are next touched (T6 or Phase 3's severity rebalance) so
the schema is the single source of truth for casing.
