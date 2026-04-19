//! Rule: `excessive-commas`.
//!
//! Flags sentences with a high comma count. Commas are the most frequent
//! marker of syntactic complexity (subordination, apposition, enumeration,
//! inline parenthetical); density is a leading indicator of overload
//! regardless of cause.
//!
//! See [`RULES.md`](../../RULES.md#excessive-commas) for the rule's
//! rationale and thresholds.

use std::num::NonZeroU32;

use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`ExcessiveCommas`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed comma count per sentence.
    pub max_commas: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 4,
            Profile::Public => 3,
            Profile::Falc => 2,
        };
        Self {
            max_commas: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`ExcessiveCommas`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ExcessiveCommas {
    config: Config,
}

impl ExcessiveCommas {
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
    pub const ID: &'static str = "excessive-commas";
}

impl Rule for ExcessiveCommas {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_commas.get();
        document
            .paragraphs_with_section()
            .flat_map(|(paragraph, section_title)| {
                let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
                sentences.into_iter().filter_map(move |sentence| {
                    let count =
                        u32::try_from(sentence.text.matches(',').count()).unwrap_or(u32::MAX);
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
    source: &SourceFile,
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
        "Sentence has {actual} commas (maximum {max}). Consider splitting it into shorter \
         sentences or extracting an enumeration as a list."
    );
    let diag = Diagnostic::new(ExcessiveCommas::ID, Severity::Warning, location, message);
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
        ExcessiveCommas::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(ExcessiveCommas::ID, "excessive-commas");
    }

    #[test]
    fn no_commas_does_not_trigger() {
        assert!(lint("A simple sentence without commas.", Profile::Public).is_empty());
    }

    #[test]
    fn at_threshold_does_not_trigger() {
        // Public: max 3 → 3 commas is fine.
        let text = "First, second, third, tail.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn exceeding_threshold_triggers() {
        // Public: max 3 → 4 commas triggers.
        let text = "First, second, third, fourth, tail.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("4 commas"));
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 3 commas: passes Public (3), fails FALC (2).
        let text = "First, second, third, tail.";
        assert!(lint(text, Profile::Public).is_empty());
        assert!(!lint(text, Profile::Falc).is_empty());
    }

    #[test]
    fn each_sentence_is_checked_independently() {
        let text = "One, two. First, second, third, fourth, tail. Three, four.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn french_text_is_detected() {
        // 4 commas, threshold 3 under Public.
        let text = "D'abord, ensuite, puis, enfin, la conclusion.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_commas.get(), 4);
        assert_eq!(Config::for_profile(Profile::Public).max_commas.get(), 3);
        assert_eq!(Config::for_profile(Profile::Falc).max_commas.get(), 2);
    }

    #[test]
    fn category_is_structure() {
        let text = "First, second, third, fourth, tail.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Structure);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Short one. First, second, third, fourth, and finally the tail.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
