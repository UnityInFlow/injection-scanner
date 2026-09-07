---
phase: 3
slug: tool-permission-abuse-cat-01-33
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-09-01
---

# Phase 3 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` harness via `cargo test`; `criterion` 0.8 for benches only (not a CI gate) |
| **Config file** | none — `[dev-dependencies]` in `Cargo.toml` |
| **Quick run command** | `cargo test --test recall_test --test corpus_test --test pattern_test` |
| **Full suite command** | `cargo test --locked` |
| **Estimated runtime** | ~60 s full suite (331 tests at phase start) |

The full gate additionally requires `cargo fmt --all -- --check` and
`cargo clippy --all-targets --locked -- -D warnings`.

---

## Sampling Rate

- **After every task commit:** `cargo test --test recall_test --test corpus_test --test pattern_test`
- **After every plan wave:** `cargo test --locked`
- **Before `/gsd-verify-work`:** full suite green, catalogue regenerated, whole-repo self-scan clean
  outside `examples/`, `patterns/`, `tests/`, `tools/`, and the GATE-03 delta sweep recorded
- **Max feedback latency:** ~60 s

**The sweep is not substitutable by tests.** Running the RELEASE binary over real directories
(`scripts/gate03-sweep.sh`, and `check .` for the self-scan) is a separate, mandatory signal: 16
green decoder unit tests shipped alongside a production panic that only the sweep found. That is a
recorded blocking anti-pattern on this milestone.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 01 | 1 | CAT-01, GATE-01 | T-03-01/02/03 | Structural corpus is collected, parseable and reachable | integration | `cargo test --test recall_test` | ❌ W0 (new collector + 3 tests) | ⬜ pending |
| 03-01-02 | 01 | 1 | GATE-01 | T-03-02 | 12 threat-model payloads, no duplicates | integration | `cargo test --test recall_test` | ✅ | ⬜ pending |
| 03-01-03 | 01 | 1 | GATE-02 | T-03-01 | Baseline pinned exactly in test and README | integration | `cargo test --locked` | ✅ | ⬜ pending |
| 03-02-01 | 02 | 1 | GATE-05 | T-03-07 | Runbook and reference doc stay unreported under `--strict` | integration | `cargo test --test corpus_test` | ✅ | ⬜ pending |
| 03-02-02 | 02 | 1 | GATE-05 | T-03-05/06 | Deny list and narrow grant stay unreported; frontmatter proven parsed | integration | `cargo test --test corpus_test` | ✅ | ⬜ pending |
| 03-03-01 | 03 | 1 | GATE-03 | T-03-08/09 | Sweep script reports findings when present, none when absent | script self-test | `bash scripts/gate03-sweep.sh /tmp/sweep-selftest tests/corpus/clean` | ❌ W0 (new script) | ⬜ pending |
| 03-03-02 | 03 | 1 | GATE-03 | T-03-09 | Pre-pattern sweep recorded over a named set | manual + script | `test -s /tmp/gate03-baseline/summary.tsv` | ❌ W0 | ⬜ pending |
| 03-03-03 | 03 | 1 | CAT-01 | T-03-10 | ROADMAP criteria match D-10/D-11/D-16 | assertion | `test "$(grep -c '^### Phase' .planning/ROADMAP.md)" = "5"` | ✅ | ⬜ pending |
| 03-04-01 | 04 | 1 | CAT-01 | — | One-way field name chosen by the developer | checkpoint | human-check | N/A | ⬜ pending |
| 03-04-02 | 04 | 1 | GATE-05 | T-03-13/14 | Field additive, typo-rejected, absent from catalogue | unit | `cargo test --locked` | ✅ | ⬜ pending |
| 03-04-03 | 04 | 1 | GATE-05 | T-03-11/12 | Pairing gate proven by fixture, not by absent patterns | unit | `cargo test --test pattern_relaxed_control_test` | ❌ W0 (new file) | ⬜ pending |
| 03-04-04 | 04 | 1 | GATE-05 | T-03-11 | PI050+ cannot ship without the field | unit | `cargo test --test pattern_policy_test` | ✅ | ⬜ pending |
| 03-05-01 | 05 | 2 | CAT-01 | T-03-19 | One-way CRITICAL confirmed by the developer | checkpoint | human-check | N/A | ⬜ pending |
| 03-05-02 | 05 | 2 | CAT-01 | T-03-16/17 | Wildcard grant CRITICAL in all three projection forms; category registered | unit + CLI | `cargo test --locked` | ✅ | ⬜ pending |
| 03-05-03 | 05 | 2 | CAT-01 | T-03-15 | Deny list never reported; `.allow` narrowing mutation-proven | unit + CLI | `cargo run --release -- check tests/corpus/clean --strict --format json` | ✅ | ⬜ pending |
| 03-05-04 | 05 | 2 | GATE-02 | — | Structural recall delta pinned; catalogue regenerated | integration | `cargo run --release -- rules --format markdown \| diff - docs/PATTERN-CATALOGUE.md` | ✅ | ⬜ pending |
| 03-06-01 | 06 | 3 | CAT-01, GATE-05 | T-03-20 | Bypass-flag persuasion HIGH; runbook and reference stay silent | unit + CLI | `cargo test --locked` | ✅ | ⬜ pending |
| 03-06-02 | 06 | 3 | CAT-01, GATE-04 | T-03-21/23/24 | Settings widening keys on the object, not the filename; one category file touched | unit + CLI | `git diff --name-only patterns/core/` | ✅ | ⬜ pending |
| 03-06-03 | 06 | 3 | GATE-02 | — | Both recall rows pinned in test and README; D-12 documented | integration | `cargo test --locked` | ✅ | ⬜ pending |
| 03-07-01 | 07 | 4 | GATE-03 | T-03-25/26/29 | Delta sweep triaged; self-scan clean; no crash | manual + script | `test -s /tmp/gate03-after/summary.tsv` | ❌ W0 (depends on 03-03) | ⬜ pending |
| 03-07-02 | 07 | 4 | GATE-02 | T-03-27 | Perf re-measured; every published number reconciled | integration | `cargo test --test perf_regression_test` | ✅ | ⬜ pending |
| 03-07-03 | 07 | 4 | GATE-04 | T-03-28 | Deferrals filed as issues; full gate green | gate | `cargo fmt --all -- --check && cargo clippy --all-targets --locked -- -D warnings && cargo test --locked` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Created inside Wave 1 rather than as a separate pre-wave, because each piece belongs to the plan
that owns its concern:

- [ ] `tests/recall_test.rs` — `structural_dir()`, `structural_payloads()`, `measure_structural()`,
      plus `the_structural_corpus_is_actually_collected`,
      `every_structural_payload_parses_as_frontmatter` and
      `the_structural_pass_is_reachable_from_the_corpus` (Plan 01, Task 1)
- [ ] `tests/corpus/attack/structural/` and `tests/corpus/attack/tool-permission-abuse.md`
      (Plan 01, Tasks 1-2)
- [ ] Five `tests/corpus/clean/` specimens (Plan 02)
- [ ] `scripts/gate03-sweep.sh` with its own self-test (Plan 03, Task 1)
- [ ] `tests/pattern_relaxed_control_test.rs` with its fixture mechanism self-test (Plan 04, Task 3)
- [ ] `every_pi05x_pattern_carries_a_relaxed_pattern` in `tests/pattern_policy_test.rs`
      (Plan 04, Task 4)

Everything else runs on existing infrastructure: `corpus_test`, `pattern_test`,
`pattern_example_test`, `pattern_policy_test`, `catalogue_test`, `perf_regression_test` and
`recall_test` all already exist and are green at 331 tests.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| GATE-03 third-party sweep | GATE-03 | The swept corpus is machine-local — `~/.claude/plugins`, sibling repositories, GSD reference docs — and is not vendored, pinned or reproducible from this repository. Verified by research: no script, no CI job, only prose mentions of the original 1,300-file figure. | `scripts/gate03-sweep.sh` makes the *procedure* reproducible; run it over the directory list recorded in `03-SWEEP.md` before and after the patterns, then `--compare` the two. The delta is the evidence, not an absolute count. |
| Whole-repo self-scan | GATE-03 | Needs the release binary over the real working tree; expected output is a judgement (`[]` outside the corpus directories), not a boolean | The one-liner in `.claude/skills/pattern-library/SKILL.md` §"Scan the whole repo" |
| D-08 field name | CAT-01 | One-way schema decision on a community-contribution surface | `checkpoint:decision`, Plan 04 Task 1 |
| D-12 CRITICAL behaviour change | CAT-01 | One-way consumer-visible change; breaks green `spec-ci-plugin` builds on upgrade | `checkpoint:decision`, Plan 05 Task 1 |
| Mutation checks | GATE-05 | Each requires temporarily breaking something and restoring it, which cannot be a committed test | Enumerated as acceptance criteria in Plans 01, 04 and 05 |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or are checkpoints with `<human-check>`
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 60 s for the quick command
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
