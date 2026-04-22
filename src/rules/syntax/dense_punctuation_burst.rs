//! Rule: `dense-punctuation-burst`.
//!
//! Flags *local* bursts of punctuation: ≥ N marks (`,`, `;`, `:`, `—`,
//! `–`) within a sliding window of W grapheme clusters. The IFLA
//! easy-to-read guidelines recommend simple punctuation; tightly
//! clustered marks signal layered subordination, parenthetical
//! interjections, or list-within-list constructions that are hard to
//! parse for readers with cognitive or attentional difficulties.
//!
//! Distinct from [`crate::rules::ExcessiveCommas`], which counts
//! commas across an entire sentence. This rule fires on *local
//! density*: a sentence with 8 commas spread evenly across 200 chars
//! does not trigger here, while a sentence with 3 commas inside one
//! 30-char span does.
//!
//! See [`RULES.md`](../../RULES.md#dense-punctuation-burst) for the
//! reference entry.

use std::num::NonZeroU32;

use unicode_segmentation::UnicodeSegmentation;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity};

/// Configuration for [`DensePunctuationBurst`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Minimum number of qualifying punctuation marks within
    /// [`window_graphemes`](Self::window_graphemes) to constitute a
    /// burst.
    pub min_marks: NonZeroU32,

    /// Width (in grapheme clusters) of the sliding window.
    pub window_graphemes: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let (min, window) = match profile {
            // Tech docs tolerate denser punctuation in lists and code-
            // adjacent prose; require 4 marks in a 30-grapheme window.
            Profile::DevDoc => (4, 30),
            Profile::Public => (3, 30),
            // Plain-language audience: same density requirement, wider
            // window — a 3-mark burst in 40 chars still trips.
            Profile::Falc => (3, 40),
        };
        Self {
            min_marks: NonZeroU32::new(min).expect("non-zero literal"),
            window_graphemes: NonZeroU32::new(window).expect("non-zero literal"),
        }
    }
}

/// The [`DensePunctuationBurst`] rule.
#[derive(Debug, Clone, Copy)]
pub struct DensePunctuationBurst {
    config: Config,
}

impl DensePunctuationBurst {
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
    pub const ID: &'static str = "syntax.dense-punctuation-burst";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[ConditionTag::General];
}

impl Rule for DensePunctuationBurst {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let min = self.config.min_marks.get() as usize;
        let window = self.config.window_graphemes.get() as usize;
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            for (line_offset, line) in paragraph.text.lines().enumerate() {
                for burst in find_bursts(line, min, window) {
                    let line_number = paragraph
                        .start_line
                        .saturating_add(u32::try_from(line_offset).unwrap_or(u32::MAX));
                    let column = u32::try_from(burst.start_column).unwrap_or(u32::MAX);
                    let length = u32::try_from(burst.length).unwrap_or(u32::MAX);
                    let location =
                        Location::new(document.source.clone(), line_number, column, length);
                    let message = format!(
                        "{} punctuation marks within {} characters create a dense burst that is \
                         hard to parse. Split the clause, drop the parenthetical, or rewrite as \
                         a list (IFLA easy-to-read guidelines).",
                        burst.mark_count, burst.length
                    );
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

#[derive(Debug, PartialEq, Eq)]
struct Burst {
    /// 1-based grapheme column where the burst starts (the first mark).
    start_column: usize,
    /// Span length, in graphemes, from the first to the last mark
    /// inclusive.
    length: usize,
    /// Number of qualifying marks inside the span.
    mark_count: usize,
}

/// Scan one line for punctuation bursts.
///
/// Walks the grapheme stream once, collecting the column of every
/// qualifying mark. Whenever a sliding window of `window` graphemes
/// holds `min` or more marks, emit a burst spanning the first to the
/// last mark in the window, then advance past that last mark so
/// overlapping windows do not double-fire on the same cluster.
fn find_bursts(line: &str, min: usize, window: usize) -> Vec<Burst> {
    let mut bursts = Vec::new();
    let columns: Vec<usize> = line
        .graphemes(true)
        .enumerate()
        .filter_map(|(idx, g)| {
            if is_qualifying_mark(g) {
                Some(idx + 1) // 1-based grapheme column
            } else {
                None
            }
        })
        .collect();

    if columns.len() < min {
        return bursts;
    }

    let mut start = 0;
    while start + min - 1 < columns.len() {
        let end = start + min - 1;
        let span = columns[end] - columns[start] + 1;
        if span <= window {
            // Greedy extend: pull in any further marks that still fit
            // inside `window` graphemes from `columns[start]`.
            let mut last = end;
            while last + 1 < columns.len() && columns[last + 1] - columns[start] < window {
                last += 1;
            }
            bursts.push(Burst {
                start_column: columns[start],
                length: columns[last] - columns[start] + 1,
                mark_count: last - start + 1,
            });
            start = last + 1;
        } else {
            start += 1;
        }
    }

    bursts
}

fn is_qualifying_mark(grapheme: &str) -> bool {
    matches!(grapheme, "," | ";" | ":" | "—" | "–")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        DensePunctuationBurst::for_profile(profile).check(&document, Language::En)
    }

    fn lint_md(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        DensePunctuationBurst::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(DensePunctuationBurst::ID, "syntax.dense-punctuation-burst");
    }

    #[test]
    fn tag_is_general() {
        assert_eq!(DensePunctuationBurst::TAGS, &[ConditionTag::General]);
    }

    #[test]
    fn category_is_syntax() {
        let diags = lint("a, b; c: done.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].category(), Category::Syntax);
    }

    #[test]
    fn three_marks_in_short_span_triggers_under_public() {
        // `, ; :` over 8 graphemes → 3 marks within 8 ≤ 30, triggers.
        let diags = lint("a, b; c: done.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 punctuation marks"));
    }

    #[test]
    fn two_marks_do_not_trigger_under_public() {
        // 2 marks total — below the 3-mark floor.
        assert!(lint("a, b; done.", Profile::Public).is_empty());
    }

    #[test]
    fn marks_spread_across_long_span_do_not_trigger() {
        // 3 marks spread over ~80 chars: each pair > 30 graphemes apart,
        // so no qualifying window contains 3.
        let text = "First clause runs long, second clause also runs long; \
                    third clause finally arrives: done.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn dev_doc_is_more_tolerant() {
        // 3 marks in 8 chars: triggers Public, passes DevDoc (needs 4).
        let text = "a, b; c: done.";
        assert!(!lint(text, Profile::Public).is_empty());
        assert!(lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn dev_doc_triggers_on_four_marks_in_window() {
        let diags = lint("a, b; c: d, done.", Profile::DevDoc);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("4 punctuation marks"));
    }

    #[test]
    fn falc_window_is_wider_than_public() {
        // Commas at grapheme columns 4, 22, 39 → 36-char span. Public
        // window (30) misses; FALC window (40) catches.
        let text = "abc,defghijklmnopqrstu,vwxyzabcdefghijkl,end.";
        assert!(lint(text, Profile::Public).is_empty());
        assert!(!lint(text, Profile::Falc).is_empty());
    }

    #[test]
    fn em_dash_and_en_dash_qualify() {
        let diags = lint("clause, sub — sub – tail.", Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn period_does_not_qualify() {
        // Three sentence terminators are not a punctuation burst.
        assert!(lint("Done. Stop. End.", Profile::Public).is_empty());
    }

    #[test]
    fn parenthesis_does_not_qualify() {
        // Brackets/parentheses are not in the qualifying set.
        assert!(lint("(a) (b) (c)", Profile::Public).is_empty());
    }

    #[test]
    fn each_burst_emits_one_diagnostic_no_overlap() {
        // Two distinct bursts, each fully contained in its own region.
        // Padding chosen so the second burst starts > 30 graphemes after
        // the end of the first.
        let text = "a, b; c: done. ____________________________________ then x, y; z: end.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn fenced_code_block_content_is_ignored() {
        let md = "Intro.\n\n```\nfn f(a, b, c, d) {}\n```\n\nMore prose.\n";
        assert!(lint_md(md, Profile::Public).is_empty());
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        let dd = Config::for_profile(Profile::DevDoc);
        assert_eq!(dd.min_marks.get(), 4);
        assert_eq!(dd.window_graphemes.get(), 30);
        let pub_ = Config::for_profile(Profile::Public);
        assert_eq!(pub_.min_marks.get(), 3);
        assert_eq!(pub_.window_graphemes.get(), 30);
        let fa = Config::for_profile(Profile::Falc);
        assert_eq!(fa.min_marks.get(), 3);
        assert_eq!(fa.window_graphemes.get(), 40);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Short, dense; burst: here. Then a clean sentence.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
