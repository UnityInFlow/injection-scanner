---
phase: quick/260828-cli
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - SECURITY.md
  - CHANGELOG.md
  - docs/RELEASE-CHECKLIST.md
  - .github/CODEOWNERS
  - .github/ISSUE_TEMPLATE/config.yml
  - .github/ISSUE_TEMPLATE/bug_report.yml
  - .github/ISSUE_TEMPLATE/pattern_proposal.yml
  - .github/ISSUE_TEMPLATE/false_positive.yml
autonomous: true
requirements: [HYGIENE-01, HYGIENE-02, HYGIENE-03, HYGIENE-04, HYGIENE-05]
user_setup: []

estimate:
  tokens: 78000
  raw_tokens: 52000
  tasks: 3
  confidence: low

must_haves:
  truths:
    - "A person who finds a crash in the scanner has a private channel to report it, and that channel is actually enabled on the repository."
    - "A person who finds a payload the scanner misses is told, in writing, to file it publicly — it is a tracked gap, not an embargoed vulnerability."
    - "A fork PR touching patterns/, src/, .github/workflows/ or SECURITY.md automatically requests review from the maintainer."
    - "A new contributor opening an issue is routed into one of three structured forms and cannot open a blank issue."
    - "A pattern proposal cannot be submitted without a severity graded against the PATTERNS.md criteria and a false-positive analysis."
    - "Someone cutting a release can follow one document from pre-tag gate to verified published binaries without reading release.yml."
    - "CHANGELOG.md states what shipped in each of the three real releases and what is on main past v0.0.3 — and nothing that is still on an unmerged branch."
    - "Every new file returns zero findings when scanned by this repository's own scanner at default settings."
  artifacts:
    - SECURITY.md
    - CHANGELOG.md
    - docs/RELEASE-CHECKLIST.md
    - .github/CODEOWNERS
    - .github/ISSUE_TEMPLATE/config.yml
    - .github/ISSUE_TEMPLATE/bug_report.yml
    - .github/ISSUE_TEMPLATE/pattern_proposal.yml
    - .github/ISSUE_TEMPLATE/false_positive.yml
  key_links:
    - "SECURITY.md advisory URL -> GitHub private vulnerability reporting must be ENABLED on the repo (it is currently disabled — Task 1 enables it)."
    - "ISSUE_TEMPLATE/config.yml contact links -> SECURITY.md and PATTERNS.md, both of which must exist at the linked path on main."
    - "docs/RELEASE-CHECKLIST.md -> .github/workflows/release.yml job names, target triples and asset names must stay in step."
    - "New markdown -> the repository's own scanner: every new file must return exit code 0 on `check`."
---

<objective>
Add the five on-disk repo-hygiene artifacts a public, downstream-consumed, fork-contributed
security tool is missing: a vulnerability policy, code ownership, structured issue intake, a
changelog, and a real release checklist.

Purpose: repo *settings* were hardened earlier in this session (merge commits disabled, `main`
ruleset requiring linear history + PR + the `fmt · clippy · test · build` check, force-push and
deletion blocked, secret scanning + push protection on, delete-branch-on-merge on). None of that
tells a reporter where to send a crash, tells a contributor what a good pattern proposal looks
like, or tells the maintainer what to do between `git tag` and "the binaries are verified". Those
are files, and they do not exist.

Output: `SECURITY.md`, `.github/CODEOWNERS`, `.github/ISSUE_TEMPLATE/{config,bug_report,pattern_proposal,false_positive}.yml`,
`CHANGELOG.md`, `docs/RELEASE-CHECKLIST.md` — plus GitHub private vulnerability reporting turned on,
because SECURITY.md points at it.
</objective>

<execution_context>
@$HOME/.claude/gsd-core/workflows/execute-plan.md
@$HOME/.claude/gsd-core/templates/summary.md
</execution_context>

<context>
@CLAUDE.md
@CONTRIBUTING.md
@PATTERNS.md
@.github/PULL_REQUEST_TEMPLATE.md
@.github/workflows/release.yml
@.planning/STATE.md
</context>

<ground_truth>

Everything below was read out of the repo, git, or the GitHub API during planning. **Do not
re-derive it by guessing; if an executor needs a fact that is not here, read it out of the source
named — and if it cannot be verified, omit it rather than write it.**

## Repository facts (verified 2026-08-28)

| Fact | Value | Source |
|---|---|---|
| Repo | `UnityInFlow/injection-scanner`, **public** | `gh api repos/…` |
| Maintainer / owner handle | `hermanngeorge15` | task brief + git author |
| GitHub Discussions | **disabled** (`has_discussions: false`) | `gh api repos/…` |
| Private vulnerability reporting | **DISABLED** (`{"enabled":false}`) — Task 1 enables it | `gh api repos/…/private-vulnerability-reporting` |
| Secret scanning + push protection | enabled | `gh api repos/…` |
| `Cargo.toml` version | `0.0.3` | `Cargo.toml:3` |
| Current branch | `chore/repo-hygiene`, contains `origin/main` (`c1511b8`) | `git` |
| Existing `.github/` contents | `workflows/`, `dependabot.yml`, `PULL_REQUEST_TEMPLATE.md` — nothing else | `ls` |
| CI status check name | `fmt · clippy · test · build` (single job `test`, `ubuntu-latest`) | `.github/workflows/ci.yml` |

## Releases — dates come from `published_at`, NOT from STATE.md

`.planning/STATE.md` records v0.0.1 as 2026-04-01 and v0.0.3 as 2026-08-23. **Both are wrong.**
`gh api repos/UnityInFlow/injection-scanner/releases` gives the authoritative published dates.
Use the API values. (STATE.md is the orchestrator's file; do not edit it here.)

| Tag | Published (`published_at`) | Assets |
|---|---|---|
| `v0.0.1` | **2026-04-02** | `injection-scanner`, `injection-scanner-darwin-arm64`, `injection-scanner-linux-x86_64` (pre-target-triple naming) |
| `v0.0.2` | **2026-06-24** | the six `injection-scanner-<target-triple>` binaries + `SHA256SUMS.txt` |
| `v0.0.3` | **2026-08-22** | the six `injection-scanner-<target-triple>` binaries + `SHA256SUMS.txt` |

## `release.yml` — the four jobs, verbatim names

1. `test` — *"Test gate (test + clippy + fmt)"*. Contains the **tag-must-match-crate-version**
   guard (reads `cargo metadata`, selects the package **by name** `injection-scanner`, compares to
   `${GITHUB_REF_NAME#v}`). Then `cargo test --locked`, `cargo clippy -- -D warnings`,
   `cargo fmt --check`.
2. `build-binaries` — *"Build ${{ matrix.target }}"*, `needs: test`. One Linux host cross-compiles
   all six targets with `cargo zigbuild`. Pinned `ZIG_VERSION: 0.14.1`,
   `CARGO_ZIGBUILD_VERSION: 0.23.0`. Packages a **raw, chmod +x, target-triple-named** binary and a
   per-asset `.sha256` sidecar. Host-arch-aware smoke test.
3. `release` — *"Create GitHub Release"*, `needs: [build-binaries]`. Permissions
   `contents: write`, `id-token: write`, `attestations: write`. Hard-requires the four Linux
   assets, treats the two darwin assets as conditional, generates `SHA256SUMS.txt` over all present
   raw binaries, signs provenance with `actions/attest-build-provenance`, creates the Release.
4. `verify-published-assets` — *"Verify published asset contract (spec-ci-plugin)"*,
   `needs: [release]`, `permissions: contents: read`. Runs **after** publication: anonymous `curl`
   of `https://github.com/UnityInFlow/injection-scanner/releases/download/<tag>/<asset>`, ELF magic
   `7f454c46`, `e_machine` at offset 18 (`3e00` x86_64 / `b700` aarch64), presence in
   `SHA256SUMS.txt`, `sha256sum -c --ignore-missing`, `--version`, and a `check` on a sample file.

Trigger: `on: push: tags: ['v*']`. All four jobs `runs-on: ubuntu-latest`.

## The six targets and their asset names

| Target triple | Asset name | Required? |
|---|---|---|
| `x86_64-unknown-linux-musl` | `injection-scanner-x86_64-unknown-linux-musl` | **hard-required** (consumer) |
| `aarch64-unknown-linux-musl` | `injection-scanner-aarch64-unknown-linux-musl` | **hard-required** (consumer) |
| `x86_64-unknown-linux-gnu` | `injection-scanner-x86_64-unknown-linux-gnu` | hard-required |
| `aarch64-unknown-linux-gnu` | `injection-scanner-aarch64-unknown-linux-gnu` | hard-required |
| `x86_64-apple-darwin` | `injection-scanner-x86_64-apple-darwin` | conditional (`experimental: true`, `continue-on-error`) |
| `aarch64-apple-darwin` | `injection-scanner-aarch64-apple-darwin` | conditional |

Plus `SHA256SUMS.txt`. Consumer verification command, verbatim from `release.yml`:

    gh attestation verify injection-scanner-x86_64-unknown-linux-musl --repo UnityInFlow/injection-scanner

## `main` past `v0.0.3` — the 18 commits (`git log v0.0.3..main --oneline`)

```
c1511b8 chore: preserve the injection lab that drove #65 and #23
ce5d658 docs: Phase 3 is complete and the adoption POC landed
1b49334 feat: install-hook — the v0.0.1 promise, three milestones late (#8)
c9f1404 feat: --fail-on, --quiet, exit code 2, rules and explain (#25)
0278130 feat: rebalance severity across the full range (#21)
6a0cfe2 feat: obfuscation is no longer a bypass (#26)
57b3551 feat: a newline is no longer a bypass (#24)
826eb6c feat: scan what agents actually ingest, and stop calling prose an attack (#23)
433e8a4 test: a false-positive corpus, because negative tests were not enough (QUAL-03)
f3a5612 docs: STATE.md said Phase 2 with zero open PRs; both were false
c04e0cb feat: replace the hand-rolled walker with the ignore crate (#22)
dc360f9 fix: withhold low-confidence findings, never discard them
44a09f7 feat: markdown context awareness — documentation is not an attack (#20)
dcf4ae0 docs: three SHA pins claimed versions they were not
ebf85e6 fix: make #[serde(default)] on `suppressed` mean something
e980d43 docs: restriction lints are off by default, and -D warnings does not change that
71cbe84 fix: actually deny unwrap_used in src/ — the other half of #19
4dcc6e7 fix: the tag guard read packages[0], which breaks on the planned lib split
```

**NOT on main — must not appear anywhere in CHANGELOG.md:** `--baseline` (PR #79,
`feat/cli-08-baseline`) and SARIF output (PR #82, `feat/cli-04-sarif`). Both verified OPEN and
unmerged. `.github/code-scanning-baseline.json` and `.github/workflows/code-scanning.yml` exist
**only on the #82 branch**.

## Detection-gap issues (verified OPEN)

- **#80** — "patterns: role-override patterns are near-literal — common synonyms defeat every one"
- **#81** — "test: nothing measures recall — the clean corpus gates false positives, no gate exists
  for missed attacks"

These are the public evidence that a missed detection is a tracked gap, not an embargoed
vulnerability. Backlog document: `docs/DETECTION-BACKLOG.md`.

## Severity grading criteria — copy the *tests*, not paraphrases (PATTERNS.md "Grading Severity")

| Severity | Test |
|---|---|
| CRITICAL | No plausible benign reading. If this string is in a document, something is wrong. |
| HIGH | Strong signal; benign use is rare and usually deliberate. |
| MEDIUM | Suspicious in context, but ordinary documents genuinely say this. |
| LOW | A heuristic or a weak signal. Names a concept, or could be an artefact. |

The three deciding rules: **naming is not carrying**; **one is an artefact, several are intent**;
**if you had to imagine the benign case, it is not CRITICAL**.

Category / ID ranges (PATTERNS.md + CONTRIBUTING.md agree): Role Override `PI001-PI009`;
Instruction Injection `PI010-PI019`; Data Exfiltration `PI020-PI029`; Jailbreaks `PI030-PI039`;
Encoding/Obfuscation `PI040-PI049`.

Submission bar (PATTERNS.md §Submitting a Pattern): at least **3 true positive** cases and at
least **2 non-match** cases. False-positive corpus lives at `tests/corpus/clean/` and must produce
**zero** findings; `tests/corpus/documentation/` must produce zero by default and more than zero
under `--strict`.

## Two live traps

**Trap 1 — the repo scans itself.** `tests/corpus_test.rs` enforces zero findings on
`tests/corpus/clean/`, and `README.md` documents that it produces zero findings by default and 15
under `--strict`. The new files are not in either corpus, so `cargo test` will not catch a
regression here — the explicit self-scan in each task's `<verify>` is the only gate. **Write no
verbatim injection payload into any new file.** Name the attack class, cite the pattern ID, do not
quote the string.

**Trap 2 — `main`'s docs describe unreleased behaviour.** `--strict`, `--fail-on`, `--quiet`,
`rules`, `explain`, `install-hook` and markdown context awareness are all on `main` and **not in
v0.0.3**. Issue templates are served from `main` to users who are probably on `v0.0.3`, so the
templates must not require any version-specific flag or subcommand. Ask for `--version` output and
work from that.

## CLI shape that constrains the verify commands

`check` takes **one** `path` argument, not a list — `src/main.rs:109`. Scan one path per
invocation. Exit codes (`README.md` §Exit Codes): `0` no findings, `1` findings at or above
`--fail-on`, `2` findings all below it. So **exit code 0 is exactly "zero findings"**.
`.github/CODEOWNERS` has no extension and is not in the walker's whole-name list, so it needs
`--all-files` to be scanned at all.

</ground_truth>

<tasks>

<task type="tracer">
  <name>Task 1: SECURITY.md, and prove the whole loop end to end</name>
  <files>SECURITY.md</files>
  <precondition>`gh` is authenticated with admin rights on `UnityInFlow/injection-scanner` — required to enable private vulnerability reporting. Assert with `gh api repos/UnityInFlow/injection-scanner/private-vulnerability-reporting`; if that call 403s, halt and report rather than continuing.</precondition>
  <action>
This is the thin vertical slice: one artifact, taken all the way from repo setting through file
through self-scan through the Rust gate to a committed change. It proves the loop before the other
two tasks widen it. Do it first and do not batch it.

Step A — enable the channel before documenting it. The repo currently has private vulnerability
reporting **disabled**. Writing a SECURITY.md that points at `/security/advisories/new` while the
feature is off sends every reporter to a 404 and pushes them into a public issue with a working
exploit in it. Run:

    gh api --method PUT repos/UnityInFlow/injection-scanner/private-vulnerability-reporting

then re-read `gh api repos/UnityInFlow/injection-scanner/private-vulnerability-reporting` and
confirm it reports enabled. If it does not, **stop and report** — do not write the file, and do not
substitute an email address (none exists; inventing one is out of scope).

Step B — write `SECURITY.md`. It must cover both directions, because this is a security tool that
parses hostile input and also *claims* to detect things.

Required sections:

- **Supported versions.** `v0.0.3` is the current release line and the only supported one. Fixes
  ship as a new tag; there are no backports to `v0.0.1`/`v0.0.2`. State that `main` is ahead of
  `v0.0.3` and is not a supported release. Single maintainer — say so.
- **Reporting a vulnerability in the scanner.** GitHub private vulnerability reporting on this repo
  (`https://github.com/UnityInFlow/injection-scanner/security/advisories/new`). No email address.
  In scope, stated concretely because the scanner reads attacker-controlled files: a panic or crash
  on a crafted input; catastrophic regex backtracking (ReDoS) in a library pattern; path traversal
  or a write outside the scanned tree; memory or CPU exhaustion from a hostile file; anything that
  lets the *content of a scanned file* affect the host running the scan, or affect a CI job's
  outcome beyond the documented exit codes.
- **Detection bypasses are NOT embargoed vulnerabilities.** This is the honest half and it must be
  unambiguous. A payload the scanner misses is a **public issue**, not an advisory. Say plainly
  that the pattern library is known-incomplete and that this is tracked in the open: cite **#80**
  (role-override patterns are near-literal; common synonyms defeat them) and **#81** (nothing
  measures recall — the clean corpus gates false positives, no gate exists for missed attacks), and
  point at `docs/DETECTION-BACKLOG.md`. Give the reason: a bypass that is embargoed is a bypass
  nobody can write a test for. Route these to the pattern-proposal issue form.
- **Also not vulnerabilities.** Inline suppression by an untrusted document is a **documented trust
  boundary**, not a defect — see the "Suppression is a trust boundary" section of `README.md`; the
  scanner's answer is that suppression is never silent and `--no-suppress` exists. A false positive
  is the `false_positive.yml` form, not an advisory.
- **What to expect.** Modest and truthful for one maintainer working on this part-time:
  acknowledgement within 7 days, an initial assessment within 14, best effort thereafter, and no
  guaranteed remediation window. State that there is **no bug bounty and no monetary reward**.
  Offer credit in the advisory and in `CHANGELOG.md` unless the reporter declines.
- **Supply chain.** Release binaries carry a signed SLSA build-provenance attestation; give the
  real verification command (see ground truth) and mention `SHA256SUMS.txt`. Note that a report of
  a *tampered release asset* is in scope and urgent, because `spec-ci-plugin` downloads and
  executes these binaries in other repositories' CI.

Tone: match `.github/PULL_REQUEST_TEMPLATE.md` and `CONTRIBUTING.md` — direct, specific, no
marketing, no "we take security very seriously". Do not invent an SLA, a team, an email alias, a
PGP key, or a bounty.

Write no verbatim injection payload. Refer to attack classes and pattern IDs.

Commit atomically: `docs: add SECURITY.md — a vulnerability channel, and an honest bypass policy`.
Do not stage anything under `.planning/`.
  </action>
  <verify>
    <automated>gh api repos/UnityInFlow/injection-scanner/private-vulnerability-reporting --jq '.enabled' | grep -qx true &amp;&amp; cargo build --release --locked &amp;&amp; ./target/release/injection-scanner check SECURITY.md --all-files &amp;&amp; echo "SELF-SCAN CLEAN (exit 0 = zero findings)"</automated>
  </verify>
  <done>
`gh api …/private-vulnerability-reporting` reports enabled. `SECURITY.md` exists, states v0.0.3 as
the supported line, routes crashes/ReDoS/traversal/exhaustion to the GitHub advisory URL, states in
writing that missed detections are public issues and cites #80, #81 and `docs/DETECTION-BACKLOG.md`,
gives the real `gh attestation verify` command, and contains no email address, no bounty and no SLA
beyond 7-day acknowledgement / 14-day assessment. The scanner returns exit code 0 on it. One commit.
  </done>
</task>

<task type="auto">
  <name>Task 2: CODEOWNERS and the three issue forms</name>
  <files>.github/CODEOWNERS, .github/ISSUE_TEMPLATE/config.yml, .github/ISSUE_TEMPLATE/bug_report.yml, .github/ISSUE_TEMPLATE/pattern_proposal.yml, .github/ISSUE_TEMPLATE/false_positive.yml</files>
  <action>
**`.github/CODEOWNERS`.** Sole owner `@hermanngeorge15`. CODEOWNERS is last-match-wins, so order
broad-to-narrow: a `*` catch-all first, then the dangerous paths. Weight it toward the files where
an unreviewed change is actually dangerous:

- `/.github/workflows/` — a workflow change can exfiltrate the token or ship a different binary.
- `/patterns/` — the detection library; a weakened regex is a silent capability loss.
- `/src/` — the engine.
- `/SECURITY.md` and `/.github/CODEOWNERS` itself.
- `/.github/code-scanning-baseline.json` — an attacker-supplied entry there silences a real alert.
  **This file is not on `main` yet**; it arrives with PR #82 (`feat/cli-04-sarif`), together with
  `.github/workflows/code-scanning.yml`. Listing it now is deliberate forward protection and is
  harmless (a CODEOWNERS pattern matching nothing is not an error) — but say so in a `#` comment so
  the next reader does not think it is a typo. It is also covered by the `/.github/` pattern if you
  include one.

Add a short header comment recording what this file does and does **not** do: it auto-requests
review from the owner; it does **not** block a merge, because the `main` ruleset requires a PR and
the `fmt · clippy · test · build` check but **not** code-owner review. Do not claim enforcement the
repo does not have. Also note GitHub does not request review from a PR's own author, so the
practical value is on fork PRs — which is exactly the contribution path `PATTERNS.md` depends on.

**`.github/ISSUE_TEMPLATE/config.yml`.**

    blank_issues_enabled: false

plus `contact_links`. Route: (a) security vulnerability -> the advisory URL
`https://github.com/UnityInFlow/injection-scanner/security/advisories/new`; (b) the security policy
-> `https://github.com/UnityInFlow/injection-scanner/blob/main/SECURITY.md`; (c) the pattern
contribution guide -> `.../blob/main/PATTERNS.md`. **Do not add a Discussions link** — Discussions
is disabled on this repo (`has_discussions: false`), and a contact link to a disabled feature is a
dead end. Questions go through the bug-report form.

**`bug_report.yml`** — YAML issue *form* (`body:` with `type: input|dropdown|textarea|checkboxes`),
not a markdown template. Fields:

- A leading `type: markdown` warning: **redact the scanned content before pasting it.** Give the
  reason rather than the rule — the file that tripped the scanner is often a real spec or a real
  RAG document, and secret scanning push protection covers commits, not issue bodies. Ask for a
  minimal reconstruction, never the original.
- `version` (input, required): output of `injection-scanner --version`. Required because `main` is
  ahead of `v0.0.3` and half the flags in the README do not exist in the released binary.
- `install_source` (dropdown, required): GitHub Release binary / `cargo install --path .` / built
  from source / via the `spec-ci-plugin` GitHub Action / via the pre-commit hook.
- `platform` (dropdown, required): the six target triples from ground truth, plus "other (say
  which)".
- `command` (textarea, required, `render: shell`): the exact command, verbatim.
- `expected` and `actual` (textareas, both required).
- `repro` (textarea, required): minimal input that reproduces it — redacted.
- A `checkboxes` acknowledgement that the pasted content contains no real credential and no
  confidential document.

**`pattern_proposal.yml`** — the highest-value form, because this is where community contributions
arrive. Fields:

- `type: markdown` intro pointing at `PATTERNS.md` for the format and ID ranges, and at
  `CONTRIBUTING.md` §"Adding a New Pattern".
- `category` (dropdown, required): the five categories with their ID ranges exactly as ground truth
  lists them.
- `severity` (dropdown, required): **the option labels are the PATTERNS.md tests, not bare
  severity words** — so a proposer grades against the criterion instead of picking a mood. Use the
  four rows from the ground-truth table verbatim.
- `evidence` (textarea, required): where this attack shape has actually been seen — an advisory, a
  write-up, an observed agent transcript, a CTF, a public corpus. Say that "an LLM could plausibly
  be tricked by this" is not evidence.
- `false_positives` (textarea, required): **what legitimate documentation would this fire on?**
  Restate the three deciding rules from PATTERNS.md (naming is not carrying; one is an artefact,
  several are intent; if you had to imagine the benign case it is not CRITICAL) and note that
  `tests/corpus/clean/` must stay at zero findings — if the proposal fires there, the grade is not
  the problem.
- `proposed_pattern` (textarea, optional, `render: yaml`): the pattern entry in the
  `patterns/core/*.yaml` shape. Optional on purpose — a good report of an attack shape is worth
  having from someone who does not write regex.
- `checkboxes` (required): at least 3 true-positive cases and at least 2 non-match cases will be
  included, per PATTERNS.md.

**`false_positive.yml`** — fields:

- `pattern_id` (input, required): e.g. `PI0XX`. Do **not** instruct the reporter to run
  `injection-scanner explain` — that subcommand is on `main` and not in `v0.0.3` (Trap 2). The
  pattern ID is printed in the default text output of every released version.
- `version` (input, required) — same reason as the bug form.
- `document_type` (input or dropdown, required): what the document legitimately is — security
  write-up, API reference, prompt-engineering guide, changelog, test fixture, other.
- `repro` (textarea, required): the **minimal** text that fires it, redacted.
- `type: markdown` note: a fix normally means adding the document to `tests/corpus/clean/`, which
  is asserted to produce zero findings by `tests/corpus_test.rs` — so a specimen that can be
  committed publicly is the single most useful thing to attach.

Across all three forms: `name`, `description`, `title` prefix and `labels` set; keep the register
of `.github/PULL_REQUEST_TEMPLATE.md` — terse, concrete, no filler. **No verbatim injection payload
in any placeholder or example.** Use a neutral placeholder where an example payload is tempting.

Commit atomically: `docs: add CODEOWNERS and structured issue intake`. Nothing under `.planning/`.
  </action>
  <verify>
    <automated>python3 -c "import yaml,sys,glob;
files=sorted(glob.glob('.github/ISSUE_TEMPLATE/*.yml'));
assert len(files)==4, files;
docs={f:yaml.safe_load(open(f)) for f in files};
cfg=docs['.github/ISSUE_TEMPLATE/config.yml'];
assert cfg.get('blank_issues_enabled') is False, cfg;
links=[l['url'] for l in cfg.get('contact_links',[])];
assert any('security/advisories/new' in u for u in links), links;
assert all('discussions' not in u.lower() for u in links), links;
forms=[v for k,v in docs.items() if k.endswith(('bug_report.yml','pattern_proposal.yml','false_positive.yml'))];
assert len(forms)==3;
[print(f['name'], len(f['body'])) for f in forms];
assert all(f.get('body') for f in forms);
print('ISSUE FORMS OK')" &amp;&amp; test -s .github/CODEOWNERS &amp;&amp; cargo build --release --locked &amp;&amp; for f in .github/CODEOWNERS .github/ISSUE_TEMPLATE; do ./target/release/injection-scanner check "$f" --all-files || exit 1; done &amp;&amp; echo "SELF-SCAN CLEAN"</automated>
  </verify>
  <done>
`.github/CODEOWNERS` assigns `@hermanngeorge15`, covers workflows, patterns, src, SECURITY.md and
the (not-yet-present, commented) code-scanning baseline, and states in a comment that it requests
review rather than blocking merges. Four YAML files parse; `blank_issues_enabled` is `false`; a
contact link reaches the advisory URL; no link points at Discussions. The pattern form's severity
dropdown carries the PATTERNS.md criteria as its option text and requires a false-positive
analysis. The scanner returns exit code 0 on every new path. One commit.
  </done>
</task>

<task type="auto">
  <name>Task 3: CHANGELOG.md, the release checklist, and the full gate</name>
  <files>CHANGELOG.md, docs/RELEASE-CHECKLIST.md</files>
  <action>
**`CHANGELOG.md`** — Keep a Changelog **1.1.0**, semver, with the standard header paragraph linking
both. Sections in Keep a Changelog order (`Added` / `Changed` / `Fixed` / `Security` — omit any
that is empty; do not emit empty headings).

Dates: use the **`published_at`** values from ground truth (2026-04-02, 2026-06-24, 2026-08-22),
not the STATE.md values, which are off by a day in both directions. STATE.md is the orchestrator's
file — do not edit it here; the discrepancy is reported back instead.

- `## [Unreleased]` — derive **only** from the 18 commits in `git log v0.0.3..main --oneline`
  listed in ground truth. Re-run that command rather than trusting the paste. Fold the `feat:`/
  `fix:` commits into user-visible entries and cite the issue number each already carries
  (`#8`, `#25`, `#21`, `#26`, `#24`, `#23`, `#22`, `#20`, `#19`). Drop pure `docs:`/`chore:`
  commits that changed no behaviour; `test:` commits only if the guarantee is user-facing.
  **Nothing about `--baseline` and nothing about SARIF** — both are on unmerged branches (#79,
  #82). If a commit's user-visible effect is not clear from its subject, read the commit rather
  than guessing, or leave it out.
- `## [0.0.3] - 2026-08-22` — from `git log v0.0.2..v0.0.3`. The substance: case-insensitive
  matching and compile-once scanning (#12, #13), per-file read-error isolation so one bad file
  cannot abort a scan (#14), the three suppression forms and reporting every match (#15, #16, #19),
  rejecting unknown `--format` values instead of falling through to text (#42), duplicate pattern
  id detection / unknown-field rejection / `--strict-patterns` (#28), `--no-suppress` and
  suppression that is never silent, the restored CI test gate, the release pipeline move to
  GitHub-hosted runners with signed SLSA provenance (#45), SHA-pinned actions + dependabot, and the
  published-asset contract gate (#18). Note under `Security` the provenance attestation and the
  `unwrap_used` deny.
- `## [0.0.2] - 2026-06-24` — the six target-triple binaries plus `SHA256SUMS.txt`; the asset
  naming changed from the v0.0.1 shape (`injection-scanner`, `injection-scanner-darwin-arm64`,
  `injection-scanner-linux-x86_64`) to `injection-scanner-<target-triple>`. That rename is the
  single most consumer-relevant fact in the file — record it as `Changed`, and note this is the
  release `spec-ci-plugin` first consumed.
- `## [0.0.1] - 2026-04-02` — initial release: the pattern library across five categories, text and
  JSON output, inline suppression, stdin mode. Keep it short; the tag has seven commits behind it.
- Link reference block at the bottom, exactly:

      [Unreleased]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.3...HEAD
      [0.0.3]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.2...v0.0.3
      [0.0.2]: https://github.com/UnityInFlow/injection-scanner/compare/v0.0.1...v0.0.2
      [0.0.1]: https://github.com/UnityInFlow/injection-scanner/releases/tag/v0.0.1

**`docs/RELEASE-CHECKLIST.md`** — the real tag-to-verified-binaries procedure, read out of
`.github/workflows/release.yml`. Name the actual jobs, targets, assets and commands from ground
truth. Structure:

1. **Pre-tag gate (local).** `cargo fmt --all -- --check`; `cargo clippy --all-targets --locked --
   -D warnings`; `cargo test --locked`; `cargo build --release --locked` — the same four the
   `fmt · clippy · test · build` CI job runs, in that order. Then: bump `version` in `Cargo.toml`
   **and** refresh `Cargo.lock` (run a build so the lock records the new version), and update
   `CHANGELOG.md` — promote `[Unreleased]` to the new version heading with today's date and add the
   new compare link. Explain *why* the version bump is its own checklist item rather than folklore:
   `release.yml`'s `test` job fails the release if `${GITHUB_REF_NAME#v}` does not equal the
   `Cargo.toml` version, and before that guard existed a tag pushed without the bump shipped
   binaries reporting the previous version, with a signed attestation over them, past every gate.
2. **Tag and push.** `git tag vX.Y.Z && git push origin vX.Y.Z`. Trigger is `push` on `v*` only —
   nothing else starts a release, and a fork cannot fire it.
3. **Watch the four jobs, by name**, in dependency order: `test` -> `build-binaries` (6 matrix legs)
   -> `release` -> `verify-published-assets`. State which failures are fatal: any Linux leg is
   fatal; the two `*-apple-darwin` legs are `continue-on-error` and a failure there is a warning
   that defers macOS, not a broken release.
4. **What is published.** The table of six asset names plus `SHA256SUMS.txt` from ground truth.
5. **Post-release verification (manual, on the published Release).** Download the musl x86_64
   asset; `gh attestation verify injection-scanner-x86_64-unknown-linux-musl --repo
   UnityInFlow/injection-scanner`; check it against `SHA256SUMS.txt`; `chmod +x` and run
   `--version`, confirming it prints the tagged version. Note that `verify-published-assets` already
   walks the consumer's exact path automatically — this manual pass is the human confirmation that
   it did, and the place to notice a red job.
6. **Consumer check against `spec-ci-plugin`.** The Action (`04-spec-ci-plugin/src/
   injection-scanner.ts`) downloads the two musl assets at a **pinned tag**, verifies them against
   `SHA256SUMS.txt`, `chmod +x`es and executes them. So after a release: confirm both musl URLs
   return 200 at the new tag, then decide whether `spec-ci-plugin`'s default
   `injection-scanner-version` should move — and record that it currently pins `v0.0.3`. A release
   is not finished until this is answered either way.
7. **Two hard constraints — a "do not change this back" box.**
   - **The musl assets stay raw, unextensioned and target-triple-named.** Not tarballs, no
     extension, exact names. `spec-ci-plugin` curls and executes them directly, so these names are a
     public API of this repository; renaming one is a breaking change that surfaces in *another*
     repo's CI. `tests/release_contract_test.rs` and `verify-published-assets` both defend it and
     they see different failures — see `CONTRIBUTING.md` §"The release asset contract".
   - **`release.yml` runs on `ubuntu-latest` deliberately.** This repo is public and the org runner
     group enforces `allows_public_repositories: false`, so a self-hosted job cannot be scheduled
     here at all — the previous `runs-on: [orangepi]` would queue until cancelled. The pipeline uses
     no org secrets, only the built-in `GITHUB_TOKEN`, and is tag-triggered only. Point at the
     August 2026 row in the root `CLAUDE.md` decisions log and at issue #45. Do not "restore" a
     self-hosted label. Same for `ci.yml`.
8. **Rollback.** A bad release is fixed by a new tag, not by mutating a published one — deleting or
   moving a tag breaks the pinned URL `spec-ci-plugin` fetches and invalidates the attestation
   binding to `refs/tags/<tag>`.

Both files are prose about attacks and releases: **no verbatim injection payload**, or the repo's
own scanner will flag its own changelog.

Then run the full gate (see `<verify>`) and commit atomically:
`docs: add CHANGELOG.md and the real release checklist`. Nothing under `.planning/`.
  </action>
  <verify>
    <automated>cargo fmt --all -- --check &amp;&amp; cargo clippy --all-targets --locked -- -D warnings &amp;&amp; cargo test --locked &amp;&amp; cargo build --release --locked &amp;&amp; for f in SECURITY.md CHANGELOG.md docs/RELEASE-CHECKLIST.md .github/CODEOWNERS .github/ISSUE_TEMPLATE; do ./target/release/injection-scanner check "$f" --all-files || { echo "FINDINGS IN $f"; exit 1; }; done &amp;&amp; grep -q "compare/v0.0.3...HEAD" CHANGELOG.md &amp;&amp; grep -q "2026-08-22" CHANGELOG.md &amp;&amp; grep -q "gh attestation verify" docs/RELEASE-CHECKLIST.md &amp;&amp; grep -q "verify-published-assets" docs/RELEASE-CHECKLIST.md &amp;&amp; echo "FULL GATE GREEN"</automated>
  </verify>
  <done>
`CHANGELOG.md` follows Keep a Changelog 1.1.0 with `[Unreleased]`, `[0.0.3] - 2026-08-22`,
`[0.0.2] - 2026-06-24`, `[0.0.1] - 2026-04-02` and the four compare/release links; every entry
traces to a commit reachable from `main`; nothing about `--baseline` or SARIF appears anywhere.
`docs/RELEASE-CHECKLIST.md` names the four real jobs, the six real target triples, the real asset
names, `SHA256SUMS.txt`, the real `gh attestation verify` command, the pre-tag version-bump gate,
the post-release `spec-ci-plugin` consumer check, and both "do not change this back" constraints.
`cargo fmt`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked` and
`cargo build --release --locked` all pass, and the scanner returns exit code 0 on all five new
paths. One commit.
  </done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|---|---|
| security reporter -> repository | A finder must choose a channel. If the private one is broken, the working exploit lands in a public issue. |
| fork contributor -> `main` | A fork PR can propose changes to workflows, patterns, and (once #82 lands) a code-scanning baseline. |
| issue reporter -> public issue body | Reporters paste the document that tripped the scanner; that document is often a real spec or RAG corpus entry. |
| this repo's release -> `spec-ci-plugin` CI | Asset names and the checksum manifest are executed in another repository. Docs that misdescribe them cause breakage there. |
| new markdown -> this repo's own scanner | Every file added here is subsequently scanned by the tool it documents. |

## STRIDE Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation Plan |
|---|---|---|---|---|---|
| T-hyg-01 | Information disclosure | `SECURITY.md` reporting channel | high | mitigate | Task 1 enables private vulnerability reporting **before** the file links it, and verifies `enabled == true`; the plan halts rather than shipping a doc that points at a disabled channel. |
| T-hyg-02 | Tampering | `patterns/`, `src/`, `.github/workflows/`, future `code-scanning-baseline.json` | medium | mitigate | `.github/CODEOWNERS` auto-requests maintainer review on those paths, including the baseline file before it exists. Residual risk stated honestly in-file: the `main` ruleset does not require code-owner review, so this is attention, not enforcement. |
| T-hyg-03 | Information disclosure | public issue bodies | medium | mitigate | Every form carries a redaction warning plus an explicit no-real-credential acknowledgement; the reason is given (push protection covers commits, not issues) so it is followed rather than clicked past. |
| T-hyg-04 | Spoofing / Repudiation | `CHANGELOG.md` | medium | mitigate | Entries derived only from `git log v0.0.3..main`; `--baseline` (#79) and SARIF (#82) explicitly excluded, so the file cannot claim capability that no released or merged code has. Dates taken from `published_at`, not from prose. |
| T-hyg-05 | Denial of service | this repo's own CI / self-scan | low | mitigate | Every task's `<verify>` scans its own new files with the built binary and requires exit code 0; the plan forbids writing verbatim payloads into new markdown. |
| T-hyg-06 | Tampering | release procedure drift | medium | mitigate | `docs/RELEASE-CHECKLIST.md` is derived from `release.yml` and restates the raw-musl-asset and `ubuntu-latest` constraints with a "do not change this back" pointer to the decisions log and issue #45. |

No package-manager installs are introduced by this plan (no npm / pip / cargo add), so the package
legitimacy gate does not apply.
</threat_model>

<verification>
Run from the repo root on branch `chore/repo-hygiene`:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked

for f in SECURITY.md CHANGELOG.md docs/RELEASE-CHECKLIST.md \
         .github/CODEOWNERS .github/ISSUE_TEMPLATE; do
  ./target/release/injection-scanner check "$f" --all-files \
    || { echo "FINDINGS IN $f"; exit 1; }
done

gh api repos/UnityInFlow/injection-scanner/private-vulnerability-reporting --jq '.enabled'
git log --oneline origin/main..HEAD          # exactly three commits, none touching .planning/
git status --short                            # clean apart from .planning/
```

`.github/workflows/ci.yml` must be untouched — it is the deliberate GitHub-hosted-runner exception.
</verification>

<success_criteria>
- All eight files exist; none existed before.
- Private vulnerability reporting reports `true`.
- Three atomic commits on `chore/repo-hygiene`, none containing `.planning/` paths, none touching
  `.github/workflows/`.
- `cargo fmt` / `cargo clippy --all-targets --locked -- -D warnings` / `cargo test --locked` /
  `cargo build --release --locked` all pass.
- The built scanner returns exit code 0 on every new path.
- The four issue-form YAML files parse; `blank_issues_enabled: false`; no Discussions link.
- `CHANGELOG.md` mentions neither `--baseline` nor SARIF.
</success_criteria>

<output>
Create `.planning/quick/260828-cli-repo-hygiene-for-production-readiness-se/260828-cli-SUMMARY.md` when done.
</output>

