---
phase: 1
requirement: ENG-01
issue: 32
status: complete
shipped: 2026-08-30
---

# Phase 1 complete — structural frontmatter engine

**PR #104** (engine) and **#106** (alias-bomb test). Issue #32 closed.

## What shipped

`src/frontmatter.rs` — extracts YAML (`---`), TOML (`+++`) and whole-file JSON, parses with a real
parser into `serde_json::Value`, and projects to canonical `path = value` text. A new
`scope: frontmatter` field on the pattern schema restricts a pattern to that projection;
`MatchContext::FrontmatterStructural` carries confidence 1.0.

The design decision: **projection, not a rule DSL.** A `path:`/`rule:` query language would have
been a second matching language to specify, test and support, and every future structural pattern
would become a schema change. Projecting and reusing the regex engine costs one schema field.

## Verification

- 27 new tests (285 → 312, then 313 with the bomb test)
- **Detection behaviour proven unchanged**: the published v0.1.0 binary and this build both report
  728 findings on this repo, identical. No pattern shipped, so nothing could move.
- YAML alias bomb refused by `serde_yaml` in ~27ms ("repetition limit exceeded"), surfaced as a
  parse error and the pass skipped. Pinned by a test because the property is held by an upstream
  implementation detail — see #105.

## Carried forward

- **A scope test is vacuous without a positive control.** `the_same_rule_is_silent_on_prose` would
  pass equally if the regex simply failed to match; `a_prose_scoped_rule_would_have_fired_on_that_same_prose`
  fires the identical sentence through a prose-scoped rule to prove the silence is scope.
- **The structural pass is inert without a frontmatter-scoped pattern.** A first bomb test "passed"
  in 0.02s having measured nothing, because the parser never ran. Load a probe via `--patterns`.
- A self-scan finding landed in `src/` for the first time. Suppressed inline with rationale rather
  than weakening the doc example.
