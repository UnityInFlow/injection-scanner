# Phase 3: Tool & permission abuse — CAT-01 (#33) - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-31
**Phase:** 3-tool-permission-abuse-cat-01-33
**Areas discussed:** Corpus shape for the structured half, Building the missing FP control, The S005 duplication boundary, Scope of the dangerous-command items

---

## Corpus shape for the structured half

**Q1 — How should structural payloads be represented, given the harness scans one line per payload?**

| Option | Description | Selected |
|--------|-------------|----------|
| Extend harness: whole-file payloads | `tests/corpus/attack/structural/`, each file one payload; `recall_test.rs` grows a second collection mode | ✓ |
| Prose-only corpus, structural measured separately | Keep the corpus line-oriented; measure structural in `frontmatter_test.rs` | |
| Single-line JSON frontmatter | One-line JSON objects the projection still parses; no harness change | |

**Notes:** Option 3 was argued against on the grounds that contorting a payload to fit the tooling is a cousin of deriving it from the patterns.

**Q2 — How are the two halves counted in EXPECTED and the README?**

| Option | Description | Selected |
|--------|-------------|----------|
| Two rows — prose and structural | Separate EXPECTED entries; a regression names which half broke | ✓ |
| One combined row | Single 12/12 entry, matching the README's existing one-row-per-category shape | |
| You decide | Claude's discretion | |

**Notes:** The deciding argument was that a combined row stays correct while the structural half silently goes to zero — which is what an inert ENG-01 pass looks like.

**Q3 — How do the 12 payloads split between prose and structural?**

| Option | Description | Selected |
|--------|-------------|----------|
| Threat-model first, split falls out | No ratio set in advance; record what results | ✓ |
| Threat-model first with a floor of 4 each | Return to the threat model if either half lands under 4 | |
| Fixed 6/6 | Decide the split up front for symmetric harness coverage | |

**Notes:** The 6/6 in the previous question's preview was Claude's illustration, not a proposal; this was flagged and corrected before asking.

**Q4 — What enforces "never derived from the patterns" when one PR writes both?**

| Option | Description | Selected |
|--------|-------------|----------|
| Corpus first, baselined against today's build | Commit payloads alone, measure against the 48-pattern binary, then add patterns; the delta is the evidence | ✓ |
| Corpus committed first, no baseline run | Git order proves it; skip the baseline measurement | |
| Rely on the sourcing rule in the corpus README | Existing convention, no ordering enforcement | |

---

## Building the missing FP control

**Q1 — Which near-miss documents join `tests/corpus/clean/`?** (multi-select)

| Option | Description | Selected |
|--------|-------------|----------|
| A skill file with real, narrow allowed-tools | Control for the structural patterns — proves the rule fires on the value, not the key | ✓ |
| A settings/permissions reference doc | Control for the prose patterns — mentioning a flag ≠ telling the agent to use it | ✓ |
| A runbook instructing a human to bypass | The hardest case; differs from the payload only by audience and provenance | ✓ |
| An MCP/agent config with a real settings.json edit | Collides directly with one of #33's prose patterns | ✓ |

**Notes:** All four selected. Clean corpus goes 16 → 20 files. Established by `grep` that the clean corpus currently contains zero permission vocabulary, so the category's control did not exist at all.

**Q2 — What enforces GATE-05's mutation testing?**

| Option | Description | Selected |
|--------|-------------|----------|
| Automated pairing test in CI | Shipped pattern must stay off the control; a relaxed form must match it | ✓ |
| Manual mutation, evidence in the PR body | How v0.1.0 caught two over-widenings; decays | |
| Both — automate and show evidence once | Durable gate plus reviewer-visible before/after | |

**Q3 — Where does the relaxed form live?**

| Option | Description | Selected |
|--------|-------------|----------|
| A test-side pairing table | `CONTROLS` in `tests/fp_control_test.rs`; schema stays frozen | |
| A new optional schema field | Beside `example` / `counter_example`, following the `raw_only` precedent | ✓ |
| You decide | Claude's discretion | |

**Notes:** Rated one-way in CONTEXT.md — `patterns/` is a community-contribution surface with `deny_unknown_fields` enforced.

**Q4 — Required for which patterns?**

| Option | Description | Selected |
|--------|-------------|----------|
| Required for new patterns, ratchet-enforced | PI050+ only; existing 48 exempt; CAT-02/03 inherit | ✓ |
| Optional everywhere, convention in CAT-01 | No ratchet change | |
| Required for all 78 — backfill the 48 | Strongest guarantee; large mechanical change against GATE-04 | |

---

## The S005 duplication boundary

**Contradiction surfaced before questioning:** ROADMAP.md's Phase 3 criterion ("complements S005 rather than duplicating it") directly conflicts with issue #33's pattern list (wildcard grants in frontmatter). Taken literally the criterion deletes the structural half and leaves ENG-01 inert.

**Q1 — Does a frontmatter-scoped PI05x fire on a file's own wildcard grant?**

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — different tool, different provenance | S005 lints your own spec at authoring time; this scans untrusted input | ✓ |
| Only the persuasion case | Honours the roadmap literally; ENG-01 stays inert | |
| Fire, but demoted and cross-referenced | Lower severity with remediation pointing at S005 | |

**Q2 — How is the resulting consumer breakage handled?**

| Option | Description | Selected |
|--------|-------------|----------|
| Ship it — it's the finding the tool exists for | Documented behaviour change in release notes and README | ✓ |
| Ship it, and verify --baseline absorbs it first | Same finding, with a proven migration path | |
| Gate the own-file grant behind --strict | Zero breakage; flagship structural finding off by default | |
| You decide | Claude's discretion | |

**Notes:** `spec-ci-plugin` shells out to this binary in consumer CI, so green builds carrying a wildcard grant go red on upgrade. Accepted deliberately.

**Q3 — Which projected keys does CAT-01 claim?**

| Option | Description | Selected |
|--------|-------------|----------|
| Permission keys only | `allowed-tools`, `tools`, `permissions`; `mcpServers`→CAT-02, `hooks`→CAT-03 | ✓ |
| Permission keys plus model/system overrides | Also claims frontmatter `model:` / `system:` | |
| All structural keys | CAT-01 arms the whole projection | |

---

## Scope of the dangerous-command items

**Findings surfaced before questioning:** `PI028 pipe-to-shell` already exists in `exfiltration.yaml` and covers most of #33's `curl | sh` item, but its regex requires `\|\s*(?:ba)?sh\b` so `curl x.sh | sudo sh` does not match. No `sudo` / `rm -rf` / `chmod 777` patterns exist anywhere.

**Q1 — What happens to the dangerous-command items?**

| Option | Description | Selected |
|--------|-------------|----------|
| Drop from CAT-01; file the PI028 gap separately | Payload execution, not authority widening; own issue, own sweep | ✓ |
| Keep them, narrowed by context | Require agent-directed framing via markdown context awareness | |
| Keep only the PI028 widening, in this PR | Closes the measured gap now; touches a second category against GATE-04 | |

**Q2 — Is "10 patterns" still the target?**

| Option | Description | Selected |
|--------|-------------|----------|
| No — the count follows the threat model | PI050-059 is an ID range, not a quota | ✓ |
| Yes — hold 10, backfill from the backlog | Pull adjacent shapes from DETECTION-BACKLOG Part 2 | |
| Yes — hold 10 by keeping the dangerous-command items | Reverses Q1 | |

**Q3 — Where does "disable a hook or a guardrail" live?**

| Option | Description | Selected |
|--------|-------------|----------|
| CAT-01 keeps prose disabling; CAT-03 keeps hooks config | Split on intent: removing a control vs installing presence | ✓ |
| Move it entirely to CAT-03 | One category owns everything hook-related | |
| You decide | Claude's discretion | |

---

## Claude's Discretion

None. Three "You decide" options were offered across the discussion and all three were declined in favour of an explicit choice.

## Deferred Ideas

- `PI028` `| sudo sh` gap — own issue, own sweep (GATE-04)
- Bare dangerous-command detector (`sudo`, `rm -rf`, `chmod 777`) — v0.3.0 backlog
- Backfill the FP-control field across the existing 48 patterns, including the two known-slack v0.1.0 controls
- `model:` / `system:` frontmatter overrides — unclaimed by any category
- The two pre-existing `docs/PATTERN-CATALOGUE.md` sweep findings (lines 73, 902)
