//! Rule: `nested-negation`.
//!
//! Flags sentences that stack multiple negations. Two or more negations in
//! the same sentence force the reader to mentally toggle truth values, a
//! known burden for readers with aphasia, ADHD, and (more broadly) anyone
//! reading under cognitive load. Plain-language guidelines (FALC, CDC
//! Clear Communication Index, plainlanguage.gov) recommend rewriting
//! double negatives as positives.
//!
//! Counting strategy:
//!
//! - **English** — sum of word-boundary matches against the language's
//!   [`NEGATIONS`] list plus occurrences of the contracted `n't` suffix
//!   (`don't`, `won't`, `isn't`, `doesn't`, …).
//! - **French** — bipartite negation: each `ne` / `n'` clitic counts as
//!   one negation, plus standalone negators (`sans`, `non`). Counting the
//!   second-position particle (`pas`, `jamais`, `plus`, …) directly would
//!   trigger false positives because many of those forms are ambiguous
//!   outside the `ne ... X` construction.
//!
//! See [`RULES.md`](../../RULES.md#nested-negation) for the threshold
//! reference.

use std::num::NonZeroU32;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`NestedNegation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed negations per sentence.
    pub max_negations: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 3,
            Profile::Public => 2,
            Profile::Falc => 1,
        };
        Self {
            max_negations: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`NestedNegation`] rule.
#[derive(Debug, Clone, Copy)]
pub struct NestedNegation {
    config: Config,
}

impl NestedNegation {
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
    pub const ID: &'static str = "nested-negation";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[
        ConditionTag::Aphasia,
        ConditionTag::Adhd,
        ConditionTag::General,
    ];
}

impl Rule for NestedNegation {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_negations.get();
        let counter = match language {
            Language::Fr => count_french,
            Language::En | Language::Unknown => count_english,
        };

        document
            .paragraphs_with_section()
            .flat_map(|(paragraph, section_title)| {
                let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
                sentences.into_iter().filter_map(move |sentence| {
                    let count = counter(&sentence.text);
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

fn count_english(sentence: &str) -> u32 {
    let lowered = sentence.to_lowercase();
    let mut count: u32 = 0;
    for token in lowered.split(|c: char| !c.is_alphanumeric() && c != '\'') {
        if token.is_empty() {
            continue;
        }
        if en::NEGATIONS.contains(&token) {
            count = count.saturating_add(1);
            continue;
        }
        // Contracted forms: don't, can't, won't, isn't, doesn't, …
        // Bare "n't" doesn't occur as a token in well-formed text, so
        // require at least one preceding letter.
        if token.len() > 2 && token.ends_with("n't") {
            count = count.saturating_add(1);
        }
    }
    count
}

fn count_french(sentence: &str) -> u32 {
    let lowered = sentence.to_lowercase();
    let mut count: u32 = 0;
    // Word-boundary tokens (apostrophes attach to the preceding clitic, so
    // strip them when checking the standalone negator list, but keep the
    // raw token for the clitic check).
    for token in lowered.split(|c: char| {
        c.is_whitespace() || matches!(c, ',' | ';' | ':' | '.' | '!' | '?' | '(' | ')' | '"')
    }) {
        if token.is_empty() {
            continue;
        }
        // `n'` clitic appears as `n'` glued to the next word: detect by
        // suffix on the token preceding the apostrophe split.
        if token == "ne" || token == "n'" || token.starts_with("n\u{2019}") {
            count = count.saturating_add(1);
            continue;
        }
        // `n'autre`, `n'est`, etc.: the token starts with `n'` because we
        // didn't split on apostrophe.
        if token.starts_with("n'") && token.len() > 2 {
            count = count.saturating_add(1);
            continue;
        }
        let bare = token.trim_matches(|c: char| !c.is_alphanumeric());
        if fr::STANDALONE_NEGATIONS.contains(&bare) {
            count = count.saturating_add(1);
        }
    }
    count
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
        "Sentence stacks {actual} negations (maximum {max}). Rewrite as a positive statement \
         or split the negations across separate sentences."
    );
    let diag = Diagnostic::new(NestedNegation::ID, Severity::Warning, location, message);
    match section {
        Some(title) => diag.with_section(title),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_plain;
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        NestedNegation::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(NestedNegation::ID, "nested-negation");
    }

    #[test]
    fn tags_include_general_so_rule_runs_by_default() {
        assert!(NestedNegation::TAGS.contains(&ConditionTag::General));
        assert!(NestedNegation::TAGS.contains(&ConditionTag::Aphasia));
        assert!(NestedNegation::TAGS.contains(&ConditionTag::Adhd));
    }

    #[test]
    fn category_is_syntax() {
        // Two negations under Public threshold (2) — bump to three.
        let text = "We do not say nothing is never possible.";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].category(), Category::Syntax);
    }

    #[test]
    fn english_single_negation_does_not_trigger() {
        assert!(lint("This is not allowed.", Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn english_double_negation_does_not_trigger_under_public() {
        // Public threshold is 2; two negations = at threshold = no diagnostic.
        let text = "We do not allow nothing here.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn english_triple_negation_triggers_under_public() {
        let text = "We do not say nothing is never possible.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 negations"));
    }

    #[test]
    fn english_contracted_nt_counts_as_negation() {
        // "don't" (1) + "isn't" (1) + "never" (1) = 3 → triggers Public.
        let text = "I don't think this isn't never going to happen.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn falc_profile_flags_a_single_double_negation() {
        // FALC max is 1; two negations triggers.
        let text = "We do not allow nothing.";
        let diags = lint(text, Profile::Falc, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn dev_doc_profile_tolerates_more_stacking() {
        // 3 negations: triggers Public (max 2), passes DevDoc (max 3).
        let text = "We do not say nothing is never wrong.";
        assert!(!lint(text, Profile::Public, Language::En).is_empty());
        assert!(lint(text, Profile::DevDoc, Language::En).is_empty());
    }

    #[test]
    fn french_bipartite_negation_counts_as_one() {
        // `ne ... pas` is ONE negation, not two.
        let text = "Nous ne sommes pas prêts.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_clitic_apostrophe_counts() {
        // `n'est ... jamais` — `n'` is the clitic, counts as one.
        let text = "Il n'est jamais venu.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_three_clitics_trigger_under_public() {
        // Three independent ne ... X clauses; "ne" appears 3 times.
        let text = "Il ne dit rien, elle ne fait rien et nous ne savons pas.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 negations"));
    }

    #[test]
    fn french_sans_counts_as_standalone_negation() {
        // `sans X sans Y` = 2 standalone negations → at Public threshold (2),
        // does not trigger; FALC (max 1) does.
        let text = "Sans plan, sans budget.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
        assert_eq!(lint(text, Profile::Falc, Language::Fr).len(), 1);
    }

    #[test]
    fn french_plus_alone_is_not_counted() {
        // `plus` outside `ne ... plus` means "more" — must not be flagged.
        let text = "Il faut plus de plus de plus de courage.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_negations.get(), 3);
        assert_eq!(Config::for_profile(Profile::Public).max_negations.get(), 2);
        assert_eq!(Config::for_profile(Profile::Falc).max_negations.get(), 1);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Short and clean. We do not say nothing is never possible. Fine again.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
