use std::fs;
use std::io::Read;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use injection_scanner::allowlist::parse_suppressions;
use injection_scanner::pattern::{PatternCategory, ScanReport};
use injection_scanner::patterns::load_all_patterns;
use injection_scanner::reporter::{format_json, format_text};
use injection_scanner::scanner::scan_content;

#[derive(Parser)]
#[command(name = "injection-scanner")]
#[command(about = "Prompt injection static scanner for AI spec files, skills, and RAG documents")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Scan files for prompt injection patterns
    Check {
        /// File or directory to scan (use - for stdin)
        path: String,
        /// Output format: text or json
        #[arg(long, default_value = "text")]
        format: String,
        /// Additional patterns directory
        #[arg(long)]
        patterns: Option<PathBuf>,
    },
}

fn scan_file(path: &str, content: &str, categories: &[PatternCategory]) -> ScanReport {
    let suppressions = parse_suppressions(content);
    scan_content(path, content, categories, &suppressions)
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

            let mut reports = Vec::new();

            if path == "-" {
                let mut content = String::new();
                std::io::stdin()
                    .read_to_string(&mut content)
                    .context("Failed to read from stdin")?;
                reports.push(scan_file("<stdin>", &content, &categories));
            } else {
                let target = PathBuf::from(&path);
                if target.is_file() {
                    let content = fs::read_to_string(&target)
                        .with_context(|| format!("Failed to read {}", target.display()))?;
                    reports.push(scan_file(&path, &content, &categories));
                } else if target.is_dir() {
                    for entry in walkdir(&target)? {
                        let content = fs::read_to_string(&entry)
                            .with_context(|| format!("Failed to read {}", entry.display()))?;
                        reports.push(scan_file(&entry.to_string_lossy(), &content, &categories));
                    }
                } else {
                    anyhow::bail!("Path does not exist: {}", path);
                }
            }

            let output = match format.as_str() {
                "json" => format_json(&reports)?,
                _ => format_text(&reports),
            };

            print!("{}", output);

            let has_findings = reports.iter().any(|r| r.has_findings());
            std::process::exit(if has_findings { 1 } else { 0 });
        }
    }
}
