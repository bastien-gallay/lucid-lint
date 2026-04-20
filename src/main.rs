//! `lucid-lint` command-line binary entry point.

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;

use lucid_lint::config::Profile;
use lucid_lint::output::Format;
use lucid_lint::scoring::{self, ScoringConfig};
use lucid_lint::{Diagnostic, Engine, Severity};

mod cli;

use cli::{CheckArgs, Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Check(args) => match run_check(args) {
            Ok(exit_code) => exit_code,
            Err(err) => {
                eprintln!("error: {err:#}");
                ExitCode::from(2)
            },
        },
    }
}

fn run_check(args: CheckArgs) -> Result<ExitCode> {
    let profile: Profile = args.profile.into();
    let format: Format = args.format.into();
    let engine = Engine::with_profile(profile);
    let scoring_config = ScoringConfig::default();

    let mut all_diagnostics: Vec<Diagnostic> = Vec::new();
    let mut total_words: u32 = 0;

    for raw_path in &args.paths {
        if is_stdin_marker(raw_path) {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .context("failed to read stdin")?;
            let report = engine.lint_stdin(&input);
            total_words = total_words.saturating_add(report.word_count);
            all_diagnostics.extend(report.diagnostics);
        } else {
            let files = collect_files(raw_path)?;
            for file in files {
                let report = engine
                    .lint_file(&file)
                    .with_context(|| format!("failed to lint {}", file.display()))?;
                total_words = total_words.saturating_add(report.word_count);
                all_diagnostics.extend(report.diagnostics);
            }
        }
    }

    // Aggregate across all inputs as a single document for v0.2 scoring.
    // Per-file and project-level roll-ups are tracked as F15 (ROADMAP).
    let scorecard = scoring::compute(&all_diagnostics, total_words, &scoring_config);

    let rendered = format.render(&all_diagnostics, &scorecard);
    io::stdout()
        .write_all(rendered.as_bytes())
        .context("failed to write output")?;

    let has_warning_or_above = all_diagnostics
        .iter()
        .any(|d| matches!(d.severity, Severity::Warning | Severity::Error));

    let severity_fail = args.fail_on_warning && has_warning_or_above;
    let score_fail = args
        .min_score
        .is_some_and(|min| scorecard.global.value < min);

    if severity_fail || score_fail {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn is_stdin_marker(path: &Path) -> bool {
    path.as_os_str() == "-"
}

fn collect_files(path: &Path) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }
    if path.is_dir() {
        let mut out = Vec::new();
        collect_files_recursive(path, &mut out)?;
        return Ok(out);
    }
    anyhow::bail!("path does not exist: {}", path.display())
}

fn collect_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("failed to read directory {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(&path, out)?;
        } else if is_lintable(&path) {
            out.push(path);
        }
    }
    Ok(())
}

fn is_lintable(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| matches!(ext, "md" | "markdown" | "txt"))
}
