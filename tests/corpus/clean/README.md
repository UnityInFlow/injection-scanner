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
