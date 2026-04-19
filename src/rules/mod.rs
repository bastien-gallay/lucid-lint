//! Lint rules.
//!
//! Each rule implements the [`Rule`] trait in its own file under this module.
//! The reference implementation to pattern-match against when adding a new rule
//! is [`sentence_too_long`] — it is intentionally minimal, well-tested, and
//! demonstrates the canonical structure.

use crate::config::Profile;
use crate::parser::Document;
use crate::types::{Diagnostic, Language};

pub mod paragraph_too_long;
pub mod sentence_too_long;

pub use paragraph_too_long::ParagraphTooLong;
pub use sentence_too_long::SentenceTooLong;

/// Common interface for a lint rule.
///
/// A rule receives a parsed [`Document`], plus context about the target
/// profile and detected language, and returns a list of [`Diagnostic`]s.
///
/// Rules MUST be deterministic: identical input must yield identical output.
///
/// # Design notes
///
/// - A rule should be constructible with a single `new(profile)` call.
/// - Configuration is resolved at construction time from the [`Profile`].
/// - Per-document state (e.g., running counters) lives in local variables
///   inside [`Rule::check`], not in the rule struct.
pub trait Rule {
    /// The rule id in kebab-case (e.g., `"sentence-too-long"`).
    ///
    /// Must match the filename of the rule module.
    fn id(&self) -> &'static str;

    /// Analyze a document and return any diagnostics.
    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic>;
}

/// Build the default set of rules for a given profile.
///
/// Rules are added incrementally following the pattern established by
/// [`SentenceTooLong`].
#[must_use]
pub fn default_rules(profile: Profile) -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(SentenceTooLong::for_profile(profile)),
        Box::new(ParagraphTooLong::for_profile(profile)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rules_is_non_empty() {
        let rules = default_rules(Profile::Public);
        assert!(!rules.is_empty());
    }

    #[test]
    fn each_rule_has_a_nonempty_id() {
        for rule in default_rules(Profile::Public) {
            assert!(!rule.id().is_empty());
            assert!(rule
                .id()
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-'));
        }
    }
}
