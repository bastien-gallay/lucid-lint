//! Human-readable terminal output with optional colors.
//!
//! Colors are applied only when stdout is a tty, unless overridden by
//! [`ColorMode::Always`] or [`ColorMode::Never`].

use std::collections::BTreeMap;
use std::fmt::Write;

use owo_colors::{OwoColorize, Stream};

use crate::explain::DOCS_BASE;
use crate::scoring::{Score, Scorecard};
use crate::types::{Diagnostic, Severity, SourceFile};

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

/// Controls when the wordmark banner is printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerPolicy {
    /// Print when stdout is a TTY — same detection rule as
    /// [`ColorMode::Auto`]. Interactive runs see the banner; CI, pipes,
    /// and redirects do not. Default.
    Auto,
    /// Print on every run, regardless of output destination.
    Always,
    /// Never print.
    Never,
}

/// Human-output rendering knobs.
#[derive(Debug, Clone, Copy)]
pub struct TtyOptions {
    /// ANSI colour policy.
    pub color_mode: ColorMode,
    /// When `true` (default), cluster diagnostics that share a `(file, rule_id)`
    /// under a rule header when the cluster has two or more hits. Set to
    /// `false` for the pre-grouping flat layout (`--no-group`).
    pub group: bool,
    /// When `true` (default), print a dim hint line under the summary
    /// pointing at `lucid-lint explain`. Disabled via `--no-explain-hint`
    /// and never printed on the empty (clean) path.
    pub explain_hint: bool,
    /// When `true`, emit the score block *before* the diagnostics so CI
    /// readers see the headline up top. Default is `false`: the score
    /// stays at the end, matching the clippy / ruff family.
    pub score_first: bool,
    /// When to print the wordmark banner. Default is [`BannerPolicy::Auto`]
    /// — clean runs only, as a peak-end flourish.
    pub banner: BannerPolicy,
}

impl TtyOptions {
    /// Build options with the given colour policy and defaults for the
    /// rest: grouping on, explain hint on, score-at-end, banner auto.
    #[must_use]
    pub const fn new(color_mode: ColorMode) -> Self {
        Self {
            color_mode,
            group: true,
            explain_hint: true,
            score_first: false,
            banner: BannerPolicy::Auto,
        }
    }
}

/// Minimum cluster size before grouping kicks in.
const GROUP_MIN: usize = 2;

/// Width of the divider that precedes the score line.
const DIVIDER_WIDTH: usize = 60;

/// Render diagnostics + scorecard as human-readable output.
#[must_use]
pub fn render(diagnostics: &[Diagnostic], scorecard: &Scorecard, options: TtyOptions) -> String {
    let diagnostics_block = render_diagnostics_block(diagnostics, options);
    let score_block = render_score_block(scorecard, options.color_mode);
    let divider = format!("{}\n", dim(&"─".repeat(DIVIDER_WIDTH), options.color_mode),);

    let banner = if banner_fires(options.banner, options.color_mode) {
        render_banner(options.color_mode)
    } else {
        String::new()
    };

    if options.score_first {
        format!("{banner}{score_block}{divider}{diagnostics_block}")
    } else {
        format!("{banner}{diagnostics_block}{divider}{score_block}")
    }
}

fn banner_fires(policy: BannerPolicy, color_mode: ColorMode) -> bool {
    match policy {
        BannerPolicy::Always => true,
        BannerPolicy::Never => false,
        BannerPolicy::Auto => should_color(color_mode),
    }
}

/// Rendered wordmark — a three-part mark mirroring the SVG logo: a
/// wavy pre-clarity run, the lens (`⟨ • ⟩`), and a clean post-clarity
/// run. Tildes are dim (prose noise), the lens is blue-bold (the brand
/// focal colour), and the dashes are bold default (strong, readable on
/// both light and dark terminals).
fn render_banner(color_mode: ColorMode) -> String {
    const WAVY: &str = "~~~~~";
    const LENS: &str = "⟨ • ⟩";
    const CLEAN: &str = "─────";
    const NAME: &str = "lucid-lint";
    const TAGLINE: &str = "cognitive accessibility linter · prose · EN / FR";
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    // Continuation-line indent lines up under the wordmark. Computed
    // from the glyph widths so a future mark rearrangement stays aligned.
    let prefix_cols =
        WAVY.chars().count() + 1 + LENS.chars().count() + 1 + CLEAN.chars().count() + 2;
    let indent = " ".repeat(prefix_cols);
    let divider = "─".repeat(TAGLINE.chars().count());
    format!(
        "{} {} {}  {}  {}\n{indent}{}\n{indent}{}\n\n",
        dim(WAVY, color_mode),
        blue_bold(LENS, color_mode),
        bold(CLEAN, color_mode),
        bold(NAME, color_mode),
        dim(&version, color_mode),
        dim(TAGLINE, color_mode),
        dim(&divider, color_mode),
    )
}

fn blue(s: &str, mode: ColorMode) -> String {
    if should_color(mode) {
        s.if_supports_color(Stream::Stdout, OwoColorize::blue)
            .to_string()
    } else {
        s.to_string()
    }
}

fn blue_bold(s: &str, mode: ColorMode) -> String {
    bold(&blue(s, mode), mode)
}

fn render_diagnostics_block(diagnostics: &[Diagnostic], options: TtyOptions) -> String {
    let mut out = String::new();
    if diagnostics.is_empty() {
        let _ = writeln!(out, "{}", green("No issues found.", options.color_mode));
        let _ = writeln!(out);
    } else {
        if options.group {
            render_grouped(&mut out, diagnostics, options);
        } else {
            for diag in diagnostics {
                let _ = write!(out, "{}", format_diagnostic(diag, options.color_mode));
            }
            let _ = writeln!(out);
            let _ = writeln!(out, "{}", summary(diagnostics, options.color_mode));
        }
        if options.explain_hint {
            let _ = writeln!(out, "{}", explain_hint_line(options.color_mode));
        }
    }
    out
}

fn render_score_block(scorecard: &Scorecard, color_mode: ColorMode) -> String {
    let mut out = String::new();
    for line in score_lines(scorecard, color_mode) {
        let _ = writeln!(out, "{line}");
    }
    out
}

/// Group diagnostics by `(file, rule_id)` and render clusters with
/// `>= GROUP_MIN` members under a compact header. Singletons keep the flat
/// one-line format.
fn render_grouped(out: &mut String, diagnostics: &[Diagnostic], options: TtyOptions) {
    // Preserve input order for singletons; for clusters, members are emitted
    // in the order they were produced (source position).
    let mut clusters: BTreeMap<(String, String), Vec<&Diagnostic>> = BTreeMap::new();
    let mut cluster_order: Vec<(String, String)> = Vec::new();

    for diag in diagnostics {
        let key = (file_label(&diag.location.file), diag.rule_id.clone());
        let slot = clusters.entry(key.clone()).or_insert_with(|| {
            cluster_order.push(key.clone());
            Vec::new()
        });
        slot.push(diag);
    }

    for key in &cluster_order {
        let members = &clusters[key];
        if members.len() >= GROUP_MIN {
            format_cluster(out, &key.0, &key.1, members, options.color_mode);
        } else {
            let _ = write!(out, "{}", format_diagnostic(members[0], options.color_mode));
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "{}", summary(diagnostics, options.color_mode));
}

fn format_cluster(
    out: &mut String,
    file: &str,
    rule_id: &str,
    members: &[&Diagnostic],
    color_mode: ColorMode,
) {
    let severity = members[0].severity;
    let count = members.len();
    let severity_word = plural_severity(severity, count);
    let header = format!(
        "{} · {count} {severity_word} in {} · {}",
        severity_label(severity, color_mode),
        bold(file, color_mode),
        dim(&format!("[{rule_id}]"), color_mode),
    );
    let _ = writeln!(out, "{header}");

    let max_loc = members
        .iter()
        .map(|d| {
            format!("{}:{}", d.location.line, d.location.column)
                .chars()
                .count()
        })
        .max()
        .unwrap_or(0);

    for diag in members {
        let loc = format!("{}:{}", diag.location.line, diag.location.column);
        // Measure padding on the plain string; colour escapes would
        // otherwise count toward width and short-pad coloured runs.
        let pad = " ".repeat(max_loc - loc.chars().count());
        let section_suffix = diag
            .section
            .as_deref()
            .map(|s| format!(" [section: {s}]"))
            .unwrap_or_default();
        let _ = writeln!(
            out,
            "  {}{pad}  {}{}",
            bold(&loc, color_mode),
            diag.message,
            dim(&section_suffix, color_mode),
        );
    }
}

fn file_label(source: &SourceFile) -> String {
    source.to_string()
}

fn plural_severity(severity: Severity, count: usize) -> &'static str {
    match (severity, count) {
        (Severity::Info, 1) => "info",
        (Severity::Info, _) => "info",
        (Severity::Warning, 1) => "warning",
        (Severity::Warning, _) => "warnings",
        (Severity::Error, 1) => "error",
        (Severity::Error, _) => "errors",
    }
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
    let rule_suffix = format!(" [{}]", diag.rule_id);

    let _ = writeln!(
        out,
        "{} {} {}{}{}",
        severity_str,
        bold(&location, color_mode),
        diag.message,
        dim(&section_suffix, color_mode),
        dim(&rule_suffix, color_mode),
    );

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
            "{} {}",
            red(&error_count.to_string(), color_mode),
            plural_severity(Severity::Error, error_count),
        ));
    }
    if warn_count > 0 {
        parts.push(format!(
            "{} {}",
            yellow(&warn_count.to_string(), color_mode),
            plural_severity(Severity::Warning, warn_count),
        ));
    }
    if info_count > 0 {
        parts.push(format!(
            "{} {}",
            dim(&info_count.to_string(), color_mode),
            plural_severity(Severity::Info, info_count),
        ));
    }

    if parts.is_empty() {
        "No issues found.".to_string()
    } else {
        format!("summary: {}.", parts.join(", "))
    }
}

fn explain_hint_line(color_mode: ColorMode) -> String {
    let text = format!("→ run 'lucid-lint explain <rule-id>' or see {DOCS_BASE}/rules/<rule-id>",);
    dim(&text, color_mode)
}

/// Number of blocks in each per-category sparkline bar.
const BAR_BUCKETS: u32 = 5;

fn score_lines(scorecard: &Scorecard, color_mode: ColorMode) -> Vec<String> {
    let mut lines = vec![format!(
        "score: {}",
        score_fragment_bold(scorecard.global, color_mode),
    )];

    let label_width = scorecard
        .per_category
        .iter()
        .map(|cs| cs.category.to_string().len())
        .max()
        .unwrap_or(0);

    for cs in &scorecard.per_category {
        let label = cs.category.to_string();
        let padded = format!("{label:<label_width$}");
        lines.push(format!(
            "       {}  {}  {}",
            dim(&padded, color_mode),
            bar(cs.score, color_mode),
            score_fragment(cs.score, color_mode),
        ));
    }
    lines
}

fn ratio_of(score: Score) -> f64 {
    if score.max == 0 {
        1.0
    } else {
        (f64::from(score.value) / f64::from(score.max)).clamp(0.0, 1.0)
    }
}

fn band_apply(ratio: f64, text: &str, color_mode: ColorMode) -> String {
    if ratio >= 0.80 {
        green(text, color_mode)
    } else if ratio >= 0.60 {
        yellow(text, color_mode)
    } else {
        red(text, color_mode)
    }
}

fn score_fragment(score: Score, color_mode: ColorMode) -> String {
    let text = format!("{}/{}", score.value, score.max);
    band_apply(ratio_of(score), &text, color_mode)
}

fn score_fragment_bold(score: Score, color_mode: ColorMode) -> String {
    let banded = score_fragment(score, color_mode);
    bold(&banded, color_mode)
}

fn bar(score: Score, color_mode: ColorMode) -> String {
    let ratio = ratio_of(score);
    // `ratio` is clamped to `[0, 1]` by `ratio_of`, and `BAR_BUCKETS` is a
    // small positive constant, so the product after rounding is safely
    // representable as `u32`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let filled = (ratio * f64::from(BAR_BUCKETS)).round() as u32;
    let filled = filled.min(BAR_BUCKETS);
    let filled_str: String = (0..filled).map(|_| '▇').collect();
    let empty_str: String = (filled..BAR_BUCKETS).map(|_| '░').collect();
    format!(
        "{}{}",
        band_apply(ratio, &filled_str, color_mode),
        dim(&empty_str, color_mode),
    )
}

/// Visible width all severity labels pad to. Matches the longest word
/// (`warning`) so locations line up vertically across mixed-severity runs.
const SEVERITY_WIDTH: usize = 7;

fn severity_label(severity: Severity, color_mode: ColorMode) -> String {
    let (word, coloured) = match severity {
        // info is dim, not cyan, so yellow `warning` owns the attention palette.
        Severity::Info => ("info", dim("info", color_mode)),
        Severity::Warning => ("warning", yellow("warning", color_mode)),
        Severity::Error => ("error", red("error", color_mode)),
    };
    let pad = " ".repeat(SEVERITY_WIDTH - word.len());
    format!("{coloured}{pad}")
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
    use crate::scoring::{self, ScoringConfig};
    use crate::types::{Location, SourceFile};

    fn sample_diag() -> Diagnostic {
        Diagnostic::new(
            "structure.sentence-too-long",
            Severity::Warning,
            Location::new(SourceFile::Anonymous, 3, 1, 42),
            "Sentence is 25 words long (maximum 22).",
        )
    }

    fn card(diags: &[Diagnostic]) -> Scorecard {
        scoring::compute(diags, 1000, &ScoringConfig::default())
    }

    #[test]
    fn render_empty_says_no_issues() {
        let out = render(&[], &card(&[]), TtyOptions::new(ColorMode::Never));
        assert!(out.contains("No issues found"));
        assert!(out.contains("score: 100/100"));
    }

    #[test]
    fn render_contains_severity_and_message() {
        let diag = sample_diag();
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            TtyOptions::new(ColorMode::Never),
        );
        assert!(out.contains("warning"));
        assert!(out.contains("Sentence is 25 words long"));
        assert!(out.contains("structure.sentence-too-long"));
    }

    #[test]
    fn render_includes_summary_counts() {
        let diag = sample_diag();
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            TtyOptions::new(ColorMode::Never),
        );
        assert!(out.contains("summary:"));
        assert!(
            out.contains("1 warning") && !out.contains("1 warnings"),
            "expected singular `1 warning` in summary, got: {out}"
        );
    }

    #[test]
    fn render_pluralises_severity_when_count_is_many() {
        let diags: Vec<Diagnostic> = (0..3).map(|_| sample_diag()).collect();
        let out = render(&diags, &card(&diags), TtyOptions::new(ColorMode::Never));
        assert!(out.contains("3 warnings"));
    }

    #[test]
    fn render_emits_explain_hint_by_default() {
        let diag = sample_diag();
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            TtyOptions::new(ColorMode::Never),
        );
        assert!(out.contains("lucid-lint explain <rule-id>"));
        assert!(out.contains("https://bastien-gallay.github.io/lucid-lint/rules/<rule-id>"));
    }

    #[test]
    fn render_suppresses_explain_hint_when_disabled() {
        let diag = sample_diag();
        let mut opts = TtyOptions::new(ColorMode::Never);
        opts.explain_hint = false;
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            opts,
        );
        assert!(!out.contains("lucid-lint explain <rule-id>"));
    }

    #[test]
    fn render_does_not_emit_explain_hint_on_empty_state() {
        let out = render(&[], &card(&[]), TtyOptions::new(ColorMode::Never));
        assert!(out.contains("No issues found"));
        assert!(
            !out.contains("lucid-lint explain"),
            "empty state must not carry the hint line: {out}"
        );
    }

    #[test]
    fn render_includes_section_when_present() {
        let diag = sample_diag().with_section("Introduction");
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            TtyOptions::new(ColorMode::Never),
        );
        assert!(out.contains("section: Introduction"));
    }

    #[test]
    fn render_shows_score_line_with_all_five_categories() {
        let diag = sample_diag();
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            TtyOptions::new(ColorMode::Never),
        );
        assert!(out.contains("score:"));
        for name in ["structure", "rhythm", "lexicon", "syntax", "readability"] {
            assert!(out.contains(name), "missing category {name} in output");
        }
    }

    #[test]
    fn render_emits_sparkline_bars() {
        let out = render(&[], &card(&[]), TtyOptions::new(ColorMode::Never));
        assert!(
            out.contains('▇') || out.contains('░'),
            "expected at least one block glyph in output: {out}"
        );
    }

    #[test]
    fn bar_fills_all_five_blocks_on_perfect_score() {
        let s = Score { value: 20, max: 20 };
        let rendered = bar(s, ColorMode::Never);
        assert_eq!(
            rendered.matches('▇').count(),
            5,
            "expected 5 filled blocks, got: {rendered}"
        );
        assert_eq!(
            rendered.matches('░').count(),
            0,
            "expected 0 empty blocks, got: {rendered}"
        );
    }

    #[test]
    fn bar_leaves_all_five_blocks_empty_on_zero_score() {
        let s = Score { value: 0, max: 20 };
        let rendered = bar(s, ColorMode::Never);
        assert_eq!(
            rendered.matches('▇').count(),
            0,
            "expected 0 filled blocks on zero, got: {rendered}"
        );
        assert_eq!(
            rendered.matches('░').count(),
            5,
            "expected 5 empty blocks on zero, got: {rendered}"
        );
    }

    #[test]
    fn score_first_reorders_output() {
        let diag = sample_diag();
        let mut opts = TtyOptions::new(ColorMode::Never);
        opts.score_first = true;
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            opts,
        );
        let score_idx = out.find("score:").expect("score line present");
        let diag_idx = out
            .find("Sentence is 25 words long")
            .expect("diagnostic message present");
        assert!(
            score_idx < diag_idx,
            "--score-first must emit the score before the diagnostics:\n{out}"
        );
    }

    #[test]
    fn banner_auto_fires_when_tty_is_detected() {
        // Under ColorMode::Always, should_color() returns true, standing
        // in for "stdout is a TTY" in the detection test.
        let out = render(&[], &card(&[]), TtyOptions::new(ColorMode::Always));
        assert!(out.contains("lucid-lint"));
        assert!(out.contains("cognitive accessibility linter"));
    }

    #[test]
    fn banner_auto_suppressed_when_not_a_tty() {
        // ColorMode::Never disables should_color(), standing in for
        // "stdout is not a TTY" (piped, redirected, CI).
        let out = render(&[], &card(&[]), TtyOptions::new(ColorMode::Never));
        assert!(!out.contains("cognitive accessibility linter"));
    }

    #[test]
    fn banner_always_fires_even_without_tty() {
        let diag = sample_diag();
        let mut opts = TtyOptions::new(ColorMode::Never);
        opts.banner = BannerPolicy::Always;
        let out = render(
            std::slice::from_ref(&diag),
            &card(std::slice::from_ref(&diag)),
            opts,
        );
        let tagline_idx = out
            .find("cognitive accessibility linter")
            .expect("tagline present");
        let diag_idx = out
            .find("Sentence is 25 words long")
            .expect("diagnostic message present");
        assert!(
            tagline_idx < diag_idx,
            "banner must precede the first diagnostic when --banner=always"
        );
    }

    #[test]
    fn banner_never_suppresses_even_on_tty() {
        let mut opts = TtyOptions::new(ColorMode::Always);
        opts.banner = BannerPolicy::Never;
        let out = render(&[], &card(&[]), opts);
        assert!(!out.contains("cognitive accessibility linter"));
    }

    #[test]
    fn severity_labels_pad_to_fixed_width() {
        use crate::types::{Location, SourceFile};
        let make = |sev: Severity| {
            Diagnostic::new(
                "structure.sentence-too-long",
                sev,
                Location::new(SourceFile::Anonymous, 1, 1, 10),
                "msg",
            )
        };
        let diags = vec![
            make(Severity::Info),
            make(Severity::Warning),
            make(Severity::Error),
        ];
        // Use --no-group so each severity emits its own top-level line.
        let mut opts = TtyOptions::new(ColorMode::Never);
        opts.group = false;
        let out = render(&diags, &card(&diags), opts);

        // Each diagnostic line starts with the padded severity word + a
        // space. The location column should begin at the same offset for
        // every severity.
        let offsets: Vec<usize> = out
            .lines()
            .filter(|l| l.starts_with("info") || l.starts_with("warning") || l.starts_with("error"))
            .map(|l| l.find("<input>").expect("anonymous source marker"))
            .collect();
        assert!(
            offsets.len() >= 3,
            "expected 3 diagnostic lines, got: {out}"
        );
        let first = offsets[0];
        assert!(
            offsets.iter().all(|o| *o == first),
            "severity columns misaligned: {offsets:?}"
        );
    }

    #[test]
    fn empty_state_has_blank_line_before_divider() {
        let out = render(&[], &card(&[]), TtyOptions::new(ColorMode::Never));
        let lines: Vec<&str> = out.lines().collect();
        let idx = lines
            .iter()
            .position(|l| l.contains("No issues found"))
            .expect("empty-state line present");
        assert!(
            lines[idx + 1].is_empty(),
            "expected blank line after success message, got: {:?}",
            lines[idx + 1]
        );
    }

    #[test]
    fn cluster_rows_align_on_varying_location_widths() {
        use crate::types::{Location, SourceFile};
        let path = std::path::PathBuf::from("readme.md");
        let make = |line: u32, col: u32| {
            Diagnostic::new(
                "structure.line-length-wide",
                Severity::Warning,
                Location::new(SourceFile::Path(path.clone()), line, col, 0),
                "Line is too wide.",
            )
        };
        let diags = vec![make(3, 1), make(15, 120), make(333, 1)];
        let out = render(&diags, &card(&diags), TtyOptions::new(ColorMode::Never));

        // Every cluster row ends the location column at the same column
        // before the `  ` double-space + message prefix.
        let msg_offsets: Vec<usize> = out
            .lines()
            .filter(|l| l.contains("Line is too wide"))
            .map(|l| l.find("Line is too wide").unwrap())
            .collect();
        assert_eq!(msg_offsets.len(), 3, "expected 3 cluster rows");
        let first = msg_offsets[0];
        assert!(
            msg_offsets.iter().all(|o| *o == first),
            "cluster message column misaligned: {msg_offsets:?}"
        );
    }

    #[test]
    fn global_score_fragment_wraps_banded_fragment() {
        // Under the test configuration colour support is stripped, so both
        // paths return plain text. The structural guarantee is that the
        // bold variant still contains the full value/max text — proving
        // the wrapping did not drop content.
        let s = Score {
            value: 71,
            max: 100,
        };
        let plain = score_fragment(s, ColorMode::Never);
        let bold_wrapped = score_fragment_bold(s, ColorMode::Never);
        assert!(bold_wrapped.contains(&plain));
        assert!(bold_wrapped.contains("71/100"));
    }
}
