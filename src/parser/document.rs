//! Document model produced by the parser.
//!
//! A [`Document`] contains an ordered list of [`Section`]s (derived from
//! headings in Markdown). Each section contains an ordered list of
//! [`Paragraph`]s. Sentences are derived on demand from paragraph text
//! via [`super::tokenizer::split_sentences`] because some rules work on
//! full paragraphs and some on sentences — we avoid eagerly paying for
//! sentence splitting when it is not needed.

use crate::types::SourceFile;

/// The parsed representation of a single input text.
#[derive(Debug, Clone)]
pub struct Document {
    /// Origin of the document.
    pub source: SourceFile,

    /// Sections of the document. The first section may have no heading
    /// (content before the first heading, or plain text input).
    pub sections: Vec<Section>,
}

impl Document {
    /// Create a new document.
    #[must_use]
    pub fn new(source: SourceFile, sections: Vec<Section>) -> Self {
        Self { source, sections }
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

    /// Paragraphs under this section.
    pub paragraphs: Vec<Paragraph>,
}

impl Section {
    /// Create a new section.
    #[must_use]
    pub fn new(title: Option<String>, depth: u32, paragraphs: Vec<Paragraph>) -> Self {
        Self { title, depth, paragraphs }
    }
}

/// A paragraph of prose.
#[derive(Debug, Clone)]
pub struct Paragraph {
    /// The paragraph text with Markdown inline markup stripped.
    pub text: String,

    /// 1-based line number in the source where the paragraph starts.
    pub start_line: u32,
}

impl Paragraph {
    /// Create a new paragraph.
    #[must_use]
    pub fn new(text: String, start_line: u32) -> Self {
        Self { text, start_line }
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

impl Sentence {
    /// Create a new sentence with explicit position.
    #[must_use]
    pub fn new(text: impl Into<String>, line: u32, column: u32) -> Self {
        Self { text: text.into(), line, column }
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
        assert_eq!(collected, vec![("Hello.".to_string(), Some("Intro".to_string()))]);
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
