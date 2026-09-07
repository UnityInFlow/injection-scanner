# Deferred Items — Quick Task 260902-jhy (fix CR-01 negation blindness)

Out-of-scope discoveries logged per the executor's scope-boundary rule: found
during this task's Task 2 whole-repo self-scan, not caused by this task's
changes, not fixed here.

## docs/PATTERN-CATALOGUE.md:74 — PI001 self-match on PI002's own description

**Found during:** Task 2, whole-repo self-scan (`cargo run --release -- check
. --exclude '.planning/**' --format json`, filtered per the pattern-library
skill).

**Issue:** `docs/PATTERN-CATALOGUE.md` line 74 renders PI002's own
`description` field, "Attempts to discard prior context", into the generated
catalogue. That prose matches PI001 (`ignore-previous-instructions`) on the
substring "discard prior context".

**Why not fixed here:** Confirmed present verbatim at the same line in the
catalogue committed at `26fc6af` — the commit this quick task started from,
before any Task 1 edit touched PI053/PI056/PI057. Not caused by this task's
changes; fixing PI001 or PI002's description text is unrelated to CR-01/WR-01
and out of scope per "only auto-fix issues directly caused by the current
task's changes."

**Suggested fix:** Reword PI002's `description` in
`patterns/core/role-override.yaml` (or wherever PI002 is defined) so it no
longer contains the literal phrase PI001 matches on, then regenerate the
catalogue.

## docs/PATTERN-CATALOGUE.md:903 — PI031 self-match on PI031's own catalogued example prose

**Found during:** Task 2, whole-repo self-scan (same run as above).

**Issue:** `docs/PATTERN-CATALOGUE.md` line 903 renders prose (from PI031's
own catalogue entry, "DAN is the canonical example of a persona attack") that
matches PI031 (`dan-mode`) on the substring "DAN mode" via loose matching in
the surrounding text/fix line.

**Why not fixed here:** Confirmed present verbatim at the same line in the
catalogue committed at `26fc6af`, before any Task 1 edit. Not caused by this
task's changes; PI031 is a jailbreak-category pattern untouched by CR-01/WR-01
and out of scope here.

**Suggested fix:** Either wrap the catalogue's own illustrative prose in a
code span so it renders as documentation rather than live prose, or reword
the fix/description text so it no longer contains "DAN mode" outside a code
span. This is the same self-referential-documentation failure mode the
pattern-library skill already documents for `docs/DETECTION-BACKLOG.md`.
