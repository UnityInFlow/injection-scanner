use std::fs;
use std::io::Read;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};

use injection_scanner::allowlist::parse_suppressions;
use injection_scanner::pattern::ScanReport;
use injection_scanner::patterns::load_all_patterns;
use injection_scanner::reporter::{format_json, format_text};
use injection_scanner::scanner::Scanner;

#[derive(Parser)]
#[command(name = "injection-scanner")]
#[command(about = "Prompt injection static scanner for AI spec files, skills, and RAG documents")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Output formats the scanner can emit.
///
/// This is a `ValueEnum` on purpose. `--format` was previously a bare `String`
/// matched against `"json"`, with everything else falling through to text — so
/// `--format sarif` produced human-readable text with a findings exit code, and
/// `--format JSON` silently lost machine-readable output. Both surfaced as
/// malformed input to the *consumer* rather than as an error here. Clap now
/// rejects unknown values at parse time and lists the valid ones.
///
/// SARIF is deliberately absent until it is actually implemented (issue #5).
/// Adding the variant before the writer exists would recreate the same class of
/// silent failure this type was introduced to remove.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Human-readable output for terminals.
    Text,
    /// Machine-readable JSON for CI consumers.
    Json,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Scan files for prompt injection patterns
    Check {
        /// File or directory to scan (use - for stdin)
        path: String,
        /// Output format
        #[arg(long, value_enum, ignore_case = true, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        /// Additional patterns directory
        #[arg(long)]
        patterns: Option<PathBuf>,
    },
}

fn scan_file(path: &str, content: &str, scanner: &Scanner) -> ScanReport {
    let suppressions = parse_suppressions(content);
    scanner.scan(path, content, &suppressions)
}

fn walkdir(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in
        fs::read_dir(dir).with_context(|| format!("Failed to read directory {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if matches!(ext, "md" | "yaml" | "yml" | "txt" | "toml") {
                files.push(path);
            }
        } else if path.is_dir() {
            files.extend(walkdir(&path)?);
        }
    }
    Ok(files)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            path,
            format,
            patterns,
        } => {
            let categories =
                load_all_patterns(patterns.as_deref()).context("Failed to load patterns")?;

            // Compile the pattern set ONCE for the whole run. Doing this per file
            // is what broke the <200ms budget: compilation dominates, so cost
            // scaled with file count rather than content (issue #13).
            let scanner = Scanner::new(&categories).context("Failed to compile patterns")?;

            let mut reports = Vec::new();

            if path == "-" {
                let mut content = String::new();
                std::io::stdin()
                    .read_to_string(&mut content)
                    .context("Failed to read from stdin")?;
                reports.push(scan_file("<stdin>", &content, &scanner));
            } else {
                let target = PathBuf::from(&path);
                if target.is_file() {
                    let content = fs::read_to_string(&target)
                        .with_context(|| format!("Failed to read {}", target.display()))?;
                    reports.push(scan_file(&path, &content, &scanner));
                } else if target.is_dir() {
                    for entry in walkdir(&target)? {
                        let content = fs::read_to_string(&entry)
                            .with_context(|| format!("Failed to read {}", entry.display()))?;
                        reports.push(scan_file(&entry.to_string_lossy(), &content, &scanner));
                    }
                } else {
                    anyhow::bail!("Path does not exist: {}", path);
                }
            }

            // Exhaustive by construction — a new variant will not compile until
            // it has a writer, which is the point of the enum.
            let output = match format {
                OutputFormat::Json => format_json(&reports)?,
                OutputFormat::Text => format_text(&reports),
            };

            print!("{}", output);

            let has_findings = reports.iter().any(|r| r.has_findings());
            std::process::exit(if has_findings { 1 } else { 0 });
        }
    }
}
