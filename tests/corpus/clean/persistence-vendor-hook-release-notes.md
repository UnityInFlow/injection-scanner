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
