//! The linting engine: orchestrates parsing, rule execution, and output.

use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::config::Profile;
use crate::language::{default_language, detect_language};
use crate::parser::{parse_markdown, parse_plain};
use crate::rules::{default_rules, Rule};
use crate::types::{Diagnostic, Language, SourceFile};

/// The linting engine.
///
/// Bundles a profile and a set of rules, and exposes methods to lint strings,
/// files, and stdin-sourced text.
pub struct Engine {
    profile: Profile,
    rules: Vec<Box<dyn Rule>>,
}

impl Engine {
    /// Build an engine with the default rule set for the given profile.
    #[must_use]
    pub fn with_profile(profile: Profile) -> Self {
        Self {
            profile,
            rules: default_rules(profile),
        }
    }

    /// Build an engine with a custom rule set.
    #[must_use]
    pub fn with_rules(profile: Profile, rules: Vec<Box<dyn Rule>>) -> Self {
        Self { profile, rules }
    }

    /// The profile this engine was configured with.
    #[must_use]
    pub const fn profile(&self) -> Profile {
        self.profile
    }

    /// Lint a string input. Markdown syntax is assumed.
    #[must_use]
    pub fn lint_str(&self, input: &str) -> Vec<Diagnostic> {
        self.lint_with_source(input, SourceFile::Anonymous, true)
    }

    /// Lint stdin-like input.
    #[must_use]
    pub fn lint_stdin(&self, input: &str) -> Vec<Diagnostic> {
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
    pub fn lint_file(&self, path: &Path) -> Result<Vec<Diagnostic>, EngineError> {
        let contents = fs::read_to_string(path).map_err(EngineError::Io)?;
        let is_markdown = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| matches!(ext, "md" | "markdown"));
        let source = SourceFile::Path(path.to_path_buf());
        Ok(self.lint_with_source(&contents, source, is_markdown))
    }

    fn lint_with_source(
        &self,
        input: &str,
        source: SourceFile,
        is_markdown: bool,
    ) -> Vec<Diagnostic> {
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
                .any(|dir| dir.rule_id == d.rule_id && dir.target_line == d.location.line)
        });
        diagnostics
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
        let diags = engine.lint_str(text);
        assert!(!diags.is_empty());
        assert!(diags.iter().all(|d| d.severity == Severity::Warning));
    }

    #[test]
    fn engine_returns_no_diagnostics_for_clean_text() {
        let engine = Engine::with_profile(Profile::Public);
        let diags = engine.lint_str("Short clean sentence. Another fine one.");
        assert!(diags.is_empty());
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
        assert!(diags_for_rule(&public.lint_str(text), "sentence-too-long") > 0);
        assert_eq!(diags_for_rule(&dev.lint_str(text), "sentence-too-long"), 0);
    }

    #[test]
    fn inline_disable_suppresses_matching_diagnostic() {
        let engine = Engine::with_profile(Profile::Public);
        let text = "Intro paragraph.\n\n\
                    <!-- lucid-lint disable-next-line sentence-too-long -->\n\
                    This is a long sentence that keeps adding more and more words until it \
                    exceeds the public profile threshold by a comfortable margin of safety.\n";
        let diags = engine.lint_str(text);
        assert_eq!(
            diags_for_rule(&diags, "sentence-too-long"),
            0,
            "expected directive to suppress sentence-too-long, got: {diags:?}"
        );
    }

    #[test]
    fn inline_disable_does_not_affect_other_rules_or_lines() {
        // Directive for an unrelated rule id does not suppress sentence-too-long.
        let engine = Engine::with_profile(Profile::Public);
        let text = "Intro.\n\n\
                    <!-- lucid-lint disable-next-line weasel-words -->\n\
                    This is a long sentence that keeps adding more and more words until it \
                    exceeds the public profile threshold by a comfortable margin of safety.\n";
        let diags = engine.lint_str(text);
        assert_eq!(diags_for_rule(&diags, "sentence-too-long"), 1);
    }

    #[test]
    fn engine_profile_accessor() {
        let engine = Engine::with_profile(Profile::Falc);
        assert_eq!(engine.profile(), Profile::Falc);
    }
}
