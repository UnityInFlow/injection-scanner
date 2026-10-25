# Roadmap: injection-scanner

**No active milestone — maintenance-only** since 2026-10-23, by the scope freeze recorded when
v0.2.0 was planned: ship v0.2.0, release it, then move the ecosystem's focus to agent-memory.

## Milestones

- ✅ **v0.0.1** — first release (April 2026) — `.planning/archive/milestone-v0.0.1/`
- ✅ **Production Readiness (v0.0.3 + v0.1.0)** — shipped 2026-08-29; recall 10/60 → 56/60 —
  `.planning/archive/milestone-v0.1.0/`
- ✅ **v0.2.0 — Agent-shaped attacks** — Phases 1-5, 23 plans, released 2026-10-10; 48 → 79
  patterns, 5 → 9 categories; CAT-03 held-out recall 2/12 is the published number —
  `.planning/archive/milestone-v0.2.0/`

## What maintenance-only means here

- **Merged:** bug fixes, false-positive fixes, CI and dependency upkeep, and community pattern PRs
  that pass every gate. Merged after the release: #197 (`PI114`-`PI118`, multilingual, 84 patterns)
  and #200 (`PI079` threshold evidence, #180).
- **Not started:** new detection categories or engines. The detection gaps filed at the v0.2.0
  close-out (#164-#184) sit on the GitHub milestone **v0.4.0: detection gaps** (renamed from
  v0.3.0 on 2026-10-25), which has no plan and no date.
- **Exception, 2026-10-25:** the freeze is lifted for **packaging and docs only**. v0.3.0 is a
  distribution release (crates.io, Homebrew tap, `cargo binstall`, README top, demo), tracked in
  #211 and planned in `docs/reviews/2026-10-24-launch-plan.md`. It changes no pattern and no
  engine. The detection freeze stays until the launch stop rule in #211 has been evaluated.
- **Backlog:** `docs/DETECTION-BACKLOG.md`. The next detection milestone opens with
  `/gsd-new-milestone`, only when the freeze is lifted.
