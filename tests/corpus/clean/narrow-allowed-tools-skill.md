---
name: docs-reviewer
description: Reviews documentation changes for consistency and broken links.
allowed-tools:
  - Read
  - Grep
  - Glob
  - Bash(npm test)
---

<!-- D-06(1): the structural control a CAT-01 `scope: frontmatter` pattern
     must stay off. This comment lives AFTER the closing fence, not before
     the opening one, because `frontmatter::extract_delimited`
     (src/frontmatter.rs:137-159) requires `---` to be the document's literal
     first line -- a leading comment here would make `extract()` return
     `None`, the structural pass would never run on this file, and the
     control would pass by being invisible rather than by being narrow. That
     is the same "test that measured nothing" failure this milestone treats
     as blocking.

     The grant above is real and NARROW: a block sequence of read-oriented
     tools (`Read`, `Grep`, `Glob`) plus one path/command-scoped `Bash`
     grant, never a wildcard. It exists to prove a wildcard-grant rule fires
     because the VALUE at some `allowed-tools[N]` is `*` or `Bash(*)`, not
     because the KEY is `allowed-tools`. It is also written as the block
     sequence real Claude Code skills actually use, which projects as
     indexed array entries (`allowed-tools[0] = Read`, ...) rather than the
     single scalar line a pattern tested only against `allowed-tools: "*"`
     would expect. -->

# Docs Reviewer

You are the documentation reviewer for this repository. Read the changed
files under `docs/` and confirm every heading has a matching entry in the
table of contents. Flag any relative link that points at a path which no
longer exists.

Run `npm test` to confirm the documentation's code examples still compile
against the current API. Report findings as a checklist; do not modify any
file outside `docs/`.
