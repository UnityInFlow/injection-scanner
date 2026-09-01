use crate::pattern::{PatternCategory, PatternError, Severity};

/// Pattern categories that loaded, alongside the failures that did not.
///
/// Separating these is what lets schema errors respect the same strict/lenient
/// policy as regex-compilation errors. Previously `deny_unknown_fields` was
/// enforced at parse time and returned `Err` immediately, so a single unknown
/// key in a community file aborted every scan — bypassing `--strict-patterns`
/// entirely and re-opening the denial-of-service that lenient loading closed.
#[derive(Debug, Default)]
pub struct LoadedPatterns {
    pub categories: Vec<PatternCategory>,
    pub errors: Vec<PatternError>,
}

const ROLE_OVERRIDE_YAML: &str = include_str!("../../patterns/core/role-override.yaml");
const INSTRUCTION_YAML: &str = include_str!("../../patterns/core/instruction-injection.yaml");
const EXFILTRATION_YAML: &str = include_str!("../../patterns/core/exfiltration.yaml");
const JAILBREAK_YAML: &str = include_str!("../../patterns/core/jailbreak.yaml");
const ENCODING_YAML: &str = include_str!("../../patterns/core/encoding.yaml");
const TOOL_PERMISSION_ABUSE_YAML: &str =
    include_str!("../../patterns/core/tool-permission-abuse.yaml");

/// Load all embedded (compile-time) pattern categories.
///
/// These patterns are baked into the binary via `include_str!` and
/// require no external files at runtime.
pub fn load_embedded_patterns() -> Result<Vec<PatternCategory>, PatternError> {
    let yamls = [
        ROLE_OVERRIDE_YAML,
        INSTRUCTION_YAML,
        EXFILTRATION_YAML,
        JAILBREAK_YAML,
        ENCODING_YAML,
        TOOL_PERMISSION_ABUSE_YAML,
    ];

    yamls
        .iter()
        .map(|yaml| {
            serde_yaml::from_str::<PatternCategory>(yaml)
                .map_err(|e| PatternError::ParseError(e.to_string()))
        })
        .collect()
}

/// Load additional patterns from an external directory.
///
/// Returns an empty result if the directory does not exist, allowing optional
/// community pattern overlays.
///
/// **Per-file error isolation.** A file that fails to read or parse is collected
/// into `errors` rather than aborting the load. External patterns are an
/// untrusted input surface: one malformed community YAML must not deny service
/// to every scan. Callers decide whether to warn or fail — see
/// [`LoadedPatterns`] and `--strict-patterns`.
pub fn load_external_patterns(dir: &std::path::Path) -> LoadedPatterns {
    let mut loaded = LoadedPatterns::default();

    if !dir.exists() {
        return loaded;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            loaded.errors.push(PatternError::ParseError(format!(
                "{}: {}",
                dir.display(),
                e
            )));
            return loaded;
        }
    };

    for entry in entries {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(e) => {
                loaded.errors.push(PatternError::ParseError(format!(
                    "{}: {}",
                    dir.display(),
                    e
                )));
                continue;
            }
        };

        if !path
            .extension()
            .is_some_and(|ext| ext == "yaml" || ext == "yml")
        {
            continue;
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                loaded.errors.push(PatternError::ParseError(format!(
                    "{}: {}",
                    path.display(),
                    e
                )));
                continue;
            }
        };

        match serde_yaml::from_str::<PatternCategory>(&content) {
            Ok(category) => loaded.categories.push(category),
            Err(e) => loaded.errors.push(PatternError::ParseError(format!(
                "{}: {}",
                path.display(),
                e
            ))),
        }
    }

    loaded
}

/// Load embedded patterns plus optional external patterns.
///
/// Embedded patterns are compile-time constants covered by a CI test, so a
/// failure there is a bug in this repository and is returned as `Err`. External
/// patterns are untrusted, so their failures are collected in
/// [`LoadedPatterns::errors`] for the caller to warn about or fail on.
pub fn load_all_patterns(
    external_dir: Option<&std::path::Path>,
) -> Result<LoadedPatterns, PatternError> {
    let mut loaded = LoadedPatterns {
        categories: load_embedded_patterns()?,
        errors: Vec::new(),
    };

    if let Some(dir) = external_dir {
        let external = load_external_patterns(dir);
        loaded.categories.extend(external.categories);
        loaded.errors.extend(external.errors);
    }

    Ok(loaded)
}

/// A pattern with its severity already resolved against the category default.
///
/// `rules`, `explain` and the SARIF writer (`src/sarif.rs`, CLI-04) all need
/// the EFFECTIVE severity — the one a user or a code-scanning consumer will
/// actually see — not the optional per-pattern override. Moved here (from
/// `src/main.rs`) verbatim, field order and the `Serialize` derive included,
/// because `rules --format json` serializes it and that output must not
/// change.
#[derive(Debug, Clone, serde::Serialize)]
pub struct GradedRule {
    pub id: String,
    pub name: String,
    pub severity: Severity,
    pub category: String,
    pub description: String,
    pub remediation: String,
    pub pattern: String,
    pub tags: Vec<String>,
}

/// Resolves the effective severity of every pattern in `categories`, sorted
/// by id.
///
/// Pure — no I/O, no stderr output — so it is safe to call from the SARIF
/// writer as well as from the CLI. Loading patterns from disk and printing
/// the per-file load warnings remain the caller's responsibility; see
/// `load_graded` in `src/main.rs`, the thin wrapper that does both and then
/// calls this.
pub fn grade(categories: &[PatternCategory]) -> Vec<GradedRule> {
    let mut rules: Vec<GradedRule> = categories
        .iter()
        .flat_map(|category| {
            category.patterns.iter().map(move |p| GradedRule {
                id: p.id.clone(),
                name: p.name.clone(),
                severity: p.severity.unwrap_or(category.default_severity),
                category: category.category.clone(),
                description: p.description.clone(),
                remediation: p.remediation.clone(),
                pattern: p.pattern.clone(),
                tags: p.tags.clone(),
            })
        })
        .collect();
    // Sorted by id so the listing is stable and diffable.
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    rules
}
