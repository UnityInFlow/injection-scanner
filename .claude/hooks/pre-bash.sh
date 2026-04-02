#!/bin/bash
COMMAND="$CLAUDE_TOOL_INPUT_COMMAND"

# Block force-push
if echo "$COMMAND" | grep -qE "git push --force|git push -f"; then
  echo "ERROR: Force push is not allowed. Use --force-with-lease and confirm with user." >&2
  exit 1
fi

# Block dropping databases
if echo "$COMMAND" | grep -qiE "drop (database|schema|table)"; then
  echo "ERROR: Dropping databases requires human confirmation." >&2
  exit 1
fi

# Block rm -rf on important directories
if echo "$COMMAND" | grep -qE "rm -rf /|rm -rf ~|rm -rf \."; then
  echo "ERROR: Recursive delete on root/home/cwd is not allowed." >&2
  exit 1
fi

# Block cargo publish without explicit confirmation
if echo "$COMMAND" | grep -q "cargo publish" && ! echo "$COMMAND" | grep -q "\-\-dry-run"; then
  echo "ERROR: cargo publish requires explicit user confirmation. Use --dry-run first." >&2
  exit 1
fi
