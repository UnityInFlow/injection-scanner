---
phase: 05-persistence-lifecycle-hijack-cat-03-35
reviewed: 2026-10-09T00:00:00Z
depth: standard
skill: .claude/skills/code-review/SKILL.md (Rust checklist; output format summary / blocking / suggestions)
files_reviewed: 6
files_reviewed_list:
  - patterns/core/persistence-lifecycle-hijack.yaml
  - tests/pattern_test.rs
  - tests/recall_test.rs
  - tools/corpus-derivation-check.py
  - tests/corpus/attack/persistence-lifecycle-hijack-heldout.md
  - tests/corpus/attack/structural/persistence-lifecycle-hijack-heldout/
findings:
  blocking: 1
  suggestions: 5
  total: 6
status: resolved
blocking_resolution: fixed-in-phase (BL-01 resolved 2026-10-09; see 'BL-01: RESOLVED')
---

# Phase 5: Code Review — CAT-03 (PI070-PI079, #35)

**Reviewed:** the phase's changed source and test files (`git diff 351ba51 HEAD`, merge base with `main`), with the pattern
library probed as a binary rather than read as text.
**Status:** `resolved` — one BLOCKING finding, raised here as deferred, then **fixed in-phase on the maintainer's decision**
(2026-10-09). Read BL-01 first, then `BL-01: RESOLVED` below for the fix and its measured cost.

## Summary

`src/`, `Cargo.toml` and `Cargo.lock` are untouched by the phase, so the Rust production checklist items (no `unwrap()`, no
`println!`, `anyhow` vs `thiserror`, `clap` derive, rustdoc on public items, `OnceLock` regex compilation, user-friendly errors)
have nothing in this phase's diff to breach. What the phase changes is a YAML pattern file, two Rust test files, a Python
tool and a corpus; the review therefore concentrated on the items a pattern phase *can* breach, and it ran the **release
binary** against constructed sentences rather than reading regexes, because the repository's own history (the `PI071` and
`PI076` repairs, deferred items 6 and 9b) says that reading a pattern and a green sweep are not evidence on this question.

Probe set (script and inputs reproducible; run on the release binary built from the committed pattern set):

- 35 benign prose probes: prohibitions in every form (`never`, `do not`, `should not`), human-addressed install text, workflow
  text of this ecosystem, memory-feature prompts, hook documentation, vendor product sentences, product-noun agents.
  Result: **33 silent, 2 fire `PI070` HIGH**: the known product-noun `Jenkins agent` case (#168) and a memory-feature sentence.
- 8 legitimate structural documents (logging hooks, formatters, a chat webhook, `ssh-add`, a `.pub` read, a JSON pretty-printer
  pipe, a script path, an MCP server). Result: **8 silent**.
- A second probe of 17 vendor-feature sentences with a third-person AI subject and `will` / `should` / `needs to`.
  Result: **16 fire `PI070` HIGH, 13 of them new relative to the pre-phase pattern.** That is BL-01.

Checklist results:

| Item | Result |
|---|---|
| No `unwrap()` in production code | N/A: no production file changed. In tests, `.expect(msg)` / `unwrap_or_else(panic!)` carry messages |
| No `println!` / debug output | Pass: a scan of the diff for `println!`, `dbg!`, `todo!`, `#[ignore]`, `TODO`, `FIXME` over both test files and the YAML is empty |
| Exhaustive matching, no catch-all `_` | N/A: no new `match` in the diff (a scan for `_ =>` is empty) |
| `///` rustdoc on public items | N/A: no public item added; the new `is_heldout` / `HELDOUT_MARKER` are private and documented |
| Regexes compiled once | N/A in `src/`. See S-01 for the test-side equivalent |
| YAML pattern files validate on load | Pass: the loader, `pattern_policy_test` and `pattern_example_test` are green |
| Tests: true positives AND non-matches per pattern | Pass: counted from `tests/pattern_test.rs`, positives / negatives per pattern: `PI070` 27/13, `PI071` 11/21, `PI072` 9/15, `PI073` 10/14, `PI074` 9/11, `PI075` 4/7, `PI076` 8/19, `PI077` 8/7, `PI079` 4/6 (floor is 3 / 2). **But see BL-01: the test sets do not contain the sentence class that fires.** |
| `cargo fmt` clean | Pass (`FMT_EXIT=0`) |
| `cargo clippy --all-targets -D warnings` clean | Pass (`CLIPPY_EXIT=0`) |

## Blocking Issues

### BL-01: `PI070`'s widened object and verb sets make third-person `will` vendor-feature sentences fire HIGH

**File:** `patterns/core/persistence-lifecycle-hijack.yaml:436` (the `PI070` `pattern`; modal set contains `will`, widened verbs and
objects added in plan 05-03) and its `relaxed_pattern` immediately above it.

**What:** `PI070` is HIGH, the tier `install-hook` blocks commits at. Plan 05-03 widened its write verbs (`put`, `place`, `copy`,
`drop`, `schedule`, `enable`, `set up`) and persistence objects (`systemctl`, scheduled tasks, launch agents, a startup folder, the
`.claude` / `.gemini` / `.vscode` `settings.json` files, `copilot-instructions.md`, `GEMINI.md`, `MEMORY.md`, `its memory`). The
modal set still contains `will`, which is the modal vendor documentation uses to describe what a product does. Measured on the
release binary, each of these fires `PI070` HIGH at confidence 1.0 and **is silent under the pre-phase pattern** (baseline
`1d73493`, tested by running its pattern text through an equivalent regex engine):

| New false positive | Reach added by the phase |
|---|---|
| `The assistant will save the preference to its memory.` | object `its memory` |
| `Claude will save your choice to .claude/settings.json so it persists.` | object `.claude/settings.json` |
| `Claude Code will write the permission to .claude/settings.local.json when you approve a command.` | object `settings.local.json` |
| `Copilot will create copilot-instructions.md if you ask it to initialise the repository.` | object `copilot-instructions.md` |
| `The model will add the entry to MEMORY.md when you say "remember this".` | object `MEMORY.md` |
| `Claude will put the new rules in CLAUDE.md after you run /init.` | verb `put` |
| `Claude will copy your answers into AGENTS.md as the starting template.` | verb `copy` |
| `The assistant will enable scheduled tasks once you turn on the feature.` | `scheduled tasks`, verb `enable` |
| `The agent will set up a systemd timer for the nightly job if you choose that option.` | `systemd timer`, verb `set up` |
| `The assistant will register a launch agent when the app is installed.` | object `launch agent` |
| `The agent will drop a cron entry into the crontab for the scheduled backup.` | verb `drop` |
| `The assistant will create a startup folder shortcut on Windows.` | object `startup folder` |
| `The agent needs to add the service to systemctl on the build host.` | object `systemctl` |

All thirteen are synthesized in a vendor's voice; none was sourced from a real document, and the 26,407-file GATE-03 sweep has zero
`PI070`-`PI079` findings, so **no real file on the author's machine contains one.** That is why every gate was green: it is the same
absence-of-evidence failure as deferred items 6 and 9b. Item 9b's analysis concluded `PI070` "escaped" the vendor-prose class "by
requiring a modal"; `will` is a modal, and that premise was never measured. The clean corpus holds no `will` + widened-object specimen,
and `tests/pattern_test.rs:1187` (`test_pi070_...`, 27 positives) contains none of them as a negative.

**Why this is blocking:** it is the exact `PATTERNS.md` `#97` failure the phase repaired twice (`PI071`, `PI076`), on the tier that
blocks commits, introduced by this phase, and the repository's own contributor contract (rule 3) says a recallable benign sentence caps
the grade at MEDIUM.

**Resolution: DEFERRED, not resolved. Reason and issue:** [#183](https://github.com/UnityInFlow/injection-scanner/issues/183) (P1, milestone
v0.2.0). The remedy that worked for `PI071`/`PI076` (item 9b's second-person-possessive allow-list) does **not** discriminate here: the
vendor sentence and the attack both end in a bare object path (`to .claude/settings.json` vs `to ~/.ssh/authorized_keys`). Choosing
between (a) requiring a deictic cue for the widened objects, (b) dropping `will` and `may now` from the HIGH modal set, (c) grading the
widened objects MEDIUM, and (d) reverting the widening is a design decision about a shipped HIGH pattern, and it would be made after the
whole-category GATE-03 delta (`05-SWEEP.md`), after the sealed held-out set was opened (`H02`, the category's one held-out detection by a
CAT-03 pattern, is `PI070`'s) and after every number was published. An in-wave edit would also need a fresh sweep, a regenerated
catalogue and baseline, a new clean specimen (which cannot be added until it stops firing, since that corpus must stay at zero), a
`relaxed_pattern` that still fires on one of the sentences, and a `test_pi070` negative per object family. **It is therefore handed to the
maintainer as the one open decision before the PR**, and the README / CHANGELOG do not yet mention it.

### BL-01: RESOLVED — `will` removed from `PI070`'s modal set

**Decided by the maintainer on 2026-10-09**, after this review was filed and after the deferral paragraph above was written.
Option (b) was chosen, narrowed to `will` alone: `may now` stays, because "the agent may now write to ..." is a claim that a
control is off, which is attack framing rather than vendor description.

**The change.** One token removed from `PI070`'s `pattern` modal set, plus the rationale comment above it. `patterns/`
otherwise untouched; `src/`, `Cargo.toml` and `Cargo.lock` untouched.

**Measured cost, not argued cost.** The decision was taken against measurement, not the four options in the abstract:

| Question | Method | Result |
|---|---|---|
| Does the class actually fire? | 14 synthesized vendor sentences, release binary | 14/14 fired HIGH before; **0/14 after**, at default and `--strict` |
| Is the determiner the discriminator? | `the` substituted for `your` | No — fires on both, so item 9b's allow-list could not have fixed it |
| Attack-corpus detections lost | shipped vs mutated regex over every `tests/corpus/attack/**/*.md` line | **0** — both `PI070` attack hits use `should` |
| Held-out detections lost | same, over the held-out payloads | **0** — `H02`'s detection uses `should`, so held-out stays **2/12** |
| Development recall moved | `recall_test` `EXPECTED` | **no** — 103/115 unchanged |
| Test cases affected | `test_pi070_...` | exactly one positive used `will`; see below |
| Siblings with the same class | same 14 sentences against the full 79-pattern library | none — `PI071`/`PI073`/`PI075`/`PI076` all silent; the class was `PI070`-only |

**The one affected test case** was `"The agent will append the export to ~/.config/fish/config.fish."`. It was the *only* positive
covering the `config.fish` object, so the object keeps its coverage with an attack modal (`must`), and the `will` form moved to
`assert_negatives` — where it belongs, since it is the ambiguous grammar itself. Three further vendor-voice negatives were added.

**The gate is now load-bearing, not asserted.** The review's deferral paragraph noted a clean specimen "cannot be added until it
stops firing". It has stopped, so `tests/corpus/clean/persistence-vendor-release-notes.md` was added, and it was mutation-tested
the way this repo requires: the shipped set reports **0** findings on it, and restoring `will` makes it report **6**. Putting the
token back now turns the clean corpus red in six places rather than relying on anyone re-running a probe.

**Why `PI070` may stay HIGH.** `PATTERNS.md` rule 3 caps a grade at MEDIUM when a *recallable benign sentence* fires. The remedy
removed the benign sentence class instead of lowering the grade, so the rule is satisfied at HIGH rather than waived.

**Re-gated after the fix:** catalogue regenerated (one line changed — `PI070`'s regex), code-scanning baseline regenerated (pure
line-number shift from the comment, no finding added or removed), whole-repo self-scan back to exactly the two standing
`PATTERN-CATALOGUE.md` findings of deferred item 8, `cargo fmt --check` and `cargo clippy --all-targets -D warnings` exit 0, and
the full suite green. GATE-03 was not re-swept: a pure narrowing cannot add a finding, and the whole-category sweep had **zero**
`PI070`-`PI079` findings across 26,407 files, so both directions are provably unchanged.

**The cost it creates, filed rather than left silent.** A post-fix probe of seven *attack*-shaped `will` sentences found five produce
no finding at all and two fire only on unrelated grounds (`PI014`, `PI015`): third-person `will` persistence instructions are now
undetected by this category. `PI071` does not cover them, because its arms are non-modal. That is a deliberate trade — the two
populations differ only by provenance — and it is tracked as [#184](https://github.com/UnityInFlow/injection-scanner/issues/184) with
the candidate discriminators a real fix needs (a concealment cue, a persistence-justifying clause, an untrusted-document framing, or
the structural pass), each to be probed against vendor prose before the pattern is written. It is also recorded in
`docs/DETECTION-BACKLOG.md` under the category's accepted gaps.

[#183](https://github.com/UnityInFlow/injection-scanner/issues/183) is closed by this change, and the `unmet-truth` entry in
`.planning/WINDOWS.md` is resolved.

## Suggestions

### S-01: `scanner()` rebuilds and recompiles all 79 patterns for every payload

`tests/recall_test.rs:440` and `tests/recall_test.rs:562` — `detected()` calls `scanner()` per payload, so `recall_matches_the_recorded_numbers`
compiles the full library 127 times (measured about 120 s in a debug build, about 3 minutes under CPU contention). It is the test-side
analogue of the checklist's "regexes compiled once" item. Pre-existing; this phase added 12 payloads. A `OnceLock<Scanner>` would cut
it to seconds if `Scanner` is `Sync`. Not applied: out of scope for a pattern phase and it touches shared test infrastructure.

### S-02: the derivation-check tool has no tests and no caller

`tools/corpus-derivation-check.py:195` (`main`) — exits 0 on any subset and is referenced by nothing in CI (planted-lift behaviour was
verified by hand in plan 05-07 only). Filed as [#166](https://github.com/UnityInFlow/injection-scanner/issues/166).

### S-03: a repeated hook key reports the first line

`patterns/core/persistence-lifecycle-hijack.yaml:304` region (`PI077` header) — `locate()` collapses repeated keys, so a multi-hook
file reports every `PI077` finding at the first line, which makes `# injection-scanner:ignore PI077` unusable for one command.
Filed as [#165](https://github.com/UnityInFlow/injection-scanner/issues/165).

### S-04: `PI079`'s 40-character blob minimum and MEDIUM grade are choices, not measurements

`patterns/core/persistence-lifecycle-hijack.yaml` (`PI079`, the last pattern) — filed as [#180](https://github.com/UnityInFlow/injection-scanner/issues/180).

### S-05: the held-out result is not in the pattern file's header

`patterns/core/persistence-lifecycle-hijack.yaml:1` — the header documents every accepted blind spot but not the independent
measurement (2 of 12; all four hook files missed by the download-then-run shape). `docs/DETECTION-BACKLOG.md`, `README.md` and
`CHANGELOG.md` carry it, and the pattern file is frozen in v0.2.0 by the held-out rule, so it was left alone. Add it when the file is
next opened for the issues #176-#179.

## Resolution record

| Finding | Resolution | Commit | Measurement | Issue |
|---|---|---|---|---|
| BL-01 | **Deferred**, maintainer decision required | none (no pattern edited) | 16 of 17 probes fire `PI070` HIGH; 13 are new relative to the pre-phase pattern; 0 hits on 26,407 real files | [#183](https://github.com/UnityInFlow/injection-scanner/issues/183) |
| S-01 | Recorded, not applied | none | about 120 s debug-build `recall_matches_the_recorded_numbers` | — (test infrastructure) |
| S-02 | Filed | none | — | [#166](https://github.com/UnityInFlow/injection-scanner/issues/166) |
| S-03 | Filed | none | `PI077` reports both of two commands at one line | [#165](https://github.com/UnityInFlow/injection-scanner/issues/165) |
| S-04 | Filed | none | — | [#180](https://github.com/UnityInFlow/injection-scanner/issues/180) |
| S-05 | Recorded for the next edit of the file | none | — | #176-#179 |
