---
task_id: 260828-pw5
status: complete
issue: 95
branch: feat/exfiltration-matrix
commit: 6cdfc9e
---

# Summary — exfiltration as a matrix (#95)

**0/12 -> 12/12. Repo total 20/60 -> 32/60 (53%).** Clean corpus stayed green and got stricter.

## The finding worth carrying forward

**Mutation-testing the false-positive control showed the gate was not holding it.**

The disclosure patterns rely on requiring the possessive (`your system prompt`, not `the
system prompt`). Relaxing PI021 to `(?:your|the)` — the exact over-widening the design
warns against — left the **entire clean corpus green**. `mcp-manifest.json`'s "Returns the
system prompt currently configured" survives only because the plural in "Returns" fails to
match the verb `return`. An accident, not the control.

So the corpus was extended (`clean/prompt-tooling-docs.md`) until it actually caught the
mutation. This is the opposite of the forbidden move: not editing the corpus so a pattern
passes, but adding a specimen so an over-wide pattern fails.

**Do this on every future widening.** Assert the FP control, then mutate it and confirm
something red. Green tests after a widening are the weakest evidence in this repo — noted
before in the 260825 CLI-08 session, now with a concrete instance.

The new specimen paid for itself immediately: PI022's tool arm fired on "list every tool
the agent is configured with, along with their descriptions", which is MCP documentation.
Second person (`tools you have access to`) is the discriminator. Pattern narrowed.

## Second finding — a test that pinned an accident

`baseline_test::an_occurrence_beyond_count_is_still_reported` asserted `matches.len() == 1`.
That encoded "exactly one pattern matches this fixture line" as if it were the invariant.
Widening PI021 made it two, and the test failed while the baseline behaviour it guards was
untouched. Rewritten to assert the occurrence split by line number, which is the actual
property. **A test that counts findings will break on every pattern-library change; a test
that checks which line they land on will not.**

## Third — the scanner flagged its own documentation again

`docs/DETECTION-BACKLOG.md` listed credential paths in double quotes, so PI029's new path
arm reported them at CRITICAL in prose context. Fixed with backticks — the remediation the
tool itself prints. The original 2026-08 audit listed "the scanner flags its own
documentation" as a finding; widening patterns reopens that risk every time, so the
whole-repo self-scan belongs in the loop, not just the 13-file corpus.

## Remaining

`instruction-injection` and `jailbreak` are still 0/12. Same defect, same fix, two files.
