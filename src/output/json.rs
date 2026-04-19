//! Stable JSON output for diagnostics.
//!
//! The schema is designed for CI integration and third-party tooling.
//! Once v1.0 ships, the schema will be frozen; v0.x reserves the right
//! to add (not remove) fields.

use serde::Serialize;

use crate::types::Diagnostic;

/// Top-level JSON report.
#[derive(Debug, Serialize)]
pub struct Report<'a> {
    /// Schema version. Bumped on breaking changes.
    pub version: u32,
    /// The diagnostics emitted.
    pub diagnostics: &'a [Diagnostic],
    /// Summary counts by severity.
    pub summary: Summary,
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
pub const SCHEMA_VERSION: u32 = 1;

/// Render diagnostics as pretty-printed JSON.
///
/// Falls back to an empty array on serialization failure (should not happen
/// with the built-in `Diagnostic` type, but we avoid panicking regardless).
#[must_use]
pub fn render(diagnostics: &[Diagnostic]) -> String {
    let report = Report {
        version: SCHEMA_VERSION,
        diagnostics,
        summary: Summary::from_diagnostics(diagnostics),
    };
    serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Location, Severity, SourceFile};

    fn sample_diag() -> Diagnostic {
        Diagnostic::new(
            "sentence-too-long",
            Severity::Warning,
            Location::new(SourceFile::Anonymous, 3, 1, 42),
            "Sentence is too long.",
        )
    }

    #[test]
    fn render_is_valid_json() {
        let diag = sample_diag();
        let json = render(&[diag]);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
        assert_eq!(parsed["version"], SCHEMA_VERSION);
    }

    #[test]
    fn render_includes_summary() {
        let diag = sample_diag();
        let json = render(&[diag]);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["summary"]["warning"], 1);
        assert_eq!(parsed["summary"]["info"], 0);
        assert_eq!(parsed["summary"]["total"], 1);
    }

    #[test]
    fn render_empty_diagnostics() {
        let json = render(&[]);
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
