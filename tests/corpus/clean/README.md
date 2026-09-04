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
| `mcp-manifest.json` | D-01 (pre-existing, Phase 3) — `config.systemPrompt` is the sharpest already-committed near-miss |
| `mcp-setup-guide.md` | D-01 (pre-existing, Phase 3) — second person addressed to the human reader, not the agent |
| `mcp-server-catalogue.json` | D-01 — one hand-written boundary manifest exercising all four real-world near-miss shapes `04-RESEARCH.md` §Q3 measured (protocol-sequencing MUST-obligation, training-awareness second person, sibling-tool naming, multi-step file-read-then-validate) |
| `mcp-registry-filesystem-tools.md` | D-01 — vendored, real, third-party tool descriptions nobody in this repository wrote |
| `mcp-registry-memory-tools.md` | D-01 — vendored, real, third-party tool descriptions nobody in this repository wrote |
| `mcp-registry-everything-instructions.md` | D-01 — vendored, real, agent-directed MCP `instructions` field content (second-person imperative, addressed to "an LLM or autonomous agent"), the sharpest vendored near-miss for the discriminator |
| `mcp-dev-tooling-setup.json` | D-03 — four ordinary local dev-tooling servers, three installed via an unpinned package-runner argument (`npx -y`, `uvx`) the way `04-RESEARCH.md` §Q2 measured as common rather than exceptional, plus one legitimate TLS (`https://`) endpoint as the transport arm's negative neighbour. MEDIUM severity is not permission to fire on this file — the clean-corpus gate counts a finding at any severity. |
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
