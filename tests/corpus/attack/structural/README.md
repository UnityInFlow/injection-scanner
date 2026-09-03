# The structural attack corpus — whole-file payloads

`tests/corpus/attack/*.md` (the parent directory) is line-oriented: one file
per category, one payload per line, each line scanned as its own document.
That format cannot express a frontmatter payload at all — `payloads()` in
`tests/recall_test.rs` splits on `\n`, and a wildcard tool grant only exists
as an attack once it is sitting inside a real, parseable YAML/TOML/JSON
document. This directory is the second collection mode that exists to close
that gap (D-01).

## One category, one subdirectory

Since Plan 04-01 Task 2, this directory holds one subdirectory per attack
category, not a single flat set of files. `tool-permission-abuse/` is CAT-01's
(#33) five payloads, moved here unchanged from what used to be this
directory's root. A category gets its own subdirectory rather than a filename
prefix (`cat01-01-...md`) because the collector derives each category's
recall row name directly from the subdirectory name — see "Collection" below
— which is what lets a second category (CAT-02's MCP/tool-poisoning
structural half, `mcpServers`) arrive as its own subdirectory without
touching CAT-01's files, row name, or pinned count at all.

## One file is one whole payload

Every file directly inside a category subdirectory (other than that
subdirectory's own `README.md`, if it has one) is scanned as a **single
document**, not split into lines. The recall harness's
`structural_categories()` reads each file whole with `fs::read_to_string` and
never calls the line-splitting `payloads()` — that splitter is exactly what
this directory exists to bypass. This rule is unchanged by the subdirectory
split; only the directory nesting is new.

## The opening fence must be the file's literal first line

`frontmatter::extract_delimited` reads the fence with `lines.next()` — the
document's literal first line. No leading `#` or HTML comment, no blank line,
unlike every other corpus file in this repo, which opens with an explanatory
comment. A leading comment here does not weaken detection; it makes
`frontmatter::extract` return `None` entirely, so the payload silently reads
as an undetected miss rather than a corpus-authoring bug.

Per-file rationale therefore lives in this README, or after the closing
fence (which `extract` ignores and is safe), never before the opening one.

## Sourcing rule (unchanged from the parent corpus)

The same rule that governs `tests/corpus/attack/README.md` applies here
unchanged: payloads are written from the threat model and the relevant GitHub
issue (#33 for `tool-permission-abuse/`), never derived from the patterns or
a regex. A payload no pattern catches is not a bug in the corpus — it is the
corpus doing its job. See the parent `README.md` for the full sourcing rule.

## Collection

This directory (and every subdirectory inside it) is excluded from
`tests/recall_test.rs::categories()`'s top-level collection by its
`p.is_file()` filter — a subdirectory is invisible to that function entirely
(D-05). `structural_categories()` is the dedicated collector that walks this
directory instead, one level down: it enumerates the immediate subdirectories
of `structural/`, and for each one derives its recall row name by appending
`-structural` to the subdirectory's own name (`tool-permission-abuse/` ->
`tool-permission-abuse-structural`), rather than the row name being written a
second time as a literal anywhere. Renaming a category's subdirectory
therefore silently renames its pinned row too — `recall_test.rs`'s
`STRUCTURAL_CATEGORY` constant and
`the_structural_corpus_is_actually_collected`'s reproduction assertion exist
specifically to catch that as a loud test failure rather than a silent
divergence. A subdirectory with no matching `EXPECTED` row also fails loudly
(the two-sided guard in the same test) — a new category must be pinned
deliberately in the commit that adds its payloads, not absorbed silently into
an existing row's count.

## Payloads

### `tool-permission-abuse/` (CAT-01, #33)

| File | Shape |
|---|---|
| `01-wildcard-allowed-tools-block-sequence.md` | Claude Code skill file granting tools via a YAML block sequence, one entry of which is an unrestricted shell grant (`Bash(*)`) |

**WR-02 (carried over from Phase 3, still open):** this table documents only
1 of the 5 payloads in `tool-permission-abuse/`. That gap predates this
plan's directory-layout change and is not closed here — folding the
subdirectory split into the same commit as backfilling four missing rows
would make it harder to tell which change caused which diff. Still tracked as
a follow-up.
