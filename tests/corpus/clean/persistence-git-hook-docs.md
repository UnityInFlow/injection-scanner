# Git hooks

<!-- CAT-03 false-positive specimen (phase 05, plan 05-02). Decision it
     defends: 05-RESEARCH.md section Q4 (no pattern slot is spent on a
     dedicated git-hook prose arm) and Pitfall 5 (a defensive hooksPath
     override is a security control, not an attack).
     Sources, per sentence:
       - the Pro Git sentence is SOURCED, copied as quoted in 05-RESEARCH.md
         section Q1 (fetched there from
         git-scm.com/book/en/v2/Customizing-Git-Git-Hooks); not re-fetched.
       - the clone command is SOURCED, copied as quoted in 05-RESEARCH.md
         section Q1 and verified on disk at
         claude-plugins-official/plugins/claude-security/skills/claude-security/jobs/suggest-patches.md
         line 66 (the plugin's patch-suggestion job).
       - the team-convention sentence is SYNTHESIZED. -->

## Enabling a hook

To enable a hook script, put a file in the hooks subdirectory of your .git directory that is named appropriately (without any extension) and is executable.

Teams that want every clone to share the same hooks usually commit them to a `hooks/` directory in the repository and point `core.hooksPath` at it once per clone.

## Switching hooks off

The patch-suggestion job clones into a scratch directory with its hooks disabled, so that no user hook fires for any command run there:

```sh
GIT_TERMINAL_PROMPT=0 git clone --shared --no-checkout --quiet -c core.hooksPath=/dev/null <repo root> <patch dir>/scratch-<id>
```
