//! `roadmap` — internal CLI for the `.roadmap/` source-of-truth pipeline.
//!
//! Subcommand status:
//! - `generate`: A2 (minimal first cut)
//! - `validate`: A2.6
//! - `add`: A2.5 (this commit)
//! - `rename`: stub

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "roadmap",
    version,
    about = "ROADMAP.md generator from .roadmap/ frontmatter source"
)]
struct Cli {
    /// Path to the `.roadmap/` directory. Defaults to `./.roadmap`.
    #[arg(long, global = true, default_value = ".roadmap")]
    root: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new feature file from a template.
    Add {
        /// Slug for the new feature (matches the filename without `.md`).
        /// Must be `f-<kebab-name>`. The legacy `f<digits>` form is
        /// rejected unless `--allow-legacy-numeric` is set.
        slug: String,
        /// Allow the legacy `f<digits>` slug shape (e.g. `f139`).
        /// Migration-only — emits a deprecation warning.
        #[arg(long)]
        allow_legacy_numeric: bool,
    },
    /// Generate ROADMAP.md from `.roadmap/` source. Writes to stdout.
    Generate,
    /// Validate the `.roadmap/` source: schema, slug uniqueness, anchor drift.
    Validate {
        /// Path to the on-disk `ROADMAP.md` to diff anchors against.
        #[arg(long, default_value = "ROADMAP.md")]
        roadmap_md: PathBuf,
        /// Treat anchor drift as a warning instead of a failure.
        /// Schema errors and slug collisions still fail the run.
        #[arg(long)]
        accept_drift: bool,
    },
    /// Rename a feature slug, rewriting cross-links.
    Rename { from: String, to: String },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(2)
        },
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        Command::Generate => {
            generate(&cli.root)?;
            Ok(ExitCode::SUCCESS)
        },
        Command::Validate {
            roadmap_md,
            accept_drift,
        } => validate_cmd(&cli.root, &roadmap_md, accept_drift),
        Command::Add {
            slug,
            allow_legacy_numeric,
        } => add_cmd(&cli.root, &slug, allow_legacy_numeric),
        Command::Rename { from, to } => bail!("`rename {from} → {to}` not implemented"),
    }
}

fn generate(root: &std::path::Path) -> Result<()> {
    let config = roadmap_cli::load_config(root).context("loading config.toml")?;
    let mut features = roadmap_cli::load_features(root).context("loading features/")?;
    roadmap_cli::sort_features(&mut features, &config);
    print!("{}", roadmap_cli::render(&features));
    Ok(())
}

fn add_cmd(root: &std::path::Path, slug: &str, allow_legacy_numeric: bool) -> Result<ExitCode> {
    let outcome = roadmap_cli::add::add(root, slug, allow_legacy_numeric)?;
    if outcome.legacy_numeric_warning {
        eprintln!(
            "warning: `{slug}` uses the legacy `f<digits>` slug shape — \
             deprecated, only intended for one-shot migration. New \
             features should use `f-<kebab-name>`."
        );
    }
    println!("created {}", outcome.path.display());
    Ok(ExitCode::SUCCESS)
}

fn validate_cmd(
    root: &std::path::Path,
    roadmap_md: &std::path::Path,
    accept_drift: bool,
) -> Result<ExitCode> {
    let report = roadmap_cli::validate::validate(root, roadmap_md)?;
    print!("{}", report.to_text());
    if report.has_hard_errors() {
        return Ok(ExitCode::FAILURE);
    }
    if report.has_drift() && !accept_drift {
        return Ok(ExitCode::FAILURE);
    }
    Ok(ExitCode::SUCCESS)
}
