---
phase: quick
plan: 261007-a8r
status: complete
completed: 2026-10-07
issue: 132
branch: docs/132-detection-backlog-self-matches
commits: [1f2b507, c65c116, 53bc452]
key-files:
  modified: [src/context.rs, tests/markdown_context_test.rs, README.md]
filed: [158]
---

# Quick Task 261007-a8r: close #132

Opened as the doc-only task #132 describes. It is not one. The task was executed
inline rather than through a planner subagent, because the first verification step
disproved the issue's premise and there was no longer a documentation plan to write;
`261007-a8r-FINDING.md` records that before any fix was attempted.

## What #132 got wrong

#132 attributes the 10 self-matches in `docs/DETECTION-BACKLOG.md` to attack-shape
phrases being wrapped in double quotes instead of backticks, and proposes
backtick-quoting them. That cannot work. Line 74 reads:

```
- `you have permission to run any command`, `you may skip confirmation`
```

Both payloads are already inside code spans and both were still reported, at HIGH —
the severity `install-hook` blocks commits at.

## Actual cause

`hidden_openers` (`src/context.rs:358`) scanned the raw line, so the
`<span style="display:none">` that line 31 *documents* for `PI017` counted as real
markup. Documentation quotes an opening tag without its closing one, so the hidden
block never closed: every line from 33 to EOF was classified `HiddenHtml`, which
scores confidence 1.0 and therefore outranks the inline-code (0.3) and table (0.3)
downgrades that exist to keep documentation quiet.

Isolated with three minimal files — a backticked payload alone reports nothing; the
same payload preceded by a line that merely names a hiding tag reports HIGH.

## Fix

An opener is skipped when it sits inside a **closed** inline code span.

`closed` is load-bearing, and the first attempt got it wrong. Keying on any code span
let a single unmatched backtick typed in front of real markup disown a genuine
opener, demoting a backticked payload inside the hidden block from 1.0 to 0.3 and
withholding it — converting a false-positive fix into an evasion. A quoted tag is
balanced; a stray backtick is not. `a_quoted_opener...` and
`an_unclosed_backtick_does_not_let_a_real_opener_be_disowned` pin both directions.

`closed_inline_code_spans` reuses `in_inline_code`'s pairing rules rather than adding
a second backtick parser. The check sits last in the `&&` chain, after the rare
`attributes_hide` test, so the common path never pays for the backtick scan.

## Verification

| Check | Result |
|---|---|
| `cargo test` | **458 passed, 0 failed** (39 binaries; 454 baseline + 4 added) |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo fmt --check` | clean |
| `the_attack_corpus_keeps_every_finding` (exact pinned counts) | unchanged |
| `recall_matches_the_recorded_numbers` | unchanged |
| `docs/DETECTION-BACKLOG.md` | 25 reported → **0** (31 correctly withheld as documentation) |
| Repo-wide self-scan, non-corpus | 12 → **2** |

The 2 remaining are `docs/PATTERN-CATALOGUE.md`'s `PI001` at :77 and `PI031` at :890.
Both report `ctx=prose`, a different cause, which #132 correctly describes as
predating the milestone. They are untouched here.

## Also changed

`README.md`'s context/confidence table never listed `Hidden html` — the one context
this fix is about. Added, with the quoted-tag rule and its closing-span limit stated
as user-visible behaviour. Its `--strict` example claimed 15 findings on the README;
measured 26 both before and after these edits, so the figure was corrected.

Still missing from that table: `Frontmatter (structural)` (1.0). Left alone —
unrelated to this fix.

## Filed

**#158** — `nesting_after` was not changed, so a quoted *closing* tag still closes a
real hidden block early. Verified pre-existing and not a regression from this fix
(`git diff 951f96b..HEAD -- src/context.rs` touches no `nesting_after` line); before
this change openers and closers were symmetrically blind to code spans, and the
behaviour in that scenario is identical either side of it. Low severity: it withholds
a finding only when the payload is also in a code span or table cell, since `Prose`
and `HiddenHtml` score the same. Not folded in because `nesting_after` is called on
substrings, so span positions would need rebasing per caller — real complication, for
a contrived shape, in the safe direction.

## Not applicable

- **ADR** — not required. The `pr-artifacts` gate exempts bug fixes, and this restores
  the code-span downgrade that #20 already decided on rather than changing the engine
  or a format contract.
- **`pattern-library` skill** — no `patterns/core/*.yaml` change, so no catalogue or
  baseline regeneration. `docs/PATTERN-CATALOGUE.md` is byte-identical.
