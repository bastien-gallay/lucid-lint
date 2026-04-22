//! Markdown parser using `pulldown-cmark`.
//!
//! Extracts headings and paragraphs while excluding:
//! - Fenced and indented code blocks
//! - Inline code
//! - HTML blocks
//!
//! Emphasis, strong, and inline links have their visible text preserved but
//! their Markdown markup stripped.

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use super::document::{Directive, Document, ListItem, Paragraph, Section};
use crate::types::SourceFile;

/// Parse a Markdown text into a [`Document`].
#[must_use]
pub fn parse_markdown(text: &str, source: SourceFile) -> Document {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;

    let mut sections: Vec<Section> = Vec::new();
    let mut directives: Vec<Directive> = Vec::new();
    let mut pending_directive_rules: Vec<String> = Vec::new();
    // Open block-form disable scopes: (rule_id, start_line).
    let mut open_blocks: Vec<(String, u32)> = Vec::new();
    let mut list_items: Vec<ListItem> = Vec::new();
    let mut list_depth: u32 = 0;
    let mut current_title: Option<String> = None;
    let mut current_depth: u32 = 0;
    let mut current_heading_line: Option<u32> = None;
    let mut current_paragraphs: Vec<Paragraph> = Vec::new();

    let mut in_heading: Option<HeadingLevel> = None;
    let mut in_paragraph = false;
    let mut in_code = false;
    let mut in_list_item = false;
    let mut buf = String::new();
    let mut paragraph_start_line: u32 = 1;

    let offsets: Vec<(Event, std::ops::Range<usize>)> =
        Parser::new_ext(text, options).into_offset_iter().collect();

    for (event, range) in offsets {
        // Flush any queued directive rule ids onto the next content block
        // (paragraph, heading, list item). Directives on intervening HTML
        // comments or blank lines are carried until a real block appears.
        if !pending_directive_rules.is_empty() && is_content_block_start(&event) {
            let target_line = offset_to_line(text, range.start);
            for rule_id in std::mem::take(&mut pending_directive_rules) {
                directives.push(Directive::new(rule_id, target_line));
            }
        }
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                finish_paragraph(
                    &mut in_paragraph,
                    &mut buf,
                    &mut current_paragraphs,
                    paragraph_start_line,
                );
                finish_section(
                    &mut sections,
                    &mut current_title,
                    &mut current_depth,
                    &mut current_heading_line,
                    &mut current_paragraphs,
                );
                in_heading = Some(level);
                current_heading_line = Some(offset_to_line(text, range.start));
                buf.clear();
            },
            Event::End(TagEnd::Heading(level)) => {
                current_title = Some(buf.trim().to_string()).filter(|s| !s.is_empty());
                current_depth = heading_depth(level);
                current_paragraphs = Vec::new();
                in_heading = None;
                buf.clear();
            },
            Event::Start(Tag::Paragraph) => {
                in_paragraph = true;
                buf.clear();
                paragraph_start_line = offset_to_line(text, range.start);
            },
            Event::End(TagEnd::Paragraph) => {
                finish_paragraph(
                    &mut in_paragraph,
                    &mut buf,
                    &mut current_paragraphs,
                    paragraph_start_line,
                );
            },
            Event::Start(Tag::List(_)) => {
                list_depth = list_depth.saturating_add(1);
            },
            Event::End(TagEnd::List(_)) => {
                list_depth = list_depth.saturating_sub(1);
            },
            Event::Start(Tag::Item) => {
                in_list_item = true;
                list_items.push(ListItem::new(
                    list_depth.max(1),
                    offset_to_line(text, range.start),
                ));
                // List items are not treated as paragraphs in v0.1: they don't
                // contribute to paragraph-level rules. However their text could
                // still feed sentence-level rules in a future iteration.
            },
            Event::End(TagEnd::Item) => {
                in_list_item = false;
            },
            Event::Start(Tag::CodeBlock(_)) => {
                in_code = true;
            },
            Event::End(TagEnd::CodeBlock) => {
                in_code = false;
            },
            Event::Code(_) => {
                // Inline code: skip contents.
            },
            Event::Html(s) | Event::InlineHtml(s) => {
                let block_line = offset_to_line(text, range.start);
                for (parsed, line) in parse_all_directives_in_html(&s, block_line) {
                    match parsed {
                        ParsedDirective::LineForm { rule_id } => {
                            // Queue; the flush at the top of the loop attaches
                            // these to the next real content block so stacked
                            // directives and blank lines between them all
                            // resolve to the same target.
                            pending_directive_rules.push(rule_id);
                        },
                        ParsedDirective::BlockOpen { rule_id } => {
                            open_blocks.push((rule_id, line));
                        },
                        ParsedDirective::BlockClose { rule_id: Some(id) } => {
                            if let Some(pos) = open_blocks.iter().rposition(|(r, _)| r == &id) {
                                let (rule_id, start) = open_blocks.remove(pos);
                                directives.push(Directive::block(rule_id, start, line));
                            }
                        },
                        ParsedDirective::BlockClose { rule_id: None } => {
                            for (rule_id, start) in std::mem::take(&mut open_blocks) {
                                directives.push(Directive::block(rule_id, start, line));
                            }
                        },
                    }
                }
            },
            Event::Text(s) => {
                if in_code {
                    continue;
                }
                if in_heading.is_some() || in_paragraph {
                    buf.push_str(&s);
                }
            },
            Event::SoftBreak if in_heading.is_some() || in_paragraph => {
                buf.push(' ');
            },
            Event::HardBreak if in_heading.is_some() || in_paragraph => {
                buf.push('\n');
            },
            _ => {},
        }
    }
    // `in_list_item` is currently tracked but not yet consumed by a rule;
    // we keep the flag wired so future list-aware rules can rely on it.
    let _ = in_list_item;

    // Flush any remaining content.
    finish_paragraph(
        &mut in_paragraph,
        &mut buf,
        &mut current_paragraphs,
        paragraph_start_line,
    );
    finish_section(
        &mut sections,
        &mut current_title,
        &mut current_depth,
        &mut current_heading_line,
        &mut current_paragraphs,
    );

    if sections.is_empty() {
        sections.push(Section::new(None, 0, Vec::new()));
    }

    // Close any unterminated block-disables at end-of-document so they still
    // suppress diagnostics up to the last line of the input.
    if !open_blocks.is_empty() {
        let last_line = offset_to_line(text, text.len().saturating_sub(1));
        for (rule_id, start) in open_blocks {
            directives.push(Directive::block(rule_id, start, last_line.max(start)));
        }
    }

    Document::with_metadata(source, sections, directives, list_items)
}

/// Whether an event starts a "content block" a directive can attach to.
/// Paragraphs, headings, and list items count; raw HTML and blank space
/// in between do not.
fn is_content_block_start(event: &Event<'_>) -> bool {
    matches!(
        event,
        Event::Start(Tag::Paragraph | Tag::Heading { .. } | Tag::Item)
    )
}

/// A disable directive parsed from a single HTML comment.
enum ParsedDirective {
    /// `<!-- lucid-lint disable-next-line <rule-id> -->`
    LineForm { rule_id: String },
    /// `<!-- lucid-lint-disable <rule-id> -->`
    BlockOpen { rule_id: String },
    /// `<!-- lucid-lint-enable [<rule-id>] -->` — `None` closes all open scopes.
    BlockClose { rule_id: Option<String> },
}

/// Parse every recognized directive in an HTML block, paired with its
/// 1-based line number relative to the document (derived from
/// `block_start_line` plus the newlines that precede the comment inside
/// `html`).
fn parse_all_directives_in_html(html: &str, block_start_line: u32) -> Vec<(ParsedDirective, u32)> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while let Some(open_rel) = html[cursor..].find("<!--") {
        let open = cursor + open_rel;
        let Some(close_rel) = html[open..].find("-->") else {
            break;
        };
        let close = open + close_rel + 3;
        let comment = &html[open..close];
        if let Some(parsed) = parse_single_directive(comment) {
            #[allow(clippy::naive_bytecount)]
            let newlines_before = html.as_bytes()[..open]
                .iter()
                .filter(|&&b| b == b'\n')
                .count();
            let line = block_start_line.saturating_add(u32::try_from(newlines_before).unwrap_or(0));
            out.push((parsed, line));
        }
        cursor = close;
    }
    out
}

/// Parse a single HTML comment into a recognized directive, or `None`.
///
/// Recognized forms (order matters — block forms are tried first because
/// they share the `lucid-lint` prefix with the line form):
///
/// - `<!-- lucid-lint-disable <rule-id> -->`
/// - `<!-- lucid-lint-enable [<rule-id>] -->`
/// - `<!-- lucid-lint disable-next-line <rule-id> -->`
fn parse_single_directive(html: &str) -> Option<ParsedDirective> {
    let inner = html
        .trim()
        .strip_prefix("<!--")?
        .strip_suffix("-->")?
        .trim();

    if let Some(rest) = inner.strip_prefix("lucid-lint-disable") {
        let rule_id = rest.strip_prefix(|c: char| c.is_whitespace())?.trim();
        if rule_id.is_empty() || !is_valid_rule_id(rule_id) {
            return None;
        }
        return Some(ParsedDirective::BlockOpen {
            rule_id: rule_id.to_string(),
        });
    }

    if let Some(rest) = inner.strip_prefix("lucid-lint-enable") {
        let trimmed = rest.trim();
        if trimmed.is_empty() {
            return Some(ParsedDirective::BlockClose { rule_id: None });
        }
        if !is_valid_rule_id(trimmed) {
            return None;
        }
        return Some(ParsedDirective::BlockClose {
            rule_id: Some(trimmed.to_string()),
        });
    }

    let rest = inner.strip_prefix("lucid-lint")?;
    let rest = rest.strip_prefix(|c: char| c.is_whitespace())?.trim_start();
    let rest = rest.strip_prefix("disable-next-line")?;
    let rule_id = rest.strip_prefix(|c: char| c.is_whitespace())?.trim();
    if rule_id.is_empty() || !is_valid_rule_id(rule_id) {
        return None;
    }
    Some(ParsedDirective::LineForm {
        rule_id: rule_id.to_string(),
    })
}

fn is_valid_rule_id(s: &str) -> bool {
    // Rule ids use the `category.rule-name` convention (F29-slim).
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.')
}

fn finish_paragraph(
    in_paragraph: &mut bool,
    buf: &mut String,
    paragraphs: &mut Vec<Paragraph>,
    start_line: u32,
) {
    if !*in_paragraph {
        return;
    }
    let text = buf.trim().to_string();
    if !text.is_empty() {
        paragraphs.push(Paragraph::new(text, start_line));
    }
    buf.clear();
    *in_paragraph = false;
}

fn finish_section(
    sections: &mut Vec<Section>,
    title: &mut Option<String>,
    depth: &mut u32,
    heading_line: &mut Option<u32>,
    paragraphs: &mut Vec<Paragraph>,
) {
    if title.is_none() && paragraphs.is_empty() && sections.is_empty() {
        // No content yet, don't push an empty leading section.
        return;
    }
    if title.is_some() || !paragraphs.is_empty() {
        let section = match heading_line.take() {
            Some(line) => {
                Section::with_heading_line(title.take(), *depth, line, std::mem::take(paragraphs))
            },
            None => Section::new(title.take(), *depth, std::mem::take(paragraphs)),
        };
        sections.push(section);
        *depth = 0;
    }
}

const fn heading_depth(level: HeadingLevel) -> u32 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn offset_to_line(text: &str, offset: usize) -> u32 {
    let capped = offset.min(text.len());
    #[allow(clippy::naive_bytecount)]
    let lines_before = text.as_bytes()[..capped]
        .iter()
        .filter(|&&b| b == b'\n')
        .count();
    (lines_before + 1).try_into().unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_markdown() {
        let md = "# Title\n\nFirst paragraph.\n\n## Sub\n\nSecond paragraph.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert_eq!(doc.sections.len(), 2);
        assert_eq!(doc.sections[0].title.as_deref(), Some("Title"));
        assert_eq!(doc.sections[0].paragraphs.len(), 1);
        assert_eq!(doc.sections[1].title.as_deref(), Some("Sub"));
        assert_eq!(doc.sections[1].paragraphs.len(), 1);
    }

    #[test]
    fn preserves_inline_emphasis_text() {
        let md = "Some *emphasized* and **strong** and `code` text.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let para = &doc.sections[0].paragraphs[0].text;
        assert!(para.contains("emphasized"));
        assert!(para.contains("strong"));
        // Inline code content is excluded.
        assert!(!para.contains("code"));
    }

    #[test]
    fn excludes_fenced_code_blocks() {
        let md = "Before.\n\n```\nignored code\n```\n\nAfter.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let texts: Vec<_> = doc.sections[0]
            .paragraphs
            .iter()
            .map(|p| p.text.clone())
            .collect();
        assert!(!texts.iter().any(|t| t.contains("ignored code")));
        assert!(texts.iter().any(|t| t == "Before."));
        assert!(texts.iter().any(|t| t == "After."));
    }

    #[test]
    fn extracts_heading_hierarchy() {
        let md = "# H1\n\nIntro.\n\n## H2\n\nSubcontent.\n\n### H3\n\nDeep.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let depths: Vec<u32> = doc.sections.iter().map(|s| s.depth).collect();
        assert_eq!(depths, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty_markdown() {
        let doc = parse_markdown("", SourceFile::Anonymous);
        assert!(doc.sections.is_empty() || doc.sections[0].paragraphs.is_empty());
    }

    #[test]
    fn handles_markdown_with_no_headings() {
        let md = "Just a paragraph.\n\nAnd another.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert_eq!(doc.sections.len(), 1);
        assert_eq!(doc.sections[0].title, None);
        assert_eq!(doc.sections[0].paragraphs.len(), 2);
    }

    #[test]
    fn extracts_disable_next_line_directive() {
        let md = "Intro.\n\n<!-- lucid-lint disable-next-line structure.sentence-too-long -->\n\
                  A long sentence that will be suppressed.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert_eq!(doc.directives.len(), 1);
        assert_eq!(doc.directives[0].rule_id, "structure.sentence-too-long");
        // Directive is on line 3, next non-blank line is 4.
        assert_eq!(doc.directives[0].start_line, 4);
        assert_eq!(doc.directives[0].end_line, 4);
    }

    #[test]
    fn ignores_non_directive_html_comments() {
        let md = "Intro.\n\n<!-- just a regular comment -->\n\nAfter.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert!(doc.directives.is_empty());
    }

    #[test]
    fn rejects_directive_with_invalid_rule_id() {
        let md = "<!-- lucid-lint disable-next-line Bad_Rule -->\nText.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert!(doc.directives.is_empty());
    }

    #[test]
    fn directive_without_following_content_is_dropped() {
        let md = "Body.\n\n<!-- lucid-lint disable-next-line structure.sentence-too-long -->\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert!(doc.directives.is_empty());
    }

    #[test]
    fn extracts_block_disable_and_enable_directive() {
        let md = "Intro.\n\n\
                  <!-- lucid-lint-disable structure.sentence-too-long -->\n\n\
                  Inside block.\n\n\
                  More inside.\n\n\
                  <!-- lucid-lint-enable -->\n\n\
                  After.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert_eq!(doc.directives.len(), 1);
        let d = &doc.directives[0];
        assert_eq!(d.rule_id, "structure.sentence-too-long");
        assert_eq!(d.start_line, 3);
        assert_eq!(d.end_line, 9);
        assert!(d.covers(5));
        assert!(d.covers(7));
        assert!(!d.covers(11));
    }

    #[test]
    fn block_enable_with_rule_id_closes_matching_scope_only() {
        let md = "<!-- lucid-lint-disable structure.sentence-too-long -->\n\n\
                  <!-- lucid-lint-disable lexicon.weasel-words -->\n\n\
                  Between.\n\n\
                  <!-- lucid-lint-enable structure.sentence-too-long -->\n\n\
                  After.\n\n\
                  <!-- lucid-lint-enable -->\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert_eq!(doc.directives.len(), 2);
        let sentence = doc
            .directives
            .iter()
            .find(|d| d.rule_id == "structure.sentence-too-long")
            .expect("sentence-too-long directive");
        let weasel = doc
            .directives
            .iter()
            .find(|d| d.rule_id == "lexicon.weasel-words")
            .expect("weasel-words directive");
        assert!(sentence.end_line < weasel.end_line);
    }

    #[test]
    fn unterminated_block_disable_extends_to_end_of_document() {
        let md = "Intro.\n\n\
                  <!-- lucid-lint-disable structure.sentence-too-long -->\n\n\
                  Body.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert_eq!(doc.directives.len(), 1);
        let d = &doc.directives[0];
        assert_eq!(d.rule_id, "structure.sentence-too-long");
        assert!(d.end_line >= d.start_line);
        assert!(d.covers(5));
    }

    #[test]
    fn enable_with_no_matching_open_scope_is_ignored() {
        let md = "<!-- lucid-lint-enable structure.sentence-too-long -->\n\nText.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert!(doc.directives.is_empty());
    }

    #[test]
    fn block_directive_with_invalid_rule_id_is_rejected() {
        let md = "<!-- lucid-lint-disable Bad_Rule -->\n\nBody.\n\n\
                  <!-- lucid-lint-enable -->\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        assert!(doc.directives.is_empty());
    }

    #[test]
    fn preserves_paragraph_start_line() {
        let md = "Line 1.\n\nLine 3.\n\nLine 5.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let lines: Vec<u32> = doc.sections[0]
            .paragraphs
            .iter()
            .map(|p| p.start_line)
            .collect();
        assert_eq!(lines.len(), 3);
        // We trust line numbering to be monotonically increasing.
        assert!(lines[0] <= lines[1] && lines[1] <= lines[2]);
    }
}
