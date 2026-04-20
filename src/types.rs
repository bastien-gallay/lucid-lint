//! Core domain types used across the crate.
//!
//! These types form the stable vocabulary for diagnostics, severities, locations,
//! and other concepts shared by multiple modules.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A single lint finding emitted by a [`Rule`](crate::rules::Rule).
///
/// [`Category`] is not stored because it is derivable from `rule_id`; use
/// [`Category::for_rule`] or [`Diagnostic::category`] when needed. [`weight`](Self::weight)
/// feeds the hybrid scoring model (see [`crate::scoring`]) and is populated at
/// emission from [`crate::scoring::default_weight_for`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Identifier of the rule that produced this diagnostic (kebab-case).
    pub rule_id: String,

    /// Severity level as declared by the rule at emission time.
    pub severity: Severity,

    /// File location where the issue was detected.
    pub location: Location,

    /// Optional heading under which the diagnostic falls, captured at emission
    /// because recomputing it from [`Location`] would require re-parsing the document.
    pub section: Option<String>,

    /// Human-readable message explaining the issue.
    pub message: String,

    /// Scoring weight for this diagnostic.
    ///
    /// Populated at [`Diagnostic::new`] from
    /// [`crate::scoring::default_weight_for`]. Rules that need a non-default
    /// weight can override via [`Diagnostic::with_weight`].
    pub weight: u32,
}

impl Diagnostic {
    /// Create a new diagnostic with no section attached.
    ///
    /// `weight` is seeded from [`crate::scoring::default_weight_for`] so that
    /// the scoring model works uniformly without rules needing to opt in.
    #[must_use]
    pub fn new(
        rule_id: impl Into<String>,
        severity: Severity,
        location: Location,
        message: impl Into<String>,
    ) -> Self {
        let rule_id = rule_id.into();
        let weight = crate::scoring::default_weight_for(&rule_id);
        Self {
            rule_id,
            severity,
            location,
            section: None,
            message: message.into(),
            weight,
        }
    }

    /// Attach a section label (typically a heading title) to this diagnostic.
    #[must_use]
    pub fn with_section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
        self
    }

    /// Override the default weight attached to this diagnostic.
    ///
    /// Most rules should leave the default alone; it is tuned centrally in
    /// [`crate::scoring::default_weight_for`].
    #[must_use]
    pub const fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    /// Returns the category this diagnostic belongs to, derived from `rule_id`.
    #[must_use]
    pub fn category(&self) -> Category {
        Category::for_rule(&self.rule_id)
    }
}

/// Severity of a [`Diagnostic`].
///
/// `error` is reserved for future use. v0.1 emits only `info` and `warning`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Signal worth knowing about. Does not fail CI by default.
    Info,
    /// Quality issue worth fixing. May fail CI depending on configuration.
    Warning,
    /// Reserved. Not emitted in v0.1.
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => f.write_str("info"),
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
        }
    }
}

/// Category groups rules by the nature of what they measure.
///
/// The taxonomy is fixed at 5 variants, matching the F14 scoring model
/// (see `brainstorm/20260420-score-semantics.md`): Structure, Rhythm,
/// Lexicon, Syntax, Readability.
///
/// Categories are stable enum variants used for filtering, grouping,
/// and scoring. They are derived from `rule_id`, not stored on [`Diagnostic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Length, nesting, punctuation, and document skeleton.
    Structure,
    /// Cadence and repetition across adjacent sentences.
    Rhythm,
    /// Vocabulary, terminology, acronyms, and lexical diversity.
    Lexicon,
    /// Sentence-level style and syntactic clarity.
    Syntax,
    /// Document-level readability metrics.
    Readability,
}

impl Category {
    /// The fixed ordering used by the scoring model and output surfaces.
    pub const ALL: [Self; 5] = [
        Self::Structure,
        Self::Rhythm,
        Self::Lexicon,
        Self::Syntax,
        Self::Readability,
    ];

    /// Map a rule id to its category.
    ///
    /// Unknown rule ids fall back to [`Category::Syntax`].
    #[must_use]
    pub fn for_rule(rule_id: &str) -> Self {
        match rule_id {
            "sentence-too-long"
            | "paragraph-too-long"
            | "deeply-nested-lists"
            | "heading-jump"
            | "excessive-commas"
            | "long-enumeration"
            | "deep-subordination" => Self::Structure,
            "consecutive-long-sentences" | "repetitive-connectors" => Self::Rhythm,
            "low-lexical-diversity"
            | "excessive-nominalization"
            | "unexplained-abbreviation"
            | "weasel-words"
            | "jargon-undefined" => Self::Lexicon,
            "passive-voice" | "unclear-antecedent" => Self::Syntax,
            "readability-score" => Self::Readability,
            _ => Self::Syntax,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Structure => f.write_str("structure"),
            Self::Rhythm => f.write_str("rhythm"),
            Self::Lexicon => f.write_str("lexicon"),
            Self::Syntax => f.write_str("syntax"),
            Self::Readability => f.write_str("readability"),
        }
    }
}

/// A 1-based file location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// Path of the source file, or a synthetic label for non-file inputs.
    pub file: SourceFile,

    /// 1-based line number.
    pub line: u32,

    /// 1-based column number, measured in Unicode grapheme clusters.
    pub column: u32,

    /// Length of the highlighted span in grapheme clusters.
    pub length: u32,
}

impl Location {
    /// Build a location with explicit line, column, and length.
    #[must_use]
    pub const fn new(file: SourceFile, line: u32, column: u32, length: u32) -> Self {
        Self {
            file,
            line,
            column,
            length,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Origin of a source text.
///
/// Distinguishes real files from stdin and from synthetic string inputs used
/// in tests and library contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "path", rename_all = "lowercase")]
pub enum SourceFile {
    /// Real file on disk.
    Path(PathBuf),
    /// Input read from standard input.
    Stdin,
    /// Synthetic input, typically used in tests or library callers.
    Anonymous,
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(p) => write!(f, "{}", p.display()),
            Self::Stdin => f.write_str("<stdin>"),
            Self::Anonymous => f.write_str("<input>"),
        }
    }
}

/// Detected or declared text language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// English.
    En,
    /// French.
    Fr,
    /// Language could not be confidently detected.
    Unknown,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::En => f.write_str("en"),
            Self::Fr => f.write_str("fr"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_for_rule_maps_known_ids() {
        assert_eq!(Category::for_rule("sentence-too-long"), Category::Structure);
        assert_eq!(Category::for_rule("excessive-commas"), Category::Structure);
        assert_eq!(
            Category::for_rule("consecutive-long-sentences"),
            Category::Rhythm
        );
        assert_eq!(
            Category::for_rule("repetitive-connectors"),
            Category::Rhythm
        );
        assert_eq!(Category::for_rule("weasel-words"), Category::Lexicon);
        assert_eq!(Category::for_rule("passive-voice"), Category::Syntax);
        assert_eq!(
            Category::for_rule("readability-score"),
            Category::Readability
        );
    }

    #[test]
    fn category_for_unknown_rule_defaults_to_syntax() {
        assert_eq!(Category::for_rule("no-such-rule"), Category::Syntax);
    }

    #[test]
    fn category_all_has_five_variants_in_fixed_order() {
        assert_eq!(Category::ALL.len(), 5);
        assert_eq!(Category::ALL[0], Category::Structure);
        assert_eq!(Category::ALL[4], Category::Readability);
    }

    #[test]
    fn severity_display_is_lowercase() {
        assert_eq!(Severity::Info.to_string(), "info");
        assert_eq!(Severity::Warning.to_string(), "warning");
        assert_eq!(Severity::Error.to_string(), "error");
    }

    #[test]
    fn diagnostic_category_is_derived_from_rule_id() {
        let location = Location::new(SourceFile::Anonymous, 1, 1, 5);
        let diag = Diagnostic::new("sentence-too-long", Severity::Warning, location, "Too long");
        assert_eq!(diag.category(), Category::Structure);
    }

    #[test]
    fn diagnostic_with_section_sets_section() {
        let location = Location::new(SourceFile::Anonymous, 1, 1, 5);
        let diag = Diagnostic::new("sentence-too-long", Severity::Warning, location, "Too long")
            .with_section("Introduction");
        assert_eq!(diag.section.as_deref(), Some("Introduction"));
    }
}
