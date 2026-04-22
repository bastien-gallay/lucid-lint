//! Rule: `mixed-numeric-format`.
//!
//! Flags a sentence that mixes digit numerals (`42`, `3.14`, `1,000`)
//! with spelled-out numerals (`two`, `trois`, `twenty`, `cent`) in the
//! same sentence. Presenting numbers inconsistently within a single
//! sentence forces the reader to switch surface forms mid-clause and
//! re-anchor the referent — a documented load for readers with
//! dyscalculia and for general plain-language comprehension.
//!
//! Grounding: CDC Clear Communication Index (item 3.5, "present numbers
//! consistently throughout") and plainlanguage.gov (Chapter 4, "use
//! numerals").
//!
//! The detection is intentionally narrow: we require a digit-numeric
//! token and a spelled-out numeral to co-occur in the same sentence.
//! `one` (EN) and `un` / `une` (FR) are excluded from the spelled-out
//! word list because they double as indefinite pronouns / articles;
//! see [`crate::language::en::SPELLED_NUMERALS`] and
//! [`crate::language::fr::SPELLED_NUMERALS`].
//!
//! See [`RULES.md`](../../RULES.md#mixed-numeric-format) for the
//! reference entry.

use unicode_segmentation::UnicodeSegmentation;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity};

/// The [`MixedNumericFormat`] rule.
///
/// The rule has no configurable threshold: the detection fires whenever
/// a single sentence contains both a digit-numeric token and a
/// spelled-out numeral. Profile presets exist only to preserve the
/// canonical rule-construction shape used across the codebase.
#[derive(Debug, Clone, Copy, Default)]
pub struct MixedNumericFormat;

impl MixedNumericFormat {
    /// Build the rule.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Build the rule using the preset for the given profile.
    ///
    /// Profile is currently inert for this rule but kept for API
    /// consistency with every other rule's constructor.
    #[must_use]
    pub const fn for_profile(_profile: Profile) -> Self {
        Self::new()
    }

    /// The rule identifier.
    pub const ID: &'static str = "structure.mixed-numeric-format";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[ConditionTag::Dyscalculia, ConditionTag::General];
}

impl Rule for MixedNumericFormat {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let spelled = spelled_list_for(language);
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
            for sentence in sentences {
                let Some((digit_sample, spelled_sample)) = find_mixed_pair(&sentence.text, spelled)
                else {
                    continue;
                };
                let length =
                    u32::try_from(sentence.text.graphemes(true).count()).unwrap_or(u32::MAX);
                let location = Location::new(
                    document.source.clone(),
                    sentence.line,
                    sentence.column,
                    length,
                );
                let message = format!(
                    "Sentence mixes a digit numeral (`{digit_sample}`) with a spelled-out numeral \
                     (`{spelled_sample}`). Plain-language and CDC Clear Communication Index \
                     guidance recommends using one form consistently within a sentence."
                );
                let mut diag = Diagnostic::new(Self::ID, Severity::Warning, location, message);
                if let Some(title) = section_title {
                    diag = diag.with_section(title);
                }
                diagnostics.push(diag);
            }
        }

        diagnostics
    }
}

fn spelled_list_for(language: Language) -> &'static [&'static str] {
    match language {
        Language::Fr => fr::SPELLED_NUMERALS,
        // Unknown falls back to English; matches the project-wide policy
        // documented in `crate::language::default_language`.
        Language::En | Language::Unknown => en::SPELLED_NUMERALS,
    }
}

/// Scan `text` for the first digit-numeric token and first spelled
/// numeral. Returns `Some((digit, spelled))` iff both kinds are present.
///
/// Tokenisation walks the string character-by-character instead of
/// relying on `unicode_words`, because the latter splits thousands
/// separators inside numbers (`1,000` → `1`, `000`) and strips decimals
/// in a way that defeats the "digit-numeric token" detection.
fn find_mixed_pair<'a>(
    text: &'a str,
    spelled: &'static [&'static str],
) -> Option<(&'a str, &'a str)> {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut digit_sample: Option<&str> = None;
    let mut spelled_sample: Option<&str> = None;
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        if b.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < len {
                let c = bytes[i];
                if c.is_ascii_digit() {
                    i += 1;
                } else if (c == b'.' || c == b',' || c == b' ')
                    && i + 1 < len
                    && bytes[i + 1].is_ascii_digit()
                {
                    // Accept `.`, `,`, or narrow space as a decimal or
                    // thousands separator when followed by digits.
                    i += 2;
                } else {
                    break;
                }
            }
            if digit_sample.is_none() {
                digit_sample = Some(&text[start..i]);
            }
        } else if is_word_byte(b) {
            let start = i;
            i += 1;
            while i < len && is_word_byte(bytes[i]) {
                i += 1;
            }
            if spelled_sample.is_none() {
                let word = &text[start..i];
                if matches_spelled(word, spelled) {
                    spelled_sample = Some(word);
                }
            }
        } else {
            // Advance by one UTF-8 code point to keep the scanner
            // byte-safe. `is_word_byte` already rejected the leading
            // byte, so skipping by the UTF-8 length is correct.
            i += utf8_char_len(b);
        }

        if let (Some(d), Some(s)) = (digit_sample, spelled_sample) {
            return Some((d, s));
        }
    }

    None
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphabetic() || b >= 0x80
}

fn utf8_char_len(leading: u8) -> usize {
    if leading < 0x80 {
        1
    } else if leading < 0xC0 {
        // Continuation byte — should not be reached as a leading byte
        // on valid UTF-8, but keep the scanner forward-progressing.
        1
    } else if leading < 0xE0 {
        2
    } else if leading < 0xF0 {
        3
    } else {
        4
    }
}

fn matches_spelled(word: &str, spelled: &'static [&'static str]) -> bool {
    // Case-insensitive ASCII comparison is enough: every entry in
    // `SPELLED_NUMERALS` is lowercase ASCII (French accented forms
    // like `zéro` are absent from the shipped list).
    if !word.is_ascii() {
        return false;
    }
    spelled
        .iter()
        .any(|candidate| word.eq_ignore_ascii_case(candidate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        MixedNumericFormat::new().check(&document, language)
    }

    fn lint_md(text: &str, language: Language) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        MixedNumericFormat::new().check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(MixedNumericFormat::ID, "structure.mixed-numeric-format");
    }

    #[test]
    fn tags_carry_dyscalculia_and_general() {
        assert!(MixedNumericFormat::TAGS.contains(&ConditionTag::Dyscalculia));
        assert!(MixedNumericFormat::TAGS.contains(&ConditionTag::General));
    }

    #[test]
    fn category_is_structure() {
        let diags = lint("We received 5 reports and twelve complaints.", Language::En);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].category(), Category::Structure);
    }

    #[test]
    fn mix_in_en_sentence_triggers() {
        let diags = lint("We received 5 reports and twelve complaints.", Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("`5`"));
        assert!(diags[0].message.contains("`twelve`"));
    }

    #[test]
    fn mix_in_fr_sentence_triggers() {
        let diags = lint(
            "Nous avons reçu 5 rapports et douze plaintes.",
            Language::Fr,
        );
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("`5`"));
        assert!(diags[0].message.contains("`douze`"));
    }

    #[test]
    fn en_excludes_indefinite_pronoun_one() {
        // `one of` is an indefinite pronoun, not a numeral — no diagnostic
        // even though `42` is a digit numeral.
        assert!(lint("One of 42 readers noticed.", Language::En).is_empty());
    }

    #[test]
    fn fr_excludes_indefinite_article_un_une() {
        assert!(lint("Un lecteur sur 42 a remarqué.", Language::Fr).is_empty());
        assert!(lint("Une page parmi 42 a bougé.", Language::Fr).is_empty());
    }

    #[test]
    fn digits_only_sentence_does_not_trigger() {
        assert!(lint("We received 5 reports and 12 complaints.", Language::En).is_empty());
    }

    #[test]
    fn spelled_only_sentence_does_not_trigger() {
        assert!(lint(
            "We received five reports and twelve complaints.",
            Language::En
        )
        .is_empty());
    }

    #[test]
    fn decimal_number_counts_as_single_digit_token() {
        let diags = lint("Pi is about 3.14 across three decimals.", Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("`3.14`"));
        assert!(diags[0].message.contains("`three`"));
    }

    #[test]
    fn thousands_separator_comma_counts_as_single_token() {
        let diags = lint(
            "The archive holds 1,000 items across ten shelves.",
            Language::En,
        );
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("`1,000`"));
    }

    #[test]
    fn fr_thousands_narrow_space_counts_as_single_token() {
        let diags = lint(
            "L'archive contient 1 000 éléments sur dix étagères.",
            Language::Fr,
        );
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("`1 000`"));
    }

    #[test]
    fn mix_across_distinct_sentences_does_not_trigger() {
        // Each sentence is consistent on its own — no diagnostic.
        assert!(lint(
            "We received 5 reports. The three reviewers agreed.",
            Language::En
        )
        .is_empty());
    }

    #[test]
    fn two_separate_mixed_sentences_yield_two_diagnostics() {
        let diags = lint(
            "We saw 5 bugs and three regressions. Later 7 more and ten followups.",
            Language::En,
        );
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn fenced_code_block_content_is_ignored() {
        let md = "Intro paragraph.\n\n```\nfive + 5 = ten\n```\n\nPlain prose with 5 apples.\n";
        assert!(lint_md(md, Language::En).is_empty());
    }

    #[test]
    fn unknown_language_falls_back_to_english() {
        let diags = lint("We saw 5 bugs and three regressions.", Language::Unknown);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "We received 5 reports and twelve complaints. All clear.";
        let diags = lint(text, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
