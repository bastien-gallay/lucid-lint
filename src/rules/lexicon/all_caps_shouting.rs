//! Rule: `all-caps-shouting`.
//!
//! Flags runs of consecutive ALL-CAPS words. ALL-CAPS prose strips the
//! shape cues (ascenders, descenders, x-height contrast) that dyslexic
//! readers rely on to disambiguate words, and it triggers screen readers
//! to spell out the run letter by letter unless the surrounding markup
//! says otherwise. Plain-language and accessibility guidelines (WCAG
//! 3.1.5, BDA Dyslexia Style Guide) recommend lowercase or sentence case
//! for emphasis.
//!
//! Detection scope:
//!
//! - A "run" is two or more consecutive ALL-CAPS words separated only by
//!   whitespace and minor punctuation (`,`, `;`, `:`, `-`).
//! - A single all-caps token is treated as an abbreviation, not shouting,
//!   and is the responsibility of [`crate::rules::UnexplainedAbbreviation`].
//!
//! See [`RULES.md`](../../RULES.md#all-caps-shouting) for the threshold
//! reference.

use std::num::NonZeroU32;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity};

/// Configuration for [`AllCapsShouting`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Minimum number of consecutive ALL-CAPS words that constitutes a
    /// "shouting run". Single all-caps tokens are abbreviations, not
    /// shouting, so the floor is at least 2.
    pub min_run_length: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let min = match profile {
            // Dev docs occasionally emphasize "DO NOT" — tolerate a 2-word
            // run, flag from 3 onwards.
            Profile::DevDoc => 3,
            Profile::Public | Profile::Falc => 2,
        };
        Self {
            min_run_length: NonZeroU32::new(min).expect("non-zero literal"),
        }
    }
}

/// The [`AllCapsShouting`] rule.
#[derive(Debug, Clone, Copy)]
pub struct AllCapsShouting {
    config: Config,
}

impl AllCapsShouting {
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
    pub const ID: &'static str = "lexicon.all-caps-shouting";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[
        ConditionTag::A11yMarkup,
        ConditionTag::Dyslexia,
        ConditionTag::General,
    ];
}

impl Rule for AllCapsShouting {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let min_run = self.config.min_run_length.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            for run in find_caps_runs(&paragraph.text, min_run) {
                let line = paragraph.start_line + run.line_offset;
                let column = run.column;
                let length = u32::try_from(run.text.chars().count()).unwrap_or(u32::MAX);
                let location = Location::new(document.source.clone(), line, column, length);
                let message = format!(
                    "{} consecutive ALL-CAPS words read as shouting and degrade legibility for \
                     dyslexic readers. Use sentence case and rely on emphasis (italics, bold) or \
                     a callout instead.",
                    run.word_count
                );
                let mut diag = Diagnostic::new(Self::ID, Severity::Warning, location, message);
                if let Some(title) = section_title {
                    diag = diag.with_section(title);
                }
                diagnostics.push(diag);
            }
        }

        diagnostics
    }
}

#[derive(Debug)]
struct Run {
    text: String,
    word_count: u32,
    line_offset: u32,
    column: u32,
}

/// Scan `text` and return every run of `min_run` or more consecutive
/// ALL-CAPS words.
fn find_caps_runs(text: &str, min_run: u32) -> Vec<Run> {
    let mut runs = Vec::new();
    let mut current: Vec<(&str, u32, u32)> = Vec::new();

    for (line_offset, line) in text.lines().enumerate() {
        let line_offset = u32::try_from(line_offset).unwrap_or(u32::MAX);
        let mut col: u32 = 0; // 1-based column of the next character
        let mut chars = line.char_indices().peekable();

        while let Some(&(byte_idx, ch)) = chars.peek() {
            col = col.saturating_add(1);
            if is_word_char(ch) {
                let word_start_byte = byte_idx;
                let word_start_col = col;
                let mut last_byte_end = byte_idx + ch.len_utf8();
                chars.next();
                while let Some(&(b, c)) = chars.peek() {
                    if is_word_char(c) {
                        col = col.saturating_add(1);
                        last_byte_end = b + c.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                let word = &line[word_start_byte..last_byte_end];
                if is_all_caps_word(word) {
                    current.push((word, line_offset, word_start_col));
                } else {
                    flush_run(&mut current, min_run, &mut runs);
                }
            } else {
                // Minor connectors keep the run alive but don't extend it.
                if !matches!(ch, ',' | ';' | ':' | '-' | ' ' | '\t') {
                    flush_run(&mut current, min_run, &mut runs);
                }
                chars.next();
            }
        }
        // End of line: minor connectors no longer apply, force a flush
        // before crossing into the next line so multi-line runs remain
        // single-line locations.
        flush_run(&mut current, min_run, &mut runs);
    }

    flush_run(&mut current, min_run, &mut runs);
    runs
}

fn flush_run(current: &mut Vec<(&str, u32, u32)>, min_run: u32, runs: &mut Vec<Run>) {
    let count = u32::try_from(current.len()).unwrap_or(u32::MAX);
    if count >= min_run {
        let first = current.first().copied().expect("len >= min_run >= 1");
        let last = current.last().copied().expect("len >= min_run >= 1");
        let span_text = format!(
            "{}…{}",
            first.0,
            // Just include the last word for context.
            last.0
        );
        runs.push(Run {
            text: span_text,
            word_count: count,
            line_offset: first.1,
            column: first.2,
        });
    }
    current.clear();
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\''
}

fn is_all_caps_word(word: &str) -> bool {
    let mut letters = 0u32;
    let mut has_lower = false;
    for c in word.chars() {
        if c.is_alphabetic() {
            letters += 1;
            if c.is_lowercase() {
                has_lower = true;
                break;
            }
        }
    }
    !has_lower && letters >= 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        AllCapsShouting::for_profile(profile).check(&document, Language::En)
    }

    fn lint_md(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        AllCapsShouting::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(AllCapsShouting::ID, "lexicon.all-caps-shouting");
    }

    #[test]
    fn tags_carry_a11y_markup_dyslexia_general() {
        assert!(AllCapsShouting::TAGS.contains(&ConditionTag::A11yMarkup));
        assert!(AllCapsShouting::TAGS.contains(&ConditionTag::Dyslexia));
        assert!(AllCapsShouting::TAGS.contains(&ConditionTag::General));
    }

    #[test]
    fn category_is_lexicon() {
        let text = "DO NOT TOUCH the wires.";
        let diags = lint(text, Profile::Public);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].category(), Category::Lexicon);
    }

    #[test]
    fn single_all_caps_word_is_an_abbreviation_not_shouting() {
        // A bare WCAG is the responsibility of `unexplained-abbreviation`.
        assert!(lint("The WCAG standard helps.", Profile::Public).is_empty());
    }

    #[test]
    fn two_consecutive_all_caps_words_trigger_under_public() {
        let diags = lint("Please DO NOT touch this.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("2 consecutive"));
    }

    #[test]
    fn dev_doc_tolerates_a_two_word_run() {
        // DevDoc min_run = 3, so DO NOT (2) is fine but DO NOT EVER (3)
        // is flagged.
        assert!(lint("Please DO NOT touch this.", Profile::DevDoc).is_empty());
        let diags = lint("Please DO NOT EVER touch this.", Profile::DevDoc);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn falc_flags_two_word_runs() {
        let diags = lint("Please DO NOT touch this.", Profile::Falc);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn all_caps_run_with_minor_punctuation_stays_together() {
        // Comma should not break the run.
        let diags = lint("Please READ ME, THEN ACT.", Profile::Public);
        // READ ME (2) — flagged. THEN ACT (2) — flagged. Two runs.
        // Note: the comma followed by space then THEN ACT is a separate run
        // because the comma keeps the run alive but the lowercase word
        // would break it. Here there is no lowercase between, so it is
        // one continuous run of 4.
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("4 consecutive"));
    }

    #[test]
    fn lowercase_word_breaks_the_run() {
        let diags = lint("DO and NOT.", Profile::Public);
        assert!(diags.is_empty());
    }

    #[test]
    fn period_breaks_the_run() {
        let diags = lint("Please DO. NOT acceptable.", Profile::Public);
        // After the period, "NOT" stands alone — single word, treated as an
        // abbreviation candidate.
        assert!(diags.is_empty());
    }

    #[test]
    fn acronym_followed_by_real_word_does_not_count() {
        // "WCAG covers" — only one all-caps word.
        assert!(lint("WCAG covers many cases.", Profile::Public).is_empty());
    }

    #[test]
    fn two_acronyms_separated_by_a_word_do_not_form_a_run() {
        assert!(lint("WCAG and ARIA help readers.", Profile::Public).is_empty());
    }

    #[test]
    fn fenced_code_block_content_is_ignored() {
        let md = "Intro paragraph.\n\n```\nDO NOT TOUCH\n```\n\nMore prose.\n";
        assert!(lint_md(md, Profile::Public).is_empty());
    }

    #[test]
    fn multiword_acronym_chain_in_plain_prose_triggers() {
        // A chain like "API HTTP TLS" is structurally indistinguishable from
        // shouting; rely on `unexplained-abbreviation` for the per-acronym
        // signal but flag the run for legibility.
        let diags = lint("Configure API HTTP TLS routing.", Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).min_run_length.get(), 3);
        assert_eq!(Config::for_profile(Profile::Public).min_run_length.get(), 2);
        assert_eq!(Config::for_profile(Profile::Falc).min_run_length.get(), 2);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Short and clean. Please DO NOT touch this. Fine again.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
