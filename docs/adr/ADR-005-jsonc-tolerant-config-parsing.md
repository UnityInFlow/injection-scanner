# ADR-005: JSONC-tolerant config parsing, plus a visible parse-failure diagnostic

Date: 2026-09-15
Status: Accepted

## Context

Issue #129: a JSONC-commented `mcp.json` / `.mcp.json` / `claude_desktop_config.json`
is rejected outright by `serde_json::from_str`. `frontmatter::analyze()` returns `Err`,
the structural (fourth) pass is skipped under the FIX-03 skip-do-not-abort rule, and the
scan reports zero matches with **no diagnostic on stdout or stderr**. Nothing
distinguishes "this file is clean" from "this file could not be read".

Two separate failures live in that one bug report. The **silence** is the reported
symptom — a skipped structural pass that looks identical to a clean result is worse
than no scan at all, because it reads as a passing security check. The **detection
gap** is that a real, currently-published host family's config file — the VS Code /
GitHub Copilot IntelliJ `mcp.json` house style, which ships `//` comments inside
otherwise-valid JSON — is invisible to `PI060`/`PI061`/`PI062`, which is why the issue
also carries the `detection-gap` label. The scanner's own walker already lists
`jsonc` and `json5` in `DEFAULT_EXTENSIONS` (`src/walk.rs`), so the tool already
claims to scan a file class its parser cannot read.

This ADR is required on the `pr-artifacts` trigger for two reasons: it changes the
config parsing engine (`src/frontmatter.rs`), and it changes the `--format json`
output contract (`ScanReport` gains `config_parse_error`).

## Decision

Ship both halves, diagnostic first:

1. **Diagnostic.** `ScanReport` gains `config_parse_error: Option<String>`, additive
   and `skip_serializing_if = "Option::is_none"` so the pinned `--format json` key set
   is unmoved for every report that parsed cleanly. `Scanner::scan_with_confidence`
   records the `Err` arm of `analyze()` instead of discarding it (the exact `if let`
   that discarded it is what issue #129 reported). `Baseline::apply` carries the field
   across its report rebuild, so `--baseline` cannot silently erase it. `main.rs`
   prints `warning: structural config pass skipped for <file> — <error>`
   unconditionally — outside `if !quiet`, matching the existing coverage-gap warning
   — before `--write-baseline` handling so both output paths carry it. `.jsonl` /
   `.ndjson` files are exempted from the print (not from the underlying record): they
   are streams of independent documents, never one config file, so a per-file warning
   on every dataset entry would be pure noise. The diagnostic never moves the exit
   code — a parse failure is not a finding, and `spec-ci-plugin` keys on exit codes.

2. **JSONC tolerance.** `frontmatter::relax_jsonc` is a byte-offset and line-count
   preserving preprocessor: a single string-aware forward pass overwrites `//` and
   `/* */` comment bytes with ASCII spaces (never a newline, so line counts never
   shift), honouring backslash escapes so a comment marker inside a string literal
   survives untouched, followed by a second string-aware pass blanking a trailing
   comma before `}` or `]`. `frontmatter::parse()`'s `ConfigSyntax::Json` arm tries a
   strict `serde_json` parse first — a well-formed document pays nothing — and only on
   failure relaxes and retries, reporting the *relaxed* parser's error so its
   line/column still point at the right place in the original file. `extract()` also
   recognises a document whose first non-whitespace byte is `/` — a JSONC header
   comment before the opening `{` — gated on that single byte so an ordinary document
   pays one comparison.

## Consequences

### Positive

- The reported bug is closed: a config parse failure is visible on stderr, under
  `--quiet`, and under `--baseline`.
- The reported detection gap is closed: a `//`-commented `mcp.json` in the VS Code /
  Copilot house style produces the `PI060`/`PI061`/`PI062` findings it would produce
  without the comments.
- Every existing reported line number is unchanged, because relaxation never shifts a
  byte offset or a line break — `locate()`'s `block.body.lines()` arithmetic and every
  `ProjectedLine.line` stay valid against the original document.
- The `--format json` key set `spec-ci-plugin` reads today is byte-for-byte unchanged
  on every report that parses cleanly; only the rare broken-config report gains a key.
- A well-formed config document pays nothing extra: `relax_jsonc` only runs as a
  fallback after a strict parse has already failed.

### Negative / Trade-offs

- `.jsonl` / `.ndjson` files that are genuinely broken get no visible warning — the
  diagnostic is print-suppressed for line-delimited streams, on the grounds that they
  are not a single config document. The underlying `config_parse_error` is still
  recorded on the report; only the CLI's stderr line is suppressed.
- `locate()` still searches the raw (comment-included) `block.body`, so a commented-out
  `"url": ...` line above the real one can attract the reported line number. Unchanged
  from `locate()`'s existing best-effort contract ("a slightly wrong line is far better
  than a finding that cannot be located at all", `src/frontmatter.rs`).
- Two string-aware forward passes over the block body on the relaxation fallback path
  only — no cost on the strict-parse-succeeds path, and no cost on YAML/TOML, whose
  native comment support means the fallback is never invoked for those syntaxes.

## Alternatives Considered

**Warning only, no tolerance.** Rejected. A warning converts an invisible miss into a
visible miss, but the file in the reproduction is not malformed — it is the documented
house style of a shipping host. Warning about it every scan is not closing the gap; it
is annotating it, and the `detection-gap` label on the issue would remain earned.

**The `json5` crate.** Rejected. `json5` accepts a far wider grammar than comments and
trailing commas — unquoted keys, single-quoted strings, hex numbers, a leading `+` on
numbers — which would widen the accepted grammar of every config file this scanner
reads, past what any of the documented host families actually ship. It is also a new
crate dependency, itself an ADR-triggering change under the `pr-artifacts` skill, for a
grammar wider than the problem requires.

**A span-preserving JSON parser.** Rejected for this issue's scope. It would also solve
`locate()`'s best-effort best-effort line mapping (a commented-out duplicate key would
no longer attract the wrong line), but it is a parser rewrite, not a preprocessor, and
this issue does not justify that scale of change. The byte-offset-preserving
preprocessor gets the line-number invariant for free without touching the parser at
all — that property is what made the preprocessor cheap.

**Blank every `//` and `/*` by naive text search, no string tracking.** Rejected. It
gets the URL case wrong: `"url": "http://metrics.internal.example.com/mcp"` would have
its `//` blanked, silently deleting the payload the scanner exists to detect — the
exact failure T-129-01 in the plan's threat register names. String-literal awareness
with backslash-escape handling is the whole reason `relax_jsonc` is a state machine
rather than a regex substitution.
