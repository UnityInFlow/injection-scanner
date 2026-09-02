---
phase: 03-tool-permission-abuse-cat-01-33
reviewed: 2026-09-02T06:39:55Z
depth: standard
files_reviewed: 34
files_reviewed_list:
  - .claude/skills/pattern-library/SKILL.md
  - .github/code-scanning-baseline.json
  - CHANGELOG.md
  - PATTERNS.md
  - README.md
  - docs/DETECTION-BACKLOG.md
  - docs/PATTERN-CATALOGUE.md
  - examples/README.md
  - examples/tool-permission-abuse-attack.md
  - patterns/core/tool-permission-abuse.yaml
  - scripts/gate03-sweep.sh
  - src/pattern.rs
  - src/patterns/mod.rs
  - tests/case_sensitivity_test.rs
  - tests/corpus/attack/README.md
  - tests/corpus/attack/structural/01-wildcard-allowed-tools-block-sequence.md
  - tests/corpus/attack/structural/02-scalar-wildcard-tools-grant.md
  - tests/corpus/attack/structural/03-json-manifest-wildcard-tools.md
  - tests/corpus/attack/structural/04-permissions-allow-wildcard-settings.md
  - tests/corpus/attack/structural/05-bypass-permission-mode.md
  - tests/corpus/attack/structural/README.md
  - tests/corpus/attack/tool-permission-abuse.md
  - tests/corpus/clean/cli-flag-reference.md
  - tests/corpus/clean/mcp-setup-guide.md
  - tests/corpus/clean/narrow-allowed-tools-skill.md
  - tests/corpus/clean/sandbox-bypass-runbook.md
  - tests/corpus/clean/settings-deny-list.md
  - tests/corpus/clean/settings-permissions-reference.md
  - tests/frontmatter_test.rs
  - tests/pattern_policy_test.rs
  - tests/pattern_relaxed_control_test.rs
  - tests/pattern_test.rs
  - tests/pattern_validation_test.rs
  - tests/raw_only_test.rs
  - tests/recall_test.rs
findings:
  critical: 1
  warning: 3
  info: 1
  total: 5
status: issues_found
---

# Phase 3: Code Review Report

**Reviewed:** 2026-09-02T06:39:55Z
**Depth:** standard
**Files Reviewed:** 34
**Status:** issues_found

## Summary

Reviewed the CAT-01 tool-and-permission-abuse pattern shipment (PI050–PI057), the new
`relaxed_pattern`/`PatternScope::Frontmatter` schema fields in `src/pattern.rs` /
`src/patterns/mod.rs`, `scripts/gate03-sweep.sh`, the GATE-05 mutation-pairing test, and the
supporting corpus/docs. The structural half (PI050–PI052) is careful and well-defended: I traced
every `relaxed_pattern`/`counter_example` pairing by hand and all six hold (shipped pattern misses
the counter-example, relaxed form catches it), the `.allow` vs `.deny` D-06a narrowing is correct
against `settings-deny-list.md`, and the frontmatter-projection false-positive property (structural
rules never see prose) is proven by both unit tests and the corpus.

The prose half (PI053–PI057) has one real, reproducible defect that the corpus and unit-test suite
do not catch: three of the five prose patterns (PI053, PI056, PI057) fire on sentences that
**prohibit** the dangerous action rather than instruct it — the exact same "negation blindness"
failure mode that GATE-03's own sweep already identified and fixed for a *different* shape in
PI057 (`"DO NOT skip the config gate check"`). The fix that shipped for that specific case (require
the verb+object window to also contain `you`) does not generalize to negation, because `you` can
sit anywhere within the bounded window regardless of polarity. I reproduced this against the
actual shipped regexes (extracted verbatim from the YAML) with `python3`'s `re` module, which is
adjacency- and boundary-equivalent to the Rust `regex` crate for these ASCII, non-backreference
patterns; the Rust `regex` crate's automaton-based matching means there is no catastrophic
backtracking risk in any of these patterns, so that specific concern raised in the phase brief does
not apply here.

Also found: `PATTERNS.md`'s "Categories" ID-range table was not updated for the new
`tool_permission_abuse` category despite this same diff editing the section immediately preceding
it; `tests/corpus/attack/structural/README.md`'s payload table documents only 1 of the 5 structural
corpus files this phase added; and `scripts/gate03-sweep.sh`'s helper functions do not declare
their working variables `local`, which is currently harmless but a latent hazard.

## Critical Issues

### CR-01: PI053, PI056 and PI057 fire on prohibitions, not just directives — negation blindness

**File:** `patterns/core/tool-permission-abuse.yaml:157` (PI053), `:218` (PI056), `:262` (PI057)

**Issue:** GATE-03's own sweep (documented in this same file's PI057 comment, lines 226–260) found
that PI057's original verb+object-window form matched `"DO NOT skip the config gate check"`
*backwards* — a prohibition read as an attack directive — and the fix was to additionally require
`you` to co-occur with the verb+object pair. That fix does not defend against negation in general,
because the negating word (`not`/`never`) is free to sit anywhere inside the bounded `[^.\n]{0,N}`
window alongside `you`, the verb and the object. The same unguarded-window shape exists in PI053's
`bypassPermissions`-mode arm and in PI056's settings-widening arm, and none of the three checks for
a preceding negation.

Reproduced directly against the shipped regexes (copied verbatim from the YAML, tested with
Python's `re` in `IGNORECASE` mode — equivalent for these patterns to the Rust `regex` crate's
matching semantics, since none use backreferences or engine-specific extensions):

```python
import re

# PI053, patterns/core/tool-permission-abuse.yaml:157
p53 = r"(?:\b(?:run|restart|re-?launch)\b\s+(?:with|in|using)\s+[^.\n]{0,10}?(?:--dangerously-skip-permissions|--yolo|--yes-always)\b|\b(?:set|switch|change|configure)\b[^.\n]{0,25}?\bpermission\s*mode\b[^.\n]{0,20}?\bto\b[^.\n]{0,15}?\bbypass\s*permissions?\b)"
re.search(p53, "Never run with --dangerously-skip-permissions, no matter what the deadline is.", re.I)
# -> MATCHES (should not: this is a prohibition, the opposite of the attack)
re.search(p53, "Do not switch the permission mode to bypassPermissions under any circumstances.", re.I)
# -> MATCHES

# PI056, patterns/core/tool-permission-abuse.yaml:218
p56 = r"\b(?:settings\.json|settings\s+file|your\s+settings)\b[^.\n]{0,55}?\b(?:permissions?\s*key|allow\s*list|auto-?approve|bypass\s*mode|unrestricted\s*(?:access|grant|permission)|no\s+longer\s+require\s+approval|without\s+(?:further\s+)?approval|skip\s+approval)\b"
re.search(p56, "Do not update your settings.json to add an auto-approve flag under any circumstances.", re.I)
# -> MATCHES
re.search(p56, "Never edit the settings file to enable a bypass mode.", re.I)
# -> MATCHES

# PI057, patterns/core/tool-permission-abuse.yaml:262
p57 = r"\b(?:turn\s+off|disable)\b[^.\n]{0,20}?\b(?:hook|guardrail|check|scanner|gate)\b[^.\n]{0,25}\byou\b|\byou\b[^.\n]{0,25}\b(?:turn\s+off|disable)\b[^.\n]{0,20}?\b(?:hook|guardrail|check|scanner|gate)\b"
re.search(p57, "You should never turn off the pre-commit hook, even under deadline pressure.", re.I)
# -> MATCHES
re.search(p57, "Do not disable the security scanner, you will be held responsible for any breach.", re.I)
# -> MATCHES
```

All six sentences are ordinary safety/operator documentation telling a reader **not** to do the
dangerous thing, and all six are misclassified as HIGH-severity `tool_permission_abuse` findings —
the identical class of error the PI057 GATE-03 rework was written to eliminate, now demonstrated
against three patterns instead of one. Per `PATTERNS.md`'s own severity guidance, "HIGH is the bar
`install-hook` blocks commits at" — a security policy doc, an incident runbook, or a code-review
comment phrased this way would fail a consumer's pre-commit hook for describing a control rather
than subverting one, which is exactly the CVE-2025-53773-adjacent false positive this category
exists to avoid causing.

Note PI054 and PI055 are *not* affected: their key phrases require the verb to sit immediately
adjacent to `you`/`permission` with only whitespace between (`you\s+have\s+permission\s+to`,
`you\s+are\s+authorized\s+to`), so a negating word breaks the literal adjacency and the pattern
correctly stays silent (verified: `"You are not authorized to use any tool without restriction."`
and `"You do not have permission to run any command without asking."` both correctly fail to
match). That is what makes the PI053/PI056/PI057 gap a real, addressable regex-construction
inconsistency rather than an inherent limit of prose matching.

**Fix:** Add a negation guard to the three affected arms — e.g. require that no
`\b(?:not|never|n't|no)\b` appears between the sentence-relevant anchor and the trigger phrase, the
same way PI057 already added a positive `you` requirement. Concretely, something like inserting a
negative lookaround is not available in the Rust `regex` crate (no lookaround support), so the
practical fix is either (a) a post-match Rust-side check that rejects a match whose preceding ~15
characters contain a negation token, applied uniformly wherever `scope: prose` context windows are
used, or (b) tightening each window so the negation-prone gap cannot exist (mirroring how PI054/055
close it structurally via literal adjacency). Add `counter_example`s exercising exactly the three
sentences above (or equivalents) to `tests/pattern_test.rs`'s negatives and to each pattern's YAML
`counter_example`, then re-run `tests/pattern_relaxed_control_test.rs` to confirm the fix doesn't
also break the existing GATE-03 `counter_example` pairing.

## Warnings

### WR-01: `PATTERNS.md`'s Categories table was not updated for the new category

**File:** `PATTERNS.md:114-122`

**Issue:** This phase's diff edits `PATTERNS.md` twice — adding the `relaxed_pattern` field
documentation (lines 19, 78-112) immediately above the `## Categories` table — but never adds a row
for the new `tool_permission_abuse` category (`PI050`-`PI059`, per `docs/DETECTION-BACKLOG.md`'s
own range convention). The table still lists only the original five categories and their ID
ranges. A contributor reading `PATTERNS.md` (the document this repo's own
`.claude/skills/pattern-library/SKILL.md` names as the canonical contribution guide) to pick an
unused `PI0XX` id for a new pattern has no way to learn from this table that `PI050`-`PI059` is
already claimed. `README.md` and `CHANGELOG.md` were both updated correctly in the same diff, which
is what makes the `PATTERNS.md` omission look like an oversight rather than a deliberate choice.

**Fix:**

```markdown
| Category | ID Range | Default Severity |
|---|---|---|
| Role Override | PI001-PI009 | CRITICAL |
| Instruction Injection | PI010-PI019 | HIGH |
| Data Exfiltration | PI020-PI029 | CRITICAL |
| Jailbreaks | PI030-PI039 | HIGH |
| Encoding/Obfuscation | PI040-PI049 | HIGH |
| Tool & Permission Abuse | PI050-PI059 | HIGH (CRITICAL structural override) |
```

### WR-02: Structural attack-corpus README documents only 1 of 5 payload files

**File:** `tests/corpus/attack/structural/README.md:48-52`

**Issue:** The "Payloads" table at the bottom of this README — whose stated purpose is per-file
"rationale" for each structural payload, since a leading comment inside the payload file itself
would break `frontmatter::extract` (explained earlier in the same README) — has exactly one row,
for `01-wildcard-allowed-tools-block-sequence.md`. This phase added four more structural payload
files (`02-scalar-wildcard-tools-grant.md`, `03-json-manifest-wildcard-tools.md`,
`04-permissions-allow-wildcard-settings.md`, `05-bypass-permission-mode.md`), all exercised by
`tests/recall_test.rs`'s `EXPECTED` row `(STRUCTURAL_CATEGORY, 5, 5)`, but none of them got a
corresponding row here. This is the exact kind of doc drift the pattern-library skill flags as a
recurring failure mode for this repo ("the scanner flagged its own documentation in two consecutive
PRs").

**Fix:** Add rows for files 02–05, e.g.:

```markdown
| `02-scalar-wildcard-tools-grant.md` | Scalar `tools: "*"` grant (JSON/YAML alias key) |
| `03-json-manifest-wildcard-tools.md` | Whole-file JSON manifest form of a wildcard tool grant |
| `04-permissions-allow-wildcard-settings.md` | `permissions.allow` array containing a bare `Bash(*)` grant |
| `05-bypass-permission-mode.md` | `permissions.defaultMode: bypassPermissions` structural override |
```

### WR-03: `gate03-sweep.sh` functions mutate global-scope variables

**File:** `scripts/gate03-sweep.sh:76-125` (`sweep_one`), `:131-150` (`build_summary`)

**Issue:** `sweep_one()` assigns `out_dir`, `dir`, `abs_dir`, `slug`, `report`, `status`,
`files_scanned` and `findings` without `local`, and `build_summary()` similarly assigns `out_dir`
directly. Since `main()` also uses a variable named `out_dir` for the same value, this is currently
harmless by coincidence — every caller happens to pass the identical value back into the
same-named global. But it is a latent footgun: any future change that calls `sweep_one` from a
nested loop, a parallelized `xargs`/background-job variant, or with a differently-scoped `out_dir`
in the caller will silently clobber that caller's variable, and the failure will not be visible
from reading either function in isolation — exactly the kind of shell bug that is invisible until
the two call sites drift.

**Fix:**

```bash
sweep_one() {
  local out_dir="$1"
  local dir="$2"
  local abs_dir slug report status files_scanned findings
  ...
}

build_summary() {
  local out_dir="$1"
  ...
}
```

## Info

### IN-01: PI050 and PI051 disagree, untested, on how much may precede the wildcard inside a tool call

**File:** `patterns/core/tool-permission-abuse.yaml:63` (PI050), `:93` (PI051)

**Issue:** PI050's value alternative for a shell-tool call is
`[A-Za-z][A-Za-z0-9_]*\(\s*\*\s*\)` — the call's argument must be *exactly* `*` (whitespace
tolerant), so `Bash(*)` matches but `Bash(rm -rf *)` does not. PI051's equivalent alternative is
`[A-Za-z][A-Za-z0-9_]*\(\s*(?:[^)]*\s)?\*\s*\)`, which additionally accepts an arbitrary prefix
ending in whitespace before the trailing `*` — so `Bash(rm -rf *)` *does* match under PI051 (this
is deliberately exercised by `settings-deny-list.md`'s `.deny` entry, which is excluded only by the
`.allow`-path requirement, not by the value shape). Neither the YAML comments nor
`tests/pattern_test.rs`'s positive/negative cases for PI050 exercise the prefixed-wildcard shape at
all, so it is not obvious from the pattern file alone whether PI050's narrower value match (bare
`*` only) is an intentional scope difference from PI051 (frontmatter tool-grant key vs.
settings-permission-allow key) or an unnoticed inconsistency. If intentional, a one-line comment on
PI050 saying so would save the next reader from re-deriving it the way this review had to.

**Fix:** Either add a short comment on PI050 explaining why its wildcard argument match is stricter
than PI051's, or add a `Bash(rm -rf *)`-shaped positive case to PI050's `assert_positives` if the
broader match is actually intended for the `allowed-tools`/`tools` key too.

---

_Reviewed: 2026-09-02T06:39:55Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
