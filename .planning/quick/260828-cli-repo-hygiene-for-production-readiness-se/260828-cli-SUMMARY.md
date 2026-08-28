---
phase: quick/260828-cli
plan: 01
subsystem: infra
tags: [github, security-policy, codeowners, issue-templates, changelog, release-process]

requires: []
provides:
  - SECURITY.md with a private vulnerability reporting channel (now enabled on the repo) and an
    explicit "missed detections are public issues, not embargoed vulnerabilities" policy
  - .github/CODEOWNERS auto-requesting maintainer review on workflows/patterns/src/SECURITY.md
  - Three structured GitHub issue forms (bug_report, pattern_proposal, false_positive) plus
    config.yml disabling blank issues
  - CHANGELOG.md (Keep a Changelog 1.1.0) covering v0.0.1 through Unreleased
  - docs/RELEASE-CHECKLIST.md — the real tag-to-verified-binaries procedure
affects: [release-process, contributor-onboarding, security-response]

actuals:
  tokens: 6858
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "GitHub Issue Forms (YAML) instead of markdown issue templates, with a version field on every form because main is ahead of the released binary"
    - "CODEOWNERS documents its own limits in-file (review-request only, not a merge block) rather than implying enforcement the branch ruleset doesn't have"

key-files:
  created:
    - SECURITY.md
    - .github/CODEOWNERS
    - .github/ISSUE_TEMPLATE/config.yml
    - .github/ISSUE_TEMPLATE/bug_report.yml
    - .github/ISSUE_TEMPLATE/pattern_proposal.yml
    - .github/ISSUE_TEMPLATE/false_positive.yml
    - CHANGELOG.md
    - docs/RELEASE-CHECKLIST.md
  modified: []

key-decisions:
  - "Enabled GitHub private vulnerability reporting on UnityInFlow/injection-scanner before writing SECURITY.md (it was disabled), so the advisory URL the file links to actually works"
  - "CHANGELOG.md [Unreleased] re-derived from a fresh `git log v0.0.3..origin/main --oneline` at execution time (20 commits, not the 18 the plan was written against) after confirming PR #66 (issue #27 — reserved pattern ID gaps PI008-PI049) had merged since planning; folded its two commits into Unreleased along with the rest"
  - "Verified PR #79 (--baseline) and PR #82 (SARIF) are still OPEN via `gh pr view` before writing the CHANGELOG, and included neither"
  - "CODEOWNERS lists .github/code-scanning-baseline.json even though it doesn't exist on main yet (arrives with PR #82), with an explanatory comment, per plan instruction"

patterns-established: []

requirements-completed: [HYGIENE-01, HYGIENE-02, HYGIENE-03, HYGIENE-04, HYGIENE-05]

coverage:
  - id: D1
    description: "SECURITY.md exists, private vulnerability reporting is enabled on the repo, and the file routes crashes/ReDoS/traversal/exhaustion to the advisory URL while stating in writing that missed detections are public issues (#80, #81, docs/DETECTION-BACKLOG.md)"
    requirement: "HYGIENE-01"
    verification:
      - kind: other
        ref: "gh api repos/UnityInFlow/injection-scanner/private-vulnerability-reporting --jq '.enabled' == true"
        status: pass
      - kind: other
        ref: "./target/release/injection-scanner check SECURITY.md --all-files (exit 0)"
        status: pass
    human_judgment: false
  - id: D2
    description: ".github/CODEOWNERS assigns @hermanngeorge15 across workflows/patterns/src/SECURITY.md; four ISSUE_TEMPLATE YAML files parse, blank_issues_enabled is false, no Discussions link, and the pattern-proposal severity dropdown carries the PATTERNS.md grading criteria verbatim"
    requirement: "HYGIENE-02"
    verification:
      - kind: unit
        ref: "python3 yaml-parse + assertion script (Task 2 <verify>) — ISSUE FORMS OK"
        status: pass
      - kind: other
        ref: "./target/release/injection-scanner check .github/CODEOWNERS --all-files && check .github/ISSUE_TEMPLATE --all-files (both exit 0)"
        status: pass
    human_judgment: false
  - id: D3
    description: "CHANGELOG.md documents v0.0.1/v0.0.2/v0.0.3 (dates from the GitHub releases API published_at) and an Unreleased section derived only from commits reachable from main, excluding --baseline (#79) and SARIF (#82)"
    requirement: "HYGIENE-03"
    verification:
      - kind: other
        ref: "grep -q 'compare/v0.0.3...HEAD' CHANGELOG.md; grep -q '2026-08-22' CHANGELOG.md; manual re-read confirms neither --baseline nor SARIF appear"
        status: pass
    human_judgment: false
  - id: D4
    description: "docs/RELEASE-CHECKLIST.md names the four real release.yml jobs, the six real target triples/asset names, SHA256SUMS.txt, the real gh attestation verify command, and both do-not-change-this-back constraints"
    requirement: "HYGIENE-04"
    verification:
      - kind: other
        ref: "grep -q 'gh attestation verify' docs/RELEASE-CHECKLIST.md; grep -q 'verify-published-assets' docs/RELEASE-CHECKLIST.md"
        status: pass
    human_judgment: false
  - id: D5
    description: "Every new file returns exit code 0 (zero findings) when scanned by this repository's own built scanner at default settings, and the full cargo gate (fmt/clippy/test/build) passes"
    requirement: "HYGIENE-05"
    verification:
      - kind: other
        ref: "cargo fmt --all -- --check; cargo clippy --all-targets --locked -- -D warnings; cargo test --locked (33 tests); cargo build --release --locked — all pass"
        status: pass
      - kind: other
        ref: "injection-scanner check on all 5 new paths (SECURITY.md, CHANGELOG.md, docs/RELEASE-CHECKLIST.md, .github/CODEOWNERS, .github/ISSUE_TEMPLATE) — zero findings on each"
        status: pass
    human_judgment: false

duration: ~20min
completed: 2026-08-28
status: complete
---

# Quick Task 260828-cli: Repo Hygiene for Production Readiness Summary

**Eight new on-disk hygiene artifacts — SECURITY.md (with private vulnerability reporting freshly enabled on the repo), CODEOWNERS, three GitHub Issue Forms, a Keep a Changelog CHANGELOG.md, and a release.yml-derived RELEASE-CHECKLIST.md — closing the gap between the repo's hardened settings and the fact that none of that was written down anywhere a reporter or contributor could find it.**

## Performance

- **Duration:** ~20 min
- **Started:** 2026-08-28T08:00:00Z (approx.)
- **Completed:** 2026-08-28T08:17:18Z
- **Tasks:** 3
- **Files modified:** 8 (all new)

## Accomplishments

- Enabled GitHub private vulnerability reporting on `UnityInFlow/injection-scanner` (was disabled)
  and wrote `SECURITY.md` to point at it, with an explicit, honest policy that a missed detection
  is a public issue (#80, #81), never an embargoed vulnerability
- `.github/CODEOWNERS` auto-requests review on `.github/workflows/`, `patterns/`, `src/`,
  `SECURITY.md`, and the not-yet-present `code-scanning-baseline.json` — documented in-file as
  attention, not enforcement, since the `main` ruleset doesn't require code-owner review
- Three GitHub Issue Forms (`bug_report.yml`, `pattern_proposal.yml`, `false_positive.yml`) plus
  `config.yml` disabling blank issues and routing to the advisory URL, `SECURITY.md`, and
  `PATTERNS.md` — the pattern-proposal form's severity dropdown carries the PATTERNS.md grading
  test text verbatim, so a proposer grades against a criterion instead of picking a mood
- `CHANGELOG.md` (Keep a Changelog 1.1.0) covering v0.0.1 → v0.0.3 → Unreleased, dated from the
  GitHub releases API `published_at` (not the stale STATE.md dates), re-derived at execution time
  against `main`'s actual head (20 commits past v0.0.3, including PR #66 which merged after the
  plan was written) — confirmed neither `--baseline` (#79) nor SARIF (#82) appear anywhere, both
  still open
- `docs/RELEASE-CHECKLIST.md` — the real tag-to-verified-binaries procedure read out of
  `release.yml`: the four jobs by name, all six target triples and asset names, the version-bump
  gate rationale, post-release verification commands, the `spec-ci-plugin` consumer check, and the
  two "do not change this back" constraints (raw musl assets, `ubuntu-latest` runner)

## Task Commits

Each task was committed atomically:

1. **Task 1: SECURITY.md, and prove the whole loop end to end** - `dd884e3` (docs)
2. **Task 2: CODEOWNERS and the three issue forms** - `9781284` (docs)
3. **Task 3: CHANGELOG.md, the release checklist, and the full gate** - `840e62d` (docs)

_No separate plan-metadata commit — per the constraints, nothing under `.planning/` was staged from
this worktree; the orchestrator handles the metadata commit._

## Files Created/Modified

- `SECURITY.md` - vulnerability reporting channel, in-scope surfaces, and the bypass-is-not-a-vuln policy
- `.github/CODEOWNERS` - review-request routing for the highest-risk paths
- `.github/ISSUE_TEMPLATE/config.yml` - disables blank issues, routes contact links
- `.github/ISSUE_TEMPLATE/bug_report.yml` - structured bug report form
- `.github/ISSUE_TEMPLATE/pattern_proposal.yml` - structured pattern contribution form
- `.github/ISSUE_TEMPLATE/false_positive.yml` - structured false-positive report form
- `CHANGELOG.md` - v0.0.1 through Unreleased, Keep a Changelog 1.1.0
- `docs/RELEASE-CHECKLIST.md` - tag-to-verified-binaries procedure

## Decisions Made

- Re-ran `git log v0.0.3..origin/main --oneline` rather than trusting the plan's quoted 18-commit
  list, per the `<state_changed_since_planning>` instruction — found 20 commits (PR #66 / issue #27
  had merged), and folded the two additional commits (pattern-ID-gap fill + PI017/PI045 retune)
  into the CHANGELOG's `[Unreleased]` section
- Confirmed via `gh pr view 79/82 --json state` that both `--baseline` and SARIF are still `OPEN`
  before writing anything, rather than relying on the plan's snapshot
- Used `published_at` from `gh api repos/.../releases` for all three release dates
  (2026-04-02 / 2026-06-24 / 2026-08-22) rather than the STATE.md dates, which are off by a day in
  both directions; did not edit STATE.md, per constraints
- Dropped several `v0.0.3..main` commits from the CHANGELOG that had no user-visible effect
  (pure `docs:`/`chore:` commits, and two narrowly internal `fix:` commits — the CI tag-guard's
  `packages[0]` fix and a `serde(default)` fix on an internal field) rather than padding the file
  with entries a consumer of the CLI wouldn't act on

## Deviations from Plan

None - plan executed exactly as written. All three tasks, their preconditions, and their `<verify>`
blocks passed on the first attempt; no Rule 1-4 deviations were triggered.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required. (GitHub private vulnerability reporting was
enabled programmatically via `gh api --method PUT .../private-vulnerability-reporting` as part of
Task 1, not left as a manual step.)

## Next Phase Readiness

- The repo now has a working, discoverable vulnerability-reporting channel, code-owner review
  routing, structured issue intake, an accurate changelog, and a real release procedure — the
  hygiene layer around the settings hardened earlier in this session.
- `docs/RELEASE-CHECKLIST.md` is ready to use for the next tag (v0.0.4 or v0.1.0, per STATE.md's
  "a v0.0.4 is a choice, not a debt" framing) — nothing blocks cutting a release using it.
- No blockers. `.github/workflows/ci.yml` was not touched, per constraints.

---
*Phase: quick/260828-cli*
*Completed: 2026-08-28*
