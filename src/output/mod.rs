//! Output formatters for diagnostics.
//!
//! Two formats are supported in v0.1:
//!
//! - [`tty`]: human-readable, colorized when stdout is a tty
//! - [`json`]: stable JSON schema for CI and tooling integration

use serde::{Deserialize, Serialize};

use crate::types::Diagnostic;

pub mod json;
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
}

impl Format {
    /// Render a list of diagnostics to a String using this format.
    #[must_use]
    pub fn render(self, diagnostics: &[Diagnostic]) -> String {
        match self {
            Self::Tty => tty::render(diagnostics, tty::ColorMode::Auto),
            Self::Json => json::render(diagnostics),
        }
    }
}
