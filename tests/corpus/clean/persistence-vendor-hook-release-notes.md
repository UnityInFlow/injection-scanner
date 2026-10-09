# Vendor release notes that describe an agent registering a hook

<!-- CAT-03 false-positive specimen (phase 05, plan 05-05). Decision it
     defends: PI076's declarative arm requires a cue pointing at content in the
     document (below / above / following / preceding) between the verb and the
     object. An AI-product subject, a registration verb and a hook object are
     the grammar of an attack AND of a vendor release note describing what its
     own installer does; PI076 inherits the category default (HIGH), the tier
     install-hook blocks commits at, so it cannot fire on these lines. This is
     the same lesson as persistence-vendor-installer-prose.md, which was learned
     the hard way for PI071 (HIGH on five ordinary vendor sentences), applied
     before the fact instead of after it. Narrow the pattern, never this file.
     Provenance: SYNTHESIZED in the voice of a vendor release note or a setup
     page, not sourced. No third party is quoted. The product names are real,
     which is the point -- the arm names them. -->

## Release notes

Claude Code adds a PostToolUse hook to your `.claude/settings.json` when you enable automatic formatting.

The assistant registers a SessionStart hook that runs the project's setup script the first time you open a folder.

Copilot writes a hooks definition to `.github/hooks` during onboarding so the cloud agent can run your checks.

## Second-person targets (deferred item 9b)

These four were measured firing `PI071` — and the second one `PI076` as well — at HIGH on the
wave-5 tree, before both declarative arms adopted `PI073`'s determiner allow-list. A deictic cue
alone did not exclude them: three of the four carry one. The discriminator is the second-person
possessive determiner on the target.

Claude Code adds the line below to your `~/.zshrc` so the CLI is on your PATH.

Gemini writes the block below into your `.gemini/settings.json` at first run.

Claude Code writes the rules below to your `CLAUDE.md` so future sessions follow them.

Claude Code adds this line to your `~/.zshrc` during setup.
