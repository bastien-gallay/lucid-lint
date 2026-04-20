//! Output formatters for diagnostics.
//!
//! Formats supported in v0.2:
//!
//! - [`tty`]: human-readable, colorized when stdout is a tty
//! - [`json`]: stable JSON schema for CI and tooling integration
//! - [`sarif`]: SARIF v2.1.0 for GitHub Code Scanning and compatible consumers

use serde::{Deserialize, Serialize};

use crate::scoring::Scorecard;
use crate::types::Diagnostic;

pub mod json;
pub mod sarif;
pub mod tty;

/// Output format selector for the CLI and library.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    /// Human-readable terminal output (default).
    #[default]
    Tty,
    /// Structured JSON output.
    Json,
    /// SARIF v2.1.0 log for GitHub Code Scanning.
    Sarif,
}

impl Format {
    /// Render diagnostics + scorecard to a String using this format.
    #[must_use]
    pub fn render(self, diagnostics: &[Diagnostic], scorecard: &Scorecard) -> String {
        match self {
            Self::Tty => tty::render(diagnostics, scorecard, tty::ColorMode::Auto),
            Self::Json => json::render(diagnostics, scorecard),
            Self::Sarif => sarif::render(diagnostics, scorecard),
        }
    }
}
