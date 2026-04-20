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
                    if is_inside_inline_code(&paragraph.text, byte_offset) {
                        continue;
                    }
                    if is_directional_pair(&lowered, byte_offset, phrase, language) {
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

/// "rather than" and "plutôt que" are conjunctions meaning "instead of",
/// not hedges. Skip a hit when the lexical unit immediately after the
/// phrase is the expected directional follower for the language.
fn is_directional_pair(lowered: &str, offset: usize, phrase: &str, language: Language) -> bool {
    let follower = match (language, phrase) {
        (Language::En, "rather") => "than",
        (Language::Fr, "plutôt") => "que",
        _ => return false,
    };
    let end = offset + phrase.len();
    if end > lowered.len() {
        return false;
    }
    let tail = lowered[end..].trim_start();
    if !tail.starts_with(follower) {
        return false;
    }
    // Guard against matching a prefix (e.g. "quelque" for "que"): the
    // byte right after the follower must be a non-word character.
    tail.as_bytes()
        .get(follower.len())
        .copied()
        .map_or(true, |b| !(b.is_ascii_alphabetic() || b == b'\''))
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
    fn category_is_lexicon() {
        let diags = lint("Clearly wrong.", Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Lexicon);
    }

    #[test]
    fn weasel_inside_inline_code_is_skipped() {
        // Author is discussing the word itself; backticks mark it as a term,
        // not as an actual hedge.
        let diags = lint("The phrase `basically` weakens a statement.", Language::En);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn weasel_outside_inline_code_still_fires() {
        // Same word in prose (no backticks) remains a hit.
        let diags = lint(
            "The phrase `ok` is fine, but this code is basically correct.",
            Language::En,
        );
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("basically"));
    }

    #[test]
    fn rather_than_is_not_a_hedge() {
        let diags = lint("Use verbs rather than nominalizations.", Language::En);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn rather_alone_still_fires() {
        let diags = lint("The result is rather poor.", Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("rather"));
    }

    #[test]
    fn plutot_que_is_not_a_hedge() {
        let diags = lint(
            "Préférer les verbes plutôt que les substantifs.",
            Language::Fr,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn plutot_alone_still_fires() {
        let diags = lint("Le résultat est plutôt décevant.", Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("plutôt"));
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
