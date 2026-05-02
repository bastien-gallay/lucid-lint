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
use lucid_lint::output::tty::BannerPolicy as TtyBannerPolicy;
use lucid_lint::output::Format as FormatConfig;
use lucid_lint::rules::readability::score::FormulaChoice;

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
    /// Print the bundled documentation for one or more rule ids.
    Explain(ExplainArgs),
}

/// Arguments for the `explain` subcommand.
#[derive(Debug, Parser)]
pub(crate) struct ExplainArgs {
    /// One or more rule ids (e.g. `sentence-too-long weasel-words`).
    ///
    /// When omitted, pair with `--list` or `--list-verbose` to print the
    /// known ids instead.
    pub(crate) rule_ids: Vec<String>,

    /// Print the list of known rule ids, one per line, and exit.
    ///
    /// Grep-friendly. Mutually exclusive with `--list-verbose`.
    #[arg(long, default_value_t = false, conflicts_with = "list_verbose")]
    pub(crate) list: bool,

    /// Print each known rule id with a one-line description and exit.
    ///
    /// Descriptions are extracted from the `## What it flags` section of
    /// each bundled rule page.
    #[arg(long, default_value_t = false)]
    pub(crate) list_verbose: bool,

    /// Keep relative markdown links as-is instead of rewriting them to
    /// absolute mdBook URLs.
    ///
    /// Useful when copy-pasting the output into another mdBook or into
    /// an AI agent's local rule inventory.
    #[arg(long, default_value_t = false)]
    pub(crate) keep_relative: bool,
}

/// Arguments for the `check` subcommand.
#[derive(Debug, Parser)]
#[allow(clippy::struct_excessive_bools)] // CLI flag record — one bool per user-facing flag is idiomatic here.
pub(crate) struct CheckArgs {
    /// Paths to files or directories to lint. Use `-` for stdin.
    ///
    /// At least one path is required.
    #[arg(required = true)]
    pub(crate) paths: Vec<PathBuf>,

    /// Profile preset.
    ///
    /// When omitted, the profile is read from `[default].profile` in
    /// the discovered `lucid-lint.toml`, or falls back to `public`.
    #[arg(long, value_enum)]
    pub(crate) profile: Option<CliProfile>,

    /// Path to a `lucid-lint.toml` config file.
    ///
    /// When omitted, the loader walks up from the current directory to
    /// the first `lucid-lint.toml` it finds, stopping at the nearest
    /// `.git` repo boundary.
    #[arg(long, value_name = "PATH")]
    pub(crate) config: Option<PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value = "tty")]
    pub(crate) format: CliFormat,

    /// Disable per-rule grouping in the `tty` format.
    ///
    /// By default, when the same rule fires two or more times on the same
    /// file, hits are clustered under a single rule header. `--no-group`
    /// keeps every diagnostic on its own line. No effect on `json` / `sarif`.
    #[arg(long, default_value_t = false)]
    pub(crate) no_group: bool,

    /// Suppress the one-line `explain` hint that follows the summary.
    ///
    /// Useful for CI log hygiene. No effect on `json` / `sarif`.
    #[arg(long, default_value_t = false)]
    pub(crate) no_explain_hint: bool,

    /// Emit the score block before the diagnostics in the `tty` format.
    ///
    /// Default places the score at the end (clippy / ruff family
    /// convention). `--score-first` is for CI logs and agents that want
    /// the headline up top. No effect on `json` / `sarif`.
    #[arg(long, default_value_t = false)]
    pub(crate) score_first: bool,

    /// When to print the wordmark banner in the `tty` format.
    ///
    /// `auto` (default) prints it when stdout is a TTY — interactive
    /// runs only, silent under CI / pipes / redirects. `always` and
    /// `never` override. No effect on `json` / `sarif`.
    #[arg(long, value_enum, default_value = "auto")]
    pub(crate) banner: CliBannerPolicy,

    /// Exit with code 1 if any warnings are found (default: `true`).
    ///
    /// Accepts `--fail-on-warning`, `--fail-on-warning=true`, or
    /// `--fail-on-warning=false`. The mirror `--no-fail-on-warning`
    /// is equivalent to `--fail-on-warning=false`. Disabling is
    /// useful when the CI gate should depend purely on `--min-score`
    /// — set the score floor, let the warning count inform the
    /// review, don't fail the build on warnings alone (F80). If both
    /// forms are passed on the same invocation, `--no-fail-on-warning`
    /// wins.
    #[arg(
        long,
        default_value_t = true,
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "true",
        value_parser = clap::builder::BoolishValueParser::new(),
    )]
    pub(crate) fail_on_warning: bool,

    /// Disable-side mirror of `--fail-on-warning` (F80). Hidden from
    /// `--help` because it is documented on the main flag.
    #[arg(long = "no-fail-on-warning", hide = true)]
    pub(crate) no_fail_on_warning: bool,

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

    /// Glob patterns of paths to skip at discovery time (F78).
    ///
    /// Comma-separated and repeatable. Merged with `[default].exclude`
    /// from `lucid-lint.toml`; CLI and TOML are additive. Matching
    /// directories are pruned (not descended into) during recursion.
    ///
    /// Examples: `--exclude 'vendor/**'`, `--exclude '**/fixtures/**,CHANGELOG.md'`.
    #[arg(long, value_delimiter = ',', value_name = "GLOB")]
    pub(crate) exclude: Vec<String>,

    /// Opt in to one or more [`Status::Experimental`] rules (F139).
    ///
    /// Repeatable and comma-separated. Pass a rule id (e.g.
    /// `--experimental structure.italic-span-long`) to enable a single
    /// experimental rule, or the literal `*` (e.g.
    /// `--experimental '*'`) to enable every experimental rule at once.
    /// Merged with `[experimental].enabled` from `lucid-lint.toml`;
    /// CLI and TOML are additive (a `*` on either side wins).
    ///
    /// Experimental rules are part of the registry but skipped by
    /// default — they ship for dogfooding while they stabilize, then
    /// promote to `Stable` in a future release.
    #[arg(long, value_delimiter = ',', value_name = "RULE_ID")]
    pub(crate) experimental: Vec<String>,

    /// Readability formula choice (F11).
    ///
    /// `auto` (default) selects Flesch-Kincaid for EN documents and
    /// Kandel-Moles for FR. `flesch-kincaid` / `kandel-moles` pin a
    /// concrete formula regardless of detected language — useful for
    /// cross-document score comparison. When omitted, falls back to
    /// `[rules.readability-score].formula` in `lucid-lint.toml`, then
    /// to `auto`.
    #[arg(long, value_enum)]
    pub(crate) readability_formula: Option<CliFormulaChoice>,
}

/// Banner policy values accepted on the command line.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliBannerPolicy {
    /// Print the banner only when the run has no diagnostics.
    Auto,
    /// Print the banner on every run.
    Always,
    /// Never print the banner.
    Never,
}

impl From<CliBannerPolicy> for TtyBannerPolicy {
    fn from(value: CliBannerPolicy) -> Self {
        match value {
            CliBannerPolicy::Auto => Self::Auto,
            CliBannerPolicy::Always => Self::Always,
            CliBannerPolicy::Never => Self::Never,
        }
    }
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
                assert!(
                    a.profile.is_none(),
                    "unspecified --profile must be None so main.rs can consult TOML"
                );
                assert!(a.config.is_none());
                assert!(a.readability_formula.is_none());
                assert!(matches!(a.format, CliFormat::Tty));
            },
            Command::Explain(_) => unreachable!("expected Check, got Explain"),
        }
    }

    #[test]
    fn check_args_parse_profile() {
        let args =
            Cli::try_parse_from(["lucid-lint", "check", "--profile", "falc", "file.md"]).unwrap();
        match args.command {
            Command::Check(a) => assert!(matches!(a.profile, Some(CliProfile::Falc))),
            Command::Explain(_) => unreachable!("expected Check, got Explain"),
        }
    }

    #[test]
    fn check_args_parse_format() {
        let args =
            Cli::try_parse_from(["lucid-lint", "check", "--format", "json", "file.md"]).unwrap();
        match args.command {
            Command::Check(a) => assert!(matches!(a.format, CliFormat::Json)),
            Command::Explain(_) => unreachable!("expected Check, got Explain"),
        }
    }

    #[test]
    fn check_args_parse_exclude_list() {
        let args = Cli::try_parse_from([
            "lucid-lint",
            "check",
            "--exclude",
            "vendor/**,CHANGELOG.md",
            "--exclude",
            "tests/fixtures/**",
            "docs",
        ])
        .unwrap();
        match args.command {
            Command::Check(a) => assert_eq!(
                a.exclude,
                vec![
                    "vendor/**".to_string(),
                    "CHANGELOG.md".to_string(),
                    "tests/fixtures/**".to_string(),
                ]
            ),
            Command::Explain(_) => unreachable!("expected Check, got Explain"),
        }
    }

    #[test]
    fn experimental_flag_collects_repeats_and_csv() {
        let args = Cli::try_parse_from([
            "lucid-lint",
            "check",
            "--experimental",
            "structure.italic-span-long,structure.number-run",
            "--experimental",
            "syntax.parenthetical-depth",
            "file.md",
        ])
        .unwrap();
        let Command::Check(a) = args.command else {
            unreachable!()
        };
        assert_eq!(
            a.experimental,
            vec![
                "structure.italic-span-long".to_string(),
                "structure.number-run".to_string(),
                "syntax.parenthetical-depth".to_string(),
            ]
        );
    }

    #[test]
    fn experimental_flag_accepts_wildcard() {
        let args =
            Cli::try_parse_from(["lucid-lint", "check", "--experimental", "*", "file.md"]).unwrap();
        let Command::Check(a) = args.command else {
            unreachable!()
        };
        assert_eq!(a.experimental, vec!["*".to_string()]);
    }

    #[test]
    fn experimental_flag_defaults_empty() {
        let args = Cli::try_parse_from(["lucid-lint", "check", "file.md"]).unwrap();
        let Command::Check(a) = args.command else {
            unreachable!()
        };
        assert!(a.experimental.is_empty());
    }

    #[test]
    fn fail_on_warning_flag_forms_round_trip() {
        // Bare flag: defaults to true.
        let a = Cli::try_parse_from(["lucid-lint", "check", "file.md"])
            .unwrap()
            .command;
        let Command::Check(a) = a else { unreachable!() };
        assert!(a.fail_on_warning);
        assert!(!a.no_fail_on_warning);

        // `--fail-on-warning=false` disables via the main flag (F80).
        let a = Cli::try_parse_from(["lucid-lint", "check", "--fail-on-warning=false", "file.md"])
            .unwrap()
            .command;
        let Command::Check(a) = a else { unreachable!() };
        assert!(!a.fail_on_warning);

        // `--no-fail-on-warning` disables via the mirror flag (F80).
        let a = Cli::try_parse_from(["lucid-lint", "check", "--no-fail-on-warning", "file.md"])
            .unwrap()
            .command;
        let Command::Check(a) = a else { unreachable!() };
        assert!(a.no_fail_on_warning);
    }

    #[test]
    fn explain_args_parse_ids() {
        let args = Cli::try_parse_from([
            "lucid-lint",
            "explain",
            "structure.sentence-too-long",
            "lexicon.weasel-words",
        ])
        .unwrap();
        match args.command {
            Command::Explain(a) => {
                assert_eq!(
                    a.rule_ids,
                    vec!["structure.sentence-too-long", "lexicon.weasel-words"]
                );
                assert!(!a.list);
            },
            Command::Check(_) => unreachable!("expected Explain, got Check"),
        }
    }

    #[test]
    fn explain_args_allow_list_without_ids() {
        let args = Cli::try_parse_from(["lucid-lint", "explain", "--list"]).unwrap();
        match args.command {
            Command::Explain(a) => {
                assert!(a.list);
                assert!(a.rule_ids.is_empty());
            },
            Command::Check(_) => unreachable!("expected Explain, got Check"),
        }
    }

    #[test]
    fn explain_args_parse_without_ids_or_flags() {
        // Bare `explain` parses successfully; main.rs is responsible for
        // printing a friendly hint and exiting non-zero.
        let args = Cli::try_parse_from(["lucid-lint", "explain"]).unwrap();
        match args.command {
            Command::Explain(a) => {
                assert!(a.rule_ids.is_empty());
                assert!(!a.list);
                assert!(!a.list_verbose);
            },
            Command::Check(_) => unreachable!("expected Explain"),
        }
    }

    #[test]
    fn explain_args_list_flags_are_mutually_exclusive() {
        assert!(
            Cli::try_parse_from(["lucid-lint", "explain", "--list", "--list-verbose"]).is_err()
        );
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
