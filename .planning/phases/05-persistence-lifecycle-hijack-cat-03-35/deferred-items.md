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

## 3. `tools/corpus-derivation-check.py` is not wired to anything and overstates what it proves (found by external review of waves 1-2)

**What:** three separate weaknesses, all verified:
- **It passes on whatever subset it is handed.** Given only the prose file it exits 0. It never
  asserts that it was pointed at the full 7 prose + 5 structural payload set, so an invocation that
  silently covers half the corpus is indistinguishable from one that covers all of it. Its own
  success line reports `{n} payload file(s)` — whatever `n` happened to be.
- **Nothing runs it.** `grep -rn 'corpus-derivation-check' .github/ tests/ scripts/` returns no
  hits; it is referenced only in prose, in `tests/corpus/attack/README.md:58` and the payload file
  header. So GATE-01's only mechanical check is a thing a human has to remember to run.
- **Its header oversells.** It detects lexical copying — shared 5-grams and high token-Jaccard
  sentences. It does not and cannot detect independent derivation: a payload rebuilt from a
  barred source's *shape* with entirely fresh wording passes it cleanly. That is exactly the gap
  the waves 1-2 review found, and the reason the held-out set in `heldout-set.md` exists.

**Why it matters:** the first two make the check skippable and partially-satisfiable; the third
means a green run should never be read as "GATE-01 satisfied", only as "no wording was lifted".

**Fix shape:** assert a minimum payload count (or take the corpus root and enumerate it itself),
wire it into `ci.yml` and/or a test, and rewrite the header to claim lexical non-copying only.

**Status:** no issue filed. Belongs in plan 05-07 Task 2's batch with the other two engine items.

## 4. Nits from the same review (no action needed beyond wave 7's existing rewrite)

- `README.md` recall table still carries an "as of 2026-09-03" date. Plan 05-07 republishes these
  numbers anyway, so it is fixed there rather than separately.
- The structural corpus is described as "five host conventions"; it is strictly 4 host families
  across 5 distinct wrapper/command-key shapes. This matches plan 05-01's own definition of the
  requirement ("no two may share a wrapper and command-key shape"), so it is wording only.

## 5. `PI070`'s subject alternation has no leading word boundary, so `LaunchAgents` supplies the subject `Agents` (found in plan 05-03)

**What:** the subject group `(?:the\s+)?(?:ai\s+)?(?:agents?|assistants?|models?|llms?|claude|copilot)\b` is not
preceded by `\b`, so it can start in the middle of a word. Measured on both the pre-edit binary
and the widened one: `Files in ~/Library/LaunchAgents will install the helper at login via
launchctl.` fires `PI070` HIGH, with the trailing `Agents` of `LaunchAgents` acting as the subject
and `will` as the modal. `The LaunchAgents folder should register the job with launchctl at boot.`
fires the same way.

**Why it matters:** it is pre-existing, but plan 05-03's object widening makes it reachable by more
sentences (every new object is another way to complete the match once a mid-token subject is
found). The sweep over 26,417 third-party files produced no `PI070` finding, so it is a latent
false positive, not a measured one.

**Not fixed:** plan 05-03 requires `PI070`'s subject alternation to stay byte-identical (narrowing
it is its own change with its own false-positive story, GATE-04). The fix is a leading `\b` (or
`(?:^|[^\w])`) on the subject, plus a clean-corpus specimen and a pattern_test negative.

**Status:** no issue filed.
