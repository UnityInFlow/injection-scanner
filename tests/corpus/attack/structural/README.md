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
directory's root. `mcp-tool-poisoning/` is CAT-02's (#34) eight structural
payloads, added by Plan 04-02 without touching CAT-01's files, row name, or
pinned count at all — proof that a category gets its own subdirectory rather
than a filename prefix (`cat01-01-...md`) precisely because the collector
derives each category's recall row name directly from the subdirectory name
(see "Collection" below).

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

### `mcp-tool-poisoning/` (CAT-02, #34)

Landed by Plan 04-02, ahead of any `PI060`+ pattern (GATE-01). All eight are
whole-file JSON documents; wrapper shapes are spread deliberately across the
three real conventions measured in `04-RESEARCH.md` §Q1/§Q2 (Claude-family
`mcpServers`, VS Code family `servers`, and the wrapper-less form where the
server name is the top-level key).

| File | Shape |
|---|---|
| `01-emphasis-wrapped-description-file-smuggle.md` | Wrapper-less launch config; a tool `description` hides an `<IMPORTANT>` emphasis wrapper instructing a read of `~/.ssh/id_rsa`, smuggled through the `notes` argument, with a concealment clause (Invariant Labs shape) |
| `02-tool-listing-wire-shape-credential-smuggle.md` | The same file-read-and-smuggle attack expressed as a captured `tools/list` wire-shape document (`{"tools": [...]}`) rather than a launch config, targeting `~/.aws/credentials` |
| `05-version-gated-rug-pull-marker.md` | A rug-pull marker gated on a version comparison, embedded in a tool's `inputSchema` property description |
| `06-date-gated-post-review-rug-pull-marker.md` | A rug-pull marker gated on a date and on having passed review, embedded in a tool's `inputSchema` property description |
| `07-unpinned-npx-install-mcpservers.md` | Config-hygiene: an unpinned `npx -y <pkg>@latest` server install, under the Claude-family `mcpServers` wrapper |
| `08-non-tls-endpoint-servers-wrapper.md` | Config-hygiene: a non-TLS `http://` endpoint (synthesized — no naturally occurring example was found, per `04-RESEARCH.md` §Q2), under the VS Code family `servers` wrapper |
| `09-remote-script-fetch-execute-server.md` | Config-hygiene: a server launched by fetching and executing a remote script (`curl ... \| sh`), wrapper-less |
| `12-encoded-description-payload.md` | A base64-encoded file-read-and-smuggle instruction inside a `tools/list` `description` value — measures whether the E2 recursive decoder reaches `scope: frontmatter` projected values at all |
