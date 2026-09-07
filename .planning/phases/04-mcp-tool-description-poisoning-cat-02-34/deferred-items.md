# CAT-02 (#34) — Deferred items

Every deferral this phase measured or carried forward, with its issue number or its existing
open status, why it was deferred, and what would trigger revisiting it. Written at close-out
(plan 04-07) so nothing survives only as tacit knowledge in a SUMMARY once this branch merges.

## Newly filed issues (measured limitations from this phase's own work)

### 1. JSONC parse gap — [issue #129](https://github.com/UnityInFlow/injection-scanner/issues/129)

**What:** A comment-bearing `mcp.json`/`.mcp.json`/`claude_desktop_config.json` (the VS Code /
GitHub Copilot IntelliJ house style) is rejected by `serde_json`, the structural pass is
skipped under the skip-do-not-abort rule (FIX-03), and the scan reports zero matches **with no
diagnostic on stdout or stderr**. Measured against a real file on a development machine
(`~/.config/github-copilot/intellij/mcp.json`) and a minimal synthetic reproduction in
`04-RESEARCH.md` §Q1.

**Why deferred:** `serde_json` does not support comments; adding tolerance is a parser/engine
capability change, not a new pattern — out of scope for a pattern-only phase (same category of
work as D-05, below).

**What would trigger revisiting it:** A future engine-capability phase, or a user report that a
JSONC-shaped config silently produced zero findings. At minimum, issue #129 suggests printing a
diagnostic on parse failure for config-shaped files even without a full JSONC parser — that
narrower fix could land independently of the full parser change.

### 2. The decoded-layer pass does not run structural (`scope: frontmatter`) patterns — [issue #130](https://github.com/UnityInFlow/injection-scanner/issues/130)

**What:** The recursive decoder (ENG-02) re-runs detection on each decoded layer using only
prose-scoped patterns. A `scope: frontmatter` structural pattern never sees a decoded value,
even inside a document the structural pass otherwise projects.

**What plan 04-05 actually measured (narrowing the issue, per the plan's own instruction):**
the prose arms (`PI063`/`PI064`) **do** reach the encoded-description payload
(`12-encoded-description-payload.md`), via the ordinary prose decoded-layer pass over the raw
JSON text — the same mechanism `PI029` already used for this payload. So this is **not** a
broad "encoded payloads are invisible" gap; it is narrower: a *structural* pattern operating on
a *projected* value would not get the same reach, because decoding only re-runs prose-scoped
patterns. No shipped CAT-02 structural pattern (`PI060`-`PI062`) currently needs to decode a
projected value, so this has not yet produced a measured miss — it is a real limitation of the
mechanism, not (yet) of a specific shipped pattern.

**Why deferred:** Engine capability change (extending or duplicating the decoded-layer pass to
also run structural patterns against decoded/re-projected values), not a pattern change.

**What would trigger revisiting it:** A future structural pattern (any category) whose target
value could plausibly be encoded by an attacker — e.g. a future credential-harvesting or
persistence-hijack signal reading a structured config field.

### 3. D-05 — structural cross-reference for tool shadowing — [issue #131](https://github.com/UnityInFlow/injection-scanner/issues/131)

**What:** `PI066`/`PI067` are heuristics on sentence shape; neither verifies that the tool named
in a shadowing/override description actually exists among the manifest's own declared tools.

**Why deferred:** Locked in advance by `04-CONTEXT.md`'s D-04/D-05 decisions, not discovered
late. Needs cross-node state in the structural projection pass (collecting declared tool names,
then cross-checking a different projected description against that set) — new engine
capability, and Phase 4 is a pattern phase.

**What would trigger revisiting it:** Real-world false-positive or false-negative data on
`PI066`/`PI067` in production that a cross-reference would resolve, or a future engine-scale
phase that adds cross-node structural state generally.

### 4. `docs/DETECTION-BACKLOG.md` self-matches 10 patterns — [issue #132](https://github.com/UnityInFlow/injection-scanner/issues/132)

**What:** The whole-repo self-scan finds `docs/DETECTION-BACKLOG.md` self-matching 10 distinct
pattern ids (`PI011`, `PI014`, `PI019`, `PI027`, `PI028`, `PI029`, `PI039`, `PI045`, `PI054`,
`PI055`), on top of the two long-standing `docs/PATTERN-CATALOGUE.md` self-matches (`PI001`,
`PI031`). Total measured set: **12 unique `(file, pattern_id)` pairs**, unchanged across plans
04-04, 04-05, 04-06 and this plan's own final sweep (see `04-SWEEP.md`'s Plan 07 section,
"Number reconciliation table").

**Why deferred:** Verified during plan 04-04 (stashing all of 04-04's changes and re-running —
identical with and without) that the 10-entry backlog set arrived with the PR #110 merge, not
with any Phase 4 plan. Not attributable to this phase; fixing it here (rewording another
category's documentation prose) is out of GATE-04's one-category scope and not in this plan's
declared `files_modified`.

**What would trigger revisiting it:** Issue #132 itself — a standalone documentation-formatting
quick task (wrap quoted attack phrasing in `docs/DETECTION-BACKLOG.md` in backtick code spans,
same remediation the two pre-existing catalogue self-matches need).

### 5. WR-02 — `tool-permission-abuse/` structural corpus README table backfill — [issue #133](https://github.com/UnityInFlow/injection-scanner/issues/133)

**What:** `tests/corpus/attack/structural/README.md`'s `tool-permission-abuse/` (CAT-01) table
documents only 1 of its 5 corpus files.

**Existing status:** Carried over from Phase 3, explicitly left open by plan 04-01's
directory-layout generalisation, and carried forward unfixed by every plan in this phase.
`.planning/WINDOWS.md` ledger entry #2, **waived** at this plan's close with the reason
"Filed as issue #133."

**Why deferred here specifically:** `tool-permission-abuse/` is CAT-01's corpus, not CAT-02's.
Backfilling its table inside a CAT-02-only PR risks the same "which change caused which diff"
confusion plan 04-01 explicitly declined to create, and the file is not in this plan's declared
`files_modified`.

**What would trigger revisiting it:** Issue #133 itself — a table-only backfill, no pattern or
corpus content change.

## Not filed (measured, but nothing to file)

### Registry candidate rejected by a shipped pattern (plan 04-03)

The plan's Task 2 instruction asks to file an issue for "any registry candidate plan 04-03
rejected because a shipped pattern fired on it." **Checked, not applicable.** `04-03-SUMMARY.md`
records explicitly: "No candidate was rejected — every fetched candidate, including one
deliberately left unvendored for scope reasons, reported zero matches" at `--min-confidence 0`.
Zero candidates were rejected during that plan's registry-vendoring work, so there is no false
positive to file.

## Existing open items, carried forward without duplication

Per the plan's own instruction, these are **not** newly filed — they are pre-existing and are
recorded here so they remain visible without creating a duplicate issue.

### D-01's accepted third-person blind spot — partially narrowed, not fully closed

**Status:** Accepted design cost, not a bug. Stated directly in `patterns/core/mcp-tool-poisoning.yaml`'s
header comment, in the README's "The deliberate blind spot" behaviour-change callout (added by
plan 04-05), and in `04-05-SUMMARY.md`'s key-decisions and coverage (D12).

**What changed since it was first accepted:** Plan 04-06's `PI066 cross-tool-shadowing` closes
the *specific* third-person shape D-01 named as its example (the Elastic-quoted
cross-tool-shadowing payload) — deliberately person-agnostic, matching on sentence subject
rather than second-person address. This is a **narrowing, not a closure**: `PI066` only reaches
a description that names *another tool's* invocation as the trigger. A bare third-person
payload aimed directly at the model, with no other tool referenced, is still outside every
CAT-02 pattern's reach — `PI063`-`PI065` require second-person address by design, and `PI066`
requires a cross-tool reference by design.

**Where this is recorded in shipped files:** `patterns/core/mcp-tool-poisoning.yaml` header
comment; `README.md`'s two behaviour-change callouts for `PI063`-`PI065` and `PI066`;
`04-05-SUMMARY.md` (D12, key-decisions); `04-06-SUMMARY.md` (key-decisions, "PI066 is
deliberately person-agnostic... closing D-01's accepted blind spot rather than restating it").

**Not filed as a new issue.** This is a named, accepted, documented design tradeoff with its own
rationale recorded at the pattern-file level — not an unmeasured gap. Revisit if real-world
false-negative data on a bare third-person payload (no cross-tool reference) surfaces.

### Rug-pull bound — `PI068`/`PI069` detect language, not the class itself

**Status:** Accepted design cost, stated in `patterns/core/mcp-tool-poisoning.yaml`'s header
comment (per 04-06's key-decisions: "stated in the pattern file's own header comment, not only
in this plan record, because a single scan cannot prove absence of a future republish"), in
three README behaviour-change callouts, and in `04-06-SUMMARY.md`.

**Not filed as a new issue.** A single static scan cannot mitigate a server that republishes a
different, poisoned description after a gating condition is met — this is a structural
limitation of static analysis on a single snapshot, not a gap this scanner's pattern set could
close with more regex. Recorded for the developer review packet in `04-07-SUMMARY.md`.

## Summary

| # | Item | Status | Reference |
|---|---|---|---|
| 1 | JSONC parse gap | Newly filed | #129 |
| 2 | Decoded-layer pass skips structural patterns | Newly filed | #130 |
| 3 | D-05 structural cross-reference | Newly filed | #131 |
| 4 | `docs/DETECTION-BACKLOG.md` 10 self-matches | Newly filed | #132 |
| 5 | WR-02 structural corpus README gap | Existing, now waived with issue | #133 |
| 6 | Registry candidate rejected (04-03) | N/A — none rejected | — |
| 7 | D-01 third-person blind spot | Existing, accepted, partially narrowed by PI066 | recorded in shipped files, not filed |
| 8 | Rug-pull bound (PI068/PI069) | Existing, accepted | recorded in shipped files, not filed |
