//! CLI entry point.
//!
//! `unwrap()` is denied here as well as in the library — this is a separate
//! crate, so `lib.rs`'s attribute does not reach it. See the note there.
#![deny(clippy::unwrap_used)]

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};

use injection_scanner::allowlist::{parse_suppressions, Suppressions};
use injection_scanner::context::DEFAULT_MIN_CONFIDENCE;
use injection_scanner::pattern::{ScanReport, Severity};
use injection_scanner::patterns::load_all_patterns;
use injection_scanner::reporter::{format_json, format_text};
use injection_scanner::scanner::Scanner;
use injection_scanner::walk::{walk, SkipReason, WalkOptions, DEFAULT_MAX_FILE_SIZE};

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

/// Severity at or above which findings fail the build.
///
/// Separate from `Severity` on purpose: this is a CLI concept with an ordering
/// and a `clap::ValueEnum` derive, and giving the domain type a CLI parser would
/// couple the pattern library to the argument parser.
#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
enum FailOn {
    Critical,
    High,
    Medium,
    Low,
}

impl FailOn {
    /// Does a finding at `severity` meet this bar?
    fn is_met_by(self, severity: Severity) -> bool {
        let rank = |s| match s {
            Severity::Critical => 3,
            Severity::High => 2,
            Severity::Medium => 1,
            Severity::Low => 0,
        };
        let bar = match self {
            FailOn::Critical => 3,
            FailOn::High => 2,
            FailOn::Medium => 1,
            FailOn::Low => 0,
        };
        rank(severity) >= bar
    }
}

/// Exit codes, which are this tool's real interface to CI.
///
/// `2` exists so "we found things, none met your bar" is distinguishable from
/// "clean". Collapsing those two would make `--fail-on critical` silently hide
/// every HIGH finding from a pipeline that only checks for zero. Matches the
/// convention `spec-linter` already uses in this ecosystem.
mod exit {
    pub const CLEAN: i32 = 0;
    pub const FAILED: i32 = 1;
    pub const BELOW_THRESHOLD: i32 = 2;
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
        /// Fail instead of warning when a pattern is invalid or has a duplicate id
        #[arg(long)]
        strict_patterns: bool,
        /// Ignore all in-file suppression directives
        ///
        /// Suppression is a control the SCANNED DOCUMENT invokes, so when the
        /// document is untrusted — a downloaded skill, a RAG corpus, a fork's
        /// pull request — its author can disarm the scanner. Use this whenever
        /// you did not write the file you are scanning.
        #[arg(long)]
        no_suppress: bool,
        /// Report findings regardless of where they appear in the document
        ///
        /// By default a payload quoted inside a fenced code block, an inline
        /// `code` span or a markdown table scores below the confidence
        /// threshold and is not reported, because that is what documentation
        /// about injection looks like. `--strict` puts them all back. Use it
        /// when the document is untrusted: a fenced block is still text a model
        /// reads.
        #[arg(long)]
        strict: bool,
        /// Minimum confidence to report, 0.0-1.0 (default 0.5)
        ///
        /// Overridden by `--strict`, which is equivalent to 0.0.
        #[arg(long, value_name = "SCORE")]
        min_confidence: Option<f32>,

        // ---- directory walking (issue #22) ----
        /// Skip paths matching this glob (repeatable)
        ///
        /// Applied on top of the unconditional deny-list, which already covers
        /// `.git`, `target`, `node_modules` and friends.
        #[arg(long, value_name = "GLOB")]
        exclude: Vec<String>,
        /// Scan paths matching this glob whatever their extension (repeatable)
        ///
        /// Cannot override the unconditional deny-list — `--include '**/*.json'`
        /// will not pull in `target/`.
        #[arg(long, value_name = "GLOB")]
        include: Vec<String>,
        /// Do not honour .gitignore
        ///
        /// Means "do not trust this repository's ignore rules", which is
        /// reasonable on a checkout you did not write. It does NOT disable the
        /// built-in deny-list: nothing agent-facing lives in `target/`.
        #[arg(long)]
        no_ignore: bool,
        /// Skip files larger than this many bytes
        #[arg(long, value_name = "BYTES", default_value_t = DEFAULT_MAX_FILE_SIZE)]
        max_file_size: u64,
        /// Follow symlinks (off by default — a symlink loop is a hang)
        #[arg(long)]
        follow_symlinks: bool,
        /// Traversal threads (0 = choose automatically)
        #[arg(long, value_name = "N", default_value_t = 0)]
        jobs: usize,
        /// Fail only at or above this severity
        ///
        /// Below it, findings are still printed but the exit code is 2 rather
        /// than 1 — "there is something here, it did not meet your bar".
        #[arg(long, value_enum, ignore_case = true, default_value_t = FailOn::Low)]
        fail_on: FailOn,
        /// Print nothing; communicate through the exit code alone
        ///
        /// For hooks and CI steps that only branch on the result.
        #[arg(long, short)]
        quiet: bool,
        /// Scan every file, not just known agent-facing types
        ///
        /// Still honours the deny-list, .gitignore and the size cap, and skips
        /// anything with a NUL byte in its first block. Use it on a corpus you
        /// did not assemble, where the extension tells you nothing.
        #[arg(long)]
        all_files: bool,
    },
    /// List every loaded pattern
    Rules {
        /// Additional patterns directory
        #[arg(long)]
        patterns: Option<PathBuf>,
        /// Output format
        #[arg(long, value_enum, ignore_case = true, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Show everything known about one pattern
    Explain {
        /// Pattern id, e.g. PI001 (case-insensitive)
        id: String,
        /// Additional patterns directory
        #[arg(long)]
        patterns: Option<PathBuf>,
    },
}

/// Resolve the confidence floor from the two flags that can set it.
///
/// `--strict` wins over `--min-confidence`: it is the "show me everything"
/// escape hatch, and having it silently lose to a stricter number would be a
/// surprising way to miss a finding.
fn resolve_min_confidence(strict: bool, min_confidence: Option<f32>) -> Result<f32> {
    if strict {
        return Ok(0.0);
    }
    match min_confidence {
        None => Ok(DEFAULT_MIN_CONFIDENCE),
        Some(value) if (0.0..=1.0).contains(&value) => Ok(value),
        Some(value) => Err(anyhow::anyhow!(
            "--min-confidence must be between 0.0 and 1.0, got {value}"
        )),
    }
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

fn scan_file(
    path: &str,
    content: &str,
    scanner: &Scanner,
    no_suppress: bool,
    min_confidence: f32,
) -> ScanReport {
    let suppressions = if no_suppress {
        Suppressions::default()
    } else {
        parse_suppressions(content)
    };
    scanner.scan_with_confidence(path, content, &suppressions, min_confidence)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            path,
            format,
            patterns,
            strict_patterns,
            no_suppress,
            strict,
            min_confidence,
            exclude,
            include,
            no_ignore,
            max_file_size,
            follow_symlinks,
            jobs,
            all_files,
            fail_on,
            quiet,
        } => {
            let min_confidence = resolve_min_confidence(strict, min_confidence)?;
            let loaded = load_all_patterns(patterns.as_deref())
                .context("Failed to load embedded patterns")?;
            let categories = loaded.categories;
            // Schema failures from external files travel the same strict/lenient
            // path as regex-compilation failures. Enforcing them at parse time
            // made --strict-patterns inert and let one malformed community file
            // abort every scan.
            let mut load_errors = loaded.errors;

            // Compile the pattern set ONCE for the whole run. Doing this per file
            // is what broke the <200ms budget: compilation dominates, so cost
            // scaled with file count rather than content (issue #13).
            // Embedded patterns are compile-time constants covered by a CI test,
            // so a failure there is a bug here and should stop the run. External
            // --patterns directories are an untrusted input surface: one malformed
            // YAML file must not deny service to every scan. Dropped patterns are
            // reported loudly so coverage is never lost silently.
            let scanner = if patterns.is_some() && !strict_patterns {
                let (scanner, errors) = Scanner::new_lenient(&categories);
                load_errors.extend(errors);
                for e in &load_errors {
                    eprintln!("warning: pattern skipped — {e}");
                }
                if !load_errors.is_empty() {
                    eprintln!(
                        "warning: {} pattern(s) were rejected and are NOT being applied",
                        load_errors.len()
                    );
                }
                scanner
            } else if let Some(first) = load_errors.into_iter().next() {
                // strict, and an external file already failed to parse
                return Err(anyhow::Error::new(first).context(
                    "Pattern validation failed (remove --strict-patterns to warn and continue)",
                ));
            } else {
                Scanner::new(&categories).context(if strict_patterns {
                    "Pattern validation failed (--strict-patterns)"
                } else {
                    "Failed to compile embedded patterns"
                })?
            };

            let mut reports = Vec::new();
            let mut skipped: usize = 0;

            if path == "-" {
                let mut content = String::new();
                std::io::stdin()
                    .read_to_string(&mut content)
                    .context("Failed to read from stdin")?;
                reports.push(scan_file(
                    "<stdin>",
                    &content,
                    &scanner,
                    no_suppress,
                    min_confidence,
                ));
            } else {
                let target = PathBuf::from(&path);
                if target.is_file() {
                    let content = fs::read_to_string(&target)
                        .with_context(|| format!("Failed to read {}", target.display()))?;
                    reports.push(scan_file(
                        &path,
                        &content,
                        &scanner,
                        no_suppress,
                        min_confidence,
                    ));
                } else if target.is_dir() {
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
                    let options = WalkOptions {
                        excludes: exclude,
                        includes: include,
                        respect_gitignore: !no_ignore,
                        max_file_size,
                        follow_symlinks,
                        jobs,
                        all_files,
                    };
                    let walked = walk(&target, &options)?;

                    // Two classes, reported differently on purpose. An
                    // unreadable entry or an oversized file is a gap the user
                    // may want to close, and is named individually. "Not a
                    // scanned file type" is the overwhelming majority on any
                    // real tree — naming each one would bury the first class in
                    // thousands of lines — so it is summarised instead. Neither
                    // is silent: a scanner that says "clean" about files it
                    // never opened is worse than one that says nothing.
                    let mut unscanned_types = 0usize;
                    for entry in &walked.skipped {
                        if entry.reason == SkipReason::UnscannedType {
                            unscanned_types += 1;
                            continue;
                        }
                        skipped += 1;
                        eprintln!(
                            "warning: skipped {} — {}",
                            entry.path.display(),
                            entry.reason.describe()
                        );
                    }
                    if walked.ignore_rules_applied {
                        eprintln!(
                            "note: .gitignore rules were applied — paths they exclude were not \
                             scanned and are not counted above. Use --no-ignore to include them."
                        );
                    }
                    if unscanned_types > 0 {
                        eprintln!(
                            "note: {unscanned_types} file(s) not scanned — not a scanned file \
                             type. Use --include <glob> to add them."
                        );
                    }

                    for entry in walked.files {
                        match fs::read_to_string(&entry) {
                            Ok(content) => reports.push(scan_file(
                                &entry.to_string_lossy(),
                                &content,
                                &scanner,
                                no_suppress,
                                min_confidence,
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

            if !quiet {
                print!("{}", output);
            }

            if skipped > 0 {
                eprintln!(
                    "warning: {} file(s) skipped and NOT scanned — see warnings above",
                    skipped
                );
            }

            let mut at_or_above = false;
            let mut below = false;
            for report in &reports {
                for finding in &report.matches {
                    if fail_on.is_met_by(finding.severity) {
                        at_or_above = true;
                    } else {
                        below = true;
                    }
                }
            }
            std::process::exit(match (at_or_above, below) {
                (true, _) => exit::FAILED,
                (false, true) => exit::BELOW_THRESHOLD,
                (false, false) => exit::CLEAN,
            });
        }

        Commands::Rules { patterns, format } => {
            let categories = load_graded(patterns.as_deref())?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&categories)?),
                OutputFormat::Text => {
                    println!("{:<8} {:<9} {:<22} NAME", "ID", "SEVERITY", "CATEGORY");
                    for rule in &categories {
                        // Severity is rendered to a String first: a width
                        // specifier only pads a custom Display impl if that impl
                        // routes through `Formatter::pad`, and this one does
                        // not — so `{:<9}` silently did nothing and the columns
                        // ran together.
                        println!(
                            "{:<8} {:<9} {:<22} {}",
                            rule.id,
                            rule.severity.to_string(),
                            rule.category,
                            rule.name
                        );
                    }
                    println!("\n{} pattern(s).", categories.len());
                }
            }
        }

        Commands::Explain { id, patterns } => {
            let categories = load_graded(patterns.as_deref())?;
            let wanted = id.to_uppercase();
            let rule = categories
                .iter()
                .find(|r| r.id.to_uppercase() == wanted)
                .with_context(|| {
                    // A bare "not found" leaves the user guessing whether they
                    // mistyped or the pattern does not exist at all.
                    let near: Vec<&str> = categories
                        .iter()
                        .map(|r| r.id.as_str())
                        .filter(|other| other.get(..3) == wanted.get(..3))
                        .collect();
                    if near.is_empty() {
                        format!("No pattern {id}. Run `injection-scanner rules` to list them.")
                    } else {
                        format!("No pattern {id}. Nearby ids: {}", near.join(", "))
                    }
                })?;

            println!("{}  {}  [{}]", rule.id, rule.name, rule.severity);
            println!("Category:    {}", rule.category);
            println!("Detects:     {}", rule.description);
            println!("Remediation: {}", rule.remediation);
            println!("Pattern:     {}", rule.pattern);
            if !rule.tags.is_empty() {
                println!("Tags:        {}", rule.tags.join(", "));
            }
            println!(
                "\nSuppress one occurrence with:\n  <!-- injection-scanner:ignore {} -->",
                rule.id
            );
        }
    }
    Ok(())
}

/// A pattern with its severity already resolved against the category default.
///
/// `rules` and `explain` both need the EFFECTIVE severity — the number a user
/// will actually see in a finding — not the optional per-pattern override. A
/// listing that showed a blank severity for every pattern inheriting its
/// category default would be worse than no listing.
#[derive(serde::Serialize)]
struct GradedRule {
    id: String,
    name: String,
    severity: Severity,
    category: String,
    description: String,
    remediation: String,
    pattern: String,
    tags: Vec<String>,
}

fn load_graded(patterns: Option<&std::path::Path>) -> Result<Vec<GradedRule>> {
    let loaded = load_all_patterns(patterns)?;
    for e in &loaded.errors {
        eprintln!("warning: pattern skipped — {e}");
    }
    let mut rules: Vec<GradedRule> = loaded
        .categories
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
    Ok(rules)
}
