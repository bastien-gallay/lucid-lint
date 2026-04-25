//! Document model produced by the parser.
//!
//! A [`Document`] contains an ordered list of [`Section`]s (derived from
//! headings in Markdown). Each section contains an ordered list of
//! [`Paragraph`]s. Each paragraph carries its sentences, computed once
//! at construction via [`super::tokenizer::split_sentences`] — eight
//! rhythm/syntax/lexicon/structure rules consume them, so paying the
//! split once and sharing across rules is strictly cheaper than the
//! previous "lazy per-rule" pattern (F103, samply 2026-04-25).

use crate::parser::tokenizer::split_sentences;
use crate::types::SourceFile;

/// The parsed representation of a single input text.
#[derive(Debug, Clone)]
pub struct Document {
    /// Origin of the document.
    pub source: SourceFile,

    /// Sections of the document. The first section may have no heading
    /// (content before the first heading, or plain text input).
    pub sections: Vec<Section>,

    /// Inline-disable directives extracted from the source. Each directive
    /// silences one rule on one target line. See [`Directive`].
    pub directives: Vec<Directive>,

    /// List items captured during parsing, with their nesting depth and line.
    /// Empty for plain-text inputs (which have no list structure).
    pub list_items: Vec<ListItem>,
}

impl Document {
    /// Create a new document with no directives and no list items.
    #[must_use]
    pub const fn new(source: SourceFile, sections: Vec<Section>) -> Self {
        Self {
            source,
            sections,
            directives: Vec::new(),
            list_items: Vec::new(),
        }
    }

    /// Create a new document carrying parser-captured metadata (directives
    /// and list items).
    #[must_use]
    pub const fn with_metadata(
        source: SourceFile,
        sections: Vec<Section>,
        directives: Vec<Directive>,
        list_items: Vec<ListItem>,
    ) -> Self {
        Self {
            source,
            sections,
            directives,
            list_items,
        }
    }

    /// Iterate over all paragraphs across all sections, yielding each paragraph
    /// with the title of the section it belongs to.
    pub fn paragraphs_with_section(&self) -> impl Iterator<Item = (&Paragraph, Option<&str>)> {
        self.sections.iter().flat_map(|section| {
            let title = section.title.as_deref();
            section.paragraphs.iter().map(move |p| (p, title))
        })
    }
}

/// A section of a document, rooted at a heading (or synthetic for pre-heading content).
#[derive(Debug, Clone)]
pub struct Section {
    /// The heading text (without the leading `#` markers).
    ///
    /// `None` for the implicit section containing content before the first
    /// heading, or for plain text input.
    pub title: Option<String>,

    /// Heading depth (1 for H1, 2 for H2, etc.). 0 for the synthetic pre-heading section.
    pub depth: u32,

    /// 1-based line of the heading in the source. `None` for the synthetic
    /// pre-heading section.
    pub heading_line: Option<u32>,

    /// Paragraphs under this section.
    pub paragraphs: Vec<Paragraph>,
}

impl Section {
    /// Create a new section without a heading line (synthetic or plain text).
    #[must_use]
    pub const fn new(title: Option<String>, depth: u32, paragraphs: Vec<Paragraph>) -> Self {
        Self {
            title,
            depth,
            heading_line: None,
            paragraphs,
        }
    }

    /// Create a new section rooted at a heading on a specific line.
    #[must_use]
    pub const fn with_heading_line(
        title: Option<String>,
        depth: u32,
        heading_line: u32,
        paragraphs: Vec<Paragraph>,
    ) -> Self {
        Self {
            title,
            depth,
            heading_line: Some(heading_line),
            paragraphs,
        }
    }
}

/// A paragraph of prose.
#[derive(Debug, Clone)]
pub struct Paragraph {
    /// The paragraph text with Markdown inline markup stripped.
    pub text: String,

    /// 1-based line number in the source where the paragraph starts.
    pub start_line: u32,

    /// Sentences extracted from `text` at construction time, with absolute
    /// source positions seeded from `start_line`. Rules consume this slice
    /// instead of re-running [`split_sentences`] per rule.
    pub sentences: Vec<Sentence>,
}

impl Paragraph {
    /// Create a new paragraph and split it into sentences.
    #[must_use]
    pub fn new(text: String, start_line: u32) -> Self {
        let sentences = split_sentences(&text, start_line, 1);
        Self {
            text,
            start_line,
            sentences,
        }
    }
}

/// A sentence extracted from a paragraph.
///
/// Produced on demand by [`super::tokenizer::split_sentences`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    /// The sentence text.
    pub text: String,

    /// 1-based line where the sentence starts (approximate).
    pub line: u32,

    /// 1-based column where the sentence starts within its line (approximate).
    pub column: u32,
}

/// A disable directive extracted from the source.
///
/// Two forms are supported:
///
/// - **Line form** (v0.1): `<!-- lucid-lint disable-next-line <rule-id> -->`
///   silences `rule_id` on the next non-blank line. `start_line == end_line`.
/// - **Block form** (v0.2, F18):
///   `<!-- lucid-lint-disable <rule-id> -->` … `<!-- lucid-lint-enable -->`
///   silences `rule_id` on every line between the two comments (inclusive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directive {
    /// The rule id silenced by this directive.
    pub rule_id: String,

    /// 1-based first line the directive covers (inclusive).
    pub start_line: u32,

    /// 1-based last line the directive covers (inclusive).
    pub end_line: u32,
}

/// A list item position captured during parsing.
///
/// Emitted once per `<li>` (or ordered-list item), carrying the 1-based
/// nesting depth (outermost list is 1) and the 1-based source line where
/// the item starts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    /// 1-based nesting depth: outermost list items are depth 1.
    pub depth: u32,

    /// 1-based line where the item starts in the source.
    pub line: u32,
}

impl ListItem {
    /// Create a new list item.
    #[must_use]
    pub const fn new(depth: u32, line: u32) -> Self {
        Self { depth, line }
    }
}

impl Directive {
    /// Create a line-form directive covering a single line.
    #[must_use]
    pub fn new(rule_id: impl Into<String>, target_line: u32) -> Self {
        Self {
            rule_id: rule_id.into(),
            start_line: target_line,
            end_line: target_line,
        }
    }

    /// Create a block-form directive covering an inclusive line range.
    #[must_use]
    pub fn block(rule_id: impl Into<String>, start_line: u32, end_line: u32) -> Self {
        Self {
            rule_id: rule_id.into(),
            start_line,
            end_line,
        }
    }

    /// Whether `line` falls inside this directive's range (inclusive).
    #[must_use]
    pub const fn covers(&self, line: u32) -> bool {
        line >= self.start_line && line <= self.end_line
    }
}

impl Sentence {
    /// Create a new sentence with explicit position.
    #[must_use]
    pub fn new(text: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            text: text.into(),
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraphs_with_section_yields_titles() {
        let section = Section::new(
            Some("Intro".to_string()),
            2,
            vec![Paragraph::new("Hello.".to_string(), 1)],
        );
        let doc = Document::new(SourceFile::Anonymous, vec![section]);
        let collected: Vec<_> = doc
            .paragraphs_with_section()
            .map(|(p, title)| (p.text.clone(), title.map(ToOwned::to_owned)))
            .collect();
        assert_eq!(
            collected,
            vec![("Hello.".to_string(), Some("Intro".to_string()))]
        );
    }

    #[test]
    fn paragraphs_with_section_yields_none_for_untitled_sections() {
        let section = Section::new(None, 0, vec![Paragraph::new("Body.".to_string(), 1)]);
        let doc = Document::new(SourceFile::Anonymous, vec![section]);
        let titles: Vec<_> = doc
            .paragraphs_with_section()
            .map(|(_, title)| title.map(ToOwned::to_owned))
            .collect();
        assert_eq!(titles, vec![None]);
    }
}
