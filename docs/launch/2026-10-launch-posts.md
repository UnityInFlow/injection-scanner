# Launch posts for v0.3.0 (drafts, do not post before the release is live)

Tracking issue #208. Source plan: `docs/reviews/2026-10-24-launch-plan.md`, Phase 5.

Preconditions before posting anything below:

- [ ] v0.3.0 Release exists with six binaries, `SHA256SUMS.txt` and attestation
- [ ] `cargo install injection-scanner` prints `injection-scanner 0.3.0` on a clean machine
- [ ] `brew install unityinflow/tap/injection-scanner` works (formula bumped to 0.3.0)
- [ ] README top on `main` shows the one-liners and the demo GIF renders on GitHub
- [ ] Two free hours after posting to answer comments

Timing: Tuesday to Thursday, 14:00 to 16:00 UTC. HN first, Reddit the next day.

## Show HN

Title (80 chars max on HN):

```
Show HN: Injection-scanner – static scanner for prompt injection in agent config files
```

URL: https://github.com/UnityInFlow/injection-scanner

Text (first comment, posted by the author right after submitting):

```
I built a Rust CLI that scans the files an AI agent reads (CLAUDE.md, skill files, MCP tool
descriptions, RAG documents) for prompt-injection patterns before they reach the model. 84 patterns
in 9 categories: role override, exfiltration, tool and permission abuse, MCP tool-description
poisoning, persistence hooks, encoding tricks, multilingual variants. It parses frontmatter as data,
so a wildcard allowed-tools grant or a hook that downloads and executes is graded structurally, not
by regex. JSON and SARIF output, baselines, inline suppression, a pre-commit hook, about 60 ms on a
40-file repo. `cargo install injection-scanner` or `brew install unityinflow/tap/injection-scanner`.

The number I most want feedback on: on the development corpus it catches 105/115 (91%). On a sealed
held-out set written blind for the newest category it caught 2/12 (17%), and the README leads with
that, because a score against the material a detector was tuned on says how the build went, not how
the detector works. Every miss is a public issue. This is a linter, not a classifier: it catches the
crude and the hidden, and a sentence-shaped payload gets through. I would like to know which real
artifacts it misses on and which documentation it false-positives on.
```

Answers to have ready:

- "Why not an LLM classifier?" Determinism, no network, 60 ms, runs in pre-commit and CI. A
  classifier is a different tool and is on the explicit not-planned list (#111).
- "17% is bad." Yes. It is the honest number for the newest category on unseen material; the
  other eight categories are measured the same way on the development corpus. The held-out set
  was sealed by hash before any pattern in that category existed. Which score would you trust more?
- "Regexes are bypassable." Agreed, and the README says so in line 18. The structural engine
  grades frontmatter and hooks as data, which a paraphrase cannot bypass.
- "Does it work on X?" Linux gnu and musl, macOS, both architectures. No Windows binary (#9).
- "False positive on my docs." Thank, ask for the file, label P1, fix in a v0.3.x within days.

## r/ClaudeAI (post the day after HN)

Title:

```
I wrote a scanner for prompt injection in CLAUDE.md, skills and MCP tool descriptions. It is honest about what it misses
```

Body:

```
If you use Claude Code with skills, hooks or MCP servers, your agent reads a lot of files that
somebody else wrote. injection-scanner is a small Rust CLI that checks those files before the model
does: CLAUDE.md, .claude/skills/*/SKILL.md, MCP tool descriptions, RAG documents. It knows 84
patterns in 9 categories and parses frontmatter as data, so a skill with `allowed-tools: "*"` or a
hook that curls and executes a script is flagged structurally, not by keyword. There is a pre-commit
hook (`injection-scanner install-hook`) and a PostToolUse hook example for Claude Code in
`integrations/claude-code/`.

The numbers: 105/115 (91%) on the corpus it was built against, and 2/12 (17%) on a sealed held-out
set for the newest category. The README leads with the second number on purpose. It is a linter,
not a classifier. What I am looking for from this sub: real CLAUDE.md and skill files it
false-positives on, and real injected files it misses. Both become public issues.

Install: `cargo install injection-scanner` or `brew install unityinflow/tap/injection-scanner`.
Repo: https://github.com/UnityInFlow/injection-scanner
```

Attach `docs/demo/demo.gif`. Cross-post to r/rust only if the HN thread had comments about the
implementation; otherwise skip it.

## Directory submissions (#209)

| Target | Section | Line to add |
|---|---|---|
| `hesreallyhim/awesome-claude-code` | `## Security` | `[injection-scanner](https://github.com/UnityInFlow/injection-scanner) - Static scanner for prompt injection in CLAUDE.md, skills, MCP tool descriptions and RAG documents. Rust CLI with SARIF output and a pre-commit hook.` |
| `ottosulin/awesome-ai-security` | `### Safety Classifiers & Prompt Injection Detection` | `[injection-scanner](https://github.com/UnityInFlow/injection-scanner) - Static, deterministic scanner for prompt-injection patterns in agent configuration and RAG files; publishes development and held-out recall side by side.` |

Read each list's CONTRIBUTING.md first; both want one PR per entry and alphabetical order.
awesome-mcp-servers was evaluated and skipped: it lists MCP servers, this is a CLI.

## Day 7 retro (#210)

Record on #211: stars, forks, crates.io downloads, release downloads, external issues and PRs,
HN points and comment count, Reddit upvotes. Compare with the stop rule in the plan: fewer than 25
stars, no external issue or PR beyond #197 and #200, and no substantive comment means back to
maintenance-only with crates.io and the tap left installable.
