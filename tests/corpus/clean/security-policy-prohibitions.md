# Contributing safely to an agent-driven repository

<!-- CR-01 (03-REVIEW.md, this milestone's Phase 3 gap-closure plan): PI053,
     PI056 and PI057 all fired on sentences that PROHIBIT the dangerous
     action instead of instructing it -- "Never run with
     --dangerously-skip-permissions...", "Do not update your settings.json
     to no longer require approval...", "Do not disable the security
     scanner, you will be held responsible...". This is exactly the prose a
     security-conscious CONTRIBUTING.md or security-policy document writes,
     and `install-hook` blocks commits at HIGH -- so the scanner would have
     redlined the CI of the repositories doing the right thing. Every
     sentence below is a prohibition, addressed to a human contributor, in
     the register a real security policy uses; none of them is a directive
     telling an agent to widen its own authority. -->

This document describes what contributors must **never** do when working in
this repository's agent-assisted workflow. It is enforced in code review, not
by the agent itself -- these are rules for the human at the keyboard.

## Running the agent

Never run with `--dangerously-skip-permissions` on a shared machine. That flag
removes the confirmation prompt for every tool call the agent issues, and a
shared machine means the blast radius of one mistake is not yours alone to
bear.

Do not run with `--yolo` in CI. Continuous integration runs unattended and
against untrusted input (fork pull requests, dependency updates); a CI job
with confirmation prompts disabled cannot be reviewed before it acts.

Do not switch the permission mode to `bypassPermissions` under any
circumstances, even temporarily "to get past a flaky prompt." If a prompt is
flaky, file a bug against the harness -- do not disable the control that
makes the prompt meaningful in the first place.

## Editing settings

Do not update your `settings.json` to no longer require approval, no matter
how repetitive the approval step feels during a long refactor. The repetition
is the point: it is the moment a human notices a command that should not run.

Never edit your settings file to add an auto-approve flag for a tool class
you have not audited. An auto-approve entry that looked safe when it was
added can become unsafe the moment that tool gains a new capability, and
nothing will re-prompt you to notice.

## Guardrails and scanners

Do not disable the security scanner, you will be held responsible for any
regression it would have caught. The scanner exists precisely because manual
review misses the class of mistake it is tuned for -- turning it off does not
remove the risk, it just removes the warning.

Never turn off the pre-commit hook, even under deadline pressure. A hook that
can be silenced under pressure is a hook that only fires when it is
convenient, which is another way of saying it never fires when it matters.

## If you believe a control is wrong

Raise it with the maintainers instead of routing around it. A control that is
genuinely wrong should be fixed or removed on purpose, in a change someone
else can review -- not bypassed quietly by whoever finds it inconvenient
first.
