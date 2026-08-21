# State: injection-scanner

## Project Reference
See: `.planning/PROJECT.md`
**Core value:** Catch prompt injection attacks before they reach production
**Milestone:** Production Readiness — v0.0.3 + v0.1.0 (opened 2026-08-21)

## Current Phase
**Phase 1 — Restore the Gate** · status: ready for planning

Phase 1 is a hard gate. CI has been non-functional since 2026-06-24 (two 24-hour queue timeouts),
so there is currently no automated test gate on this repository. **No Phase 2-4 work merges until
Phase 1 is green.**

### Phase status
| Phase | Goal | Status |
|---|---|---|
| 1 | Restore the Gate | Ready for planning |
| 2 | Correctness — ship v0.0.3 | Blocked on Phase 1 |
| 3 | Signal Quality | Blocked on Phase 2 |
| 4 | Integration — ship v0.1.0 | Blocked on Phase 3 |

## Releases
- **v0.0.1** (2026-04-01): 30 patterns, 5 categories, text/JSON output, inline suppression, stdin mode
- **v0.0.2** (2026-06-24): 6 target-triple binaries + SHA256SUMS. Consumed by `spec-ci-plugin`.

## Open decisions carried into this milestone

**CI policy (2026-08-21).** Public/fork CI runs on a **GitHub-hosted runner**, secretless with
`contents: read`. This is a deliberate, scoped exception to the "never use ubuntu-latest" rule in
both CLAUDE.md files, matching the D-02 split already sanctioned for `spec-ci-plugin` in July 2026.
Rationale: the org runner group enforces `allows_public_repositories: false`, so no self-hosted job
can run on this public repo at all — and the private-window alternative would deny CI to every fork
PR, which kills the community pattern contributions `PATTERNS.md` is built around. All secret-bearing
and release work stays self-hosted on `[orangepi]`, reachable only from tag push / `release: published`.

**Scope (2026-08-21).** "Production ready" is defined as v0.0.3 + v0.1.0. The agentic categories
(`PI050`–`PI079`) that differentiate this tool are explicitly deferred to v0.2.0 — they depend on the
frontmatter engine, and shipping them on top of a scanner that misses sentence case would be building
on sand.

## Known unverified
`arc-runner-unityinflow` is believed to match zero registered runners and the org runner group is
believed to enforce `allows_public_repositories: false`. Both come from the root CLAUDE.md decisions
log; an independent reviewer's `gh api orgs/UnityInFlow/actions/runner-groups` returned 403 without
org-admin rights. The Phase 1 design is correct either way, but someone with org admin should confirm.

## Session Notes
- 2026-08-21: Deep-dive audit (`docs/AUDIT-2026-08.md`) + independent verification pass. 6 critical
  and 7 high findings. 32 GitHub issues filed (#12-#43). Planning reconciled; new 4-phase milestone
  opened. Prior milestone archived to `.planning/archive/milestone-v0.0.1/`.
- 2026-04-02: Harness engineering setup complete.
- 2026-04-01: v0.0.1 released.

---
*Last updated: 2026-08-21*
