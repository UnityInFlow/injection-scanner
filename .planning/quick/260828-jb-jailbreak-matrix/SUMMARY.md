---
task_id: 260828-jb
status: complete
issue: 99
branch: feat/jailbreak-matrix
commit: 78178e3
---

# Summary — jailbreak matches the exploit, not its name (#99)

**1/12 -> 12/12. Repo total 45/60 -> 56/60 (93%).** Fourth and last category of the series.

## The distinct failure mode

The other three categories held literal *phrases*. This one held **technique names**:
`grandma exploit`, `jailbreak prompt`, `DAN mode`. Those are strings that appear in
security writing and do not appear in payloads, so the patterns were wrong in both
directions — noisy on write-ups, silent on attacks. PI035's LOW grade was compensating
for a pattern that could not be right rather than reflecting a genuine low-signal finding.

**Carry forward:** when a pattern's severity has been lowered to make it tolerable, treat
that as evidence the pattern is wrong, not as a tuning outcome. PI035 went LOW -> MEDIUM
once it matched an actual payload shape.

## The skill loop paid off on its first use

#97 added "ask what the nearest legitimate document looks like, and write it into the
corpus first". Applied here, that produced `clean/jailbreak-writeup.md` — and the
**pre-existing** patterns scored four findings on it, two at HIGH, before anything was
changed. The corpus proved the thesis of the issue before a line of the fix was written.

Both false-positive controls mutation-tested: restoring `grandma exploit` reddens two
assertions, dropping PI038's activation verb reddens two.

## An example file was the tell

`examples/jailbreak-attack.md` contained "try the grandma exploit next" — a technique
name, in a file whose purpose is carrying payloads. It is the only reason the name-matching
arm ever looked justified, and it made a broken pattern appear to have a true positive.
**When a pattern's only true positive is in a fixture you control, check the fixture.**

## A test fixture that depended on a bug

`sarif_test` sourced its LOW example by scanning `tests/corpus_test.rs` for PI035 — that
is, it relied on PI035 firing on a source comment that merely names a technique. Fixing
the pattern removed the fixture. Repointed at PI041 in the generated catalogue.
**A test that harvests its fixture from live behaviour will break when that behaviour was
the bug.** Not an argument against the technique — it is better than typed literals — but
worth knowing.

## Series total

| | before | after |
|---|---|---|
| role_override | 1/12 | 11/12 |
| exfiltration | 0/12 | 12/12 |
| instruction_injection | 0/12 | 12/12 |
| jailbreak | 1/12 | 12/12 |
| encoding | 9/12 | 9/12 |
| **total** | **10/60 (17%)** | **56/60 (93%)** |

Clean corpus never regressed; it gained three specimens and got strictly harder.
`LEGACY_UNTESTED` fell from 30 entries to 11.

## Remaining, and deliberate

Three encoding misses need the decoder (#30) — a length-based regex cannot separate a
base64 payload from a file path. One role-override precedence claim is not separable from
ordinary documentation. Both recorded in `recall_test` and the README.
