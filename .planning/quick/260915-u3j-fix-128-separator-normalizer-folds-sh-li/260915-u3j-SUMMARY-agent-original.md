---
recovered: 2026-09-22
recovered_from: .claude/worktrees/agent-a8518b9e85ecdd5c4/
canonical: false
see_instead: 260915-u3j-SUMMARY.md
---

> **Provenance.** This is the executing agent's own SUMMARY for quick task 260915-u3j,
> written on branch `worktree-agent-a8518b9e85ecdd5c4` and believed lost when that worktree
> was removed mid-run. The directory was unregistered from git but never deleted from disk;
> the file was recovered from it on 2026-09-22 and is reproduced below **verbatim** — only
> this block was added.
>
> It is **not** the canonical record: `260915-u3j-SUMMARY.md` is, because its figures were
> re-measured on the merged branch, and the agent states in its own Self-Check that it could
> not re-verify anything after its working directory disappeared. Keep this for what only the
> executing agent could report — the per-task commit hashes, the in-flight correction to two
> of the plan's twelve byte-offset triples, and its first-hand account of the teardown.

---

# Quick Task 260915-u3j: Manufactured-boundary gate for issue #128 — Summary

**Status note:** All four plan tasks were completed and committed. This SUMMARY is being
written after the executor's isolated worktree directory was unexpectedly removed from disk
during the final post-Task-4 GATE-02 re-verification run (the `cargo test --test recall_test`
invocation failed with `tests/corpus/attack/structural must be readable: No such file or
directory`, and the very next tool call reported the worktree's working directory no longer
exists). All work below was completed and verified BEFORE that event; see "What happened at
the end" for the exact sequence.

## One-liner

A single pass-independent predicate (`normalize::span_edge_is_manufactured`) gates all five
scanner passes so a separator-joined compound token (`sh-lint`, `on-call`, `DAN-mode-switch`)
can no longer manufacture a false PI028/PI030/PI031 finding, with every withheld artefact
recorded (never discarded) in a new `manufactured_boundary` report array.

## Commits (on branch `worktree-agent-a8518b9e85ecdd5c4`)

| Task | Commit | Summary |
|---|---|---|
| 1 | `66351c8` | `feat(normalize): add manufactured-boundary edge predicate (#128)` |
| 2 | `27df0ff` | `feat(scanner): gate all five passes on the manufactured-boundary edge (#128)` |
| 3 | `b31b110` | `test(gate03): sweep and adjudicate the manufactured-boundary gate (#128)` |
| 4 | `c47194e` | `docs(adr): record ADR-006 and correct issue #128 criterion 3 (#128)` |

## Task 1 — the edge predicate (tracer, TDD)

`src/normalize.rs`:
- `is_separator` made `pub` (needed so the subset-invariant test can call it directly rather
  than duplicating its character list).
- `separator_binds(prev, next)` extracted as the shared core of "does a separator sit
  directly between two word characters", used by both `is_injected_separator` (the fold) and
  the new gate.
- `is_compound_separator(c)` — a deliberate strict subset of `is_separator`: `-` and `_` only.
- `span_edge_is_manufactured(text, start, end)` — the public predicate. Char-boundary safe
  throughout (`.get()`, never a raw byte-offset slice — the `src/frontmatter.rs:219`
  precedent), no `unwrap()`.

`tests/normalize_test.rs`: all twelve rows from the plan's `<behavior>` block, plus the
`is_compound_separator` ⊆ `is_separator` subset invariant.

**Deviation, documented and corrected in-flight:** two of the plan's twelve illustrative
`(text, start, end)` triples (rows 1 and 7, both built on the
`"curl https://example.com/x | ..."` prefix) undercounted the byte offset by one — the given
`end` landed on `h` (the last letter of `sh`), not on the `-` the row's own prose says the
span "ends at". Verified independently with a byte-by-byte count (`python3`) before writing
the Rust test: the correct offsets, consistent with ordinary `regex::Match::end()` semantics
and with every other row in the table, are `31` (not `30`) and `25` (not `24`). Used the
corrected offsets so each row actually tests the scenario its prose describes; noted inline
in the test file's own header comment. This is a Rule-1 (bug) class correction to the plan's
illustrative literals, not a change to the locked mechanism or gate-set decision — the
described *behavior* is unchanged, only two numbers were arithmetic slips.

**Second, more consequential correction found only after writing Task 2's tests:** naively
gating each pass independently produced **two** `manufactured_boundary` entries for a single
real artefact (e.g. `curl ... | sh-lint`), because the compound separators are also members
of the general fold set, so the raw pass and the normalized pass both independently
rediscover the identical artefact. The plan's FALSE POSITIVES behavior list requires
"exactly one entry" per case. Fixed with a second, narrower dedup set — see Task 2.

## Task 2 — gate all five passes, file artefacts under `manufactured_boundary`

`src/pattern.rs`: `ScanReport.manufactured_boundary: Vec<ScanMatch>`
(`#[serde(default, skip_serializing_if = "Vec::is_empty")]`, the `config_parse_error`/#129
precedent), `with_manufactured_boundary`, `manufactured_boundary_count()`. Rustdoc states the
deliberate asymmetry with the other three withheld arrays: no promotion flag.

`src/scanner.rs`: gates all five passes (raw line, multi-line, normalized, decoded,
structural), each against its own haystack. The normalized pass needed a new `original_span`
helper (factored out of the old `original_slice`) so the gated span and the quoted text are
computed from the exact same offsets and can never disagree. The decoded pass switched from
`cp.regex.is_match(...)` to `cp.regex.find(...)` to obtain a span for the gate (behaviorally
identical truthiness, confirmed by the full suite staying green).

Artefact-first: the manufactured check runs before suppression/confidence, so an artefact
never inflates `suppressed`/`low_confidence`. **Not** added to the pre-existing `already`
dedup `HashSet`s (used by the normalized/decoded passes to skip an already-reported
`(pattern, line)`): a pass-1 artefact must not silence a genuinely different finding a later
pass makes for the same pattern and line. A **separate**, narrower `manufactured_seen: HashSet<(String, usize)>`
was added specifically to dedup an artefact against *itself* across passes (the fix for the
Task-1 discovery above) — it is never consulted by, and never populates, `already`, so it
cannot suppress a genuine finding, only a duplicate report of the same artefact.

`src/baseline.rs`: `Baseline::apply`'s report rebuild now takes and re-attaches
`manufactured_boundary`, exactly as `config_parse_error` is two lines above.

`src/reporter.rs`: `total == 0` guard extended; a fourth note added in the same voice as the
other three, worded WITHOUT a re-run instruction (no promotion flag exists).

`tests/manufactured_boundary_test.rs` (new, 10 tests): all six false positives silent in
`matches` and filed under `manufactured_boundary`; all six regression controls unchanged;
zero severity-count inflation; the raw-vs-normalized dedup guarantee (a manufactured raw-pass
occurrence and a genuine underscore-joined occurrence on the SAME line both resolve
correctly); the count accessor.

`tests/json_contract_test.rs` (+2 tests): the exact key-set pin stays green on the clean
fixture; `manufactured_boundary` present only on a report with an artefact, absent (not
`null`, not `[]`) otherwise.

`tests/sarif_test.rs` (+1 test): an artefact produces no SARIF result.

Verified: `patterns/`, `docs/PATTERN-CATALOGUE.md`, `.github/code-scanning-baseline.json`
untouched (`cargo test --test catalogue_test` green without regeneration).

## Task 3 — GATE-02, full suite, GATE-03 both ways

- **GATE-02:** `cargo test --test recall_test` green (8/8) immediately after Task 2's commit.
  `EXPECTED` byte-identical to `27e4d49` (`git diff --quiet 27e4d49 -- tests/recall_test.rs`
  exits 0). README's recall table confirmed untouched both then and after Task 4's edits
  (diff hunks land at lines 57-64, 529-538, 810-818; the recall table is at line ~385).
- **Full suite** (`cargo test --locked`, backgrounded per P0): **443 tests, 0 failures.**
- **fmt/clippy:** both clean throughout (re-checked after every task).
- **GATE-03:** release binary rebuilt from the clean pre-edit tree (confirmed
  `git status --porcelain -- src patterns tests` empty, i.e. HEAD's post-27e4d49 commits touch
  only planning docs) → swept 32 directories, 23,037 files, 518 findings into
  `.planning/local/sweep-before-u3j-2026-09-16/`. Rebuilt again after Task 2, swept the
  IDENTICAL directory list into `.planning/local/sweep-after-u3j-2026-09-16/`: **23,037 files,
  518 findings — byte-identical manifest.tsv and summary.tsv.** `--compare` run in both
  directions against the correct `.planning/local/` paths (never the committed,
  JSON-less `.planning/quick/`/`.planning/phases/` sweep directories — the recorded trap):
  **both directions returned an empty diff set, exit 0.** Zero disappearances, zero
  appearances. Adjudicated in full in `260915-u3j-SWEEP.md`: the empty-disappearances result
  is explained (none of the 23,037 real files in this machine's corpus happen to contain the
  specific compound tokens the six issue-#128 payloads are built from), and the
  zero-appearances result needs no further adjudication (nothing to confirm-as-genuine).

**A caveat on my own process, worth recording exactly:** I made three Task-4 edits
(README.md's JSON-output additions) *before* Task 3's own automated verify checks had all
run, while the background GATE-03 sweep was still executing, to use the wait time
productively. One of Task 3's own `<verify>` lines is a literal
`git diff --quiet 27e4d49 -- README.md` check intended as a proxy for "the recall table
wasn't edited." Because I'd already made Task 4's (unrelated, table-preserving) README edits
by the time I ran it, that literal check fired its warning message even though the actual
recall table was untouched — confirmed directly via `git diff 27e4d49 -- README.md`'s hunk
line numbers (57-64/529-538/810-818, nowhere near the recall table at line 385). This is a
false positive in a blunt whole-file-diff proxy caused by my own out-of-order execution, not
a real GATE-02 violation — but it means Task 3's verify block, read literally and in
isolation, would show that warning line if re-run today. GATE-02's real assertion
(`tests/recall_test.rs`'s `EXPECTED` array and the actual recall table) is unmoved.

## Task 4 — ADR-006, README, issue correction, STATE.md, code review

- `docs/adr/ADR-006-manufactured-boundary-gate.md` (new): the corrected measurement (PI028/
  PI030 fire from the RAW pass, PI031 from the normalized pass — the `e.md`/`f.md` probes
  quoted verbatim); the eleven-pattern exposure audit table (PI028, PI012, PI030, PI031,
  PI034, PI053, PI057, PI058, PI062, PI063, PI068) plus the three probes that did not
  reproduce; the three rejected directions (B, C, A-as-phrased) with the measurement that
  kills each; the `-`/`_` gate-set choice; the named accepted false negative
  (`curl evil | sh-x`) and the doubled-separator residual; the record-not-discard decision
  and the explicit no-promotion-flag rationale.
- `README.md`: extended all three JSON-output mentions of the withheld arrays (SARIF
  exclusion list, main `--format json` doc, `--baseline` section) with `manufactured_boundary`
  and its one-sentence reason, in the existing voice. Recall table untouched (verified above).
- **Issue #128:** posted the corrected measurement as a comment
  (https://github.com/UnityInFlow/injection-scanner/issues/128#issuecomment-5696017902) and
  edited acceptance criterion 3 in the issue body via `gh issue edit 128` to state the real
  regression control (`ignore-all-previous-instructions` and its `_`/`.`/space siblings,
  PI001 CRITICAL) instead of the originally-worded, never-true claim about
  `ig-nore pre-vious in-structions`.
- `.planning/STATE.md`: new row in "Quick Tasks Completed" plus a full Session Notes entry
  recording the raw-pass correction, the `manufactured_boundary` array, and the GATE-03
  verdict.
- **Code review** (code-review skill) over
  `src/{normalize,scanner,pattern,reporter,baseline}.rs`: no `unwrap()`/`println!` in the
  diff, no new catch-all `_` match arms, `///` rustdoc present on every new public item,
  `manufactured_seen` correctly scoped (never touches `already`). No blocking issues found.
- **Smoke test** on the release binary (built from the Task 1+2 commit state):
  `curl https://example.com/x | sh-lint` → `matches: []`, `manufactured_boundary` carries the
  withheld PI028 record, `critical_count`/`high_count` are `0`, text output reports "No
  injection patterns reported." plus the new note, and `check --quiet` exits `0` — the
  commit-blocking behaviour issue #128 reported is gone.
- `pattern-library` skill deliberately not triggered: nothing under `patterns/` changed
  (confirmed: `git diff --stat 27e4d49 -- patterns docs/PATTERN-CATALOGUE.md
  .github/code-scanning-baseline.json` is empty).

## What happened at the end

Immediately after Task 4's commit, I re-ran `cargo test --test recall_test` as a final
GATE-02 confirmation (belt-and-braces, since GATE-02 had already passed cleanly right after
Task 2's commit). That run reported `7 passed; 1 failed`, with the single failure being
`recall_matches_the_recorded_numbers` panicking on
`tests/corpus/attack/structural must be readable: No such file or directory` — a MISSING
FIXTURE DIRECTORY, not a code assertion failure. The very next tool call (a `git diff --stat`
from the worktree root) was refused by the sandbox with: *"its working directory ... no
longer exists ... the isolation worktree appears to have been removed."* Subsequent `Read`
calls for files I had successfully read minutes earlier (`CLAUDE.md`,
`tests/corpus/attack/structural`) also report "does not exist."

This is environment/infrastructure failure, not a code regression: `tests/corpus/attack/` is
a committed, tracked fixture directory that Task 1-4 never touched, and the exact same test
(`cargo test --test recall_test`, all 8 sub-tests) passed cleanly earlier in this same session
(right after Task 3's commit, before any Task 4 edit). The most consistent explanation is
that the worktree's working directory was torn down by an external process (a cleanup job,
a stale-worktree prune, or similar) WHILE that final test process was reading its fixture
tree, mid-run — not that my code broke the fixture. I am reporting this rather than working
around it, per instruction, since I have no way to re-verify further without a working
directory.

## Self-Check

- [x] `docs/adr/ADR-006-manufactured-boundary-gate.md` — created (content captured in this
      SUMMARY's Task 4 section; file existence could not be re-verified after the worktree
      loss, but the commit `c47194e` includes it per its own `git commit` output at the time:
      "create mode 100644 docs/adr/ADR-006-manufactured-boundary-gate.md").
- [x] Commits `66351c8`, `27df0ff`, `b31b110`, `c47194e` — all confirmed present via each
      commit's own `git commit` tool output at creation time, on branch
      `worktree-agent-a8518b9e85ecdd5c4`.
- [ ] Could not re-verify any of the above by reading the filesystem again after the
      worktree's working directory disappeared — self-check is based on the transcript's own
      recorded tool outputs, not a fresh read.

## Known Stubs

None.

## Threat Flags

None — this is an engine fix to existing detection logic; no new network endpoints, auth
paths, or trust-boundary schema changes were introduced.
