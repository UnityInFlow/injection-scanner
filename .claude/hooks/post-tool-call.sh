#!/bin/bash
cd "$CLAUDE_PROJECT_DIR" || exit 0

# Rust: compile check + clippy — silent on success, surfaces errors only
OUTPUT=$(cargo clippy -- -D warnings 2>&1)
if [ $? -ne 0 ]; then
  echo "Clippy errors:" >&2
  echo "$OUTPUT" >&2
  exit 2
fi

# SUCCESS: completely silent — nothing added to context
