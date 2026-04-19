//! Human-readable terminal output with optional colors.
//!
//! Colors are applied only when stdout is a tty, unless overridden by
//! [`ColorMode::Always`] or [`ColorMode::Never`].

use std::fmt::Write;

use owo_colors::{OwoColorize, Stream};

use crate::types::{Diagnostic, Severity};

/// Controls whether ANSI color codes are emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Use colors when stdout is a tty.
    Auto,
    /// Always emit color codes.
    Always,
    /// Never emit color codes.
    Never,
}

/// Render a list of diagnostics as human-readable output.
#[must_use]
pub fn render(diagnostics: &[Diagnostic], color_mode: ColorMode) -> String {
    let mut out = String::new();

    if diagnostics.is_empty() {
        let _ = writeln!(out, "{}", green("No issues found.", color_mode));
        return out;
    }

    for diag in diagnostics {
        let _ = write!(out, "{}", format_diagnostic(diag, color_mode));
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "{}", summary(diagnostics, color_mode));

    out
}

fn format_diagnostic(diag: &Diagnostic, color_mode: ColorMode) -> String {
    let mut out = String::new();
    let severity_str = severity_label(diag.severity, color_mode);
    let location = diag.location.to_string();
    let section_suffix = diag
        .section
        .as_deref()
        .map(|s| format!(" [section: {s}]"))
        .unwrap_or_default();

    let _ = writeln!(
        out,
        "{} {} {}{}",
        severity_str,
        bold(&location, color_mode),
        diag.message,
        dim(&section_suffix, color_mode),
    );
    let _ = writeln!(out, "  {} {}", dim("rule:", color_mode), diag.rule_id);

    out
}

fn summary(diagnostics: &[Diagnostic], color_mode: ColorMode) -> String {
    let info_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Info)
        .count();
    let warn_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    let error_count = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();

    let mut parts = Vec::new();
    if error_count > 0 {
        parts.push(format!(
            "{} errors",
            red(&error_count.to_string(), color_mode)
        ));
    }
    if warn_count > 0 {
        parts.push(format!(
            "{} warnings",
            yellow(&warn_count.to_string(), color_mode)
        ));
    }
    if info_count > 0 {
        parts.push(format!(
            "{} info",
            cyan(&info_count.to_string(), color_mode)
        ));
    }

    if parts.is_empty() {
        "No issues found.".to_string()
    } else {
        format!("Summary: {}.", parts.join(", "))
    }
}

fn severity_label(severity: Severity, color_mode: ColorMode) -> String {
    match severity {
        Severity::Info => cyan("info", color_mode),
        Severity::Warning => yellow("warning", color_mode),
        Severity::Error => red("error", color_mode),
    }
}

fn should_color(color_mode: ColorMode) -> bool {
    match color_mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => supports_color(),
    }
}

#[cfg(not(test))]
fn supports_color() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}

// Disable colors in tests for deterministic snapshot output.
#[cfg(test)]
const fn supports_color() -> bool {
    false
}

fn green(s: &str, mode: ColorMode) -> String {
    if should_color(mode) {
        s.if_supports_color(Stream::Stdout, OwoColorize::green)
            .to_string()
    } else {
        s.to_string()
    }
}

fn red(s: &str, mode: ColorMode) -> String {
    if should_color(mode) {
        s.if_supports_color(Stream::Stdout, OwoColorize::red)
            .to_string()
    } else {
        s.to_string()
    }
}

fn yellow(s: &str, mode: ColorMode) -> String {
    if should_color(mode) {
        s.if_supports_color(Stream::Stdout, OwoColorize::yellow)
            .to_string()
    } else {
        s.to_string()
    }
}

fn cyan(s: &str, mode: ColorMode) -> String {
    if should_color(mode) {
        s.if_supports_color(Stream::Stdout, OwoColorize::cyan)
            .to_string()
    } else {
        s.to_string()
    }
}

fn bold(s: &str, mode: ColorMode) -> String {
    if should_color(mode) {
        s.if_supports_color(Stream::Stdout, OwoColorize::bold)
            .to_string()
    } else {
        s.to_string()
    }
}

fn dim(s: &str, mode: ColorMode) -> String {
    if should_color(mode) {
        s.if_supports_color(Stream::Stdout, OwoColorize::dimmed)
            .to_string()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Location, SourceFile};

    fn sample_diag() -> Diagnostic {
        Diagnostic::new(
            "sentence-too-long",
            Severity::Warning,
            Location::new(SourceFile::Anonymous, 3, 1, 42),
            "Sentence is 25 words long (maximum 22).",
        )
    }

    #[test]
    fn render_empty_says_no_issues() {
        let out = render(&[], ColorMode::Never);
        assert!(out.contains("No issues found"));
    }

    #[test]
    fn render_contains_severity_and_message() {
        let diag = sample_diag();
        let out = render(&[diag], ColorMode::Never);
        assert!(out.contains("warning"));
        assert!(out.contains("Sentence is 25 words long"));
        assert!(out.contains("sentence-too-long"));
    }

    #[test]
    fn render_includes_summary_counts() {
        let diag = sample_diag();
        let out = render(&[diag], ColorMode::Never);
        assert!(out.contains("Summary:"));
        assert!(out.contains("1 warnings"));
    }

    #[test]
    fn render_includes_section_when_present() {
        let diag = sample_diag().with_section("Introduction");
        let out = render(&[diag], ColorMode::Never);
        assert!(out.contains("section: Introduction"));
    }
}
