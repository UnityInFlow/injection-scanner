---
phase: 03-tool-permission-abuse-cat-01-33
plan: 05
status: complete
completed: 2026-09-01
requirements: [CAT-01, GATE-02, GATE-04, GATE-05]
tasks_completed: 4
tasks_total: 4
---

# 03-05 — Structural CAT-01: the first shipped `scope: frontmatter` patterns

## What was built

Three CRITICAL structural patterns in a new `tool_permission_abuse` category, arming
ENG-01's structural pass in a shipped binary for the first time.

| ID | Name | Severity | Scope | Fires on |
|---|---|---|---|---|
| PI050 | wildcard-tool-grant | CRITICAL | frontmatter | a bare `*` or `Bash(*)` tool grant in a file's own frontmatter — scalar, block-sequence and JSON-manifest projections |
| PI051 | wildcard-permission-allow | CRITICAL | frontmatter | `permissions.allow` carrying a wildcard; requires the `.allow` path segment |
| PI052 | bypass-permission-mode | CRITICAL | frontmatter | `permissions.defaultMode: bypassPermissions` |

## Decisions

**D-12 — confirmed Option A by the developer on 2026-09-01.** A frontmatter-scoped `PI05x`
fires on a file's own wildcard tool grant, at **CRITICAL**.

- Justification is **provenance, not phrasing** (D-10/D-11). `spec-linter` S005 lints a spec you
  wrote, in your own repo, at authoring time. This scanner is pointed at untrusted input — a skill
  from a registry, a third-party config, a RAG document. The same `allowed-tools: *` is a lint
  finding in one context and an attack in the other; the two consumers never coincide.
- **Accepted consequence:** `spec-ci-plugin` shells out to this binary in consumer CI, so any
  consumer repo containing a wildcard tool grant in frontmatter goes from a green build to a red
  one on upgrade. This was accepted deliberately, not overlooked. It is one-way in effect: a later
  re-narrowing would itself be a second published behaviour change, and downstream CI would already
  have broken in the interval.
- **Migration path:** `--baseline`, shipped in v0.1.0.
- Rejected: **B** (ship at HIGH — does not avoid the break, since `install-hook` already blocks at
  HIGH; it only changes how the finding reads and sorts) and **C** (do not ship the structural half
  — would leave ENG-01 inert in the shipped binary, the exact ROADMAP error D-11 identifies).

> **Debt owed by Plan 06:** the README + CHANGELOG deliberate-behaviour-change entries for D-12.
> Plan 06 must not close without them.

## Measured results

**Recall — the delta is the evidence (D-04, GATE-01):**

| Row | Before | After | Delta |
|---|---|---|---|
| tool-permission-abuse-structural | 0 / 5 | **5 / 5** | **+5** |
| tool-permission-abuse (prose) | 0 / 7 | 0 / 7 | 0 — no spillover |
| **Total** | 58 / 72 (80.6%) | **63 / 72 (87.5%)** | +5 |

The prose row was re-measured in the same run and did not move, which is what proves no structural
pattern spills onto a prose payload. Library grew 48 → **51 patterns across 6 categories**.

**Per-payload detection** (all `context: frontmatter_structural`):

- `01-wildcard-allowed-tools-block-sequence.md` → PI050 CRITICAL
- `02-scalar-wildcard-tools-grant.md` → PI050 CRITICAL
- `03-json-manifest-wildcard-tools.md` → PI050 CRITICAL
- `04-permissions-allow-wildcard-settings.md` → PI051 CRITICAL
- `05-bypass-permission-mode.md` → PI052 CRITICAL

## must_haves — verified, not asserted

- **Wildcard grant reported at CRITICAL in both the scalar and array-indexed projection form**
  (D-10, trap 4) — confirmed on payloads 01, 02 and 03.
- **A `permissions.deny` entry is never reported** (D-06a, trap 3) — `tests/corpus/clean/settings-deny-list.md`
  returns no detections. PI051's required `.allow` path segment is what holds this.
- **The category is registered in `load_embedded_patterns()`** (trap 5) — `TOOL_PERMISSION_ABUSE_YAML`
  at `src/patterns/mod.rs:21`, entry at `:35`. Not dead YAML.
- **The structural pass is armed for the first time in a shipped binary** — 5/5 detected against
  Plan 01's committed 0/5 baseline.

## Self-Check

- `cargo test --locked` — **347 passed, 0 failed** (exit 0)
- `cargo clippy --all-targets --locked -- -D warnings` — clean (exit 0)
- `cargo fmt --all -- --check` — clean (exit 0)
- `check tests/corpus/clean --strict` — **0 findings** across all 20 specimens (exit 0)

The clean-corpus result is the one that mattered: `sandbox-bypass-runbook.md` (an operator runbook
that legitimately instructs a human to run `--dangerously-skip-permissions` in a disposable
container) and `narrow-allowed-tools-skill.md` (a real, correctly narrow grant) both stay silent.
The patterns are precise rather than greedy.

## Commits

- `9ffcf46` — feat(03-05): PI050 wildcard-tool-grant — first shipped scope:frontmatter pattern
- `188cbb4` — feat(03-05): PI051 wildcard-permission-allow, PI052 bypass-permission-mode
- `1f6bf38` — docs(03-05): re-pin recall at 63/72 and regenerate the code-scanning baseline

## Issues encountered

**Three executor runs were destroyed by environment failures before this plan completed.** None
were caused by the plan. Run 1 died to a machine-sleep event, run 2 to a total DNS/network outage,
run 3 to the 600-second stream watchdog while the machine sat at a load average of ~200 (a wedged
`memtrace` MCP server pegged at 100% CPU for 9 days, a 7-day-old qemu VM, and 85 stray
`claude`/`node` processes — none of them this phase's).

The fourth run completed Tasks 1–4 but was killed by the watchdog after finishing Task 4 and before
it could commit or write this SUMMARY. **The orchestrator committed that work as the #2070 rescue
path** (`1f6bf38`) and wrote this SUMMARY, after independently verifying every gate above and every
`must_have` against the built binary rather than trusting the dead agent's claims.

## Out of scope, logged not fixed

`README.md:271` still reads "`tests/corpus/attack/` holds 60 realistic payloads" — stale since
Plan 01 took the corpus to 72, and internally inconsistent with line 330 which already says 72.
Recorded in `deferred-items.md`; it predates this plan and sits outside Task 4's two-place GATE-02
scope.
