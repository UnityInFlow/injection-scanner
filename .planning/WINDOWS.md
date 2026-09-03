---
schema_version: 1
open_count: 2
waived_count: 0
fixed_count: 0
total_count: 2
last_updated: 2026-09-03T09:08:34.501Z
---

# Broken Windows Ledger

> Cross-phase defect register. With `workflow.windows_enforce` enabled, `/gsd-ship` blocks while `open_count > 0`.
> Waive with `gsd-tools windows waive <id> "<reason>"` (reason required).
> Mark fixed with `gsd-tools windows fixed <id>`.

| id | phase | kind | file | line | description | status | reason | recorded_at | resolved_at |
|----|-------|------|------|------|-------------|--------|--------|-------------|-------------|
| 1 | 04 | todo | src/frontmatter.rs | 219 | walk() scalar truncation panics on real multi-byte UTF-8 content over MAX_VALUE_LEN (assertion failed: self.is_char_boundary(new_len)); reproduced against ~/.cursor/extensions during 04-01 baseline capture; not fixed (Task 1 required zero source diff) — file a follow-up issue | open |  | 2026-09-03T09:08:30.773Z |  |
| 2 | 04 | todo | tests/corpus/attack/structural/README.md |  | WR-02 (carried over from Phase 3): the Payloads table documents only 1 of tool-permission-abuse/'s 5 corpus files; noted explicitly in the README as still open after the 04-01 directory-layout generalisation | open |  | 2026-09-03T09:08:34.501Z |  |

````json
[
  {
    "id": 1,
    "kind": "todo",
    "phase": "04",
    "file": "src/frontmatter.rs",
    "line": 219,
    "description": "walk() scalar truncation panics on real multi-byte UTF-8 content over MAX_VALUE_LEN (assertion failed: self.is_char_boundary(new_len)); reproduced against ~/.cursor/extensions during 04-01 baseline capture; not fixed (Task 1 required zero source diff) — file a follow-up issue",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-03T09:08:30.773Z",
    "resolved_at": null
  },
  {
    "id": 2,
    "kind": "todo",
    "phase": "04",
    "file": "tests/corpus/attack/structural/README.md",
    "line": null,
    "description": "WR-02 (carried over from Phase 3): the Payloads table documents only 1 of tool-permission-abuse/'s 5 corpus files; noted explicitly in the README as still open after the 04-01 directory-layout generalisation",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-03T09:08:34.501Z",
    "resolved_at": null
  }
]
````
