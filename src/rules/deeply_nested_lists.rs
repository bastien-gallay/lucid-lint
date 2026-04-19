//! Rule: `deeply-nested-lists`.
//!
//! Flags list items nested beyond a readable depth. Horizontal indentation
//! is a positional cue, and more than a few levels of nesting forces the
//! reader to reconstruct a mental hierarchy that defeats the scanning
//! benefit lists are meant to offer.
//!
//! See [`RULES.md`](../../RULES.md#deeply-nested-lists) for the rule's
//! rationale and thresholds.

use std::num::NonZeroU32;

use crate::config::Profile;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`DeeplyNestedLists`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed nesting depth. Level 1 is the outermost list.
    pub max_depth: NonZeroU32,
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
            max_depth: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`DeeplyNestedLists`] rule.
#[derive(Debug, Clone, Copy)]
pub struct DeeplyNestedLists {
    config: Config,
}

impl DeeplyNestedLists {
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
    pub const ID: &'static str = "deeply-nested-lists";
}

impl Rule for DeeplyNestedLists {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_depth.get();
        document
            .list_items
            .iter()
            .filter(|item| item.depth > max)
            .map(|item| build_diagnostic(&document.source, item.line, item.depth, max))
            .collect()
    }
}

fn build_diagnostic(source: &SourceFile, line: u32, depth: u32, max: u32) -> Diagnostic {
    let location = Location::new(source.clone(), line, 1, 1);
    let message = format!(
        "List item at depth {depth} exceeds maximum depth of {max}. Consider flattening the \
         structure, splitting into multiple lists, or using subsections with headings."
    );
    Diagnostic::new(DeeplyNestedLists::ID, Severity::Warning, location, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_markdown;
    use crate::types::SourceFile;

    fn lint(md: &str, profile: Profile) -> Vec<Diagnostic> {
        let doc = parse_markdown(md, SourceFile::Anonymous);
        DeeplyNestedLists::for_profile(profile).check(&doc, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(DeeplyNestedLists::ID, "deeply-nested-lists");
    }

    #[test]
    fn flat_list_does_not_trigger() {
        let md = "- one\n- two\n- three\n";
        assert!(lint(md, Profile::Public).is_empty());
    }

    #[test]
    fn at_threshold_does_not_trigger() {
        // Public: max 3 → depth 3 is allowed.
        let md = "- a\n  - b\n    - c\n";
        assert!(lint(md, Profile::Public).is_empty());
    }

    #[test]
    fn beyond_threshold_triggers_once_per_offending_item() {
        // Public: max 3 → depth 4 triggers.
        let md = "- a\n  - b\n    - c\n      - d\n";
        let diags = lint(md, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("depth 4"));
    }

    #[test]
    fn falc_profile_is_stricter() {
        // depth 3: passes Public (3), fails FALC (2).
        let md = "- a\n  - b\n    - c\n";
        assert!(lint(md, Profile::Public).is_empty());
        assert!(!lint(md, Profile::Falc).is_empty());
    }

    #[test]
    fn ordered_and_unordered_lists_both_count() {
        let md = "1. a\n   1. b\n      1. c\n         1. d\n";
        let diags = lint(md, Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_depth.get(), 4);
        assert_eq!(Config::for_profile(Profile::Public).max_depth.get(), 3);
        assert_eq!(Config::for_profile(Profile::Falc).max_depth.get(), 2);
    }

    #[test]
    fn category_is_structure() {
        let md = "- a\n  - b\n    - c\n      - d\n";
        let diags = lint(md, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Structure);
    }

    #[test]
    fn snapshot_fixture() {
        let md = "- a\n  - b\n    - c\n      - d\n        - e\n";
        let diags = lint(md, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
