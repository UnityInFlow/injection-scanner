---
task_id: 260828-ii
issue: 97
branch: feat/instruction-injection-matrix
status: complete
---

# Quick task — instruction injection keys on authority, not mood (#97)

Third application of the #80 rewrite, and the first where widening vocabulary was the
wrong tool.

## Why this one is different

`clean/agent-spec.md` is a CLAUDE.md: imperative throughout, addressed to a model. An
injection payload has the same grammatical shape. The difference is **provenance**, which
a regex cannot see — so the pattern has to key on framings only an untrusted document
would use, not on the imperative mood every legitimate agent spec is written in.

Three such framings:
- an aside *about* the model, addressed to it ("Note to the assistant:")
- a claim of authority over the user or the conversation ("takes precedence over")
- a claim that a control is off ("verification has been disabled")

## Gate additions

`clean/config-precedence.md` — because "X takes precedence over Y" is the most common
sentence in configuration documentation and the corpus never said so.
