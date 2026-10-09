# False-positive corpus

Realistic documents that **must return zero findings at the default confidence
threshold**. Every file here is modelled on something that actually produced a
false positive, in this repository or in a pattern proposed against it — not on
invented near-misses.

The rule is one file per failure mode, and the header of each file names the
pattern that got it wrong and where. If you are adding a file, it should be
because something misfired, and the header should say what.

This exists because per-pattern negative tests are not enough. `PI048`
(`[A-Za-z0-9+/]{48,}`) shipped in a pull request **with** negative tests and
still produced 3,494 false positives on this project's own documentation: `/` is
a base64 character, so the pattern matched every file path over 48 characters.
Its negatives — `shortToken123`, `abcd`, `not-base64-at-all!!!` — all failed on
*length*, so none of them could have caught a failure of *shape*.

A negative test proves a pattern rejects the case its author thought of. A
corpus proves it survives contact with documents nobody wrote for it.

## CAT-02 (`mcp-*`) — the MCP tool-description boundary family

`mcp-*` files exist to catch D-01's discriminator (second-person, agent-directed
address in a tool `description`) before it is over-widened, not after. Phase 4
plan 04-03 measured, with a temporary matching file, that both `corpus_test`'s
`specimens()` and `pattern_relaxed_control_test`'s `corpus_clean_specimens()`
call `read_dir` once and filter on `is_file` — neither recurses. A subdirectory
(the `tests/corpus/clean/mcp/` layout both `04-RESEARCH.md` and `04-PATTERNS.md`
originally proposed) would sit invisibly outside both gates: a file placed there
that unambiguously matched a shipped pattern left both gates green. **Every
CAT-02 specimen is therefore a flat file sharing the `mcp-` filename prefix,
never a subdirectory.**

A `PI060`+ pattern that fires on any `mcp-*` file is narrowed, not accommodated
by editing the specimen — the standing instruction the rest of this README
already states for every other file here.

| File | Decision it defends |
|---|---|
| `mcp-file-tool-response-docs.md` | PR #152 review — five file-tool response and proxy descriptions that exposed four HIGH PI064 false positives after the third-person destination-grammar widening. All five must stay silent under strict scanning. |
| `mcp-manifest.json` | D-01 (pre-existing, Phase 3) — `config.systemPrompt` is the sharpest already-committed near-miss |
| `mcp-setup-guide.md` | D-01 (pre-existing, Phase 3) — second person addressed to the human reader, not the agent |
| `mcp-server-catalogue.json` | D-01 — one hand-written boundary manifest exercising all four real-world near-miss shapes `04-RESEARCH.md` §Q3 measured (protocol-sequencing MUST-obligation, training-awareness second person, sibling-tool naming, multi-step file-read-then-validate) |
| `mcp-registry-filesystem-tools.md` | D-01 — vendored, real, third-party tool descriptions nobody in this repository wrote |
| `mcp-registry-memory-tools.md` | D-01 — vendored, real, third-party tool descriptions nobody in this repository wrote |
| `mcp-registry-everything-instructions.md` | D-01 — vendored, real, agent-directed MCP `instructions` field content (second-person imperative, addressed to "an LLM or autonomous agent"), the sharpest vendored near-miss for the discriminator |
| `mcp-dev-tooling-setup.json` | D-03 — four ordinary local dev-tooling servers, three installed via an unpinned package-runner argument (`npx -y`, `uvx`) the way `04-RESEARCH.md` §Q2 measured as common rather than exceptional, plus one legitimate TLS (`https://`) endpoint as the transport arm's negative neighbour. MEDIUM severity is not permission to fire on this file — the clean-corpus gate counts a finding at any severity. |
| `mcp-plugin-local-launch.json` | Review #34-r1 finding 1 — the near-misses PI060's install-SOURCE narrowing excludes, taken from real third-party plugin manifests: a relative local script path (`./mcp/server.mjs`), a built-output path (`build/index.js`), and the `mcp-remote` bridge whose second argument is a bare `https://` URL. Widening PI060 to `user/project` shorthand or to generic URLs left the corpus green before this file existed, while firing on two real official manifests — the gate was not holding the property. |
| `mcp-companion-tools.md` | D-04 — two sibling tools whose descriptions legitimately reference each other in both directions `04-RESEARCH.md` names ("this complements that", "prefer X over Y for this job"). Neither description changes what the referenced tool does — the boundary D-04's cross-tool-shadowing heuristic must not cross. |

A fourth CAT-02 specimen, `../documentation/mcp-tool-poisoning-writeup.md`,
lives in the sibling `documentation/` corpus rather than here — it is a
write-up ABOUT MCP tool poisoning, not a manifest, so it is held to that
corpus's two-sided contract instead (zero findings at default, at least one
under `--strict`). It defends the anti-gaming clause: writing about this
attack must not itself be reported as the attack. Its header records that
the strict-mode matches it currently relies on come from patterns that
already ship (`PI015`, `PI028`, `PI029`), not from `PI060`+ — plan 04-07
re-checks it once the full CAT-02 set lands.

## CAT-03 (`persistence-*`) — the persistence and lifecycle-hijack boundary family

`persistence-*` files exist so the false-positive gate is in the tree **before**
the patterns that could fire on it (phase 05, plan 05-02). Every one is a flat
file sharing the `persistence-` prefix, for the same reason the CAT-02 section
above gives: both enumerations read one level deep and skip a directory, so a
specimen in a subdirectory leaves both gates green while proving nothing.

A `PI07x` pattern that fires on any `persistence-*` file is **narrowed, not
accommodated** by editing the specimen. Where a header says a sentence is
synthesized, no third party is quoted for it.

| File | Decision it defends |
|---|---|
| `persistence-shell-install-prose.md` | D-02 / `PI070` / `PI071` — the second-person shell-profile, authorized_keys and crontab sentences that are the commonest lines in any install guide (two sourced from the Homebrew and nvm install pages as quoted in `05-RESEARCH.md` §Q1, two synthesized). `PI070` already excludes `you` for this reason; a persistence-object pattern added later must not undo it. |
| `persistence-git-hook-docs.md` | `05-RESEARCH.md` §Q4 and Pitfall 5 — the Pro Git sentence that is word-for-word how a human is told to enable a hook (the reason no pattern slot is spent on a git-hook prose arm), and the defensive `core.hooksPath=/dev/null` clone idiom from Anthropic's own security plugin. A pattern that flags a security control is the `permissions.deny`-versus-`permissions.allow` failure in a new costume. |
| `persistence-memory-feature-docs.md` | D-04 / `PI074` grading — Claude Code's own memory page says conversation-only instructions can be added to an instruction file to make them persist. The durability cue is present in a legitimate, sourced sentence, which is why a memory-write arm is graded below the band that blocks commits. |
| `persistence-instruction-file-writes.md` | D-04 — agent-directed, legitimate writes into a named instruction file (a GSD workflow's routing line, a skill describing itself as updating `CLAUDE.md`, a tool writing a named section). `05-RESEARCH.md` §Q3 measured that nothing in the grammar separates these from the attack, only the determiner (`the project's`, not `your`) and the absence of a future-reader cue. Two synthesized sections keep that measurement in this repo: a deictic variant of the routing line (cue-less, `the project's`) and two vendor-README lines addressed to the human (`your`). A throwaway cue-less, determiner-blind probe fires on both; see the plan 05-02 SUMMARY. |
| `persistence-file-template-conventions.md` | D-02 — licence-header, copyright-notice, file-template and docstring rules, grammatically identical to self-propagation and differing only in the object. All synthesized. Forces `PI072` to discriminate on the propagated object rather than the addressee. |
| `persistence-hook-setup-docs.md` | `05-RESEARCH.md` §Q4 — hook-registration sentences addressed to a human developer, plus this repository's own `CLAUDE.md` / `PROJECT.md` wording about its pre-commit hook installer. A bare hook-registration draft fired on both, which is why the prose hook arm requires an AI addressee. |
| `persistence-legitimate-hooks-config.json` | D-03 AMENDMENT — the nested wrapper-form hooks document with the ordinary command distribution (interpreter plus script path, plugin-root script, and the official guide's inline commands including a write *into* `~/.claude/`). A binding-only structural rule fires on all 8 of its commands (measured by throwaway probe; see the plan 05-02 SUMMARY), so the sensitive-value half of the conjunction has to separate a secret-path read and a persistence-target write from an ordinary logging append. |
| `persistence-local-hook-endpoint.json` | D-03 AMENDMENT — the flat wrapper convention carrying a loopback `type: http` handler (the official documentation's own example shape, which a remote-endpoint rule must let through by requiring a registrable dotted host) and a plain `curl` post to a chat webhook (which a fetch-and-exec rule must not be widened to cover). |
| `persistence-root-form-hooks-config.json` | D-03 AMENDMENT — the mixed root-and-`hooks` shape (lifecycle event names at the document root *and* a wrapper object), every command benign. Counterpart to the structural attack payload that puts the attack on the root side: reaching the root form is a property of the path shape, not of the file being an attack. |
| `persistence-corporate-audit-endpoint.json` | D-07 ship-or-drop for `PI078` — the nested wrapper convention carrying lifecycle handlers that are HTTP endpoints on **dotted, non-loopback** hosts: a compliance audit endpoint and an OpenTelemetry collector. `05-RESEARCH.md` §Q4 records this as `PI078`'s own known false positive ("corporate audit endpoint fires"), but until this file the only HTTP specimen was a loopback URL, which a dotted-host rule passes by construction — so the gate could not see the one case that decides the pattern. This document and the structural attack payload `04-http-handler-remote-lifecycle-endpoint.md` are the same shape (lifecycle binding, HTTP handler, dotted host, no command, no secret path). If `PI078` fires here it flags a compliance control; if it stays silent it cannot reach payload 04 either. Plan 05-06 must resolve that with evidence, and D-07 permits leaving the id unallocated rather than shipping a rule with no discriminator. |
| `persistence-vendor-installer-prose.md` | `PI071`'s declarative arm, and `PATTERNS.md`'s `#97` rule that HIGH is what `install-hook` blocks commits at. `PI070` keeps install prose out structurally by requiring an agent subject **and a modal**; `PI071` drops the modal and its subject list names products, so a release note has an attack's grammar and differs only by provenance. Five of these sentences were **measured** firing `PI071` at HIGH on the plan 05-03 binary before the declarative arm was narrowed to require a deictic cue between the verb and the object. All synthesized. |
| `persistence-memory-assistant-prompt.md` | `PI075`'s content-free-trigger requirement. The nearest document to a trigger-conditioned memory write that a person would call legitimate is a memory-enabled assistant's own system prompt: operator-written, model-addressed, and the same grammar as the attack ("when the user does X, save Y to your memory"). What separates them is the trigger. An attack needs one that is certain to happen (a bare "yes", "thanks", any confirmation); a real memory instruction waits for a disclosure worth keeping. Without the trigger requirement `PI075` fires HIGH here, and HIGH is what `install-hook` blocks commits at. All synthesized. |
| `persistence-vendor-readme-your-instruction-file.md` | `PI073`'s second conjunct. `05-RESEARCH.md` §Q3 measured a determiner-blind draft firing on the vendor-README shape: a product telling a **human** to add its rules to **their own** instruction file (`your CLAUDE.md`). `persistence-instruction-file-writes.md` holds that shape but with no durability cue, so removing `PI073`'s determiner exclusion left the whole corpus green — the exclusion was asserted, not proven. These three lines carry deictic content, an instruction-file target **and** a durability cue, so the second-person possessive is the only thing keeping `PI073` off them (plan 05-05 measured: determiner-blind mutation fires on all three). The converse is an accepted blind spot: an attack that says `your` is missed for the same reason. All synthesized. |

## Provenance — vendored third-party files (D-06(3))

Every row below was triaged **outside this repository**, in a scratch location, by
scanning the fetched candidate with the release binary at `--min-confidence 0`
before it was vendored. A candidate that reported any finding would not have
been vendored (none did — see the plan 04-03 SUMMARY for the full triage
record). Licence is confirmed **per file, per path, at the vendored commit** —
not inherited from the repository's top-level licence, which
[github.com/modelcontextprotocol/servers](https://github.com/modelcontextprotocol/servers)
itself reports as `NOASSERTION` because the repository is mid-transition from
MIT to Apache-2.0 and the relicensing-consent status is not publicly
enumerable per commit. Both vendored README files carry their own explicit
`## License` section stating MIT for the server they document; that
self-declaration is the licence recorded below, not a repository-wide
inference.

| File | Source repository | Commit SHA | Path at that commit | Licence | Date | Complete / subset |
|---|---|---|---|---|---|---|
| `mcp-registry-filesystem-tools.md` | `github.com/modelcontextprotocol/servers` | `d73f99efbfd40c3aa1b61e88728b3d49fb52608f` | `src/filesystem/README.md` | MIT — per the file's own `## License` section | 2026-09-04 | Subset — the `## API` section (`### Tools` + `### Tool annotations`) only; every tool definition in that section is included whole, none truncated. Setup/usage instructions and the license section itself are omitted. |
| `mcp-registry-memory-tools.md` | `github.com/modelcontextprotocol/servers` | `d73f99efbfd40c3aa1b61e88728b3d49fb52608f` | `src/memory/README.md` | MIT — per the file's own `## License` section | 2026-09-04 | Subset — the `### Tools` section only (all 9 tool definitions, whole). The file's own `### System Prompt` section (a real second-person example prompt) was also triaged and found clean, but is a client-usage example rather than a tool definition and was left out of this task's scope. |
| `mcp-registry-everything-instructions.md` | `github.com/modelcontextprotocol/servers` | `d73f99efbfd40c3aa1b61e88728b3d49fb52608f` | `src/everything/docs/instructions.md` | CC-BY-4.0 — per the repository's top-level `LICENSE`, which states documentation contributions (excluding specifications) are licensed under CC-BY-4.0 unconditionally, independent of the code relicensing-consent ambiguity | 2026-09-04 | Complete — the whole 28-line file |

**Rejected candidates: none.** Every fetched candidate (the three vendored
files above, plus the memory server's `### System Prompt` section, triaged and
found clean but left unvendored for scope reasons rather than a pattern hit)
reported zero matches at `--min-confidence 0` in the scratch triage location.
No candidate tripped a shipped pattern. `@modelcontextprotocol/server-postgres`
— named in `04-RESEARCH.md` §Q2 as observed live on the research machine — no
longer exists in this registry's current `src/` tree at the vendored commit
(archived or moved elsewhere); `server-filesystem` and `server-memory` were
the closest available match to "actually installed and in use" per that same
research. No package was installed for this task; every candidate was fetched
as a file via `curl` from `raw.githubusercontent.com`. `git diff --stat
Cargo.toml Cargo.lock` is empty.
