//! `roadmap` — internal CLI for the `.roadmap/` source-of-truth pipeline.
//!
//! Subcommands are stubs at scaffold time; implementations land per
//! F-roadmap-toml-source action plan (A2 generate, A2.5 add, A2.6 validate).

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "roadmap", version, about = "ROADMAP.md generator from .roadmap/ frontmatter source", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new feature file from a template.
    Add {
        /// Slug, e.g. `f-roadmap-toml-source` (lowercased; `f-` prefix optional).
        slug: String,
    },
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
        Command::Add { slug } => bail!("`add {slug}` not implemented (A2.5)"),
        Command::Generate => bail!("`generate` not implemented (A2)"),
        Command::Validate => bail!("`validate` not implemented (A2.6)"),
        Command::Rename { from, to } => bail!("`rename {from} → {to}` not implemented"),
    }
}
