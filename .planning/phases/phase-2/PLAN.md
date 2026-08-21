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
- [x] **T4 — FIX-04 (#15).** Three directives: `ignore` (same line), `ignore-next-line`, and
      `ignore-file` (honoured only in the first 10 lines, so a file-wide escape hatch stays visible).
      ID regex relaxed to `[A-Za-z][A-Za-z0-9_-]*` so community pattern packs can be suppressed at
      all. README rewritten to match. Also closes **M-02 / #19** — the `unwrap()` became an `expect()`
      with a test that guards it, and `src/` is now unwrap-free.
- [x] **T5 — FIX-05 (#16).** `find_iter` with a cap of 10 matches per pattern per line.
- [x] **T6 — SCAN-08 (#28).** Duplicate-ID detection (first claim wins, second reported),
      `#[serde(deny_unknown_fields)]` so a misspelled key is rejected rather than silently defaulted,
      and `--strict-patterns` to turn external-pattern warnings into failure.
- [~] **T7 — INT-01 (#18, #43, #56).** Consumer fixes done, asset-contract smoke test still open.
      `spec-ci-plugin` PR #9 (`fix/injection-scanner-consumer`, CI green, 41 tests) fixes all four:
      SHA256 verification before `chmod`/exec with a mismatch returning `fail` rather than `warn`;
      cache keyed by version *and* target triple; `DEFAULT_SCANNER_VERSION` as the single source of
      truth with tests pinning `action.yml` and the README to it, and anything below `v0.0.2` refused
      with the reason instead of 404ing; `--no-suppress` passed by default behind a new
      `allow-suppressions` input (#56). Flag support is probed from the binary (`check --help`), not
      inferred from the version string — so this did **not** need to wait on the v0.0.3 tag, and a
      repo pinned to an older release degrades with a visible note instead of dying on an
      unrecognised argument. Verified against the real v0.0.2 release *and* a local build of #55:
      the `ignore-file` payload from #56 fails the gate by default and passes only under
      `allow-suppressions: true`. Downloads moved from shell `curl` to `fetch`, removing an Action
      input interpolated into a shell command.
      *Still open: **#18**, the release-time musl asset-contract smoke test (this repo's side).*
      *Follow-up: bump `DEFAULT_SCANNER_VERSION` to v0.0.3 once #55 is merged and tagged — that is
      when `--no-suppress` starts applying by default.*
- [x] **T8 — PERF-01 (#29).** PR #58, CI green. The guard is a **ratio**, not a wall-clock bound:
      an absolute threshold could not work here, because the regressed build was 806ms — inside the
      "500 files under 1s" bound the handoff proposed, so that test would have passed on the very
      regression it existed to catch. `tests/perf_regression_test.rs` instead compares a 500-file
      scan against one pattern-set compile (currently ~1.8x, trips at 50x); a build that compiles
      per file cannot come in under 500x by construction, and both sides move with machine speed so
      it holds on any hardware. Verified by injecting the regression: 15.2s against a 1.41s budget.
      A second test asserts the same content costs the same as one file or as a hundred.
      `benches/scan.rs` adds criterion coverage of all four shapes #29 asks for: compile 2.58ms,
      one ~20k-line file 12.5ms, 500 small files 4.94ms, pathological single line 0.98ms.
      CI gains an end-to-end release-binary gate — 500 files on disk, best of three, 200ms budget —
      which covers what the ratio test structurally cannot see (the walk, file I/O). **Measured
      13ms on the hosted runner**, ~15x headroom.
      *Scope: item 2 of #29 only. Coverage gating, the FP corpus (QUAL-03, Phase 3) and fuzzing
      remain open on that issue.*

## Success criteria

- `Ignore all previous instructions` is detected ✅
- 500-file scan completes under 200ms ✅ (17ms)
- A binary file in the tree does not abort the run ✅
- `--format bogus` exits non-zero with a usage error ✅
- Both README suppression forms work as documented ✅

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
