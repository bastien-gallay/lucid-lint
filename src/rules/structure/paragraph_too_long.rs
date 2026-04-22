//! Rule: `paragraph-too-long`.
//!
//! Flags paragraphs that exceed either a sentence count or a word count
//! threshold. A paragraph is a mental chunk and a reprise unit for
//! interrupted readers; long paragraphs dilute reprise points.
//!
//! See [`RULES.md`](../../RULES.md#paragraph-too-long) for the rule's
//! rationale and thresholds.

use std::num::NonZeroU32;

use crate::config::Profile;
use crate::parser::{split_sentences, word_count, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`ParagraphTooLong`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum number of sentences per paragraph.
    pub max_sentences: NonZeroU32,

    /// Maximum number of words per paragraph.
    pub max_words: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let (sentences, words) = match profile {
            Profile::DevDoc => (7, 150),
            Profile::Public => (5, 100),
            Profile::Falc => (3, 60),
        };
        Self {
            max_sentences: NonZeroU32::new(sentences).expect("non-zero literal"),
            max_words: NonZeroU32::new(words).expect("non-zero literal"),
        }
    }
}

/// The [`ParagraphTooLong`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ParagraphTooLong {
    config: Config,
}

impl ParagraphTooLong {
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
    pub const ID: &'static str = "structure.paragraph-too-long";
}

impl Rule for ParagraphTooLong {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let max_sentences = self.config.max_sentences.get();
        let max_words = self.config.max_words.get();

        document
            .paragraphs_with_section()
            .filter_map(|(paragraph, section_title)| {
                let sentences =
                    u32::try_from(split_sentences(&paragraph.text, paragraph.start_line, 1).len())
                        .unwrap_or(u32::MAX);
                let words = word_count(&paragraph.text);
                if sentences <= max_sentences && words <= max_words {
                    return None;
                }
                Some(build_diagnostic(
                    &document.source,
                    paragraph.start_line,
                    &paragraph.text,
                    sentences,
                    words,
                    max_sentences,
                    max_words,
                    section_title,
                ))
            })
            .collect()
    }
}

#[allow(clippy::too_many_arguments)]
fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    paragraph_text: &str,
    sentences: u32,
    words: u32,
    max_sentences: u32,
    max_words: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(paragraph_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, 1, length);
    let message = match (sentences > max_sentences, words > max_words) {
        (true, true) => format!(
            "Paragraph is {sentences} sentences and {words} words (max {max_sentences} sentences, \
             {max_words} words). Consider splitting it."
        ),
        (true, false) => format!(
            "Paragraph is {sentences} sentences (max {max_sentences}). Consider splitting it."
        ),
        (false, true) => {
            format!("Paragraph is {words} words (max {max_words}). Consider splitting it.")
        },
        (false, false) => unreachable!("build_diagnostic called when no threshold is exceeded"),
    };
    let diag = Diagnostic::new(ParagraphTooLong::ID, Severity::Warning, location, message);
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
        let rule = ParagraphTooLong::for_profile(profile);
        rule.check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(ParagraphTooLong::ID, "structure.paragraph-too-long");
    }

    #[test]
    fn short_paragraph_does_not_trigger() {
        let text = "A short paragraph. With two sentences.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn exceeding_sentence_count_triggers() {
        // Public: max 5 sentences → 6 sentences trigger.
        let text = "One. Two. Three. Four. Five. Six.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("sentences"));
    }

    #[test]
    fn exceeding_word_count_triggers() {
        // Public: max 100 words. Build a 105-word single-sentence paragraph.
        let mut text = String::new();
        for _ in 0..35 {
            text.push_str("alpha beta gamma ");
        }
        text.push('.');
        let diags = lint(&text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("words"));
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 4 sentences: passes Public (5), fails FALC (3).
        let text = "One. Two. Three. Four.";
        assert!(lint(text, Profile::Public).is_empty());
        assert!(!lint(text, Profile::Falc).is_empty());
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_sentences.get(), 7);
        assert_eq!(Config::for_profile(Profile::DevDoc).max_words.get(), 150);
        assert_eq!(Config::for_profile(Profile::Public).max_sentences.get(), 5);
        assert_eq!(Config::for_profile(Profile::Public).max_words.get(), 100);
        assert_eq!(Config::for_profile(Profile::Falc).max_sentences.get(), 3);
        assert_eq!(Config::for_profile(Profile::Falc).max_words.get(), 60);
    }

    #[test]
    fn diagnostic_has_location_at_paragraph_start() {
        let text = "Lead.\n\nOne. Two. Three. Four. Five. Six.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].location.column, 1);
        assert!(diags[0].location.line >= 3);
    }

    #[test]
    fn category_is_structure() {
        let text = "One. Two. Three. Four. Five. Six.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Structure);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "One. Two. Three. Four. Five. Six. Seven.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
