//! `roadmap` — internal CLI for the `.roadmap/` source-of-truth pipeline.
//!
//! Subcommand status:
//! - `generate`: A2 (this commit, minimal first cut)
//! - `add`, `validate`, `rename`: stubs (A2.5, A2.6 follow)

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Add { slug: String },
    /// Generate ROADMAP.md from `.roadmap/` source. Writes to stdout.
    Generate,
    /// Validate the `.roadmap/` source: schema, slug uniqueness, anchor diff.
    Validate,
    /// Rename a feature slug, rewriting cross-links.
    Rename { from: String, to: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Generate => generate(&cli.root),
        Command::Add { slug } => bail!("`add {slug}` not implemented (A2.5)"),
        Command::Validate => bail!("`validate` not implemented (A2.6)"),
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
