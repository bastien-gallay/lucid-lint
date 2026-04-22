//! Stable JSON output for diagnostics.
//!
//! The schema is designed for CI integration and third-party tooling.
//! Once v1.0 ships, the schema will be frozen; v0.x reserves the right
//! to add (not remove) fields.

use serde::Serialize;

use crate::scoring::{Score, Scorecard};
use crate::types::{Category, Diagnostic};

/// Top-level JSON report.
#[derive(Debug, Serialize)]
pub struct Report<'a> {
    /// Schema version. Bumped on breaking changes.
    pub version: u32,
    /// The diagnostics emitted.
    pub diagnostics: &'a [Diagnostic],
    /// Summary counts by severity.
    pub summary: Summary,
    /// Aggregate document score.
    pub score: Score,
    /// Per-category sub-scores in fixed [`Category::ALL`] order.
    pub category_scores: Vec<CategoryScoreEntry>,
}

/// One entry in the per-category score array.
///
/// Unlike [`crate::scoring::CategoryScore`], this flattens `category` into a
/// string so the JSON is stable under enum renames inside the codebase.
#[derive(Debug, Serialize)]
pub struct CategoryScoreEntry {
    /// Lowercase category name (e.g. `"structure"`).
    pub category: String,
    /// Current value, clamped to `max`.
    pub value: u32,
    /// Maximum achievable value.
    pub max: u32,
}

impl CategoryScoreEntry {
    fn from_scorecard(scorecard: &Scorecard) -> Vec<Self> {
        scorecard
            .per_category
            .iter()
            .map(|cs| Self {
                category: category_name(cs.category).to_string(),
                value: cs.score.value,
                max: cs.score.max,
            })
            .collect()
    }
}

const fn category_name(c: Category) -> &'static str {
    match c {
        Category::Structure => "structure",
        Category::Rhythm => "rhythm",
        Category::Lexicon => "lexicon",
        Category::Syntax => "syntax",
        Category::Readability => "readability",
    }
}

/// Counts by severity.
#[derive(Debug, Default, Serialize)]
pub struct Summary {
    /// Number of `info` diagnostics.
    pub info: usize,
    /// Number of `warning` diagnostics.
    pub warning: usize,
    /// Number of `error` diagnostics.
    pub error: usize,
    /// Total number of diagnostics.
    pub total: usize,
}

impl Summary {
    fn from_diagnostics(diagnostics: &[Diagnostic]) -> Self {
        use crate::types::Severity;
        let mut s = Self::default();
        for d in diagnostics {
            match d.severity {
                Severity::Info => s.info += 1,
                Severity::Warning => s.warning += 1,
                Severity::Error => s.error += 1,
            }
        }
        s.total = diagnostics.len();
        s
    }
}

/// Schema version. Bumped on breaking output changes.
///
/// v2 (0.2.0): adds `score`, `category_scores`, and per-diagnostic `weight`;
/// renames `category` values (`length` → `structure`, `lexical` → `lexicon`,
/// `style` → `syntax`, `global` → `readability`).
pub const SCHEMA_VERSION: u32 = 2;

/// Render diagnostics + scorecard as pretty-printed JSON.
///
/// Falls back to an empty object on serialization failure (should not happen
/// with the built-in types, but we avoid panicking regardless).
#[must_use]
pub fn render(diagnostics: &[Diagnostic], scorecard: &Scorecard) -> String {
    let report = Report {
        version: SCHEMA_VERSION,
        diagnostics,
        summary: Summary::from_diagnostics(diagnostics),
        score: scorecard.global,
        category_scores: CategoryScoreEntry::from_scorecard(scorecard),
    };
    serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::{self, ScoringConfig};
    use crate::types::{Location, Severity, SourceFile};

    fn sample_diag() -> Diagnostic {
        Diagnostic::new(
            "structure.sentence-too-long",
            Severity::Warning,
            Location::new(SourceFile::Anonymous, 3, 1, 42),
            "Sentence is too long.",
        )
    }

    fn scorecard(diags: &[Diagnostic]) -> Scorecard {
        scoring::compute(diags, 1000, &ScoringConfig::default())
    }

    #[test]
    fn render_is_valid_json() {
        let diag = sample_diag();
        let card = scorecard(std::slice::from_ref(&diag));
        let json = render(&[diag], &card);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
        assert_eq!(parsed["version"], SCHEMA_VERSION);
    }

    #[test]
    fn render_includes_summary() {
        let diag = sample_diag();
        let card = scorecard(std::slice::from_ref(&diag));
        let json = render(&[diag], &card);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["summary"]["warning"], 1);
        assert_eq!(parsed["summary"]["info"], 0);
        assert_eq!(parsed["summary"]["total"], 1);
    }

    #[test]
    fn render_includes_score_and_categories() {
        let diag = sample_diag();
        let card = scorecard(std::slice::from_ref(&diag));
        let json = render(&[diag], &card);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["score"]["value"].as_u64().is_some());
        assert_eq!(parsed["score"]["max"], 100);
        let cats = parsed["category_scores"].as_array().unwrap();
        assert_eq!(cats.len(), 5);
        assert_eq!(cats[0]["category"], "structure");
        assert_eq!(cats[4]["category"], "readability");
    }

    #[test]
    fn render_diagnostics_carry_weight() {
        let diag = sample_diag();
        let card = scorecard(std::slice::from_ref(&diag));
        let json = render(&[diag], &card);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["diagnostics"][0]["weight"], 2);
    }

    #[test]
    fn render_empty_diagnostics() {
        let card = scorecard(&[]);
        let json = render(&[], &card);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["summary"]["total"], 0);
        assert!(parsed["diagnostics"].as_array().unwrap().is_empty());
    }

    #[test]
    fn summary_counts_by_severity() {
        let diagnostics = vec![
            Diagnostic::new(
                "a",
                Severity::Info,
                Location::new(SourceFile::Anonymous, 1, 1, 1),
                "m1",
            ),
            Diagnostic::new(
                "a",
                Severity::Warning,
                Location::new(SourceFile::Anonymous, 1, 1, 1),
                "m2",
            ),
            Diagnostic::new(
                "a",
                Severity::Warning,
                Location::new(SourceFile::Anonymous, 1, 1, 1),
                "m3",
            ),
        ];
        let summary = Summary::from_diagnostics(&diagnostics);
        assert_eq!(summary.info, 1);
        assert_eq!(summary.warning, 2);
        assert_eq!(summary.total, 3);
    }
}
