---
task_id: 260828-jkn
status: complete
issue: 80
branch: feat/pi80-role-override-matrix
commit: f236ca3
---

# Summary — role override as a matrix (#80)

**Recall on `tests/corpus/attack/role-override.md`: 1/12 -> 11/12. Clean corpus unmoved at 0.**
Repo total 10/60 -> 20/60.

## What was done

PI001–PI009 widened in place from literal phrases into verb × modifier × object
compositions. Ids, names, severities and concepts preserved. Widened rather than
extended because PI001–PI009 is the entire reserved block for the category.

| Id | Grew to cover |
|---|---|
| PI001 | any nullify verb × any priorness word × any instruction object (`Override: all prior constraints`, `set aside your earlier guidelines`) |
| PI002 | the objectless form — prior context referred to by position (`Disregard everything above`) |
| PI004 | `assume the role of` / `pretend to be` / `roleplay as`, not just `act as` |
| PI005 | `as your new system message`, not just `your new primary directive` |
| PI006 | `forget the persona you were assigned`, not just `forget everything you know` |
| PI007 | second arm — declarative invalidation (`Your previous instructions contained an error`) |
| PI009 | verb-class split — hard verbs bare, soft verbs (`update`) need a config-binding qualifier |

PI003 and PI008 unchanged.

## Decisions worth carrying forward

**Pattern `name` is a consumer contract, not a label.** Six names were renamed for accuracy,
then reverted. `pattern_name` ships in the JSON `spec-ci-plugin` reads, so a rename is a
consumer-visible break. Accuracy of a name is not worth that; the widened concept lives in
`description` instead. The rename also broke `cli_surface_test`, which was the signal.

**The precision gate is broader than the corpus.** `tests/corpus/clean/` is 12 files, which is
thin evidence for a widening this size. A full self-scan was the second check: 51 findings
from the widened patterns, every one in `examples/`, `patterns/`, `tests/` or
`tools/injection-lab/`, and none in README.md, PATTERNS.md, CLAUDE.md, `src/` or `docs/`
prose. Worth doing on any future pattern widening — the corpus cannot cover what nobody
wrote a specimen for.

**Two vocabulary exclusions are load-bearing and easy to lose.** `old` and `legacy` are not
priorness words, and the object noun is required wherever the verb is ordinary English. Both
exist because of specific lines in `clean/agent-spec.md` and `clean/hard-wrapped-prose.md`.
Anyone re-tuning these patterns will be tempted to add them; the yaml header says why not.

**`update your instructions` is the FP that nearly shipped.** PI009 is HIGH, which is what
`install-hook` writes as the default fail threshold, so a false positive there blocks
commits. Splitting the verb list by benignness was the fix — soft verbs need a qualifier.

## Deliberately out of scope

`exfiltration`, `instruction-injection` and `jailbreak` are still 0/12 with the identical
defect. Same fix, three more files, one PR each — a single PR widening all four categories
would have had an unreviewable false-positive blast radius.

The one role-override payload still missed is an instruction-precedence claim
("...take precedence"), which belongs to the PI010–PI019 block, not here.
