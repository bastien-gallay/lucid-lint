//! Rule: `long-enumeration`.
//!
//! Flags inline prose enumerations that would be clearer as bulleted
//! lists. A prose list of five-plus items is harder to scan than a
//! proper list structure.
//!
//! Shares detection logic with [`crate::rules::structure::excessive_commas`] via
//! [`crate::rules::enumeration::detect_enumerations`] so that both rules
//! agree on what counts as an enumeration.
//!
//! See [`RULES.md`](../../RULES.md#long-enumeration).

use std::num::NonZeroU32;

use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::enumeration::detect_enumerations;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`LongEnumeration`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Minimum number of items for a prose enumeration to trigger.
    pub min_items: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset. All profiles currently share
    /// the same threshold.
    #[must_use]
    pub fn for_profile(_profile: Profile) -> Self {
        Self {
            min_items: NonZeroU32::new(5).expect("non-zero literal"),
        }
    }
}

/// The [`LongEnumeration`] rule.
#[derive(Debug, Clone, Copy)]
pub struct LongEnumeration {
    config: Config,
}

impl LongEnumeration {
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
    pub const ID: &'static str = "structure.long-enumeration";
}

impl Rule for LongEnumeration {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let threshold = self.config.min_items.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            for sentence in split_sentences(&paragraph.text, paragraph.start_line, 1) {
                for enumeration in detect_enumerations(&sentence.text, language) {
                    if enumeration.items < threshold {
                        continue;
                    }
                    diagnostics.push(build_diagnostic(
                        &document.source,
                        sentence.line,
                        sentence.column,
                        &sentence.text,
                        enumeration.items,
                        section_title,
                    ));
                }
            }
        }

        diagnostics
    }
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    sentence_text: &str,
    items: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(sentence_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Inline enumeration of {items} items. Consider converting it into a bulleted list so \
         readers can scan the items."
    );
    let diag = Diagnostic::new(LongEnumeration::ID, Severity::Warning, location, message);
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
        LongEnumeration::for_profile(Profile::Public).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(LongEnumeration::ID, "structure.long-enumeration");
    }

    #[test]
    fn short_enumeration_does_not_trigger() {
        // 4 items — detection recognizes the pattern but rule needs 5+.
        let text = "Red, green, blue, and yellow work well together.";
        assert!(lint(text, Language::En).is_empty());
    }

    #[test]
    fn enumeration_of_five_items_triggers() {
        let text = "Red, green, blue, yellow, and purple make the palette.";
        let diags = lint(text, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("5 items"));
    }

    #[test]
    fn french_enumeration_triggers() {
        let text = "Rouge, vert, bleu, jaune, et violet composent la palette.";
        let diags = lint(text, Language::Fr);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn non_oxford_form_is_not_detected_in_v0_1() {
        // Out of scope for v0.1 (see detector docs).
        let text = "Red, green, blue, yellow and purple compose the palette.";
        assert!(lint(text, Language::En).is_empty());
    }

    #[test]
    fn category_is_structure() {
        let text = "Red, green, blue, yellow, and purple make the palette.";
        let diags = lint(text, Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Structure);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Red, green, blue, yellow, and purple make the palette.";
        let diags = lint(text, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
