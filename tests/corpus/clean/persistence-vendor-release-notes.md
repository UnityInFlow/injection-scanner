# Release notes — Assistant Desktop 4.2

Added 2026-10-09 for BL-01 / issue #183. Every sentence here is ordinary vendor
product documentation describing what the product does to its own configuration.
The grammar is a third-person AI subject, the modal `will`, a write verb and a
persistence object — the same shape as an agent-directed persistence
instruction. The two differ by provenance, which a regex cannot see, so this
file is the gate that keeps `PI070` from grading release notes HIGH. HIGH is the
tier `install-hook` blocks commits at, so a regression here blocks contributors'
commits on their own changelogs.

## Settings and preferences

Claude will save your choice to `.claude/settings.json` so the preference
persists between sessions. Claude Code will write the permission to
`.claude/settings.local.json` when you approve a command, and the assistant will
add the selected model to your `copilot-instructions.md` if you opt in.

Gemini will write these preferences to `GEMINI.md` for future sessions. The
model will add the entry to `MEMORY.md` when you ask it to remember something,
and the assistant will store the preference in its memory for the rest of the
session.

## Shell and startup integration

If you enable shell completion, the installer will append the export to
`~/.config/fish/config.fish`, and on zsh it will write your preference to
`~/.zshrc` instead. The agent will put the new rules in `CLAUDE.md` after you
run the initialiser.

## Scheduled work

The assistant will register a launch agent so the helper starts at login. On
Linux the service will be enabled with `systemctl --user` so it survives
reboots; on Windows the assistant will create a startup folder shortcut
instead. The updater will schedule a task with `schtasks` that runs daily.

## Repository hooks

On first run the CLI will add a git hook to `.git/hooks/pre-commit` so the
formatter runs before each commit. You can remove it at any time from the
settings panel.
