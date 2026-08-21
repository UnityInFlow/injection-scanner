# Phase 1 Plan: Restore the Gate

**Milestone:** Production Readiness (v0.0.3 + v0.1.0)
**Requirements:** CI-01, CI-02 · **Issue:** #17
**Status:** in progress · **Branch:** `chore/milestone-production-readiness`

## Goal

A working, trustworthy CI signal on every pull request, including from forks. This phase is a hard
gate — no Phase 2-4 work merges until it is green.

## Current state

CI has been non-functional since 2026-06-24. Two consecutive runs on `main` were cancelled after
`24h0m2s` — queued forever, never picked up.

```
completed  cancelled  ...  CI  main  push  24h0m2s  2026-06-24
completed  cancelled  ...  CI  main  push  24h0m2s  2026-06-24
```

Cause: `ci.yml` matrixes over `[arc-runner-unityinflow, orangepi]`. `arc-runner-unityinflow` matches
zero registered runners, and the repo is public under `allows_public_repositories: false`, so the
`orangepi` leg cannot run either.

## Tasks

- [x] **T1** — Rewrite `ci.yml`: GitHub-hosted runner, secretless, `permissions: contents: read`,
      running fmt + clippy + test + build. No self-hosted label anywhere in the file.
- [x] **T2** — Remove the stale `arc-runner-unityinflow` references from `release.yml` comments so
      the file no longer advertises a runner that cannot be scheduled.
- [x] **T3** — Add `--locked` to the CI cargo invocations so `Cargo.lock` drift fails loudly.
- [ ] **T4** — Push the branch, open a PR, confirm CI runs to completion.
- [ ] **T5** — Confirm no self-hosted job is reachable from any fork-firable trigger.

## Success criteria

- A pull request runs fmt, clippy, test and build to completion in under 10 minutes
- No self-hosted job is reachable from `pull_request` or any other fork-firable trigger
- No secret is exposed to a fork-triggered workflow
- `arc-runner-unityinflow` appears nowhere in `.github/`

## Deliberate deviation from CLAUDE.md

Both CLAUDE.md files said "never use `ubuntu-latest`". This phase uses it for fork-facing CI only.
That is the sanctioned D-02 exception already granted to `spec-ci-plugin`; it is recorded in the
root decisions log (August 2026 row) and in this repo's CI section. Release work stays self-hosted.

## BLOCKER discovered during planning — release pipeline is also dead

`release.yml` runs all three jobs on `[orangepi]`. Under `allows_public_repositories: false` on a
public repo, **those jobs cannot be scheduled either**. The v0.0.2 release succeeded on 2026-06-24
only because the runner group's setting had drifted to `true`; enforcement back to `false` landed in
July 2026 (root CLAUDE.md, OPS-02). So **v0.0.3 cannot currently ship**, and this phase does not fix
that — it only fixes the PR gate.

Relevant fact for the decision: injection-scanner's release needs a Rust toolchain, zig,
cargo-zigbuild and `GITHUB_TOKEN`. It uses **no org secrets** — no signing key, no Sonatype
credentials, no npm token. That is unlike every other repo the self-hosted policy was written for.

Options, to be decided before Phase 2 ships:
- **A — private window.** Flip the repo private, tag, let `orangepi` build, flip public. Proven by
  prompt-vc. Preserves the policy exactly; manual and easy to get wrong under time pressure.
- **B — extend the exception to release.** Since no org secret is involved, run the release on a
  GitHub-hosted runner too. Simplest, and x86_64-native for the most-downloaded target. Widens the
  sanctioned exception from "fork CI" to "fork CI + secretless release" — a real policy change.
- **C — workflow-restricted runner group.** Permit this repo on a dedicated group. Cleanest, needs
  org-admin access.

Tracked as issue #44.
