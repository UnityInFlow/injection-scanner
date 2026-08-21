use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use regex::Regex;

/// How many lines from the top of a file `ignore-file` may be declared in.
///
/// Bounded so a suppression buried at line 900 of a document cannot silently
/// disable a rule for everything above it — a file-wide escape hatch should be
/// visible in the first screenful.
const IGNORE_FILE_HEADER_LINES: usize = 10;

static SUPPRESSION_RE: OnceLock<Regex> = OnceLock::new();

/// Matches all three suppression directives and captures the pattern-ID list.
///
/// The ID pattern is deliberately broader than `PI\d+`: community pattern packs
/// use their own prefixes, and the previous hard-coded `PI` meant a contributed
/// pattern could never be suppressed at all.
fn suppression_regex() -> &'static Regex {
    SUPPRESSION_RE.get_or_init(|| {
        Regex::new(
            r"injection-scanner:(ignore-next-line|ignore-file|ignore)\s+([A-Za-z][A-Za-z0-9_-]*(?:\s*,\s*[A-Za-z][A-Za-z0-9_-]*)*)",
        )
        .expect("suppression regex is a compile-time constant and is covered by a unit test")
    })
}

/// Parsed suppression directives for one file.
///
/// Three forms, because the previous single form contradicted its own
/// documentation: the README described `ignore` as applying to the *next* line
/// while the implementation only honoured the *same* line, so anyone following
/// the docs got no suppression at all (audit C-04).
#[derive(Debug, Default, Clone)]
pub struct Suppressions {
    /// `injection-scanner:ignore <ids>` — applies to the line it appears on.
    same_line: HashMap<usize, Vec<String>>,
    /// `injection-scanner:ignore-next-line <ids>` — applies to the following line.
    next_line: HashMap<usize, Vec<String>>,
    /// `injection-scanner:ignore-file <ids>` — applies to the whole file.
    file_wide: HashSet<String>,
}

impl Suppressions {
    /// Is `pattern_id` suppressed on this 1-based line?
    pub fn is_suppressed(&self, line: usize, pattern_id: &str) -> bool {
        if self.file_wide.contains(pattern_id) {
            return true;
        }
        let on = |map: &HashMap<usize, Vec<String>>, key: usize| {
            map.get(&key)
                .is_some_and(|ids| ids.iter().any(|id| id == pattern_id))
        };
        on(&self.same_line, line) || on(&self.next_line, line.saturating_sub(1))
    }

    /// Pattern IDs suppressed for the entire file.
    pub fn file_wide_ids(&self) -> impl Iterator<Item = &str> {
        self.file_wide.iter().map(String::as_str)
    }

    /// True when no directive of any kind was found.
    pub fn is_empty(&self) -> bool {
        self.same_line.is_empty() && self.next_line.is_empty() && self.file_wide.is_empty()
    }
}

/// Parse every suppression directive in `content`.
///
/// Line numbers are 1-based. A line may carry more than one directive.
pub fn parse_suppressions(content: &str) -> Suppressions {
    let re = suppression_regex();
    let mut out = Suppressions::default();

    for (line_index, line) in content.lines().enumerate() {
        let line_number = line_index + 1;

        for caps in re.captures_iter(line) {
            let ids: Vec<String> = caps[2].split(',').map(|s| s.trim().to_string()).collect();

            match &caps[1] {
                "ignore" => out.same_line.entry(line_number).or_default().extend(ids),
                "ignore-next-line" => out.next_line.entry(line_number).or_default().extend(ids),
                "ignore-file" => {
                    // Only honoured near the top of the file; see the constant.
                    if line_number <= IGNORE_FILE_HEADER_LINES {
                        out.file_wide.extend(ids);
                    }
                }
                // The regex alternation has exactly these three arms, so this is
                // unreachable in practice; treated as "no directive" rather than
                // panicking on a future regex edit.
                other => debug_assert!(false, "unhandled suppression directive: {other}"),
            }
        }
    }

    out
}
