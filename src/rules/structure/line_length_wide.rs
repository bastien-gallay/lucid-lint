//! Rule: `line-length-wide`.
//!
//! Flags author-chosen lines whose character width exceeds a per-profile
//! ceiling. WCAG 1.4.8 (AAA) caps rendered text at roughly 80 characters
//! per line because longer lines force the eye to track further between
//! saccades and increase re-reading on return-sweep — a known difficulty
//! for dyslexic readers (BDA Dyslexia Style Guide).
//!
//! "Author-chosen" matters: Markdown soft breaks are collapsed to spaces
//! at parse time because the renderer reflows them to fit the viewport,
//! so their source length tells us nothing about what the reader sees.
//! Only line breaks the author intended to keep are checked here:
//! Markdown hard breaks (`<br>` or two trailing spaces) and explicit
//! newlines in plain-text input. A soft-wrapped Markdown paragraph
//! collapses to a single logical line and is therefore exempt — even if
//! the joined text exceeds the ceiling. Use [`structure.paragraph-too-long`]
//! to bound paragraph density. Fenced and indented code blocks are
//! excluded upstream by the Markdown parser.
//!
//! See [`RULES.md`](../../RULES.md#line-length-wide) for the threshold
//! reference.
//!
//! [`structure.paragraph-too-long`]: ../paragraph_too_long/index.html

use std::num::NonZeroU32;

use unicode_segmentation::UnicodeSegmentation;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity};

/// Configuration for [`LineLengthWide`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed character width (in grapheme clusters) per line.
    pub max_line_length: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 120,
            Profile::Public => 100,
            // WCAG 1.4.8 (AAA) recommends 80 characters.
            Profile::Falc => 80,
        };
        Self {
            max_line_length: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`LineLengthWide`] rule.
#[derive(Debug, Clone, Copy)]
pub struct LineLengthWide {
    config: Config,
}

impl LineLengthWide {
    /// Build the rule from explicit config.
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Build the rule using the preset for the given profile.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        Self::new(Config::for_profile(profile))
    }

    /// The rule identifier.
    pub const ID: &'static str = "structure.line-length-wide";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[ConditionTag::Dyslexia, ConditionTag::General];
}

impl Rule for LineLengthWide {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_line_length.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            // Soft-wrapped Markdown paragraphs collapse to a single logical
            // line at parse time (soft breaks → spaces). Treating that
            // joined text as one giant "line" measures source mechanics,
            // not the rendered width WCAG 1.4.8 actually targets — so skip
            // paragraphs that carry no authorial `\n`. Hard breaks and
            // plain-text newlines are preserved in `paragraph.text`.
            if !paragraph.text.contains('\n') {
                continue;
            }
            for (line_offset, line) in paragraph.text.lines().enumerate() {
                let grapheme_count =
                    u32::try_from(line.graphemes(true).count()).unwrap_or(u32::MAX);
                if grapheme_count > max {
                    let line_number = paragraph
                        .start_line
                        .saturating_add(u32::try_from(line_offset).unwrap_or(u32::MAX));
                    let location =
                        Location::new(document.source.clone(), line_number, 1, grapheme_count);
                    let message =
                        format!("Line is {grapheme_count} characters wide (maximum {max}).");
                    let mut diag = Diagnostic::new(Self::ID, Severity::Warning, location, message);
                    if let Some(title) = section_title {
                        diag = diag.with_section(title);
                    }
                    diagnostics.push(diag);
                }
            }
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        LineLengthWide::for_profile(profile).check(&document, Language::En)
    }

    /// Wrap a single test line between two short ones so the paragraph
    /// carries authorial `\n` separators. The new rule semantics exempt
    /// paragraphs without any inner break — without this scaffold every
    /// threshold test would vacuously pass.
    fn lint_isolated_line(line: &str, profile: Profile) -> Vec<Diagnostic> {
        let text = format!("Lead.\n{line}\nTrail.");
        lint(&text, profile)
    }

    fn lint_md(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        LineLengthWide::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(LineLengthWide::ID, "structure.line-length-wide");
    }

    #[test]
    fn tags_carry_dyslexia_and_general() {
        assert!(LineLengthWide::TAGS.contains(&ConditionTag::Dyslexia));
        assert!(LineLengthWide::TAGS.contains(&ConditionTag::General));
    }

    #[test]
    fn category_is_structure() {
        let line = format!("{}x", "a".repeat(120));
        let diags = lint_isolated_line(&line, Profile::Falc);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].category(), Category::Structure);
    }

    #[test]
    fn short_line_does_not_trigger() {
        assert!(lint_isolated_line("A short line.", Profile::Public).is_empty());
    }

    #[test]
    fn line_at_threshold_does_not_trigger() {
        // Public threshold = 100 chars exactly is allowed.
        let line = "a".repeat(100);
        assert!(lint_isolated_line(&line, Profile::Public).is_empty());
    }

    #[test]
    fn line_above_threshold_triggers_under_public() {
        let line = "a".repeat(101);
        let diags = lint_isolated_line(&line, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("101 characters"));
    }

    #[test]
    fn falc_is_strictest() {
        // 90 chars: passes Public (100) and DevDoc (120), fails FALC (80).
        let line = "a".repeat(90);
        assert!(lint_isolated_line(&line, Profile::Public).is_empty());
        assert!(lint_isolated_line(&line, Profile::DevDoc).is_empty());
        let diags = lint_isolated_line(&line, Profile::Falc);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn dev_doc_is_most_tolerant() {
        // 110 chars: triggers Public, passes DevDoc.
        let line = "a".repeat(110);
        assert!(!lint_isolated_line(&line, Profile::Public).is_empty());
        assert!(lint_isolated_line(&line, Profile::DevDoc).is_empty());
    }

    #[test]
    fn each_long_line_gets_its_own_diagnostic() {
        let long = "a".repeat(110);
        let text = format!("{long}\n{long}\n{long}");
        let diags = lint(&text, Profile::Public);
        assert_eq!(diags.len(), 3);
    }

    #[test]
    fn multibyte_grapheme_clusters_count_as_one() {
        // ñ is 1 grapheme, 2 bytes — 100 of them exactly hit the Public
        // threshold (in graphemes), 101 spill over.
        let at_threshold = "ñ".repeat(100);
        assert!(lint_isolated_line(&at_threshold, Profile::Public).is_empty());
        let over_threshold = "ñ".repeat(101);
        assert_eq!(
            lint_isolated_line(&over_threshold, Profile::Public).len(),
            1
        );
    }

    #[test]
    fn fenced_code_block_is_ignored() {
        let code_line = "a".repeat(150);
        let md = format!("Intro.\n\n```\n{code_line}\n```\n\nMore prose.\n");
        assert!(lint_md(&md, Profile::Public).is_empty());
    }

    #[test]
    fn soft_wrapped_markdown_paragraph_is_exempt() {
        // A multi-sentence prose paragraph written as one source line is
        // soft-wrapped by mdBook on render. Its joined length is huge but
        // the rendered line width is whatever the viewport decides — the
        // rule must not fire on this shape, which is the v0.2 dogfood bug.
        let prose = "word ".repeat(80);
        assert!(prose.chars().count() > 200);
        let md = format!("# Title\n\n{}\n", prose.trim());
        assert!(lint_md(&md, Profile::Public).is_empty());
    }

    #[test]
    fn markdown_hard_break_is_checked() {
        // Two trailing spaces before a newline emit a Markdown HardBreak,
        // which the parser preserves as `\n` in `paragraph.text`. The
        // author chose this wrap, so the rule does measure each chunk.
        let long = "a".repeat(150);
        let md = format!("Lead-in.  \n{long}  \nTrail.\n");
        let diags = lint_md(&md, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("150 characters"));
    }

    #[test]
    fn long_heading_is_out_of_scope() {
        // Long headings render in display type at much larger sizes; WCAG
        // 1.4.8 targets body-text line length. The parser stores headings
        // as section titles, not paragraphs, so they are out of scope by
        // construction. This test pins that contract.
        let heading = format!("# {}", "a".repeat(200));
        let md = format!("{heading}\n\nShort body.\n");
        assert!(lint_md(&md, Profile::Public).is_empty());
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(
            Config::for_profile(Profile::DevDoc).max_line_length.get(),
            120
        );
        assert_eq!(
            Config::for_profile(Profile::Public).max_line_length.get(),
            100
        );
        assert_eq!(Config::for_profile(Profile::Falc).max_line_length.get(), 80);
    }

    #[test]
    fn snapshot_fixture() {
        let text = format!("Short.\n{}\nShort again.", "a".repeat(110));
        let diags = lint(&text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
