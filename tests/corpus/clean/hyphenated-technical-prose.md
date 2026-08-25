# Architecture decisions

<!-- Guards the separator fold added for #26. Every hyphen, slash and underscore
     below sits between two word characters, which is exactly the shape the
     normalizer treats as separator injection. Folding them is what defeats
     hyphen-separated payloads — but it must not turn ordinary technical prose
     into findings.

     The payload itself is deliberately NOT quoted here. An HTML comment scores
     1.0, on purpose, because hidden text is a delivery mechanism rather than a
     disclaimer — so a comment is the one place in a `clean/` specimen where
     quoting a payload is guaranteed to fail. It did. -->

We chose a read-only, copy-on-write file-system layer for the state-of-the-art
context-aware cache. It is well-known that write-through caching under-performs
here.

The build runs `cargo-zigbuild` cross-compilation for x86_64-unknown-linux-musl
and aarch64-unknown-linux-gnu, writing to target/release/injection-scanner.

Config keys use snake_case: max_file_size, follow_symlinks, respect_gitignore.
Feature flags are kebab-case: no-ignore, all-files, strict-patterns.

Latency is measured end-to-end, p50/p95/p99, in micro-seconds.

Non-ASCII prose must survive too: the café serves crème brûlée, the naïve
approach fails, and Björn's résumé mentions Zürich.
