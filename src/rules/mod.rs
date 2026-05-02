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

use std::collections::BTreeSet;

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

    /// Lifecycle status of the rule (F139).
    ///
    /// Defaults to [`Status::Stable`]: the rule is part of the default
    /// rule set and runs unconditionally. Rules under active iteration
    /// — typically the next-version cohort opening a dogfood window —
    /// override to [`Status::Experimental`]. Experimental rules ship in
    /// the registry but are filtered out of the active rule set unless
    /// the user opts in via `[experimental] enabled = [...]` in the
    /// config or `--experimental <id>` on the CLI.
    fn status(&self) -> Status {
        Status::Stable
    }
}

/// Lifecycle status of a rule (F139).
///
/// Models the same concept as clippy's `nursery`, biome's `nursery`,
/// `ESLint` experimental rules, and Rust's `#[unstable]`: a rule is
/// either part of the default contract ([`Self::Stable`]) or is shipped
/// for opt-in dogfooding while it stabilizes ([`Self::Experimental`]).
///
/// Promotion is a one-line change in the rule's `status()` impl plus a
/// `CHANGELOG` entry, by design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// Default-active. The rule is part of every run unless a user
    /// silences it via `[[ignore]]`, condition tags, or per-rule config.
    Stable,
    /// Off by default. The rule is registered but skipped at engine
    /// construction time unless the user opts in.
    Experimental,
}

/// User opt-in selector for [`Status::Experimental`] rules (F139).
///
/// Three shapes, mapping to the three TOML / CLI surfaces:
///
/// - [`Self::None`] — the default. No experimental rules run.
/// - [`Self::All`] — opt in to every experimental rule (`enabled = "*"`
///   in TOML, `--experimental '*'` on the CLI).
/// - [`Self::Ids`] — opt in to a specific set, by rule id.
#[derive(Debug, Clone, Default)]
pub enum ExperimentalOptIn {
    /// No experimental rules run.
    #[default]
    None,
    /// Every experimental rule runs.
    All,
    /// Only experimental rules whose id is in this set run.
    Ids(BTreeSet<String>),
}

impl ExperimentalOptIn {
    /// Build an [`ExperimentalOptIn`] from an iterable of selectors.
    ///
    /// A selector of `"*"` (anywhere in the iterator) collapses the
    /// result to [`Self::All`]. An empty iterator yields [`Self::None`].
    /// Otherwise the selectors are collected as rule ids.
    pub fn from_selectors<I, S>(selectors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut ids = BTreeSet::new();
        for sel in selectors {
            let s: String = sel.into();
            if s == "*" {
                return Self::All;
            }
            ids.insert(s);
        }
        if ids.is_empty() {
            Self::None
        } else {
            Self::Ids(ids)
        }
    }

    /// Whether `rule_id` is opted in under this selector.
    #[must_use]
    pub fn covers(&self, rule_id: &str) -> bool {
        match self {
            Self::None => false,
            Self::All => true,
            Self::Ids(ids) => ids.contains(rule_id),
        }
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

/// Return the ids of every [`Status::Experimental`] rule registered
/// in the default rule set (F139). Profile-independent: experimental
/// status is a property of the rule, not of the profile.
///
/// Used by the `explain --list-verbose` renderer to annotate the
/// experimental cohort and by tests that pin the size of the cohort.
#[must_use]
pub fn experimental_rule_ids() -> std::collections::BTreeSet<&'static str> {
    default_rules(Profile::DEFAULT)
        .iter()
        .filter(|r| r.status() == Status::Experimental)
        .map(|r| r.id())
        .collect()
}

/// Filter `rules` down to those active under the user's experimental
/// opt-in (F139).
///
/// [`Status::Stable`] rules are always retained. [`Status::Experimental`]
/// rules are retained only when [`ExperimentalOptIn::covers`] is true
/// for their id.
#[must_use]
pub fn filter_by_experimental(
    rules: Vec<Box<dyn Rule>>,
    opt_in: &ExperimentalOptIn,
) -> Vec<Box<dyn Rule>> {
    rules
        .into_iter()
        .filter(|r| match r.status() {
            Status::Stable => true,
            Status::Experimental => opt_in.covers(r.id()),
        })
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
    fn every_default_rule_is_stable() {
        // F139: until the v0.3 cohort lands on the experimental
        // substrate, no shipped rule is Experimental. Once the cohort
        // arrives, this test should be removed (it would be a guard
        // against a regression that no longer exists).
        for rule in default_rules(Profile::Public) {
            assert_eq!(
                rule.status(),
                Status::Stable,
                "rule `{}` is Experimental but no v0.2.x rule should be (yet)",
                rule.id()
            );
        }
    }

    #[test]
    fn experimental_opt_in_from_selectors_normalises_star() {
        let star = ExperimentalOptIn::from_selectors(["structure.foo", "*"]);
        assert!(matches!(star, ExperimentalOptIn::All));
    }

    #[test]
    fn experimental_opt_in_from_empty_iter_is_none() {
        let none = ExperimentalOptIn::from_selectors(Vec::<String>::new());
        assert!(matches!(none, ExperimentalOptIn::None));
    }

    #[test]
    fn experimental_opt_in_covers_matches_specific_ids() {
        let ids = ExperimentalOptIn::from_selectors([
            "structure.italic-span-long",
            "structure.number-run",
        ]);
        assert!(ids.covers("structure.italic-span-long"));
        assert!(!ids.covers("structure.sentence-too-long"));
    }

    #[test]
    fn filter_by_experimental_keeps_all_stable_when_off() {
        // No fake experimental rules exist yet — the v0.2 default set
        // is all Stable, so the filter is a no-op.
        let kept = filter_by_experimental(default_rules(Profile::Public), &ExperimentalOptIn::None);
        assert_eq!(kept.len(), 25);
    }

    /// Bench substrate against a synthetic experimental rule, since no
    /// real rule is `Status::Experimental` until the v0.3 cohort lands.
    /// Once F49 ships on this substrate, the corpus snapshot for that
    /// rule replaces this fixture as the end-to-end proof.
    struct FakeExperimental;

    impl Rule for FakeExperimental {
        fn id(&self) -> &'static str {
            "structure.fake-experimental"
        }

        fn check(&self, _document: &Document, _language: Language) -> Vec<Diagnostic> {
            Vec::new()
        }

        fn status(&self) -> Status {
            Status::Experimental
        }
    }

    fn registry_with_fake_experimental() -> Vec<Box<dyn Rule>> {
        let mut rules = default_rules(Profile::Public);
        rules.push(Box::new(FakeExperimental));
        rules
    }

    #[test]
    fn filter_by_experimental_strips_experimental_when_off() {
        let kept =
            filter_by_experimental(registry_with_fake_experimental(), &ExperimentalOptIn::None);
        assert_eq!(
            kept.len(),
            25,
            "experimental rule must be filtered out by default"
        );
        assert!(kept.iter().all(|r| r.id() != "structure.fake-experimental"));
    }

    #[test]
    fn filter_by_experimental_keeps_experimental_under_wildcard() {
        let kept =
            filter_by_experimental(registry_with_fake_experimental(), &ExperimentalOptIn::All);
        assert_eq!(kept.len(), 26);
        assert!(kept.iter().any(|r| r.id() == "structure.fake-experimental"));
    }

    #[test]
    fn filter_by_experimental_keeps_only_opted_in_ids() {
        let opt_in = ExperimentalOptIn::from_selectors(["structure.fake-experimental"]);
        let kept = filter_by_experimental(registry_with_fake_experimental(), &opt_in);
        assert_eq!(kept.len(), 26);
        assert!(kept.iter().any(|r| r.id() == "structure.fake-experimental"));

        // A different id in the opt-in set leaves the experimental rule
        // filtered out.
        let other = ExperimentalOptIn::from_selectors(["structure.does-not-exist"]);
        let kept = filter_by_experimental(registry_with_fake_experimental(), &other);
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
