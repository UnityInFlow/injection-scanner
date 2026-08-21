# State: injection-scanner

## Project Reference
See: `.planning/PROJECT.md`
**Core value:** Catch prompt injection attacks before they reach production
**Milestone:** Production Readiness — v0.0.3 + v0.1.0 (opened 2026-08-21)

## Current Phase
**Phase 2 — Correctness (v0.0.3)** · status: in progress, 4 of 8 tasks merged to `main`

CI is green again. PR #44 (`chore/milestone-production-readiness`) runs on a GitHub-hosted runner
and completed every step — fmt, clippy, test, build, smoke — picked up in 29 seconds against the
previous two 24-hour queue timeouts. The gate is restored.

**Next:** Phase 2 is underway. PR #44 is deliberately held unmerged pending an independent review of
the workflow changes (`scratchpad/opencode-phase1-review-prompt.md`) — any finding should land in the
branch, not on `main`.

### Phase status
| Phase | Goal | Status |
|---|---|---|
| 1 | Restore the Gate | ✅ Merged (PR #44) |
| 2 | Correctness — ship v0.0.3 | 4 of 8 merged (PR #51) |
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

## Blockers

**None.** #45 is resolved — see below.

## Resolved

**#45 — release pipeline (2026-08-21, option B).** `release.yml`'s three jobs moved from `[orangepi]`
to `ubuntu-latest`, and now emit a signed SLSA build-provenance attestation. The pipeline uses no org
secrets, only `GITHUB_TOKEN`, and is tag-triggered only — a fork cannot fire it. Recorded in the root
decisions log. Side benefit: on an x86_64 host the two x86_64 Linux binaries are now natively
executable, so the release smoke test actually runs them rather than checking for presence.

## Known unverified
`arc-runner-unityinflow` is believed to match zero registered runners and the org runner group is
believed to enforce `allows_public_repositories: false`. Both come from the root CLAUDE.md decisions
log; an independent reviewer's `gh api orgs/UnityInFlow/actions/runner-groups` returned 403 without
org-admin rights. The Phase 1 design is correct either way, but someone with org admin should confirm.

## Session Notes
- 2026-08-21: Phase 2 T7 / INT-01 — `spec-ci-plugin` consumer fixes (#43) and `--no-suppress` on the
  Action (#56) landed as spec-ci-plugin PR #9, CI green, awaiting merge. Four defects: unverified
  download, unversioned `/tmp` cache, three disagreeing version defaults, and PR-controlled
  suppression. Notable: `--no-suppress` support is probed via `check --help` rather than inferred
  from the version string, which decoupled this work from the v0.0.3 tag entirely. Remaining in T7:
  #18, the release-time musl asset-contract smoke test on this side.
- 2026-08-21: External review round on #44/#46. Four real defects found and fixed, one conclusion
  corrected (the "43% FP increase" was mostly correct new detection on attack fixtures). Actions
  SHA-pinned. #44 and #51 merged to `main`. #46 was auto-closed by GitHub when its base branch was
  deleted on merge — superseded by #51, no work lost.
- 2026-08-21: Phase 1 complete. CI restored via the D-02 split; run 32463308109 green on PR #44.
  Release blocker #45 discovered and filed during phase planning.
- 2026-08-21: Deep-dive audit (`docs/AUDIT-2026-08.md`) + independent verification pass. 6 critical
  and 7 high findings. 32 GitHub issues filed (#12-#43). Planning reconciled; new 4-phase milestone
  opened. Prior milestone archived to `.planning/archive/milestone-v0.0.1/`.
- 2026-04-02: Harness engineering setup complete.
- 2026-04-01: v0.0.1 released.

---
*Last updated: 2026-08-21*
