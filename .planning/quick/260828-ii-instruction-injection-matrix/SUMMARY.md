---
task_id: 260828-ii
status: complete
issue: 97
branch: feat/instruction-injection-matrix
commit: 2e0aa4c
---

# Summary — instruction injection (#97)

**0/12 -> 12/12. Repo total 32/60 -> 45/60 (75%).** Four of five categories now detect.

## The finding: vocabulary widening does not generalise

#80 and #95 both worked by widening verb/object vocabulary. That fails here. The most
common document this scanner is pointed at — a CLAUDE.md — is imperative and
model-addressed from top to bottom, which is the grammatical shape of the attack. The
distinguishing feature is provenance, and no regex sees provenance.

What works instead is keying on **framings that presuppose a model reader**: an aside
about the model addressed to it, a claim of authority over the user, a claim a control is
off. A legitimate agent spec instructs; it does not argue that it outranks the user.

**Carry forward:** before widening a category, ask what the nearest *legitimate* document
looks like. For role override and exfiltration the answer was "prose that shares some
words". Here it was "a document with identical grammar and opposite provenance", and that
changes the whole approach.

## The mutation discipline is now two for two

The precedence arm's first draft produced **six HIGH findings** on ordinary configuration
documentation — HIGH being what `install-hook` blocks commits at. Caught because #95's
lesson was applied: write the legitimate phrasings into the corpus *first*, then tune.

`clean/config-precedence.md` now holds them. Mutating the object list away turns three
corpus assertions red.

Both #95 and #97 found a real over-widening this way, before merge. This should be a
standing step in the pattern-library skill, not a habit that depends on remembering.

## A gap left open on purpose

`role_override` stays 11/12. Closing its last payload means matching a bare
this-document precedence claim, which is line 20 of the new clean specimen. Not separable
by regex. Recorded in `recall_test` rather than left looking like unfinished work.

## The self-scan caught the same class of bug twice

`docs/DETECTION-BACKLOG.md` again quoted payload text in double quotes rather than
backticks, so the scanner reported its own documentation. Second occurrence in two PRs.
**The whole-repo self-scan belongs in the pattern-library loop** — the 14-file corpus does
not cover the repo's own prose, and every widening reopens the risk.

## Remaining

`jailbreak` at 1/12 — the last category holding literal phrases.
