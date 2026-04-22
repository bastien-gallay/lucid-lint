//! Rule: `deep-subordination`.
//!
//! Flags cascading subordinate clauses: two or more relative pronouns or
//! subordinating conjunctions in sequence, unbroken by strong punctuation.
//! Cascades force the reader to hold several open referents in working
//! memory at once (Gibson 1998, *Dependency Locality Theory*).
//!
//! Enumerations that list the subordinators themselves ("the French
//! pronouns are qui, que, dont, où") are excluded via the shared
//! enumeration detector.
//!
//! See [`RULES.md`](../../RULES.md#deep-subordination).

use std::collections::HashSet;
use std::num::NonZeroU32;
use std::sync::LazyLock;

use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::enumeration::detect_enumerations;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

static EN_SUBORDINATORS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // Relative pronouns / adverbs
        "which", "that", "who", "whom", "whose", "where", "when", "why",
        // Subordinating conjunctions
        "although", "because", "since", "unless", "until", "though", "while", "before", "after",
        "whereas",
    ]
    .into_iter()
    .collect()
});

static FR_SUBORDINATORS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "qui",
        "que",
        "qu",
        "dont",
        "où",
        "lequel",
        "laquelle",
        "lesquels",
        "lesquelles",
        "quand",
        "lorsque",
        "lorsqu",
        "puisque",
        "puisqu",
        "quoique",
        "quoiqu",
        "tandis",
        "pendant",
        "comme",
        "afin",
    ]
    .into_iter()
    .collect()
});

/// Configuration for [`DeepSubordination`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum consecutive subordinators allowed before the rule fires.
    /// A run strictly exceeding this triggers.
    pub max_consecutive_subordinators: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 3,
            Profile::Public | Profile::Falc => 2,
        };
        Self {
            max_consecutive_subordinators: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`DeepSubordination`] rule.
#[derive(Debug, Clone, Copy)]
pub struct DeepSubordination {
    config: Config,
}

impl DeepSubordination {
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
    pub const ID: &'static str = "structure.deep-subordination";
}

impl Rule for DeepSubordination {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let Some(subordinators) = subordinators_for(language) else {
            return Vec::new();
        };
        let max = self.config.max_consecutive_subordinators.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            for sentence in split_sentences(&paragraph.text, paragraph.start_line, 1) {
                let enum_spans = detect_enumerations(&sentence.text, language);
                let run = longest_subordinator_run(&sentence.text, subordinators, &enum_spans);
                if run > max {
                    diagnostics.push(build_diagnostic(
                        &document.source,
                        sentence.line,
                        sentence.column,
                        &sentence.text,
                        run,
                        max,
                        section_title,
                    ));
                }
            }
        }
        diagnostics
    }
}

fn subordinators_for(language: Language) -> Option<&'static HashSet<&'static str>> {
    match language {
        Language::En => Some(&EN_SUBORDINATORS),
        Language::Fr => Some(&FR_SUBORDINATORS),
        Language::Unknown => None,
    }
}

fn longest_subordinator_run(
    sentence: &str,
    subordinators: &HashSet<&'static str>,
    enum_spans: &[crate::rules::enumeration::Enumeration],
) -> u32 {
    let lower = sentence.to_lowercase();
    let bytes = sentence.as_bytes();
    let len = bytes.len();
    // Count subordinators per "clause group" — a run of text not broken by
    // a semicolon or colon. Intervening words between subordinators do
    // not reset the count; only strong punctuation does.
    let mut current: u32 = 0;
    let mut best: u32 = 0;
    let mut i = 0;
    while i < len {
        let b = bytes[i];
        if b == b';' || b == b':' {
            current = 0;
            i += 1;
            continue;
        }
        if !is_token_byte(b) {
            i += 1;
            continue;
        }
        let start = i;
        while i < len && is_token_byte(bytes[i]) {
            i += 1;
        }
        let token = &lower[start..i];
        let normalized = token.trim_end_matches('\'');
        if subordinators.contains(normalized) && !is_in_enumeration(start, enum_spans) {
            current = current.saturating_add(1);
            best = best.max(current);
        }
    }
    best
}

fn is_token_byte(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'\'' || b >= 0x80
}

fn is_in_enumeration(offset: usize, spans: &[crate::rules::enumeration::Enumeration]) -> bool {
    spans.iter().any(|e| offset >= e.start && offset < e.end)
}

#[allow(clippy::too_many_arguments)]
fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    sentence_text: &str,
    run: u32,
    max: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(sentence_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Sentence chains {run} subordinators in a row (max {max}). Split the sentence or \
         replace a subordinate clause with an independent one."
    );
    let diag = Diagnostic::new(DeepSubordination::ID, Severity::Warning, location, message);
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
        DeepSubordination::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(DeepSubordination::ID, "structure.deep-subordination");
    }

    #[test]
    fn simple_sentence_does_not_trigger() {
        assert!(lint("The cat sat on the mat.", Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn single_subordinate_does_not_trigger() {
        let text = "The book that she wrote is excellent.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn two_stacked_subordinators_at_public_do_not_trigger() {
        // Public max is 2, triggers strictly above.
        let text = "The book that you said which was written by her is good.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn three_stacked_subordinators_trigger_at_public() {
        let text = "The tool that the team which the company whose founders built it uses.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("subordinators"));
    }

    #[test]
    fn dev_doc_tolerates_three_stacked_subordinators() {
        let text = "The tool that the team which the company whose founders built it uses.";
        assert!(lint(text, Profile::DevDoc, Language::En).is_empty());
    }

    #[test]
    fn strong_punctuation_breaks_the_run() {
        // Colon resets the cascade.
        let text = "The book that you said: which was written by her and that she loves.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn french_cascade_triggers() {
        let text = "Le document qui a été rédigé que nous avons constitué où nous sommes.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn enumeration_of_pronouns_is_ignored() {
        // RULES.md false-positive example: a list of pronouns should not
        // count as a cascade.
        let text = "Les pronoms relatifs en français sont: qui, que, dont, et où.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn unknown_language_skips_rule() {
        let text = "The tool that the team which the company whose founders built it uses.";
        assert!(lint(text, Profile::Public, Language::Unknown).is_empty());
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert_eq!(
            Config::for_profile(Profile::DevDoc)
                .max_consecutive_subordinators
                .get(),
            3
        );
        assert_eq!(
            Config::for_profile(Profile::Public)
                .max_consecutive_subordinators
                .get(),
            2
        );
        assert_eq!(
            Config::for_profile(Profile::Falc)
                .max_consecutive_subordinators
                .get(),
            2
        );
    }

    #[test]
    fn category_is_structure() {
        let text = "The tool that the team which the company whose founders built it uses.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Structure);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "The tool that the team which the company whose founders built it uses.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
