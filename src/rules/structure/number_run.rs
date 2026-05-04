//! Rule: `structure.number-run`.
//!
//! Flags sentences that pack more than a configurable number of
//! numeric tokens together. plainlanguage.gov is explicit on the
//! framing — *"Don't put a lot of numbers together in one sentence"*
//! / *"Avoid placing too many statistics close together"* — and
//! readers with dyscalculia carry the cost first: each numeric token
//! forces a quantity-to-symbol re-anchoring that does not benefit
//! from the running prose context the same way ordinary words do.
//!
//! Cohort sibling of [F49](../../../ROADMAP.md#f49)
//! (`structure.italic-span-long`) under the
//! [F-experimental-rule-status](../../../ROADMAP.md#f-experimental-rule-status)
//! substrate. Ships as [`Status::Experimental`] in v0.2.x; flips to
//! `Stable` at the v0.3 cut as part of the cohort flip.
//!
//! ## Numeric-token definition
//!
//! A numeric token is a contiguous run of ASCII digits, optionally
//! containing one decimal separator (`.` or `,`) followed by more
//! digits. Hyphen, colon, slash, and whitespace split tokens.
//!
//! | Input | Token count | Reason |
//! |---|---|---|
//! | `42` | 1 | Bare integer |
//! | `3.14` | 1 | One decimal separator |
//! | `1,000` | 1 | One comma separator |
//! | `2026-05-04` | 3 | Hyphens split — a date *is* three numbers from a load standpoint |
//! | `$3.50` | 1 | Currency prefix is non-digit, ignored |
//! | `v1.2.3` | 1 | First decimal kept; second `.` is not followed by digits in a run that already used its separator (the scan stops at the second `.` because the `.2` sub-run already consumed one separator) |
//! | `1st` | 1 | Trailing letters are not part of the token but the digits still count |
//!
//! The rule walks the post-flattening `Paragraph.text` produced by
//! [`crate::parser::parse_markdown`], so fenced code blocks are
//! excluded. Inline code spans are part of paragraph text in v1; if
//! a corpus shows real friction (technical prose with constant
//! tables in inline code), an inline-code-stripping option can be
//! added later — YAGNI today.
//!
//! See [`RULES.md`](../../RULES.md#number-run) for the reference entry.

use std::num::NonZeroU32;

use unicode_segmentation::UnicodeSegmentation;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::{Rule, Status};
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`NumberRun`].
///
/// `max_numbers` is strongly typed with [`NonZeroU32`] to make
/// impossible states impossible: a zero threshold would flag every
/// sentence that contained a single number, including ordinary prose
/// like *"Page 12 covers the rationale."* — which is exactly the
/// legitimate use case plainlanguage.gov endorses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed numeric tokens per sentence.
    pub max_numbers: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 6,
            Profile::Public => 4,
            Profile::Falc => 3,
        };
        Self {
            max_numbers: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`NumberRun`] rule.
#[derive(Debug, Clone, Copy)]
pub struct NumberRun {
    config: Config,
}

impl NumberRun {
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
    pub const ID: &'static str = "structure.number-run";
}

impl Rule for NumberRun {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_numbers.get();
        let mut diags = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
            for sentence in sentences {
                let Some((count, first_offset)) = scan_numeric_run(&sentence.text, max) else {
                    continue;
                };
                diags.push(build_diagnostic(
                    &document.source,
                    &sentence.text,
                    sentence.line,
                    sentence.column,
                    first_offset,
                    count,
                    max,
                    section_title,
                ));
            }
        }
        diags
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        // plainlanguage.gov grounds the rule in the dyscalculia-load
        // case; gated under user-active conditions per F71/F72.
        &[ConditionTag::Dyscalculia]
    }

    fn status(&self) -> Status {
        // Cohort sibling of F49. Flips to Stable at the v0.3 cut as
        // part of the F-experimental-rule-status cohort flip.
        Status::Experimental
    }
}

/// Count numeric tokens in `text`. Returns `Some((count, byte_offset))`
/// pointing at the first numeric token when `count > max`, else `None`.
///
/// The byte offset is measured from the start of `text`; callers that
/// want a column convert via grapheme counting against the prefix.
fn scan_numeric_run(text: &str, max: u32) -> Option<(u32, usize)> {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut count: u32 = 0;
    let mut first_offset: Option<usize> = None;
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        if b.is_ascii_digit() {
            let start = i;
            i += 1;
            // One optional decimal separator inside a numeric token.
            let mut separator_used = false;
            while i < len {
                let c = bytes[i];
                if c.is_ascii_digit() {
                    i += 1;
                } else if !separator_used
                    && (c == b'.' || c == b',')
                    && i + 1 < len
                    && bytes[i + 1].is_ascii_digit()
                {
                    separator_used = true;
                    i += 2;
                } else {
                    break;
                }
            }
            count = count.saturating_add(1);
            if first_offset.is_none() {
                first_offset = Some(start);
            }
        } else {
            i += utf8_char_len(b);
        }
    }

    if count > max {
        first_offset.map(|off| (count, off))
    } else {
        None
    }
}

fn utf8_char_len(leading: u8) -> usize {
    if leading < 0x80 {
        1
    } else if leading < 0xC0 {
        // Continuation byte — should not appear as a leading byte on
        // valid UTF-8, but keep the scanner forward-progressing.
        1
    } else if leading < 0xE0 {
        2
    } else if leading < 0xF0 {
        3
    } else {
        4
    }
}

fn build_diagnostic(
    source: &SourceFile,
    sentence_text: &str,
    sentence_line: u32,
    sentence_column: u32,
    first_offset: usize,
    actual: u32,
    max: u32,
    section: Option<&str>,
) -> Diagnostic {
    // Convert the first-token byte offset into a grapheme-column
    // delta against the sentence's anchor column.
    let prefix = &sentence_text[..first_offset];
    let prefix_graphemes = u32::try_from(prefix.graphemes(true).count()).unwrap_or(u32::MAX);
    let column = sentence_column.saturating_add(prefix_graphemes);
    let length = u32::try_from(sentence_text.graphemes(true).count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), sentence_line, column, length);
    let message = format!(
        "Sentence packs {actual} numeric tokens (maximum {max}). plain-language guidance \
         recommends not placing many numbers or statistics together in one sentence; \
         split the sentence or move some figures to a list or table."
    );
    let diag = Diagnostic::new(NumberRun::ID, Severity::Warning, location, message);
    match section {
        Some(title) => diag.with_section(title),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        NumberRun::for_profile(profile).check(&document, Language::En)
    }

    fn lint_md(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        NumberRun::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case_and_category_prefixed() {
        assert_eq!(NumberRun::ID, "structure.number-run");
        assert_eq!(
            NumberRun::for_profile(Profile::Public).id(),
            "structure.number-run"
        );
    }

    #[test]
    fn ships_as_experimental() {
        // Cohort sibling of F49 — flips to Stable at v0.3 cut.
        assert_eq!(
            NumberRun::for_profile(Profile::Public).status(),
            Status::Experimental
        );
    }

    #[test]
    fn carries_dyscalculia_condition_tag() {
        let rule = NumberRun::for_profile(Profile::Public);
        assert_eq!(rule.condition_tags(), &[ConditionTag::Dyscalculia]);
    }

    #[test]
    fn category_is_structure() {
        // Public threshold = 4 — five bare integers fire.
        let diags = lint("Counts hit 1, 2, 3, 4, 5 across reviews.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].category(), Category::Structure);
    }

    #[test]
    fn sentence_under_threshold_does_not_trigger() {
        // 3 numbers under Public's 4.
        let diags = lint("Counts hit 1, 2, and 3 across reviews.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn sentence_at_threshold_does_not_trigger() {
        // 4 numbers at Public's 4 (boundary inclusive, like F49).
        let diags = lint("Counts hit 1, 2, 3, and 4 across reviews.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn sentence_over_threshold_triggers() {
        let diags = lint("Counts hit 1, 2, 3, 4, 5 across reviews.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, NumberRun::ID);
        assert_eq!(diags[0].severity, Severity::Warning);
        assert!(diags[0].message.contains("5 numeric tokens"));
        assert!(diags[0].message.contains("maximum 4"));
    }

    #[test]
    fn decimal_is_one_token() {
        // Public threshold = 4. `3.14 and 2.71 and 1.41 and 1.61 and 0.57`
        // is 5 decimal tokens — fires.
        let diags = lint(
            "Constants include 3.14, 2.71, 1.41, 1.61, and 0.57 across the table.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1, "got {diags:?}");
        assert!(diags[0].message.contains("5 numeric tokens"));
    }

    #[test]
    fn three_decimals_under_threshold() {
        // 3 decimal tokens under Public's 4.
        let diags = lint(
            "Constants 3.14, 2.71, and 1.41 are common.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn hyphenated_date_is_three_tokens() {
        // `2026-05-04` splits into three numeric tokens. With one
        // additional bare number we hit Public's threshold of 4 and
        // do not fire; with two additional we exceed and fire.
        let under = lint(
            "Released on 2026-05-04 with 1 patch attached.",
            Profile::Public,
        );
        assert!(under.is_empty(), "got {under:?}");
        let over = lint(
            "Released on 2026-05-04 with 1 patch and 2 hotfixes.",
            Profile::Public,
        );
        assert_eq!(over.len(), 1);
        assert!(over[0].message.contains("5 numeric tokens"));
    }

    #[test]
    fn fenced_code_block_excluded() {
        // Code blocks are dropped by the markdown parser before the
        // rule sees the paragraph buffer.
        let md = "Plain prose intro.\n\n\
                  ```\n\
                  vals = [1, 2, 3, 4, 5, 6, 7, 8]\n\
                  ```\n\n\
                  Plain prose outro.";
        assert!(lint_md(md, Profile::Public).is_empty());
    }

    #[test]
    fn devdoc_profile_is_more_tolerant() {
        // 5 numbers fires on Public (4) but not on DevDoc (6).
        let text = "Counts hit 1, 2, 3, 4, 5 across reviews.";
        assert!(!lint(text, Profile::Public).is_empty());
        assert!(lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 4 numbers passes Public (4) but fires FALC (3).
        let text = "Counts hit 1, 2, 3, and 4 across reviews.";
        assert!(lint(text, Profile::Public).is_empty());
        assert!(!lint(text, Profile::Falc).is_empty());
    }

    #[test]
    fn french_input_is_caught_too() {
        // Detection is language-agnostic — digits are universal.
        let diags = lint(
            "Les comptages atteignent 1, 2, 3, 4, 5 selon les revues.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1, "got {diags:?}");
    }

    #[test]
    fn position_points_at_first_numeric_token() {
        // "Counts hit " is 11 chars (graphemes), so first digit is at
        // column 12 (1-based).
        let diags = lint("Counts hit 1, 2, 3, 4, 5 across reviews.", Profile::Public);
        assert_eq!(diags[0].location.line, 1);
        assert_eq!(diags[0].location.column, 12);
        assert!(diags[0].location.length > 0);
    }

    #[test]
    fn multiple_offending_sentences_each_fire() {
        let text = "First batch was 1, 2, 3, 4, 5 across reviews. \
                    Second batch was 6, 7, 8, 9, 10 across audits.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 2, "got {diags:?}");
    }

    #[test]
    fn citation_salad_fires() {
        // Citation clusters are a real dyscalculic-load pattern; the
        // rule is correct to fire here.
        let diags = lint(
            "See work by Smith 2020, Jones 2021, Wei 2022, Park 2023, and Lee 2024.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1, "got {diags:?}");
    }

    #[test]
    fn sentence_with_no_numbers_does_not_trigger() {
        let diags = lint(
            "The team eventually decided to ship the migration on schedule.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_numbers.get(), 6);
        assert_eq!(Config::for_profile(Profile::Public).max_numbers.get(), 4);
        assert_eq!(Config::for_profile(Profile::Falc).max_numbers.get(), 3);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Mild paragraph mentions 1 and 2 figures only.\n\n\
                    Heavy paragraph hits 1, 2, 3, 4, 5 numbers in a row.\n\n\
                    Plain prose without any digits at all here.";
        let document = parse_markdown(text, SourceFile::Anonymous);
        let diags = NumberRun::for_profile(Profile::Public).check(&document, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
