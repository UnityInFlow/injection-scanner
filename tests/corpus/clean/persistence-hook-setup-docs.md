# Setting up lifecycle hooks

<!-- CAT-03 false-positive specimen (phase 05, plan 05-02). Decision it
     defends: 05-RESEARCH.md section Q4 -- a bare hook-registration prose
     draft fired on a host's own hooks guide and on this repository's own
     CLAUDE.md and PROJECT.md, which is why the prose hook arm plan 05-05
     ships requires an AI addressee. The sentences below are addressed to a
     human developer, or are plain product description.
     Sources, per sentence:
       - the "Runs as a pre-commit hook" sentence and the HOOK-01 line are
         SOURCED, copied from this repository's CLAUDE.md line 7 and
         .planning/PROJECT.md lines 4 and 42.
       - every other sentence is SYNTHESIZED in the register of a host's
         hooks guide. 05-RESEARCH.md quotes that guide's commands, not its
         prose, and it was not re-fetched, so none of these are attributed. -->

## Registering a hook

To run a script whenever a session starts, add a `hooks` block with a SessionStart entry to the `.claude/settings.json` file in your project.

You can also create a hook by running `/hooks` in the editor, which writes the entry into your user settings for you.

Developers who want a formatter to run after every edit can register a PostToolUse hook in their settings file.

## The pre-commit hook

Runs as a pre-commit hook in <200ms.

**HOOK-01**: `injection-scanner install-hook` installs pre-commit hook

Run `injection-scanner install-hook` once per clone to install the pre-commit hook, and remove `.git/hooks/pre-commit` to uninstall it.
