//! Rule: `syntax.parenthetical-depth`.
//!
//! Flags sentences whose maximum balanced-bracket nesting depth across
//! `()` and `[]` reaches the profile threshold. Stacked parentheticals
//! force the reader to track multiple suspended frames at once — a
//! recognised "hard sentence" signal in the plainlanguage.gov and
//! Hemingway editing traditions, and a particular cost for ADHD
//! readers, who carry the working-memory load first.
//!
//! Cohort sibling of [F49](../../../ROADMAP.md#f49)
//! (`structure.italic-span-long`) under the
//! [F-experimental-rule-status](../../../ROADMAP.md#f-experimental-rule-status)
//! substrate. Ships as [`Status::Experimental`] in v0.2.x; flips to
//! `Stable` at the v0.3 cut as part of the cohort flip.
//!
//! ## Boundary with [F22](../../../ROADMAP.md#f22) `structure.excessive-commas`
//!
//! `excessive-commas` already discounts flat `(A, B, C)` enumerations
//! at depth 1 via
//! [`crate::rules::enumeration::parenthesised_list_comma_count`]. This
//! rule fires only when nesting depth reaches 2 or more, so the two
//! rules are mechanically orthogonal: one flat parenthesised list never
//! trips this rule.
//!
//! ## Algorithm
//!
//! Sentence-scoped, language-agnostic, single byte-pass:
//!
//! 1. Walk the sentence one `char` at a time.
//! 2. Increment a running depth on `(` or `[`; decrement on `)` or `]`.
//! 3. A close that would push depth below zero resets the depth to
//!    zero (mirrors the fail-open posture of
//!    [`crate::rules::enumeration::parenthesised_list_comma_count`] on
//!    unbalanced parens).
//! 4. Track the maximum depth reached and the byte offset of the
//!    opener that achieved it.
//! 5. After the walk, emit one diagnostic per sentence iff
//!    `max_depth >= profile threshold`.
//!
//! ## Skips (false-positive guards)
//!
//! - **Code spans / fenced code blocks**: already excluded upstream by
//!   the Markdown parser (their content does not appear in
//!   `Paragraph.text`).
//! - **Unbalanced brackets**: ignored by the depth-floor reset; the
//!   rule cannot crash on partial markup.
//!
//! ## Deferred (not in MVP)
//!
//! Em-dash pairs (`— … —`), curly braces (`{}`), and comma-flanked
//! appositives are intentionally out of scope. Em-dash pair detection
//! is fragile (en/em-dash confusion, hyphen ambiguity) and would
//! smuggle scope back in. Filed as `F-syntax-appositive-depth` if
//! dogfood demands it.
//!
//! See [`RULES.md`](../../RULES.md) for the reference entry.

use std::num::NonZeroU32;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::{Rule, Status};
use crate::types::{Diagnostic, Language, Location, Severity};

/// Configuration for [`ParentheticalDepth`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Sentence-level maximum nesting depth at which the rule fires.
    pub max_depth: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let depth = match profile {
            // Tech docs tolerate deeper nesting (citations, qualifiers,
            // type signatures rendered in prose). 4 is the loosest tier.
            Profile::DevDoc => 4,
            // General-audience docs: 3 stacked frames is the upper
            // limit before parsing cost spikes for typical readers.
            Profile::Public => 3,
            // Plain-language audience: 2 is already a stack of two
            // suspended thoughts. FALC asks for one idea per sentence.
            Profile::Falc => 2,
        };
        Self {
            max_depth: NonZeroU32::new(depth).expect("non-zero literal"),
        }
    }
}

/// The [`ParentheticalDepth`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ParentheticalDepth {
    config: Config,
}

impl ParentheticalDepth {
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
    pub const ID: &'static str = "syntax.parenthetical-depth";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[ConditionTag::Adhd, ConditionTag::General];
}

impl Rule for ParentheticalDepth {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        // ADHD readers carry the working-memory cost of stacked
        // parentheticals first; `general` mirrors plainlanguage.gov +
        // Hemingway scope — every reader benefits from shallow nesting,
        // not only ADHD readers.
        Self::TAGS
    }

    fn status(&self) -> Status {
        // Cohort sibling of F49 — flips to Stable at the v0.3 cut as
        // part of the F-experimental-rule-status cohort flip.
        Status::Experimental
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let threshold = self.config.max_depth.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
            for sentence in sentences {
                let Some(scan) = scan_max_depth(&sentence.text) else {
                    continue;
                };
                if scan.max_depth < threshold {
                    continue;
                }
                let column = sentence
                    .column
                    .saturating_add(u32::try_from(scan.max_opener_char_offset).unwrap_or(u32::MAX));
                let location = Location::new(document.source.clone(), sentence.line, column, 1);
                let message = format!(
                    "Nested parentheticals reach depth {}; readers must hold {} suspended \
                     thoughts to reach the close. Split the sentence or unnest the inner \
                     bracket (plainlanguage.gov, Hemingway).",
                    scan.max_depth, scan.max_depth,
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

/// Outcome of one sentence-level depth scan.
#[derive(Debug, PartialEq, Eq)]
struct DepthScan {
    /// Maximum depth reached anywhere in the sentence.
    max_depth: u32,
    /// 0-based char offset of the opener that achieved `max_depth`.
    /// Reported in the diagnostic so the reader sees the exact start
    /// of the deepest frame.
    max_opener_char_offset: usize,
}

/// Scan a sentence for nested-bracket depth.
///
/// Returns `None` when the sentence is empty or contains no brackets
/// (the caller skips emit). Returns `Some(scan)` otherwise; the caller
/// compares `scan.max_depth` to its profile threshold.
///
/// Treats `(` and `[` as openers and `)` and `]` as closers,
/// language-agnostic. A close that would push depth below zero
/// resets the running depth to zero (fail-open on unbalanced
/// brackets).
fn scan_max_depth(sentence: &str) -> Option<DepthScan> {
    let mut depth: u32 = 0;
    let mut max_depth: u32 = 0;
    let mut max_opener_char_offset: usize = 0;
    let mut saw_bracket = false;

    for (char_offset, ch) in sentence.chars().enumerate() {
        match ch {
            '(' | '[' => {
                saw_bracket = true;
                depth = depth.saturating_add(1);
                if depth > max_depth {
                    max_depth = depth;
                    max_opener_char_offset = char_offset;
                }
            },
            ')' | ']' => {
                saw_bracket = true;
                depth = depth.saturating_sub(1);
            },
            _ => {},
        }
    }

    if !saw_bracket {
        return None;
    }
    Some(DepthScan {
        max_depth,
        max_opener_char_offset,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        ParentheticalDepth::for_profile(profile).check(&document, Language::En)
    }

    fn lint_md(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        ParentheticalDepth::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(ParentheticalDepth::ID, "syntax.parenthetical-depth");
    }

    #[test]
    fn carries_adhd_and_general_condition_tags() {
        let rule = ParentheticalDepth::for_profile(Profile::Public);
        assert_eq!(
            rule.condition_tags(),
            &[ConditionTag::Adhd, ConditionTag::General]
        );
    }

    #[test]
    fn ships_as_experimental() {
        let rule = ParentheticalDepth::for_profile(Profile::Public);
        assert_eq!(rule.status(), Status::Experimental);
    }

    #[test]
    fn category_is_syntax() {
        // Depth 3 trips Public (threshold 3).
        let diags = lint("a (b (c (d) e) f) g.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].category(), Category::Syntax);
    }

    #[test]
    fn flat_parenthetical_does_not_trigger() {
        // Depth 1 throughout — F22 territory, F57 stays silent.
        assert!(lint("Note (a, b, c) here.", Profile::Public).is_empty());
        assert!(lint("Note (a, b, c) here.", Profile::Falc).is_empty());
    }

    #[test]
    fn no_brackets_does_not_trigger() {
        assert!(lint("Plain prose, no brackets at all.", Profile::Falc).is_empty());
    }

    #[test]
    fn depth_two_trips_falc_only() {
        // Depth exactly 2: triggers FALC (threshold 2), passes Public
        // (threshold 3) and DevDoc (threshold 4).
        let text = "Note (an aside (parenthetical) here) ends.";
        assert_eq!(lint(text, Profile::Falc).len(), 1);
        assert!(lint(text, Profile::Public).is_empty());
        assert!(lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn depth_three_trips_public_not_devdoc() {
        // Depth exactly 3.
        let text = "a (b (c (d) e) f) g.";
        assert!(!lint(text, Profile::Falc).is_empty());
        assert!(!lint(text, Profile::Public).is_empty());
        assert!(lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn depth_four_trips_devdoc() {
        // Depth exactly 4: triggers every profile, including DevDoc.
        let text = "a (b (c (d (e) f) g) h) i.";
        assert!(!lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn message_names_the_depth_reached() {
        let diags = lint("a (b (c (d) e) f) g.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(
            diags[0].message.contains("depth 3"),
            "expected depth 3 in message, got: {}",
            diags[0].message
        );
    }

    #[test]
    fn mixed_paren_and_square_bracket_count_as_one_family() {
        // ( … [ … ] … ) reaches depth 2 — flagged under FALC.
        let text = "Note (see [tracker] here) ends.";
        assert_eq!(lint(text, Profile::Falc).len(), 1);
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn unbalanced_close_does_not_panic_and_does_not_inflate_depth() {
        // Stray closes should not push depth into negative territory
        // and then make a later opener look "deep".
        let text = "stray ) close, then (a (b) c) end.";
        // Sentence reaches max depth 2 only — passes Public, trips FALC.
        assert!(lint(text, Profile::Public).is_empty());
        assert_eq!(lint(text, Profile::Falc).len(), 1);
    }

    #[test]
    fn unbalanced_open_does_not_panic() {
        // Open without matching close: depth goes up but the rule
        // still works. FALC threshold 2 — depth 2 reached, fires.
        let text = "open (a (b without close.";
        assert_eq!(lint(text, Profile::Falc).len(), 1);
    }

    #[test]
    fn deepest_opener_is_pointed_at() {
        // The diagnostic column should target the inner-most opener,
        // not the outer one.
        let text = "abc (de (fg) hi) jk.";
        let diags = lint(text, Profile::Falc);
        assert_eq!(diags.len(), 1);
        // "abc (de " is 8 chars before the inner '(' at char 8.
        // Sentence starts at column 1; opener is at sentence column
        // 1 + 8 = 9.
        assert_eq!(diags[0].location.column, 9);
    }

    #[test]
    fn one_diagnostic_per_sentence() {
        // Two sentences, each at depth 2 — two diags under FALC.
        let text = "Note (one (two) three) end. Then (alpha (beta) gamma) close.";
        let diags = lint(text, Profile::Falc);
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn fenced_code_block_content_is_ignored() {
        // Code fences are stripped by the Markdown parser before the
        // rule ever sees them.
        let md = "Intro.\n\n```\nfn f(g(h(i(j))))\n```\n\nMore prose.\n";
        assert!(lint_md(md, Profile::Falc).is_empty());
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        let dd = Config::for_profile(Profile::DevDoc);
        assert_eq!(dd.max_depth.get(), 4);
        let pub_ = Config::for_profile(Profile::Public);
        assert_eq!(pub_.max_depth.get(), 3);
        let fa = Config::for_profile(Profile::Falc);
        assert_eq!(fa.max_depth.get(), 2);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Note (an aside (parenthetical) here) ends.";
        let diags = lint(text, Profile::Falc);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
