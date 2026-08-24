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
