//! Rule: `structure.italic-span-long`.
//!
//! Flags italic spans (`*…*` / `_…_`) whose word count exceeds a
//! configurable threshold. Long italic runs degrade letter-shape
//! recognition for dyslexic readers — slanted glyphs are harder to
//! parse than upright ones, and the British Dyslexia Association
//! style guide explicitly recommends keeping italic emphasis to a
//! short phrase.
//!
//! Cohort lead for the v0.3 condition-tag rules cohort
//! ([F46](../../../ROADMAP.md#f46) / [F49](../../../ROADMAP.md#f49) /
//! [F51](../../../ROADMAP.md#f51) / [F53](../../../ROADMAP.md#f53) /
//! [F57](../../../ROADMAP.md#f57)). Ships as
//! [`Status::Experimental`](crate::rules::Status::Experimental) in
//! v0.2.x via the [F139](../../../ROADMAP.md#f139) substrate; flips
//! to `Stable` at the v0.3 cut.
//!
//! Reads `Paragraph.inline` (the [F143](../../../ROADMAP.md#f143)
//! inline-AST substrate). The lazy-build contract — empty inline
//! tree means "no spans worth modeling" — short-circuits the
//! rule on the common case (paragraphs without emphasis).

use std::num::NonZeroU32;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::parser::{word_count, Document, EmphasisSpan, Inline};
use crate::rules::{Rule, Status};
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`ItalicSpanLong`].
///
/// Threshold is strongly typed with [`NonZeroU32`] to make impossible
/// states impossible: a zero threshold would flag every italic span,
/// including single-word emphasis which is precisely the legitimate
/// use case the BDA style guide endorses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed word count per italic span.
    pub max_words: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 12,
            Profile::Public => 8,
            Profile::Falc => 5,
        };
        Self {
            max_words: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`ItalicSpanLong`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ItalicSpanLong {
    config: Config,
}

impl ItalicSpanLong {
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
    pub const ID: &'static str = "structure.italic-span-long";
}

impl Rule for ItalicSpanLong {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_words.get();
        let mut diags = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            // F143 lazy-build contract: empty inline = no spans to
            // model. Short-circuit before walking nodes.
            if paragraph.inline.is_empty() {
                continue;
            }
            for node in &paragraph.inline {
                check_node(node, section_title, &document.source, &mut diags, max);
            }
        }
        diags
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        // Dyslexia-targeted: BDA style guide is the grounding.
        &[ConditionTag::Dyslexia]
    }

    fn status(&self) -> Status {
        // Experimental in v0.2.x; flips to Stable at v0.3 cut as
        // part of the F139 cohort flip.
        Status::Experimental
    }
}

fn check_node(
    node: &Inline,
    section: Option<&str>,
    source: &SourceFile,
    out: &mut Vec<Diagnostic>,
    max: u32,
) {
    let Inline::Emphasis(span) = node else {
        return;
    };
    let span_text = flatten_emphasis(span);
    let count = word_count(&span_text);
    if count > max {
        out.push(build_diagnostic(
            span, &span_text, count, max, section, source,
        ));
    }
    // Nested emphasis is rare but supported by the substrate; recurse
    // into children so a long inner span is also reported on its own
    // line. (The outer span's count includes inner words.)
    for child in &span.children {
        check_node(child, section, source, out, max);
    }
}

fn flatten_emphasis(span: &EmphasisSpan) -> String {
    let mut s = String::new();
    flatten_into(&span.children, &mut s);
    s
}

fn flatten_into(nodes: &[Inline], out: &mut String) {
    for n in nodes {
        match n {
            Inline::Text(t) => out.push_str(t),
            Inline::Emphasis(span) => flatten_into(&span.children, out),
        }
    }
}

fn build_diagnostic(
    span: &EmphasisSpan,
    span_text: &str,
    actual: u32,
    max: u32,
    section: Option<&str>,
    source: &SourceFile,
) -> Diagnostic {
    let length = u32::try_from(span_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), span.start_line, span.start_column, length);
    let message = format!(
        "Italic span is {actual} words long (maximum {max}). Long italic runs strain dyslexic readers; consider shortening the emphasized phrase or removing the italics."
    );
    let diag = Diagnostic::new(ItalicSpanLong::ID, Severity::Warning, location, message);
    match section {
        Some(title) => diag.with_section(title),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_markdown;

    fn lint(md: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_markdown(md, SourceFile::Anonymous);
        ItalicSpanLong::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case_and_category_prefixed() {
        assert_eq!(ItalicSpanLong::ID, "structure.italic-span-long");
        assert_eq!(
            ItalicSpanLong::for_profile(Profile::Public).id(),
            "structure.italic-span-long"
        );
    }

    #[test]
    fn ships_as_experimental() {
        // Cohort lead — flips to Stable at v0.3 cut, see F139.
        assert_eq!(
            ItalicSpanLong::for_profile(Profile::Public).status(),
            Status::Experimental
        );
    }

    #[test]
    fn carries_dyslexia_condition_tag() {
        // BDA grounding is the rule's reason for existing; the tag
        // gates it under user-active conditions per F71/F72.
        let rule = ItalicSpanLong::for_profile(Profile::Public);
        assert_eq!(rule.condition_tags(), &[ConditionTag::Dyslexia]);
    }

    #[test]
    fn short_italic_span_does_not_trigger() {
        let diags = lint("Some *italic* in middle.", Profile::Public);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn span_at_threshold_does_not_trigger() {
        // Public threshold = 8; an 8-word span is allowed.
        let diags = lint(
            "Some *one two three four five six seven eight* end.",
            Profile::Public,
        );
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn span_over_threshold_triggers() {
        // 10-word span exceeds Public's 8-word threshold.
        let diags = lint(
            "Some *one two three four five six seven eight nine ten* end.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, ItalicSpanLong::ID);
        assert_eq!(diags[0].severity, Severity::Warning);
        assert!(diags[0].message.contains("10 words long"));
        assert!(diags[0].message.contains("maximum 8"));
    }

    #[test]
    fn diagnostic_points_at_opening_delimiter() {
        // Position is sourced from the F143 EmphasisSpan, which
        // anchors at the opening `*`. "Some " is 5 chars → col 6.
        let diags = lint(
            "Some *one two three four five six seven eight nine ten* end.",
            Profile::Public,
        );
        assert_eq!(diags[0].location.line, 1);
        assert_eq!(diags[0].location.column, 6);
        assert!(diags[0].location.length > 0);
    }

    #[test]
    fn devdoc_profile_is_more_tolerant() {
        // 10-word span fires on Public (8) but not on DevDoc (12).
        let md = "Some *one two three four five six seven eight nine ten* end.";
        assert!(!lint(md, Profile::Public).is_empty());
        assert!(lint(md, Profile::DevDoc).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 6-word span passes Public (8) but fails FALC (5).
        let md = "Some *one two three four five six* end.";
        assert!(lint(md, Profile::Public).is_empty());
        assert!(!lint(md, Profile::Falc).is_empty());
    }

    #[test]
    fn underscore_emphasis_is_also_caught() {
        // Both Markdown emphasis delimiters route through the same
        // `Tag::Emphasis` event; the rule must catch both.
        let diags = lint(
            "Some _one two three four five six seven eight nine ten_ end.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn strong_does_not_trigger() {
        // **bold** flattens into Text in the inline tree (F143's
        // narrow-by-design contract); the rule sees no Emphasis,
        // produces no diagnostic.
        let md = "Some **one two three four five six seven eight nine ten** here.";
        assert!(lint(md, Profile::Public).is_empty());
    }

    #[test]
    fn paragraph_without_emphasis_short_circuits() {
        // F143 lazy-build: emphasis-free paragraphs ship with empty
        // `inline`. The rule's short-circuit branch is exercised by
        // any input where no `*` / `_` ever fires.
        let md = "Plain prose with no italic at all, just regular text and words running on for many many words longer than any threshold.";
        assert!(lint(md, Profile::Public).is_empty());
    }

    #[test]
    fn multiple_long_spans_each_fire() {
        let md = "First *one two three four five six seven eight nine ten* and \
                  second *eleven twelve thirteen fourteen fifteen sixteen seventeen \
                  eighteen nineteen twenty* end.";
        let diags = lint(md, Profile::Public);
        assert_eq!(diags.len(), 2, "got {diags:?}");
    }

    #[test]
    fn french_input_is_caught_too() {
        // Substrate is language-agnostic; tokenizer counts FR words
        // the same way it counts EN words.
        let md = "Une phrase avec *un un deux trois quatre cinq six sept huit neuf dix* finale.";
        let diags = lint(md, Profile::Public);
        assert_eq!(diags.len(), 1, "got {diags:?}");
    }

    #[test]
    fn nested_emphasis_outer_and_inner_each_evaluated() {
        // Outer span includes inner span's words in its count.
        // Public threshold = 8. Outer = "one two three four five six
        // seven inner_one inner_two inner_three" = 10 words → fires.
        // Inner = "inner_one inner_two inner_three" = 3 words → ok.
        let md = "Some *one two three four five six seven \
                  _inner_one inner_two inner_three_* tail.";
        let diags = lint(md, Profile::Public);
        // Outer fires; inner does not.
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn italic_inside_code_block_is_not_flagged() {
        // Code blocks are excluded from paragraph buffers (and from
        // the inline tree); confirm the rule honours that.
        let md = "Before.\n\n\
                  ```\n\
                  not *one two three four five six seven eight nine ten* italic\n\
                  ```\n\n\
                  After.";
        assert!(lint(md, Profile::Public).is_empty());
    }

    #[test]
    fn italic_inside_tight_list_item_is_caught() {
        // F129's tight-list paragraph synthesis must seed the inline
        // tree (already covered by F143 substrate tests; rule-side
        // verification here).
        let md = "- bullet with *one two three four five six seven eight nine ten* inside.\n";
        let diags = lint(md, Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn category_is_structure() {
        let md = "Some *one two three four five six seven eight nine ten* end.";
        let diags = lint(md, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Structure);
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_words.get(), 12);
        assert_eq!(Config::for_profile(Profile::Public).max_words.get(), 8);
        assert_eq!(Config::for_profile(Profile::Falc).max_words.get(), 5);
    }

    #[test]
    fn snapshot_fixture() {
        let md = "Short *one two* fine.\n\n\
                  Long *one two three four five six seven eight nine ten* fires.\n\n\
                  Plain prose paragraph with no italic at all.";
        let diags = lint(md, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
