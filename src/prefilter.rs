//! Aho-Corasick literal prefiltering for the pattern set (issue #4).
//!
//! The scanner's inner loop used to be "for every line, for every pattern, run
//! the regex". With ~55 patterns that is ~55 independent searches over the same
//! bytes, and each one re-derives its own literal prefilter from scratch inside
//! the `regex` crate. This module hoists that work: one Aho-Corasick automaton
//! holds the required literal prefixes of *every* pattern, one pass over the
//! haystack says which patterns could possibly match, and only those regexes
//! are run.
//!
//! # The safety property
//!
//! A prefilter for a security scanner is only allowed to be an optimisation. It
//! must never change what is reported, which means the literal set it holds for
//! a pattern must be a **necessary** condition for that pattern to match — if
//! none of a pattern's literals occur in the haystack, that pattern provably
//! has no match there.
//!
//! That guarantee is not hand-rolled. The literals come from
//! [`regex_syntax::hir::literal::Extractor`] in `ExtractKind::Prefix` mode,
//! applied to the same HIR the `regex` crate itself compiles — it is the very
//! machinery `regex` uses to build its own prefilters. A finite, non-empty
//! sequence from that extractor means every match of the expression begins with
//! one of its literals. An *infinite* sequence means "unknown", and a sequence
//! containing an empty literal means "could start with anything"; both are
//! treated here as "this pattern is unfilterable", and such patterns are run
//! against every haystack exactly as before. A correct partial prefilter beats
//! a fast wrong one, so anything the extractor will not vouch for falls through
//! to the regex path.
//!
//! # Case
//!
//! Literals are extracted from the **case-folded** HIR — i.e. the parser is
//! given the same `case_insensitive` flag the scanner gives `RegexBuilder`, so
//! folding is Unicode-aware and `(?i)send` yields the `ſend` variant as well as
//! `Send`. They are then ASCII-lowercased and deduplicated, and the automaton
//! is built `ascii_case_insensitive`. That is a deliberate widening: every
//! literal the extractor produced is an ASCII-case variant of the lowercased
//! form it collapses into, so the automaton matches a **superset** of the
//! extracted set. A superset is safe — it can only cause a pattern to be run
//! when it need not have been — while the collapse keeps the automaton from
//! growing as 2^n in the length of every case-insensitive word.

use std::collections::HashMap;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use regex_syntax::hir::literal::{ExtractKind, Extractor};
use regex_syntax::ParserBuilder;

/// Longest required prefix, in bytes, kept for any one literal.
///
/// Case folding makes literal extraction exponential in literal length: every
/// ASCII letter of a case-insensitive word doubles the sequence, so an
/// untruncated `(?i)disregard` alone is 512 literals before the surrounding
/// alternation is considered. Truncating to a short prefix keeps the automaton
/// small while retaining most of the selectivity — in English prose a
/// four-byte prefix such as `igno` or `disr` is already rare.
///
/// Truncation is always safe in the direction that matters: a prefix of a
/// required literal is itself required.
///
/// Four is measured, not guessed. Against `benches/scan.rs` on the embedded
/// set, with `single_large_file` / `many_small_files_500` / patterns covered:
///
/// | prefix | large file | 500 files | patterns filtered |
/// |---|---|---|---|
/// | 3 | 101ms | 41ms | 58 of 61 |
/// | 4 | 63ms | 25ms | 56 of 61 |
/// | 5 | 59ms | 24ms | 51 of 61 |
///
/// Three is too blunt — three-byte prefixes of English verbs are common enough
/// in prose that the automaton rarely says no. Five is marginally faster today
/// but drops five more patterns to the unfiltered path, because their
/// case-folded sequences blow past [`LITERAL_BUDGET_PER_PATTERN`] at that
/// length; that is coverage traded for single-digit percent, and it gets worse
/// as the pattern library grows.
const LITERAL_PREFIX_BYTES: usize = 4;

/// How many literals one pattern may contribute before it is given up on.
///
/// Generous, because the extractor counts literals *before* the ASCII-case
/// collapse below shrinks them, and because giving up costs real performance
/// (the pattern then runs on every line) while a large set costs only build
/// time, paid once per process.
const LITERAL_BUDGET_PER_PATTERN: usize = 20_000;

/// How many characters of a single character class the extractor may expand.
///
/// The default is 10. Case-folding an ASCII letter produces a class of two or
/// three members, which fits easily; the raised limit is for the small
/// hand-written classes that open some patterns, such as the zero-width and
/// bidi-control sets in `encoding.yaml`, which would otherwise be abandoned.
const LITERAL_CLASS_LIMIT: usize = 64;

/// The required literal prefixes of `pattern`, or `None` if it has none.
///
/// `None` means "this expression could begin with anything the extractor is
/// willing to vouch for" and is the safe answer: a caller must then run the
/// pattern unconditionally. It is returned for an expression that starts with a
/// character class too large to enumerate, a `\w`-style Unicode class, an
/// anchor or look-around, or anything else that leaves the extracted sequence
/// infinite or containing the empty literal.
///
/// `case_sensitive` must be the same flag the scanner passes to
/// `RegexBuilder::case_insensitive` (inverted), or the returned literals do not
/// describe the regex that will actually be run.
///
/// The returned literals are ASCII-lowercased byte strings, sorted and
/// deduplicated, and are intended to be matched **case-insensitively over
/// ASCII** — see the module docs for why that is a safe widening.
pub fn required_prefixes(pattern: &str, case_sensitive: bool) -> Option<Vec<Vec<u8>>> {
    Some(
        required_prefixes_inline(pattern, case_sensitive)?
            .into_iter()
            .map(|prefix| prefix.as_bytes().to_vec())
            .collect(),
    )
}

/// As [`required_prefixes`], without allocating a `Vec` per literal.
///
/// The prefilter builds from this. `required_prefixes` is the same answer in
/// the shape a caller outside this module can use.
fn required_prefixes_inline(pattern: &str, case_sensitive: bool) -> Option<Vec<Prefix>> {
    let hir = ParserBuilder::new()
        .case_insensitive(!case_sensitive)
        .build()
        .parse(pattern)
        .ok()?;

    let seq = Extractor::new()
        .kind(ExtractKind::Prefix)
        .limit_literal_len(LITERAL_PREFIX_BYTES)
        .limit_total(LITERAL_BUDGET_PER_PATTERN)
        .limit_class(LITERAL_CLASS_LIMIT)
        .extract(&hir);

    // `None` here is the extractor's "infinite" sequence: it could not bound
    // what a match starts with, so nothing may be skipped on its say-so.
    let literals = seq.literals()?;
    if literals.is_empty() {
        // An empty sequence means the expression matches nothing at all. That
        // is almost certainly a mistake in the pattern rather than an
        // optimisation opportunity, and acting on it would make the scanner
        // silently stop running a rule. Fall through to the regex instead.
        return None;
    }

    // Collected as fixed-size arrays, not `Vec<u8>`, and deduplicated before
    // anything reaches the heap. Case folding makes the pre-dedup sequence
    // large — the embedded set produces ~22,000 literals that collapse to
    // ~3,900 distinct prefixes — and allocating a `Vec` per literal to throw
    // most of them away cost more than the parse and the extraction combined.
    let mut prefixes: Vec<Prefix> = Vec::with_capacity(literals.len());
    for literal in literals {
        let bytes = literal.as_bytes();
        if bytes.is_empty() {
            // A match may begin at any byte; there is nothing to filter on.
            return None;
        }
        prefixes.push(Prefix::truncated(bytes));
    }
    prefixes.sort_unstable();
    prefixes.dedup();
    Some(prefixes)
}

/// A literal prefix of at most [`LITERAL_PREFIX_BYTES`] bytes, inline.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Prefix {
    len: u8,
    bytes: [u8; LITERAL_PREFIX_BYTES],
}

impl Prefix {
    /// The first [`LITERAL_PREFIX_BYTES`] bytes of `literal`, ASCII-lowercased.
    ///
    /// Byte-wise ASCII lowercasing leaves UTF-8 continuation bytes (0x80 to
    /// 0xBF) untouched, so a literal cut mid-character stays exactly the byte
    /// string it was — which is all the automaton needs, since it searches
    /// bytes rather than characters.
    fn truncated(literal: &[u8]) -> Self {
        let len = literal.len().min(LITERAL_PREFIX_BYTES);
        let mut bytes = [0u8; LITERAL_PREFIX_BYTES];
        bytes[..len].copy_from_slice(&literal[..len]);
        bytes.make_ascii_lowercase();
        Self {
            len: len as u8,
            bytes,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }
}

/// Which patterns could possibly match a given haystack.
///
/// Reused across every line of a scan rather than reallocated, because the
/// whole point of the prefilter is to make the per-line cost small.
pub struct CandidateSet {
    marked: Vec<bool>,
    /// How many patterns are still *not* marked. Lets a haystack that has
    /// already selected everything stop searching.
    remaining: usize,
}

impl CandidateSet {
    /// A set sized for `pattern_count` patterns, with every pattern selected.
    pub fn new(pattern_count: usize) -> Self {
        Self {
            marked: vec![true; pattern_count],
            remaining: 0,
        }
    }

    /// Deselect everything.
    fn clear(&mut self) {
        self.marked.fill(false);
        self.remaining = self.marked.len();
    }

    /// Select everything — the answer when there is no prefilter.
    fn mark_all(&mut self) {
        self.marked.fill(true);
        self.remaining = 0;
    }

    fn insert(&mut self, index: usize) {
        if let Some(slot) = self.marked.get_mut(index) {
            if !*slot {
                *slot = true;
                self.remaining -= 1;
            }
        }
    }

    /// Whether the pattern at `index` may match the haystack this set describes.
    ///
    /// An out-of-range index answers `true`. The set is an optimisation, and an
    /// optimisation that is asked a question it cannot answer must fail towards
    /// running the regex.
    pub fn contains(&self, index: usize) -> bool {
        self.marked.get(index).copied().unwrap_or(true)
    }
}

/// One Aho-Corasick automaton over the required literals of a whole pattern set.
pub struct LiteralPrefilter {
    automaton: AhoCorasick,
    /// Literal id, as the automaton numbers them, to the patterns requiring it.
    ///
    /// A `Vec<Vec<usize>>` rather than one pattern per literal because distinct
    /// patterns share prefixes constantly — `igno` is required by the two
    /// role-override rules and by an instruction-injection rule — and feeding
    /// the same literal to the automaton several times would make it do the
    /// same work several times.
    owners: Vec<Vec<usize>>,
    /// Patterns with no usable literal set. Always candidates.
    unfilterable: Vec<usize>,
    /// How many patterns this prefilter can actually rule out. Reported by
    /// [`Self::filterable_count`] so a test can assert the thing is not a no-op.
    filterable: usize,
}

impl LiteralPrefilter {
    /// Build a prefilter for `patterns`, given as `(regex source, case sensitive)`
    /// in the order the caller will index them.
    ///
    /// Returns `None` when the automaton could not be built, or when it could
    /// rule nothing out — in both cases the caller should skip prefiltering
    /// rather than pay for an automaton that never says no.
    pub fn build(patterns: &[(&str, bool)]) -> Option<Self> {
        let mut literals: Vec<Prefix> = Vec::new();
        let mut owners: Vec<Vec<usize>> = Vec::new();
        // Literal to its id in `literals`, so a prefix shared by several
        // patterns is fed to the automaton once. `igno` alone is required by
        // both role-override rules and by an instruction-injection rule.
        let mut index_of: HashMap<Prefix, usize> = HashMap::new();
        let mut unfilterable = Vec::new();
        let mut filterable = 0usize;

        for (pattern_index, (source, case_sensitive)) in patterns.iter().enumerate() {
            match required_prefixes_inline(source, *case_sensitive) {
                Some(prefixes) => {
                    filterable += 1;
                    for prefix in prefixes {
                        let id = *index_of.entry(prefix).or_insert_with(|| {
                            literals.push(prefix);
                            owners.push(Vec::new());
                            literals.len() - 1
                        });
                        owners[id].push(pattern_index);
                    }
                }
                None => unfilterable.push(pattern_index),
            }
        }

        if filterable == 0 {
            return None;
        }

        let automaton = AhoCorasickBuilder::new()
            // `Standard` is the only match kind that supports the overlapping
            // search below, and overlapping is required: a leftmost search
            // resumes after each match and would miss a literal that starts
            // inside one already reported — `bcd` in `abcd` when `abc` is also
            // a literal. Missing it would deselect a pattern that can match,
            // which is precisely the silent under-reporting this must not do.
            .match_kind(MatchKind::Standard)
            .ascii_case_insensitive(true)
            .build(literals.iter().map(Prefix::as_bytes))
            .ok()?;

        Some(Self {
            automaton,
            owners,
            unfilterable,
            filterable,
        })
    }

    /// How many patterns this prefilter is able to rule out.
    pub fn filterable_count(&self) -> usize {
        self.filterable
    }

    /// Fill `out` with the patterns that could match `haystack`.
    pub fn select(&self, haystack: &str, out: &mut CandidateSet) {
        out.clear();
        for &pattern_index in &self.unfilterable {
            out.insert(pattern_index);
        }
        if out.remaining == 0 {
            return;
        }
        for found in self.automaton.find_overlapping_iter(haystack) {
            let Some(owners) = self.owners.get(found.pattern().as_usize()) else {
                continue;
            };
            for &pattern_index in owners {
                out.insert(pattern_index);
            }
            if out.remaining == 0 {
                break;
            }
        }
    }
}

/// Fill `out` for `haystack`, selecting everything when there is no prefilter.
pub(crate) fn select_into(
    prefilter: Option<&LiteralPrefilter>,
    haystack: &str,
    out: &mut CandidateSet,
) {
    match prefilter {
        Some(prefilter) => prefilter.select(haystack, out),
        None => out.mark_all(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_the_alternation_a_real_pattern_opens_with() {
        let prefixes = required_prefixes("(?:ignore|disregard)\\s+previous", false)
            .expect("a leading literal alternation must be extractable");
        assert!(prefixes.contains(&b"igno".to_vec()));
        assert!(prefixes.contains(&b"disr".to_vec()));
    }

    #[test]
    fn declines_a_pattern_that_can_start_with_anything() {
        assert!(required_prefixes("\\w+\\s+secret", false).is_none());
        assert!(required_prefixes(".*secret", false).is_none());
    }

    #[test]
    fn declines_a_pattern_that_can_match_the_empty_string() {
        assert!(required_prefixes("a*", false).is_none());
    }

    #[test]
    fn case_insensitive_literals_are_collapsed_to_lowercase() {
        let prefixes =
            required_prefixes("SEND", false).expect("a plain literal must be extractable");
        assert!(
            prefixes.contains(&b"send".to_vec()),
            "the ASCII form must survive the collapse: {prefixes:?}"
        );
        // Folding is Unicode-aware, so `s` also folds to U+017F LATIN SMALL
        // LETTER LONG S and `(?i)SEND` genuinely matches `\u{17f}end`. That
        // variant has to be carried as its own literal — ASCII-case-insensitive
        // matching cannot reach it from `send`, and dropping it would make the
        // prefilter unsound for exactly the obfuscation this tool exists to
        // catch.
        assert!(
            prefixes.iter().any(|p| p.starts_with("\u{17f}".as_bytes())),
            "the non-ASCII case-fold variant must be kept: {prefixes:?}"
        );
    }

    #[test]
    fn a_case_sensitive_pattern_keeps_its_own_casing_folded_only_for_matching() {
        let prefixes =
            required_prefixes("SYSTEM OVERRIDE", true).expect("a plain literal must extract");
        // Lowercased for the automaton, which then matches case-insensitively;
        // the regex itself still enforces the casing.
        assert_eq!(prefixes, vec![b"syst".to_vec()]);
    }

    #[test]
    fn overlapping_literals_are_all_reported() {
        let prefilter = LiteralPrefilter::build(&[("abc", false), ("bcd", false)])
            .expect("two literal patterns must build");
        let mut candidates = CandidateSet::new(2);
        prefilter.select("zzabcdzz", &mut candidates);
        assert!(candidates.contains(0), "abc must be found at offset 2");
        assert!(
            candidates.contains(1),
            "bcd overlaps abc and must still be found — a leftmost search would miss it"
        );
    }

    #[test]
    fn a_pattern_with_no_literals_is_always_a_candidate() {
        let prefilter = LiteralPrefilter::build(&[("ignore", false), ("\\w+zzz", false)])
            .expect("one filterable pattern is enough to build");
        let mut candidates = CandidateSet::new(2);
        prefilter.select("nothing here", &mut candidates);
        assert!(!candidates.contains(0));
        assert!(candidates.contains(1));
    }

    #[test]
    fn no_prefilter_selects_everything() {
        let mut candidates = CandidateSet::new(3);
        select_into(None, "anything", &mut candidates);
        assert!((0..3).all(|i| candidates.contains(i)));
    }
}
