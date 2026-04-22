//! Rule: `unexplained-abbreviation`.
//!
//! Flags uppercase acronym-like tokens that are not whitelisted and
//! not defined in the document. An undefined acronym forces the
//! reader to guess or look up, breaking the flow.
//!
//! The v0.2 rule is two-pass (F9): a pre-scan of the whole document
//! collects acronyms defined in either canonical form —
//! `Full Expansion (ACRONYM)` or `ACRONYM (Full Expansion)` — and
//! subsequent occurrences of those tokens are silenced. The baseline
//! whitelist is narrower than v0.1 (F31): only the ubiquitous
//! infrastructure stack ships in `dev-doc`. Project-specific
//! acronyms belong in `[rules.unexplained-abbreviation].whitelist`.
//!
//! See [`RULES.md`](../../RULES.md#unexplained-abbreviation) for the
//! rule's rationale and references (WCAG 3.1.4, RGAA 9.4).

use std::collections::HashSet;
use std::num::NonZeroU32;
use std::sync::LazyLock;

use crate::config::Profile;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Common acronyms and all-caps emphasis keywords accepted without
/// definition at every profile that ships a baseline. Includes RFC 2119
/// style requirement keywords (MUST, SHALL, …) which appear capitalized
/// for emphasis rather than as abbreviations.
static COMMON_WHITELIST: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // Common abbreviations
        "PDF",
        "SMS",
        "GPS",
        "ID",
        "OK",
        "FAQ",
        // RFC 2119 / convention keywords used as emphasis
        "MUST",
        "SHALL",
        "SHOULD",
        "MAY",
        "NOT",
        "REQUIRED",
        "RECOMMENDED",
        "OPTIONAL",
    ]
    .into_iter()
    .collect()
});

/// Infrastructure-stack acronyms accepted in `dev-doc` prose without
/// explicit definition. F31 narrowed this list to items a general
/// tech-adjacent reader can be expected to know — the web stack, the
/// hardware stack, and common transport protocols. Domain-specific
/// initialisms (accessibility standards, engineering-practice
/// acronyms, AI/language tech) now belong in the user whitelist via
/// `[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml`.
static TECH_WHITELIST: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // Web / protocol
        "URL", "HTML", "CSS", "JSON", "XML", "HTTP", "HTTPS", "UTF", "IO",
        // Programming / tooling
        "API", "CLI", "GUI", "OS", "CPU", "RAM", "SSD", "USB", "IDE", "SDK", "CI", "CD",
    ]
    .into_iter()
    .collect()
});

/// Configuration for [`UnexplainedAbbreviation`].
#[derive(Debug, Clone)]
pub struct Config {
    /// Minimum letter count for a token to be treated as an acronym.
    pub min_length: NonZeroU32,

    /// User-provided additional whitelist entries (uppercase).
    pub whitelist: Vec<String>,

    /// Profile-driven baseline whitelist.
    baseline: BaselineWhitelist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BaselineWhitelist {
    /// `Common` acronyms only (PDF, SMS, …).
    Minimal,
    /// `Common` plus technical IT acronyms.
    Extended,
    /// No baseline — every acronym must be defined.
    None,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let (min_length, baseline) = match profile {
            Profile::DevDoc => (3, BaselineWhitelist::Extended),
            Profile::Public => (2, BaselineWhitelist::Minimal),
            Profile::Falc => (2, BaselineWhitelist::None),
        };
        Self {
            min_length: NonZeroU32::new(min_length).expect("non-zero literal"),
            whitelist: Vec::new(),
            baseline,
        }
    }

    fn is_whitelisted(&self, token: &str) -> bool {
        if self.whitelist.iter().any(|w| w == token) {
            return true;
        }
        match self.baseline {
            BaselineWhitelist::None => false,
            BaselineWhitelist::Minimal => COMMON_WHITELIST.contains(token),
            BaselineWhitelist::Extended => {
                COMMON_WHITELIST.contains(token) || TECH_WHITELIST.contains(token)
            },
        }
    }

    /// Append project-specific entries to the user whitelist. Additive
    /// over the profile baseline — callers use this to restore narrower
    /// acronyms that F31 moved out of the shipped baseline.
    #[must_use]
    pub fn with_extra_whitelist(mut self, extra: Vec<String>) -> Self {
        self.whitelist.extend(extra);
        self
    }
}

/// The [`UnexplainedAbbreviation`] rule.
#[derive(Debug, Clone)]
pub struct UnexplainedAbbreviation {
    config: Config,
}

impl UnexplainedAbbreviation {
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
    pub const ID: &'static str = "lexicon.unexplained-abbreviation";
}

impl Rule for UnexplainedAbbreviation {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let min = self.config.min_length.get();

        // Pass 1 (F9): collect acronyms defined anywhere in the
        // document in either canonical form. A single definition
        // silences every subsequent occurrence, matching how readers
        // actually use documentation (scroll up to find the expansion
        // once, remember it thereafter).
        let defined = collect_defined_acronyms(document, min);

        // Pass 2: emit diagnostics for the rest.
        let mut diagnostics = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            for (byte_offset, token) in iter_acronyms(&paragraph.text) {
                let letter_count =
                    u32::try_from(token.chars().filter(|c| c.is_alphabetic()).count())
                        .unwrap_or(u32::MAX);
                if letter_count < min {
                    continue;
                }
                if defined.contains(token) {
                    continue;
                }
                if self.config.is_whitelisted(token) {
                    continue;
                }
                let (line_offset, column) = line_column_at(&paragraph.text, byte_offset);
                let line = paragraph.start_line.saturating_add(line_offset);
                diagnostics.push(build_diagnostic(
                    &document.source,
                    line,
                    column,
                    token,
                    section_title,
                ));
            }
        }

        diagnostics.sort_by_key(|d| (d.location.line, d.location.column));
        diagnostics
    }
}

/// Pre-scan the document and collect every acronym that is explicitly
/// defined in either canonical form:
///
/// - `Full Expansion (ACRONYM)` — the expansion precedes, the acronym
///   is in parentheses. Example: `World Wide Web (WWW)`.
/// - `ACRONYM (Full Expansion)` — the acronym precedes, the expansion
///   is in parentheses. Example: `WWW (World Wide Web)`.
///
/// The "expansion" side must contain at least two alphabetic words so
/// that throwaway notes like `SEO (check later)` do not accidentally
/// count as definitions.
fn collect_defined_acronyms(document: &Document, min_letters: u32) -> HashSet<String> {
    let mut defined = HashSet::new();
    for (paragraph, _section) in document.paragraphs_with_section() {
        let text = paragraph.text.as_str();
        collect_defined_in_text(text, min_letters, &mut defined);
    }
    defined
}

fn collect_defined_in_text(text: &str, min_letters: u32, out: &mut HashSet<String>) {
    // Collect every acronym with its byte span in a single pass, then
    // inspect neighbouring parenthesised phrases to decide whether the
    // acronym is defined.
    let acronyms: Vec<(usize, &str)> = iter_acronyms(text)
        .filter(|(_, tok)| {
            let letters = u32::try_from(tok.chars().filter(|c| c.is_alphabetic()).count())
                .unwrap_or(u32::MAX);
            letters >= min_letters
        })
        .collect();
    let bytes = text.as_bytes();
    for &(start, token) in &acronyms {
        let end = start + token.len();

        // Form 1: `ACRONYM (expansion)` — inspect what follows.
        if let Some(paren_open) = next_non_space(bytes, end) {
            if bytes.get(paren_open) == Some(&b'(') {
                if let Some(paren_close) = find_matching_close(bytes, paren_open + 1) {
                    let inner = &text[paren_open + 1..paren_close];
                    if has_two_alpha_words(inner) {
                        out.insert(token.to_string());
                        continue;
                    }
                }
            }
        }

        // Form 2: `Expansion (ACRONYM)` — inspect the immediate
        // surroundings: we must be inside parentheses preceded by at
        // least two alphabetic words.
        if start > 0 && bytes.get(start - 1) == Some(&b'(') {
            if let Some(paren_close) = find_matching_close(bytes, start) {
                if paren_close == end {
                    // Walk left past the `(` and any whitespace, then
                    // inspect the phrase up to the previous sentence
                    // boundary.
                    let before = &text[..start.saturating_sub(1)];
                    if has_two_alpha_words(trim_to_definition_head(before)) {
                        out.insert(token.to_string());
                    }
                }
            }
        }
    }
}

/// Return the byte index of the first non-space character at or after
/// `start`, or `None` if only whitespace remains.
fn next_non_space(bytes: &[u8], start: usize) -> Option<usize> {
    (start..bytes.len()).find(|&i| !matches!(bytes[i], b' ' | b'\t'))
}

/// Find the matching `)` for an `(` opened at `open_plus_one - 1`.
/// Handles one level of nesting — enough for the `Definition (outer
/// (nested) thing)` edge case without a full paren stack.
fn find_matching_close(bytes: &[u8], start: usize) -> Option<usize> {
    let mut depth: i32 = 1;
    let mut i = start;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            },
            b'\n' => return None, // definitions don't span paragraphs
            _ => {},
        }
        i += 1;
    }
    None
}

/// Trim to the phrase immediately preceding the opening `(` — walk
/// back to the nearest sentence terminator, colon, or newline so that
/// a prior sentence cannot be mistaken for the expansion.
fn trim_to_definition_head(before: &str) -> &str {
    let cut = before
        .rfind(['.', '!', '?', ':', '\n'])
        .map_or(0, |i| i + 1);
    before[cut..].trim()
}

/// Does `text` contain at least two whitespace-separated tokens that
/// start with an alphabetic character? Short parenthetical notes like
/// `(yes)`, `(TBD)`, `(check later)` with fewer than two alpha words
/// fail this test, preventing spurious "definitions".
fn has_two_alpha_words(text: &str) -> bool {
    text.split_whitespace()
        .filter(|w| w.chars().next().is_some_and(char::is_alphabetic))
        .count()
        >= 2
}

/// Iterate uppercase acronym-shaped tokens in `text`, yielding
/// `(byte_offset, token)` pairs. A token is a maximal run of ASCII
/// uppercase letters or digits, bounded by non-alphanumeric characters,
/// containing at least one letter.
fn iter_acronyms(text: &str) -> impl Iterator<Item = (usize, &str)> {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    std::iter::from_fn(move || {
        while i < len {
            // Skip anything that cannot start a token.
            while i < len && !is_acronym_byte(bytes[i]) {
                // Advance by the UTF-8 char at `i` to stay on char boundaries.
                let step = utf8_char_len(bytes[i]);
                i += step;
            }
            if i >= len {
                return None;
            }
            // Require that the byte before the run is not a word character, to
            // avoid matching inside mixed-case identifiers (e.g. the "API" in
            // "myAPIcall"). We can check bytes directly here because word
            // characters we care about (ASCII letters/digits) are all ASCII.
            let start = i;
            if start > 0 && is_identifier_byte(bytes[start - 1]) {
                // Not a word boundary (letter, digit, or underscore to the
                // left — likely part of a filename / identifier).
                while i < len && is_acronym_byte(bytes[i]) {
                    i += 1;
                }
                continue;
            }
            // Consume the acronym run.
            while i < len && is_acronym_byte(bytes[i]) {
                i += 1;
            }
            // Right boundary: same rule.
            if i < len && is_identifier_byte(bytes[i]) {
                continue;
            }
            // Skip tokens immediately followed by a dot plus a lowercase/digit,
            // which looks like a file extension (`ROADMAP.md`, `API.json`).
            // A trailing dot followed by whitespace or end-of-text is treated
            // as sentence-ending punctuation and does not trigger this skip.
            if i + 1 < len && bytes[i] == b'.' && is_extension_byte(bytes[i + 1]) {
                continue;
            }
            let slice = &text[start..i];
            if slice.chars().any(|c| c.is_ascii_uppercase()) {
                return Some((start, slice));
            }
        }
        None
    })
}

const fn is_acronym_byte(b: u8) -> bool {
    b.is_ascii_uppercase() || b.is_ascii_digit()
}

const fn is_identifier_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

const fn is_extension_byte(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit()
}

const fn utf8_char_len(first_byte: u8) -> usize {
    if first_byte < 0x80 {
        1
    } else if first_byte < 0xE0 {
        2
    } else if first_byte < 0xF0 {
        3
    } else {
        4
    }
}

fn line_column_at(text: &str, byte_offset: usize) -> (u32, u32) {
    let capped = byte_offset.min(text.len());
    let prefix = &text[..capped];
    #[allow(clippy::naive_bytecount)]
    let line_offset =
        u32::try_from(prefix.bytes().filter(|&b| b == b'\n').count()).unwrap_or(u32::MAX);
    let current_line_start = prefix.rfind('\n').map_or(0, |pos| pos + 1);
    let column =
        u32::try_from(text[current_line_start..capped].chars().count() + 1).unwrap_or(u32::MAX);
    (line_offset, column)
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    token: &str,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(token.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!("Acronym \"{token}\" is not defined on first use.");
    let diag = Diagnostic::new(
        UnexplainedAbbreviation::ID,
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
    use crate::parser::parse_plain;
    use crate::types::SourceFile;

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        UnexplainedAbbreviation::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(
            UnexplainedAbbreviation::ID,
            "lexicon.unexplained-abbreviation"
        );
    }

    #[test]
    fn prose_without_acronyms_does_not_trigger() {
        assert!(lint("A quiet sentence of ordinary words.", Profile::Public).is_empty());
    }

    #[test]
    fn whitelisted_acronym_does_not_trigger() {
        // PDF is in the common whitelist at every profile that ships one.
        assert!(lint("Open the PDF file.", Profile::Public).is_empty());
    }

    #[test]
    fn unknown_acronym_triggers() {
        // ZQX is not whitelisted anywhere.
        let diags = lint("Send it through the ZQX adapter.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("ZQX"));
    }

    #[test]
    fn dev_doc_allows_tech_acronyms() {
        // HTTP and API pass dev-doc but not FALC.
        let text = "Call the HTTP API.";
        assert!(lint(text, Profile::DevDoc).is_empty());
        // FALC has an empty whitelist — both are flagged.
        let falc = lint(text, Profile::Falc);
        assert_eq!(falc.len(), 2);
    }

    #[test]
    fn public_does_not_allow_tech_acronyms_by_default() {
        // Public: min_length 2, Minimal whitelist. HTTP is not in it.
        let diags = lint("Check the HTTP status.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("HTTP"));
    }

    #[test]
    fn min_length_is_respected() {
        // Dev-doc min_length is 3: "OK" (2 letters) would not trigger even if
        // unlisted — and it's actually whitelisted anyway.
        let cfg = Config::for_profile(Profile::DevDoc);
        assert_eq!(cfg.min_length.get(), 3);
        // A 2-letter acronym under dev-doc: not counted as an acronym at all.
        let diags = lint("The ZQ panel is broken.", Profile::DevDoc);
        assert!(diags.is_empty());
    }

    #[test]
    fn mixed_case_tokens_are_ignored() {
        // WiFi, iPhone, camelCase: not all-caps, so not acronym-shaped here.
        let diags = lint("Connect via WiFi to the iPhone.", Profile::Public);
        assert!(diags.is_empty());
    }

    #[test]
    fn embedded_in_word_is_ignored() {
        // "myAPIcall": API is inside a mixed-case identifier — left boundary
        // is a lowercase letter, so it's not an acronym token here.
        let diags = lint("The myAPIcall helper is deprecated.", Profile::DevDoc);
        assert!(diags.is_empty());
    }

    #[test]
    fn trailing_digits_count_as_part_of_the_acronym() {
        // "IPv4" has lowercase — ignored. "IP4" would be all-caps+digits.
        let diags = lint("The IP4 field is missing.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("IP4"));
    }

    #[test]
    fn multiple_occurrences_all_flagged() {
        let diags = lint("Use ZQX now. Also ZQX later. ZQX again.", Profile::Public);
        assert_eq!(diags.len(), 3);
    }

    #[test]
    fn user_whitelist_silences_entry() {
        let cfg = Config {
            whitelist: vec!["ZQX".to_string()],
            ..Config::for_profile(Profile::Public)
        };
        let doc = parse_plain("Send it through the ZQX.", SourceFile::Anonymous);
        let diags = UnexplainedAbbreviation::new(cfg).check(&doc, Language::En);
        assert!(diags.is_empty());
    }

    // --- F9: two-pass definition detection ---

    #[test]
    fn definition_with_expansion_first_silences_rule() {
        // "World Wide Web (WWW)" defines WWW → subsequent WWW silenced.
        let text = "The World Wide Web (WWW) is huge. The WWW is everywhere.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn definition_with_acronym_first_silences_rule() {
        // "WWW (World Wide Web)" also counts.
        let text = "WWW (World Wide Web) powers the internet. WWW is universal.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn definition_silences_even_prior_occurrences() {
        // Definition-anywhere → doc-scoped silencing. A reader who hits
        // "WWW" on line 1 can scroll to line 2's expansion.
        let text = "The WWW is everywhere. Note: WWW (World Wide Web).";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn short_parenthetical_note_is_not_a_definition() {
        // "(TBD)" is too short — must not count as a definition of ZQX.
        let text = "The ZQX (TBD). Later the ZQX acts up.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn definition_does_not_carry_across_sentence_boundary_on_the_left() {
        // "A prior sentence. Other text (ZQX)" must NOT count as a
        // definition of ZQX — the expansion candidate is only the
        // current sentence's leading words, not everything before.
        let text = "A prior sentence. Foo (ZQX). Use ZQX elsewhere.";
        let diags = lint(text, Profile::Public);
        // "Foo" alone is one word — not enough for a definition.
        assert_eq!(diags.len(), 2);
    }

    // --- F31: narrowed baseline + additive user whitelist ---

    #[test]
    fn baseline_no_longer_ships_accessibility_acronyms() {
        // WCAG was in the pre-F31 tech whitelist; now it must be flagged
        // in dev-doc unless the user restored it.
        let diags = lint("Follow WCAG guidelines.", Profile::DevDoc);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("WCAG"));
    }

    #[test]
    fn baseline_still_ships_web_stack() {
        // URL/HTML/HTTP/API remain in the baseline — F31 only removed
        // domain-specific initialisms.
        assert!(lint("The HTTP URL hits an API.", Profile::DevDoc).is_empty());
    }

    #[test]
    fn with_extra_whitelist_restores_project_acronyms() {
        let rule = UnexplainedAbbreviation::new(
            Config::for_profile(Profile::DevDoc)
                .with_extra_whitelist(vec!["WCAG".to_string(), "ARIA".to_string()]),
        );
        let doc = parse_plain("WCAG and ARIA apply.", SourceFile::Anonymous);
        assert!(rule.check(&doc, Language::En).is_empty());
    }

    #[test]
    fn category_is_lexicon() {
        let diags = lint("Fix the ZQX.", Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Lexicon);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Check ZQX today. Then HTTP the API and read the FAQ.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
