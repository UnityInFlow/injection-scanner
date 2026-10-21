#!/bin/bash
cd "$CLAUDE_PROJECT_DIR" || exit 0

# 0. Skip the whole gate when none of its inputs changed.
#
# This hook runs after EVERY turn. Before #191 landed, `cargo test` took longer
# than this hook's own 900s timeout (11m02s locally, 18.1 min on CI), so a
# session that never touched Rust spent fifteen minutes stalling and was then
# killed without reporting anything. #191 cut that to 3m01s locally by caching
# the Scanner per test binary, so the gate now completes -- but three minutes
# of recompiling after a turn that edited a Markdown file is still three
# minutes spent to learn nothing.
#
# So gate on whether anything fmt, clippy or the tests actually read has
# changed: the working tree first, then commits not yet on origin/main, so a
# feature branch is still checked after its work is committed.
WATCHED=('*.rs' 'patterns/' 'tests/' 'Cargo.toml' 'Cargo.lock' 'build.rs' 'rust-toolchain.toml')

DIRTY=$(git status --porcelain -- "${WATCHED[@]}" 2>/dev/null)
UNMERGED=""
if git rev-parse --verify --quiet origin/main >/dev/null 2>&1; then
  UNMERGED=$(git diff --name-only origin/main...HEAD -- "${WATCHED[@]}" 2>/dev/null)
fi

if [ -z "$DIRTY" ] && [ -z "$UNMERGED" ]; then
  exit 0
fi

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
