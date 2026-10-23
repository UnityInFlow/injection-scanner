---
phase: 04-mcp-tool-description-poisoning-cat-02-34
plan: 04
subsystem: patterns
tags: [cat-02, mcp, structural-patterns, config-hygiene, gate02, gate03, gate05, measured-narrowing]

requires:
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 01
    provides: the pre-edit GATE-03 baseline and the PI050+ relaxed_pattern policy widened to an open-ended id predicate
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 02
    provides: the twelve CAT-02 threat-model attack payloads and the measured pre-pattern recall baseline
  - phase: 04-mcp-tool-description-poisoning-cat-02-34
    plan: 03
    provides: the flat mcp-* clean-corpus false-positive gate, including the D-03 config-hygiene boundary manifest these three patterns are judged against
provides:
  - the mcp_tool_poisoning category (PI060-PI069), registered in load_embedded_patterns and guarded by a category-is-loaded test that was mutation-proven to fail when the registration is removed
  - D-03's three config-hygiene signals -- PI060 unvetted-mcp-server-source, PI061 plaintext-mcp-endpoint, PI062 remote-script-mcp-launch -- all scope frontmatter, all leaf-anchored, all MEDIUM by category default
  - a measured adjudication of D-03's "unpinned npx -y" wording against real manifests, and the recorded accepted cost of the branch taken
  - examples/mcp-tool-poisoning-attack.md, the PATTERNS.md and README Categories rows, a README behaviour-change callout, the re-pinned recall numbers, and the regenerated catalogue and code-scanning baseline
  - a two-directional GATE-03 delta over 23,764 real third-party files with every entry adjudicated, plus a new mid-phase sweep reference point (sweep-mainbase-04-04-2026-09-06) that later plans should compare against instead of the now-two-generations-old 04-01 baseline
affects: [04-05, 04-06, 04-07]

actuals:
  tokens: 92000
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - "A locked decision's WORDING adjudicated against a fresh measurement before the regex is written: D-03 says 'unpinned npx -y', the measurement says that is the ecosystem default, so the discriminator becomes the install SOURCE and the resulting blind spot is named in the YAML header, the README callout, the recall comment and this summary rather than discovered in review"
    - "Loopback excluded without a lookahead (the `regex` crate has none) by requiring the host to be a registrable domain -- at least one dot and a final label of letters -- which rejects localhost, 127.0.0.1, 0.0.0.0 and [::1] structurally"
    - "A three-way sweep comparison when `main` has moved under a phase: baseline / merge-base / candidate, so a merged PR's delta is not attributed to the plan under test"

key-files:
  created:
    - patterns/core/mcp-tool-poisoning.yaml
    - examples/mcp-tool-poisoning-attack.md
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-after-04-04-2026-09-06/
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/sweep-mainbase-04-04-2026-09-06/
  modified:
    - src/patterns/mod.rs
    - tests/pattern_test.rs
    - tests/recall_test.rs
    - examples/README.md
    - PATTERNS.md
    - README.md
    - docs/PATTERN-CATALOGUE.md
    - .github/code-scanning-baseline.json
    - .planning/phases/04-mcp-tool-description-poisoning-cat-02-34/04-SWEEP.md

key-decisions:
  - "PI060 discriminates on the install SOURCE, not on the absence of a version pin. Measured first: a scratch probe outside this repository, over the 46 real .mcp.json / mcp.json / claude_desktop_config.json files inside the 04-SWEEP.md directory list, found the plain unpinned package-runner shape on 8 distinct real manifests -- 8 of the 24 that declare any launch command at all -- against 1 manifest using an off-registry source. The COMMON branch of the plan's Task 1 was therefore taken."
  - "The accepted cost of that branch is that tests/corpus/attack/structural/mcp-tool-poisoning/07-unpinned-npx-install-mcpservers.md stays an undetected payload on purpose. It is named in the YAML header, in the README behaviour-change callout, in the recall_test EXPECTED comment and here."
  - "Category default severity is MEDIUM, set once on the category rather than three times per pattern, so D-03's below-the-commit-blocking-line requirement holds by construction. No shipped pattern in this file carries a severity field at all."
  - "PI061 excludes loopback without a lookahead, since the `regex` crate has none, by requiring a registrable-domain host. The same construction also declines single-label LAN hostnames (http://build-box:8080), which is an accepted and documented cost -- those are the same local-development shape as loopback."
  - "PI062 reuses PI028 pipe-to-shell's fetch-and-execute alternation character for character rather than authoring a second vocabulary (T-04-21). The two are the same signal read by different passes; only the leaf anchoring and opencode's command-as-array tolerance differ."
  - "Example and counter_example blocks in the YAML use the wrapper-less and `servers` shapes rather than `mcpServers`, so the plan's filtered grep -- no wrapper key anywhere outside a comment -- holds literally as well as in intent, and the examples themselves demonstrate wrapper independence."
  - "A THIRD sweep run was added, from a release binary built at 0d50e92 (the merge-base this branch started from). Comparing straight to the 04-01 baseline attributes PR #110's 66 removals to this plan; the merge-base pair isolates 04-04 at +1 finding and 0 removals."

requirements-completed: [CAT-02, GATE-02, GATE-03, GATE-04, GATE-05]

coverage:
  - id: D1
    description: "The mcp_tool_poisoning category exists, is registered in load_embedded_patterns, and the registration is proven load-bearing by a performed mutation: removing the loader entry makes test_mcp_tool_poisoning_category_is_loaded FAIL with 'mcp_tool_poisoning category must be registered in load_embedded_patterns()' at tests/pattern_test.rs:1635, alongside test_pi060_unvetted_mcp_server_source and test_total_pattern_count (3 failed, 65 passed). Restored immediately afterwards."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_test with the loader entry removed (3 failed) and restored (70 passed)"
        status: pass
    human_judgment: false
  - id: D2
    description: "No PI060+ regex requires a manifest wrapper key as a prefix. `grep -v '^ *#' patterns/core/mcp-tool-poisoning.yaml | grep -c 'mcpServers'` prints 0, and each pattern's positive cases span all three real wrapper conventions plus the opencode command-as-array value shape, so a wrapper-anchored regex cannot pass the unit tests."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "shell grep (prints 0); cargo test --test pattern_test test_pi060_unvetted_mcp_server_source (4 positives across mcpServers / no-wrapper / servers / mcp+array), test_pi061_plaintext_mcp_endpoint (3 wrapper shapes), test_pi062_remote_script_mcp_launch (4 shapes)"
        status: pass
    human_judgment: false
  - id: D3
    description: "The three config-hygiene signals ship in their own MEDIUM band by construction: `grep -c '^    severity:' patterns/core/mcp-tool-poisoning.yaml` prints 0, so all three inherit `default_severity: MEDIUM` and none can be individually forgotten. MEDIUM is below HIGH, the severity install-hook blocks commits at."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "shell grep (prints 0); docs/PATTERN-CATALOGUE.md renders all three as MEDIUM (category default)"
        status: pass
    human_judgment: false
  - id: D4
    description: "Whether the plain unpinned package-runner install is a viable signal was decided by measuring real manifests, not by reading the decision that proposed it. Scratch probe outside the repository, three probe arms, 46 real manifests inside the 04-SWEEP.md directory list: 8 fired on the package-runner launch shape, 6 on a bare -y argument, 1 on an off-registry source; 24 declare a command key at all, 1 is unparseable JSONC (Q1's known gap). The sweep independently reproduces the off-registry count as 1 finding in 23,764 files."
    requirement: "CAT-02"
    verification:
      - kind: integration
        ref: "release binary + scratch probe pattern directory outside the repository, over an enumerated list of 46 real manifests; counts recorded in the YAML header and below"
        status: pass
    human_judgment: false
  - id: D5
    description: "Each of the three patterns carries a counter_example that is the near-miss its narrowing excludes and a relaxed_pattern that drops exactly that narrowing, mutation-tested in CI: pattern_relaxed_control_test proves the shipped form misses each counter_example while the relaxed form catches it. pattern_policy_test's open-ended id >= 50 predicate covers them automatically; no id was added to the legacy exemption list."
    requirement: "GATE-05"
    verification:
      - kind: unit
        ref: "cargo test --test pattern_relaxed_control_test, --test pattern_policy_test, --test pattern_example_test — all pass"
        status: pass
    human_judgment: false
  - id: D6
    description: "The PI060-PI069 row is in PATTERNS.md's Categories table in the SAME commit as the first pattern (4b0c1c7), which is WR-01's exact miss from Phase 3, not repeated. The README Pattern Categories row, the pattern-count sentence and a behaviour-change callout landed in the same commit."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "git show --stat 4b0c1c7 lists PATTERNS.md and README.md; grep -c 'PI060' PATTERNS.md = 1, README.md = 2"
        status: pass
    human_judgment: false
  - id: D7
    description: "Recall re-pinned exactly (GATE-02) in the same commit as the pattern change, in both tests/recall_test.rs EXPECTED and the README table: mcp-tool-poisoning-structural 4/8 -> 5/8, category row 6/12 -> 7/12, total 99/109 -> 100/109. Only payload 08 moves, caught by PI061; payload 09 was already detected by PI028 and is now also reached structurally by PI062, which changes no count; payload 07 stays a deliberate miss."
    requirement: "GATE-02"
    verification:
      - kind: unit
        ref: "cargo test --test recall_test (8 passed) after re-pinning; the failure output before re-pinning named 'mcp-tool-poisoning-structural: detected 5/8, EXPECTED 4 (improvement — update the number and the README)'"
        status: pass
    human_judgment: false
  - id: D8
    description: "The example file makes every new pattern reachable by the attack-corpus ratchet, and the unexercised list did not grow. Scanning examples/mcp-tool-poisoning-attack.md at a zero confidence threshold reports PI028, PI060, PI061 and PI062."
    requirement: "CAT-02"
    verification:
      - kind: integration
        ref: "./target/release/injection-scanner check examples/mcp-tool-poisoning-attack.md --format json --min-confidence 0.0 -> ['PI028','PI060','PI061','PI062']; cargo test --test corpus_test passes including no_new_pattern_escapes_the_attack_corpus"
        status: pass
    human_judgment: false
  - id: D9
    description: "Every clean-corpus specimen still reports zero matches at a zero confidence threshold AND under --strict, including plan 04-03's D-03 boundary manifest mcp-dev-tooling-setup.json, which is built from exactly the npx -y shape PI060 deliberately does not detect."
    requirement: "GATE-05"
    verification:
      - kind: integration
        ref: "per-file scan of every tests/corpus/clean/* at --min-confidence 0.0 --strict: zero matches on all of them; cargo test --test corpus_test passes"
        status: pass
    human_judgment: false
  - id: D10
    description: "The GATE-03 delta was measured in BOTH directions over 23,764 real third-party files across the same 32-directory list, and every entry is individually adjudicated. Additions: 1 (serena/.mcp.json:4 PI060, a true positive on a real off-registry git install). Removals: 0. The 66 removals a straight comparison to the 04-01 baseline reports are all PI017 (27, retired into MatchContext::HiddenHtml) and PI026 (39, made badge-safe) from the PR #110 merge on main, proven not to belong to this plan by the merge-base comparison."
    requirement: "GATE-03"
    verification:
      - kind: integration
        ref: "scripts/gate03-sweep.sh --compare run in both directions against sweep-mainbase-04-04-2026-09-06 (1 addition / 0 removals) and against sweep-baseline-2026-09-03 (1 addition / 66 removals, all attributed); full record in 04-SWEEP.md"
        status: pass
    human_judgment: false
  - id: D11
    description: "The whole-repo self-scan set outside examples/, patterns/, tests/ and tools/ is unchanged by this plan. The set is identical file-and-pattern to the same scan run on the unmodified tree (verified by stashing all work and re-running): docs/DETECTION-BACKLOG.md (PI011, PI014, PI019, PI027, PI028, PI029, PI039, PI045, PI054, PI055) and docs/PATTERN-CATALOGUE.md (PI001, PI031). The DETECTION-BACKLOG entries are NOT in 04-SWEEP.md's baseline set and are NOT this plan's -- they arrived with the PR #110 merge; see Deviations."
    requirement: "GATE-03"
    verification:
      - kind: integration
        ref: "the pattern-library skill's self-scan pipeline, run on this branch and on the same tree with all work stashed; identical (file, pattern_id) sets"
        status: pass
    human_judgment: false
  - id: D12
    description: "No dependency was added or changed: git diff --stat Cargo.toml Cargo.lock is empty at every commit in this plan (T-04-SC)."
    requirement: "CAT-02"
    verification:
      - kind: unit
        ref: "git diff --stat Cargo.toml Cargo.lock (empty)"
        status: pass
    human_judgment: false

duration: unrecorded_precise_start
completed: 2026-09-06
status: complete
---

# Phase 4 Plan 04: CAT-02's structural half Summary

**Shipped D-03's three config-hygiene signals — `PI060` unvetted-mcp-server-source, `PI061` plaintext-mcp-endpoint, `PI062` remote-script-mcp-launch — in their own MEDIUM band, with the whole category scaffolding landing beside them, and with the one open question in the plan settled by measuring 46 real manifests rather than by re-reading the decision that raised it.**

## Performance

- **Duration:** commit span across three sequential commits; precise start timestamp not captured
- **Tasks:** 3/3 completed
- **Files modified:** 13 (2 source/pattern files created, 2 sweep-record directories created, 9 modified)
- **Test count:** 373 passing (up from 357 at the 04-03 baseline; +16 across three new pattern tests, a category-is-loaded guard, and the merged PR #110's own tests)

## Task Commits

| Task | Commit | Subject |
|---|---|---|
| 1 | `4b0c1c7` | `feat(04-04): open CAT-02 with PI060 unvetted-mcp-server-source (#34)` |
| 2 | `90271e0` | `feat(04-04): complete the CAT-02 config-hygiene band with PI061 and PI062 (#34)` |
| 3 | `caa67ce` | `docs(04-04): two-directional GATE-03 delta for the CAT-02 structural half (#34)` |

## The measurement, and which branch was taken

This is the plan's `estimate.confidence: low` item and its Task 3 instruction: *where a locked
decision meets a measured obstacle, measure first.*

D-03 puts "unpinned `npx -y <pkg>` servers" in scope. `04-RESEARCH.md` §Q2 had already recorded
that shape as the ecosystem default rather than an outlier, and the plan required that reading to
be re-measured rather than trusted. A scratch pattern directory **outside this repository** — three
probe arms, never committed, never added to `patterns/` — was run with the release binary over the
46 real `.mcp.json` / `mcp.json` / `claude_desktop_config.json` files inside the `04-SWEEP.md`
directory list.

| Probe arm | Fired on |
|---|---:|
| `PRB001` package-runner launch (`command = npx\|uvx\|bunx\|pnpx`) | **8 of 46 manifests** |
| `PRB002` bare unpinned auto-confirm flag (`args[N] = -y`) | **6 of 46 manifests** |
| `PRB003` off-registry install source | **1 of 46 manifests** |
| — either PRB001 or PRB002 (the plain unpinned shape) | **8 of 46 manifests** |

Denominators worth stating: of the 46, **24 declare a launch `command` at all** and 21 declare a
`url`; 1 (`~/.config/github-copilot/intellij/mcp.json`) is JSONC and unparseable, which is Q1's
already-recorded gap, not a new finding. So the plain unpinned package-runner shape is **8 of the
24 manifests that could carry it — a third of them.**

**The plan's COMMON branch was taken.** Three independent lines of evidence make it decisive
rather than marginal:

1. 8 real manifests is not an outlier count; the shape is how Context7, playwright, the official
   filesystem and postgres servers and others are all installed.
2. This repository's own false-positive gate already contains the shape.
   `tests/corpus/clean/mcp-dev-tooling-setup.json`, committed by plan 04-03 as the *D-03 boundary
   specimen*, is built from `npx -y @example/docs-search-mcp` and `npx -y @example/changelog-mcp@latest`.
   A pin-based pattern fails `corpus_test` on day one, before any argument about severity.
3. The single off-registry manifest found by the probe is independently reproduced by the GATE-03
   sweep as exactly one finding in 23,764 files.

So `PI060` is keyed on the **install source** — a git reference, a repository shorthand, or an
archive URL — which is the "unknown hosts / unvetted source" half of D-03 expressed in a form a
manifest can actually be judged on.

### The accepted cost, named here rather than found in review

`tests/corpus/attack/structural/mcp-tool-poisoning/07-unpinned-npx-install-mcpservers.md`
(`npx -y quick-utils-mcp@latest`) **stays an undetected payload on purpose.** It is textually
indistinguishable from the clean-corpus specimen sitting two directories away, and no regex
separates them. This is recorded in four places so it cannot be quietly lost: the YAML header, the
README behaviour-change callout, the `recall_test` `EXPECTED` comment beside the pinned number, and
here. It is the same discipline CR-01 forced into the open in Phase 3.

## What each pattern keys on

| Id | Signal | Narrowing under GATE-05 test | Counter-example |
|---|---|---|---|
| `PI060` | `args`/`command` leaf whose value is an off-registry code source (`git+<scheme>://`, `ssh://`, `git://`, `git@host:path`, `github:`/`gitlab:`/`bitbucket:` shorthand, or an HTTP(S) `.git`/`.tgz`/`.tar.gz`/`.zip` archive) | the off-registry-source requirement | a plain registry package handed to the same package runner |
| `PI061` | endpoint leaf carrying a non-TLS URL whose host is a registrable domain | the loopback exclusion | `http://localhost:3000/mcp` |
| `PI062` | `args`/`command` leaf whose value fetches remote content and pipes it to an interpreter | the requirement that fetched content be *executed*, not merely downloaded | the same `curl` redirected to a file with `-o` |

Two construction notes worth not rediscovering:

- **The structural pass renders and matches ONE projected line at a time** (`src/scanner.rs`, fourth
  pass). No structural regex can require two leaves to co-occur, which is why "unpinned `npx -y`"
  cannot be expressed as one pattern at all — `command = npx` and `args[0] = -y` are separate lines.
  That alone would have forced a single-leaf discriminator even without the false-positive argument.
- **The `regex` crate has no lookahead.** `PI061` excludes loopback structurally instead, by
  requiring the host to be a registrable domain: at least one dot and a final label of letters.
  `localhost` (no dot), `127.0.0.1` and `0.0.0.0` (numeric final label) and `[::1]` (not a hostname
  character) all fail that requirement. Single-label LAN names like `http://build-box:8080` are
  declined by the same construction — an accepted, documented cost, since those are the same
  local-development shape as loopback.

## The loader-removal mutation (T-04-16)

Performed and restored. With `MCP_TOOL_POISONING_YAML` removed from `load_embedded_patterns`'s
array, `cargo test --test pattern_test` reports **3 failed, 65 passed**, the first being:

```
thread 'test_mcp_tool_poisoning_category_is_loaded' panicked at tests/pattern_test.rs:1635:5:
mcp_tool_poisoning category must be registered in load_embedded_patterns()
```

Restored immediately; 70 passed afterwards. (One incident worth recording: restoring the file via
`mv file.bak file` preserved the *older* mtime, so cargo reused a stale test artifact and
`catalogue_test` failed against a phantom 61-pattern set. `touch src/patterns/mod.rs` resolved it.
A stale-artifact failure after a mutation experiment looks exactly like a real regression.)

## Recall, re-pinned exactly (GATE-02)

| Row | Before | After | Cause |
|---|---:|---:|---|
| `mcp-tool-poisoning-structural` | 4/8 | **5/8** | `PI061` catches payload 08 |
| MCP & Tool-Description Poisoning (README) | 6/12 | **7/12** | same |
| **Total** | 99/109 | **100/109 (91.7%)** | same |

Payload 09 (`curl … | sh`) was already detected by `PI028` over the raw JSON text and is now *also*
reached structurally by `PI062` — a second route to an already-counted payload, which changes no
number. Payload 07 stays missed on purpose. Payloads 05 and 06 (both rug-pull markers) are a later
plan's work.

## GATE-03: two directions, one entry, zero removals

Full record in `04-SWEEP.md`. Headline: over the same 32-directory list, **23,764 files**, the
comparison that isolates this plan (merge-base `0d50e92` → this branch) reports **1 addition and
0 removals**.

The single addition —
`~/.claude/plugins/marketplaces/…/serena/.mcp.json:4 PI060` — was opened and read. It is a real,
wrapper-less manifest launching `uvx --from git+https://github.com/oraios/serena`, adjudicated a
**true positive**: the signal is a supply-chain property of the installation, not a claim about the
author, which is exactly why the band sits at MEDIUM.

`PI061` and `PI062` add **zero** findings on real files. That is the measurement, not a defect —
§Q2 found every real endpoint on TLS, and no real manifest here pipes a download into a shell. Both
are covered by unit tests, their example/counter-example pairing, their `relaxed_pattern` mutation
control and the attack-corpus ratchet.

## Decisions Made

See `key-decisions` in the frontmatter. The two most consequential: (1) the install-source
discriminator and its named blind spot, chosen from a measurement; (2) MEDIUM set once on the
category rather than three times per pattern, so no shipped pattern in this file carries a
`severity` field at all and D-03's below-the-commit-blocking-line requirement cannot be forgotten
one pattern at a time.

## Deviations from Plan

Four, all recorded rather than silent.

1. **Pattern counts.** The plan said the total-count test goes to 57 after Task 1 and 59 after
   Task 2. Those numbers were written before `main` moved: commit `0d50e92` merged PR #110, adding
   `PI058`, `PI070` and `PI110`–`PI113`, so the branch starts at **61**, not 56. The counts shipped
   are **62** and **64**. The plan's intent — the count test moves by exactly the number of
   patterns added, with its running commentary extended — is honoured.

2. **A third sweep run was added, and the plan's `<verify>` comparison pair was not the one used
   for the verdict.** The plan's automated verify compares the 04-01 baseline (`b4f05ef`) directly
   to this plan's run. Because PR #110 landed in between — retiring `PI017` into
   `MatchContext::HiddenHtml` and making `PI026` badge-safe — that comparison reports **66
   removals that belong to PR #110, not to plan 04-04**, and would have been adjudicated as this
   plan's had it been taken at face value. A release binary was therefore built from `0d50e92`
   itself and swept over the identical list, giving the merge-base pair that actually isolates this
   plan. Both pairs are recorded in both directions. The new run is committed as
   `sweep-mainbase-04-04-2026-09-06/` and **plans 04-05 to 04-07 should compare against it**, not
   against the now-two-generations-old 04-01 baseline.

3. **The self-scan set does not match `04-SWEEP.md`'s baseline**, and the plan's acceptance
   criterion asked that it match exactly. It does not, and the difference is not this plan's: the
   baseline recorded exactly two standing self-matches in `docs/PATTERN-CATALOGUE.md`, whereas the
   current tree also reports ten in `docs/DETECTION-BACKLOG.md`. This was verified by stashing
   **all** of this plan's work and re-running the scan: the set is identical file-for-file and
   pattern-for-pattern with and without this plan's changes, so it arrived with the PR #110 merge.
   The only movement attributable here is the two catalogue self-matches shifting line number
   (76→77, 877→878) because the catalogue grew. **A pre-existing finding worth someone's attention:
   `docs/DETECTION-BACKLOG.md` is now flagging ten times against the repo's own documentation,
   which is the exact failure mode the pattern-library skill warns about and the 2026-08 audit
   listed. It predates this branch; it is named here rather than fixed, because fixing it inside a
   pattern PR would confound this plan's GATE-03 delta.**

4. **Output directory date.** The plan named `sweep-after-04-04-2026-09-03`; the run happened on
   2026-09-06 and the directory is named for the day it ran.

One further judgment call inside the plan's own discretion: the YAML's `example` and
`counter_example` blocks were rewritten to use the wrapper-less and `servers` shapes so that the
plan's literal acceptance grep (`grep -v '^ *#' … | grep -c 'mcpServers'` prints `0`) holds against
the whole file rather than only against the regexes. A second acceptance grep,
`grep -c 'severity:'` printing `0`, cannot hold literally — the string appears in
`default_severity: MEDIUM`, which is the very field the criterion requires to exist. It was
evaluated as `grep -c '^    severity:'` (per-pattern overrides only), which prints `0`.

## Issues Encountered

- **A stale cargo artifact after the loader-removal mutation** (see above) produced a
  `catalogue_test` failure describing a pattern set that no longer existed. Worth recognising: a
  mutation experiment that restores a file with an older mtime can leave cargo convinced nothing
  changed.
- **The 04-01 baseline's raw JSON reports are gitignored** (`RAW-REPORTS.md` records why), so the
  `--compare` runs read from `.planning/local/` in the main checkout rather than from the worktree.
  This is by design and worked, but a plan whose `<verify>` block names the phase-directory path
  will not run as literally written.

## User Setup Required

None.

## Next Phase Readiness

- **Plans 04-05/04-06 must not compare against `sweep-baseline-2026-09-03`.** Use
  `sweep-mainbase-04-04-2026-09-06/` (or a fresh pre-edit capture) or they will re-inherit PR
  #110's 66 removals as their own delta. This is the single most important thing to carry forward.
- The category file, its registration, its guard test, the docs rows and the example file all
  exist now. Plan 04-05's description-poisoning arms add patterns to an existing file and must
  carry an explicit `severity: HIGH` override, since the category default is MEDIUM — the inverse
  of the CAT-01 file's arrangement.
- Plan 04-03's warning holds and is now sharper: `mcp-server-catalogue.json`'s
  `resolve_reference_id` description is the specimen the D-01 prose arms will fail on first if they
  narrow only on second-person-plus-imperative rather than on the *object* of the instruction.
- `docs/DETECTION-BACKLOG.md`'s ten self-matches (deviation 3) deserve their own issue or quick
  task. They are a pre-existing regression on `main`, not this plan's, and they make "the self-scan
  set is unchanged" a harder criterion to state cleanly for every later plan in this phase.
- The three arms here add **zero** real-world findings for `PI061` and `PI062`. If a later plan
  wants real-world evidence for the transport arm specifically, the corpus does not contain any and
  synthesising it is the only option — §Q2 already established that.

---
*Phase: 04-mcp-tool-description-poisoning-cat-02-34*
*Completed: 2026-09-06*

## Self-Check: PASSED

Both claimed created files exist on disk (`patterns/core/mcp-tool-poisoning.yaml`,
`examples/mcp-tool-poisoning-attack.md`), both claimed sweep directories exist with
`manifest.tsv`/`summary.tsv`/`checksums.sha256`/`RAW-REPORTS.md`, all three task commit hashes
(`4b0c1c7`, `90271e0`, `caa67ce`) are present in `git log`, and the full gate is green:
`cargo test --locked` 373 passed / 0 failed, `cargo fmt --all -- --check` clean,
`cargo clippy --all-targets --locked -- -D warnings` clean.
