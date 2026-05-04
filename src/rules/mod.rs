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
pub use lexicon::homophone_density::HomophoneDensity;
pub use lexicon::jargon_undefined::JargonUndefined;
pub use lexicon::low_lexical_diversity::LowLexicalDiversity;
pub use lexicon::redundant_intensifier::RedundantIntensifier;
pub use lexicon::unexplained_abbreviation::UnexplainedAbbreviation;
pub use lexicon::weasel_words::WeaselWords;
pub use readability::large_number_unanchored::LargeNumberUnanchored;
pub use readability::score::ReadabilityScore;
pub use rhythm::consecutive_long_sentences::ConsecutiveLongSentences;
pub use rhythm::repetitive_connectors::RepetitiveConnectors;
pub use structure::deep_subordination::DeepSubordination;
pub use structure::deeply_nested_lists::DeeplyNestedLists;
pub use structure::excessive_commas::ExcessiveCommas;
pub use structure::heading_jump::HeadingJump;
pub use structure::italic_span_long::ItalicSpanLong;
pub use structure::line_length_wide::LineLengthWide;
pub use structure::long_enumeration::LongEnumeration;
pub use structure::mixed_numeric_format::MixedNumericFormat;
pub use structure::number_run::NumberRun;
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
        // F143-substrate cohort lead. Ships as Status::Experimental
        // in v0.2.x via F139; flips to Stable at v0.3 cut.
        Box::new(ItalicSpanLong::for_profile(profile)),
        // F46 — cohort sibling of F49. Ships as Status::Experimental
        // in v0.2.x via F139; flips to Stable at v0.3 cut.
        Box::new(HomophoneDensity::for_profile(profile)),
        // F51 — cohort sibling of F49. Ships as Status::Experimental
        // in v0.2.x via F139; flips to Stable at v0.3 cut.
        Box::new(NumberRun::for_profile(profile)),
        // F53 — cohort sibling of F49. Ships as Status::Experimental
        // in v0.2.x via F139; flips to Stable at v0.3 cut.
        Box::new(LargeNumberUnanchored::for_profile(profile)),
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
    fn every_stable_default_rule_is_tagged_general() {
        // v0.2 baseline: every Stable rule shipped with the General
        // condition tag (always-active under default conditions).
        // Experimental rules (F139 cohort, starting with F49) are
        // condition-tag-targeted and intentionally exempt — they ship
        // off-by-default via the experimental opt-in and gate further
        // on the user's active conditions.
        for rule in default_rules(Profile::Public) {
            if rule.status() == Status::Experimental {
                continue;
            }
            assert!(
                rule.condition_tags().contains(&ConditionTag::General),
                "Stable rule `{}` is missing the `general` condition tag (v0.2 baseline)",
                rule.id()
            );
        }
    }

    #[test]
    fn filter_by_conditions_keeps_general_rules() {
        // 25 Stable General-tagged rules plus F53
        // (`readability.large-number-unanchored`), which is
        // Experimental but carries `general` alongside `dyscalculia`
        // because its grounding (CDC CCI / plainlanguage.gov "Use
        // Numbers Effectively") applies to every reader, not only
        // dyscalculic ones. F53 still ships off-by-default via the
        // experimental opt-in; the `general` tag only matters once
        // the user has opted into the rule.
        let kept = filter_by_conditions(default_rules(Profile::Public), &[]);
        assert_eq!(kept.len(), 26);
    }

    #[test]
    fn experimental_cohort_is_tracked() {
        // Replaces the pre-cohort `every_default_rule_is_stable`
        // guard. F49 (`structure.italic-span-long`) is the cohort
        // lead; F46 / F51 / F53 / F57 follow on the same substrate.
        // At v0.3 cut, every entry here flips to Stable and this
        // test loosens (or is removed). Until then, this guards
        // against accidentally graduating one of them too early.
        let experimental: std::collections::BTreeSet<&str> =
            experimental_rule_ids().iter().copied().collect();
        let expected = [
            "lexicon.homophone-density",
            "readability.large-number-unanchored",
            "structure.italic-span-long",
            "structure.number-run",
        ];
        for id in &expected {
            assert!(
                experimental.contains(id),
                "expected `{id}` to ship as Experimental in v0.2.x; got {experimental:?}"
            );
        }
        // Catches accidentally graduating an experimental rule
        // before the v0.3 cut without updating this list.
        assert_eq!(
            experimental.len(),
            expected.len(),
            "experimental cohort drifted; got {experimental:?}, expected {expected:?}"
        );
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
        // With F49 (cohort lead) shipping as Experimental, the
        // default registry has 26 rules; the no-opt-in filter strips
        // F49 → 25 Stable rules remain.
        let kept = filter_by_experimental(default_rules(Profile::Public), &ExperimentalOptIn::None);
        assert_eq!(kept.len(), 25);
    }

    /// Synthetic experimental rule retained alongside F49 to give the
    /// substrate's filter tests a *known-isolated* fixture: F49 lives
    /// in `default_rules`, so a "wildcard keeps all" assertion that
    /// only references the real registry would conflate F49 + future
    /// cohort siblings with the substrate semantics. `FakeExperimental`
    /// lets the substrate be tested in isolation from cohort drift.
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
        // Registry holds the 29 default rules (25 Stable + F49 + F46
        // + F51 + F53 Experimental) plus FakeExperimental = 30 total.
        // With the opt-in off, all five Experimental rules are
        // stripped → 25.
        let kept =
            filter_by_experimental(registry_with_fake_experimental(), &ExperimentalOptIn::None);
        assert_eq!(
            kept.len(),
            25,
            "experimental rules must be filtered out by default"
        );
        assert!(kept.iter().all(|r| r.id() != "structure.fake-experimental"));
        assert!(kept.iter().all(|r| r.id() != "structure.italic-span-long"));
        assert!(kept.iter().all(|r| r.id() != "lexicon.homophone-density"));
    }

    #[test]
    fn filter_by_experimental_keeps_experimental_under_wildcard() {
        // Wildcard keeps all 30 rules in the registry: 25 Stable +
        // F49 + F46 + F51 + F53 (real Experimental) + FakeExperimental.
        let kept =
            filter_by_experimental(registry_with_fake_experimental(), &ExperimentalOptIn::All);
        assert_eq!(kept.len(), 30);
        assert!(kept.iter().any(|r| r.id() == "structure.fake-experimental"));
        assert!(kept.iter().any(|r| r.id() == "structure.italic-span-long"));
        assert!(kept.iter().any(|r| r.id() == "lexicon.homophone-density"));
    }

    #[test]
    fn filter_by_experimental_keeps_only_opted_in_ids() {
        // Opt-in includes the synthetic id only — F49 and F46 stay
        // filtered out because neither is on the list. Result: 25
        // Stable + FakeExperimental = 26.
        let opt_in = ExperimentalOptIn::from_selectors(["structure.fake-experimental"]);
        let kept = filter_by_experimental(registry_with_fake_experimental(), &opt_in);
        assert_eq!(kept.len(), 26);
        assert!(kept.iter().any(|r| r.id() == "structure.fake-experimental"));
        assert!(kept.iter().all(|r| r.id() != "structure.italic-span-long"));
        assert!(kept.iter().all(|r| r.id() != "lexicon.homophone-density"));

        // A different id in the opt-in set leaves all experimental
        // rules filtered out.
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
