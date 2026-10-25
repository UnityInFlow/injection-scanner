# injection-scanner: launch plan (2026-10-24)

Tracking issue: [#211](https://github.com/UnityInFlow/injection-scanner/issues/211). Checkout state
at the time of writing: `main` clean, up to date with `origin/main` at `e1bf8b6`.

## Where it stands (verified today)

- Rust static scanner for prompt-injection patterns in files agents read. **84 patterns / 9
  categories** (`patterns/core/`, counted; `CLAUDE.md:19` agrees), not the 79 of the v0.2.0 notes.
- `cargo test`: **482 passed, 0 failed, 0 ignored, 42 suites, 4m37s**. `cargo build --release`:
  5 s with a warm cache, binary 4.9 MB. `tests/recall_test.rs::recall_matches_the_recorded_numbers`
  passes, so the README's 105/115 (91.3%) and the CAT-03 held-out **2/12 (16.7%)** (`README.md:413-444`)
  are CI-pinned claims, not prose.
- 5 releases, latest v0.2.0 (2026-10-10): six raw target-triple binaries + `SHA256SUMS.txt`, SLSA
  attestation; **19 downloads total**. 0 stars, 3 forks, 0 watchers, no topics on the repo.
- Not on crates.io (`/api/v1/crates/injection-scanner` → 404 = name free). No Homebrew formula
  (`UnityInFlow/homebrew-tap` has `agent-memory.rb`, `prompt-vc.rb`). `release.yml:69-70`: "binaries
  only -- there is NO crates.io publish job". No cargo-dist; cross-compile via cargo-zigbuild.
- `Cargo.toml` has description/license/repository/keywords/categories but no `readme`, `rust-version`,
  `exclude` or `[package.metadata.binstall]`. `cargo package`: **447 files, 4.2 MiB (1.3 MiB
  compressed)**: under the 10 MB limit, but 212 of those files are git-tracked `.planning/`.
- README is 1,040 lines; install = download + chmod; `cargo install --path .`; pre-commit example
  pins `rev: v0.0.3` (`README.md:906`). No demo. CI and Code scanning green on `main` (2026-10-23).
- Repo entered **maintenance-only** on 2026-10-23 (`.planning/STATE.md`, `ROADMAP.md:15-23`): bug
  fixes and community PRs merge; no new detection work; ecosystem focus moved to agent-memory.
- Two community PRs merged this month (#197, #200 by BasilZafar11): the only external demand
  signal in the portfolio besides the forks.

## Does this agree with the owner's own notes?

- `00-MASTER-ANALYSIS.md:266` planned exactly this for weeks 5-6: "Pre-built binaries + Homebrew
  tap"; line 408 flags HN viral potential for this tool. Binaries shipped; the tap and the HN post
  never happened. **Agrees**: this plan is the undelivered half.
- `PORTFOLIO-VERDICT.md:25` says shipping was never the bottleneck, distribution and demand were,
  and keeps injection-scanner as Tier 1 (line 35). Its rule "no v0.0.1 until one person who is not
  you has asked for it" is already satisfied here (two merged community PRs). **Agrees in spirit**:
  this is a ~12 h distribution test of the one tool with external validation, not more building.
- It **disagrees** with the repo's own freeze only in that packaging needs the freeze lifted for
  packaging work (Decision 5). No detection category or engine is touched.
- One deviation from the brief: awesome-mcp-servers' Security section lists MCP *servers*; this is a
  CLI, so the two directory targets are awesome-claude-code and awesome-ai-security (#209).

## Launch goal and definition of done

A stranger installs it in one line, sees what it does in 30 seconds, and reads the honest number
before anything else. Done when: `cargo install` and `brew install` one-liners work on a clean
machine; `cargo binstall` resolves the existing assets; README top ≤ 20 lines; 20-30 s GIF; v0.3.0
released with checksums and attestation verified; Show HN posted; listed via PRs in two directories;
CI/CodeQL badges green. Full checklist in #211.

## Depends on / unblocks

Depends on nothing. Unblocks: `spec-ci-plugin` moves its default pin from v0.0.3 to v0.3.0 after this
release; `secure-web` consumes the same binary. Shared conventions (hosted runners, trusted
publishing, tap, VHS, README top, directories): `PORTFOLIO-DECISIONS-2026-10-24.md` in the
workspace root, section "Shared conventions".

## First 2 hours (do before anything else)

1. Step 1a without the binstall metadata: add `exclude` and `readme` to `Cargo.toml`, `cargo publish
   --dry-run --locked`, then `cargo publish --locked` by hand at the current 0.2.0. The crate exists
   from that moment and `cargo install injection-scanner` works today.
2. Set GitHub topics (`prompt-injection`, `ai-security`, `claude-code`, `mcp`, `rust`, `sarif`).
3. Open the Show HN draft from Phase 5 in a file; do not post yet.

**Stop rule.** Four weeks after the Show HN post: fewer than 25 stars, no external issue or PR beyond
the two already merged, and no substantive HN or Reddit comment. Then record a retro on #211, keep
crates.io and the tap installable, and return to maintenance-only. No v0.4 work before that gate.

## Step-by-step plan

### Phase 1: Packaging (~4 h)

| Step | What / why | How | Done when | h | Issue |
|---|---|---|---|---|---|
| 1a | Make `Cargo.toml` publishable; stop shipping `.planning/` and reference docs in the crate; let `cargo binstall` find the raw assets | Add `readme`, `rust-version`, `exclude = [".planning/", "docs/superpowers/", "tools/", "integrations/", ".github/", "CLAUDE.md", "TODO.md", "03-injection-scanner.md", "claude-code-harness-engineering-guide-v2.md"]`, `[package.metadata.binstall]` with `pkg-url = "{ repo }/releases/download/v{ version }/{ name }-{ target }"`, `pkg-fmt = "bin"`. Asset names must not change (release contract, `tests/release_contract_test.rs`) | `cargo package --list` has no `.planning/`; `cargo publish --dry-run --locked` ok; `cargo binstall --dry-run --manifest-path Cargo.toml injection-scanner` resolves | 1.5 | #203 (umbrella #41) |
| 1b | Publish to crates.io from the tag | New `publish-crate` job in `release.yml`, `needs: [release]`, GitHub-hosted, crates.io **trusted publishing** via `rust-lang/crates-io-auth-action` (no org secrets: the file's binding rule, `CLAUDE.md` §CI). First publish of v0.3.0 is manual because trusted publishing is configured only after the crate exists | Job present and actionlint-clean; v0.3.0 on crates.io; `cargo install injection-scanner` prints 0.3.0 | 1.5 | #204 |
| 1c | Homebrew formula in the org tap | `Formula/injection-scanner.rb` in `UnityInFlow/homebrew-tap`, modelled on `agent-memory.rb` but with raw binaries: `url ".../v#{version}/injection-scanner-aarch64-apple-darwin"`, `bin.install "injection-scanner-aarch64-apple-darwin" => "injection-scanner"`, sha256 from `SHA256SUMS.txt`; four `on_macos`/`on_linux` arms. Bump by hand each release (automation would need a PAT in the tap: an org secret) | `brew install unityinflow/tap/injection-scanner && brew test injection-scanner && brew audit --strict injection-scanner` pass on arm64 macOS | 1 | #10 |

### Phase 2: README rewrite (~2 h): #205

Lead with the honest number; move install-by-download below the one-liners; fix `rev: v0.0.3`.
Proposed top section (verbatim draft; badges and one-liners go live only after Phase 1 and 4):

```markdown
# injection-scanner

[CI] [CodeQL] [crates.io] [MIT]

Static scanner for prompt injection in the files your AI agent reads: `CLAUDE.md`, skill files,
MCP tool descriptions, RAG documents. Rust, one binary, no network, about 60 ms on a 40-file repo.

**84 patterns, 9 categories. Measured, not claimed:** 105/115 (91%) on the development corpus and
**2/12 (17%) on a sealed held-out set** for the newest category. Both numbers are pinned in CI and
reported side by side: [how they are measured](#how-much-does-it-actually-catch).

    cargo install injection-scanner        # or: brew install unityinflow/tap/injection-scanner
    injection-scanner check .              # exit 1 on findings; --format sarif for code scanning
    injection-scanner install-hook         # pre-commit hook: scans staged files, blocks HIGH+

![demo](docs/demo/demo.gif)

What it is: a pattern-and-structure linter with a maintained library, baselines and inline
suppression. What it is not: a classifier, a runtime guard or a guarantee: a determined attacker
can phrase around a regex. For runtime detection look at promptfoo, llm-guard or rebuff.
```

Done when the first 20 lines contain what/numbers/one-liners/demo/"what it is not", no number in
them is unpinned by `tests/recall_test.rs`, and `injection-scanner check README.md` still exits 0.

### Phase 3: Demo (~1.5 h): #206

`vhs` is installed (`/opt/homebrew/bin/vhs`); asciinema is not. Tape at `docs/demo/demo.tape`,
output `docs/demo/demo.gif` (< 2 MB, < 30 s). Record in a **scratch repo**, not this one: a
root scan here exits 1 because `docs/PATTERN-CATALOGUE.md` self-matches (#171) and `.planning/`
emits frontmatter warnings. Commands shown:

1. `cat skill.md`: a plausible skill file with one planted PI001 line taken from
   `tests/corpus/attack/role-override.md`.
2. `injection-scanner check skill.md`: verified today to print two CRITICAL findings (PI001,
   PI021) and the summary line, exit 1.
3. `injection-scanner explain PI001`: rule, remediation, suppression comment (10 lines).
4. `injection-scanner install-hook && git add -A && git commit -m "add skill"`: "Commit blocked".

Done when `vhs docs/demo/demo.tape` reproduces the GIF and it is legible at GitHub README width.

### Phase 4: Release v0.3.0 (~1.5 h attention): #207

Follow `docs/RELEASE-CHECKLIST.md` verbatim; the packaging-specific deltas:

- Decisions 3 and 4 below are taken first (milestone rename; freeze lifted for packaging in
  `.planning/STATE.md`, `.planning/ROADMAP.md`, `CLAUDE.md` §Status).
- `CHANGELOG.md`: promote `[Unreleased]` (already holds PI114-PI118) to `## [0.3.0]`, add
  `### Distribution`.
- Bump to 0.3.0, refresh `Cargo.lock`, commit together (the `test` job compares tag vs Cargo.toml).
- Local gate: fmt, clippy `-D warnings`, `cargo test --locked`, `cargo build --release --locked`.
- Tag, watch `test` → `build-binaries` → `release` → `verify-published-assets`. **If a darwin leg
  fails (they are `continue-on-error`), stop and re-tag as v0.3.1**: the formula needs both.
- After publication: manual `cargo publish --locked`, configure trusted publishing, write the
  formula, `cargo binstall` on a clean machine, `gh attestation verify`, `sha256sum -c`.
- Checklist §6: `spec-ci-plugin` pins **v0.0.3**; decide whether it moves to v0.3.0 and record it.

### Phase 5: Launch (~3 h): #208, #209

**Show HN title:** `Show HN: Injection-scanner – static scanner for prompt injection in agent config files`

**Post draft:**

> I built a Rust CLI that scans the files an AI agent reads: CLAUDE.md, skill files, MCP tool
> descriptions, RAG documents: for prompt-injection patterns before they reach the model. 84
> patterns in 9 categories (role override, exfiltration, tool/permission abuse, MCP tool-description
> poisoning, persistence hooks, encoding tricks, multilingual variants). It parses frontmatter as
> data, so a wildcard `allowed-tools` grant or a hook that downloads-and-executes is graded
> structurally, not by regex. JSON and SARIF output, baselines, inline suppression, a pre-commit
> hook, ~60 ms on a 40-file repo. `cargo install injection-scanner` or
> `brew install unityinflow/tap/injection-scanner`.
>
> The number I most want feedback on: on the development corpus it catches 105/115 (91%). On a
> sealed held-out set written blind for the newest category it caught **2/12 (17%)**, and the README
> leads with that, because a score against the material a detector was tuned on says how the build
> went, not how the detector works. Every miss is a public issue. This is a linter, not a
> classifier: it catches the crude and the hidden, and a sentence-shaped payload gets through. I
> would like to know which real artifacts it misses on and which documentation it false-positives on.

**r/ClaudeAI:** title `I wrote a scanner for prompt injection in CLAUDE.md, skills and MCP tool
descriptions: it is honest about what it misses`; two paragraphs (what it checks in a Claude Code
setup incl. the `integrations/claude-code/` PostToolUse hook; the numbers and an explicit ask for
false positives on real CLAUDE.md files), the GIF, one link. Post the day after HN.

**Directories (#209):** PR to `hesreallyhim/awesome-claude-code` `## Security` (56k stars, active)
and `ottosulin/awesome-ai-security` `### MCP Security` or `### Safety Classifiers & Prompt Injection
Detection` (1.6k, active). awesome-mcp-servers evaluated and skipped (servers only). Timing:
Tue-Thu 14:00-16:00 UTC, two free hours afterwards.

### Phase 6: First week (~1 h/day): #210

Watch: new issues/PRs (answer within 24 h), release and crates.io download counts, stars/forks,
the HN thread for 6 h, Reddit for 48 h. Triage: false positive on ordinary documentation = **P1**,
fix in a v0.3.x within days (add the text to `tests/corpus/clean/`); install breakage = **P1**,
same day; detection gap = P2, into `docs/DETECTION-BACKLOG.md` and the detection milestone, thank,
no date; feature requests → existing issues. Do not promise: Windows (#9), classifier (#111),
runtime mode (#11), or recall by a date. Day 7: retro comment on #211.

## Decisions the owner must make

Defaults are recorded in `PORTFOLIO-DECISIONS-2026-10-24.md`, "Owner decisions"; shared ones (hosted
runners, trusted publishing, tap by hand, Windows deferred) are S1 to S5 there and not repeated.

1. **Crate name.** Keep `injection-scanner`: free on crates.io, equals the binary name and
   `#[command(name)]` (`src/main.rs:25`), and the release asset names depend on it. Default.
2. **Crate contents.** (a) Keep `tests/` + corpus (1.0 MB) and exclude `.planning/` and the reference
   docs, crate about 230 files, well under 10 MB; or (b) source-only. (a) is the default: the corpus
   is the evidence behind the published numbers and lets a `cargo` user run `recall_test`.
3. **Version and milestone.** The GitHub milestone `v0.3.0` holds 36 open detection-gap issues with
   no plan or date. Default: rename it `v0.4.0: detection gaps` and ship this as v0.3.0 (new install
   channels justify a minor bump). Alternative: ship as v0.2.1 and leave the milestone alone.
4. **Freeze.** Lift the maintenance-only freeze for packaging and docs only, and record that in
   `STATE.md`/`ROADMAP.md`/`CLAUDE.md` so the next agent session does not read it as a contradiction.

## Later, only if demand (gated on the stop rule not firing)

- v0.4: the P1 held-out miss #176 (download-then-run lifecycle hook, 4/4 structural misses) first;
  then whatever the first week's false-positive reports say, before any new category.
- `.pre-commit-hooks.yaml` already exists (`language: rust`); a GitHub Action wrapper and the
  library/CLI split (#41) only when someone other than spec-ci-plugin wants to embed it.
- spec-ci-plugin: move its default pin from v0.0.3 to v0.3.0 and use the SARIF upload path (#5).

## Issue index

| Issue | Role |
|---|---|
| [#211](https://github.com/UnityInFlow/injection-scanner/issues/211) | Tracking: Launch v0.3.0 (this plan) |
| [#203](https://github.com/UnityInFlow/injection-scanner/issues/203) | 1a Cargo.toml metadata, exclude, binstall |
| [#204](https://github.com/UnityInFlow/injection-scanner/issues/204) | 1b release.yml crates.io publish job |
| [#10](https://github.com/UnityInFlow/injection-scanner/issues/10) | 1c Homebrew formula (existing, commented) |
| [#41](https://github.com/UnityInFlow/injection-scanner/issues/41) | Distribution umbrella (existing, commented) |
| [#205](https://github.com/UnityInFlow/injection-scanner/issues/205) | 2 README top rewrite |
| [#206](https://github.com/UnityInFlow/injection-scanner/issues/206) | 3 Demo recording |
| [#207](https://github.com/UnityInFlow/injection-scanner/issues/207) | 4 v0.3.0 release checklist |
| [#208](https://github.com/UnityInFlow/injection-scanner/issues/208) | 5 Show HN + r/ClaudeAI drafts |
| [#209](https://github.com/UnityInFlow/injection-scanner/issues/209) | 5 Directory submissions |
| [#210](https://github.com/UnityInFlow/injection-scanner/issues/210) | 6 First-week response plan |
| [#9](https://github.com/UnityInFlow/injection-scanner/issues/9) | Windows binary: explicitly out of scope (commented) |
| [#171](https://github.com/UnityInFlow/injection-scanner/issues/171) | Self-scan exits 1: why the demo runs in a scratch repo (commented) |
