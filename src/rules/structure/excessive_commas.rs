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
use crate::rules::enumeration::{enumeration_comma_count, parenthesised_list_comma_count};
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
    pub const ID: &'static str = "structure.excessive-commas";
}

impl Rule for ExcessiveCommas {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_commas.get();
        document
            .paragraphs_with_section()
            .flat_map(|(paragraph, section_title)| {
                let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
                sentences.into_iter().filter_map(move |sentence| {
                    let total =
                        u32::try_from(sentence.text.matches(',').count()).unwrap_or(u32::MAX);
                    // Discount commas that belong to a recognized inline
                    // enumeration: those are style questions for the
                    // `long-enumeration` rule, not subordination load.
                    // Parenthesised token lists `(A, B, C, …)` are
                    // discounted too — the Oxford detector misses them
                    // and they dominate doc-prose FPs (F22).
                    let enum_commas = enumeration_comma_count(&sentence.text, language);
                    let paren_commas = parenthesised_list_comma_count(&sentence.text);
                    let count = total
                        .saturating_sub(enum_commas)
                        .saturating_sub(paren_commas);
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
        assert_eq!(ExcessiveCommas::ID, "structure.excessive-commas");
    }

    #[test]
    fn oxford_enumeration_commas_are_discounted() {
        // 4 commas total, but the enumeration accounts for them all, so
        // the effective count is 0 — below threshold.
        let text = "Red, green, blue, yellow, and purple make the palette.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn sentence_with_enumeration_plus_extra_commas_still_triggers() {
        // The enumeration "red, green, and blue" accounts for 2 commas; the
        // remaining 4 commas come from subordinate clauses and push the
        // effective count above the Public threshold.
        let text = "Note, although we agreed, we packed red, green, and blue, carefully, and \
                    quietly.";
        let diags = lint(text, Profile::Public);
        assert!(!diags.is_empty());
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
    fn parenthesised_token_list_is_discounted() {
        // 6 raw commas, but 5 of them sit inside two parenthesised token
        // lists — the effective subordination count is 1, well under the
        // dev-doc threshold of 4.
        let text = "Numerals come in digit form (`1`, `2`, `3`) and spelled form \
                    (`one`, `two`, `three`, `four`), matching behavior.";
        assert!(lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn parenthesised_list_plus_subordination_still_triggers() {
        // 3 paren commas discounted, but 4 subordination commas remain,
        // pushing above the dev-doc threshold of 4.
        // 8 raw commas total: 3 inside the paren list (discounted), 5 in
        // surrounding subordinate clauses. Net 5 > DevDoc threshold 4.
        let text = "Although we listed the colours (red, green, blue, yellow) in the brief, \
                    the team decided, after much debate among stakeholders, to revise the palette, \
                    before shipping, despite the tight deadline.";
        let diags = lint(text, Profile::DevDoc);
        assert!(!diags.is_empty());
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
