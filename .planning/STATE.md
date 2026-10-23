---
gsd_state_version: 1.0
milestone: none
milestone_name: maintenance-only
status: maintenance
stopped_at: "v0.2.0 released 2026-10-10 and archived 2026-10-23 to .planning/archive/milestone-v0.2.0/. No active milestone: injection-scanner is maintenance-only by the scope freeze, and the ecosystem focus is agent-memory v0.1.0."
last_updated: "2026-10-23T00:00:00.000Z"
state_head: 2f672c8
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# State: injection-scanner

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-10-23)
**Core value:** Catch prompt injection attacks before they reach production
**Current focus:** none here — maintenance-only. Ecosystem focus is agent-memory v0.1.0.

> The full v0.2.0 state (341 lines, including "Four things from Phase 5 worth not rediscovering"
> and the standing gates) is archived at `.planning/archive/milestone-v0.2.0/STATE.md`. Read it
> before opening the next milestone.

## Shipped

**v0.2.0 — Agent-shaped attacks**, released 2026-10-10. Structural frontmatter engine (#32),
recursive decoder (#30), and three categories: tool & permission abuse (#33), MCP &
tool-description poisoning (#34), persistence & lifecycle hijack (#35). 79 patterns at release.

## Since the release

- **#197 merged 2026-10-23** (community, BasilZafar11): `PI114`-`PI118` multilingual evasion.
  Library is **84 patterns / 9 categories**. Development recall **105/115 (91.3%)**, up from
  103/115. The CAT-03 held-out number is unchanged at **2/12** and is still the published one.
- **#200 merged 2026-10-23** (community, BasilZafar11): independent evidence and a threshold
  sensitivity test for `PI079` (closes #180). Regex and MEDIUM grade unchanged; remediation now asks
  the reviewer to confirm the user authorized the key. See `docs/PI079-EVIDENCE.md`.
- CI upkeep: scanner caching in tests, PERF-01 budget 200 → 400 ms for the larger library (#188).

## Open work

- Detection gaps and debt from the v0.2.0 close-out: #164-#184 on GitHub milestone **v0.3.0**
  (unplanned). The one P1 is #176 (download-then-run lifecycle hook, 4/4 held-out structural misses).
- Live list: `gh issue list --repo UnityInFlow/injection-scanner`. It beats this file.

## Session Continuity

Last session: 2026-10-23 — merged #197 and #200, archived v0.2.0.
Resume with: `gh pr list` for contributor PRs; otherwise no planned work in this repo.
