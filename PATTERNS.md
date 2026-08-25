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
    description: "What this detects"
    remediation: "How to fix"
    tags: [tag1, tag2]
```

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
4. Run `cargo test`
5. Submit PR with title: `feat: add PI0XX pattern-name`
