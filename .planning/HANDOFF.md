# Handoff — injection-scanner, Production Readiness milestone

**Written:** 2026-08-21 · **Read this first when resuming.**
Everything needed to continue without the prior conversation.

---

## Where we are

**Milestone:** Production Readiness — v0.0.3 + v0.1.0 (`.planning/ROADMAP.md`, 4 phases)
**Phase 1:** ✅ merged · **Phase 2:** all 8 tasks done — T7/T8 sit in open PRs · **Phases 3-4:** not started
**`main` is green.** 75 tests. CI runs on every PR and takes ~2 minutes.
**Nothing is released.** No v0.0.3 tag. No user has any of this yet.

### What triggered all of this

A deep-dive audit of v0.0.2 found the tool did not do what its README claimed:
sentence-case payloads were missed, one binary file aborted a scan, `--format sarif`
returned text, the scanner flagged its own documentation, and CI had been dead since
2026-06-24. Full findings and evidence: **`docs/AUDIT-2026-08.md`** (read §3 and §4).
Everything since has been working through that list.

---

## Open work

### PR #55 — `--no-suppress` (branch `feat/suppression-trust-boundary`)
Rebased on `main`, CI green, 81 tests. **Not merged on purpose** — see "Next steps".

Suppression directives live *inside the scanned file*, so whoever can edit that file
decides what the scanner reports. Since this tool scans untrusted content, the adversary
is usually the document author. #55 does not try to remove that (impossible — every
linter with inline suppression has it) but makes it **visible** (suppressed findings are
reported and appear in JSON) and **refusable** (`--no-suppress`).

### Issue #43 + #56 — `spec-ci-plugin` consumer defects — **fixed, PR open**

**spec-ci-plugin PR #9** (`fix/injection-scanner-consumer`) fixes all four. CI green, 41 tests
(19 new, hermetic — both `fetch` and the binary are injectable). Not merged yet.

Verified against the real v0.0.2 release *and* a local build of #55: the `ignore-file` payload
from #56 fails the gate by default and passes only under `allow-suppressions: true`.

The one design call worth knowing: `--no-suppress` support is probed from the binary
(`check --help`) rather than inferred from the version string. That decoupled the work from the
v0.0.3 tag — nothing here waits on #55 — and a repo pinned to an older release degrades with a
visible note in the PR comment instead of dying on an unrecognised argument. `DEFAULT_SCANNER_VERSION`
stays `v0.0.2` until v0.0.3 exists; bump it then.

Original defect list, for reference:
In the sibling repo `../04-spec-ci-plugin`, file `src/injection-scanner.ts`. Three defects,
all verified:
1. **No integrity check** — `curl` → `chmodSync(0o755)` → `execFileSync`. `SHA256SUMS.txt`
   is published and never consulted.
2. **Cache not version-keyed** — `join("/tmp", binaryName)` with
   `if (existsSync(downloadPath)) return downloadPath`. On persistent self-hosted runners
   the first binary ever downloaded is executed forever; pinning the version does nothing.
3. **Version defaults disagree** — `action.yml:25` → `v0.0.2`, `src/index.ts:22` → `v0.0.1`.
   The v0.0.1 release used a *different asset naming scheme*
   (`injection-scanner-linux-x86_64`), so the fallback path 404s.

Also **#56**: that Action should pass `--no-suppress`, since it scans contributor-controlled
pull requests. Do #43 and #56 together in one pass against tool 04.

### `suppressed` record shape — **fixed in #55 (`fe08083`)**

`suppressed` is now `Vec<ScanMatch>`, identical to `matches`. It was a four-field stub that could
not describe the finding it recorded, which defeats the purpose of recording it. Pinned by
`tests/suppression_symmetry_test.rs`. `SuppressedMatch` was never released, so nothing depended on
the old shape.

### Issue #29 — perf regression guard — **fixed, PR #58 open**

CI green. **Correction to the earlier plan recorded here:** the "500 files under ~1s"
wall-clock bound suggested in this file would have *passed* on the regression it was
meant to catch — the regressed build was 806ms. The guard shipped is a ratio instead
(500-file scan versus one pattern-set compile: ~1.8× now, trips at 50×, and a per-file
compile cannot come in under 500×). Verified by injecting the regression: 15.2s against
a 1.41s budget. Criterion benches cover all four shapes #29 asks for, and a CI
end-to-end release-binary gate measures **13ms** against the 200ms PERF-01 budget.

Scope note: this closed item 2 of #29 only. Coverage gating, the false-positive corpus
(that is QUAL-03, Phase 3) and fuzzing remain open on the issue.

### Issue #18 — asset contract — **fixed, PR #59 open**

CI green. Contract is now enforced in two places rather than described in a comment:
`tests/release_contract_test.rs` (parses `release.yml` as YAML in the normal test gate,
checks the workflow still produces the names tool 04 requests) and a
`verify-published-assets` release job (walks the consumer's exact anonymous download
path at the new tag). Verified by six mutations, each failing exactly one assertion.

One mutation found a gap nobody was watching: the upload glob list and the
`attest-build-provenance` `subject-path` list are independent, and nothing kept them in
step — an asset dropped from `subject-path` still ships, silently without the provenance
the release notes tell consumers to verify. Now asserted.

### Dependabot #47–#50 — Action major-version bumps
`checkout` v4→v7, `cache` v4→v6, `upload-artifact` v5→v7, `attest-build-provenance` v2→v4.
An external review checked each against the inputs actually used and graded **all four SAFE**
(no used input was removed or renamed; checkout v7 is a security improvement). Not yet merged.

---

## Next steps, in order

1. ~~**#43 + #56**~~ — done, `spec-ci-plugin` PR #9, awaiting merge.
2. ~~**External review of #55 + PR #9**~~ — done. Prompt:
   `scratchpad/opencode-suppression-trust-review-prompt.md`, report:
   `…-review-report.md`. **The review found nothing and graded all four safe to merge**; an
   adversarial re-check found four real holes, all reproduced and now fixed. See STATE.md for the
   list. Two are worth remembering as classes, not incidents:
   - **Every failure path in the consumer used to return `warn`, and the adversary controls the
     input that triggers them.** An oversized report overflowed Node's 1MB `execFileSync` buffer and
     the gate passed. "Unanswered scan" is now `fail`; only acquisition failure still warns.
   - **A verified download stays verified for exactly one run** unless the cache is re-checked.
   The reviewer read the code accurately and confirmed the PR descriptions rather than attacking
   them. A zero-defect review across four PRs is a signal about the review.
3. **Merge the queue**: `spec-ci-plugin` PR #9, injection-scanner #55, #58 and #59, and
   Dependabot #47–#50 (all four already graded SAFE; #49 and #48 also clear the Node 20
   deprecation warning now showing on every CI run).
4. **Tag v0.0.3.** Phase 2 code work is complete — nothing but merges stands in the way.
5. **Bump `DEFAULT_SCANNER_VERSION` to v0.0.3** in `04-spec-ci-plugin` once the tag exists —
   that is when `--no-suppress` starts applying by default. Until then the Action reports the gap
   rather than closing it.
6. Phase 3 (signal quality) — see `.planning/ROADMAP.md`.

---

## Decisions that must not be silently reverted

| Decision | Why | Where recorded |
|---|---|---|
| `ci.yml` and `release.yml` run on **GitHub-hosted** runners | Repo is public under `allows_public_repositories: false`, so **no self-hosted job can be scheduled**. A self-hosted label means the job queues until cancelled — that is what killed CI for two months. Sanctioned D-02 exception. | Root `CLAUDE.md` decisions log (Aug 2026 rows), repo `CLAUDE.md` CI section, in-file comments |
| Release is GitHub-hosted **with SLSA provenance** | Release uses **no org secrets**, only `GITHUB_TOKEN`, and is tag-triggered so forks cannot fire it. Ephemeral runner is better provenance than a persistent box whose `/tmp` survives between jobs. | Same |
| All Actions are **SHA-pinned** | A compromised action could alter source *before* the build that produces signed provenance — attestation over unpinned actions vouches confidently for tampered code. | `.github/dependabot.yml` comment |
| **JSON top-level stays an array** | `spec-ci-plugin` does `JSON.parse(output) as Array<...>` and reads `reports[0]`. An envelope breaks it. New fields must be *additive*. Pinned by a test. | `tests/scan_resilience_test.rs` |
| musl assets stay **raw, unextensioned, target-triple-named** | The Action downloads and executes them directly. | `release.yml` header comment |
| Patterns are **case-insensitive by default** | Pressing Shift previously defeated the scanner. `case_sensitive: true` is the opt-out. | `src/pattern.rs` |
| **Embedded patterns strict, external lenient** | Embedded are CI-tested constants; external are untrusted, so one malformed community YAML must not deny service. `--strict-patterns` opts in. | `src/patterns/mod.rs` |

---

## Traps already hit — do not repeat

- **Deleting a base branch auto-closes stacked PRs.** Merging #44 with `--delete-branch`
  closed #46; closed PRs cannot be retargeted, so it had to be reopened as #51. Rebase
  dependent PRs *before* deleting anything.
- **Rebase-merge rewrites SHAs**, so every stacked PR below needs rebasing afterwards.
- **Backticks inside `git commit -m "..."`** are command substitution in the shell. One
  commit message shipped with a hole in it. Use `-F -` with a quoted heredoc.
- **Test-first caught a contract break**: the FIX-03 test originally asserted a JSON
  envelope, and checking the consumer before implementing is what revealed it would break
  tool 04.

---

## Review history

Three external review rounds (reports in `scratchpad/opencode-*-report.md`). Pattern each
time: **finds 1–4 real defects, and overstates or misframes at least one.** Verify every
claim by reproduction before acting. Examples:
- Correctly found PI012 matching this tool's own suppression syntax (a real own-goal).
- Correctly found `deny_unknown_fields` defeating the lenient path (a real regression).
- **Wrongly** reported a "43% false-positive increase" — most of it was correct new
  detection on attack fixtures.
- **Wrongly** proposed comment markers as the `ignore-file` fix — theater, since an attacker
  can write a comment marker too.
- **Wrongly** graded per-line suppression "safe" from a test that put the directive on the
  wrong line.

Two bugs were found by **self-review, not the reviewers**: the suppressed-count undercount
(`is_match` vs `find_iter`) and the subdirectory walk abort.

---

## Useful commands

```bash
cargo test --locked                                    # 75 on main, 81 on #55
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --all
gh pr list -R UnityInFlow/injection-scanner
gh run list -R UnityInFlow/injection-scanner --limit 5
```

## Map

- `docs/AUDIT-2026-08.md` — findings, evidence, revision history
- `docs/DETECTION-BACKLOG.md` — 30 → ~128 patterns, 8 detection engines (v0.2.0+)
- `docs/ROADMAP-v0.1.0.md` — release sequencing to v1.0.0
- `.planning/ROADMAP.md` · `REQUIREMENTS.md` (24 reqs) · `STATE.md` · `phases/phase-*/PLAN.md`
- `TODO.md` — checklist mapped to issues
