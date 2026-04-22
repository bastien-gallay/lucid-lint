//! Rule: `unclear-antecedent`.
//!
//! Flags two patterns that the v0.1 heuristic can catch without full
//! anaphora resolution:
//!
//! 1. A bare demonstrative pronoun at sentence start followed directly
//!    by a verb or auxiliary ("This motivated the change", "Cela a
//!    permis…") — no noun phrase binds the pronoun, so the reader has
//!    to guess what it refers to.
//! 2. A personal pronoun as the first word of a paragraph ("It is…",
//!    "Il faut…") — the antecedent is in a previous paragraph, if it
//!    exists at all.
//!
//! Severity is `info` because the heuristic is approximate. Full
//! anaphora resolution is a future `lucid-lint-nlp` plugin concern.
//!
//! See [`RULES.md`](../../RULES.md#unclear-antecedent).

use std::collections::HashSet;
use std::sync::LazyLock;

use unicode_segmentation::UnicodeSegmentation;

use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

static EN_DEMONSTRATIVES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["this", "that", "these", "those"].into_iter().collect());

static FR_DEMONSTRATIVES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "ce", "cela", "ceci", "ça", "celui", "celle", "ceux", "celles",
    ]
    .into_iter()
    .collect()
});

static EN_PERSONAL_PRONOUNS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["it", "they", "them"].into_iter().collect());

static FR_PERSONAL_PRONOUNS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["il", "elle", "ils", "elles"].into_iter().collect());

/// Tokens that, when following a bare demonstrative, indicate the
/// pronoun is standing alone (no bound noun). Kept tight to avoid
/// flagging legitimate noun phrases like "this approach".
static EN_VERBS_AFTER_DEM: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "is",
        "was",
        "are",
        "were",
        "has",
        "have",
        "had",
        "will",
        "would",
        "can",
        "could",
        "may",
        "might",
        "must",
        "does",
        "did",
        "makes",
        "made",
        "means",
        "meant",
        "shows",
        "showed",
        "motivated",
        "requires",
        "required",
        "enables",
        "enabled",
        "helps",
        "helped",
        "works",
        "worked",
    ]
    .into_iter()
    .collect()
});

static FR_VERBS_AFTER_DEM: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "est",
        "était",
        "a",
        "ont",
        "sera",
        "seront",
        "fut",
        "furent",
        "peut",
        "peuvent",
        "doit",
        "doivent",
        "va",
        "vont",
        "motiv",
        "permet",
        "permettra",
    ]
    .into_iter()
    .collect()
});

/// Configuration for [`UnclearAntecedent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Check for bare demonstratives at sentence start.
    pub check_demonstratives: bool,

    /// Check for personal pronouns as the first token of a paragraph.
    pub check_paragraph_start_pronouns: bool,
}

impl Config {
    /// Build a config from a profile preset. All profiles use the same
    /// defaults in v0.1.
    #[must_use]
    pub const fn for_profile(_profile: Profile) -> Self {
        Self {
            check_demonstratives: true,
            check_paragraph_start_pronouns: true,
        }
    }
}

/// The [`UnclearAntecedent`] rule.
#[derive(Debug, Clone, Copy)]
pub struct UnclearAntecedent {
    config: Config,
}

impl UnclearAntecedent {
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
    pub const ID: &'static str = "syntax.unclear-antecedent";
}

impl Rule for UnclearAntecedent {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let Some(lists) = PronounLists::for_language(language) else {
            return Vec::new();
        };

        let mut diagnostics = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            if self.config.check_paragraph_start_pronouns {
                if let Some(first) = first_word(&paragraph.text) {
                    if lists.personal.contains(first.as_str()) {
                        diagnostics.push(build_diagnostic(
                            &document.source,
                            paragraph.start_line,
                            1,
                            &paragraph.text,
                            format!(
                                "Paragraph opens with the pronoun \"{first}\". Readers have no \
                                 antecedent in this paragraph — name the referent instead."
                            ),
                            section_title,
                        ));
                    }
                }
            }

            if self.config.check_demonstratives {
                for sentence in split_sentences(&paragraph.text, paragraph.start_line, 1) {
                    if let Some(pronoun) = bare_demonstrative(&sentence.text, &lists) {
                        diagnostics.push(build_diagnostic(
                            &document.source,
                            sentence.line,
                            sentence.column,
                            &sentence.text,
                            format!(
                                "Sentence starts with a bare demonstrative \"{pronoun}\". Name \
                                 the referent (e.g. \"this change\", \"cette décision\") to \
                                 avoid forcing the reader to guess."
                            ),
                            section_title,
                        ));
                    }
                }
            }
        }
        diagnostics
    }
}

struct PronounLists {
    demonstratives: &'static HashSet<&'static str>,
    personal: &'static HashSet<&'static str>,
    verbs_after: &'static HashSet<&'static str>,
}

impl PronounLists {
    fn for_language(language: Language) -> Option<Self> {
        match language {
            Language::En => Some(Self {
                demonstratives: &EN_DEMONSTRATIVES,
                personal: &EN_PERSONAL_PRONOUNS,
                verbs_after: &EN_VERBS_AFTER_DEM,
            }),
            Language::Fr => Some(Self {
                demonstratives: &FR_DEMONSTRATIVES,
                personal: &FR_PERSONAL_PRONOUNS,
                verbs_after: &FR_VERBS_AFTER_DEM,
            }),
            Language::Unknown => None,
        }
    }
}

fn first_word(text: &str) -> Option<String> {
    text.unicode_words().next().map(str::to_lowercase)
}

fn bare_demonstrative(sentence: &str, lists: &PronounLists) -> Option<String> {
    let lower = sentence.to_lowercase();
    let mut iter = lower.unicode_words();
    let first = iter.next()?;
    if !lists.demonstratives.contains(first) {
        return None;
    }
    let second = iter.next()?;
    // Flag only when the next token is a verb/auxiliary — otherwise the
    // demonstrative is likely binding a noun ("this approach", "cette
    // décision") and not ambiguous.
    if lists.verbs_after.contains(second) {
        Some(first.to_string())
    } else {
        None
    }
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    context_text: &str,
    message: String,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(context_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let diag = Diagnostic::new(UnclearAntecedent::ID, Severity::Info, location, message);
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

    fn lint(text: &str, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        UnclearAntecedent::for_profile(Profile::Public).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(UnclearAntecedent::ID, "syntax.unclear-antecedent");
    }

    #[test]
    fn bare_this_plus_verb_triggers() {
        let text = "The cache was slow. This motivated the change.";
        let diags = lint(text, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("this"));
    }

    #[test]
    fn this_plus_noun_does_not_trigger() {
        // "this approach" is a bound demonstrative — no ambiguity.
        let text = "The cache was slow. This approach motivated the change.";
        assert!(lint(text, Language::En).is_empty());
    }

    #[test]
    fn french_cela_plus_verb_triggers() {
        let text = "Les performances étaient médiocres avec le cache LRU. Cela a motivé le \
                    changement.";
        let diags = lint(text, Language::Fr);
        assert!(!diags.is_empty());
        assert!(diags[0].message.contains("cela"));
    }

    #[test]
    fn paragraph_starting_with_it_triggers() {
        let text = "It was raining throughout the afternoon.";
        let diags = lint(text, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("it"));
    }

    #[test]
    fn paragraph_not_starting_with_pronoun_does_not_trigger() {
        let text = "The rain fell throughout the afternoon.";
        assert!(lint(text, Language::En).is_empty());
    }

    #[test]
    fn severity_is_info() {
        let text = "The cache was slow. This motivated the change.";
        let diags = lint(text, Language::En);
        assert_eq!(diags[0].severity, Severity::Info);
    }

    #[test]
    fn category_is_syntax() {
        let text = "The cache was slow. This motivated the change.";
        let diags = lint(text, Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Syntax);
    }

    #[test]
    fn unknown_language_skips_rule() {
        let text = "The cache was slow. This motivated the change.";
        assert!(lint(text, Language::Unknown).is_empty());
    }

    #[test]
    fn check_demonstratives_toggle_off_silences_demonstratives() {
        let cfg = Config {
            check_demonstratives: false,
            check_paragraph_start_pronouns: false,
        };
        let doc = parse_plain(
            "The cache was slow. This motivated the change.",
            SourceFile::Anonymous,
        );
        let diags = UnclearAntecedent::new(cfg).check(&doc, Language::En);
        assert!(diags.is_empty());
    }

    #[test]
    fn snapshot_fixture() {
        let text = "It was late. The cache was slow. This motivated the change.";
        let diags = lint(text, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
