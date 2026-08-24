//! Directory walking (issue #22).
//!
//! Replaces a hand-rolled recursive `read_dir` that descended into `.git`,
//! `target` and `node_modules`, ignored `.gitignore` entirely, read files of any
//! size fully into memory, and followed symlinks with no loop guard. On a real
//! repository that is where the performance budget went — and the findings it
//! produced inside `target/` were noise nobody could act on.
//!
//! Built on the `ignore` crate, the walker behind ripgrep, which brings
//! `.gitignore` handling and parallel traversal.
//!
//! Two properties are deliberate and load-bearing:
//!
//! **Results are sorted.** Parallel traversal yields paths in whatever order
//! threads finish. The JSON output is an array consumers iterate, and unstable
//! ordering would make diffs and baselines churn for no reason. Sorting once at
//! the end costs nothing next to the scan.
//!
//! **Nothing is skipped silently.** Every file the walker declines to scan —
//! wrong extension, over the size cap, excluded by glob — is counted and
//! reported. A scanner that says "clean" about files it never opened is worse
//! than one that says nothing, because the first answer gets believed.

use std::path::{Path, PathBuf};

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;

/// Directories never descended into, even under `--no-ignore`.
///
/// `--no-ignore` means "do not trust this repository's `.gitignore`", which is a
/// reasonable thing to want when scanning someone else's checkout. It does not
/// mean "scan 40,000 files of build output". Nothing agent-facing lives in these
/// directories; anything that does is generated from a source file that is
/// itself scanned.
pub const ALWAYS_EXCLUDED: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "target",
    "node_modules",
    "vendor",
    "dist",
    "build",
    ".venv",
    "venv",
    "__pycache__",
    ".mypy_cache",
    ".pytest_cache",
    ".gradle",
    ".next",
    ".tox",
];

/// Extensions scanned by default.
///
/// Deliberately unchanged from the hand-rolled walker in this change. Widening
/// it is issue #23, and doing both at once would make it impossible to tell a
/// walker regression from a file-type regression.
pub const DEFAULT_EXTENSIONS: &[&str] = &["md", "yaml", "yml", "txt", "toml"];

/// Default size cap, in bytes.
///
/// A CLAUDE.md, skill file or RAG document above 10 MB does not exist in
/// practice; a 10 MB match is a data file that happens to end in `.txt`. The old
/// walker read it fully into memory with no ceiling at all.
pub const DEFAULT_MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Why a file the walker reached was not handed to the scanner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipReason {
    /// Extension not in the scanned set, and no `--include` matched it.
    UnscannedType,
    /// Larger than the configured cap.
    TooLarge { size: u64, limit: u64 },
    /// The directory entry itself could not be read.
    Unreadable(String),
}

impl SkipReason {
    /// One-line description for a stderr warning.
    pub fn describe(&self) -> String {
        match self {
            SkipReason::UnscannedType => "not a scanned file type".to_string(),
            SkipReason::TooLarge { size, limit } => {
                format!("{size} bytes exceeds the {limit} byte limit (--max-file-size)")
            }
            SkipReason::Unreadable(message) => message.clone(),
        }
    }
}

/// A file the walker declined to scan.
#[derive(Debug, Clone)]
pub struct Skipped {
    pub path: PathBuf,
    pub reason: SkipReason,
}

/// Knobs for a walk. `Default` reproduces the CLI defaults.
#[derive(Debug, Clone)]
pub struct WalkOptions {
    /// Repeatable `--exclude` globs, applied on top of [`ALWAYS_EXCLUDED`].
    pub excludes: Vec<String>,
    /// Repeatable `--include` globs. A file matching one is scanned whatever its
    /// extension.
    pub includes: Vec<String>,
    /// Honour `.gitignore` and friends. `--no-ignore` clears this.
    pub respect_gitignore: bool,
    /// Skip files larger than this many bytes.
    pub max_file_size: u64,
    /// Follow symlinks. Off by default — a symlink loop is a hang, and a symlink
    /// out of the tree is a scan of somewhere the user did not name.
    pub follow_symlinks: bool,
    /// Traversal threads. `0` lets the walker choose.
    pub jobs: usize,
}

impl Default for WalkOptions {
    fn default() -> Self {
        Self {
            excludes: Vec::new(),
            includes: Vec::new(),
            respect_gitignore: true,
            max_file_size: DEFAULT_MAX_FILE_SIZE,
            follow_symlinks: false,
            jobs: 0,
        }
    }
}

/// What a walk produced.
#[derive(Debug, Default)]
pub struct Walked {
    /// Files to scan, sorted.
    pub files: Vec<PathBuf>,
    /// Files reached but not scanned, sorted, each with its reason.
    pub skipped: Vec<Skipped>,
    /// An ignore file was found and honoured, so paths were withheld that this
    /// walk never saw and therefore cannot count.
    ///
    /// The counted skips above are files the walker reached and declined.
    /// Ignore rules prune whole subtrees before that point — which is the reason
    /// they are fast, and the reason they cannot be itemised without giving up
    /// the speed. Disclosing that they were *applied* is the honest middle: the
    /// exact set is one `--no-ignore` re-run away.
    ///
    /// This matters more than it looks. A skills pack shipping a `.gitignore`
    /// containing `*` would otherwise scan nothing and report "clean".
    pub ignore_rules_applied: bool,
}

/// Is this path scannable under these options?
///
/// An `--include` glob wins over the extension list: that is the entire point of
/// the flag. It cannot override [`ALWAYS_EXCLUDED`], which is enforced earlier by
/// the walker's directory filter.
fn is_scannable(path: &Path, overrides: Option<&ignore::overrides::Override>) -> bool {
    if let Some(overrides) = overrides {
        // `matched` is `Whitelist` only for an explicit `--include` hit.
        if overrides.matched(path, false).is_whitelist() {
            return true;
        }
    }
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| DEFAULT_EXTENSIONS.contains(&ext))
}

/// Walk `root`, returning the files to scan and the ones deliberately skipped.
///
/// Never returns `Err` for a problem *inside* the tree — an unreadable
/// subdirectory becomes a `Skipped` entry rather than aborting the walk, which
/// is the same isolation the per-file read already had (issue #14). Only a
/// failure to build the glob set is an error, and that is a bad flag value.
pub fn walk(root: &Path, options: &WalkOptions) -> anyhow::Result<Walked> {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .follow_links(options.follow_symlinks)
        // Without this, `.gitignore` is honoured only inside a git repository —
        // so the same directory scanned as a checkout and as an extracted
        // tarball gives different answers. A scanner whose coverage depends on
        // whether `.git` happens to be present is a scanner you cannot reason
        // about.
        .require_git(false)
        .git_ignore(options.respect_gitignore)
        .git_global(options.respect_gitignore)
        .git_exclude(options.respect_gitignore)
        .ignore(options.respect_gitignore)
        .parents(options.respect_gitignore);

    if options.jobs > 0 {
        builder.threads(options.jobs);
    }

    // The unconditional deny-list, plus any `--exclude`. Both are expressed as
    // ignore-globs so one filter handles them; `--include` is a separate
    // override set because whitelisting here would defeat the deny-list.
    let mut excludes = OverrideBuilder::new(root);
    for name in ALWAYS_EXCLUDED {
        excludes.add(&format!("!**/{name}/**"))?;
        excludes.add(&format!("!**/{name}"))?;
    }
    for glob in &options.excludes {
        excludes.add(&format!("!{glob}"))?;
    }
    builder.overrides(excludes.build()?);

    let includes = if options.includes.is_empty() {
        None
    } else {
        let mut b = OverrideBuilder::new(root);
        for glob in &options.includes {
            b.add(glob)?;
        }
        Some(b.build()?)
    };

    let mut files = Vec::new();
    let mut skipped = Vec::new();

    // Cheap and root-only on purpose: a stat call, not a second traversal. It
    // catches the case that actually bites — a pack or corpus shipping an
    // ignore file at its root — without paying to find every nested one.
    let ignore_rules_applied = options.respect_gitignore
        && [".gitignore", ".ignore"]
            .iter()
            .any(|name| root.join(name).exists());

    for result in builder.build() {
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => {
                // A traversal error carries the path when it has one; without
                // one there is nothing actionable to report, so it is dropped
                // rather than attributed to the wrong file.
                if let ignore::Error::WithPath { path, err } = &error {
                    skipped.push(Skipped {
                        path: path.clone(),
                        reason: SkipReason::Unreadable(err.to_string()),
                    });
                }
                continue;
            }
        };

        // `file_type()` is `None` only for the root of a stdin walk, which this
        // never builds. Directories are traversal, not candidates.
        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }
        let path = entry.path();

        if !is_scannable(path, includes.as_ref()) {
            skipped.push(Skipped {
                path: path.to_path_buf(),
                reason: SkipReason::UnscannedType,
            });
            continue;
        }

        // Checked from the directory entry rather than by reading the file: the
        // whole point is not to pull a huge file into memory to discover it is
        // huge. A metadata failure is not fatal — the read will fail next and
        // report itself.
        if let Ok(metadata) = entry.metadata() {
            if metadata.len() > options.max_file_size {
                skipped.push(Skipped {
                    path: path.to_path_buf(),
                    reason: SkipReason::TooLarge {
                        size: metadata.len(),
                        limit: options.max_file_size,
                    },
                });
                continue;
            }
        }

        files.push(path.to_path_buf());
    }

    // Parallel traversal finishes in nondeterministic order; the output array is
    // something consumers diff. See the module note.
    files.sort();
    skipped.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(Walked {
        files,
        skipped,
        ignore_rules_applied,
    })
}
