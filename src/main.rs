//! `lucid-lint` command-line binary entry point.

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;
use globset::{Glob, GlobSet, GlobSetBuilder};

use lucid_lint::condition::ConditionTag;
use lucid_lint::config::{Config as FileConfig, Profile};
use lucid_lint::explain;
use lucid_lint::output::{tty, Format};
use lucid_lint::rules::readability::score::FormulaChoice;
use lucid_lint::scoring::{self, ScoringConfig};
use lucid_lint::{Diagnostic, Engine, Severity};

mod cli;

use cli::{CheckArgs, Cli, Command, ExplainArgs};

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
        Command::Explain(args) => run_explain(args),
    }
}

fn run_explain(args: ExplainArgs) -> ExitCode {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if args.list {
        for id in explain::known_ids() {
            let _ = writeln!(handle, "{id}");
        }
        return ExitCode::SUCCESS;
    }

    if args.list_verbose {
        let entries = explain::known_ids_with_descriptions(90);
        let width = entries.iter().map(|(id, _)| id.len()).max().unwrap_or(0);
        for (id, desc) in entries {
            let _ = writeln!(handle, "  {id:<width$}  {desc}");
        }
        return ExitCode::SUCCESS;
    }

    if args.rule_ids.is_empty() {
        eprintln!(
            "error: pass one or more rule ids, or use --list / --list-verbose.\n\
             \n\
             Examples:\n  \
             lucid-lint explain sentence-too-long\n  \
             lucid-lint explain sentence-too-long weasel-words\n  \
             lucid-lint explain --list\n  \
             lucid-lint explain --list-verbose"
        );
        return ExitCode::from(2);
    }

    let (rendered, all_found) = explain::render_many(&args.rule_ids, args.keep_relative);
    if handle.write_all(rendered.as_bytes()).is_err() {
        return ExitCode::from(2);
    }
    if all_found {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn run_check(args: CheckArgs) -> Result<ExitCode> {
    let file_config = load_file_config(args.config.as_deref())?;

    let profile = resolve_profile(args.profile, file_config.as_ref());
    let format: Format = args.format.into();
    let conditions = resolve_conditions(&args.conditions, file_config.as_ref());
    let formula = resolve_formula(args.readability_formula, file_config.as_ref())?;
    let scoring_config: ScoringConfig = file_config
        .as_ref()
        .map(|c| c.scoring.clone().into_scoring_config())
        .unwrap_or_default();

    let unexplained_whitelist = match file_config.as_ref() {
        Some(c) => c
            .unexplained_abbreviation_whitelist()
            .map_err(|e| anyhow::anyhow!("{e}"))?,
        None => Vec::new(),
    };

    let engine = Engine::with_profile_and_conditions(profile, &conditions)
        .with_readability_formula(formula)
        .with_unexplained_whitelist(unexplained_whitelist)
        .with_scoring_config(scoring_config.clone());

    let exclude_matcher = build_exclude_matcher(&args.exclude, file_config.as_ref())?;

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
            let files = collect_files(raw_path, exclude_matcher.as_ref())?;
            for file in files {
                let report = engine
                    .lint_file(&file)
                    .with_context(|| format!("failed to lint {}", file.display()))?;
                total_words = total_words.saturating_add(report.word_count);
                all_diagnostics.extend(report.diagnostics);
            }
        }
    }

    // F19: drop diagnostics whose rule id is listed in `[[ignore]]`.
    // Applied here (post-engine, pre-scoring) so scoring, rendering,
    // and exit-code logic all see the filtered view.
    apply_config_ignores(&mut all_diagnostics, file_config.as_ref());

    // Aggregate across all inputs as a single document for v0.2 scoring.
    // Per-file and project-level roll-ups are tracked as F15 (ROADMAP).
    let scorecard = scoring::compute(&all_diagnostics, total_words, &scoring_config);

    let rendered = match format {
        Format::Tty => {
            let mut tty_options = tty::TtyOptions::new(tty::ColorMode::Auto);
            tty_options.group = !args.no_group;
            tty_options.explain_hint = !args.no_explain_hint;
            tty_options.score_first = args.score_first;
            tty_options.banner = args.banner.into();
            tty::render(&all_diagnostics, &scorecard, tty_options)
        },
        _ => format.render(&all_diagnostics, &scorecard),
    };
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

fn resolve_profile(cli: Option<cli::CliProfile>, file: Option<&FileConfig>) -> Profile {
    if let Some(p) = cli {
        return p.into();
    }
    file.map(|c| c.default.profile).unwrap_or_default()
}

fn resolve_conditions(
    cli: &[cli::CliConditionTag],
    file: Option<&FileConfig>,
) -> Vec<ConditionTag> {
    if !cli.is_empty() {
        return cli.iter().copied().map(Into::into).collect();
    }
    file.map(|c| c.default.conditions.clone())
        .unwrap_or_default()
}

fn resolve_formula(
    cli: Option<cli::CliFormulaChoice>,
    file: Option<&FileConfig>,
) -> Result<FormulaChoice> {
    if let Some(choice) = cli {
        return Ok(choice.into());
    }
    let from_file = match file {
        Some(c) => c
            .readability_formula()
            .map_err(|e| anyhow::anyhow!("{e}"))?,
        None => None,
    };
    Ok(from_file.unwrap_or_default())
}

/// Load the user's `lucid-lint.toml`.
///
/// If `explicit` is `Some`, the path must exist and parse cleanly — any
/// error is surfaced. Otherwise auto-discover by walking up from the
/// current working directory; a missing file is not an error (returns
/// `Ok(None)`), but a present-but-unparseable one is.
fn load_file_config(explicit: Option<&Path>) -> Result<Option<FileConfig>> {
    if let Some(path) = explicit {
        let cfg = FileConfig::from_file(path)
            .with_context(|| format!("failed to load config {}", path.display()))?;
        return Ok(Some(cfg));
    }
    let cwd = std::env::current_dir().context("failed to resolve current directory")?;
    let Some(path) = FileConfig::discover_from(&cwd) else {
        return Ok(None);
    };
    let cfg = FileConfig::from_file(&path)
        .with_context(|| format!("failed to load config {}", path.display()))?;
    Ok(Some(cfg))
}

fn collect_files(path: &Path, exclude: Option<&GlobSet>) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        // Explicit file arguments bypass exclusion — if the user named
        // it directly, they meant it.
        return Ok(vec![path.to_path_buf()]);
    }
    if path.is_dir() {
        let mut out = Vec::new();
        collect_files_recursive(path, path, exclude, &mut out)?;
        return Ok(out);
    }
    anyhow::bail!("path does not exist: {}", path.display())
}

/// Walk `dir` recursively, pruning entries that match the exclude set.
///
/// Exclusion globs are matched against the path **relative to `root`**
/// so patterns like `vendor/**` behave intuitively regardless of
/// whether the user passed a relative or absolute root.
fn collect_files_recursive(
    root: &Path,
    dir: &Path,
    exclude: Option<&GlobSet>,
    out: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("failed to read directory {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if is_excluded(&path, root, exclude) {
            continue;
        }
        if path.is_dir() {
            collect_files_recursive(root, &path, exclude, out)?;
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

/// Drop diagnostics whose rule id appears in the config's `[[ignore]]`
/// list (F19). Unknown rule ids in the config are tolerated silently,
/// matching the `[scoring.weights]` precedent.
fn apply_config_ignores(diagnostics: &mut Vec<Diagnostic>, file: Option<&FileConfig>) {
    let Some(config) = file else {
        return;
    };
    if config.ignores.is_empty() {
        return;
    }
    let silenced: std::collections::HashSet<&str> =
        config.ignores.iter().map(|i| i.rule_id.as_str()).collect();
    diagnostics.retain(|d| !silenced.contains(d.rule_id.as_str()));
}

/// Build a [`GlobSet`] from CLI and TOML `exclude` lists (F78).
///
/// Returns `Ok(None)` when both lists are empty, avoiding the cost of
/// matcher construction for the common case. Any invalid pattern is
/// surfaced as an error pointing at the offending glob.
fn build_exclude_matcher(cli: &[String], file: Option<&FileConfig>) -> Result<Option<GlobSet>> {
    let from_file = file.map_or(&[] as &[String], |c| c.default.exclude.as_slice());
    if cli.is_empty() && from_file.is_empty() {
        return Ok(None);
    }
    let mut builder = GlobSetBuilder::new();
    for pattern in from_file.iter().chain(cli.iter()) {
        let glob =
            Glob::new(pattern).with_context(|| format!("invalid exclude pattern `{pattern}`"))?;
        builder.add(glob);
    }
    let set = builder.build().context("failed to compile exclude globs")?;
    Ok(Some(set))
}

/// Match `path` against the exclude set, after stripping the walk
/// `root` so patterns like `vendor/**` hit `/abs/root/vendor/skip.md`
/// via its `vendor/skip.md` relative form. As a fallback, the raw
/// path (with a leading `./` stripped) is also tested — covering
/// absolute-path globs and explicit-file cases.
fn is_excluded(path: &Path, root: &Path, exclude: Option<&GlobSet>) -> bool {
    let Some(set) = exclude else {
        return false;
    };
    if let Ok(rel) = path.strip_prefix(root) {
        if set.is_match(rel) {
            return true;
        }
    }
    let stripped = path.strip_prefix("./").unwrap_or(path);
    set.is_match(stripped)
}
