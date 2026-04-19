//! Rule: `sentence-too-long`.
//!
//! Flags sentences whose word count exceeds a configurable threshold.
//!
//! This is the canonical reference rule for v0.1. Use it as the template
//! when implementing the 15 other rules. It demonstrates:
//!
//! - The [`Rule`] trait implementation.
//! - Per-profile threshold configuration via [`Config`].
//! - Use of the shared parser and tokenizer.
//! - Emission of [`Diagnostic`]s with location and section context.
//! - Unit tests + snapshot tests.
//!
//! See [`RULES.md`](../../RULES.md#sentence-too-long) for the rule's rationale
//! and threshold reference.

use std::num::NonZeroU32;

use crate::config::Profile;
use crate::parser::{split_sentences, word_count, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity};

/// Configuration for [`SentenceTooLong`].
///
/// Thresholds are strongly typed with [`NonZeroU32`] to make impossible states
/// impossible: a zero threshold would flag every sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed word count per sentence.
    pub max_words: NonZeroU32,

    /// Whether to skip text inside fenced code blocks.
    pub exclude_code_blocks: bool,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max_words = match profile {
            Profile::DevDoc => 30,
            Profile::Public => 22,
            Profile::Falc => 15,
        };
        Self {
            // Safe: the literal values above are all non-zero.
            max_words: NonZeroU32::new(max_words).expect("non-zero literal"),
            exclude_code_blocks: true,
        }
    }
}

/// The [`SentenceTooLong`] rule.
#[derive(Debug, Clone, Copy)]
pub struct SentenceTooLong {
    config: Config,
}

impl SentenceTooLong {
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

    /// The rule identifier. Exposed as an associated constant for use in tests
    /// and external registries without instantiating the rule.
    pub const ID: &'static str = "sentence-too-long";
}

impl Rule for SentenceTooLong {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_words.get();

        document
            .paragraphs_with_section()
            .flat_map(|(paragraph, section_title)| {
                let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
                sentences.into_iter().filter_map(move |sentence| {
                    let count = word_count(&sentence.text);
                    if count > max {
                        Some(build_diagnostic(
                            &document.source,
                            &sentence.text,
                            sentence.line,
                            sentence.column,
                            count,
                            max,
                            section_title,
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

fn build_diagnostic(
    source: &crate::types::SourceFile,
    sentence_text: &str,
    line: u32,
    column: u32,
    actual: u32,
    max: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(sentence_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Sentence is {actual} words long (maximum {max}). Consider splitting it into shorter sentences."
    );
    let diag = Diagnostic::new(SentenceTooLong::ID, Severity::Warning, location, message);
    match section {
        Some(title) => diag.with_section(title),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_plain;
    use crate::types::{Language, SourceFile};

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        let rule = SentenceTooLong::for_profile(profile);
        rule.check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(SentenceTooLong::ID, "sentence-too-long");
        assert_eq!(
            SentenceTooLong::for_profile(Profile::Public).id(),
            "sentence-too-long"
        );
    }

    #[test]
    fn short_sentence_does_not_trigger() {
        let text = "This sentence is fine.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn long_sentence_triggers_warning() {
        // 25 words, threshold is 22 for Public.
        let text = "This is a rather long sentence that keeps adding more and more words \
                    until it exceeds the public profile threshold by a comfortable margin.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, "sentence-too-long");
        assert_eq!(diags[0].severity, Severity::Warning);
        assert!(diags[0].message.contains("words long"));
    }

    #[test]
    fn exactly_at_threshold_does_not_trigger() {
        // Rule triggers strictly above max_words.
        // For public: 22 is allowed, 23 is not.
        let text = "one two three four five six seven eight nine ten \
                    eleven twelve thirteen fourteen fifteen sixteen seventeen \
                    eighteen nineteen twenty twenty1 twenty2.";
        // That's 22 words.
        let diags = lint(text, Profile::Public);
        assert!(diags.is_empty(), "got diags: {diags:?}");
    }

    #[test]
    fn dev_doc_profile_is_more_tolerant() {
        // 25 words: triggers Public but not DevDoc.
        let text = "This is a rather long sentence that keeps adding more and more words \
                    until it exceeds the public profile threshold by a comfortable margin.";
        assert!(!lint(text, Profile::Public).is_empty());
        assert!(lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 17 words: does not trigger Public (22) but triggers FALC (15).
        let text = "This sentence contains exactly seventeen words so FALC flags it while \
                    Public lets it pass.";
        let falc = lint(text, Profile::Falc);
        let public = lint(text, Profile::Public);
        assert!(!falc.is_empty(), "FALC should have flagged: {falc:?}");
        assert!(
            public.is_empty(),
            "Public should not have flagged: {public:?}"
        );
    }

    #[test]
    fn multiple_sentences_each_checked() {
        let text = "Short one. This is a rather long sentence that keeps adding more and \
                    more words until it definitely exceeds the public profile threshold. \
                    Short again.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn category_is_derived_correctly() {
        let text = "This is a rather long sentence that keeps adding more and more words \
                    until it exceeds the public profile threshold by a comfortable margin.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Length);
    }

    #[test]
    fn diagnostic_has_location() {
        let text = "This is a rather long sentence that keeps adding more and more words \
                    until it exceeds the public profile threshold by a comfortable margin.";
        let diags = lint(text, Profile::Public);
        assert!(diags[0].location.line >= 1);
        assert!(diags[0].location.column >= 1);
        assert!(diags[0].location.length > 0);
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_words.get(), 30);
        assert_eq!(Config::for_profile(Profile::Public).max_words.get(), 22);
        assert_eq!(Config::for_profile(Profile::Falc).max_words.get(), 15);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Short. \
                    This is a rather long sentence that keeps adding more and more words \
                    until it exceeds the public profile threshold by a comfortable margin. \
                    Fine one.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
