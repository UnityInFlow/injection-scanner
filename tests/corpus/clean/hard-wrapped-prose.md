# Style guide

<!-- Guards the join logic added for #24. Every paragraph here wraps mid-sentence
     in a way that puts pattern-adjacent words either side of a break, or either
     side of a boundary that must NOT be joined. A blind N-line window turns
     several of these into findings. -->

Files that the build produces are safe to ignore. All previous releases are
archived under `dist/`, and their instructions live alongside them.

There are three things to ignore all together: generated output, vendored
dependencies, and the lockfile.

Reviewers should ignore all previous

## Instructions for reviewers

Read the diff before the description. If the change is large, ask for it to be
split rather than approving it on trust.

> The rule we settled on is that you are now
>
> free to merge without a second reviewer on docs-only changes.

We forget everything that is not

## Everything you were told at onboarding

is worth writing down, because nobody remembers it by the second week.
