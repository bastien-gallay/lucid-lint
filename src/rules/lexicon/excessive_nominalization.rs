//! Rule: `excessive-nominalization`.
//!
//! Flags sentences whose nominalization density exceeds a threshold.
//! Nominalization (turning an action verb into an abstract noun)
//! raises processing cost and hides the agent of the action. A sentence
//! with four or more "-tion / -ment / -ance / …" words has almost
//! certainly lost its actors.
//!
//! Detection is pure suffix matching: cheap, deterministic, and
//! language-aware only to the extent of picking the right suffix list.
//!
//! See [`RULES.md`](../../RULES.md#excessive-nominalization) for the
//! rule's rationale, heavy/light example, and thresholds.

use std::num::NonZeroU32;

use unicode_segmentation::UnicodeSegmentation;

use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Minimum length (chars) before a word is considered for suffix matching.
/// Below this, suffix hits are almost always noise (`al`, `age`, `ness`
/// as standalone words).
const MIN_WORD_LEN: usize = 5;

/// English nominalization suffixes (lowercase).
const EN_SUFFIXES: &[&str] = &[
    "tion", "sion", "ment", "ance", "ence", "ity", "ism", "ness", "al",
];

/// French nominalization suffixes (lowercase).
const FR_SUFFIXES: &[&str] = &[
    "tion", "sion", "ment", "ance", "ence", "age", "ité", "isme", "ure",
];

/// Configuration for [`ExcessiveNominalization`].
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum nominalizations allowed per sentence. A sentence strictly
    /// exceeding this triggers.
    pub max_per_sentence: NonZeroU32,

    /// Override the language default suffix list (lowercase, no leading `-`).
    /// An empty vector means "use the language default".
    pub custom_suffixes: Vec<String>,
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
            max_per_sentence: NonZeroU32::new(max).expect("non-zero literal"),
            custom_suffixes: Vec::new(),
        }
    }
}

/// The [`ExcessiveNominalization`] rule.
#[derive(Debug, Clone)]
pub struct ExcessiveNominalization {
    config: Config,
}

impl ExcessiveNominalization {
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
    pub const ID: &'static str = "lexicon.excessive-nominalization";
}

impl Rule for ExcessiveNominalization {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_per_sentence.get();
        let suffixes = self.suffixes_for(language);
        if suffixes.is_empty() {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            for sentence in split_sentences(&paragraph.text, paragraph.start_line, 1) {
                let count = count_nominalizations(&sentence.text, &suffixes);
                if count > max {
                    diagnostics.push(build_diagnostic(
                        &document.source,
                        sentence.line,
                        sentence.column,
                        &sentence.text,
                        count,
                        max,
                        section_title,
                    ));
                }
            }
        }
        diagnostics
    }
}

impl ExcessiveNominalization {
    fn suffixes_for(&self, language: Language) -> Vec<String> {
        if !self.config.custom_suffixes.is_empty() {
            return self.config.custom_suffixes.clone();
        }
        match language {
            Language::En => EN_SUFFIXES.iter().map(|s| (*s).to_string()).collect(),
            Language::Fr => FR_SUFFIXES.iter().map(|s| (*s).to_string()).collect(),
            Language::Unknown => Vec::new(),
        }
    }
}

fn count_nominalizations(sentence: &str, suffixes: &[String]) -> u32 {
    let lowered = sentence.to_lowercase();
    let mut count: u32 = 0;
    for word in lowered.unicode_words() {
        let char_len = word.chars().count();
        if char_len < MIN_WORD_LEN {
            continue;
        }
        if suffixes
            .iter()
            .any(|suffix| char_len > suffix.chars().count() && word.ends_with(suffix.as_str()))
        {
            count = count.saturating_add(1);
        }
    }
    count
}

#[allow(clippy::too_many_arguments)]
fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    sentence_text: &str,
    count: u32,
    max: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(sentence_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Sentence has {count} nominalizations (max {max}). Consider rewriting with action verbs \
         and explicit agents."
    );
    let diag = Diagnostic::new(
        ExcessiveNominalization::ID,
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
    use crate::types::SourceFile;

    fn lint(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        ExcessiveNominalization::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(
            ExcessiveNominalization::ID,
            "lexicon.excessive-nominalization"
        );
    }

    #[test]
    fn action_verbs_do_not_trigger() {
        let text = "We ship code. We write tests. We deploy often.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn dense_nominalization_triggers() {
        // Public: max 3 → 4+ nominalizations trigger.
        let text = "The implementation of the configuration requires completion of the \
                    serialization and deserialization steps.";
        // implementation, configuration, completion, serialization, deserialization = 5
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("5 nominalizations"));
    }

    #[test]
    fn dev_doc_profile_tolerates_more() {
        // 4 nominalizations: triggers Public (max 3) but not DevDoc (max 4).
        let text = "The implementation of the configuration requires completion of the \
                    serialization step.";
        // implementation, configuration, completion, serialization = 4 → not > 4 on dev-doc
        assert!(lint(text, Profile::DevDoc, Language::En).is_empty());
        assert!(!lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 3 nominalizations: passes Public (max 3) but fails FALC (max 2).
        let text = "The implementation of the configuration requires attention.";
        // implementation, configuration, attention = 3 → FALC > 2 triggers
        assert!(lint(text, Profile::Public, Language::En).is_empty());
        assert!(!lint(text, Profile::Falc, Language::En).is_empty());
    }

    #[test]
    fn french_suffixes_match_in_french() {
        let text = "La réalisation de l'analyse de la conformité permettra l'identification \
                    des axes d'amélioration.";
        // réalisation, conformité, identification, amélioration = 4 → Public triggers
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn short_words_are_ignored() {
        // "ness" or "age" as isolated short words should not count.
        let text = "It had age and ness and tion and sion and ence and ance.";
        assert!(lint(text, Profile::Falc, Language::En).is_empty());
    }

    #[test]
    fn case_insensitive_match() {
        let text = "The IMPLEMENTATION and the COMPLETION and the CONFIGURATION and the \
                    NORMALIZATION overlap.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn per_sentence_scope() {
        // Each sentence is checked independently.
        let text = "The completion is fine. The implementation of the configuration requires \
                    further normalization of the serialization and deserialization.";
        let diags = lint(text, Profile::Public, Language::En);
        // Only the second sentence has enough nominalizations.
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn unknown_language_skips_rule() {
        let text = "Implementation configuration completion normalization serialization.";
        assert!(lint(text, Profile::Public, Language::Unknown).is_empty());
    }

    #[test]
    fn custom_suffix_list_overrides_defaults() {
        let cfg = Config {
            max_per_sentence: NonZeroU32::new(1).expect("non-zero literal"),
            custom_suffixes: vec!["xyz".to_string()],
        };
        let doc = parse_plain(
            "The foo and the implementation and the configuration.",
            SourceFile::Anonymous,
        );
        let diags = ExcessiveNominalization::new(cfg).check(&doc, Language::En);
        // No word ends with "xyz" so nothing fires.
        assert!(diags.is_empty());
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert_eq!(
            Config::for_profile(Profile::DevDoc).max_per_sentence.get(),
            4
        );
        assert_eq!(
            Config::for_profile(Profile::Public).max_per_sentence.get(),
            3
        );
        assert_eq!(Config::for_profile(Profile::Falc).max_per_sentence.get(), 2);
    }

    #[test]
    fn category_is_lexicon() {
        let text = "The implementation of the configuration requires completion of the \
                    serialization step here.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Lexicon);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "The implementation of the configuration requires completion of the \
                    serialization and deserialization steps.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
