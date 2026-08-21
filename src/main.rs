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

/// Human-readable reason a file could not be read.
///
/// `io::Error`'s own Display for an encoding failure is "stream did not contain
/// valid UTF-8", which reads like a scanner bug rather than a skipped binary.
fn describe_read_error(e: &std::io::Error) -> String {
    match e.kind() {
        std::io::ErrorKind::InvalidData => {
            "not valid UTF-8 (binary or non-UTF-8 encoding)".to_string()
        }
        std::io::ErrorKind::PermissionDenied => "permission denied".to_string(),
        std::io::ErrorKind::NotFound => "not found (broken symlink?)".to_string(),
        _ => e.to_string(),
    }
}

fn scan_file(path: &str, content: &str, scanner: &Scanner) -> ScanReport {
    let suppressions = parse_suppressions(content);
    scanner.scan(path, content, &suppressions)
}

/// Collect scannable files under `dir`, isolating per-directory failures.
///
/// An unreadable subdirectory is skipped and counted, not propagated: a single
/// permission-denied directory previously aborted the whole walk, which is the
/// same defect as the per-file case in issue #14 one level up. Failure to read
/// the directory the user actually named is still a hard error — that one is
/// reported by the caller.
fn walkdir(dir: &PathBuf, skipped: &mut Vec<(PathBuf, String)>) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries =
        fs::read_dir(dir).with_context(|| format!("Failed to read directory {}", dir.display()))?;

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                skipped.push((dir.clone(), describe_read_error(&e)));
                continue;
            }
        };
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if matches!(ext, "md" | "yaml" | "yml" | "txt" | "toml") {
                files.push(path);
            }
        } else if path.is_dir() {
            match walkdir(&path, skipped) {
                Ok(nested) => files.extend(nested),
                Err(e) => skipped.push((path, root_cause(&e))),
            }
        }
    }
    Ok(files)
}

/// Innermost message of an `anyhow` chain, for a one-line skip warning.
fn root_cause(e: &anyhow::Error) -> String {
    e.chain()
        .last()
        .map_or_else(|| e.to_string(), |c| c.to_string())
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
            // Embedded patterns are compile-time constants covered by a CI test,
            // so a failure there is a bug here and should stop the run. External
            // --patterns directories are an untrusted input surface: one malformed
            // YAML file must not deny service to every scan. Dropped patterns are
            // reported loudly so coverage is never lost silently.
            let scanner = if patterns.is_some() {
                let (scanner, errors) = Scanner::new_lenient(&categories);
                for e in &errors {
                    eprintln!("warning: pattern skipped — {e}");
                }
                if !errors.is_empty() {
                    eprintln!(
                        "warning: {} pattern(s) failed to compile and are NOT being applied",
                        errors.len()
                    );
                }
                scanner
            } else {
                Scanner::new(&categories).context("Failed to compile embedded patterns")?
            };

            let mut reports = Vec::new();
            let mut skipped: usize = 0;

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
                    let mut skipped_dirs: Vec<(PathBuf, String)> = Vec::new();
                    // Per-file error isolation. Previously a single unreadable or
                    // non-UTF-8 file propagated with `?` and killed the entire
                    // walk, which in CI reads as a scanner crash rather than a
                    // result. Skips are surfaced on stderr so nothing is silently
                    // left unscanned (issue #14).
                    //
                    // NOTE: skips are NOT added to the JSON output. The top-level
                    // shape must stay an array — spec-ci-plugin does
                    // `JSON.parse(output) as Array<...>`. A JSON envelope carrying
                    // `skipped` is deferred to v0.1.0 as a coordinated breaking
                    // change (audit L-02).
                    let walked = walkdir(&target, &mut skipped_dirs)?;
                    for (dir, reason) in &skipped_dirs {
                        skipped += 1;
                        eprintln!("warning: skipped directory {} — {}", dir.display(), reason);
                    }
                    for entry in walked {
                        match fs::read_to_string(&entry) {
                            Ok(content) => reports.push(scan_file(
                                &entry.to_string_lossy(),
                                &content,
                                &scanner,
                            )),
                            Err(e) => {
                                skipped += 1;
                                eprintln!(
                                    "warning: skipped {} — {}",
                                    entry.display(),
                                    describe_read_error(&e)
                                );
                            }
                        }
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

            if skipped > 0 {
                eprintln!(
                    "warning: {} file(s) skipped and NOT scanned — see warnings above",
                    skipped
                );
            }

            let has_findings = reports.iter().any(|r| r.has_findings());
            std::process::exit(if has_findings { 1 } else { 0 });
        }
    }
}
