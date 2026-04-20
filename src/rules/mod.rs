//! Lint rules.
//!
//! Each rule implements the [`Rule`] trait in its own file under this module.
//! The reference implementation to pattern-match against when adding a new rule
//! is [`sentence_too_long`] — it is intentionally minimal, well-tested, and
//! demonstrates the canonical structure.

use crate::condition::{rule_enabled, ConditionTag};
use crate::config::Profile;
use crate::parser::Document;
use crate::types::{Diagnostic, Language};

pub mod all_caps_shouting;
pub mod conditional_stacking;
pub mod consecutive_long_sentences;
pub mod deep_subordination;
pub mod deeply_nested_lists;
pub mod enumeration;
pub mod excessive_commas;
pub mod excessive_nominalization;
pub mod heading_jump;
pub mod jargon_undefined;
pub mod long_enumeration;
pub mod low_lexical_diversity;
pub mod nested_negation;
pub mod paragraph_too_long;
pub mod passive_voice;

pub mod readability_score;
pub mod repetitive_connectors;
pub mod sentence_too_long;
pub mod unclear_antecedent;
pub mod unexplained_abbreviation;
pub mod weasel_words;

pub use all_caps_shouting::AllCapsShouting;
pub use conditional_stacking::ConditionalStacking;
pub use consecutive_long_sentences::ConsecutiveLongSentences;
pub use deep_subordination::DeepSubordination;
pub use deeply_nested_lists::DeeplyNestedLists;
pub use excessive_commas::ExcessiveCommas;
pub use excessive_nominalization::ExcessiveNominalization;
pub use heading_jump::HeadingJump;
pub use jargon_undefined::JargonUndefined;
pub use long_enumeration::LongEnumeration;
pub use low_lexical_diversity::LowLexicalDiversity;
pub use nested_negation::NestedNegation;
pub use paragraph_too_long::ParagraphTooLong;
pub use passive_voice::PassiveVoice;
pub use readability_score::ReadabilityScore;
pub use repetitive_connectors::RepetitiveConnectors;
pub use sentence_too_long::SentenceTooLong;
pub use unclear_antecedent::UnclearAntecedent;
pub use unexplained_abbreviation::UnexplainedAbbreviation;
pub use weasel_words::WeaselWords;

/// Common interface for a lint rule.
///
/// A rule receives a parsed [`Document`], plus context about the target
/// profile and detected language, and returns a list of [`Diagnostic`]s.
///
/// Rules MUST be deterministic: identical input must yield identical output.
///
/// # Design notes
///
/// - A rule should be constructible with a single `new(profile)` call.
/// - Configuration is resolved at construction time from the [`Profile`].
/// - Per-document state (e.g., running counters) lives in local variables
///   inside [`Rule::check`], not in the rule struct.
pub trait Rule {
    /// The rule id in kebab-case (e.g., `"sentence-too-long"`).
    ///
    /// Must match the filename of the rule module.
    fn id(&self) -> &'static str;

    /// Analyze a document and return any diagnostics.
    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic>;

    /// Condition tags this rule targets (F71).
    ///
    /// Defaults to `&[ConditionTag::General]`, meaning the rule is always
    /// active. Rules targeting a specific cognitive condition override this
    /// to declare the relevant tags. The engine uses
    /// [`crate::condition::rule_enabled`] to decide whether the rule runs
    /// for a given user `conditions = [...]` config (F72).
    fn condition_tags(&self) -> &'static [ConditionTag] {
        &[ConditionTag::General]
    }
}

/// Filter `rules` down to those enabled by the user's active conditions.
///
/// See [`rule_enabled`] for the per-rule semantics. Rules tagged
/// [`ConditionTag::General`] are always retained; rules without `General`
/// are retained only if any of their tags appears in `active`.
#[must_use]
pub fn filter_by_conditions(
    rules: Vec<Box<dyn Rule>>,
    active: &[ConditionTag],
) -> Vec<Box<dyn Rule>> {
    rules
        .into_iter()
        .filter(|r| rule_enabled(r.condition_tags(), active))
        .collect()
}

/// Build the default set of rules for a given profile.
///
/// Rules are added incrementally following the pattern established by
/// [`SentenceTooLong`].
#[must_use]
pub fn default_rules(profile: Profile) -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(SentenceTooLong::for_profile(profile)),
        Box::new(ParagraphTooLong::for_profile(profile)),
        Box::new(HeadingJump::for_profile(profile)),
        Box::new(DeeplyNestedLists::for_profile(profile)),
        Box::new(ExcessiveCommas::for_profile(profile)),
        Box::new(ConsecutiveLongSentences::for_profile(profile)),
        Box::new(WeaselWords::for_profile(profile)),
        Box::new(UnexplainedAbbreviation::for_profile(profile)),
        Box::new(JargonUndefined::for_profile(profile)),
        Box::new(ExcessiveNominalization::for_profile(profile)),
        Box::new(RepetitiveConnectors::for_profile(profile)),
        Box::new(ReadabilityScore::for_profile(profile)),
        Box::new(LongEnumeration::for_profile(profile)),
        Box::new(DeepSubordination::for_profile(profile)),
        Box::new(PassiveVoice::for_profile(profile)),
        Box::new(UnclearAntecedent::for_profile(profile)),
        Box::new(LowLexicalDiversity::for_profile(profile)),
        Box::new(NestedNegation::for_profile(profile)),
        Box::new(ConditionalStacking::for_profile(profile)),
        Box::new(AllCapsShouting::for_profile(profile)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rules_is_non_empty() {
        let rules = default_rules(Profile::Public);
        assert!(!rules.is_empty());
    }

    #[test]
    fn every_default_rule_is_tagged_general() {
        for rule in default_rules(Profile::Public) {
            assert!(
                rule.condition_tags().contains(&ConditionTag::General),
                "rule `{}` is missing the `general` condition tag (v0.2 baseline)",
                rule.id()
            );
        }
    }

    #[test]
    fn filter_by_conditions_keeps_general_rules() {
        let kept = filter_by_conditions(default_rules(Profile::Public), &[]);
        assert_eq!(kept.len(), 20);
    }

    #[test]
    fn each_rule_has_a_nonempty_id() {
        for rule in default_rules(Profile::Public) {
            assert!(!rule.id().is_empty());
            assert!(rule
                .id()
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-'));
        }
    }
}
