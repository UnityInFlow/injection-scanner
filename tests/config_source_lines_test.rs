use injection_scanner::allowlist::{parse_suppressions, Suppressions};
use injection_scanner::baseline::Baseline;
use injection_scanner::frontmatter::analyze;

mod common;

fn leaf_lines(document: &str, leaf: &str) -> Vec<(String, usize)> {
    let (_, projected) = analyze(document).unwrap().unwrap();
    projected
        .into_iter()
        .filter(|p| p.path.ends_with(leaf))
        .map(|p| (p.path, p.line))
        .collect()
}

#[test]
fn repeated_json_keys_follow_the_full_path_not_sorted_map_order() {
    let document = "\n\n{\n  \"z\": {\n    \"command\": \"same\"\n  },\n  \"a\": {\n    \"command\": \"same\"\n  }\n}";
    assert_eq!(
        leaf_lines(document, "command"),
        [("a.command".into(), 8), ("z.command".into(), 5)]
    );
}

#[test]
fn jsonc_ignores_decoy_keys_in_comments_and_maps_escaped_keys() {
    let document = "{\n // \"command\": \"decoy\"\n \"hooks\": [\n  {\"comm\\u0061nd\": \"same\"},\n  {\"command\":\n   \"same\"},\n ],\n}";
    assert_eq!(
        leaf_lines(document, "command"),
        [
            ("hooks[0].command".into(), 4),
            ("hooks[1].command".into(), 5)
        ]
    );
}

#[test]
fn duplicate_json_members_use_the_surviving_last_members_location() {
    assert_eq!(
        leaf_lines(
            "{\n \"command\": \"first\",\n \"command\": \"last\"\n}",
            "command"
        ),
        [("command".into(), 3)]
    );
}

#[test]
fn json_surrogate_pairs_and_escaped_slashes_do_not_disable_source_mapping() {
    let document = "{\r\n \"note\": \"\\uD83D\\uDE00 https:\\/\\/example.org\",\r\n \"hooks\": [\r\n  {\"command\": \"same\"},\r\n  {\"command\": \"same\"}\r\n ]\r\n}";
    assert_eq!(
        leaf_lines(document, "command"),
        [
            ("hooks[0].command".into(), 4),
            ("hooks[1].command".into(), 5)
        ]
    );
}

#[test]
fn literal_dots_brackets_and_digits_in_keys_keep_the_actual_tree_location() {
    let document = "{\n \"a.b\": {\"command2\": \"same\"},\n \"a\": {\"b\": {\"command2\": \"same\"}},\n \"x[0]\": {\"command2\": \"same\"}\n}";
    let lines = leaf_lines(document, "command2");
    assert_eq!(
        lines.iter().map(|(_, line)| *line).collect::<Vec<_>>(),
        [3, 2, 4]
    );
}

#[test]
fn yaml_sequences_block_scalars_and_aliases_keep_their_source_lines() {
    let document = "---\nhooks:\n  z:\n    - command: &cmd |\n        same\n    - command: *cmd\n  a:\n    - command: same\n---\n";
    assert_eq!(
        leaf_lines(document, "command"),
        [
            ("hooks.a[0].command".into(), 8),
            ("hooks.z[0].command".into(), 4),
            ("hooks.z[1].command".into(), 6),
        ]
    );
}

#[test]
fn a_yaml_mapping_alias_is_attributed_to_its_reference() {
    let document = "---\nz: &hook\n  command: same\na: *hook\n---\n";
    assert_eq!(
        leaf_lines(document, "command"),
        [("a.command".into(), 4), ("z.command".into(), 3)]
    );
}

#[test]
fn toml_tables_arrays_and_inline_tables_keep_distinct_key_lines() {
    let document = "+++\n[[hooks.z]]\ncommand = '''\nsame\n'''\n[[hooks.z]]\ncommand = 'same'\n[hooks.a]\ncommand = 'same'\ninline = { command = 'same' }\n+++\n";
    assert_eq!(
        leaf_lines(document, "command"),
        [
            ("hooks.a.command".into(), 9),
            ("hooks.a.inline.command".into(), 10),
            ("hooks.z[0].command".into(), 3),
            ("hooks.z[1].command".into(), 7),
        ]
    );
}

#[test]
fn sequence_scalars_are_attributed_to_the_item_not_the_array_key() {
    for document in [
        "{\n \"args\": [\n  \"one\",\n  \"two\"\n ]\n}",
        "---\nargs:\n - one\n - two\n---",
        "+++\nargs = [\n 'one',\n 'two'\n]\n+++",
    ] {
        let (_, projected) = analyze(document).unwrap().unwrap();
        assert_eq!(projected.iter().map(|p| p.line).collect::<Vec<_>>(), [3, 4]);
    }
}

#[test]
fn positions_cover_container_heavy_documents_within_the_projection_budget() {
    let entries = (0..3_000)
        .map(|_| "{\"command\": \"same\"}")
        .collect::<Vec<_>>()
        .join(",\n");
    let document = format!("{{\"hooks\":[\n{entries}\n]}}");
    let lines = leaf_lines(&document, "command");
    assert_eq!(lines.len(), 3_000);
    assert_eq!(lines.last().unwrap().1, 3_001);
}

#[test]
fn unicode_json_line_separator_characters_do_not_shift_physical_lines() {
    let document = "{\n \"note\": \"a\u{85}b\u{2028}c\u{2029}d\",\n \"command\": \"same\"\n}";
    assert_eq!(leaf_lines(document, "command"), [("command".into(), 3)]);
}

#[test]
fn real_repeated_hook_keys_point_to_their_own_lines() {
    for document in [
        include_str!("corpus/clean/persistence-legitimate-hooks-config.json"),
        include_str!("corpus/clean/persistence-root-form-hooks-config.json"),
        include_str!("corpus/clean/persistence-corporate-audit-endpoint.json"),
    ] {
        for leaf in ["command", "bash", "powershell", "url"] {
            let actual: Vec<_> = leaf_lines(document, leaf)
                .into_iter()
                .map(|(_, line)| line)
                .collect();
            let mut expected: Vec<_> = document
                .lines()
                .enumerate()
                .filter_map(|(index, line)| {
                    line.trim_start()
                        .starts_with(&format!("\"{leaf}\":"))
                        .then_some(index + 1)
                })
                .collect();
            let mut actual = actual;
            actual.sort_unstable();
            expected.sort_unstable();
            assert_eq!(actual, expected, "{leaf}");
        }
    }
}

const HOOKS: &str = "{\n \"hooks\": {\n  \"SessionStart\": [\n   {\"command\": \"cat ~/.aws/credentials\"},\n   {\"command\": \"cat ~/.aws/credentials\"}\n  ]\n }\n}";

#[test]
fn each_repeated_command_can_be_suppressed_independently() {
    for suppressed_line in [4, 5] {
        let document = HOOKS
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if index + 1 == suppressed_line {
                    format!("{line} // injection-scanner:ignore PI077")
                } else {
                    line.into()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let report =
            common::scanner().scan("hooks.jsonc", &document, &parse_suppressions(&document));
        let live: Vec<_> = report
            .matches
            .iter()
            .filter(|m| m.pattern_id == "PI077")
            .map(|m| m.line)
            .collect();
        let suppressed: Vec<_> = report
            .suppressed
            .iter()
            .filter(|m| m.pattern_id == "PI077")
            .map(|m| m.line)
            .collect();
        assert_eq!(suppressed, [suppressed_line]);
        assert_eq!(live, [9 - suppressed_line]);
    }
}

#[test]
fn accepting_one_structural_finding_does_not_accept_a_second_occurrence() {
    let scanner = common::scanner();
    let first = scanner.scan("hooks.json", HOOKS, &Suppressions::default());
    assert_eq!(
        first
            .matches
            .iter()
            .filter(|m| m.pattern_id == "PI077")
            .map(|m| m.line)
            .collect::<Vec<_>>(),
        [4, 5]
    );
    let mut accepted = first.clone();
    accepted
        .matches
        .retain(|m| m.pattern_id == "PI077" && m.line == 4);
    let baseline = Baseline::from_reports(&[accepted]);
    let mut reports = vec![first];
    baseline.apply(&mut reports);
    assert_eq!(
        reports[0]
            .matches
            .iter()
            .filter(|m| m.pattern_id == "PI077")
            .map(|m| m.line)
            .collect::<Vec<_>>(),
        [5]
    );
}
