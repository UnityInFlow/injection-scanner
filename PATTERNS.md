# Pattern Contribution Guide

injection-scanner uses a YAML-based pattern library. Core patterns are embedded at compile time. Community patterns can be added via PR.

## Pattern Format

```yaml
category: category_name
default_severity: CRITICAL  # CRITICAL | HIGH | MEDIUM | LOW
patterns:
  - id: PI0XX
    name: descriptive-name
    pattern: "regex\\s+pattern"
    severity: HIGH  # optional -- overrides category default
    case_sensitive: false  # optional -- default false
    raw_only: false        # optional -- default false, see below
    example: "the plainest form of the attack"          # REQUIRED
    counter_example: "legitimate text that must not match"  # optional, expected
    relaxed_pattern: "a deliberately widened form of `pattern`"  # required for PI050+, see below
    description: "What this detects"
    remediation: "How to fix"
    tags: [tag1, tag2]
```

Unknown fields are rejected at load time, so a typo fails loudly rather than
being silently ignored.

### `example` and `counter_example`

`example` is **required**. It is the plainest one-line form of the attack the
pattern exists to catch, and it is binding: `tests/pattern_example_test.rs`
asserts it actually matches the regex beside it.

`counter_example` is optional but expected wherever the pattern could plausibly
fire on ordinary prose. It is the false positive you were most worried about,
written down — a test asserts it does **not** match, so the near-miss stays
pinned instead of living in a review comment.

Both are rendered into [`docs/PATTERN-CATALOGUE.md`](docs/PATTERN-CATALOGUE.md),
which is generated. After any change to the library, regenerate it:

```bash
cargo run --release -- rules --format markdown > docs/PATTERN-CATALOGUE.md
```

`tests/catalogue_test.rs` fails if you forget. That is deliberate: a stale
catalogue tells a reader, in the repo's own voice, that the scanner catches
something it may no longer catch.

### `case_sensitive`

Defaults to `false`. Payloads are natural language, and an attacker
capitalising a sentence must not defeat detection. Set it to `true` only where
the casing itself carries the signal.

### `raw_only`

Defaults to `false`. Every pattern runs twice: once against the raw source
text, and once against a Unicode-normalized copy that folds confusables,
zero-width characters and spacing tricks back to plain Latin. That second pass
is what stops `іgnore all previous instructions` (Cyrillic `і`) from walking
past a pattern.

`raw_only: true` **turns that second pass off for your pattern.** It exists for
one narrow case: a detector whose signal *is* the raw bytes, such as PI045's
mixed-script check. Normalization folds the confusable back to Latin, so on the
normalized pass every bilingual document looks like a mixed-script token.

For anything else, `raw_only: true` weakens your pattern — it hands an attacker
the obfuscation bypass the normalized pass exists to close. Expect a reviewer
to ask why.

> **Tags never change behaviour.** `tags` is free-form metadata for grouping and
> search. If you want to opt out of the normalized pass, set `raw_only`; naming
> a tag `homoglyph` does nothing. This is pinned by
> `tests/raw_only_test.rs::a_tag_alone_can_never_disable_the_normalized_pass`.

### `relaxed_pattern`

**Required for `PI050` and above** (D-09); optional and unused below that
range — the existing 48 patterns stay exempt.

`relaxed_pattern` is a deliberately **widened** variant of your pattern's own
regex, with the narrowing removed. It is never loaded into the live scanner —
it exists only so `tests/pattern_relaxed_control_test.rs` can prove your
narrowing is actually load-bearing rather than asserted in a PR description.
That test builds a second `Scanner` where `relaxed_pattern` stands in for
`pattern`, and asserts the shipped scanner does **not** match your
`counter_example` while the relaxed one **does**.

This exists because "break it and confirm the corpus goes red" was a ritual,
not a gate, and two PRs in a row shipped a widening whose control was not
actually held by the corpus:

- **#95.** PI021's disclosure arms depend on requiring the possessive (`your
  system prompt`, not `the system prompt`). Relaxing it to `(?:your|the)`
  left the entire clean corpus green — the near-miss survived by an
  unrelated accident of pluralisation, not because the corpus caught it.
- **#97.** PI018's precedence arm produced six HIGH findings on ordinary
  configuration prose in its first draft. HIGH is what `install-hook` blocks
  commits at.

Like `raw_only`, it is deliberately a schema field rather than a tag, so a
typo in a community pattern file fails to load instead of silently shipping
without a mutation-tested control.

`relaxed_pattern` is **not** rendered into
[`docs/PATTERN-CATALOGUE.md`](docs/PATTERN-CATALOGUE.md) (D-08a). The
catalogue documents what the scanner detects — and the shipped regex is
already public there in the `Regex` details block — while this field
describes what the scanner deliberately does *not* detect. That is test
scaffolding, not a detection to disclose.

## Categories

| Category | ID Range | Default Severity |
|---|---|---|
| Role Override | PI001-PI009 | CRITICAL |
| Instruction Injection | PI010-PI019 | HIGH |
| Data Exfiltration | PI020-PI029 | CRITICAL |
| Jailbreaks | PI030-PI039 | HIGH |
| Encoding/Obfuscation | PI040-PI049 | HIGH |

## Grading Severity

Every pattern in the library used to be CRITICAL or HIGH — 14 and 16, with no
MEDIUM and no LOW anywhere. Everything was an emergency, so nothing was, and
`--fail-on <severity>` had nothing meaningful to threshold on.

Grade against **how much benign text shares the phrasing**, not against how bad
the attack would be if real. Every pattern here describes a bad outcome; that is
not what separates them.

| Severity | Test | Examples |
|---|---|---|
| **CRITICAL** | No plausible benign reading. If this string is in a document, something is wrong. | `ignore all previous instructions`, `[SYSTEM OVERRIDE]`, `send the following to https://…`, a run of zero-width characters |
| **HIGH** | Strong signal; benign use is rare and usually deliberate. | `DAN mode`, `developer mode enabled`, `your real purpose is`, `bypass the content filter` |
| **MEDIUM** | Suspicious in context, but ordinary documents genuinely say this. | `you are now …`, `New task:`, `fetch https://…`, `in this hypothetical scenario where you …` |
| **LOW** | A heuristic or a weak signal. Names a concept, or could be an artefact. | `jailbreak prompt` (the term, not a payload), a single zero-width character |

Three rules that decide most cases:

1. **Naming is not carrying.** A pattern that matches a *term* — `jailbreak
   prompt`, `DAN` — fires on every security write-up that discusses the
   technique, this repository's own documentation included. That is LOW or
   MEDIUM, never CRITICAL.
2. **One is an artefact, several are intent.** A single zero-width character
   arrives from a word processor or a copied web page. A run of three is
   deliberate. `PI041` is LOW and `PI042` is CRITICAL for exactly this reason.
3. **If you had to imagine the benign case, it is not CRITICAL.** If you can
   recall a real document that would match, it is MEDIUM at most.

Grades are enforced two ways. `tests/pattern_test.rs` asserts every level stays
populated, so the library cannot silently drift back to all-CRITICAL. And
`tests/corpus/clean/` holds real documents that must produce **zero** findings —
if your pattern fires there, the grade is not the problem.

## Submitting a Pattern

1. Fork the repo
2. Add pattern to the appropriate `patterns/core/*.yaml` file
3. Include in your PR:
   - At least 3 true positive test cases
   - At least 2 non-match cases (false positive prevention)

   Both are enforced in CI by `tests/pattern_policy_test.rs` — add them with the
   `assert_positives` / `assert_negatives` helpers in `tests/pattern_test.rs`.

   **Make the non-match cases near-misses.** A negative that fails for a trivial
   reason proves nothing. The cautionary example is PI048, which shipped with
   negatives `shortToken123`, `not-base64-at-all!!!` and `abcd` — every one of
   them failing on *length* — and still matched any file path over 48
   characters, for 3,494 false positives on our own documentation. Write the
   legitimate text you would most fear the pattern firing on.
4. Regenerate the catalogue: `cargo run --release -- rules --format markdown > docs/PATTERN-CATALOGUE.md`
5. Run `cargo test`
6. Submit PR with title: `feat: add PI0XX pattern-name`
