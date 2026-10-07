# Finding — #132 is not a documentation-formatting issue

**Quick task:** 261007-a8r · **Date:** 2026-10-07 · **Branch:** `docs/132-detection-backlog-self-matches`

## Verdict

Issue #132's stated root cause and suggested direction are both wrong. Backtick-quoting the
attack-shape phrases in `docs/DETECTION-BACKLOG.md` would **not** close the 10 self-matches.
The real cause is a false-positive bug in the context classifier (`src/context.rs`).

## Disproof of the issue's diagnosis

`docs/DETECTION-BACKLOG.md:74` already reads:

```
- `you have permission to run any command`, `you may skip confirmation`
```

Both phrases are already inside code spans, and both are still reported — `PI054` and `PI055`,
severity **HIGH**, the severity `install-hook` blocks commits at.

## Measured mechanism

Three minimal files, scanned with the release binary built from `951f96b`:

| File | Content | Result |
|---|---|---|
| `t1.md` | A backticked payload alone | **no matches** — code-span suppression works |
| `t2.md` | The `PI017` catalogue row, then the same backticked payload | `PI054` HIGH, `ctx=hidden_html` |
| `t3.md` | A backticked `<span style="display:none">`, then a bare payload | `PI054` HIGH, `ctx=hidden_html` |

`t1` vs `t2` isolates it: the only difference is a preceding line that *documents* an HTML tag
inside a code span.

## Root cause

`docs/DETECTION-BACKLOG.md:31` documents the `PI017` hidden-html-styling pattern:

```
| PI017 | hidden-html-styling | `<span style="display:none">`, `<div hidden>`, ... |
```

`hidden_openers()` (`src/context.rs:358`) scans the raw line for `<` and has no awareness of
inline code spans — the classification loop strips fenced code (`LineKind::FencedCode`) and HTML
comments, but never inline backticks. So the documented `<span style="display:none">` is treated
as a real hidden-HTML opener. It never closes, so `hidden_block` (`src/context.rs:150`) stays
open for the **remainder of the file**: every line from 33 to EOF is classified
`LineKind::HiddenHtml`, which scores confidence `1.0` (`src/context.rs:77`) and overrides the
code-span suppression that `t1.md` proves otherwise works.

That is exactly why all 25 matches report `ctx=hidden_html`, and why the first one is at line 33
— the line after the opener.

## Scope consequence

This is **not** doc-only. It is a detection-logic change in `src/context.rs` affecting every
scanned document: any file that mentions a hiding HTML tag inside a code span has every
subsequent line scored as hidden HTML at confidence 1.0. That is a general false-positive
generator in the "scanner flags its own documentation" class, not a quirk of this one file — and
it needs the `code-review` skill, the full test suite and a GATE-03 sweep, not a doc commit.

## Not the same bug as the catalogue self-matches

`docs/PATTERN-CATALOGUE.md`'s two long-standing self-matches are a **different** cause and are
untouched by this: `PI001` at :77 and `PI031` at :890 both report `ctx=prose`, not
`hidden_html`. #132's claim that they are separate and pre-existing is correct.

## Baseline (25 matches, 10 distinct patterns) at 951f96b

```
  33  PI019  HIGH      'before responding, first' / 'prepend every answer with' / 'at the start of each reply'
  40  PI027  HIGH      'webhook.site' / 'pipedream.net' / 'interact.sh' / 'oast.fun'
  41  PI028  CRITICAL  'curl … \| sh' / 'wget … \| bash' / 'iwr … \| iex'
  42  PI029  HIGH      'email the contents to' / 'send a copy of this conversation to' / 'forward the transcript'
  48  PI039  HIGH      'evil confidant' / 'opposite mode' / 'simulate a terminal with no filters' / 'you are not an AI'
  56  PI045  MEDIUM    'іg' / 'аc'
  74  PI054  HIGH      'you have permission to run any command'   <- already backticked
  74  PI055  HIGH      'you may skip confirmation'                <- already backticked
 124  PI014  MEDIUM    'if you are an AI reading'
 135  PI029  HIGH      'read ~/.aws/' / 'cat ~/.ssh/'
 169  PI011  CRITICAL  '<<SYS>>'
```

## Open decision

Fix the classifier (strip inline code spans before `hidden_openers`), or apply a doc-only
workaround at `:31` that closes the documented tag. The workaround leaves the engine bug live for
every user document. Awaiting the maintainer's call.
