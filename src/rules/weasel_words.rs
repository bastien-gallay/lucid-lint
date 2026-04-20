//! Rule: `weasel-words`.
//!
//! Flags vague qualifiers that weaken a statement. A weasel word silently
//! asks the reader to decide whether the qualification matters, is true,
//! or is measurable — attention this reader was not supposed to spend.
//!
//! Word lists are per-language; see
//! [`crate::language::en::WEASELS`] and [`crate::language::fr::WEASELS`].
//! The rule skips documents whose language is [`Language::Unknown`]
//! rather than guessing.
//!
//! See [`RULES.md`](../../RULES.md#weasel-words) for references.

use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::phrase_search::{find_word_bounded, line_column_at};
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`WeaselWords`].
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Additional weasel phrases for English, in lowercase.
    pub custom_weasels_en: Vec<String>,

    /// Additional weasel phrases for French, in lowercase.
    pub custom_weasels_fr: Vec<String>,

    /// Weasel phrases to silence from the defaults (exact lowercase match).
    pub disable: Vec<String>,
}

impl Config {
    /// Build a config from a profile preset. All profiles share the same
    /// defaults; individual suppression is done via inline directives or
    /// the `disable` list.
    #[must_use]
    pub fn for_profile(_profile: Profile) -> Self {
        Self::default()
    }
}

/// The [`WeaselWords`] rule.
#[derive(Debug, Clone)]
pub struct WeaselWords {
    config: Config,
}

impl WeaselWords {
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
    pub const ID: &'static str = "weasel-words";

    fn phrases_for(&self, language: Language) -> Vec<String> {
        let (defaults, custom) = match language {
            Language::En => (
                en::WEASELS.iter().copied().collect::<Vec<_>>(),
                self.config.custom_weasels_en.as_slice(),
            ),
            Language::Fr => (
                fr::WEASELS.iter().copied().collect::<Vec<_>>(),
                self.config.custom_weasels_fr.as_slice(),
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

impl Rule for WeaselWords {
    fn id(&self) -> &'static str {
        Self::ID
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
        "Weasel phrase \"{phrase}\" weakens the statement. Replace with concrete language or \
         remove it."
    );
    let diag = Diagnostic::new(WeaselWords::ID, Severity::Warning, location, message);
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
        WeaselWords::for_profile(Profile::Public).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(WeaselWords::ID, "weasel-words");
    }

    #[test]
    fn clean_text_does_not_trigger() {
        assert!(lint("The binary compiles and runs.", Language::En).is_empty());
    }

    #[test]
    fn single_word_weasel_is_flagged() {
        let diags = lint("The output is basically correct.", Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("basically"));
    }

    #[test]
    fn multi_word_weasel_is_flagged() {
        let diags = lint("This is sort of the point.", Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("sort of"));
    }

    #[test]
    fn case_insensitive_match() {
        let diags = lint("Clearly, the situation is Obviously fine.", Language::En);
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn word_boundary_prevents_partial_matches() {
        // "justice" must not match "just".
        assert!(lint("Social justice matters.", Language::En).is_empty());
        // "numerous" should still match itself.
        assert!(!lint("Numerous errors were found.", Language::En).is_empty());
    }

    #[test]
    fn french_weasel_is_flagged() {
        let diags = lint(
            "Il semble que la plupart des utilisateurs soient d'accord.",
            Language::Fr,
        );
        assert!(diags.len() >= 2);
    }

    #[test]
    fn multi_word_french_requires_both_tokens() {
        // "beaucoup" alone is not a weasel; "beaucoup de" is.
        let alone = lint("Il y avait beaucoup.", Language::Fr);
        let with_de = lint("Il y avait beaucoup de bugs.", Language::Fr);
        assert!(alone.is_empty());
        assert_eq!(with_de.len(), 1);
    }

    #[test]
    fn unknown_language_skips_rule() {
        assert!(lint("Just some obvious weasels.", Language::Unknown).is_empty());
    }

    #[test]
    fn disable_list_silences_specific_phrase() {
        let cfg = Config {
            disable: vec!["basically".to_string()],
            ..Config::default()
        };
        let doc = parse_plain("Clearly basically fine.", SourceFile::Anonymous);
        let diags = WeaselWords::new(cfg).check(&doc, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("clearly"));
    }

    #[test]
    fn custom_weasels_augment_defaults() {
        let cfg = Config {
            custom_weasels_en: vec!["possibly".to_string()],
            ..Config::default()
        };
        let doc = parse_plain("This is possibly true.", SourceFile::Anonymous);
        let diags = WeaselWords::new(cfg).check(&doc, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("possibly"));
    }

    #[test]
    fn category_is_lexical() {
        let diags = lint("Clearly wrong.", Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Lexical);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Clearly the system is basically fine, though it has some issues.";
        let diags = lint(text, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
