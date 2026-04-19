//! Core domain types used across the crate.
//!
//! These types form the stable vocabulary for diagnostics, severities, locations,
//! and other concepts shared by multiple modules.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A single lint finding emitted by a [`Rule`](crate::rules::Rule).
///
/// The struct is intentionally minimal. See `ROADMAP.md` for the v0.2 scoring model
/// that will extend this with `weight`, `category`, and richer suggestions.
///
/// Note: [`Category`] is not stored here because it is derivable from `rule_id`.
/// Use [`Category::for_rule`] when needed.
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
}

impl Diagnostic {
    /// Create a new diagnostic with no section attached.
    #[must_use]
    pub fn new(
        rule_id: impl Into<String>,
        severity: Severity,
        location: Location,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity,
            location,
            section: None,
            message: message.into(),
        }
    }

    /// Attach a section label (typically a heading title) to this diagnostic.
    #[must_use]
    pub fn with_section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
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
/// Categories are stable enum variants used for filtering and grouping.
/// They are derived from `rule_id`, not stored on [`Diagnostic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Absolute length of sentences and paragraphs.
    Length,
    /// Syntactic and document structure.
    Structure,
    /// Patterns and cadence across multiple sentences.
    Rhythm,
    /// Vocabulary, terminology, acronyms.
    Lexical,
    /// Writing style and clarity.
    Style,
    /// Document-level metrics.
    Global,
}

impl Category {
    /// Map a rule id to its category.
    ///
    /// Unknown rule ids fall back to [`Category::Style`].
    #[must_use]
    pub fn for_rule(rule_id: &str) -> Self {
        match rule_id {
            "sentence-too-long" | "paragraph-too-long" => Self::Length,
            "excessive-commas" | "long-enumeration" | "deep-subordination"
            | "deeply-nested-lists" | "heading-jump" => Self::Structure,
            "consecutive-long-sentences" => Self::Rhythm,
            "low-lexical-diversity" | "excessive-nominalization"
            | "unexplained-abbreviation" | "weasel-words" | "jargon-undefined" => Self::Lexical,
            "passive-voice" | "repetitive-connectors" | "unclear-antecedent" => Self::Style,
            "readability-score" => Self::Global,
            _ => Self::Style,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length => f.write_str("length"),
            Self::Structure => f.write_str("structure"),
            Self::Rhythm => f.write_str("rhythm"),
            Self::Lexical => f.write_str("lexical"),
            Self::Style => f.write_str("style"),
            Self::Global => f.write_str("global"),
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
    pub fn new(file: SourceFile, line: u32, column: u32, length: u32) -> Self {
        Self { file, line, column, length }
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
#[serde(tag = "kind", rename_all = "lowercase")]
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
        assert_eq!(Category::for_rule("sentence-too-long"), Category::Length);
        assert_eq!(Category::for_rule("excessive-commas"), Category::Structure);
        assert_eq!(Category::for_rule("consecutive-long-sentences"), Category::Rhythm);
        assert_eq!(Category::for_rule("weasel-words"), Category::Lexical);
        assert_eq!(Category::for_rule("passive-voice"), Category::Style);
        assert_eq!(Category::for_rule("readability-score"), Category::Global);
    }

    #[test]
    fn category_for_unknown_rule_defaults_to_style() {
        assert_eq!(Category::for_rule("no-such-rule"), Category::Style);
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
        assert_eq!(diag.category(), Category::Length);
    }

    #[test]
    fn diagnostic_with_section_sets_section() {
        let location = Location::new(SourceFile::Anonymous, 1, 1, 5);
        let diag = Diagnostic::new("sentence-too-long", Severity::Warning, location, "Too long")
            .with_section("Introduction");
        assert_eq!(diag.section.as_deref(), Some("Introduction"));
    }
}
