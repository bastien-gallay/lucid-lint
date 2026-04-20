//! Rule: `conditional-stacking`.
//!
//! Flags sentences that chain multiple conditional clauses. Each `if` /
//! `when` / `unless` / `quand` / `si` opens a branch the reader must keep
//! on a mental stack until the outer clause resolves; two or three of them
//! stacked in one sentence is a known load multiplier for readers with
//! aphasia, ADHD, and (more broadly) anyone reading under cognitive load.
//! Plain-language guidelines (FALC, plainlanguage.gov) recommend splitting
//! conditional chains into separate sentences or a bullet list.
//!
//! Detection is per-sentence, language-aware:
//!
//! - **English** — sum of word-bounded matches against
//!   [`crate::language::en::CONDITIONALS`].
//! - **French** — sum of word-bounded matches against
//!   [`crate::language::fr::CONDITIONALS`] plus
//!   [`crate::language::fr::SI_CLITICS`] (`s'il`, `s'ils`).
//!
//! See [`RULES.md`](../../RULES.md#conditional-stacking) for the threshold
//! reference.

use std::num::NonZeroU32;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::phrase_search::count_word_bounded;
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`ConditionalStacking`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed conditional connectors per sentence.
    pub max_conditionals: NonZeroU32,
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
            max_conditionals: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`ConditionalStacking`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ConditionalStacking {
    config: Config,
}

impl ConditionalStacking {
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
    pub const ID: &'static str = "conditional-stacking";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[
        ConditionTag::Aphasia,
        ConditionTag::Adhd,
        ConditionTag::General,
    ];
}

impl Rule for ConditionalStacking {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_conditionals.get();
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
    sum_matches(&lowered, en::CONDITIONALS)
}

fn count_french(sentence: &str) -> u32 {
    let lowered = sentence.to_lowercase();
    sum_matches(&lowered, fr::CONDITIONALS).saturating_add(sum_matches(&lowered, fr::SI_CLITICS))
}

fn sum_matches(haystack_lower: &str, needles: &[&str]) -> u32 {
    let mut total: u32 = 0;
    for needle in needles {
        let n = count_word_bounded(haystack_lower, needle);
        total = total.saturating_add(u32::try_from(n).unwrap_or(u32::MAX));
    }
    total
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
        "Sentence stacks {actual} conditional clauses (maximum {max}). Split the conditions \
         across separate sentences or convert them to a bullet list."
    );
    let diag = Diagnostic::new(
        ConditionalStacking::ID,
        Severity::Warning,
        location,
        message,
    );
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
        ConditionalStacking::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(ConditionalStacking::ID, "conditional-stacking");
    }

    #[test]
    fn tags_include_general_so_rule_runs_by_default() {
        assert!(ConditionalStacking::TAGS.contains(&ConditionTag::General));
        assert!(ConditionalStacking::TAGS.contains(&ConditionTag::Aphasia));
        assert!(ConditionalStacking::TAGS.contains(&ConditionTag::Adhd));
    }

    #[test]
    fn category_is_syntax() {
        let text = "If we ship, when the build passes, unless the gate fails, we deploy.";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].category(), Category::Syntax);
    }

    #[test]
    fn english_single_conditional_does_not_trigger() {
        assert!(lint("If you agree, we ship.", Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn english_two_conditionals_at_public_threshold() {
        // Public threshold = 2; exactly 2 = not flagged.
        let text = "If we ship and when the build passes, we deploy.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn english_three_conditionals_trigger_under_public() {
        let text = "If we ship, when the build passes, unless the gate fails, we deploy.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 conditional"));
    }

    #[test]
    fn english_multiword_conditional_is_counted() {
        // "as long as" + "if" = 2 → at Public threshold, not flagged. Add
        // one more → flagged.
        let text = "As long as the test passes and if the lint is clean and when CI is green, \
                    we publish.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn falc_profile_flags_two_conditionals() {
        let text = "If we ship and when the build passes, we deploy.";
        let diags = lint(text, Profile::Falc, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn dev_doc_profile_tolerates_three() {
        // 3 conditionals: triggers Public (max 2), passes DevDoc (max 3).
        let text = "If we ship, when the build passes, unless the gate fails, we deploy.";
        assert!(!lint(text, Profile::Public, Language::En).is_empty());
        assert!(lint(text, Profile::DevDoc, Language::En).is_empty());
    }

    #[test]
    fn french_single_si_does_not_trigger() {
        assert!(lint(
            "Si vous êtes prêt, nous publions.",
            Profile::Public,
            Language::Fr
        )
        .is_empty());
    }

    #[test]
    fn french_three_conditionals_trigger() {
        let text = "Si nous expédions, quand le test passe, à moins que la barrière échoue, \
                    nous déployons.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 conditional"));
    }

    #[test]
    fn french_si_clitic_is_counted() {
        // s'il + quand + lorsque = 3, triggers Public.
        let text = "S'il accepte, quand vous êtes prêt, lorsque la revue est terminée, \
                    nous fusionnons.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(
            Config::for_profile(Profile::DevDoc).max_conditionals.get(),
            3
        );
        assert_eq!(
            Config::for_profile(Profile::Public).max_conditionals.get(),
            2
        );
        assert_eq!(Config::for_profile(Profile::Falc).max_conditionals.get(), 1);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Short and clean. If we ship, when the build passes, unless the gate fails, \
                    we deploy. Fine again.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
