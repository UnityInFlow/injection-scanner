---
phase: quick
plan: 261007-cvr
status: complete
completed: 2026-10-07
issue: 133
branch: docs/133-corpus-readme-payload-table
key-files:
  modified: [tests/corpus/attack/structural/README.md]
---

# Quick Task 261007-cvr: close #133 (WR-02)

Backfilled the four missing rows in `tests/corpus/attack/structural/README.md`'s
`tool-permission-abuse/` Payloads table, which had documented 1 of 5 payloads since
Phase 3. Documentation only — one file, one commit.

## Shape descriptions were verified, not copied

#133 suggests row text. Each was checked against the actual payload file rather than
taken as given, and one needed correcting:

- `04-permissions-allow-wildcard-settings.md` also carries a populated `deny` list
  (`Read(.env)`), which #133's suggested row omits. Worth stating, because it is the
  point of the specimen: the deny entry does not narrow the `Bash(*)` wildcard sitting
  beside it. The row says so.
- `02`, `03` and `05` matched their suggested descriptions; the wording was tightened
  to name what distinguishes each from `01` (scalar vs. sequence, array entry,
  permission *mode* rather than a grant).

Also added the one-line framing the `mcp-tool-poisoning/` table already has, since
the shapes do spread deliberately: 3 YAML-frontmatter skill files (`01`, `02`, `05`)
and 2 bare JSON documents (`03`, `04`).

## WR-02's ledger entry needed no change

`.planning/WINDOWS.md` entry 2 is already `waived` with `resolved_at` set, waived *in
Phase 4* on the grounds that it was filed as #133 and out of scope for a CAT-02-only
PR under GATE-04. That record is historically accurate as written, so it was left
alone rather than rewritten now that #133 is actually fixed.

## Verification

| Check | Result |
|---|---|
| `cargo test --test recall_test` | 8 passed, 0 failed |
| `cargo test --test corpus_test` | 5 passed, 0 failed |
| Payload files | untouched — `git diff --name-only` lists only the README |
| Pinned recall row (`tool-permission-abuse-structural`) | unchanged |

No pattern, corpus content or counter-example change, so the `pattern-library` skill
does not apply and nothing was regenerated.
