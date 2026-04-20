//! Rule condition tags (F71).
//!
//! Each [`Rule`](crate::rules::Rule) declares one or more [`ConditionTag`]s
//! describing the cognitive condition the rule primarily targets. The tag
//! ontology is fixed; profiles (`dev-doc`, `public`, `falc`) and condition
//! tags are orthogonal axes — profiles set strictness, conditions enable
//! audience-specific signals.
//!
//! Filter semantics applied by the engine (F72):
//!
//! - A rule with [`ConditionTag::General`] in its tag list is always
//!   enabled.
//! - A rule without [`ConditionTag::General`] runs only when the user's
//!   `conditions = [...]` list intersects with the rule's tags.
//!
//! The 17 v0.2 rules are all `general`, so the default behavior is
//! unchanged.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Cognitive condition a rule targets.
///
/// The set is fixed. Adding a variant is a deliberate API change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConditionTag {
    /// Prose-adjacent markup signals (e.g. all-caps shouting, missing alt
    /// text). WCAG / RGAA territory.
    A11yMarkup,
    /// Dyslexia-targeted signals (BDA Dyslexia Style Guide).
    Dyslexia,
    /// Dyscalculia-targeted signals (numeric format, anchoring).
    Dyscalculia,
    /// Aphasia-targeted signals (FALC, plain language).
    Aphasia,
    /// Attention-fragility signals (ADHD).
    Adhd,
    /// Non-native reader signals (vocabulary rarity, idiomatic phrasing).
    NonNative,
    /// Always-on rules. Most v0.2 rules carry this tag.
    General,
}

impl ConditionTag {
    /// Fixed iteration order, used by docs and tests.
    pub const ALL: [Self; 7] = [
        Self::A11yMarkup,
        Self::Dyslexia,
        Self::Dyscalculia,
        Self::Aphasia,
        Self::Adhd,
        Self::NonNative,
        Self::General,
    ];

    /// Canonical kebab-case name (matches CLI / config spelling).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A11yMarkup => "a11y-markup",
            Self::Dyslexia => "dyslexia",
            Self::Dyscalculia => "dyscalculia",
            Self::Aphasia => "aphasia",
            Self::Adhd => "adhd",
            Self::NonNative => "non-native",
            Self::General => "general",
        }
    }
}

impl fmt::Display for ConditionTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error returned when a string does not match a known [`ConditionTag`].
#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown condition tag `{0}` (expected one of: a11y-markup, dyslexia, dyscalculia, aphasia, adhd, non-native, general)")]
pub struct UnknownConditionTag(pub String);

impl FromStr for ConditionTag {
    type Err = UnknownConditionTag;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a11y-markup" | "a11y_markup" => Ok(Self::A11yMarkup),
            "dyslexia" => Ok(Self::Dyslexia),
            "dyscalculia" => Ok(Self::Dyscalculia),
            "aphasia" => Ok(Self::Aphasia),
            "adhd" => Ok(Self::Adhd),
            "non-native" | "non_native" | "nonnative" => Ok(Self::NonNative),
            "general" => Ok(Self::General),
            other => Err(UnknownConditionTag(other.to_string())),
        }
    }
}

/// Decide whether a rule should run given its declared tags and the user's
/// active condition list.
///
/// A rule with [`ConditionTag::General`] is always enabled. Otherwise the
/// rule runs iff its tag set intersects with `active`.
#[must_use]
pub fn rule_enabled(rule_tags: &[ConditionTag], active: &[ConditionTag]) -> bool {
    if rule_tags.contains(&ConditionTag::General) {
        return true;
    }
    rule_tags.iter().any(|t| active.contains(t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_through_str() {
        for tag in ConditionTag::ALL {
            assert_eq!(ConditionTag::from_str(tag.as_str()).unwrap(), tag);
        }
    }

    #[test]
    fn from_str_is_case_insensitive_and_accepts_underscores() {
        assert_eq!(
            ConditionTag::from_str("A11Y-MARKUP").unwrap(),
            ConditionTag::A11yMarkup
        );
        assert_eq!(
            ConditionTag::from_str("non_native").unwrap(),
            ConditionTag::NonNative
        );
    }

    #[test]
    fn from_str_rejects_unknown() {
        assert_eq!(
            ConditionTag::from_str("autism"),
            Err(UnknownConditionTag("autism".to_string()))
        );
    }

    #[test]
    fn general_rules_always_run_regardless_of_active_conditions() {
        assert!(rule_enabled(&[ConditionTag::General], &[]));
        assert!(rule_enabled(
            &[ConditionTag::General, ConditionTag::Dyslexia],
            &[ConditionTag::Adhd]
        ));
    }

    #[test]
    fn condition_only_rules_require_intersection() {
        assert!(!rule_enabled(&[ConditionTag::Dyslexia], &[]));
        assert!(!rule_enabled(
            &[ConditionTag::Dyslexia],
            &[ConditionTag::Adhd]
        ));
        assert!(rule_enabled(
            &[ConditionTag::Dyslexia],
            &[ConditionTag::Dyslexia]
        ));
        assert!(rule_enabled(
            &[ConditionTag::Dyslexia, ConditionTag::Adhd],
            &[ConditionTag::Adhd]
        ));
    }

    #[test]
    fn serde_uses_kebab_case() {
        let json = serde_json::to_string(&ConditionTag::A11yMarkup).unwrap();
        assert_eq!(json, "\"a11y-markup\"");
        let parsed: ConditionTag = serde_json::from_str("\"non-native\"").unwrap();
        assert_eq!(parsed, ConditionTag::NonNative);
    }
}
