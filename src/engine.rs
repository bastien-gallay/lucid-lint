//! The linting engine: orchestrates parsing, rule execution, and output.

use std::fs;
use std::num::NonZeroU32;
use std::path::Path;

use thiserror::Error;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{default_language, detect_language};
use crate::parser::{parse_markdown, parse_plain, word_count};
use crate::rules::lexicon::unexplained_abbreviation::{self, UnexplainedAbbreviation};
use crate::rules::readability::score::{self, FormulaChoice, ReadabilityScore};
use crate::rules::structure::excessive_commas::{self, ExcessiveCommas};
use crate::rules::{default_rules, filter_by_conditions, Rule};
use crate::scoring::{self, Scorecard, ScoringConfig};
use crate::types::{Diagnostic, Language, SourceFile};

/// Aggregated result of a lint run over a single document.
///
/// Pairs the list of diagnostics with the [`Scorecard`] that aggregates them
/// and the word count used as the density denominator.
#[derive(Debug, Clone)]
pub struct Report {
    /// Diagnostics emitted, in rule-discovery order.
    pub diagnostics: Vec<Diagnostic>,
    /// Global and per-category scores for this document.
    pub scorecard: Scorecard,
    /// Word count used to normalize scoring density.
    pub word_count: u32,
}

/// The linting engine.
///
/// Bundles a profile and a set of rules, and exposes methods to lint strings,
/// files, and stdin-sourced text.
pub struct Engine {
    profile: Profile,
    rules: Vec<Box<dyn Rule>>,
    scoring_config: ScoringConfig,
}

impl Engine {
    /// Build an engine with the default rule set for the given profile.
    #[must_use]
    pub fn with_profile(profile: Profile) -> Self {
        Self {
            profile,
            rules: default_rules(profile),
            scoring_config: ScoringConfig::default(),
        }
    }

    /// Build an engine for the given profile, restricting the rule set to
    /// rules tagged `general` plus those whose condition tags intersect with
    /// `conditions` (F71 + F72).
    #[must_use]
    pub fn with_profile_and_conditions(profile: Profile, conditions: &[ConditionTag]) -> Self {
        Self {
            profile,
            rules: filter_by_conditions(default_rules(profile), conditions),
            scoring_config: ScoringConfig::default(),
        }
    }

    /// Build an engine with a custom rule set.
    #[must_use]
    pub fn with_rules(profile: Profile, rules: Vec<Box<dyn Rule>>) -> Self {
        Self {
            profile,
            rules,
            scoring_config: ScoringConfig::default(),
        }
    }

    /// Attach a custom [`ScoringConfig`], overriding the defaults.
    #[must_use]
    pub fn with_scoring_config(mut self, scoring_config: ScoringConfig) -> Self {
        self.scoring_config = scoring_config;
        self
    }

    /// Override the [`ReadabilityScore`] rule's formula choice (F11).
    ///
    /// When `FormulaChoice::Auto` is passed the engine keeps the default
    /// per-language selection; other variants pin a concrete formula
    /// regardless of the document's detected language.
    ///
    /// If the rule set does not currently include a `readability-score`
    /// rule (e.g., it was filtered out), this is a no-op — the rule will
    /// not be re-added.
    #[must_use]
    pub fn with_readability_formula(mut self, formula: FormulaChoice) -> Self {
        for rule in &mut self.rules {
            if rule.id() == ReadabilityScore::ID {
                let config = score::Config::for_profile(self.profile).with_formula(formula);
                *rule = Box::new(ReadabilityScore::new(config));
                break;
            }
        }
        self
    }

    /// Extend the [`UnexplainedAbbreviation`] rule's user whitelist
    /// with project-specific entries (F31). The extras are additive
    /// over the profile baseline — callers typically use this to
    /// restore the narrower acronyms that F31 moved out of the shipped
    /// `dev-doc` baseline (`WCAG`, `ARIA`, `ADHD`, `LLM`, …).
    ///
    /// If the rule set does not currently include an
    /// `unexplained-abbreviation` rule, this is a no-op.
    #[must_use]
    pub fn with_unexplained_whitelist(mut self, extra: Vec<String>) -> Self {
        if extra.is_empty() {
            return self;
        }
        for rule in &mut self.rules {
            if rule.id() == UnexplainedAbbreviation::ID {
                let config = unexplained_abbreviation::Config::for_profile(self.profile)
                    .with_extra_whitelist(extra);
                *rule = Box::new(UnexplainedAbbreviation::new(config));
                break;
            }
        }
        self
    }

    /// Override the [`ExcessiveCommas`] rule's `max_commas` threshold.
    ///
    /// If the rule set does not currently include an `excessive-commas`
    /// rule (e.g., it was filtered out), this is a no-op.
    #[must_use]
    pub fn with_excessive_commas_max_commas(mut self, max_commas: NonZeroU32) -> Self {
        for rule in &mut self.rules {
            if rule.id() == ExcessiveCommas::ID {
                let config =
                    excessive_commas::Config::for_profile(self.profile).with_max_commas(max_commas);
                *rule = Box::new(ExcessiveCommas::new(config));
                break;
            }
        }
        self
    }

    /// The profile this engine was configured with.
    #[must_use]
    pub const fn profile(&self) -> Profile {
        self.profile
    }

    /// Lint a string input. Markdown syntax is assumed.
    #[must_use]
    pub fn lint_str(&self, input: &str) -> Report {
        self.lint_with_source(input, SourceFile::Anonymous, true)
    }

    /// Lint stdin-like input.
    #[must_use]
    pub fn lint_stdin(&self, input: &str) -> Report {
        self.lint_with_source(input, SourceFile::Stdin, true)
    }

    /// Lint a file from disk.
    ///
    /// Markdown is assumed for `.md` and `.markdown` extensions; other files
    /// are treated as plain text.
    ///
    /// # Errors
    ///
    /// Returns [`EngineError::Io`] if the file cannot be read.
    pub fn lint_file(&self, path: &Path) -> Result<Report, EngineError> {
        let contents = fs::read_to_string(path).map_err(EngineError::Io)?;
        let is_markdown = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| matches!(ext, "md" | "markdown"));
        let source = SourceFile::Path(path.to_path_buf());
        Ok(self.lint_with_source(&contents, source, is_markdown))
    }

    fn lint_with_source(&self, input: &str, source: SourceFile, is_markdown: bool) -> Report {
        let normalized = normalize_input(input);
        let input = normalized.as_ref();
        let language = match detect_language(input) {
            Language::Unknown => default_language(),
            detected => detected,
        };

        let document = if is_markdown {
            parse_markdown(input, source)
        } else {
            parse_plain(input, source)
        };

        let mut diagnostics = Vec::new();
        for rule in &self.rules {
            diagnostics.extend(rule.check(&document, language));
        }
        diagnostics.retain(|d| {
            !document
                .directives
                .iter()
                .any(|dir| dir.rule_id == d.rule_id && dir.covers(d.location.line))
        });

        let words = word_count(input);
        let scorecard = scoring::compute(&diagnostics, words, &self.scoring_config);

        Report {
            diagnostics,
            scorecard,
            word_count: words,
        }
    }
}

/// Normalize input at the engine boundary so every rule consumes the same
/// shape of text: leading UTF-8 BOM stripped (F110), and NFC-normalized so
/// `café` (precomposed) and `café` (decomposed) hash to the same key (F111).
fn normalize_input(input: &str) -> std::borrow::Cow<'_, str> {
    use unicode_normalization::{is_nfc_quick, IsNormalized, UnicodeNormalization};

    let stripped = input.strip_prefix('\u{FEFF}');
    let body = stripped.unwrap_or(input);
    match is_nfc_quick(body.chars()) {
        IsNormalized::Yes if stripped.is_none() => std::borrow::Cow::Borrowed(input),
        IsNormalized::Yes => std::borrow::Cow::Owned(body.to_string()),
        _ => std::borrow::Cow::Owned(body.nfc().collect()),
    }
}

/// Errors returned by the engine.
#[derive(Debug, Error)]
pub enum EngineError {
    /// I/O error reading a file.
    #[error("failed to read input file")]
    Io(#[source] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Severity;

    #[test]
    fn engine_applies_default_rules() {
        let engine = Engine::with_profile(Profile::Public);
        let text = "This is a rather long sentence that keeps adding more and more words \
                    until it exceeds the public profile threshold by a comfortable margin.";
        let report = engine.lint_str(text);
        assert!(!report.diagnostics.is_empty());
        assert!(report
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning));
    }

    #[test]
    fn engine_returns_no_warnings_for_clean_text() {
        let engine = Engine::with_profile(Profile::Public);
        let report = engine.lint_str("Short clean sentence. Another fine one.");
        assert!(report
            .diagnostics
            .iter()
            .all(|d| d.severity == Severity::Info));
    }

    fn diags_for_rule(diags: &[Diagnostic], rule_id: &str) -> usize {
        diags.iter().filter(|d| d.rule_id == rule_id).count()
    }

    #[test]
    fn engine_respects_profile() {
        let public = Engine::with_profile(Profile::Public);
        let dev = Engine::with_profile(Profile::DevDoc);
        // 25 words: triggers Public (22) but not DevDoc (30) for sentence-too-long.
        let text = "This is a long sentence that keeps adding more and more words until it \
                    exceeds the public profile threshold by a comfortable margin of safety.";
        assert!(
            diags_for_rule(
                &public.lint_str(text).diagnostics,
                "structure.sentence-too-long"
            ) > 0
        );
        assert_eq!(
            diags_for_rule(
                &dev.lint_str(text).diagnostics,
                "structure.sentence-too-long"
            ),
            0
        );
    }

    #[test]
    fn inline_disable_suppresses_matching_diagnostic() {
        let engine = Engine::with_profile(Profile::Public);
        let text = "Intro paragraph.\n\n\
                    <!-- lucid-lint disable-next-line structure.sentence-too-long -->\n\
                    This is a long sentence that keeps adding more and more words until it \
                    exceeds the public profile threshold by a comfortable margin of safety.\n";
        let report = engine.lint_str(text);
        assert_eq!(
            diags_for_rule(&report.diagnostics, "structure.sentence-too-long"),
            0,
            "expected directive to suppress sentence-too-long, got: {:?}",
            report.diagnostics
        );
    }

    #[test]
    fn inline_disable_does_not_affect_other_rules_or_lines() {
        let engine = Engine::with_profile(Profile::Public);
        let text = "Intro.\n\n\
                    <!-- lucid-lint disable-next-line weasel-words -->\n\
                    This is a long sentence that keeps adding more and more words until it \
                    exceeds the public profile threshold by a comfortable margin of safety.\n";
        let report = engine.lint_str(text);
        assert_eq!(
            diags_for_rule(&report.diagnostics, "structure.sentence-too-long"),
            1
        );
    }

    #[test]
    fn block_disable_suppresses_diagnostics_within_scope() {
        let engine = Engine::with_profile(Profile::Public);
        let long_sentence = "This is a long sentence that keeps adding more and more words \
                             until it exceeds the public profile threshold by a comfortable \
                             margin of safety.";
        let text = format!(
            "Intro.\n\n\
             <!-- lucid-lint-disable structure.sentence-too-long -->\n\n\
             {long_sentence}\n\n\
             {long_sentence}\n\n\
             <!-- lucid-lint-enable -->\n\n\
             {long_sentence}\n",
        );
        let report = engine.lint_str(&text);
        // The two long sentences inside the block are suppressed; the one
        // after the enable comment still triggers.
        assert_eq!(
            diags_for_rule(&report.diagnostics, "structure.sentence-too-long"),
            1,
            "expected block directive to suppress 2 of 3 diagnostics, got: {:?}",
            report.diagnostics,
        );
    }

    #[test]
    fn engine_profile_accessor() {
        let engine = Engine::with_profile(Profile::Falc);
        assert_eq!(engine.profile(), Profile::Falc);
    }

    #[test]
    fn normalize_input_passes_through_clean_ascii_borrowed() {
        // Fast path: already-NFC + no BOM → Cow::Borrowed, no allocation.
        let input = "Plain ASCII sentence.";
        let out = normalize_input(input);
        assert!(matches!(out, std::borrow::Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), input);
    }

    #[test]
    fn normalize_input_passes_through_nfc_unicode_borrowed() {
        // Already-NFC accented text without a BOM also stays borrowed.
        let input = "Le café est prêt.";
        let out = normalize_input(input);
        assert!(matches!(out, std::borrow::Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), input);
    }

    #[test]
    fn normalize_input_strips_leading_bom_only() {
        let out = normalize_input("\u{FEFF}hello");
        assert_eq!(out.as_ref(), "hello");
    }

    #[test]
    fn normalize_input_does_not_strip_inner_bom() {
        // Only the *leading* BOM is stripped; inner U+FEFF (zero-width
        // no-break space) is preserved so it doesn't silently mutate prose.
        let input = "hello\u{FEFF}world";
        let out = normalize_input(input);
        assert_eq!(out.as_ref(), input);
    }

    #[test]
    fn normalize_input_nfc_normalizes_decomposed_text() {
        // NFD `cafe + U+0301` → NFC `café`.
        let out = normalize_input("cafe\u{0301}");
        assert_eq!(out.as_ref(), "café");
    }

    #[test]
    fn normalize_input_strips_bom_and_nfc_normalizes() {
        // Combined path: leading BOM + NFD body.
        let out = normalize_input("\u{FEFF}cafe\u{0301}");
        assert_eq!(out.as_ref(), "café");
    }

    #[test]
    fn normalize_input_handles_empty_string() {
        let out = normalize_input("");
        assert_eq!(out.as_ref(), "");
        assert!(matches!(out, std::borrow::Cow::Borrowed(_)));
    }

    #[test]
    fn bom_prefix_does_not_shift_diagnostics() {
        let engine = Engine::with_profile(Profile::Public);
        let body = "This is a long sentence that keeps adding more and more words until it \
                    exceeds the public profile threshold by a comfortable margin of safety.";
        let with_bom = format!("\u{FEFF}{body}");
        let plain = engine.lint_str(body);
        let bommed = engine.lint_str(&with_bom);
        assert_eq!(plain.diagnostics.len(), bommed.diagnostics.len());
        for (a, b) in plain.diagnostics.iter().zip(bommed.diagnostics.iter()) {
            assert_eq!(a.rule_id, b.rule_id);
            assert_eq!(a.location.line, b.location.line);
            assert_eq!(a.location.column, b.location.column);
            assert_eq!(a.message, b.message);
        }
    }

    #[test]
    fn nfd_input_yields_same_diagnostics_as_nfc() {
        // "café" precomposed (NFC) vs decomposed (NFD: e + combining acute).
        // Rules using HashMap keys (e.g. low-lexical-diversity) would treat
        // the two as different words without normalization at the boundary.
        let engine = Engine::with_profile(Profile::Public);
        let nfc = "Le café est bon. Le café est chaud. Le café est noir. Le café est fort.";
        let nfd = "Le cafe\u{0301} est bon. Le cafe\u{0301} est chaud. Le cafe\u{0301} est noir. \
                   Le cafe\u{0301} est fort.";
        let a = engine.lint_str(nfc);
        let b = engine.lint_str(nfd);
        assert_eq!(a.diagnostics.len(), b.diagnostics.len());
        for (x, y) in a.diagnostics.iter().zip(b.diagnostics.iter()) {
            assert_eq!(x.rule_id, y.rule_id);
            assert_eq!(x.location.line, y.location.line);
        }
    }

    #[test]
    fn lone_cr_line_endings_are_normalized() {
        // Classic Mac line endings: bare \r between paragraphs.
        // Parser already maps \r → \n at src/parser/mod.rs; this pins the
        // behaviour so a future refactor can't silently drop it.
        let engine = Engine::with_profile(Profile::Public);
        let lf = "First paragraph.\n\nSecond paragraph.\n\nThird.";
        let cr = "First paragraph.\r\rSecond paragraph.\r\rThird.";
        let a = engine.lint_str(lf);
        let b = engine.lint_str(cr);
        assert_eq!(a.word_count, b.word_count);
        assert_eq!(a.diagnostics.len(), b.diagnostics.len());
    }

    #[test]
    fn zero_width_chars_inside_words_pin_behaviour() {
        // Zero-width chars (U+200B/200C/200D) sometimes survive copy-paste
        // from social-media or PDF sources. Pin observed behaviour: input
        // round-trips through the engine without panicking and produces a
        // valid Report. The exact word count is not asserted — `nfc()` does
        // not strip them, and `unicode-segmentation`'s word boundary rules
        // decide whether they split tokens.
        let engine = Engine::with_profile(Profile::Public);
        let text = "Hello\u{200B}world. Bonjour\u{200C}le\u{200D}monde.";
        let report = engine.lint_str(text);
        let _ = report.word_count;
    }

    #[test]
    fn engine_produces_scorecard_with_fixed_max() {
        let engine = Engine::with_profile(Profile::Public);
        let report = engine.lint_str("Short clean sentence. Another fine one.");
        assert_eq!(
            report.scorecard.global.max,
            crate::scoring::DEFAULT_CATEGORY_MAX * 5
        );
        assert_eq!(report.scorecard.per_category.len(), 5);
    }
}
