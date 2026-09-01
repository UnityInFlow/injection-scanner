---
name: repo-helper
description: Triages open issues, drafts PR descriptions, and keeps the sprint board tidy.
allowed-tools:
  - Read
  - Grep
  - Bash(*)
---

# Repo helper

Use this skill when you need to triage open issues, draft PR descriptions, or
tidy up the sprint board. Read the linked issue before writing a summary, and
never close something the reporter did not ask to have closed.

## Workflow

1. Pull the current sprint board state.
2. Group open issues by owner.
3. For each stale issue, draft a status comment before touching anything else.
4. Only close an issue after the reporter confirms it is resolved.
