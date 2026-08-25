//! Baseline files for incremental adoption on an existing repository (issue
//! #25, CLI-08).
//!
//! An existing repository cannot adopt this scanner if day one is a wall of
//! findings. `--write-baseline` records the current findings as accepted;
//! `--baseline` then withholds exactly those findings from later scans,
//! leaving new ones to still fail the build. This module is deliberately
//! narrow: it identifies and moves findings; the CLI plumbing (flag parsing,
//! stdin rejection, process exit codes) lives in `main.rs`.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::pattern::ScanReport;

/// The only baseline schema version this build understands.
///
/// A file written by a future version with a higher number must be rejected
/// rather than silently misread — see `Baseline::load`.
pub const CURRENT_VERSION: u32 = 1;

/// One accepted finding, identified by `(file, pattern_id, digest)`.
///
/// `#[serde(deny_unknown_fields)]`: a mistyped field name must fail the parse
/// rather than produce an entry that silently matches nothing, because an
/// entry that matches nothing is a standing licence to re-introduce the
/// finding it accepted. `first_seen_line` is `#[serde(default)]` and is
/// informational only — it is deliberately NOT part of identity, so editing
/// anything above a finding does not force the baseline to be regenerated.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaselineEntry {
    pub file: String,
    pub pattern_id: String,
    pub digest: String,
    pub count: usize,
    #[serde(default)]
    pub first_seen_line: usize,
}

/// A committed record of accepted findings.
///
/// The top-level shape here is a JSON *object*, not an array — this is a new
/// artifact and is unrelated to the `--format json` report stream, whose
/// array-at-top-level contract `spec-ci-plugin` depends on is untouched.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baseline {
    pub version: u32,
    #[serde(default)]
    pub generated_by: String,
    pub entries: Vec<BaselineEntry>,
}

/// Identity of one accepted finding: `(file, pattern_id, digest)`.
///
/// Line number is deliberately excluded (D-1): editing anything above a
/// finding must not invalidate its baseline entry, or the file stops being a
/// record of a decision and becomes churn on every commit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Identity {
    file: String,
    pattern_id: String,
    digest: String,
}

/// sha256 over the UTF-8 bytes of `matched_text`, formatted `sha256:<hex>`.
///
/// The payload is hashed, not stored, so a committed baseline stays inert
/// under `DEFAULT_EXTENSIONS` (which includes `json`) — the adoption artifact
/// must never itself become a finding source. Hashing also removes the
/// crafted-collision surface: the adversary authors the scanned text, so a
/// weak digest would let a *new* payload be tuned onto an already-accepted
/// fingerprint (T-QT-02).
fn fingerprint(matched_text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(matched_text.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

/// Strips exactly one leading `./`, leaving an already-relative path
/// untouched.
///
/// The installed pre-commit hook runs `check .` from inside a staging copy,
/// so hook-generated paths carry the `./` prefix while `check docs/foo.md`
/// does not. A baseline written by one and consumed by the other must agree
/// on the path key, or the baseline is useless in the exact workflow it
/// exists for.
fn normalise_file(path: &str) -> String {
    path.strip_prefix("./").unwrap_or(path).to_string()
}

impl Baseline {
    /// Builds a baseline from a set of reports' `matches` only.
    ///
    /// `suppressed` and `low_confidence` are already withheld by other
    /// mechanisms and are not the user's accept-this decision, so they are
    /// not eligible to become baseline entries. Entries are grouped by the
    /// identity triple, with `count` set to the number of occurrences seen
    /// and `first_seen_line` set to the lowest line number in the group.
    /// Entries are sorted by `(file, pattern_id, digest)` so the file is
    /// stable and diffable across runs — an adoption artifact that reorders
    /// itself produces noise in every PR.
    pub fn from_reports(reports: &[ScanReport]) -> Baseline {
        let mut groups: HashMap<Identity, (usize, usize)> = HashMap::new();

        for report in reports {
            for m in &report.matches {
                let identity = Identity {
                    file: normalise_file(&m.file),
                    pattern_id: m.pattern_id.clone(),
                    digest: fingerprint(&m.matched_text),
                };
                let slot = groups.entry(identity).or_insert((0, m.line));
                slot.0 += 1;
                if m.line < slot.1 {
                    slot.1 = m.line;
                }
            }
        }

        let mut entries: Vec<BaselineEntry> = groups
            .into_iter()
            .map(|(identity, (count, first_seen_line))| BaselineEntry {
                file: identity.file,
                pattern_id: identity.pattern_id,
                digest: identity.digest,
                count,
                first_seen_line,
            })
            .collect();
        entries.sort_by(|a, b| {
            (a.file.as_str(), a.pattern_id.as_str(), a.digest.as_str()).cmp(&(
                b.file.as_str(),
                b.pattern_id.as_str(),
                b.digest.as_str(),
            ))
        });

        Baseline {
            version: CURRENT_VERSION,
            generated_by: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            entries,
        }
    }

    /// Writes the baseline as pretty JSON with a trailing newline.
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize baseline")?;
        std::fs::write(path, format!("{json}\n"))
            .with_context(|| format!("Failed to write baseline to {}", path.display()))
    }

    /// Reads and parses a baseline file, rejecting any version other than
    /// [`CURRENT_VERSION`].
    ///
    /// A malformed, missing, or unknown-version baseline is a hard error —
    /// never a silent no-op. A silently-ignored baseline would let a
    /// repository believe it is gated when it is not.
    pub fn load(path: &Path) -> Result<Baseline> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read baseline file {}", path.display()))?;
        let baseline: Baseline = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse baseline file {}", path.display()))?;
        if baseline.version != CURRENT_VERSION {
            anyhow::bail!(
                "Baseline file {} declares version {}, but this build only understands \
                 version {}. Regenerate it with --write-baseline.",
                path.display(),
                baseline.version,
                CURRENT_VERSION
            );
        }
        Ok(baseline)
    }

    /// Moves every finding whose fingerprint is in this baseline out of
    /// `matches` and into `baselined`, rebuilding each report through
    /// [`ScanReport::with_baselined`] so the severity tallies are recomputed
    /// over the reduced `matches`.
    ///
    /// A per-fingerprint `count` budget bounds acceptance: occurrence N+1
    /// beyond the recorded `count` stays in `matches` and still fails the
    /// build (T-QT-03). Matches are walked in the order the scanner produced
    /// them, so which occurrence stays visible when the budget runs out is
    /// stable across runs.
    ///
    /// Returns the entries whose budget was never touched at all — zero
    /// occurrences consumed — which is the definition of stale used to
    /// surface the prunable-entry note.
    pub fn apply(&self, reports: &mut [ScanReport]) -> Vec<BaselineEntry> {
        let mut budget: HashMap<Identity, usize> = HashMap::new();
        for entry in &self.entries {
            budget.insert(
                Identity {
                    file: normalise_file(&entry.file),
                    pattern_id: entry.pattern_id.clone(),
                    digest: entry.digest.clone(),
                },
                entry.count,
            );
        }
        let mut touched: HashSet<Identity> = HashSet::new();

        for report in reports.iter_mut() {
            let mut kept = Vec::new();
            let mut baselined = std::mem::take(&mut report.baselined);

            for m in std::mem::take(&mut report.matches) {
                let identity = Identity {
                    file: normalise_file(&m.file),
                    pattern_id: m.pattern_id.clone(),
                    digest: fingerprint(&m.matched_text),
                };
                let has_budget = budget
                    .get(&identity)
                    .is_some_and(|remaining| *remaining > 0);
                if has_budget {
                    if let Some(remaining) = budget.get_mut(&identity) {
                        *remaining -= 1;
                    }
                    touched.insert(identity);
                    baselined.push(m);
                } else {
                    kept.push(m);
                }
            }

            let file = std::mem::take(&mut report.file);
            let suppressed = std::mem::take(&mut report.suppressed);
            let low_confidence = std::mem::take(&mut report.low_confidence);
            *report = ScanReport::with_baselined(file, kept, suppressed, low_confidence, baselined);
        }

        self.entries
            .iter()
            .filter(|entry| {
                let identity = Identity {
                    file: normalise_file(&entry.file),
                    pattern_id: entry.pattern_id.clone(),
                    digest: entry.digest.clone(),
                };
                !touched.contains(&identity)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_deterministic_prefixed_and_sensitive_to_one_character() {
        let a = fingerprint("ignore all previous instructions");
        let b = fingerprint("ignore all previous instructions");
        let c = fingerprint("ignore all previous instructionZ");

        assert_eq!(a, b, "the same input must hash the same way every time");
        assert!(
            a.starts_with("sha256:"),
            "the digest must be prefixed sha256: so a future algorithm change is \
             self-describing: got {a}"
        );
        let hex = a.strip_prefix("sha256:").expect("prefix checked above");
        assert_eq!(
            hex.len(),
            64,
            "sha256 hex-encodes to 64 lowercase characters: got {hex} ({} chars)",
            hex.len()
        );
        assert!(
            hex.chars()
                .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase()),
            "the hex must be lowercase: got {hex}"
        );
        assert_ne!(
            a, c,
            "a one-character-different input must not collide — a weak/predictable \
             digest would let an attacker tune a new payload onto an accepted fingerprint"
        );
    }

    #[test]
    fn normalise_file_strips_exactly_one_leading_dot_slash() {
        assert_eq!(
            normalise_file("./docs/foo.md"),
            "docs/foo.md",
            "the pre-commit hook runs `check .`, which reports paths with a leading \
             ./ — a baseline that does not strip it can never match that invocation"
        );
        assert_eq!(
            normalise_file("docs/foo.md"),
            "docs/foo.md",
            "an already-relative path must be left untouched"
        );
        assert_eq!(
            normalise_file("././docs/foo.md"),
            "./docs/foo.md",
            "only ONE leading ./ is stripped, not repeated ones"
        );
    }
}
