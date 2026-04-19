//! Parsing of input text into a structured [`Document`] model.
//!
//! The parser's job is to extract prose from Markdown (or plain text),
//! split it into paragraphs and sentences, and attach heading-based sections
//! so rules can emit diagnostics with rich context.

mod document;
mod markdown;
mod tokenizer;

pub use document::{Directive, Document, ListItem, Paragraph, Section, Sentence};
pub use markdown::parse_markdown;
pub use tokenizer::{split_sentences, word_count};

use crate::types::SourceFile;

/// Parse a plain text input into a [`Document`].
///
/// Plain text has no heading structure, so the result contains a single
/// unnamed section with all paragraphs.
#[must_use]
pub fn parse_plain(text: &str, source: SourceFile) -> Document {
    let paragraphs: Vec<Paragraph> = text
        .split("\n\n")
        .enumerate()
        .filter_map(|(idx, chunk)| {
            let trimmed = chunk.trim();
            if trimmed.is_empty() {
                None
            } else {
                // Best-effort line tracking: paragraphs are separated by blank lines,
                // so we approximate starting line by counting previous chunks.
                let start_line = count_lines_until(text, idx);
                Some(Paragraph::new(trimmed.to_string(), start_line))
            }
        })
        .collect();

    let section = Section::new(None, 0, paragraphs);
    Document::new(source, vec![section])
}

/// Count the 1-based line where the nth double-newline chunk starts.
fn count_lines_until(text: &str, chunk_index: usize) -> u32 {
    if chunk_index == 0 {
        return 1;
    }
    let mut line: u32 = 1;
    let mut chunk: usize = 0;
    let mut prev_was_newline = false;
    for c in text.chars() {
        if c == '\n' {
            line += 1;
            if prev_was_newline {
                chunk += 1;
                if chunk == chunk_index {
                    return line;
                }
            }
            prev_was_newline = true;
        } else {
            prev_was_newline = false;
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plain_splits_on_blank_lines() {
        let text = "First paragraph.\n\nSecond paragraph.\n\nThird.";
        let doc = parse_plain(text, SourceFile::Anonymous);
        assert_eq!(doc.sections.len(), 1);
        assert_eq!(doc.sections[0].paragraphs.len(), 3);
    }

    #[test]
    fn parse_plain_ignores_empty_chunks() {
        let text = "\n\n\nFirst.\n\n\n\nSecond.";
        let doc = parse_plain(text, SourceFile::Anonymous);
        assert_eq!(doc.sections[0].paragraphs.len(), 2);
    }

    #[test]
    fn parse_plain_single_paragraph() {
        let text = "Just one paragraph, with one sentence.";
        let doc = parse_plain(text, SourceFile::Anonymous);
        assert_eq!(doc.sections[0].paragraphs.len(), 1);
    }
}
