//! Characterization tests for issue #105 (`serde_yaml` 0.9 is unmaintained).
//!
//! This crate parses YAML from two untrusted sources: community pattern files
//! (`patterns/core/*.yaml`, `--patterns`) and YAML frontmatter in scanned
//! documents (ENG-01, #32). Swapping the underlying parser is a security-
//! relevant change precisely because it could silently alter what the scanner
//! accepts, rejects, or extracts — with nothing else noticing.
//!
//! These tests pin the observable behaviour of the YAML loader on the edge
//! cases that matter for a parser sitting on adversary-controlled input:
//! malformed syntax, unknown fields, duplicate keys, anchors/aliases (both
//! ordinary and pathological), deep nesting, empty documents, and YAML's
//! implicit-typing gotchas on scalars that look like plain strings. They must
//! pass unchanged before and after the dependency swap — that is the evidence
//! the swap is behaviour-preserving. Where a case exercises a *policy* this
//! crate enforces (e.g. `deny_unknown_fields`) rather than raw parser
//! behaviour, the assertion is written against that policy, not against a
//! specific upstream error string.

use std::fs;

use injection_scanner::frontmatter::{self, ConfigBlock, ConfigSyntax};
use injection_scanner::pattern::PatternCategory;
use injection_scanner::patterns::load_external_patterns;

/// Cheap per-call uniqueness so a second, concurrently running process of
/// this same test binary (a worktree's `cargo test`, a Stop hook, another CI
/// job on a shared runner) never resolves to the same directory and deletes
/// a fixture out from under it (issue #124).
fn unique_suffix() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "injscan-yaml-swap-{}-{}-{name}",
        std::process::id(),
        unique_suffix()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir must be creatable");
    dir
}

fn yaml_block(body: &str) -> ConfigBlock {
    ConfigBlock {
        syntax: ConfigSyntax::Yaml,
        body: body.to_string(),
        start_line: 2,
    }
}

// ------------------------------------------------------------- malformed YAML

#[test]
fn malformed_yaml_is_rejected_by_the_frontmatter_parser() {
    let block = yaml_block("broken: [unclosed\n");
    let err = frontmatter::parse(&block).expect_err("unclosed flow sequence must not parse");
    assert!(
        err.contains("invalid YAML frontmatter"),
        "error must be tagged as a YAML frontmatter failure: {err}"
    );
}

#[test]
fn malformed_yaml_is_rejected_by_the_pattern_loader() {
    let yaml = "category: t\ndefault_severity: LOW\npatterns: [unterminated\n";
    let parsed: Result<PatternCategory, _> = serde_yaml::from_str(yaml);
    assert!(parsed.is_err(), "malformed pattern YAML must be rejected");
}

// -------------------------------------------------------------- unknown keys

#[test]
fn unknown_top_level_pattern_field_is_rejected() {
    // Pins the `deny_unknown_fields` contract at the category level, not just
    // the per-pattern level already covered in `pattern_validation_test.rs`.
    let yaml = "category: t\ndefault_severity: LOW\npatterns: []\nextra_field: surprise\n";
    let parsed: Result<PatternCategory, _> = serde_yaml::from_str(yaml);
    assert!(
        parsed.is_err(),
        "an unrecognised top-level key must be rejected, not ignored"
    );
}

// -------------------------------------------------------------- duplicate keys

#[test]
fn duplicate_top_level_keys_are_rejected_when_deserializing_into_a_typed_struct() {
    // Measured, not assumed: deserializing straight into `PatternCategory`
    // rejects a duplicated top-level key outright ("duplicate field
    // `category`") rather than letting the last occurrence win. This is
    // serde's struct visitor rejecting the second occurrence of a field it has
    // already seen, independent of the underlying YAML event source — so it
    // is a property of the strongly-typed load path, not of the raw parser.
    // Contrast with `duplicate_keys_in_frontmatter_resolve_to_the_last_occurrence`
    // below, which deserializes into a free-form `Value` and behaves
    // differently. A parser swap that instead started silently accepting a
    // duplicated pattern-file key would be a real regression this test
    // catches.
    let yaml = "category: first\ncategory: second\ndefault_severity: LOW\npatterns: []\n";
    let parsed: Result<PatternCategory, _> = serde_yaml::from_str(yaml);
    assert!(
        parsed.is_err(),
        "a duplicated top-level key must be rejected when loading a pattern category"
    );
}

#[test]
fn duplicate_keys_in_frontmatter_resolve_to_the_last_occurrence() {
    let block = yaml_block("name: first\nname: second\n");
    let value = frontmatter::parse(&block).expect("duplicate keys must still parse");
    assert_eq!(
        value.get("name").and_then(|v| v.as_str()),
        Some("second"),
        "the last occurrence must win in the projected value"
    );
}

// -------------------------------------------------------------- anchors/aliases

#[test]
fn ordinary_anchors_and_aliases_expand_as_expected() {
    // The non-pathological case: legitimate YAML frontmatter is allowed to use
    // anchors for de-duplication, and the expansion must actually happen —
    // this is the positive control for the billion-laughs refusal test below.
    let block = yaml_block("base: &base\n  role: admin\nuser: *base\n");
    let value = frontmatter::parse(&block).expect("ordinary anchor/alias YAML must parse");
    assert_eq!(
        value.pointer("/user/role").and_then(|v| v.as_str()),
        Some("admin"),
        "an alias must expand to the anchored value"
    );
}

#[test]
fn a_yaml_alias_bomb_in_an_external_pattern_file_does_not_hang_the_loader() {
    // The frontmatter engine already pins this property (see
    // `tests/frontmatter_test.rs::a_yaml_alias_bomb_is_refused_rather_than_expanded`)
    // for adversary-authored *documents*. Community pattern files are the
    // other untrusted-input surface named in issue #105, so the same DoS
    // shape is pinned here for `load_external_patterns`.
    let dir = temp_dir("alias-bomb");
    // The bomb must be refused during the raw YAML parse itself, before
    // `PatternCategory`'s schema validation ever runs — so it is built as
    // top-level alias-expanding keys sitting alongside the (unreachable,
    // never-validated) required fields, exactly mirroring the frontmatter
    // engine's billion-laughs fixture.
    let mut bomb = String::from("category: bomb\ndefault_severity: LOW\n");
    bomb.push_str("a0: &a0 [\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\"]\n");
    for level in 1..9 {
        let prev = format!("*a{}", level - 1);
        let row: Vec<&str> = std::iter::repeat_n(prev.as_str(), 9).collect();
        bomb.push_str(&format!("a{}: &a{} [{}]\n", level, level, row.join(",")));
    }
    bomb.push_str("patterns: []\n");
    fs::write(dir.join("bomb.yaml"), &bomb).expect("fixture must write");

    let started = std::time::Instant::now();
    let loaded = load_external_patterns(&dir);
    let elapsed = started.elapsed();

    assert!(
        !loaded.errors.is_empty(),
        "an alias bomb must be surfaced as a load error, not silently expanded"
    );
    assert!(
        loaded.categories.is_empty(),
        "a bombed file must not contribute a category"
    );
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "refusal must be fast; took {elapsed:?}"
    );
}

// -------------------------------------------------------------- deep nesting

#[test]
fn deeply_nested_frontmatter_does_not_panic_the_parser() {
    // The projection's own `MAX_DEPTH` (12) only trims the walk *after* the
    // document is already fully parsed into a `Value` — it cannot protect
    // against a parser-level stack overflow while descending adversary-
    // controlled nesting. This pins that parsing 500 levels of nested flow
    // sequences completes without crashing the process, regardless of whether
    // the parser accepts or rejects it.
    let depth = 500;
    let mut body = String::from("deep: ");
    body.push_str(&"[".repeat(depth));
    body.push('1');
    body.push_str(&"]".repeat(depth));
    body.push('\n');
    let block = yaml_block(&body);

    let result = std::panic::catch_unwind(|| frontmatter::parse(&block));
    assert!(
        result.is_ok(),
        "deep nesting must be handled without panicking the parser thread"
    );
}

// -------------------------------------------------------------- empty documents

#[test]
fn an_empty_frontmatter_body_parses_to_null() {
    let block = yaml_block("");
    let value = frontmatter::parse(&block).expect("an empty YAML document is valid YAML (null)");
    assert!(value.is_null(), "an empty document must parse as null");
}

#[test]
fn an_empty_pattern_file_is_rejected_for_missing_required_fields() {
    // Unlike frontmatter's free-form `Value`, `PatternCategory` has required
    // fields with no defaults (`category`, `default_severity`, `patterns`), so
    // an empty document must fail schema validation rather than silently
    // producing a category with no patterns.
    let parsed: Result<PatternCategory, _> = serde_yaml::from_str("");
    assert!(
        parsed.is_err(),
        "an empty pattern file must not deserialize into a valid category"
    );
}

// -------------------------------------------------------------- unusual scalars

#[test]
fn boolean_and_numeric_looking_scalars_load_as_literal_strings_in_typed_fields() {
    // Measured, not assumed: this parser follows the YAML 1.2 *core schema*
    // for implicit typing, not YAML 1.1's — so the classic "Norway problem"
    // (`no` silently becoming boolean `false`) does not reproduce when
    // deserializing straight into a `String` field. `no`/`yes`/`on`/`off` and
    // a zero-padded `007` (not valid core-schema int syntax) all come through
    // as the literal string written, unquoted. If a replacement parser
    // instead followed YAML 1.1 resolution here, `id: no` would fail this
    // assertion by no longer round-tripping as `"no"`.
    let yaml =
        "category: t\ndefault_severity: LOW\npatterns:\n  - id: no\n    name: t\n    pattern: x\n";
    let parsed: PatternCategory =
        serde_yaml::from_str(yaml).expect("an unquoted `no` must load as the string \"no\"");
    assert_eq!(parsed.patterns[0].id, "no");

    let yaml =
        "category: t\ndefault_severity: LOW\npatterns:\n  - id: 007\n    name: t\n    pattern: x\n";
    let parsed: PatternCategory =
        serde_yaml::from_str(yaml).expect("an unquoted `007` must load as the string \"007\"");
    assert_eq!(parsed.patterns[0].id, "007");
}

#[test]
fn a_quoted_boolean_like_scalar_loads_as_the_literal_string() {
    // The unambiguous form: quoting always defeats implicit typing, regardless
    // of which core-schema rules the parser follows.
    let yaml =
        "category: t\ndefault_severity: LOW\npatterns:\n  - id: \"no\"\n    name: t\n    pattern: x\n";
    let parsed: PatternCategory =
        serde_yaml::from_str(yaml).expect("a quoted scalar must load as a plain string");
    assert_eq!(parsed.patterns[0].id, "no");
}

#[test]
fn frontmatter_projects_core_schema_booleans_but_not_norway_style_words() {
    // The security-relevant version of the scalar-typing question: frontmatter
    // parses into a free-form `Value`, where the parser — not a target
    // `String` field — decides the scalar's type. An attacker writing
    // `allowed-tools: no` intending the literal word must not have it silently
    // retyped to boolean `false` and change what a `scope: frontmatter`
    // pattern sees rendered. This pins the measured split: canonical
    // `true`/`false`/`null` resolve to their types, while `no`/`yes`/`on`/`off`
    // and a zero-padded `007` stay strings.
    let block = yaml_block(
        "canonical_true: true\ncanonical_null: null\nword_no: no\nword_yes: yes\npadded: 007\n",
    );
    let value = frontmatter::parse(&block).expect("valid YAML must parse");
    assert_eq!(value.get("canonical_true"), Some(&serde_json::json!(true)));
    assert_eq!(value.get("canonical_null"), Some(&serde_json::json!(null)));
    assert_eq!(
        value.get("word_no").and_then(|v| v.as_str()),
        Some("no"),
        "`no` must render as the literal word, not boolean false"
    );
    assert_eq!(
        value.get("word_yes").and_then(|v| v.as_str()),
        Some("yes"),
        "`yes` must render as the literal word, not boolean true"
    );
    assert_eq!(
        value.get("padded").and_then(|v| v.as_str()),
        Some("007"),
        "a zero-padded scalar must render as the literal digits, not a number"
    );
}
