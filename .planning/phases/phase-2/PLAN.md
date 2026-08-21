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
- [x] **T3 — FIX-03 (#14).** Per-file error isolation in the directory walk; skips reported on stderr
      with a human-readable reason, plus a summary line. An explicitly named unreadable file still
      errors — skipping is for *walks*, not for a file the user asked for by name.
- [ ] **T4 — FIX-04 (#15).** `ignore` / `ignore-next-line` / `ignore-file`; relax the `PI\d+` ID regex; fix the README.
- [ ] **T5 — FIX-05 (#16).** `find_iter` with a per-line cap.
- [ ] **T6 — SCAN-08 (#28).** Duplicate-ID detection, `deny_unknown_fields`, `--strict-patterns`.
- [ ] **T7 — INT-01 (#18, #43).** `spec-ci-plugin` consumer fixes + release-time asset-contract smoke test.
- [ ] **T8 — PERF-01 (#29).** Criterion benchmark so the 17ms result is defended, not just observed.

## Success criteria

- `Ignore all previous instructions` is detected ✅
- 500-file scan completes under 200ms ✅ (17ms)
- A binary file in the tree does not abort the run ✅
- `--format bogus` exits non-zero with a usage error ✅
- Both README suppression forms work as documented

## External review findings, addressed (2026-08-21)

An independent review of PR #44/#46 (`scratchpad/opencode-pr-review-report.md`) found four real
defects. All confirmed by reproduction and fixed:

1. **PI012 matched this tool's own suppression syntax.** Case-insensitive `INJECT` matched the
   "inject" inside `<!-- injection-scanner:ignore PI001 -->`, so using the documented suppression
   feature raised a HIGH finding. Fixed with **word boundaries**, not the reviewer's suggested
   `case_sensitive: true` — that would have fixed the symptom while reintroducing a lowercase
   bypass (`<!-- inject: … -->`). `\bINJECT\b` no longer matches "injection" but still matches
   "INJECT:" and "inject:".
2. **An unreadable subdirectory still aborted the walk.** The first cut of FIX-03 isolated per-*file*
   read errors but left `?` on `fs::read_dir`, so one permission-denied subdir killed the scan — the
   same defect one level up. Now skipped and reported.
3. **Strict-by-default was premature for external patterns.** One malformed YAML in a `--patterns`
   directory aborted every scan. Embedded patterns stay strict (compile-time constants, covered by a
   CI test); external patterns are now lenient with loud per-pattern warnings, until
   `--strict-patterns` (#28) makes it opt-in.
4. **No false-positive regression guard.** Four new tests pin the PI012 behaviour in both
   directions, the subdirectory case, and the malformed-external-pattern case.

**Where the review overstated.** It reported a "43% false-positive increase (58 → 83)". After the
PI012 fix the delta is 58 → 74, and the remainder is concentrated in `examples/` — which are
**attack fixture files**. The newly-firing patterns there are PI004, PI005, PI006, PI013, PI014,
PI020, PI023, PI032: real payloads written in sentence case that the scanner previously *missed*.
That is the fix working, not noise. README is at parity with `main` (13 = 13). Genuine new prose
false positives do exist — PI003 on "You are now ready to proceed", PI014 on "The developer wants
you to test this feature", PI034 on "In this hypothetical scenario where you can find the docs" —
but those are weak patterns, not a casing problem, and they are Phase 3's job (context classifier
QUAL-01 and severity rebalance QUAL-02).

**Also corrected:** the review recorded the 806ms → 17ms figure as "hardware differs" against its own
86ms baseline. Re-measured against `origin/main` on an identical 500-file corpus: main 0.78s user /
1.22s wall, fix 0.01s user / 0.02s wall. The original figure stands; the reviewer used a smaller corpus.

**Still open from the review:** Actions are tag-pinned, not SHA-pinned, and
`dtolnay/rust-toolchain@stable` is a moving branch ref. Graded NEEDS-HARDENING, not a blocker.
Belongs on PR #44.

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

**JSON envelope deferred — downstream contract preserved (audit L-02).** FIX-03 wanted to surface
skipped files in the report. The obvious shape is an envelope: `{"reports": [...], "skipped": [...]}`.
That would have **broken `spec-ci-plugin` immediately** — it does
`JSON.parse(output) as Array<...>` and reads `reports[0]`. The top-level array is therefore
preserved and skips go to stderr. A JSON envelope is deferred to v0.1.0, where it can ship as a
documented breaking change coordinated with tool 04. Worth noting this was caught by writing the
test first: the test asserted an envelope, and checking the consumer before implementing is what
revealed the trap.

**Exit-code collision discovered (affects Phase 4 / CLI-06, issue #25).** Clap emits **exit 2** for a
usage error, which is now what `--format sarif` returns. CLI-06 plans to use **exit 2 for
"warnings only"**, following the `spec-linter` convention. Those two meanings cannot share a code —
a CI job could not distinguish "you passed a bad flag" from "findings exist but only below the
failure threshold". This must be resolved when CLI-06 is designed, not after. Options: use exit 3
for warnings-only here, or keep 2 for warnings and remap clap's usage error. Recorded on #25.

**Known follow-up:** five patterns still carry a now-redundant inline `(?i)`. Harmless, but they
should be stripped when the pattern files are next touched (T6 or Phase 3's severity rebalance) so
the schema is the single source of truth for casing.
