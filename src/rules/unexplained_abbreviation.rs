//! Rule: `unexplained-abbreviation`.
//!
//! Flags uppercase acronym-like tokens not present in a whitelist. An
//! undefined acronym forces the reader to guess or lookup, breaking the
//! flow. The v0.1 form is pattern-based; a two-pass definition-aware
//! version is tracked as F9 in `ROADMAP.md`.
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

/// Technical and accessibility acronyms accepted in `dev-doc` prose.
static TECH_WHITELIST: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // Web / protocol
        "URL", "HTML", "CSS", "JSON", "XML", "HTTP", "HTTPS", "UTF", "IO",
        // Programming / tooling
        "API", "CLI", "GUI", "OS", "CPU", "RAM", "SSD", "USB", "IDE", "SDK", "CI", "CD", "LRU",
        "WASM", "MIT", // AI / language tech
        "LLM", "NLP", // Accessibility and readability standards
        "WCAG", "WAI", "ARIA", "RGAA", "EAA", "FALC", "AA", "AAA", "ADHD",
        // Ubiquitous engineering-practice initialisms
        "YAGNI", "DRY", "KISS", "SOLID", "TDD", "BDD", "MVP", "MSRV",
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
    pub const ID: &'static str = "unexplained-abbreviation";
}

impl Rule for UnexplainedAbbreviation {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let min = self.config.min_length.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            for (byte_offset, token) in iter_acronyms(&paragraph.text) {
                let letter_count =
                    u32::try_from(token.chars().filter(|c| c.is_alphabetic()).count())
                        .unwrap_or(u32::MAX);
                if letter_count < min {
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
    let message = format!(
        "Acronym \"{token}\" is not whitelisted. Define it on first use, e.g. \"{token} \
         (Full Expansion)\"."
    );
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
        assert_eq!(UnexplainedAbbreviation::ID, "unexplained-abbreviation");
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

    #[test]
    fn category_is_lexical() {
        let diags = lint("Fix the ZQX.", Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Lexical);
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
