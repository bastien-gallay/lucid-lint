//! Rule: `passive-voice`.
//!
//! Flags paragraphs whose passive-voice count exceeds a threshold.
//! Passive voice hides the agent and lengthens the sentence. Legitimate
//! uses exist (unknown agent, scientific style); the rule flags, the
//! author decides.
//!
//! The v0.1 detector is a pattern heuristic: `be`-form (EN) or
//! `être`-form (FR) followed by a past-participle-shaped token within a
//! short window. Expected precision: ~70–80%, as documented in
//! `RULES.md`. A POS-parser-based detector is a future `lucid-lint-nlp`
//! plugin candidate.

use std::collections::HashSet;
use std::sync::LazyLock;

use unicode_segmentation::UnicodeSegmentation;

use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

static EN_BE_FORMS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    ["am", "is", "are", "was", "were", "be", "been", "being"]
        .into_iter()
        .collect()
});

/// Most common irregular English past participles. Deliberately short;
/// the `-ed` suffix heuristic covers the regular cases.
static EN_IRREGULAR_PP: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "been",
        "done",
        "said",
        "made",
        "seen",
        "taken",
        "given",
        "known",
        "written",
        "read",
        "shown",
        "put",
        "kept",
        "found",
        "brought",
        "sent",
        "left",
        "gone",
        "heard",
        "told",
        "run",
        "become",
        "met",
        "held",
        "lost",
        "felt",
        "meant",
        "built",
        "paid",
        "set",
        "grown",
        "spent",
        "sold",
        "bought",
        "got",
        "stood",
        "drawn",
        "caught",
        "forgotten",
        "hit",
        "chosen",
        "broken",
    ]
    .into_iter()
    .collect()
});

static FR_ETRE_FORMS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "suis", "es", "est", "sommes", "êtes", "sont", "étais", "était", "étions", "étiez",
        "étaient", "serai", "sera", "seront", "seraient", "fut", "furent", "été",
    ]
    .into_iter()
    .collect()
});

/// Maximum number of tokens to scan ahead of a be-form before giving up
/// on finding a past participle (covers "was very carefully reviewed").
const LOOKAHEAD_TOKENS: usize = 3;

/// Configuration for [`PassiveVoice`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum passive constructions allowed per paragraph. Triggers
    /// strictly above this value.
    pub max_per_paragraph: u32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub const fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 3,
            Profile::Public => 1,
            Profile::Falc => 0,
        };
        Self {
            max_per_paragraph: max,
        }
    }
}

/// The [`PassiveVoice`] rule.
#[derive(Debug, Clone, Copy)]
pub struct PassiveVoice {
    config: Config,
}

impl PassiveVoice {
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
    pub const ID: &'static str = "passive-voice";
}

impl Rule for PassiveVoice {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_per_paragraph;
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            let count = count_passives(&paragraph.text, paragraph.start_line, language);
            if count > max {
                diagnostics.push(build_diagnostic(
                    &document.source,
                    paragraph.start_line,
                    &paragraph.text,
                    count,
                    max,
                    section_title,
                ));
            }
        }
        diagnostics
    }
}

fn count_passives(paragraph_text: &str, start_line: u32, language: Language) -> u32 {
    let mut total: u32 = 0;
    for sentence in split_sentences(paragraph_text, start_line, 1) {
        total = total.saturating_add(count_sentence_passives(&sentence.text, language));
    }
    total
}

fn count_sentence_passives(sentence: &str, language: Language) -> u32 {
    let (aux_set, pp_check): (&HashSet<&'static str>, fn(&str) -> bool) = match language {
        Language::En => (&EN_BE_FORMS, is_en_past_participle),
        Language::Fr => (&FR_ETRE_FORMS, is_fr_past_participle),
        Language::Unknown => return 0,
    };

    let lower = sentence.to_lowercase();
    let words: Vec<&str> = lower.unicode_words().collect();
    let mut count: u32 = 0;
    let mut i = 0;
    while i < words.len() {
        let token = words[i].trim_end_matches('\'');
        if aux_set.contains(token) {
            let end = (i + 1 + LOOKAHEAD_TOKENS).min(words.len());
            let matched = words
                .iter()
                .enumerate()
                .skip(i + 1)
                .take(end - i - 1)
                .find(|(_, w)| pp_check(w));
            if let Some((j, _)) = matched {
                count = count.saturating_add(1);
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    count
}

fn is_en_past_participle(word: &str) -> bool {
    if EN_IRREGULAR_PP.contains(word) {
        return true;
    }
    word.len() > 3 && word.ends_with("ed")
}

fn is_fr_past_participle(word: &str) -> bool {
    // Common irregulars that do not match a standard suffix rule,
    // including feminine forms that end with `-e` (which would otherwise
    // collide with ordinary adjectives).
    if matches!(
        word,
        "été"
            | "eu"
            | "fait"
            | "faite"
            | "faits"
            | "faites"
            | "mis"
            | "mise"
            | "mises"
            | "pris"
            | "prise"
            | "prises"
            | "dit"
            | "dite"
            | "dites"
            | "vu"
            | "vue"
            | "vues"
            | "né"
            | "née"
            | "nés"
            | "nées"
    ) {
        return true;
    }
    // Regular suffixes for the masculine/feminine singular/plural forms
    // of the past participle. Keep a minimum length to avoid matching
    // short adjectives.
    if word.len() < 4 {
        return false;
    }
    for suffix in [
        "ées", "ée", "és", "é", "ies", "ie", "is", "i", "ues", "ue", "us", "u",
    ] {
        if word.ends_with(suffix) {
            return true;
        }
    }
    false
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    paragraph_text: &str,
    count: u32,
    max: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(paragraph_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, 1, length);
    let message = format!(
        "Paragraph has {count} passive-voice constructions (max {max}). Prefer active voice, or \
         suppress with an inline directive if the passive is intentional."
    );
    let diag = Diagnostic::new(PassiveVoice::ID, Severity::Warning, location, message);
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
        PassiveVoice::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(PassiveVoice::ID, "passive-voice");
    }

    #[test]
    fn active_prose_does_not_trigger() {
        let text = "We shipped the release today. The team celebrated.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn single_passive_at_public_triggers_none() {
        // Public max is 1; a single passive is within budget.
        let text = "The release was shipped today.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn two_passives_trigger_at_public() {
        let text = "The release was shipped today. The team was thanked by management.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("2 passive"));
    }

    #[test]
    fn dev_doc_tolerates_three_passives() {
        let text = "The release was shipped. The team was thanked. Metrics were recorded.";
        // 3 passives — dev-doc max is 3, triggers strictly above.
        assert!(lint(text, Profile::DevDoc, Language::En).is_empty());
    }

    #[test]
    fn falc_flags_any_passive() {
        let text = "The release was shipped today.";
        let diags = lint(text, Profile::Falc, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn adverb_between_be_and_participle_is_allowed() {
        let text = "The release was quickly shipped. The team was promptly thanked.";
        let diags = lint(text, Profile::Public, Language::En);
        // 2 passives → above max 1.
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn subject_attribute_is_not_flagged() {
        // "He is happy" — be-form + adjective, not passive.
        let text = "He is happy. She is curious. They are calm.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn french_passive_is_detected() {
        // 2 passives in French under Public (max 1).
        let text = "Le rapport est validé par le comité. La décision est prise par le conseil.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn unknown_language_skips_rule() {
        let text = "The release was shipped. The team was thanked.";
        assert!(lint(text, Profile::Public, Language::Unknown).is_empty());
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_per_paragraph, 3);
        assert_eq!(Config::for_profile(Profile::Public).max_per_paragraph, 1);
        assert_eq!(Config::for_profile(Profile::Falc).max_per_paragraph, 0);
    }

    #[test]
    fn category_is_style() {
        let text = "The release was shipped. The team was thanked by management.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Style);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "The release was shipped today. The team was thanked by management.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
