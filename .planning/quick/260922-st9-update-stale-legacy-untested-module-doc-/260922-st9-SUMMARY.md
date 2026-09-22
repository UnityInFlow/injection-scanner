---
status: complete
task: 260922-st9
type: quick
title: Update stale LEGACY_UNTESTED module doc
files_changed:
  - tests/pattern_policy_test.rs
commit: b85a536
---

# 260922-st9: Update stale LEGACY_UNTESTED module doc — Summary

Doc-comment-only fix to `tests/pattern_policy_test.rs`. PR #138 (be0ef9e,
2026-09-22) backfilled the last 11 legacy pattern ids and emptied
`LEGACY_UNTESTED`, closing #89 — but the module-level `## The ratchet`
doc block and the const's own doc comment still described it in the
present tense as a populated debt register ("they are listed in
`LEGACY_UNTESTED`", "Backfilling is tracked in #89"). Read against
`const LEGACY_UNTESTED: &[&str] = &[];`, that text reads as either a lie
or a disabled check — when in fact an empty exemption list is the ratchet
at its strictest, since every shipped pattern now has to clear the
3-positive/2-negative minimum with nothing exempt.

## Changes

**Task 1 — module header (`## The ratchet`, lines 16-26 → rewritten):**
Put the origin story in the past tense (30 pre-helper patterns, the four
widenings #80/#95/#97/#99 taking that to 11, the #89 backfill emptying
the list), stated explicitly that an empty list means maximum strictness
rather than a disabled check, named the empty state as the ratchet's
reached end state, restated the three still-meaningful properties (not
listed → must comply, list may not grow, an entry that starts complying
must leave), and gave forward guidance in both directions: nothing goes
back on the list, and the empty const + its tests stay — deleting them
would reopen the exemption route the may-not-grow guard holds shut. The
first `//!` paragraph (origin issues #70/#27, PR #66/PI048 evidence) was
left byte-identical, as required.

**Task 2 — `LEGACY_UNTESTED` const doc comment:** Kept the bold
"**Do not add to this list.**" instruction and its reason. Replaced the
present-tense "Backfilling is tracked in #89" with the past-tense "The
#89 backfill completed and covered every remaining id" (#89 is closed).
Kept the still-true "a test below fails if you leave a compliant id
behind" sentence with its existing `—` em-dash form byte-identical. Added
the one thing the old comment lacked: the register is empty on purpose
and stays that way, pointing at the module docs above rather than
duplicating the argument.

No `fn`, `#[test]`, `assert!` message, or function-body `//` comment was
touched. `const LEGACY_UNTESTED: &[&str] = &[];` is unchanged.

## Deviations from Plan

None — plan executed exactly as written. The worktree branch precondition
in the original plan (`branch must be docs/legacy-untested-module-doc`)
was explicitly waived by the orchestrator for this isolated-worktree run;
per the task instructions, HEAD (`be0ef9e`, PR #138) and the target
`LEGACY_UNTESTED` state were verified directly instead.

## Verification

- `cargo fmt --all -- --check` — exit 0 (both before and after commit)
- `cargo test --test pattern_policy_test` — `test result: ok. 5 passed;
  0 failed; 0 ignored; 0 measured; 0 filtered out`
- `grep -cE '^const LEGACY_UNTESTED: &\[&str\] = &\[\];$'
  tests/pattern_policy_test.rs` → `1`
- Diff-shape check (`git diff -U0 ... | grep -vE '^[-+]//[!/]' | wc -l`)
  → `0` on both the pre-commit working diff and the post-commit
  `HEAD~1..HEAD` diff — every changed line is a `//!` or `///` comment
  line
- `git show --stat --oneline HEAD` → 1 file changed, only
  `tests/pattern_policy_test.rs`

## Self-Check: PASSED

- FOUND: tests/pattern_policy_test.rs (exists, contains
  `const LEGACY_UNTESTED: &[&str] = &[];`)
- FOUND: commit `b85a536` (`git log --oneline` on this branch)
