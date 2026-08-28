---
name: pattern-library
description: Use when adding, editing, removing or re-tuning a detection pattern in patterns/core/*.yaml, or when a PR touches the pattern library. Covers the required schema fields, the example/counter_example contract, regenerating docs/PATTERN-CATALOGUE.md, and the false-positive gates that will otherwise fail CI.
---

# Pattern Library — Changing What the Scanner Detects

Any change under `patterns/core/` changes what a security tool claims to catch.
Four artifacts must stay in step, and CI enforces all four.

## The loop

1. Edit or add the pattern in `patterns/core/<category>.yaml`.
2. Add positive and negative test cases in `tests/pattern_test.rs`
   (**≥3 positives, ≥2 negatives** — the `PATTERNS.md` policy).
3. **Regenerate the catalogue.** Not optional; `tests/catalogue_test.rs` fails otherwise:
   ```bash
   cargo run --release -- rules --format markdown > docs/PATTERN-CATALOGUE.md
   ```
4. Run the gate:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --locked -- -D warnings
   cargo test --locked
   ```

## Required schema fields

Every pattern needs `id`, `name`, `pattern`, `description`, `remediation`, `tags`
and **`example`**. `counter_example` is optional but expected for anything that
could plausibly fire on ordinary prose.

```yaml
  - id: PI0XX
    name: descriptive-name
    example: "the plainest form of the attack, one line"
    counter_example: "legitimate text that looks like it but must not match"
    severity: HIGH          # optional, overrides the category default
    case_sensitive: true    # optional, default false
    raw_only: true          # optional, default false — see the warning below
    pattern: "regex"
    description: "What this detects"
    remediation: "How to fix it"
    tags: [tag1, tag2]
```

Unknown fields are rejected at load time, so a typo fails loudly.

**`example` and `counter_example` are binding.** `tests/pattern_example_test.rs`
asserts the example matches its own pattern and the counter_example does not.
They are rendered into `docs/PATTERN-CATALOGUE.md`, so a stale one is a
documented claim the scanner no longer honours.

**Never set `raw_only: true`** unless the pattern's signal *is* the raw bytes
(only PI045, the mixed-script detector, qualifies today). It turns off the
Unicode-normalized pass, which is what stops a confusable character from
walking a payload past the pattern.

**Tags never change behaviour.** They are metadata. If you want to opt out of
the normalized pass, set `raw_only`.

## The gates that catch pattern mistakes

| Gate | What it means when it fails |
|---|---|
| `catalogue_test` | You changed the library and did not regenerate the catalogue. Run the command in step 3. |
| `pattern_example_test` | Your `example` no longer matches, or your `counter_example` now does — the regex moved. |
| `corpus_test` | Your pattern fires on a legitimate document in `tests/corpus/clean/`. **Fix the pattern, not the corpus.** |
| `markdown_context_test` | The attack-corpus counts are pinned exactly. A rise is as suspicious as a fall — explain it. |

### The corpus rule, stated plainly

`tests/corpus/clean/` is the only false-positive gate this repo has. Editing a
clean document so a new pattern passes inverts what the gate is for. That has
happened: PI045 once matched whole Unicode blocks, so `Δt` and `kΩ` were
deleted from the corpus rather than the pattern being narrowed.

Edit the corpus only when the tension is genuinely inherent — for example, no
regex can separate a collector domain in an egress deny list from the same
domain in an exfil instruction. Say so in the PR when you do.

Note `corpus_test` also runs under `--strict`, which ignores the code-fence
confidence downgrade. Being saved by a code fence is not the same as being
correct, so check with `--strict` before concluding a document is clean.

## Severity

Grading criteria live in `PATTERNS.md`. Two things worth repeating:

- **HIGH is the bar `install-hook` writes by default.** A HIGH false positive
  blocks people's commits. Prefer MEDIUM for anything heuristic.
- Match the severity of the nearest existing pattern for the same attack, or
  say why it differs.

## Before you finish

- [ ] `docs/PATTERN-CATALOGUE.md` regenerated and committed
- [ ] ≥3 positives and ≥2 negatives in `tests/pattern_test.rs`
- [ ] `example` present; `counter_example` present unless there is no plausible near-miss
- [ ] No verbatim payload written into a new file outside `examples/`, `patterns/` or `tests/` — this repo scans itself
- [ ] If you touched `examples/` or `patterns/`, regenerate the code-scanning baseline:
      `cargo run --release -- check . --exclude '.planning/**' --write-baseline .github/code-scanning-baseline.json`
- [ ] Full gate green
