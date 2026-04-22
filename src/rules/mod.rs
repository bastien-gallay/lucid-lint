//! Lint rules.
//!
//! Each rule implements the [`Rule`] trait in its own file under the
//! category subdirectory it belongs to. Rule IDs follow the pattern
//! `category.rule-name` (for example `structure.excessive-commas`,
//! `lexicon.weasel-words`, `readability.score`) so that the ID and the
//! module path line up one-to-one: `structure::excessive_commas` exposes
//! `"structure.excessive-commas"`. The reference implementation to
//! pattern-match against when adding a new rule is
//! [`structure::sentence_too_long`] — it is intentionally minimal,
//! well-tested, and demonstrates the canonical structure.

use crate::condition::{rule_enabled, ConditionTag};
use crate::config::Profile;
use crate::parser::Document;
use crate::types::{Diagnostic, Language};

pub mod enumeration;
pub mod lexicon;
pub mod readability;
pub mod rhythm;
pub mod structure;
pub mod syntax;

pub use lexicon::all_caps_shouting::AllCapsShouting;
pub use lexicon::consonant_cluster::ConsonantCluster;
pub use lexicon::excessive_nominalization::ExcessiveNominalization;
pub use lexicon::jargon_undefined::JargonUndefined;
pub use lexicon::low_lexical_diversity::LowLexicalDiversity;
pub use lexicon::redundant_intensifier::RedundantIntensifier;
pub use lexicon::unexplained_abbreviation::UnexplainedAbbreviation;
pub use lexicon::weasel_words::WeaselWords;
pub use readability::score::ReadabilityScore;
pub use rhythm::consecutive_long_sentences::ConsecutiveLongSentences;
pub use rhythm::repetitive_connectors::RepetitiveConnectors;
pub use structure::deep_subordination::DeepSubordination;
pub use structure::deeply_nested_lists::DeeplyNestedLists;
pub use structure::excessive_commas::ExcessiveCommas;
pub use structure::heading_jump::HeadingJump;
pub use structure::line_length_wide::LineLengthWide;
pub use structure::long_enumeration::LongEnumeration;
pub use structure::mixed_numeric_format::MixedNumericFormat;
pub use structure::paragraph_too_long::ParagraphTooLong;
pub use structure::sentence_too_long::SentenceTooLong;
pub use syntax::conditional_stacking::ConditionalStacking;
pub use syntax::dense_punctuation_burst::DensePunctuationBurst;
pub use syntax::nested_negation::NestedNegation;
pub use syntax::passive_voice::PassiveVoice;
pub use syntax::unclear_antecedent::UnclearAntecedent;

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
    /// The rule id in `category.rule-name` form (e.g.
    /// `"structure.sentence-too-long"`).
    ///
    /// The category prefix must match the subdirectory the rule lives
    /// in, and the rule-name part must match the filename.
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
        Box::new(LineLengthWide::for_profile(profile)),
        Box::new(MixedNumericFormat::for_profile(profile)),
        Box::new(RedundantIntensifier::for_profile(profile)),
        Box::new(DensePunctuationBurst::for_profile(profile)),
        Box::new(ConsonantCluster::for_profile(profile)),
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
        assert_eq!(kept.len(), 25);
    }

    #[test]
    fn each_rule_has_a_well_formed_id() {
        for rule in default_rules(Profile::Public) {
            let id = rule.id();
            assert!(!id.is_empty(), "empty rule id");
            assert!(
                id.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '-' || c == '.'),
                "rule id `{id}` contains unexpected characters (only lowercase, `-`, `.` allowed)"
            );
            let parts: Vec<&str> = id.split('.').collect();
            assert_eq!(
                parts.len(),
                2,
                "rule id `{id}` must be `category.rule-name`"
            );
            assert!(
                !parts[0].is_empty() && !parts[1].is_empty(),
                "rule id `{id}` has an empty category or name half"
            );
        }
    }
}
