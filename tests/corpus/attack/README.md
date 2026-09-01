# The attack corpus — measuring recall

`tests/corpus/clean/` proves the scanner does not fire on legitimate documents.
This directory is its mirror: it proves the scanner *does* fire on attacks, and
puts a number on how often.

Before this corpus existed, a build scoring 100% on the clean corpus and 10% on
real attacks passed every gate the repo had. Coverage measured code, benchmarks
measured speed, the clean corpus measured precision — nothing measured whether
the scanner finds attacks.

## The sourcing rule

**Payloads here must not be derived from the patterns.**

This is the whole point, and it is easy to get wrong. A corpus assembled from
each pattern's own `example` field would score 100% recall by construction and
measure nothing at all — the patterns would be graded against strings written
to match them.

So payloads are written from the *threat model*: how an attacker actually
phrases the attack, drawn from published prompt-injection technique families,
without consulting the regexes. A payload that no pattern catches is not a bug
in the corpus. It is the corpus doing its job.

If you add a payload, add it because it is a realistic attack, never because
you know a pattern will catch it.

## Layout

One file per category the README claims to detect. One payload per line; blank
lines and lines beginning with `#` are ignored. Each line is scanned as its own
document, so recall is per payload rather than per file.

Categories that are deliberately **not** here yet: MCP/tool-description
poisoning, indirect RAG-borne injection. The README does not claim those
yet — they are the `PI060`–`PI089` ranges deferred within v0.2.0. Including
them would depress the headline number for detection the tool never
advertised. Add them here in the same commit that claims them.

Tool and permission abuse (`PI050`–`PI059`) is no longer in that list: its 12
threat-model payloads landed ahead of any pattern (D-04), so the corpus
proves the ordering rather than asserting it. See `structural/` below for
where its structured half lives.

## The `structural/` directory

Not every payload in this category is a line. A wildcard tool grant only
exists as an attack once it is sitting inside a real, parseable YAML/TOML/JSON
document — a single line split out of that document is not the attack, it is
a fragment of one. `structural/` is a second collection mode: each file
there is one whole payload, scanned as a single document rather than split
into lines. See `structural/README.md` for the format and its fence-position
constraint.

The `encoding.md` payloads are different in kind: each one is an obfuscation of
a payload the scanner catches in plain form, so a miss there means an evasion
works, not that a phrasing is unrecognised.

## The number

`tests/recall_test.rs` scans every payload and pins the per-category detection
counts **exactly** — not as a floor. An exact pin is deliberate: it means an
improvement fails the build too, forcing the number to be updated here and in
the README rather than quietly drifting upward while the published figure goes
stale. A scanner that will not state its own recall is asking to be trusted on
the strength of its feature list.

A drop is the serious direction. It means detection regressed, and the test
prints exactly which payloads stopped being caught.
