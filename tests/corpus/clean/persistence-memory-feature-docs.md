# How the assistant remembers things

<!-- CAT-03 false-positive specimen (phase 05, plan 05-02). Decision it
     defends: D-04 and the PI074 grading (a memory-write arm is graded below
     the band that blocks commits). Legitimate product documentation says
     that conversation-only instructions can be added to an instruction file
     to make them persist, and what happens when a user asks the assistant to
     remember a preference. The durability cue (persist, remember) is present
     in a legitimate sentence, so a pattern keyed on the cue alone is wrong.
     Sources: every quoted sentence below is SOURCED, copied as quoted in
     05-RESEARCH.md section Q1 (fetched there from
     code.claude.com/docs/en/memory); not re-fetched for this plan. The
     research notes elide the middle of the third sentence, so it is kept as
     the two fragments the notes preserve and is NOT reconstructed here. -->

## Persisting an instruction

To add instructions to CLAUDE.md instead, ask Claude directly, like "add this to CLAUDE.md," or edit the file yourself via /memory.

Add conversation-only instructions to CLAUDE.md to make them persist.

## Auto memory

When you ask Claude to remember something, like "always use pnpm, not npm" ... Claude saves it to auto memory.
