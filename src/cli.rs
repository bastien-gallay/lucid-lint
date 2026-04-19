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

use lucid_lint::config::Profile as ProfileConfig;
use lucid_lint::output::Format as FormatConfig;

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
}

impl From<CliFormat> for FormatConfig {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Tty => Self::Tty,
            CliFormat::Json => Self::Json,
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
            }
        }
    }

    #[test]
    fn check_args_parse_profile() {
        let args = Cli::try_parse_from([
            "lucid-lint", "check", "--profile", "falc", "file.md",
        ])
        .unwrap();
        match args.command {
            Command::Check(a) => assert!(matches!(a.profile, CliProfile::Falc)),
        }
    }

    #[test]
    fn check_args_parse_format() {
        let args = Cli::try_parse_from([
            "lucid-lint", "check", "--format", "json", "file.md",
        ])
        .unwrap();
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
