# ADR-007: Path-aware source positions for structural findings

Date: 2026-10-25
Status: Proposed

## Context

Issue #165: after parsing configuration into `serde_json::Value`, projection
searched the original text for the leaf key and selected its first occurrence.
Repeated command and URL keys therefore shared a source line. Traversal can be
sorted rather than source ordered, so moving a search cursor forward is not a
correct fix. Matching by scalar value also fails for repeated identical values.

The structural scanner already consumes `ProjectedLine.line` correctly.
Suppression is keyed by this line. The baseline implementation excludes lines
from identity: it hashes matched text and budgets an explicit
occurrence count. SARIF adds occurrence ordinals to the same text digest.
Those contracts must remain stable.

## Decision

Build an auxiliary source-position tree and look up each projected leaf by its
actual key/index path. Semantic parsing, canonical projection text, and pattern
matching remain authoritative in their existing implementations.

- `yaml-rust2` 0.10.4 supplies marked events for YAML and JSON source positions.
  Default encoding features are disabled because input is already UTF-8.
  JSONC is comment/comma-relaxed first; JSON string literals are decoded and
  re-encoded for this position pass to support surrogate pairs and prevent
  Unicode line separators inside strings from shifting physical line numbers.
  Neither transformation changes the semantic tree or physical newline count.
- `toml_edit` 0.22 supplies immutable parsed spans. This crate was already in
  the locked dependency graph through `toml`; it is now also a direct dependency.
  Line offsets are indexed once and searched rather than repeatedly scanning
  the source prefix. Tables, arrays of tables, arrays, and inline tables are
  traversed without cloning TOML subtrees.
- Map values use their key's line, sequence members use their own line, and
  alias-expanded YAML leaves use the alias reference. JSON duplicate members
  follow the semantic parser's surviving last member.
- Position collection is bounded by the projection depth and an auxiliary
  node budget of `MAX_NODES * (MAX_DEPTH + 1)` to accommodate containers as
  well as leaves. Alias expansion also consumes this budget. An unavailable
  position falls back to the block's first line; projection is never dropped
  merely because the optional source-position pass cannot map it.

Also correct the JSON block start-line calculation for leading blank lines.

## Consequences

Repeated keys now have independent source lines in reports, suppression, and
SARIF locations. Consumers may see later findings previously hidden by an
ignore on the first key. Regression tests cover the three real clean hook
fixtures named by the issue, repeated identical commands, reordered objects,
JSONC comments, escaped and unusual keys, Unicode, YAML blocks and aliases,
TOML containers, sequence items, and independent suppression and baselining.

The JSON/SARIF field sets, baseline schema and fingerprint algorithm are
unchanged. Existing baseline fingerprints remain valid; first-seen line
metadata is regenerated where corrected locations move it. No patterns,
severity levels, or recall expectations change.

Costs: structured documents need a second parser pass and a bounded position
tree. The lockfile adds `yaml-rust2` and four transitive packages. Tests, release
benchmarks, and the existing performance guard check this path. The source-map
fallback remains best effort for unsupported or budget-exceeding structures.

## Alternatives considered

- Per-key occurrence counters depend on source traversal order and cannot
  resolve repeated keys in reordered objects, comments, or string values.
- Matching key/value text cannot distinguish identical repeated commands.
- A handwritten source lexer for three configuration languages would duplicate
  string escapes, flow syntax, aliases, and multiline-string handling.
- Replacing the semantic parsers would broaden compatibility risk beyond the
  source-attribution bug. The additional parsers supply positions only.

References: [yaml-rust2 marked events](https://docs.rs/yaml-rust2/0.10.4/yaml_rust2/parser/index.html),
[toml_edit immutable documents](https://docs.rs/toml_edit/0.22.27/toml_edit/struct.ImDocument.html).
