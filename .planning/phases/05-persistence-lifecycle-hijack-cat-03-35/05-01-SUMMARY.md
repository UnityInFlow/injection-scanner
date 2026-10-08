---
phase: 05-persistence-lifecycle-hijack-cat-03-35
plan: 01
subsystem: testing
tags: [gate-01, gate-03, recall-corpus, frontmatter, lifecycle-hooks, rust]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    provides: scripts/gate03-sweep.sh, the 32-row sweep list, the structural-corpus layout
provides:
  - GATE-03 pre-edit baseline for CAT-03 (33 rows, 26,417 files, 1,306 findings) at sha 1d73493
  - twelve threat-model CAT-03 payloads (7 prose, 5 structural) written before any PI071+ pattern
  - tools/corpus-derivation-check.py, a committed GATE-01 derivation gate
  - the measured pre-pattern score, prose 0/7 and structural 1/5, with every hit attributed
  - the published recall denominator at 115
affects: [05-02, 05-03, 05-04, 05-05, 05-06, 05-07]

actuals:
  tokens: 17700
  tasks: 3
  commits: 4

tech-stack:
  added: []
  patterns:
    - "two-wrapper-shape projection probe with a path-anchored negative control (lifecycle hooks)"
    - "derivation gate: 5-word n-gram / 0.6 token-Jaccard against barred text, read by the script not the author"

key-files:
  created:
    - tools/corpus-derivation-check.py
    - tests/corpus/attack/structural/persistence-lifecycle-hijack/ (5 payloads)
    - .planning/phases/05-persistence-lifecycle-hijack-cat-03-35/sweep-baseline-05-01-2026-10-08/
    - .planning/phases/05-persistence-lifecycle-hijack-cat-03-35/deferred-items.md
  modified:
    - tests/corpus/attack/persistence-lifecycle-hijack.md
    - tests/recall_test.rs
    - README.md
    - examples/persistence-lifecycle-hijack-attack.md
    - examples/README.md
    - tests/corpus/attack/README.md
    - tests/corpus/attack/structural/README.md
    - .github/code-scanning-baseline.json

key-decisions:
  - "Pinned detected counts in the Task 2 commit rather than leaving recall_test red until Task 3 (never commit failing tests)"
  - "Widened the hooks-config collection roots beyond the four named agent roots, which yield only 34 files on this machine"
  - "Replaced the two extension-tree rows with frozen doc-type-only copies because --all-files over multi-MB JS bundles does not finish"

requirements-completed: [CAT-03, GATE-01, GATE-02, GATE-03]

duration: ~55min
completed: 2026-10-08
status: complete
---

# Phase 5 Plan 01: GATE-03 baseline and the twelve CAT-03 payloads Summary

**CAT-03 now has a pre-edit GATE-03 baseline and twelve blind-written payloads whose measured pre-pattern score is prose 0/7 and structural 1/5 (total 97/115, 84.3%); the old row's 6/6 was one template caught by PI070, and no pattern ships.**

## Commits

| # | Hash | What |
|---|---|---|
| 1A | `c64ee60` | GATE-03 pre-edit baseline: manifest, summary, checksums, RAW-REPORTS.md, deferred-items.md |
| 1B | `5e347d8` | payload 01 end-to-end, the `persistence-lifecycle-hijack-structural` row, the two-shape projection probe |
| 2 | `3ac45a7` | payloads 02-05 and the rewritten prose file, both READMEs, `tools/corpus-derivation-check.py`, EXPECTED re-pinned |
| 3 | `689fe30` | README denominator 109 -> 115, examples rewrite, code-scanning baseline, EXPECTED comments |

## Sweep baseline (Task 1, commit A)

- **Binary built from:** `1d734937410c8025c9d484834a505d2154511199`. Precondition `git status --porcelain -- src patterns tests examples Cargo.toml Cargo.lock` printed nothing. `cargo build --release --locked` finished in 38.60s.
- **Totals:** 33 rows, **26,417 files, 1,306 findings**, 0 rows dropped. Sweep wall time 207s (baseline run) and 221s (stability run).
- **Stability probe** (two back-to-back runs of the identical list on the unmodified tree), quoted from the log:
  ```
  COMPARE sweep-stab-a -> sweep-baseline-05-01-2026-10-08 rc=0
  COMPARE sweep-baseline-05-01-2026-10-08 -> sweep-stab-a rc=0
  ```
  Both empty; `diff` of the two `manifest.tsv` and the two `summary.tsv` identical.
- **Hermeticity checks:** `manifest.tsv` has 33 data rows; `awk -F'\t' '$1 ~ /^\$HOME\/\.(claude|codex|cursor|gemini)$/' manifest.tsv | wc -l` prints `0`; the committed directory holds 0 `*.json`; `.planning/local/sweep-baseline-05-01-2026-10-08/` holds 33. A `find` over the frozen hooks set for session/snapshot/todo/telemetry/`node_modules`/`target` segments prints `0` of 263 files.

### Row substitutions and deviations (all recorded in RAW-REPORTS.md)

1. **Extension trees: the plan's substitution was attempted and failed, for a reason other than its fallback trigger.** The plan substitutes the real `~/.cursor/extensions` and `~/.vscode/extensions` roots and falls back only if the binary exits above 2. There was no crash. The `~/.cursor/extensions` row (39,738 files, 1.4 GB) ran over 9 minutes at 100% CPU without finishing, so I killed it. A per-file probe over the 60 largest files found **24 of 60 timed out at 25s** (23 `.js`, 1 `.map`; smallest 4.0 MB). Both rows became frozen doc-type-only copies under `.planning/local/sweep-inputs-ext-05/` (4,655 / 5,970 files copied, 1,234 / 1,593 counted by the scanner). The slowness itself is logged in `deferred-items.md`, undiagnosed.
2. **Hooks-config row: the plan's ">=100 files from the four named roots" is not achievable.** The four roots yield **34** files after the exclusions. I applied the same collection rule to `~/.config` and `~/Documents/workspace-1-ideas` (the researcher's own 328-file method), reaching **263 files, 59 findings** (all `PI051`). Exclusion list and rule are in RAW-REPORTS.md.
3. **Path-keyed `--compare` caveat (needs the orchestrator).** `--compare` keys on the absolute file path inside each JSON report. The three repo-local rows (hooks set, two extension copies) embed this disposable worktree's path, so a later plan in a different checkout will see those three rows as wholesale add/remove. RAW-REPORTS.md gives two ways to handle it. **The raw JSON and the frozen inputs live under gitignored `.planning/local/` in this worktree and are lost when it is removed.** I saved a copy: `/private/tmp/claude-501/-Users-jirihermann-Documents-workspace-1-ideas-unity-in-flow-ai-03-injection-scanner/075878c2-0f94-4e7d-bfd0-0572587c1bcd/scratchpad/gate03-local-05-01.tgz` (23 MB, contains `.planning/local/`). Recovery steps are also committed.

## The corpus: 12 payloads, 7 prose and 5 structural

No ratio was set in advance; this is what the threat model produced.

| # | Where | Host / genre | Bullet | Pre-pattern result |
|---|---|---|---|---|
| P1 | prose line 1 | support KB page, passive obligation, automated-reader byline, `~/.zprofile` | 3 | miss |
| P2 | prose line 2 | vendor setup page, anaphoric "it", LaunchAgents | 3 | **deliberate miss** |
| P3 | prose line 3 | issue-thread comment, verb "stick", crontab | 3 | miss |
| P4 | prose line 4 | contributor-bot README, bare-imperative hook install | 4 | **deliberate miss** |
| P5 | prose line 5 | knowledge-base chunk, bare-imperative write into `GEMINI.md` with a future-reader cue | 1 | miss |
| P6 | prose line 6 | release-note footnote, trigger-conditioned memory write | 1 | miss |
| P7 | prose line 7 | prompt-library page, self-propagation with an instruction-class object | 5 | miss |
| S1 | `structural/.../01-session-hook-secret-path-read.md` | Codex **mixed** root-and-`hooks` shape, `command`, attack on the root side | 2 | **hit (PI029, prose spillover)** |
| S2 | `02-skill-frontmatter-hook-appends-shell-profile.md` | Claude skill YAML frontmatter, nested, `command` | 2/3 | miss |
| S3 | `03-copilot-hooks-json-authorized-keys-append.md` | Copilot flat, `bash` key | 2/3 | miss |
| S4 | `04-http-handler-remote-lifecycle-endpoint.md` | Claude nested `type: http`, `url` | 2 | miss |
| S5 | `05-cursor-flat-hook-posts-session-to-webhook.md` | Cursor flat camelCase, `command`, plain webhook | 2 | **deliberate miss** |

Five distinct (format, wrapper shape, command key) tuples, checked by script: `('json','mixed(root+wrapper)','command')`, `('yaml-frontmatter','wrapper-nested','command')`, `('json','wrapper-flat','bash')`, `('json','wrapper-nested','url')`, `('json','wrapper-flat','command')`. Prose lines + structural files = 7 + 5 = 12; no occurrence of PI070's `example` in any payload (script count `0`). Document genres: support KB, vendor setup page, issue thread, contributor README, release note, prompt-library page (six).

## Measured pre-pattern baseline (Task 3)

`recall_matches_the_recorded_numbers` reported, before the pin:

```
persistence-lifecycle-hijack  0/7     0.0%
persistence-lifecycle-hijack-structural  1/5    20.0%
TOTAL                  97/115   84.3%
```

The prose row fell from 6/6 to **0/7**. That is GATE-01 working: the six replaced payloads, scanned individually on the same binary, were `PI070 HIGH` on all six and `PI014 MEDIUM` co-firing on payloads 3 and 6, and payload 1 equals PI070's `example` (`line1==PI070 example: True`).

### Every detection, attributed

Scanned individually with the release binary in JSON mode, counting reported, low-confidence and suppressed findings:

| Payload | Pattern | Severity | Match context | Confidence | Matched text |
|---|---|---|---|---|---|
| S1 `01-session-hook-secret-path-read.md` | `PI029` email-the-contents | HIGH | **prose** | 1.0 | `cat ~/.ssh/` (line 7) |
| P1-P7, S2-S5 | none | - | - | - | "no findings at all", no low-confidence, no suppressed |

**Prose spillover finding:** the only structural payload counted (S1) is reached by `PI029` running over the raw JSON line, not by any `scope: frontmatter` pattern; none exists for this category. A later structural pattern therefore cannot claim S1 as new recall. So the structural arm's real headroom is the four payloads at zero. S1 was authored blind with a `cat` of the key path; a `cp` form (the one in `05-CONTEXT.md` D-03) was not tried, so I make no claim about whether it would have escaped `PI029`.

### Recall pins and published numbers

- `EXPECTED`: `("persistence-lifecycle-hijack", 0, 7)` and `("persistence-lifecycle-hijack-structural", 1, 5)`; rows sum to 12.
- README: CAT-03 row `1 / 12`, `8.3%`; total `97 / 115`, `84.3%`. A script sum over the table prints `(97, 115)` against the total row `(97, 115)`. Occurrences of the previous denominator `109` in the current README: `0`.
- The Pattern Categories table count and the library-size sentence are untouched.

## Probe and mutation check

`the_projection_reaches_both_lifecycle_wrapper_shapes` (new): a `hooks`-segment frontmatter probe fires on the wrapper form, the purely root-form document and the committed mixed payload; a path-anchored twin fires on the wrapper form and on the mixed payload's wrapped half, and stays silent on the root form and on the mixed payload's attack line. It also asserts the committed payload projects at least one line.

Mutation (rewrote the segment probe to `^hooks\.[^=\s]*command\s*=\s*\S`) made the test fail:

```
thread 'the_projection_reaches_both_lifecycle_wrapper_shapes' panicked at tests/recall_test.rs:1002:5:
segment probe must fire on the wrapper-less root-form shape (events at the document root, `hooks` appearing only as the inner handler list)
test result: FAILED. 0 passed; 1 failed
```

Restored; the test passes.

## Derivation check (GATE-01)

`python3 tools/corpus-derivation-check.py --barred 1d734937410c8025c9d484834a505d2154511199:tests/corpus/attack/persistence-lifecycle-hijack.md --payloads <the prose file and the 5 structural files>`:
```
OK: 6 payload file(s) share no 5-word run and no > 0.6 token-Jaccard sentence with 1 barred source(s).   (rc=0)
```
`... --barred .planning/phases/05-persistence-lifecycle-hijack-cat-03-35/05-RESEARCH.md --payloads <same>`:
```
OK: 6 payload file(s) share no 5-word run and no > 0.6 token-Jaccard sentence with 1 barred source(s).   (rc=0)
```
(6 files carry the 12 payloads: the prose file holds 7.)

**Seen to fail** (planted lifts, scratch files removed): an inherited payload with `issue` changed to `problem` exited `1` with 13 collisions, e.g. `shares the 5-word run 'agent must append the key' with 1d73493...:tests/corpus/attack/persistence-lifecycle-hijack.md line 12` and `token-Jaccard 0.88 (> 0.6)`. A sentence from the research notes' allowed §Q2 (`live-reloads settings ...`) exited `1` with 15 collisions against `05-RESEARCH.md line 393`. I did not plant a lift from the barred §Q1 or Appendix A, since that would have shown me their text.

**Blindness statement.** I never read `05-RESEARCH.md` §Q1 (lines 289-374) or Appendix A (line 838 to end). I read only §Q2 (375-424), §Q6 (548-597) and the closing verification pass (879-909). A `grep -n '^## '` for section headings showed me the heading text of Q1 and Appendix A, nothing under them. The check's report names the payload-side n-gram and the barred line number only, never barred text. `05-CONTEXT.md` D-03/D-04 quote a few attack sentences (e.g. a `cp ~/.ssh/...` hook, a "Write these rules to CLAUDE.md" line); I did not reuse them, and neither check found overlap.

## Verification

| Gate | Result |
|---|---|
| `cargo test --locked` (background, full) | 39 binaries, **459 passed, 0 failed** (>= 458; +1 is the new probe test), exit 0 |
| `cargo test --test recall_test --locked` | 9 passed (Tasks 1 and 2 and 3) |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --all-targets --locked -- -D warnings` | clean |
| `git diff --stat 1d73493 HEAD -- patterns Cargo.toml Cargo.lock .planning/STATE.md .planning/ROADMAP.md` | 0 lines; `patterns/core/` lists the same 9 files |
| `markdown_context_test` pin for the examples file | `grep -c 'persistence-lifecycle-hijack-attack' tests/markdown_context_test.rs` prints `0` before and after |
| `no_new_pattern_escapes_the_attack_corpus` | passes; the rewritten example still fires `PI070` (1 HIGH, line 5), which is why it keeps one fresh `PI070`-shaped line |

**Self-scan one-liner did not print `[]`.** It printed `[('./docs/PATTERN-CATALOGUE.md', 77, 'PI001'), ('./docs/PATTERN-CATALOGUE.md', 890, 'PI031')]`. These are the two standing findings `.planning/.continue-here.md` records (line numbers have drifted from 73/902); no `docs/` file was touched by this plan, so they are pre-existing rather than introduced, but I did not check them against the base commit.

## Deviations from Plan

**1. [Rule 3 - Blocking] Extension-tree rows replaced with frozen doc-type copies** (Task 1, above). Commit `c64ee60`.

**2. [Rule 3 - Blocking] Hooks-config roots widened** (Task 1, above). Commit `c64ee60`.

**3. [Rule 1 - never commit a failing test] Detected counts pinned in Task 2, not Task 3.** The plan leaves them for Task 3; doing so would put a red `recall_test` in git history. The Task 2 commit pins the measured 0/7 and 1/5; Task 3 adds the attribution, README and comments. Commits `3ac45a7`, `689fe30`.

**4. [Rule 1 - accuracy] "Eleven of twelve are synthesis" became "all twelve".** The plan's header wording would have claimed one sourced payload; none is a transcription. The prose file header says all twelve are synthesis with a cited mechanism. Commit `3ac45a7`.

**5. [Rule 2 - stale claim] `examples/README.md` row edited** (not in the plan's file list): its `3 HIGH` for the persistence example became false the moment the replaced payloads left it. Now `1 HIGH`. Commit `689fe30`.

**6. Code-scanning baseline regeneration pruned 26 unrelated entries** (25 `docs/DETECTION-BACKLOG.md`, 1 `patterns/core/multilingual.yaml`), alongside the expected removals (8 corpus-file, 2 examples-file) and 1 added entry. The workflow comment says stale entries should be pruned. Not independently confirmed that CI's environment produces none of them. Commit `689fe30`.

**7. Wording fix:** the attack README and prose header first said "`PI070` alone caught all six"; measured, `PI014` also fired on two. Corrected before commit.

## Known Stubs

None. Three corpus payloads are intentional documented misses, not stubs; they are named in the prose file's header and the structural README.

## Threat Flags

None. No new endpoint, auth path or trust boundary. `tools/corpus-derivation-check.py` reads files and runs `git show <rev>:<path>`; it writes nothing.

## Open items for the orchestrator

1. **Preserve `.planning/local/`.** Copy `.planning/local/sweep-baseline-05-01-2026-10-08/` and the two `sweep-inputs-*` directories out of this worktree before it is removed, or restore from the tarball above. See the path-keyed caveat in RAW-REPORTS.md: until the three repo-local rows are re-captured from the checkout that runs later comparisons, plans 05-03..05-07 will see those rows as full additions and removals.
2. `deferred-items.md` item 1 (multi-MB text files and `--all-files`) has no issue filed.
3. `STATE.md` and `ROADMAP.md` untouched, per the brief.

## Self-Check: PASSED

- Created files verified present: `tools/corpus-derivation-check.py`, all five `structural/persistence-lifecycle-hijack/0N-*.md`, `sweep-baseline-05-01-2026-10-08/{manifest.tsv,summary.tsv,checksums.sha256,RAW-REPORTS.md}`, `deferred-items.md`.
- Commits verified in `git log`: `c64ee60`, `5e347d8`, `3ac45a7`, `689fe30`.
- Not self-checked beyond the above: the SUMMARY commit itself (made immediately after this file is written).
