//! The linting engine: orchestrates parsing, rule execution, and output.

use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{default_language, detect_language};
use crate::parser::{parse_markdown, parse_plain, word_count};
use crate::rules::lexicon::unexplained_abbreviation::{self, UnexplainedAbbreviation};
use crate::rules::readability::score::{self, FormulaChoice, ReadabilityScore};
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
