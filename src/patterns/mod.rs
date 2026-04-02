use crate::pattern::{PatternCategory, PatternError};

const ROLE_OVERRIDE_YAML: &str = include_str!("../../patterns/core/role-override.yaml");
const INSTRUCTION_YAML: &str = include_str!("../../patterns/core/instruction-injection.yaml");
const EXFILTRATION_YAML: &str = include_str!("../../patterns/core/exfiltration.yaml");
const JAILBREAK_YAML: &str = include_str!("../../patterns/core/jailbreak.yaml");
const ENCODING_YAML: &str = include_str!("../../patterns/core/encoding.yaml");

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
/// Returns an empty `Vec` if the directory does not exist,
/// allowing optional community pattern overlays.
pub fn load_external_patterns(dir: &std::path::Path) -> Result<Vec<PatternCategory>, PatternError> {
    let mut categories = Vec::new();

    if !dir.exists() {
        return Ok(categories);
    }

    for entry in std::fs::read_dir(dir).map_err(|e| PatternError::ParseError(e.to_string()))? {
        let entry = entry.map_err(|e| PatternError::ParseError(e.to_string()))?;
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|ext| ext == "yaml" || ext == "yml")
        {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| PatternError::ParseError(format!("{}: {}", path.display(), e)))?;
            let category: PatternCategory = serde_yaml::from_str(&content)
                .map_err(|e| PatternError::ParseError(format!("{}: {}", path.display(), e)))?;
            categories.push(category);
        }
    }

    Ok(categories)
}

/// Load embedded patterns plus optional external patterns.
///
/// This is the primary entry point for pattern loading. External
/// patterns extend (not replace) the embedded set.
pub fn load_all_patterns(
    external_dir: Option<&std::path::Path>,
) -> Result<Vec<PatternCategory>, PatternError> {
    let mut categories = load_embedded_patterns()?;

    if let Some(dir) = external_dir {
        let external = load_external_patterns(dir)?;
        categories.extend(external);
    }

    Ok(categories)
}
