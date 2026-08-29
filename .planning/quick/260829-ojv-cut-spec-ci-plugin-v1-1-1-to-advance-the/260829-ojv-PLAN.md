---
id: 260829-ojv
slug: cut-spec-ci-plugin-v1-1-1-to-advance-the
date: 2026-08-29
mode: quick
status: in-progress
---

# Cut spec-ci-plugin v1.1.1 — make the v0.1.0 default actually reach consumers

Work happens in the sibling repo `04-spec-ci-plugin`; artifacts are tracked here, where the
milestone lives.

## Why

`spec-ci-plugin` `main` (`08cb520`) defaults to injection-scanner `v0.1.0`. Its Marketplace
consumers pin `@v1`, which still points at `5adc903` — one commit behind, and that one commit is
the entire point. Until `v1` moves, every `@v1` consumer still downloads injection-scanner
`v0.0.3`, the 10/60-recall build. The release is published but inert.

## Mechanics, read out of `.github/workflows/release.yml`

- `v1` is a **plain moving git tag**, never a Release object. The workflow says so in capitals;
  creating a `v1` Release would break immutable releases.
- The mover fires on `release: published` and force-pushes `v1` at the published `vX.Y.Z`.
- It also accepts `workflow_dispatch` with a `tag` input, for exactly the gap where main has
  moved but no release has been cut.
- Runs on `ubuntu-latest` deliberately — this repo is public, so no self-hosted job can be
  scheduled at all. Do not "restore" a self-hosted label.
- **History to respect:** the three commits before v1.1.0 are all fixes to this mover, and at
  v1.0.0 it queued on an offline fleet and `v1` had to be moved by hand. Do not assume it worked —
  verify `v1` actually moved.

## Task 1 — Version bump

**Files:** `package.json`, `dist/index.js`
**Action:** `1.1.0` → `1.1.1`. Patch, not minor: no input, output or behaviour changes — only the
default value of an existing input. Rebuild `dist/` (tracked, and gated by a dist-diff check).
**Verify:** `npm run lint`, `npm test`, `npm run format`, `npm run build` all clean.
`tests/self-version.test.ts` pins `package.json`'s major against the `@v1` the README advertises —
staying on major 1 keeps that true.
**Done:** one atomic commit on a branch, PR, rebase-merge. This repo has zero merge commits.

## Task 2 — Tag and publish the Release

**Files:** none
**Action:** annotated tag `v1.1.1` on the merged main; create the GitHub Release, which is what
fires the mover.
**Verify:** the `Release` workflow runs and succeeds.
**Done:** Release published.

## Task 3 — Verify `v1` actually moved (the whole point)

**Files:** none
**Action:** re-fetch `v1` and confirm it points at the `v1.1.1` commit, not `5adc903`.
**Verify:** `git rev-list --count v1..origin/main` is 0, and the `v1` tree's
`DEFAULT_SCANNER_VERSION` reads `v0.1.0`. If the mover did not run, dispatch it manually with the
`tag` input rather than force-pushing by hand.
**Done:** an `@v1` consumer resolves to code that downloads injection-scanner v0.1.0.

## Constraints

- **Never create a Release object for `v1`.**
- Force-pushing `v1` is the mechanism, not a mistake — but let the audited workflow do it.
- No merge commits.
