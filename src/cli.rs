//! Command-line interface for `lucid-lint`.
//!
//! Uses `clap` with the derive API for parsing. The CLI is intentionally small
//! in v0.1: one verb (`check`), a handful of options.
//!
//! This module is compiled only as part of the binary crate. Items are
//! `pub(crate)` because they are consumed by `main.rs` but never re-exported
//! through the library.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use lucid_lint::condition::ConditionTag;
use lucid_lint::config::Profile as ProfileConfig;
use lucid_lint::output::Format as FormatConfig;
use lucid_lint::rules::readability_score::FormulaChoice;

/// Top-level CLI.
#[derive(Debug, Parser)]
#[command(
    name = "lucid-lint",
    version,
    about = "A cognitive accessibility linter for prose.",
    long_about = None,
)]
pub(crate) struct Cli {
    /// Subcommand to run.
    #[command(subcommand)]
    pub(crate) command: Command,
}

/// Subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Check one or more files (or stdin with `-`).
    Check(CheckArgs),
}

/// Arguments for the `check` subcommand.
#[derive(Debug, Parser)]
pub(crate) struct CheckArgs {
    /// Paths to files or directories to lint. Use `-` for stdin.
    ///
    /// At least one path is required.
    #[arg(required = true)]
    pub(crate) paths: Vec<PathBuf>,

    /// Profile preset.
    #[arg(long, value_enum, default_value = "public")]
    pub(crate) profile: CliProfile,

    /// Output format.
    #[arg(long, value_enum, default_value = "tty")]
    pub(crate) format: CliFormat,

    /// Exit with code 1 if any warnings are found.
    #[arg(long, default_value_t = true)]
    pub(crate) fail_on_warning: bool,

    /// Fail (exit 1) if the aggregate document score is below this threshold.
    ///
    /// The maximum is currently 100 (5 categories × 20). See the `scoring`
    /// page in the user guide for the calibration details.
    #[arg(long)]
    pub(crate) min_score: Option<u32>,

    /// Active condition tags (F72). Comma-separated kebab-case names from
    /// the fixed ontology: `a11y-markup`, `dyslexia`, `dyscalculia`,
    /// `aphasia`, `adhd`, `non-native`, `general`. Repeatable.
    ///
    /// Rules tagged `general` are always active; tagged rules without
    /// `general` only run when their tag appears here.
    #[arg(long, value_enum, value_delimiter = ',')]
    pub(crate) conditions: Vec<CliConditionTag>,

    /// Readability formula choice (F11).
    ///
    /// `auto` (default) selects Flesch-Kincaid for EN documents and
    /// Kandel-Moles for FR. `flesch-kincaid` / `kandel-moles` pin a
    /// concrete formula regardless of detected language — useful for
    /// cross-document score comparison.
    #[arg(long, value_enum, default_value = "auto")]
    pub(crate) readability_formula: CliFormulaChoice,
}

/// Formula choice values accepted on the command line.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliFormulaChoice {
    /// Per-language auto-select (F10 behaviour).
    Auto,
    /// Flesch-Kincaid grade level, regardless of language.
    #[value(name = "flesch-kincaid")]
    FleschKincaid,
    /// Kandel-Moles ease score, regardless of language.
    #[value(name = "kandel-moles")]
    KandelMoles,
}

impl From<CliFormulaChoice> for FormulaChoice {
    fn from(value: CliFormulaChoice) -> Self {
        match value {
            CliFormulaChoice::Auto => Self::Auto,
            CliFormulaChoice::FleschKincaid => Self::FleschKincaid,
            CliFormulaChoice::KandelMoles => Self::KandelMoles,
        }
    }
}

/// Condition-tag values accepted on the command line.
///
/// Mirror of [`ConditionTag`] kept inside the binary crate so the library
/// stays independent of `clap`.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliConditionTag {
    /// Prose-adjacent markup signals.
    #[value(name = "a11y-markup")]
    A11yMarkup,
    /// Dyslexia-targeted signals.
    Dyslexia,
    /// Dyscalculia-targeted signals.
    Dyscalculia,
    /// Aphasia-targeted signals.
    Aphasia,
    /// Attention-fragility signals (ADHD).
    Adhd,
    /// Non-native reader signals.
    #[value(name = "non-native")]
    NonNative,
    /// Always-on rules (default tag for v0.2 rules).
    General,
}

impl From<CliConditionTag> for ConditionTag {
    fn from(value: CliConditionTag) -> Self {
        match value {
            CliConditionTag::A11yMarkup => Self::A11yMarkup,
            CliConditionTag::Dyslexia => Self::Dyslexia,
            CliConditionTag::Dyscalculia => Self::Dyscalculia,
            CliConditionTag::Aphasia => Self::Aphasia,
            CliConditionTag::Adhd => Self::Adhd,
            CliConditionTag::NonNative => Self::NonNative,
            CliConditionTag::General => Self::General,
        }
    }
}

/// Profile values accepted on the command line.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliProfile {
    /// Technical documentation, API docs.
    #[value(name = "dev-doc")]
    DevDoc,
    /// General audience content.
    Public,
    /// Easy-to-Read (FALC).
    Falc,
}

impl From<CliProfile> for ProfileConfig {
    fn from(value: CliProfile) -> Self {
        match value {
            CliProfile::DevDoc => Self::DevDoc,
            CliProfile::Public => Self::Public,
            CliProfile::Falc => Self::Falc,
        }
    }
}

/// Output format values accepted on the command line.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliFormat {
    /// Human-readable terminal output.
    Tty,
    /// Structured JSON.
    Json,
    /// SARIF v2.1.0 log for GitHub Code Scanning.
    Sarif,
}

impl From<CliFormat> for FormatConfig {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Tty => Self::Tty,
            CliFormat::Json => Self::Json,
            CliFormat::Sarif => Self::Sarif,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_is_well_formed() {
        Cli::command().debug_assert();
    }

    #[test]
    fn check_args_parse_defaults() {
        let args = Cli::try_parse_from(["lucid-lint", "check", "file.md"]).unwrap();
        match args.command {
            Command::Check(a) => {
                assert_eq!(a.paths.len(), 1);
                assert!(matches!(a.profile, CliProfile::Public));
                assert!(matches!(a.format, CliFormat::Tty));
            },
        }
    }

    #[test]
    fn check_args_parse_profile() {
        let args =
            Cli::try_parse_from(["lucid-lint", "check", "--profile", "falc", "file.md"]).unwrap();
        match args.command {
            Command::Check(a) => assert!(matches!(a.profile, CliProfile::Falc)),
        }
    }

    #[test]
    fn check_args_parse_format() {
        let args =
            Cli::try_parse_from(["lucid-lint", "check", "--format", "json", "file.md"]).unwrap();
        match args.command {
            Command::Check(a) => assert!(matches!(a.format, CliFormat::Json)),
        }
    }

    #[test]
    fn profile_conversion_is_exhaustive() {
        let cases = [
            (CliProfile::DevDoc, ProfileConfig::DevDoc),
            (CliProfile::Public, ProfileConfig::Public),
            (CliProfile::Falc, ProfileConfig::Falc),
        ];
        for (cli, expected) in cases {
            let converted: ProfileConfig = cli.into();
            assert_eq!(converted, expected);
        }
    }
}
