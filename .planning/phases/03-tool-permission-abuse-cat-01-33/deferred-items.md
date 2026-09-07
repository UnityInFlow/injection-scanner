# Deferred Items — Phase 3 (03-tool-permission-abuse-cat-01-33)

Out-of-scope discoveries logged per the executor's scope-boundary rule: found
during this phase's plans, not caused by them, not fixed here.

## README.md:271 — stale "60 realistic payloads" count

**Found during:** 03-05 Task 4 (recall re-pin), while updating the recall table.

**Issue:** `README.md` line 271 reads "`tests/corpus/attack/` holds 60
realistic payloads written from the threat model" — stale since Plan 01
(03-01) added CAT-01's 12 threat-model payloads, bringing the true total to
72. Line 330 elsewhere in the same file already correctly says "72 payloads",
so the file is internally inconsistent between the two locations.

**Why not fixed here:** Plan 05's Task 4 scope is explicitly the two-place
GATE-02 rule (pattern-count sentence + category table) plus the recall table
row — this sentence is a third, unrelated location that predates this plan's
changes (introduced by Plan 01, not Plan 05). Out of scope per the "only
auto-fix issues directly caused by the current task's changes" boundary.

**Suggested fix:** Change "holds 60 realistic payloads" to "holds 72 realistic
payloads" at README.md:271.
