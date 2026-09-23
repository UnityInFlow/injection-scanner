---
phase: 260923-dbv
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - CLAUDE.md
autonomous: true
requirements: [DOC-STATUS-01]

estimate:
  tokens: 18000
  raw_tokens: 18000
  tasks: 2
  confidence: low

must_haves:
  truths:
    - "A reader opening CLAUDE.md sees v0.1.0 (2026-08-29) named as the latest release, not a two-versions-stale one."
    - "A reader sees the v0.2.0 'Agent-shaped attacks' milestone named as current, with Phase 4 (#34) as the phase in flight."
    - "A reader is told CI is green and is the merge gate, and is pointed at the existing '## CI / Self-Hosted Runners' section before touching any workflow."
    - "A reader is told how to re-verify the block (gh release list, gh run list --branch main) and when it was last verified (2026-09-23)."
    - "The ecosystem-phase marker on line 9 ('**Phase:** 1') is byte-identical to its pre-change value."
    - "No file other than CLAUDE.md is modified, and no compiled behaviour, test or pattern changes."
  artifacts:
    - CLAUDE.md
  key_links:
    - "Status block -> '## CI / Self-Hosted Runners' section (pointer, not a duplicate of the policy)"
    - "Status block -> .planning/ROADMAP.md + .planning/STATE.md (live milestone source of truth)"
    - "Status block -> .planning/archive/milestone-v0.1.0/ (where the superseded milestone went)"
---

<objective>
Reconcile the `## Status` block in `03-injection-scanner/CLAUDE.md` with the live repository, and
give it the same anti-drift guard the root ecosystem `CLAUDE.md` carries.

Purpose: the block currently announces a release that is two versions behind, names a milestone
that was archived on 2026-08-29 as current, and asserts that CI is broken and blocks all merges.
The last claim is actively harmful — it is false (the last eight runs on `main` succeeded, and
four PRs merged through green CI in the last two days) and it tells a reader to stop working.

Output: a rewritten `## Status` block and one corrected parenthetical in the CI section. Nothing
else in the file, the repo, or `.planning/` moves.
</objective>

<execution_context>
@$HOME/.claude/gsd-core/workflows/execute-plan.md
@$HOME/.claude/gsd-core/templates/summary.md
</execution_context>

<context>
@CLAUDE.md
@.planning/ROADMAP.md
@.planning/STATE.md
</context>

<facts_verified_2026_09_23>
These were verified against the live repo on 2026-09-23 by the planner. Do NOT re-run `gh` to
re-litigate them — use them as given. (You may quote them; you may not silently "correct" them.)

- Releases, newest first: **v0.1.0 (2026-08-29, marked Latest)**, v0.0.3 (2026-08-22),
  v0.0.2 (2026-06-24), v0.0.1.
- Live milestone: **v0.2.0 — Agent-shaped attacks**, opened 2026-08-30 (`.planning/ROADMAP.md`
  line 1 and its header). Five phases. Phases 1-3 are `[x]`: structural frontmatter engine
  (ENG-01, #32), recursive decoder (ENG-02, #30), tool & permission abuse (CAT-01, #33).
  **Phase 4 — MCP & tool-description poisoning (CAT-02, #34)** is current; `.planning/STATE.md`
  records all 7 of its plans complete, awaiting the phase-complete marker.
- The superseded milestone is archived at `.planning/archive/milestone-v0.1.0/`; that directory
  exists. Its audit document `docs/AUDIT-2026-08.md` still exists and is still worth pointing at.
- CI is **green**: the last 8 runs on `main` succeeded on both the `CI` and `Code scanning`
  workflows, including today. PRs #143, #144, #138, #146 and #147 merged through it in the last
  two days.
- `docs/DETECTION-BACKLOG.md` and `TODO.md` both still exist — those two pointers on the current
  line 18 are still correct and should survive the rewrite.
</facts_verified_2026_09_23>

<trap_do_not_fall_for_this>
**Line 9 reads `**Phase:** 1 | **Stack:** Rust | **Distribution:** pre-built binaries + Homebrew`.**

That "Phase 1" is the **ecosystem build phase** from the root `../CLAUDE.md` 20-tool table —
injection-scanner is a Phase 1 tool in a four-phase ecosystem build order. It is correct, it is
current, and it has nothing to do with `.planning/ROADMAP.md` phase numbering.

Two different numbering systems collide on the word "Phase" in this file. Confusing them is the
single most likely way to break this task. **Line 9 must come out of this change byte-identical.**
A verify gate below asserts exactly that.
</trap_do_not_fall_for_this>

<tasks>

<task type="tracer">
  <name>Task 1: Rewrite the `## Status` block against the verified facts</name>
  <files>CLAUDE.md</files>
  <action>
Read CLAUDE.md first to get the exact current wording and confirm line numbers — the block spans
from the `## Status` heading through the blank line before `## Reference Documents` (currently
lines 11-21). Replace the three stale paragraphs inside that block (currently at line 13, lines
15-18, and line 20) with new prose. Keep the `## Status` heading itself. Touch nothing above line
11 and nothing at or below the `## Reference Documents` heading.

Use `Edit` with a scoped `old_string`, not `Write` — CLAUDE.md is an existing 150-line file and a
whole-file rewrite would put every other section at risk.

The replacement block must carry these four elements, in this order, written as ordinary prose in
the voice of the surrounding document (short bold lead-ins, one idea per paragraph):

1. **Release state.** Name v0.1.0 (2026-08-29) as the latest release. Listing the prior tags
   (v0.0.3 2026-08-22, v0.0.2 2026-06-24, v0.0.1) as history is fine and useful; presenting any of
   them as the current shipped state is the bug being fixed here. Assert nothing about what
   downstream consumers pin — that was not verified today and must not be invented.

2. **Milestone state.** Name the v0.2.0 "Agent-shaped attacks" milestone (opened 2026-08-30) as
   current. Say that Phases 1-3 are complete and that Phase 4 (MCP & tool-description poisoning,
   CAT-02, #34) is the one in flight. Keep the three live-planning pointers that exist today
   (`.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/STATE.md`) and the two that
   follow them (`docs/DETECTION-BACKLOG.md`, `TODO.md`). Point at
   `.planning/archive/milestone-v0.1.0/` as where the superseded milestone went, and keep the
   pointer to `docs/AUDIT-2026-08.md`. Refer to the superseded milestone **by its archive path
   only** — do not restate its name, so that the negative gate below stays unambiguous.

3. **CI state.** State that CI is green and that it is the merge gate every PR goes through.
   Immediately qualify it so the reader does not conclude CI is unconstrained: the GitHub-hosted
   runner is deliberate, and `## CI / Self-Hosted Runners` further down this same file is the
   binding policy to read before editing any workflow. **Point at that section; do not restate
   its content.** Duplicating the `allows_public_repositories` / `arc-runner-unityinflow` rules
   here creates a second copy that can drift out of sync with the first — which is the exact class
   of failure this whole task exists to repair. Do not modify that section in this task.

4. **Anti-drift guard**, as a blockquote, modelled on the "Verify status before planning from it"
   warning in the root `../CLAUDE.md`. It must: say this block has drifted before and briefly how
   (a release two versions stale, an archived milestone presented as current); name the two
   commands that settle it — `gh release list --repo UnityInFlow/injection-scanner` and
   `gh run list --branch main`; state that the live answer beats this file whenever the two
   disagree; and end with **Last verified: 2026-09-23**.

Scope discipline for this task: CLAUDE.md is the only file you may open for writing. Do not edit
`.planning/ROADMAP.md`, `.planning/STATE.md`, `.planning/REQUIREMENTS.md`, the root `../CLAUDE.md`,
or any file under `src/`, `patterns/`, `tests/` or `.github/`. This change compiles nothing and
tests nothing — there is no source change to validate.
  </action>
  <verify>
    <automated>cd "$(git rev-parse --show-toplevel)" && BLOCK=$(sed -n '/^## Status$/,/^## Reference Documents$/p' CLAUDE.md) && echo "--- negative gate (expect 0) ---" && printf '%s\n' "$BLOCK" | grep -Eic 'current milestone: production readiness|dead since|hard gate|v0\.0\.2 shipped' ; true</automated>
    <automated>cd "$(git rev-parse --show-toplevel)" && BLOCK=$(sed -n '/^## Status$/,/^## Reference Documents$/p' CLAUDE.md) && for s in 'v0.1.0' 'v0.2.0' '2026-08-29' '2026-09-23' 'gh release list' 'gh run list' 'CI / Self-Hosted Runners' '.planning/archive/milestone-v0.1.0/'; do printf '%s\n' "$BLOCK" | grep -qF "$s" && echo "OK   $s" || echo "MISS $s"; done</automated>
    <automated>cd "$(git rev-parse --show-toplevel)" && echo "--- line 9 untouched (expect 0) ---" && git diff -U0 -- CLAUDE.md | grep -Fc '**Stack:** Rust | **Distribution:**' ; true</automated>
    <automated>cd "$(git rev-parse --show-toplevel)" && echo "--- CI policy tokens untouched (expect 0) ---" && git diff -U0 -- CLAUDE.md | grep -Ec '^[-+].*(ubuntu-latest|allows_public_repositories|arc-runner-unityinflow|attestation|id-token)' ; true</automated>
  </verify>
  <done>
- The negative gate prints `0`.
- The presence gate prints `OK` for all 8 tokens and `MISS` for none.
- The line-9 gate prints `0` — `**Phase:** 1 | **Stack:** Rust | ...` appears on neither side of the diff.
- The CI-policy gate prints `0` — no line carrying a runner/permissions token was added or removed.
- `v0.0.2` may still appear in the block, but only as a historical tag in a release list, never as the current shipped state.
  </done>
</task>

<task type="auto">
  <name>Task 2: Re-date the CI section's revision note and confirm the diff is docs-only</name>
  <files>CLAUDE.md</files>
  <action>
One residual pointer at an archived milestone survives outside the Status block. Line 103 opens the
`## CI / Self-Hosted Runners` section with a parenthetical revision note that attributes the
revision to "Phase 1, this milestone". Read that line to get its exact wording. "This milestone"
now resolves to v0.2.0, which is wrong — that revision was made on 2026-08-21, during the milestone
that is now archived at `.planning/archive/milestone-v0.1.0/`.

Edit **only that parenthetical**, so it attributes the revision to the milestone now archived at
that path rather than to "this milestone". Keep the date 2026-08-21, keep the rest of the sentence,
and keep the reference to the August 2026 row in the root CLAUDE.md decisions log.

Everything else in that section stays byte-identical. Do not touch the runner guidance, the YAML
examples, the permissions blocks, or the attestation paragraph — that content is current, binding,
and load-bearing for anyone editing a workflow file in this repo.

Then confirm the whole change is docs-only and single-file.
  </action>
  <verify>
    <automated>cd "$(git rev-parse --show-toplevel)" && echo "--- files changed (expect exactly: CLAUDE.md) ---" && git diff --name-only</automated>
    <automated>cd "$(git rev-parse --show-toplevel)" && echo "--- diffstat ---" && git diff --stat -- CLAUDE.md</automated>
    <automated>cd "$(git rev-parse --show-toplevel)" && echo "--- no src/test/pattern/workflow file touched (expect 0) ---" && git diff --name-only | grep -Ec '^(src/|tests/|patterns/|\.github/|Cargo\.(toml|lock))' ; true</automated>
    <automated>cd "$(git rev-parse --show-toplevel)" && echo "--- no stale milestone pointer left anywhere in file (expect 0) ---" && grep -Eic 'this milestone\)\.|Production Readiness \(v0\.0\.3' CLAUDE.md ; true</automated>
  </verify>
  <done>
- `git diff --name-only` prints exactly one line: `CLAUDE.md`.
- The source/test/pattern/workflow gate prints `0`.
- The stale-pointer gate prints `0`.
- `git diff --stat` shows a change confined to the Status block region and the single CI revision-note line — roughly 10-20 changed lines, no more.
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| repo docs -> future agent/contributor behaviour | CLAUDE.md is read as binding instruction by every agent session in this repo; a false statement here propagates into actions, not just into a reader's head |

No code, network, parsing or dependency boundary is crossed: this plan adds no package, runs no
new binary, and changes no input-handling path. No package-manager install task exists, so the
package-legitimacy gate does not apply.

## STRIDE Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation Plan |
|-----------|----------|-----------|----------|-------------|-----------------|
| T-dbv-01 | Tampering (integrity of instructions) | `## Status` in CLAUDE.md | medium | mitigate | The false "CI is broken, nothing merges" claim is the live instance of this threat — it halts work on a green repo. Task 1 replaces it with the verified state plus the two re-verification commands and a last-verified date, so the next drift is detectable rather than silently believed. |
| T-dbv-02 | Tampering (policy erosion) | `## CI / Self-Hosted Runners` in CLAUDE.md | high | mitigate | A rewrite that restates the runner policy inside the Status block creates a second copy that can drift and could lead someone to point a self-hosted label at this public repo (queues forever) or a secret-bearing job at a fork-firable trigger. Task 1 forbids restating it and only cross-references; a verify gate asserts zero added/removed lines carrying `ubuntu-latest`, `allows_public_repositories`, `arc-runner-unityinflow`, `id-token` or `attestation`. |
| T-dbv-03 | Tampering (collateral edit) | line 9 `**Phase:** 1`, `.planning/*` | medium | mitigate | The two "Phase" numbering systems invite an executor to "fix" the correct ecosystem-phase marker, or to sync `.planning/` files that are another milestone's business. Called out explicitly in `<trap_do_not_fall_for_this>`; enforced by the line-9 diff gate and by `git diff --name-only` returning exactly `CLAUDE.md`. |
| T-dbv-04 | Information disclosure | new prose | low | accept | The added text names only public repo/release/issue identifiers already public on GitHub. No secret, token or runner hostname is introduced. |
</threat_model>

<verification>
Documentation-only change. Verification is deliberately limited to diff scope and string gates.

**Do not run `cargo test`, `cargo build` or `cargo clippy` for this task.** Nothing that compiles
changes, and the full suite takes roughly six minutes — a known cause of executor watchdog stalls
in this repo. If you feel the urge to run the suite, that is a signal you have edited something
outside CLAUDE.md; check `git diff --name-only` instead.

Full check, run once at the end:

1. `git diff --name-only` -> exactly `CLAUDE.md`
2. `git diff --stat -- CLAUDE.md` -> change confined to the Status block plus one CI-note line
3. Status-block negative gate -> `0`
4. Status-block presence gate -> 8 x `OK`, 0 x `MISS`
5. Line-9 diff gate -> `0`
6. CI-policy-token diff gate -> `0`
</verification>

<success_criteria>
- `## Status` names v0.1.0 (2026-08-29) as the latest release.
- `## Status` names the v0.2.0 "Agent-shaped attacks" milestone as current, with Phases 1-3 complete and Phase 4 (#34) in flight.
- `## Status` states CI is green and is the merge gate, and routes the reader to `## CI / Self-Hosted Runners` without duplicating it.
- `## Status` ends with an anti-drift blockquote naming `gh release list --repo UnityInFlow/injection-scanner` and `gh run list --branch main`, and **Last verified: 2026-09-23**.
- The CI section's revision note attributes itself to the archived milestone, not "this milestone".
- Line 9 is byte-identical; `## CI / Self-Hosted Runners` policy content is byte-identical.
- `git diff --name-only` is exactly `CLAUDE.md`.
</success_criteria>

<output>
Create `.planning/quick/260923-dbv-reconcile-the-stale-claude-md-status-blo/260923-dbv-SUMMARY.md` when done.

Commit with:
`docs: reconcile the stale CLAUDE.md status block`

The standing pre-PR gates (`.claude/skills/code-review`, `.claude/skills/pr-artifacts`) run at PR
time, not inside this plan. `.claude/skills/pattern-library` does not apply — no
`patterns/core/*.yaml` is touched.
</output>
