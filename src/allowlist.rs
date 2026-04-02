use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

static SUPPRESSION_RE: OnceLock<Regex> = OnceLock::new();

fn suppression_regex() -> &'static Regex {
    SUPPRESSION_RE
        .get_or_init(|| Regex::new(r"injection-scanner:ignore\s+(PI\d+(?:\s*,\s*PI\d+)*)").unwrap())
}

/// Parse inline suppressions from content.
///
/// Scans each line for `<!-- injection-scanner:ignore PI001 -->` comments
/// and returns a map of `line_number -> Vec<pattern_id>`.
/// Line numbers are 1-based.
pub fn parse_suppressions(content: &str) -> HashMap<usize, Vec<String>> {
    let re = suppression_regex();
    let mut suppressions = HashMap::new();

    for (line_num, line) in content.lines().enumerate() {
        if let Some(caps) = re.captures(line) {
            let ids: Vec<String> = caps[1].split(',').map(|s| s.trim().to_string()).collect();
            suppressions.insert(line_num + 1, ids);
        }
    }

    suppressions
}

/// Check if a specific pattern is suppressed on a given line.
///
/// Returns `true` only if the exact `pattern_id` appears in the
/// suppression list for that line number — suppression is per-pattern,
/// not file-global.
pub fn is_suppressed(
    suppressions: &HashMap<usize, Vec<String>>,
    line: usize,
    pattern_id: &str,
) -> bool {
    suppressions
        .get(&line)
        .is_some_and(|ids| ids.iter().any(|id| id == pattern_id))
}
