# CAT-03 held-out payload set — v0.2.0's published recall number

**Status:** sealed and verified 2026-10-09, before any `PI071`+ pattern existed.
**Commitment recorded at:** this file, plan 05-07 Task 1, and STATE.md's `stopped_at`.

## Why it exists

GATE-01's bar is that corpus payloads are never derived from the patterns. An external review of
waves 1-2 established that the twelve development payloads clear that bar **lexically** —
`tools/corpus-derivation-check.py` reports 0 collisions and was seen to fail on planted lifts —
but not **structurally**: `05-01-PLAN.md` lines 273-302 restate each payload's genre, grammar,
host convention and expected hit-or-miss, which is the same information as `05-RESEARCH.md` §Q1's
seed table that was deliberately barred from the authoring agent's context. Barring the research
section did not achieve blindness, because the plan carried the shapes.

Two facts kept this from being fatal, and both were checked rather than assumed:

- The seeds were written **before any prototype regex** and measured against the shipping library,
  so they are threat-model-derived at root, not pattern-derived.
- The five `#35` backlog bullets largely determine the shapes anyway — bullet 3 simply *is*
  shell-profile / cron / launchd — so re-authoring "blind" would likely reproduce them, buying the
  appearance of independence rather than the fact of it.

So the twelve payloads are kept as the **development corpus**, and independence is established by
measurement instead of by provenance argument: a genuinely blind held-out set, authored before the
patterns, opened only after they are frozen.

## What the set is

12 payloads — 8 prose across 8 document genres, 4 structural files covering Claude Code, Cursor,
Copilot and Gemini CLI — with all five `#35` mechanisms represented at least twice.

Authored in read-only mode in an empty directory by an agent whose only input was a
mechanism-level paraphrase of `#35` carrying no quoted trigger phrases. It ran no shell commands.
The set passes `tools/corpus-derivation-check.py` against 13 barred sources: `05-RESEARCH.md`,
`05-01-PLAN.md`, `05-CONTEXT.md`, the development attack corpus, and every `patterns/core/*.yaml`.
One payload was rewritten once, by the same blind author, to clear a coincidental 5-gram.

## Location and seal

Outside the repository, read-only:

    $HOME/.local/share/unityinflow/injection-scanner/heldout-v0.2.0-cat03/

**`SHA256SUMS` manifest hash (the seal):**

    6979b4e87bd637dd377e1bebef711420db99cc2df3869dddfdd03cda27b046bd

Verified by this phase's orchestrator with `shasum -a 256 SHA256SUMS` at the time of recording.
The orchestrator listed filenames and hashed the manifest **only**, and deliberately did not read
`prose.md` or `structural/` — it writes the wave 3-6 executor briefs, and reading those shapes
would leak them into the pattern work and destroy the set's value.

## Binding rules

1. **Waves 3 through 6 must not open that directory.** Pattern work never sees it. This is stated
   as an explicit prohibition in every executor brief for those waves.
2. **The seal is recorded before the patterns exist**, so the set cannot be quietly swapped for an
   easier one after the fact. That is the whole point of writing the hash down here.
3. **Plan 05-07** verifies `SHA256SUMS` against the hash above, re-runs the derivation check
   against the **final** `patterns/core/*.yaml`, imports the set as a **separate** held-out recall
   row, and publishes held-out recall as the v0.2.0 CAT-03 number with the development-corpus score
   reported alongside it.
4. **Held-out misses are reported, not tuned away, in v0.2.0.** A pattern edited to catch a
   held-out payload after the set is opened converts the held-out set into a second development
   corpus and destroys the only independent measurement this phase has. Misses become backlog
   items for the next milestone.
