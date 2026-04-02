#!/bin/bash
cd "$CLAUDE_PROJECT_DIR" || exit 0

# 1. Run tests silently — surface errors only
TEST_OUTPUT=$(cargo test 2>&1)
if [ $? -ne 0 ]; then
  echo "Tests failed:" >&2
  echo "$TEST_OUTPUT" >&2
  exit 2
fi

# 2. Clippy lint
CLIPPY_OUTPUT=$(cargo clippy -- -D warnings 2>&1)
if [ $? -ne 0 ]; then
  echo "Clippy errors:" >&2
  echo "$CLIPPY_OUTPUT" >&2
  exit 2
fi

# 3. Format check
FMT_OUTPUT=$(cargo fmt --check 2>&1)
if [ $? -ne 0 ]; then
  echo "Formatting issues (run cargo fmt):" >&2
  echo "$FMT_OUTPUT" >&2
  exit 2
fi

# 4. Verify PR artifact checklist on feature branches
BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null)
if [[ "$BRANCH" == feat/* ]] || [[ "$BRANCH" == fix/* ]]; then
  ISSUE_LINKED=$(git log origin/main..HEAD --format="%s %b" 2>/dev/null | grep -cE "#[0-9]+")
  if [ "$ISSUE_LINKED" -eq 0 ]; then
    echo "No GitHub issue linked in commit history. Add before finishing." >&2
    exit 2
  fi
fi

# SUCCESS: completely silent
