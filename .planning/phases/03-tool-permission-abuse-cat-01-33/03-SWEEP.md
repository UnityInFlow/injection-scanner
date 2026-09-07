# GATE-03 Pre-Pattern Sweep — Phase 3 (CAT-01, #33)

**Recorded:** 2026-09-01
**Git SHA the binary was built from:** `0c3c956077ace8257f011377deb43b73b825e799`
**Pattern count at time of sweep:** 48 (`cargo run --release -- rules --format json` returns 48
entries). `grep -rn 'scope:' patterns/core/*.yaml` returns nothing — **verified this baseline
predates any structural (`scope: frontmatter`) pattern**, and therefore predates any PI05x pattern.

## Why this file exists, stated plainly

GATE-03's historical ~1,300-file sweep (PR #103/#102) was a manual, machine-local run against
`~/.claude/plugins/cache`, "the GSD workflow reference set," and seven sibling repositories. It was
never scripted, never vendored into this repository, and no fixed corpus is pinned anywhere —
`grep -rn "1,300"` finds only prose. **The corpus swept below is machine-local by construction and
is not a reproducible absolute.** Re-running `scripts/gate03-sweep.sh` on a different machine, or on
this machine after directories are added or removed, will produce different file counts. That is
expected, not a bug in the script.

The number that matters is not "N files, M findings" in isolation — it is the **delta** that
`scripts/gate03-sweep.sh --compare` computes between this baseline (pre-CAT-01-pattern) and the
after-sweep Plan 07 records (post-CAT-01-pattern), run over the SAME directory list, on the SAME
machine. This file is the "before" half of that comparison, and it is the honest form GATE-03 takes
in this repository: a committed procedure plus a named, counted baseline, not a memorized number.

## Candidates considered

Every candidate the historical sweep's directory families map to on this machine was considered.
All were present.

| Candidate | Family | Present? |
|---|---|---|
| This repository itself (worktree root) | — | Yes |
| `~/.claude/plugins/cache` | local agent-tooling cache | Yes |
| `~/.claude/skills` | local agent-tooling cache | Yes |
| `~/.claude/gsd-core` | GSD workflow reference set | Yes |
| 19 sibling UnityInFlow tool repositories (`01-spec-linter` … `20-mcp-test`, excluding this repo) | sibling UnityInFlow repositories | Yes (all 19) |

Three-directory minimum from the task precondition is exceeded — 22 directories were swept in
total, comfortably more than the seven sibling repositories the historical sweep used.

## Reproduction command

```bash
cargo build --release
bash scripts/gate03-sweep.sh /tmp/gate03-baseline \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/03-injection-scanner/.claude/worktrees/agent-ade5856f6fc048a75 \
  /Users/jirihermann/.claude/plugins/cache \
  /Users/jirihermann/.claude/skills \
  /Users/jirihermann/.claude/gsd-core \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/01-spec-linter \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/02-ai-changelog \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/04-spec-ci-plugin \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/05-budget-breaker \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/06-token-dashboard \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/07-mcp-hub \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/08-kore-runtime \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/09-agent-tracer \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/10-agent-memory \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/11-context-manager \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/12-agent-replayer \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/13-eu-ai-act-toolkit \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/14-agent-sandbox \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/15-llm-diff \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/16-prompt-vc \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/17-skills-registry \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/18-agent-bench \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/19-skill-composer \
  /Users/jirihermann/Documents/workspace-1-ideas/unity-in-flow-ai/20-mcp-test
```

(Directory names and counts below are specific to this machine on 2026-09-01. Plan 07 re-running
this exact command line on the same machine is what makes the `--compare` delta meaningful; a
different machine will see different candidates and different counts, which is why every directory
and its count is named explicitly rather than left implicit.)

## Directory list swept, with per-directory file counts

`manifest.tsv` from `/tmp/gate03-baseline`, reproduced verbatim:

| Directory | Files scanned | Findings | Status |
|---|---:|---:|---|
| `03-injection-scanner` (this repo, worktree root) | 143 | 483 | swept |
| `~/.claude/plugins/cache` | 923 | 10 | swept |
| `~/.claude/skills` | 71 | 0 | swept |
| `~/.claude/gsd-core` | 555 | 14 | swept |
| `01-spec-linter` | 48 | 0 | swept |
| `02-ai-changelog` | 50 | 0 | swept |
| `04-spec-ci-plugin` | 43 | 0 | swept |
| `05-budget-breaker` | 59 | 0 | swept |
| `06-token-dashboard` | 76 | 0 | swept |
| `07-mcp-hub` | 403 | 1 | swept |
| `08-kore-runtime` | 204 | 0 | swept |
| `09-agent-tracer` | 106 | 0 | swept |
| `10-agent-memory` | 49 | 0 | swept |
| `11-context-manager` | 6 | 0 | swept |
| `12-agent-replayer` | 182 | 0 | swept |
| `13-eu-ai-act-toolkit` | 102 | 3 | swept |
| `14-agent-sandbox` | 60 | 0 | swept |
| `15-llm-diff` | 63 | 0 | swept |
| `16-prompt-vc` | 48 | 0 | swept |
| `17-skills-registry` | 172 | 6 | swept |
| `18-agent-bench` | 7 | 0 | swept |
| `19-skill-composer` | 58 | 0 | swept |
| `20-mcp-test` | 71 | 1 | swept |
| **Total** | **3499** | **518** | |

**Total files swept: 3,499.** No directory was skipped — all 22 candidates were present and
readable on this machine.

Findings in this repository's own 483-count are dominated by the corpus itself (`tests/corpus/attack/`
is designed to trigger the scanner, `examples/` and `patterns/` carry example/counter_example text
by design). The pre-existing findings *outside* those directories are called out separately below,
per the task's instruction not to let them be confused with Plan 07's delta.

Findings in the third-party directories (`~/.claude/plugins/cache`: 10, `~/.claude/gsd-core`: 14,
`07-mcp-hub`: 1, `13-eu-ai-act-toolkit`: 3, `17-skills-registry`: 6, `20-mcp-test`: 1) are exactly
the kind of real-world, un-curated bytes GATE-03 exists to run the release binary against — this is
where the ENG-02 panic surfaced, not in the corpus. This baseline does not classify each of these as
true/false positive (that classification is Plan 07's after-sweep comparison against the CAT-01
patterns); it records what the pre-pattern binary found so the delta is interpretable.

## Findings by pattern id and severity

`summary.tsv` from `/tmp/gate03-baseline`, reproduced verbatim, across all 22 swept directories:

| Pattern | Severity | Count |
|---|---|---:|
| PI001 | CRITICAL | 123 |
| PI002 | CRITICAL | 6 |
| PI003 | MEDIUM | 13 |
| PI004 | CRITICAL | 7 |
| PI005 | CRITICAL | 9 |
| PI006 | HIGH | 12 |
| PI007 | CRITICAL | 12 |
| PI008 | HIGH | 5 |
| PI009 | HIGH | 9 |
| PI010 | MEDIUM | 8 |
| PI011 | CRITICAL | 4 |
| PI012 | HIGH | 6 |
| PI013 | HIGH | 4 |
| PI014 | MEDIUM | 13 |
| PI015 | HIGH | 21 |
| PI016 | HIGH | 5 |
| PI017 | HIGH | 18 |
| PI018 | HIGH | 16 |
| PI019 | HIGH | 5 |
| PI020 | CRITICAL | 10 |
| PI021 | CRITICAL | 23 |
| PI022 | CRITICAL | 10 |
| PI023 | CRITICAL | 12 |
| PI024 | CRITICAL | 6 |
| PI025 | MEDIUM | 2 |
| PI026 | HIGH | 6 |
| PI027 | HIGH | 6 |
| PI028 | CRITICAL | 5 |
| PI029 | HIGH | 11 |
| PI030 | HIGH | 7 |
| PI031 | HIGH | 7 |
| PI032 | HIGH | 12 |
| PI033 | HIGH | 7 |
| PI034 | MEDIUM | 9 |
| PI035 | MEDIUM | 5 |
| PI036 | HIGH | 7 |
| PI037 | HIGH | 2 |
| PI038 | MEDIUM | 5 |
| PI039 | HIGH | 20 |
| PI040 | MEDIUM | 3 |
| PI041 | LOW | 10 |
| PI042 | CRITICAL | 1 |
| PI043 | CRITICAL | 5 |
| PI044 | HIGH | 3 |
| PI045 | MEDIUM | 8 |
| PI046 | MEDIUM | 9 |
| PI047 | MEDIUM | 6 |
| PI049 | MEDIUM | 5 |
| **Total** | | **518** |

No `PI050`–`PI059` rows exist, as expected — this is the pre-pattern baseline.

## Pre-existing findings inside this repository, outside `examples/`, `patterns/`, `tests/`, `tools/`

Sweeping this repository with `--all-files` (the flag every `gate03-sweep.sh` invocation passes, so
that the release binary's own scan is exercised the same way third-party directories are) reaches
`.rs` source files, which the pattern-library skill's narrower default one-liner
(`check . --format json`, no `--all-files`) does not — `.rs` is not in `DEFAULT_EXTENSIONS`
(`src/walk.rs`). Twelve findings result, all illustrative payload text inside doc comments, code
comments or unit-test literals rather than a real attack, and none of them newly introduced by this
plan:

| File | Line | Pattern | Note |
|---|---:|---|---|
| `docs/PATTERN-CATALOGUE.md` | 73 | PI001 | **Already known** — cited in `03-CONTEXT.md`'s canonical references as predating this milestone, identical under v0.1.0 |
| `docs/PATTERN-CATALOGUE.md` | 902 | PI031 | **Already known** — same source, same status |
| `benches/scan.rs` | 91 | PI001 | Benchmark fixture: `"Ignore all previous instructions. ".repeat(5_000)`, used to measure the per-pattern per-line match cap. `benches/` is not in the skill's exclusion list, so this is newly visible under `--all-files` rather than newly introduced |
| `src/baseline.rs` | 298 | PI001 | `#[cfg(test)] mod tests` — fingerprint-determinism unit test literal (`"ignore all previous instructions"`) |
| `src/baseline.rs` | 299 | PI001 | Same test, second literal |
| `src/baseline.rs` | 300 | PI001 | Same test, third literal (one-character variant) |
| `src/normalize.rs` | 7 | PI001 | Module doc comment (`//!`) illustrating the five obfuscation spellings ENG-02's normalization pass defeats, inside a ` ```text ` fence that is not fence-aware for `.rs` files |
| `src/normalize.rs` | 9 | PI045 | Same doc comment, the Cyrillic-homoglyph line |
| `src/normalize.rs` | 9 | PI001 | Same doc comment, same line, second pattern arm |
| `src/normalize.rs` | 10 | PI001 | Same doc comment, the fullwidth-character line |
| `src/scanner.rs` | 390 | PI001 | Code comment quoting the literal string `"ignore all previous instructions"` to explain why the original (not normalized) text is quoted in a finding |
| `src/scanner.rs` | 392 | PI001 | Same comment, continuation |

None of these are new — they are artifacts of `.rs` files never having been included in a whole-repo
self-scan before (the pattern-library skill's one-liner only reaches markdown/config extensions).
Recording them here means Plan 07's after-sweep `--compare` will not misattribute them to a new
CAT-01 pattern; they exist identically before any PI05x pattern is written. Fixing or suppressing
them (adding `injection-scanner:ignore` directives, excluding `benches/` from the skill's convention,
etc.) is out of scope for this plan — GATE-04 forbids a category-widening PR from also carrying
unrelated pattern-library or corpus edits.

## After-sweep (Plan 07)

Run 2026-09-02, on the same machine, over the same 22-directory list, with `PI050`–`PI057` landed
and the release binary rebuilt.

### Totals

| | Files | Findings |
|---|---:|---:|
| Baseline (pre-pattern, 2026-09-01) | 3,356 | 518 |
| After-sweep (post-pattern, 2026-09-02) | 3,371 | 575 |
| Delta | +15 | **+57** |

The +15 files are this repository's own growth across Plans 01–07 (new corpus payloads, new clean
specimens, new test files).

### The delta that matters: one new third-party finding

`--compare` reports 517 findings present in the candidate and absent from the baseline. **516 of
those are an artifact of the comparison key, not of the patterns**, and must not be read as
regressions — see the caveat below. Excluding this repository's own worktree, the genuine
third-party delta across ~3,200 files in 21 external directories is:

| File | Line | Pattern | Severity | Verdict |
|---|---:|---|---|---|
| `17-skills-registry/.claude/settings.local.json` | 3 | PI051 | CRITICAL | **TRUE POSITIVE** |

That file's `permissions.allow` list contains `Bash(rm -rf *)`, `Bash(rm -rf ./*)` and
`Bash(jar xf *)` — real pre-approved unrestricted grants in a sibling repository, exactly the shape
CAT-01 exists to detect. It is reported, not suppressed, and no pattern was widened or narrowed in
response to it. **This is the category justifying itself on its first real sweep.**

**Zero new false positives in third-party documentation.**

### Caveat: the self-repo half of `--compare` is not meaningful this run

`--compare` keys on `path:line:pattern_id`. The baseline swept this repository through Plan 03's
worktree (`.claude/worktrees/agent-ade5856f6fc048a75`), which no longer exists; the after-sweep
swept Plan 07's worktree (`.claude/worktrees/agent-a722421e377079562`). Every self-repo finding
therefore has a different path key and appears "new" — 516 of them — regardless of whether the
finding itself changed. Those 516 were audited rather than dismissed:

- **49 are PI05x**, and every one is in a file that contains the attack strings by construction:
  `tests/corpus/attack/` payloads (13), `examples/tool-permission-abuse-attack.md` (9),
  `patterns/core/tool-permission-abuse.yaml`'s own `example:` fields (7), and
  `tests/pattern_test.rs`'s per-pattern cases (19). Self-scan artifacts of the same class the
  baseline section above already documents, not detections of anything real.
- **The remaining 467 are pre-existing non-CAT-01 findings** carried over unchanged from the
  baseline under a different path prefix.

A future sweep should either pin a stable path for the self-repo entry or teach `--compare` to
normalise the repository root before keying, so this half of the diff carries signal. Logged, not
fixed here — GATE-04 forbids a category PR from also carrying unrelated tooling changes.

### What GATE-03 caught, and why the gate earned its place

The after-sweep was run twice. **The first run is the reason this section exists.**

`PI057` (disable-guardrail-directive) as Plan 06 shipped it — `turn off|disable|remove|skip` within
20 characters of `hook|guardrail|check|scanner|gate` — produced **49 third-party hits, 48 of them
false positives**:

- **`skip` was the dominant driver.** It is ubiquitous in ordinary engineering prose
  (`GSD_SKIP_SCHEMA_CHECK`, `--skip-git-repo-check`, "skip that hook") and, worse, it matched
  **negated** directives backwards: *"DO NOT skip the config gate check"* is an instruction to keep
  a control, and the pattern read it as an instruction to remove one.
- **`remove`** added two more, both dev-tooling prose about removing a *stale* hook reference rather
  than disabling a *live* one.
- Dropping `skip` and `remove` cleared 47 of the 48.
- The 48th — *"Pass `--no-nli` to disable the contradiction check entirely"*
  (`15-llm-diff/README.md:57`) — proved `turn off`/`disable` alone are not enough either: ordinary
  CLI-flag documentation uses the identical verb+object grammar as the attack.

`PI057` was re-narrowed to require **second-person address** — `you` co-occurring with the
verb+object pair inside one sentence — the same framing `PI054`/`PI055` already require. Its
`counter_example` is now that llm-diff sentence, sourced verbatim from the sweep, and
`tests/corpus/clean/cli-flag-reference.md` was added to pin the fix. The clean corpus is now 21
specimens and still scores zero.

This is precisely the failure mode GATE-03 was written for: `PI057` passed every unit test, held
all 20 clean specimens, and scored 12/12 on the threat-model corpus while being wrong 98% of the
time on real documentation. No corpus-based gate would have caught it.
