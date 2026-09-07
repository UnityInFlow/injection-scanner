---
schema_version: 1
open_count: 0
waived_count: 1
fixed_count: 1
total_count: 2
last_updated: 2026-09-07T18:13:18.253Z
---

# Broken Windows Ledger

> Cross-phase defect register. With `workflow.windows_enforce` enabled, `/gsd-ship` blocks while `open_count > 0`.
> Waive with `gsd-tools windows waive <id> "<reason>"` (reason required).
> Mark fixed with `gsd-tools windows fixed <id>`.

| id | phase | kind | file | line | description | status | reason | recorded_at | resolved_at |
|----|-------|------|------|------|-------------|--------|--------|-------------|-------------|
| 1 | 04 | todo | src/frontmatter.rs | 219 | walk() scalar truncation panics on real multi-byte UTF-8 content over MAX_VALUE_LEN (assertion failed: self.is_char_boundary(new_len)); reproduced against ~/.cursor/extensions during 04-01 baseline capture; not fixed (Task 1 required zero source diff) — file a follow-up issue | fixed |  | 2026-09-03T09:08:30.773Z | 2026-09-07T18:10:37.614Z |
| 2 | 04 | todo | tests/corpus/attack/structural/README.md |  | WR-02 (carried over from Phase 3): the Payloads table documents only 1 of tool-permission-abuse/'s 5 corpus files; noted explicitly in the README as still open after the 04-01 directory-layout generalisation | waived | Filed as issue #133 — CAT-01's own corpus doc table backfill, out of scope for a CAT-02-only PR (GATE-04); not fixed here | 2026-09-03T09:08:34.501Z | 2026-09-07T18:13:18.253Z |

````json
[
  {
    "id": 1,
    "kind": "todo",
    "phase": "04",
    "file": "src/frontmatter.rs",
    "line": 219,
    "description": "walk() scalar truncation panics on real multi-byte UTF-8 content over MAX_VALUE_LEN (assertion failed: self.is_char_boundary(new_len)); reproduced against ~/.cursor/extensions during 04-01 baseline capture; not fixed (Task 1 required zero source diff) — file a follow-up issue",
    "status": "fixed",
    "reason": "",
    "recorded_at": "2026-09-03T09:08:30.773Z",
    "resolved_at": "2026-09-07T18:10:37.614Z"
  },
  {
    "id": 2,
    "kind": "todo",
    "phase": "04",
    "file": "tests/corpus/attack/structural/README.md",
    "line": null,
    "description": "WR-02 (carried over from Phase 3): the Payloads table documents only 1 of tool-permission-abuse/'s 5 corpus files; noted explicitly in the README as still open after the 04-01 directory-layout generalisation",
    "status": "waived",
    "reason": "Filed as issue #133 — CAT-01's own corpus doc table backfill, out of scope for a CAT-02-only PR (GATE-04); not fixed here",
    "recorded_at": "2026-09-03T09:08:34.501Z",
    "resolved_at": "2026-09-07T18:13:18.253Z"
  }
]
````
