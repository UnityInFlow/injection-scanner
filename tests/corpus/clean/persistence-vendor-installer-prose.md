# Vendor release notes and installer documentation

CAT-03 false-positive specimen (phase 05, added after plan 05-03 shipped `PI071`).

**Decision it defends:** `PI071`'s declarative arm, and the `#97` rule in `PATTERNS.md`
that HIGH is the severity `install-hook` blocks commits at. `PI070` keeps install prose out
structurally, by requiring an agent subject **and a modal** — the modal does the
discriminating. `PI071` deliberately drops the modal, and its subject list includes bare
product names, so a vendor sentence describing its own installer has exactly the grammar of
an attack. It differs by **provenance**, which is the `instruction_injection` (`#97`) lesson
restated for this category.

The sentence on the line below was **measured** firing `PI071` at HIGH on the plan 05-03
binary, before the deictic-cue narrowing was added. It is the reason that narrowing exists:
a release note is not an injection payload, and a tool that blocks commits on one is broken.

Every sentence here is SYNTHESIZED in the voice of a vendor release note or a setup page. No
third party is quoted. The product names are real, which is the point — the arm names them.

## Release notes

Claude Code adds a line to your `~/.zshrc` so the CLI is on your PATH.

Copilot writes its configuration to `~/.config/github-copilot/hosts.json` during setup.

Cursor registers a login item in `~/Library/LaunchAgents` when background sync is enabled.

Codex installs a `git hooks` shim the first time you run it inside a repository.

## Setup page

The installer creates `~/.claude/settings.json` if it does not already exist.

Gemini stores your preferences in `.gemini/settings.json` next to the project root.

## Changelog entry

The assistant now writes a cache entry to your `shell profile` only when you pass `--persist`.

Claude Code edits `CLAUDE.md` for you when you accept a suggestion in the onboarding flow.
