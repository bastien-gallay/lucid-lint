//! Rule: `readability.large-number-unanchored`.
//!
//! Flags large numerals or magnitude words that appear in a sentence
//! with no nearby anchor — unit, percentage, currency symbol, ratio
//! pattern, or curated comparator phrase. The CDC Clear Communication
//! Index asks whether numbers are *clear and meaningful for the
//! primary audience*; plainlanguage.gov is more direct on the
//! mechanism — *"Use Numbers Effectively"* recommends giving every
//! large figure a comparison or denominator the reader can ground.
//! Readers with dyscalculia carry the cost first: a context-free
//! "4.8 milliards" forces an unaided magnitude estimate that ordinary
//! prose context does not provide.
//!
//! Cohort sibling of [F49](../../../ROADMAP.md#f49)
//! (`structure.italic-span-long`) under the
//! [F-experimental-rule-status](../../../ROADMAP.md#f-experimental-rule-status)
//! substrate. Ships as [`Status::Experimental`] in v0.2.x; flips to
//! `Stable` at the v0.3 cut as part of the cohort flip. Boundary with
//! the dyscalculia-tagged sibling [F51](../../../ROADMAP.md#f51)
//! (`structure.number-run`): F51 fires on numeric *clusters* (≥ N
//! tokens in one sentence); this rule fires on a *single* large or
//! magnitude-word numeral that lacks anchoring context.
//!
//! ## Candidate definition
//!
//! A sentence-level candidate is either:
//!
//! 1. A numeric token whose digit count is ≥ 4 *and* whose integer
//!    value is ≥ the profile threshold. The scanner collapses common
//!    thousands separators (`,`, `.`, ASCII space, NBSP, thin space,
//!    narrow NBSP) between digit groups so `1 000` (FR) and `1,000`
//!    (EN) both count as one 4-digit token with value 1000.
//! 2. A magnitude word — `million`(s), `billion`(s), `trillion`(s) in
//!    EN; `million`(s), `milliard`(s), `billion`(s), `trillion`(s) in
//!    FR. Whole-word, case-insensitive.
//!
//! ## Skips (false-positive guards)
//!
//! - **Year-shaped**: exactly 4 contiguous digits (no thousands or
//!   decimal separators) with value in `1000..=2999`.
//! - **Ordinal**: digit run immediately followed by a letter (EN: `1st`,
//!   `2nd`; FR ordinal exponents are unicode but the rule keys on the
//!   adjacent letter pattern).
//! - **Figure / page / section reference**: candidate preceded
//!   (within ~16 chars, same sentence) by `Figure`, `Fig.`, `Page`,
//!   `Section`, `§`, `p.`, `pp.`, or the FR equivalents
//!   (`figure`, `page`, `section`, `tableau`, `chapitre`).
//!
//! ## Anchor types (sentence-scoped)
//!
//! Any of the following anywhere in the sentence anchors *all*
//! candidates in that sentence:
//!
//! - Percent sign (`%`).
//! - Currency symbol (`$`, `€`, `£`, `¥`).
//! - Unit token from a small curated list (`km`, `kg`, `m²`, `°C`, …).
//! - Ratio pattern: `X out of Y`, `X sur Y`, or `X / Y` between digits.
//! - Comparator phrase from
//!   [`crate::language::en::ANCHOR_COMPARATORS_EN`] /
//!   [`crate::language::fr::ANCHOR_COMPARATORS_FR`].
//!
//! See [`RULES.md`](../../RULES.md) for the reference entry.

use unicode_segmentation::UnicodeSegmentation;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::{split_sentences, Document};
use crate::rules::{Rule, Status};
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Lower bound on digit count to consider a numeric token "large".
const MIN_DIGITS: u32 = 4;

/// Inclusive lower bound for the year-shaped guard.
const YEAR_LOW: u64 = 1000;
/// Inclusive upper bound for the year-shaped guard.
const YEAR_HIGH: u64 = 2999;

/// Maximum number of bytes between a figure-ref keyword and the
/// candidate digit it covers. Keeps the lookback bounded; one or two
/// short tokens fit (`Figure 12`, `p. 1234`).
const FIGURE_REF_LOOKBACK_BYTES: usize = 16;

/// Currency symbols recognised as anchors. Kept short — symbols
/// readers see in mainstream EN/FR prose.
const CURRENCY_SYMBOLS: &[char] = &['$', '€', '£', '¥'];

/// Unit tokens recognised as anchors. Whole-word match,
/// case-sensitive on the symbol form (so `KG` does *not* match — units
/// have a canonical form). Language-agnostic by design — most SI
/// symbols travel.
const UNIT_TOKENS: &[&str] = &[
    "km", "m", "cm", "mm", "µm", "nm", "ha", "m²", "m³", "km²", "km/h", "kg", "g", "mg", "µg", "t",
    "L", "l", "mL", "ml", "cL", "cl", "h", "min", "s", "ms", "µs", "°C", "°F", "K", "Hz", "kHz",
    "MHz", "GHz", "MB", "GB", "KB", "TB", "PB", "Mo", "Go", "Ko", "To", "mph", "kWh", "Wh", "W",
    "kW", "MW", "bps", "Mbps", "Gbps",
];

/// EN figure-ref tokens that anchor a following digit by reference,
/// not by magnitude. Whole-word, case-insensitive.
const FIGURE_REFS_EN: &[&str] = &[
    "figure", "fig.", "fig", "page", "pages", "section", "sections", "p.", "pp.", "chapter",
    "table", "tab.", "tab", "row", "column", "line", "no.", "n.", "step", "rule", "issue", "PR",
    "pr", "#",
];

/// FR figure-ref tokens. Whole-word, case-insensitive.
const FIGURE_REFS_FR: &[&str] = &[
    "figure", "fig.", "fig", "page", "pages", "section", "sections", "p.", "pp.", "chapitre",
    "tableau", "tableaux", "ligne", "colonne", "n°", "no.", "no", "étape", "règle", "issue", "PR",
    "pr", "#", "annexe", "annexes",
];

/// EN magnitude words. Whole-word, case-insensitive.
const MAGNITUDES_EN: &[&str] = &[
    "million",
    "millions",
    "billion",
    "billions",
    "trillion",
    "trillions",
];

/// FR magnitude words. Whole-word, case-insensitive. `billion` in FR
/// long-scale is 10^12 not 10^9 — different magnitude from EN, but
/// either way it is large enough to demand anchoring, so the rule
/// treats both identically.
const MAGNITUDES_FR: &[&str] = &[
    "million",
    "millions",
    "milliard",
    "milliards",
    "billion",
    "billions",
    "trillion",
    "trillions",
];

/// Configuration for [`LargeNumberUnanchored`].
///
/// `min_value` is the integer value at and above which a numeric token
/// of ≥ 4 digits becomes a candidate. Tokens that meet the digit-count
/// gate but fall below `min_value` are skipped — quietly tolerating
/// page-number-like quantities ("Page 1234" already gets the figure-
/// ref skip; this is a second safety net).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Inclusive lower bound on the integer value of a numeric
    /// candidate.
    pub min_value: u64,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let min = match profile {
            Profile::DevDoc => 100_000,
            Profile::Public => 10_000,
            Profile::Falc => 1_000,
        };
        Self { min_value: min }
    }
}

/// The [`LargeNumberUnanchored`] rule.
#[derive(Debug, Clone, Copy)]
pub struct LargeNumberUnanchored {
    config: Config,
}

impl LargeNumberUnanchored {
    /// Build the rule from explicit config.
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Build the rule using the preset for the given profile.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        Self::new(Config::for_profile(profile))
    }

    /// The rule identifier.
    pub const ID: &'static str = "readability.large-number-unanchored";
}

impl Rule for LargeNumberUnanchored {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let Some(lookups) = LanguageLookups::for_language(language) else {
            return Vec::new();
        };
        let mut diags = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
            for sentence in sentences {
                if has_anchor(&sentence.text, &lookups) {
                    continue;
                }
                let Some(candidate) =
                    first_unanchored_candidate(&sentence.text, self.config.min_value, &lookups)
                else {
                    continue;
                };
                diags.push(build_diagnostic(
                    &document.source,
                    &sentence.text,
                    sentence.line,
                    sentence.column,
                    candidate,
                    section_title,
                ));
            }
        }
        diags
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        // CDC CCI grounds the dyscalculia load (numbers must be
        // clear and meaningful for the primary audience); the
        // `general` tag mirrors plainlanguage.gov scope — every
        // reader benefits from anchored figures, not only dyscalculic
        // readers.
        &[ConditionTag::Dyscalculia, ConditionTag::General]
    }

    fn status(&self) -> Status {
        // Cohort sibling of F49 — flips to Stable at the v0.3 cut as
        // part of the F-experimental-rule-status cohort flip.
        Status::Experimental
    }
}

/// Per-language reference data. Held as borrowed slices so the rule
/// stays `Copy`.
struct LanguageLookups {
    comparators: &'static [&'static str],
    magnitudes: &'static [&'static str],
    figure_refs: &'static [&'static str],
    ratio_word: &'static str,
}

impl LanguageLookups {
    fn for_language(language: Language) -> Option<Self> {
        match language {
            Language::En => Some(Self {
                comparators: en::ANCHOR_COMPARATORS_EN,
                magnitudes: MAGNITUDES_EN,
                figure_refs: FIGURE_REFS_EN,
                ratio_word: "out of",
            }),
            Language::Fr => Some(Self {
                comparators: fr::ANCHOR_COMPARATORS_FR,
                magnitudes: MAGNITUDES_FR,
                figure_refs: FIGURE_REFS_FR,
                ratio_word: "sur",
            }),
            Language::Unknown => None,
        }
    }
}

/// Describes one unanchored candidate found in a sentence.
#[derive(Debug, Clone, Copy)]
struct Candidate {
    /// Byte offset in the sentence where the candidate begins.
    offset: usize,
    /// Human-readable kind for the diagnostic message.
    kind: CandidateKind,
}

#[derive(Debug, Clone, Copy)]
enum CandidateKind {
    /// A numeric token whose digit count is ≥ `MIN_DIGITS` and whose
    /// integer value is ≥ profile threshold.
    LargeNumeral { digits: u32, value: u64 },
    /// A magnitude word (`million(s)`, `milliard(s)`, …).
    Magnitude,
}

/// Sentence-level anchor check. Returns true when *any* anchor type
/// is present anywhere in the sentence.
fn has_anchor(sentence: &str, lookups: &LanguageLookups) -> bool {
    if sentence.contains('%') {
        return true;
    }
    if sentence.chars().any(|c| CURRENCY_SYMBOLS.contains(&c)) {
        return true;
    }
    let lower = sentence.to_lowercase();
    for &phrase in lookups.comparators {
        if lower.contains(phrase) {
            return true;
        }
    }
    if has_unit_token(sentence) {
        return true;
    }
    if has_ratio_pattern(&lower, lookups.ratio_word) {
        return true;
    }
    false
}

/// Whole-word search for any [`UNIT_TOKENS`] entry. Matches the
/// canonical case so `KG` does not register (units carry case).
fn has_unit_token(sentence: &str) -> bool {
    for token in tokenize_words(sentence) {
        if UNIT_TOKENS.contains(&token) {
            return true;
        }
    }
    false
}

/// Detect ratio patterns of the form `digit RATIO_WORD digit` or
/// `digit / digit` (with optional surrounding whitespace).
fn has_ratio_pattern(sentence_lower: &str, ratio_word: &str) -> bool {
    if find_word_then_digit(sentence_lower, ratio_word) {
        return true;
    }
    // `digit / digit` style — look for any `/` flanked by digits
    // (allowing a single space on either side).
    let bytes = sentence_lower.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'/' {
            let before = preceding_nonspace_digit(bytes, i);
            let after = following_nonspace_digit(bytes, i);
            if before && after {
                return true;
            }
        }
    }
    false
}

fn preceding_nonspace_digit(bytes: &[u8], index: usize) -> bool {
    let mut j = index;
    while j > 0 {
        j -= 1;
        match bytes[j] {
            b' ' => {},
            b'0'..=b'9' => return true,
            _ => return false,
        }
    }
    false
}

fn following_nonspace_digit(bytes: &[u8], index: usize) -> bool {
    let mut j = index + 1;
    while j < bytes.len() {
        match bytes[j] {
            b' ' => j += 1,
            b'0'..=b'9' => return true,
            _ => return false,
        }
    }
    false
}

/// Look for `<word> <digit>` in `sentence_lower`. Used for ratio
/// detection where `<word>` is the language's ratio connector.
fn find_word_then_digit(sentence_lower: &str, word: &str) -> bool {
    let mut search_from = 0;
    while let Some(rel) = sentence_lower[search_from..].find(word) {
        let start = search_from + rel;
        let end = start + word.len();
        let prev_ok = start == 0
            || sentence_lower.as_bytes()[start - 1].is_ascii_whitespace()
            || sentence_lower.as_bytes()[start - 1] == b',';
        let next_byte = sentence_lower.as_bytes().get(end).copied();
        let next_ok = matches!(next_byte, Some(b' '));
        if prev_ok && next_ok {
            // Walk past the word + spaces, see if we land on a digit.
            let mut k = end + 1;
            while k < sentence_lower.len() && sentence_lower.as_bytes()[k] == b' ' {
                k += 1;
            }
            if k < sentence_lower.len() && sentence_lower.as_bytes()[k].is_ascii_digit() {
                return true;
            }
        }
        search_from = end;
    }
    false
}

/// Walk a sentence and return the first surviving candidate, or
/// `None` if every candidate was filtered.
fn first_unanchored_candidate(
    sentence: &str,
    min_value: u64,
    lookups: &LanguageLookups,
) -> Option<Candidate> {
    let bytes = sentence.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() {
            let token = scan_numeric_token(sentence, i);
            // Skip ordinal: digit run immediately followed by ASCII letter
            // (no whitespace).
            let suffix_letter = bytes.get(token.end).is_some_and(u8::is_ascii_alphabetic);
            if suffix_letter {
                i = token.end;
                continue;
            }
            // Skip year-shaped numerals.
            let is_year_shape = !token.had_separator
                && token.digits == 4
                && (YEAR_LOW..=YEAR_HIGH).contains(&token.value);
            if is_year_shape {
                i = token.end;
                continue;
            }
            // Skip figure-ref-preceded digits.
            if preceded_by_figure_ref(sentence, i, lookups.figure_refs) {
                i = token.end;
                continue;
            }
            // Apply the magnitude gate.
            if token.digits >= MIN_DIGITS && token.value >= min_value {
                return Some(Candidate {
                    offset: i,
                    kind: CandidateKind::LargeNumeral {
                        digits: token.digits,
                        value: token.value,
                    },
                });
            }
            i = token.end;
        } else if b.is_ascii_alphabetic() {
            let word_end = scan_word_end(bytes, i);
            let word = &sentence[i..word_end];
            if is_magnitude_word(word, lookups.magnitudes) {
                return Some(Candidate {
                    offset: i,
                    kind: CandidateKind::Magnitude,
                });
            }
            i = word_end;
        } else {
            // Advance one UTF-8 character. The leading byte tells us
            // the byte length.
            i += utf8_char_len(b);
        }
    }
    None
}

/// Result of [`scan_numeric_token`].
struct NumericToken {
    /// End byte offset (exclusive) in the sentence.
    end: usize,
    /// Number of digit characters consumed.
    digits: u32,
    /// Integer value (digits only, ignoring separators). Saturates at
    /// `u64::MAX` for absurdly long numerals.
    value: u64,
    /// True when the token contained at least one inter-digit
    /// separator — used to gate the year-shape skip.
    had_separator: bool,
}

/// Greedy scan of a numeric token starting at `start` (which must
/// point at an ASCII digit). Collapses thousands and decimal
/// separators (`,`, `.`, ASCII space, NBSP, thin space, narrow NBSP)
/// when they sit between two digit chars.
fn scan_numeric_token(sentence: &str, start: usize) -> NumericToken {
    let mut digits: u32 = 0;
    let mut value: u64 = 0;
    let mut had_separator = false;
    let mut end = start;
    let mut chars = sentence[start..].char_indices().peekable();
    let mut last_was_digit = false;

    while let Some((rel, ch)) = chars.peek().copied() {
        if ch.is_ascii_digit() {
            digits = digits.saturating_add(1);
            value = value
                .saturating_mul(10)
                .saturating_add(u64::from(ch as u8 - b'0'));
            end = start + rel + ch.len_utf8();
            last_was_digit = true;
            chars.next();
        } else if last_was_digit && is_numeric_separator(ch) {
            // Peek ahead — collapse the separator only if a digit
            // follows. Otherwise the separator is sentence punctuation
            // and the token ends.
            chars.next(); // consume the separator
            if let Some(&(_, next)) = chars.peek() {
                if next.is_ascii_digit() {
                    had_separator = true;
                    last_was_digit = false;
                    continue;
                }
            }
            break;
        } else {
            break;
        }
    }

    NumericToken {
        end,
        digits,
        value,
        had_separator,
    }
}

fn is_numeric_separator(ch: char) -> bool {
    matches!(ch, ',' | '.' | ' ' | '\u{a0}' | '\u{2009}' | '\u{202f}')
}

fn utf8_char_len(leading: u8) -> usize {
    // Continuation bytes (0x80..0xC0) should not appear as a leading
    // byte on valid UTF-8; treating them as 1 keeps the scanner
    // forward-progressing without panicking on malformed input.
    if leading < 0xC0 {
        1
    } else if leading < 0xE0 {
        2
    } else if leading < 0xF0 {
        3
    } else {
        4
    }
}

/// Find the end of an ASCII-letter word starting at `start`. FR
/// accented letters live outside ASCII so the simple scan stops at
/// them — acceptable here because magnitude words (`million`,
/// `milliard`, …) are pure-ASCII in both languages.
fn scan_word_end(bytes: &[u8], start: usize) -> usize {
    let mut j = start;
    while j < bytes.len() && (bytes[j].is_ascii_alphabetic() || bytes[j] == b'\'') {
        j += 1;
    }
    j
}

fn is_magnitude_word(word: &str, magnitudes: &[&str]) -> bool {
    let lower = word.to_ascii_lowercase();
    magnitudes.iter().any(|&m| m == lower)
}

/// True when one of `figure_refs` ends within `FIGURE_REF_LOOKBACK_BYTES`
/// before `digit_offset`, separated only by whitespace or punctuation
/// commonly seen between a ref label and its number (`.`, `:`, `°`).
fn preceded_by_figure_ref(sentence: &str, digit_offset: usize, figure_refs: &[&str]) -> bool {
    let lookback_start = digit_offset.saturating_sub(FIGURE_REF_LOOKBACK_BYTES);
    // Walk backward through whitespace / connector chars to find the
    // first preceding word (or symbol).
    let prefix = &sentence[lookback_start..digit_offset];
    let lower = prefix.to_lowercase();
    let trimmed = lower.trim_end_matches(|c: char| {
        c.is_whitespace() || c == '.' || c == ':' || c == '°' || c == '#'
    });
    for &keyword in figure_refs {
        if trimmed.ends_with(keyword) {
            // Make sure it's a whole-word match — char before keyword
            // must be a non-letter or string start.
            let kw_start = trimmed.len().saturating_sub(keyword.len());
            let prev_byte = trimmed.as_bytes().get(kw_start.wrapping_sub(1)).copied();
            let whole_word = prev_byte.map_or(true, |b| !(b as char).is_ascii_alphabetic());
            if whole_word {
                return true;
            }
        }
    }
    // Special-case the bare `#` symbol — `#1234` is an issue / PR ref.
    if prefix.trim_end().ends_with('#') {
        return true;
    }
    false
}

/// Tokenize `sentence` into ASCII-alphabetic-and-symbol whole words
/// (length ≥ 1). Symbols like `m²`, `°C` are emitted as their visible
/// forms because the iterator splits on whitespace and ASCII
/// punctuation only — the unit characters survive.
fn tokenize_words(sentence: &str) -> impl Iterator<Item = &str> + '_ {
    sentence
        .split(|c: char| {
            c.is_whitespace() || matches!(c, ',' | ';' | '!' | '?' | '(' | ')' | '[' | ']')
        })
        .filter(|s| !s.is_empty())
}

fn build_diagnostic(
    source: &SourceFile,
    sentence_text: &str,
    sentence_line: u32,
    sentence_column: u32,
    candidate: Candidate,
    section: Option<&str>,
) -> Diagnostic {
    let prefix = &sentence_text[..candidate.offset];
    let prefix_graphemes = u32::try_from(prefix.graphemes(true).count()).unwrap_or(u32::MAX);
    let column = sentence_column.saturating_add(prefix_graphemes);
    let length = u32::try_from(sentence_text.graphemes(true).count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), sentence_line, column, length);
    let message = match candidate.kind {
        CandidateKind::LargeNumeral { digits, value } => format!(
            "Large numeral ({digits}-digit, value ≈ {value}) appears with no anchor in this sentence \
             (no unit, percentage, ratio, or comparison phrase). plain-language guidance recommends \
             giving large numbers a comparison or denominator the reader can ground."
        ),
        CandidateKind::Magnitude => String::from(
            "Magnitude word appears with no anchor in this sentence (no unit, percentage, ratio, \
             or comparison phrase). plain-language guidance recommends pairing magnitude words \
             with a unit or a comparison the reader can ground.",
        ),
    };
    let diag = Diagnostic::new(
        LargeNumberUnanchored::ID,
        Severity::Warning,
        location,
        message,
    );
    match section {
        Some(title) => diag.with_section(title),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::Category;

    fn lint_en(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        LargeNumberUnanchored::for_profile(profile).check(&document, Language::En)
    }

    fn lint_fr(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        LargeNumberUnanchored::for_profile(profile).check(&document, Language::Fr)
    }

    fn lint_md_en(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        LargeNumberUnanchored::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case_and_category_prefixed() {
        assert_eq!(
            LargeNumberUnanchored::ID,
            "readability.large-number-unanchored"
        );
    }

    #[test]
    fn ships_as_experimental() {
        assert_eq!(
            LargeNumberUnanchored::for_profile(Profile::Public).status(),
            Status::Experimental
        );
    }

    #[test]
    fn carries_dyscalculia_and_general_condition_tags() {
        let rule = LargeNumberUnanchored::for_profile(Profile::Public);
        assert_eq!(
            rule.condition_tags(),
            &[ConditionTag::Dyscalculia, ConditionTag::General]
        );
    }

    #[test]
    fn category_is_readability() {
        let diags = lint_en(
            "Le projet a couté 1234567 selon le rapport final.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1, "got {diags:?}");
        assert_eq!(diags[0].category(), Category::Readability);
    }

    #[test]
    fn unknown_language_skips_silently() {
        let document = parse_plain(
            "The budget reached 4,800,000,000 last year.",
            SourceFile::Anonymous,
        );
        let diags =
            LargeNumberUnanchored::for_profile(Profile::Public).check(&document, Language::Unknown);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).min_value, 100_000);
        assert_eq!(Config::for_profile(Profile::Public).min_value, 10_000);
        assert_eq!(Config::for_profile(Profile::Falc).min_value, 1_000);
    }

    // -------- numeric candidate detection --------

    #[test]
    fn large_unanchored_numeral_fires() {
        let diags = lint_en("The budget reached 4800000000 last year.", Profile::Public);
        assert_eq!(diags.len(), 1, "got {diags:?}");
        assert_eq!(diags[0].rule_id, LargeNumberUnanchored::ID);
        assert_eq!(diags[0].severity, Severity::Warning);
        assert!(diags[0].message.contains("Large numeral"));
    }

    #[test]
    fn small_numeral_under_threshold_does_not_fire() {
        // 9999 is below the public threshold of 10000.
        let diags = lint_en("The room held 9999 attendees comfortably.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn three_digit_number_does_not_fire() {
        let diags = lint_en("The team filed 250 issues last sprint.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    // -------- thousands separators --------

    #[test]
    fn en_comma_thousands_separator_collapses() {
        // `1,000,000` is one 7-digit token, value 1_000_000.
        let diags = lint_en("The fund crossed 1,000,000 last quarter.", Profile::Public);
        assert_eq!(diags.len(), 1, "got {diags:?}");
    }

    #[test]
    fn fr_nbsp_thousands_separator_collapses() {
        // FR uses NBSP between thousands groups: `1\u{a0}000\u{a0}000`.
        let diags = lint_fr(
            "Le fonds a dépassé 1\u{a0}000\u{a0}000 le trimestre dernier.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1, "got {diags:?}");
    }

    // -------- year guard --------

    #[test]
    fn four_digit_year_is_not_a_candidate() {
        // 2024 fits the year-shape guard (1000..=2999, no separator).
        let diags = lint_en("In 2024 the team shipped the migration.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn year_with_thousands_separator_still_fires_when_large() {
        // `1,066` carries a thousands separator → not year-shaped →
        // value 1066 below public's 10000 threshold → still no fire.
        let diags = lint_en("About 1,066 troops landed near Hastings.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    // -------- ordinal guard --------

    #[test]
    fn ordinal_is_not_a_candidate() {
        // The 12345th-place finisher is a contrived example, but the
        // ordinal suffix lets us verify the digit-then-letter skip.
        let diags = lint_en("Look at the 12345th attendee on the list.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    // -------- figure / page / section refs --------

    #[test]
    fn page_reference_is_skipped() {
        let diags = lint_en("See page 12345 in the appendix.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn figure_reference_is_skipped() {
        let diags = lint_en("See figure 12345 in chapter 2.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn issue_hash_reference_is_skipped() {
        let diags = lint_en("Closes #12345 in the tracker.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    // -------- sentence-level anchors --------

    #[test]
    fn percentage_anchors_the_whole_sentence() {
        let diags = lint_en(
            "The fund grew 4800000000 dollars, which is 12% of GDP.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn currency_symbol_anchors() {
        let diags = lint_en(
            "The fund crossed $4800000000 last quarter.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn unit_token_anchors() {
        let diags = lint_en(
            "The reservoir holds 4800000000 L of fresh water.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn ratio_pattern_out_of_anchors_en() {
        let diags = lint_en(
            "About 4800000000 out of 7900000000 people had access.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn ratio_pattern_sur_anchors_fr() {
        let diags = lint_fr(
            "Environ 4800000000 sur 7900000000 personnes y ont accès.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn comparator_phrase_anchors_en() {
        let diags = lint_en(
            "The fund crossed 4800000000 last quarter, roughly the size of a small economy.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn comparator_phrase_anchors_fr() {
        let diags = lint_fr(
            "Le fonds a atteint 4800000000 le trimestre dernier, soit environ le PIB d'un petit pays.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    // -------- magnitude words --------

    #[test]
    fn magnitude_word_alone_fires() {
        let diags = lint_en(
            "The proposal mentioned several billion in vague spending.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1, "got {diags:?}");
        assert!(diags[0].message.contains("Magnitude word"));
    }

    #[test]
    fn fr_milliard_alone_fires() {
        let diags = lint_fr(
            "La proposition mentionne plusieurs milliards de dépenses vagues.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1, "got {diags:?}");
    }

    #[test]
    fn magnitude_word_with_unit_anchored() {
        let diags = lint_en(
            "The plan budgets several million kg of feedstock for the year.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    // -------- profile thresholds --------

    #[test]
    fn devdoc_profile_is_more_tolerant() {
        // 50 000 fires Public (10 000) but not DevDoc (100 000).
        let text = "The cluster ran 50000 jobs across the weekend window.";
        assert!(!lint_en(text, Profile::Public).is_empty());
        assert!(lint_en(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 5000 fires FALC (1000) but not Public (10 000).
        let text = "The cluster ran 5000 jobs across the weekend window.";
        assert!(lint_en(text, Profile::Public).is_empty());
        assert!(!lint_en(text, Profile::Falc).is_empty());
    }

    // -------- structure / parser interaction --------

    #[test]
    fn fenced_code_block_excluded() {
        let md = "Plain prose intro.\n\n\
                  ```\n\
                  let big = 4800000000;\n\
                  ```\n\n\
                  Plain prose outro.";
        assert!(lint_md_en(md, Profile::Public).is_empty());
    }

    #[test]
    fn one_diagnostic_per_sentence() {
        // Two unanchored sentences → two diagnostics, one each.
        let text = "The fund crossed 4800000000 last quarter. \
                    The reserve grew 5500000000 the next year.";
        let diags = lint_en(text, Profile::Public);
        assert_eq!(diags.len(), 2, "got {diags:?}");
    }

    #[test]
    fn position_points_at_first_candidate_token() {
        let diags = lint_en("The fund crossed 4800000000 last quarter.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].location.line, 1);
        // "The fund crossed " is 17 graphemes → first digit at column 18.
        assert_eq!(diags[0].location.column, 18);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Mild paragraph quotes 250 attendees only.\n\n\
                    Heavy paragraph cites 4800000000 in vague spending across regions.\n\n\
                    Plain prose without any large numbers at all here.";
        let document = parse_markdown(text, SourceFile::Anonymous);
        let diags =
            LargeNumberUnanchored::for_profile(Profile::Public).check(&document, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
