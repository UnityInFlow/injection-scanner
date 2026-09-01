# The structural attack corpus — whole-file payloads

`tests/corpus/attack/*.md` (the parent directory) is line-oriented: one file
per category, one payload per line, each line scanned as its own document.
That format cannot express a frontmatter payload at all — `payloads()` in
`tests/recall_test.rs` splits on `\n`, and a wildcard tool grant only exists
as an attack once it is sitting inside a real, parseable YAML/TOML/JSON
document. This directory is the second collection mode that exists to close
that gap (D-01).

## One file is one whole payload

Every file directly in this directory (other than this README) is scanned as
a **single document**, not split into lines. The recall harness's
`structural_payloads()` reads each file whole with `fs::read_to_string` and
never calls the line-splitting `payloads()` — that splitter is exactly what
this directory exists to bypass.

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
unchanged: payloads are written from the threat model and GitHub issue #33,
never derived from the patterns or a regex. A payload no pattern catches is
not a bug in the corpus — it is the corpus doing its job. See the parent
`README.md` for the full sourcing rule.

## Collection

This directory is excluded from `tests/recall_test.rs::categories()`'s
top-level collection by its `p.is_file()` filter — a subdirectory is
invisible to that function entirely (D-05). `structural_payloads()` is the
dedicated collector that walks this directory instead; this README is
excluded from it by name, exactly as `categories()` excludes the top-level
`README.md`.

## Payloads

| File | Shape |
|---|---|
| `01-wildcard-allowed-tools-block-sequence.md` | Claude Code skill file granting tools via a YAML block sequence, one entry of which is an unrestricted shell grant (`Bash(*)`) |
