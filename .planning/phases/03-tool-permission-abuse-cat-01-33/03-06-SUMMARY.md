---
phase: 03-tool-permission-abuse-cat-01-33
plan: 06
status: complete
completed: 2026-09-02
requirements: [CAT-01, GATE-02, GATE-04, GATE-05]
---

# 03-06 — Prose CAT-01: persuasion to widen the agent's authority

## What was built

Five HIGH prose patterns extending the `tool_permission_abuse` category, completing CAT-01.
Where Plan 05's structural patterns read a *parsed key*, these read *language* — a document
arguing that the agent should have more authority than it was given.

| ID | Name | Severity | Detects |
|---|---|---|---|
| PI053 | skip-permissions-flag | HIGH | `--dangerously-skip-permissions`, `bypassPermissions`, `--yolo` and bypass-mode instructions |
| PI054 | unrestricted-permission-grant | HIGH | claims of unrestricted authority — "you have permission to run any command" |
| PI055 | skip-confirmation | HIGH | directives to stop asking — "no need to ask", "you may skip confirmation" |
| PI056 | widen-settings | HIGH | settings-file widening directives — the CVE-2025-53773 shape |
| PI057 | disable-guardrail | HIGH | imperatives to turn off a hook, guardrail, check, scanner or gate |

The category is now **8 patterns**: 3 structural (CRITICAL) + 5 prose (HIGH).

## D-12 documentation debt — discharged

Plan 05 shipped the structural half under D-12 and recorded that **this plan owed the README and
CHANGELOG deliberate-behaviour-change entries**. Both are written:

- **CHANGELOG** — an `### Added` entry for `PI050`–`PI057`, and a `### Changed` entry stating that
  a wildcard tool grant in a scanned file's own frontmatter is now CRITICAL where it previously
  produced no detection at all (the `scope: frontmatter` pass existed in the schema but no pattern
  used it, so it was inert in every shipped binary).
- **README** — a callout carrying the same behaviour change, the `spec-ci-plugin` consumer-CI
  green→red impact, the `--baseline` migration path (shipped v0.1.0), and the reason the
  `spec-linter` S005 overlap is deliberate: S005 lints a spec you wrote, in your own repo, at
  authoring time; this scanner is pointed at untrusted input, so the same grant is a lint finding
  in one context and an attack in the other.

## Measured results

| Row | Before | After | Delta |
|---|---|---|---|
| tool-permission-abuse (prose) | 0 / 7 | **7 / 7** | **+7** |
| tool-permission-abuse-structural | 5 / 5 | 5 / 5 | unchanged |
| **CAT-01 combined** | 5 / 12 | **12 / 12 (100%)** | +7 |
| **Library total** | 63 / 72 (87.5%) | **70 / 72 (97.2%)** | +7 |

Library grew 51 → **56 patterns across 6 categories**. CAT-01 went from a measured 0/12
pre-pattern baseline (D-04) to 100% across two plans, with every increment attributable: Plan 05's
+5 was structural-only with the prose row held at 0/7 as the control, and this plan's +7 is
prose-only with the structural row unchanged at 5/5.

## Self-Check

- `cargo test --locked` — **353 passed, 0 failed** (exit 0)
- `cargo clippy --all-targets --locked -- -D warnings` — clean (exit 0)
- `cargo fmt --all -- --check` — clean (exit 0)
- `check tests/corpus/clean --strict` — **0 findings** across all 20 specimens (exit 0)

The clean-corpus result is the load-bearing one for prose patterns, which are far likelier to
overreach than structural ones. Both adversarial controls stay silent: `sandbox-bypass-runbook.md`
(an operator runbook that legitimately instructs a *human* to run `--dangerously-skip-permissions`
in a disposable container) and `settings-permissions-reference.md` (third-person reference prose
naming the dangerous flags without instructing anyone to use them). The patterns separate
*persuading an agent to widen its authority* from *documentation describing the same flags*.

## Commits

- `b5a7fa9` — feat(03-06): PI053 skip-permissions-flag, PI054 unrestricted-permission-grant
- `ae9935d` — feat(03-06): PI055 skip-confirmation, PI056 widen-settings, PI057 disable-guardrail
- `21325a2` — docs(03-06): D-12 behaviour-change entries, catalogue regen, recall at 70/72

## Also cleared

`docs/DETECTION-BACKLOG.md` CAT-01 section reworked to code spans, and the stale
`README.md:271` "60 realistic payloads" corrected to 72 — the out-of-scope item Plan 05 logged in
`deferred-items.md`. That deferred item is now closed.

## Issues encountered

The executor was terminated by a **session rate limit (HTTP 429)** after completing all pattern
work, the documentation debt, the catalogue regen and the baseline regen, but before it could
commit them or write this SUMMARY. **The orchestrator committed that work as the #2070 rescue
path** (`21325a2`) and wrote this SUMMARY, after independently running every gate above against
the built binary rather than trusting the terminated agent's claims.

This is the second consecutive plan requiring orchestrator rescue. Plan 05 lost three runs to
machine sleep, a network outage and the 600s stall watchdog under a ~200 load average; this plan
lost one to a quota reset. In both cases the executor's per-task commit discipline is what made
the work recoverable — the pattern commits survived intact both times, and only the trailing
documentation step had to be rescued.
