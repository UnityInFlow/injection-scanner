# CAT-03 (#35) — Deferred items

Out-of-scope discoveries logged during Phase 5 execution so they do not survive only as tacit
knowledge in a SUMMARY. None is fixed here.

## 1. Multi-megabyte text files make `check --all-files` effectively hang (found in plan 05-01)

**What:** `injection-scanner check <path> --all-files --no-ignore` over a single minified
JavaScript bundle of roughly 4 MB or more does not finish within 25 seconds. Measured on
`$HOME/.cursor/extensions` (one file at a time, release binary at `1d73493`): 24 of the 60
largest files timed out, down to a 4.0 MB `dist/server/server.js`; a 14 MB
`shd101wyy.markdown-preview-enhanced-0.8.30-universal/out/native/extension.js` was one of them.
A whole-tree scan of that directory ran more than 9 minutes at 100% CPU before it was killed.
Non-text blobs (19-52 MB) are rejected as binary in 0.1s, so the cost is in the text path, not in
file I/O.

**Why it matters:** the default extension filter keeps `.js` out of an ordinary `check .`, so a
user rarely hits this; `--all-files` removes that protection, and it is what `scripts/gate03-sweep.sh`
always passes. It is a scan-time denial-of-service shape on attacker-supplied input if anyone
runs `--all-files` over a tree they do not control.

**Not diagnosed:** which stage is super-linear (the decoder, normalisation, the prefilter, a
regex on a single very long line) was not investigated.

**Workaround used:** plan 05-01's baseline sweeps frozen doc-type-only copies of the two extension
trees (see `sweep-baseline-05-01-2026-10-08/RAW-REPORTS.md`).

**Revisit when:** someone proposes `--all-files` as a CI default, or a user reports a hang.
No issue filed yet.

## 2. `locate()` maps every repeated config key to the first occurrence (found in plan 05-02)

**What:** when a projected configuration document repeats a key, the projection's `locate()`
resolves all of them to the line of the **first** occurrence. Measured on
`tests/corpus/clean/persistence-legitimate-hooks-config.json` (nested wrapper shape): a
binding-only `scope: frontmatter` probe produced 8 hits and every one reported **line 10**.

**Why it matters:** two consumer-visible mechanisms key on line numbers.
- `# injection-scanner:ignore` / `<!-- injection-scanner:ignore -->` suppression directives are
  resolved per line, so a directive aimed at one repeated binding suppresses by the wrong line,
  and a genuine second finding on another line cannot be suppressed independently.
- `.github/code-scanning-baseline.json` fingerprints embed the line, so several distinct
  findings collapse onto one baseline entry — accepting one accepts them all.

This lands squarely on the CAT-03 structural arm (`PI077`, plan 05-06), which is the first
pattern set to match repeated `command` / `bash` keys in one document at scale.

**Not diagnosed:** whether the first-occurrence collapse is in the projection's path->line map or
in how the structural pass reports a `ProjectedLine`. No fix attempted.

**Status:** no issue filed. Plan 05-07 Task 2 files the engine issues for this phase; this one
belongs in that batch alongside the whole-file YAML/TOML unprojected-config item.
