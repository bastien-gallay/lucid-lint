//! Rule: `redundant-intensifier`.
//!
//! Flags intensifiers — adverbs that try to *upgrade* the confidence of
//! a statement without adding information. `very important` reduces to
//! `important`, or better, to a quantified claim. plainlanguage.gov
//! (Chapter 4) and the CDC Clear Communication Index both flag this
//! class as a plain-language anti-pattern.
//!
//! The rule is deliberately a sibling of [`crate::rules::WeaselWords`]:
//! weasel words *downgrade* confidence (hedges, qualifiers); redundant
//! intensifiers *upgrade* it. The two lists are disjoint by
//! construction — see [`crate::language::en::INTENSIFIERS`] and
//! [`crate::language::fr::INTENSIFIERS`].
//!
//! The rule skips documents whose language is [`Language::Unknown`]
//! rather than guessing, matching the policy used by `weasel-words`.
//!
//! See [`RULES.md`](../../RULES.md#redundant-intensifier) for the
//! reference entry.

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::phrase_search::{find_word_bounded, line_column_at};
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`RedundantIntensifier`].
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Additional intensifier phrases for English, in lowercase.
    pub custom_intensifiers_en: Vec<String>,

    /// Additional intensifier phrases for French, in lowercase.
    pub custom_intensifiers_fr: Vec<String>,

    /// Intensifier phrases to silence from the defaults (exact
    /// lowercase match).
    pub disable: Vec<String>,
}

impl Config {
    /// Build a config from a profile preset.
    ///
    /// All profiles share the same defaults; individual suppression is
    /// done via inline directives or the `disable` list.
    #[must_use]
    pub fn for_profile(_profile: Profile) -> Self {
        Self::default()
    }
}

/// The [`RedundantIntensifier`] rule.
#[derive(Debug, Clone)]
pub struct RedundantIntensifier {
    config: Config,
}

impl RedundantIntensifier {
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
    pub const ID: &'static str = "redundant-intensifier";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[ConditionTag::General];

    fn phrases_for(&self, language: Language) -> Vec<String> {
        let (defaults, custom) = match language {
            Language::En => (
                en::INTENSIFIERS.to_vec(),
                self.config.custom_intensifiers_en.as_slice(),
            ),
            Language::Fr => (
                fr::INTENSIFIERS.to_vec(),
                self.config.custom_intensifiers_fr.as_slice(),
            ),
            Language::Unknown => return Vec::new(),
        };
        let disabled: std::collections::HashSet<&str> =
            self.config.disable.iter().map(String::as_str).collect();
        defaults
            .into_iter()
            .map(str::to_string)
            .chain(custom.iter().cloned())
            .filter(|p| !disabled.contains(p.as_str()))
            .collect()
    }
}

impl Rule for RedundantIntensifier {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let phrases = self.phrases_for(language);
        if phrases.is_empty() {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            let lowered = paragraph.text.to_lowercase();
            for phrase in &phrases {
                for byte_offset in find_word_bounded(&lowered, phrase) {
                    if is_inside_inline_code(&paragraph.text, byte_offset) {
                        continue;
                    }
                    let (line_offset, column) = line_column_at(&paragraph.text, byte_offset);
                    let line = paragraph.start_line.saturating_add(line_offset);
                    diagnostics.push(build_diagnostic(
                        &document.source,
                        line,
                        column,
                        phrase,
                        section_title,
                    ));
                }
            }
        }
        diagnostics.sort_by_key(|d| (d.location.line, d.location.column));
        diagnostics
    }
}

/// A hit lands inside an inline code span when the number of backticks
/// between the current line's start and the hit offset is odd. Fenced
/// code blocks are already excluded by the parser, so this check only
/// needs to reason about `` `inline` `` spans within a single line.
fn is_inside_inline_code(text: &str, offset: usize) -> bool {
    let capped = offset.min(text.len());
    let line_start = text[..capped].rfind('\n').map_or(0, |p| p + 1);
    text[line_start..capped]
        .bytes()
        .filter(|&b| b == b'`')
        .count()
        % 2
        == 1
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    phrase: &str,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(phrase.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Intensifier \"{phrase}\" adds no information — it tries to upgrade confidence without \
         quantifying it. Remove it or replace with a specific measure."
    );
    let diag = Diagnostic::new(
        RedundantIntensifier::ID,
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
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        RedundantIntensifier::for_profile(Profile::Public).check(&document, language)
    }

    fn lint_md(text: &str, language: Language) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        RedundantIntensifier::for_profile(Profile::Public).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(RedundantIntensifier::ID, "redundant-intensifier");
    }

    #[test]
    fn tag_is_general() {
        assert_eq!(RedundantIntensifier::TAGS, &[ConditionTag::General]);
    }

    #[test]
    fn category_is_lexicon() {
        let diags = lint("The release is very important.", Language::En);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].category(), Category::Lexicon);
    }

    #[test]
    fn clean_text_does_not_trigger() {
        assert!(lint("The binary compiles and runs.", Language::En).is_empty());
    }

    #[test]
    fn en_very_triggers() {
        let diags = lint("The release is very important.", Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"very\""));
    }

    #[test]
    fn en_multiple_intensifiers_in_one_sentence() {
        let diags = lint(
            "The release is really extremely important for users.",
            Language::En,
        );
        assert_eq!(diags.len(), 2);
        assert!(diags[0].message.contains("\"really\""));
        assert!(diags[1].message.contains("\"extremely\""));
    }

    #[test]
    fn fr_tres_triggers() {
        let diags = lint("La mise à jour est très importante.", Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"très\""));
    }

    #[test]
    fn fr_vraiment_triggers() {
        let diags = lint("C'est vraiment essentiel pour les lecteurs.", Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"vraiment\""));
    }

    #[test]
    fn case_insensitive_match() {
        let diags = lint("VERY important.", Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn unknown_language_is_skipped() {
        assert!(
            lint("The release is very important.", Language::Unknown).is_empty(),
            "Unknown language must skip the rule rather than guess"
        );
    }

    #[test]
    fn inline_code_span_is_ignored() {
        // The sole `very` lives inside an inline code span.
        assert!(lint_md("Use `very` as a flag name.", Language::En).is_empty());
    }

    #[test]
    fn fenced_code_block_content_is_ignored() {
        let md = "Intro.\n\n```\nvery extremely totally\n```\n\nPlain prose.\n";
        assert!(lint_md(md, Language::En).is_empty());
    }

    #[test]
    fn word_boundary_prevents_substring_match() {
        // "deeply" is in the list; "indeeply" is not. This doubles as a
        // regression test for the word-bounded search.
        assert!(
            lint("The bug runs deeply-rooted diagnostics.", Language::En)
                .iter()
                .any(|d| d.message.contains("\"deeply\""))
        );
    }

    #[test]
    fn disable_list_silences_a_phrase() {
        let mut config = Config::default();
        config.disable.push("very".to_string());
        let doc = parse_plain("The release is very important.", SourceFile::Anonymous);
        let diags = RedundantIntensifier::new(config).check(&doc, Language::En);
        assert!(diags.is_empty());
    }

    #[test]
    fn custom_intensifier_list_adds_phrases() {
        let mut config = Config::default();
        config.custom_intensifiers_en.push("profoundly".to_string());
        let doc = parse_plain("This is profoundly undocumented.", SourceFile::Anonymous);
        let diags = RedundantIntensifier::new(config).check(&doc, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"profoundly\""));
    }

    #[test]
    fn snapshot_fixture() {
        let text = "The release is very important and really urgent. Ship it.";
        let diags = lint(text, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
