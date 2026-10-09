---
schema_version: 1
open_count: 0
waived_count: 1
fixed_count: 2
total_count: 3
last_updated: 2026-10-09T12:21:48.063Z
---

# Broken Windows Ledger

> Cross-phase defect register. With `workflow.windows_enforce` enabled, `/gsd-ship` blocks while `open_count > 0`.
> Waive with `gsd-tools windows waive <id> "<reason>"` (reason required).
> Mark fixed with `gsd-tools windows fixed <id>`.

| id | phase | kind | file | line | description | status | reason | recorded_at | resolved_at |
|----|-------|------|------|------|-------------|--------|--------|-------------|-------------|
| 1 | 04 | todo | src/frontmatter.rs | 219 | walk() scalar truncation panics on real multi-byte UTF-8 content over MAX_VALUE_LEN (assertion failed: self.is_char_boundary(new_len)); reproduced against ~/.cursor/extensions during 04-01 baseline capture; not fixed (Task 1 required zero source diff) — file a follow-up issue | fixed |  | 2026-09-03T09:08:30.773Z | 2026-09-07T18:10:37.614Z |
| 2 | 04 | todo | tests/corpus/attack/structural/README.md |  | WR-02 (carried over from Phase 3): the Payloads table documents only 1 of tool-permission-abuse/'s 5 corpus files; noted explicitly in the README as still open after the 04-01 directory-layout generalisation | waived | Filed as issue #133 — CAT-01's own corpus doc table backfill, out of scope for a CAT-02-only PR (GATE-04); not fixed here | 2026-09-03T09:08:34.501Z | 2026-09-07T18:13:18.253Z |
| 3 | 5 | unmet-truth | patterns/core/persistence-lifecycle-hijack.yaml | 436 | BLOCKING review finding BL-01 (issue 183): PI070 widened objects and verbs make third-person will vendor-feature sentences fire HIGH (13 of 17 probes new vs pre-phase). RESOLVED 2026-10-09 on the maintainer's decision: `will` removed from PI070's modal set (`may now` kept). Measured cost zero attack-corpus and zero held-out detections; held-out stays 2/12 and development 103/115. Clean specimen persistence-vendor-release-notes.md added and mutation-tested (shipped 0, `will` restored 6). Issue 183 closed. See 05-REVIEW.md 'BL-01: RESOLVED'. | fixed |  | 2026-10-09T12:02:45.581Z | 2026-10-09T12:21:48.063Z |

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
  },
  {
    "id": 3,
    "kind": "unmet-truth",
    "phase": "5",
    "file": "patterns/core/persistence-lifecycle-hijack.yaml",
    "line": 436,
    "description": "BLOCKING review finding BL-01 (issue 183): PI070 widened objects and verbs make third-person will vendor-feature sentences fire HIGH (13 of 17 probes new vs pre-phase). RESOLVED 2026-10-09 on the maintainer's decision: `will` removed from PI070's modal set (`may now` kept). Measured cost zero attack-corpus and zero held-out detections; held-out stays 2/12 and development 103/115. Clean specimen persistence-vendor-release-notes.md added and mutation-tested (shipped 0, `will` restored 6). Issue 183 closed. See 05-REVIEW.md 'BL-01: RESOLVED'.",
    "status": "fixed",
    "reason": "",
    "recorded_at": "2026-10-09T12:02:45.581Z",
    "resolved_at": "2026-10-09T12:21:48.063Z"
  }
]
````
