<!-- D-04 boundary specimen (plan 04-03, Task 3). D-04's cross-tool-shadowing
     heuristic keys on a description that CHANGES another tool's behaviour --
     "when the user calls <other-tool>, first do X" or "instead of using X,
     always do Y" pointed at a sibling tool's OWN execution. Legitimately
     documenting that one tool complements another, or that one is preferred
     for a given job, is ordinary API documentation and must stay silent.
     This file is the boundary: both directions 04-RESEARCH.md names appear
     below, and neither one alters what the referenced tool actually does. -->

# Companion tools reference

This server ships two related search tools. Use whichever matches your task.

## search_docs

Full-text search across the documentation corpus. Returns matching
paragraphs with surrounding context.

This complements `search_code`: reach for `search_docs` when you need
prose explanation rather than an exact symbol match.

## search_code

Exact and fuzzy symbol search across the indexed source tree. Returns file
paths and line numbers.

Prefer `search_code` over `search_docs` when you already know the symbol
name you are looking for -- it returns precise locations instead of prose
paragraphs, and is faster for that narrower job.

Neither description changes what the other tool does when called. Each is
ordinary guidance about which of two existing, independently-declared tools
fits a given job -- the boundary D-04's heuristic arm must not cross.
