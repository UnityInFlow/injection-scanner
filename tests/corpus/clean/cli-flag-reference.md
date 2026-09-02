# CLI Flag Reference — Optional Checks and Hooks

This is a real, third-person reference document listing optional flags a command-line
tool exposes. It describes what each flag does; it never instructs anyone, human or
agent, to disable anything. Added under GATE-03 (Plan 07, #33): the third-party sweep
found this exact sentence shape — `disable the <adjective> check` — in a real project's
README (`llm-diff`, describing its `--no-nli` flag) and it false-positived against
PI057's original verb+object-window form. This specimen pins the fix.

## Flags

- `--no-lint` — skips the lint check for this run. Useful in CI when a separate lint
  job already covers the same files.
- `--no-nli` — pass this to disable the contradiction check entirely and fall back to
  whole-output drift comparison only. The NLI cross-encoder model is not downloaded
  when this flag is set.
- `--skip-git-repo-check` — bypasses the repository-detection preflight step. Intended
  for CI environments where the working directory is a checkout without a `.git` folder.
- `--no-hooks` — disables the pre-commit hook installed by `install-hook`, for a single
  invocation, without removing the hook file itself.

## Behavior when a flag is not recognized

An unrecognized flag causes the CLI to print its usage text and exit non-zero rather
than silently ignoring it. This document is reference material for maintainers and
users, not a directive addressed to any reader — every sentence above describes what
the software does, in the third person, never what someone should do to a live
guardrail.
