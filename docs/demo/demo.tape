# injection-scanner demo tape (issue #206). Produces docs/demo/demo.gif.
#
# Re-record from the project root after `cargo build --release --locked`:
#
#   DEMO_DIR=/path/to/scratch-dir vhs docs/demo/demo.tape
#
# DEMO_DIR must be a throwaway directory (default: /tmp/injection-scanner-demo);
# the hidden setup block below copies docs/demo/skill.md into it, runs `git init`
# there, removes any previous index and pre-commit hook so the recording is
# repeatable, and prepends target/release to PATH so the commands read as a
# plain `injection-scanner ...`. The recording never runs inside this repo: a
# root scan here exits 1 because the pattern catalogue matches itself (#171).
# `core.hooksPath .git/hooks` is set only so that install-hook reports the hook
# as ./.git/hooks/pre-commit instead of a long machine-specific absolute path.
#
# docs/demo/skill.md is a plausible skill file with one planted PI001 line taken
# from tests/corpus/attack/role-override.md.

Output docs/demo/demo.gif

Require git

Set Shell bash
Set Width 1000
Set Height 600
Set FontSize 16
Set Padding 20
Set Theme "Catppuccin Mocha"
Set TypingSpeed 40ms
Set Framerate 24

Hide
Type@1ms `export PATH="$PWD/target/release:$PATH"; D="${DEMO_DIR:-/tmp/injection-scanner-demo}"; mkdir -p "$D" && cp docs/demo/skill.md "$D/skill.md" && cd "$D" && git init -q && git config user.name "Demo User" && git config user.email "demo@example.com" && git config core.hooksPath .git/hooks && rm -f .git/index .git/hooks/pre-commit && clear`
Enter
Sleep 500ms
Show

Type "cat skill.md"
Sleep 400ms
Enter
Sleep 3500ms

Type "injection-scanner check skill.md"
Sleep 400ms
Enter
Sleep 4s

Type "injection-scanner explain PI001"
Sleep 400ms
Enter
Sleep 4500ms

Type "injection-scanner install-hook && git add -A && git commit -m 'add skill'"
Sleep 400ms
Enter
Sleep 5s
