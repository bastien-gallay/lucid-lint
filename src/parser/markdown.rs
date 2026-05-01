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
    // Depth of the current list-item context. > 0 while we are between
    // `Tag::Start(Item)` and the matching `Tag::End(Item)`. Tracked as a
    // depth (not a bool) so a nested item still counts as "inside an item"
    // even after its parent's text has emitted a paragraph.
    let mut list_item_depth: u32 = 0;
    // Line where the most recent `Tag::Start(Item)` opened, awaiting a
    // decision on whether pulldown-cmark will emit a `Tag::Paragraph`
    // wrapper (loose list) or fire content events directly (tight list).
    // Cleared on the first event after Item start: if it's a Paragraph
    // start we let pulldown drive; otherwise we synthesize a paragraph
    // for the item's inline content.
    let mut pending_item_start: Option<u32> = None;
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
        // Resolve pending tight-list-item paragraph synthesis. In a
        // loose list pulldown-cmark wraps item content in `Tag::Paragraph`
        // — the normal paragraph machinery handles it. In a tight list
        // (single item, or items without separating blank lines) it fires
        // content events directly inside `Tag::Item`; synthesize a
        // paragraph so all paragraph-level rules see the content.
        if let Some(line) = pending_item_start {
            match &event {
                Event::Start(Tag::Paragraph | Tag::Item | Tag::List(_))
                | Event::End(TagEnd::Item | TagEnd::List(_)) => {
                    // Loose-list (Paragraph), empty item, or sub-list-only
                    // item. Nothing to synthesize.
                    pending_item_start = None;
                },
                _ => {
                    // Tight-list path: synthesize a paragraph anchored
                    // at the item's start line. `from_list_item` is
                    // derived from `list_item_depth` at finish time.
                    in_paragraph = true;
                    paragraph_start_line = line;
                    buf.clear();
                    pending_item_start = None;
                },
            }
        }
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                finish_paragraph(
                    &mut in_paragraph,
                    list_item_depth,
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
                    list_item_depth,
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
                // Finish any in-flight paragraph (e.g. the parent item's
                // inline text when a nested list begins) before opening
                // a fresh item context.
                finish_paragraph(
                    &mut in_paragraph,
                    list_item_depth,
                    &mut buf,
                    &mut current_paragraphs,
                    paragraph_start_line,
                );
                list_item_depth = list_item_depth.saturating_add(1);
                let item_line = offset_to_line(text, range.start);
                list_items.push(ListItem::new(list_depth.max(1), item_line));
                // Defer the tight-vs-loose decision to the next event.
                // See the lookahead block at the top of the for-loop.
                pending_item_start = Some(item_line);
            },
            Event::End(TagEnd::Item) => {
                // Close any synthetic (tight-list) paragraph the item
                // opened. Loose-list paragraphs were already finished by
                // their own `Tag::End(Paragraph)`, so this is a no-op
                // when `in_paragraph` is already false.
                finish_paragraph(
                    &mut in_paragraph,
                    list_item_depth,
                    &mut buf,
                    &mut current_paragraphs,
                    paragraph_start_line,
                );
                list_item_depth = list_item_depth.saturating_sub(1);
                pending_item_start = None;
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
                // `<br>` is an authorial line break the renderer respects;
                // map it to `\n` so paragraph-level rules that care about
                // author-chosen wrap (e.g. `structure.line-length-wide`)
                // see it the same way as a Markdown HardBreak. Comments
                // carrying suppression directives flow through unchanged.
                if (in_heading.is_some() || in_paragraph) && html_is_br_tag(&s) {
                    buf.push('\n');
                }
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
    // Flush any remaining content.
    finish_paragraph(
        &mut in_paragraph,
        list_item_depth,
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
    list_item_depth: u32,
    buf: &mut String,
    paragraphs: &mut Vec<Paragraph>,
    start_line: u32,
) {
    if !*in_paragraph {
        return;
    }
    let text = buf.trim().to_string();
    if !text.is_empty() {
        let para = if list_item_depth > 0 {
            Paragraph::from_list_item(text, start_line)
        } else {
            Paragraph::new(text, start_line)
        };
        paragraphs.push(para);
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

/// Recognise `<br>`, `<br/>`, `<br />` (any case, optional whitespace).
///
/// Pulldown-cmark emits a `<br>` tag as `Event::InlineHtml`, not as
/// `Event::HardBreak`. We treat it as an authorial newline so paragraph
/// rules see the same shape they get from a Markdown `HardBreak` (two
/// trailing spaces).
fn html_is_br_tag(s: &str) -> bool {
    let trimmed = s.trim();
    let Some(inner) = trimmed.strip_prefix('<').and_then(|t| t.strip_suffix('>')) else {
        return false;
    };
    let inner = inner.trim_end_matches('/').trim();
    inner.eq_ignore_ascii_case("br")
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
    fn br_tag_inside_paragraph_is_a_hard_break() {
        // `<br>` is parsed as InlineHtml, not HardBreak — the parser must
        // still preserve it as a `\n` in paragraph.text so rules that key
        // off authorial line breaks (e.g. `structure.line-length-wide`)
        // see it like the two-trailing-spaces hard-break form.
        for variant in ["<br>", "<br/>", "<br />", "<BR>", "<Br />"] {
            let md = format!("Lead.{variant}Tail.");
            let doc = parse_markdown(&md, SourceFile::Anonymous);
            let para = &doc.sections[0].paragraphs[0].text;
            assert!(
                para.contains("Lead.\nTail."),
                "variant {variant:?} produced {para:?}"
            );
        }
    }

    #[test]
    fn html_comment_directives_do_not_inject_newlines() {
        // Suppression directives ride on InlineHtml too. Make sure the
        // <br> handling didn't accidentally turn comments into breaks.
        let md = "Lead. <!-- lucid-lint-disable rhythm.foo --> Tail.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let para = &doc.sections[0].paragraphs[0].text;
        assert!(!para.contains('\n'), "got {para:?}");
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

    // ---- F129: list-item paragraphs ----

    #[test]
    fn tight_list_item_emits_a_paragraph() {
        // Single-bullet (tight) list. Pulldown-cmark fires Text events
        // directly inside Tag::Item without a wrapping Tag::Paragraph;
        // the parser must synthesize one so paragraph-level rules see
        // the content.
        let md = "- One bullet, five commas: a, b, c, d, e, f.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let paras: Vec<_> = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .collect();
        assert_eq!(paras.len(), 1, "got {paras:?}");
        assert!(paras[0].text.contains("five commas"));
        assert!(paras[0].from_list_item);
    }

    #[test]
    fn loose_list_item_paragraphs_are_marked_from_list_item() {
        // Two bullets with a blank line between them — pulldown-cmark
        // wraps each item content in Tag::Paragraph (loose list). The
        // resulting paragraphs must carry the same `from_list_item`
        // marker as the synthesized tight ones, so list-aware rules
        // behave identically across both shapes.
        let md = "- First item, comma-heavy: a, b, c, d, e.\n\n\
                  - Second item, also comma-heavy: f, g, h, i, j.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let paras: Vec<_> = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .collect();
        assert_eq!(paras.len(), 2);
        assert!(paras.iter().all(|p| p.from_list_item));
    }

    #[test]
    fn body_paragraphs_are_not_marked_from_list_item() {
        let md = "A regular body paragraph.\n\nAnother one.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let paras: Vec<_> = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .collect();
        assert_eq!(paras.len(), 2);
        assert!(paras.iter().all(|p| !p.from_list_item));
    }

    #[test]
    fn nested_list_emits_one_paragraph_per_item() {
        // Outer item has its own inline text; inner item also has its
        // own. Each must materialise as its own paragraph (not merged).
        let md = "- outer item, with three commas: a, b, c.\n  \
                  - inner item, also three: d, e, f.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let paras: Vec<_> = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .collect();
        assert_eq!(paras.len(), 2, "got {paras:?}");
        assert!(paras.iter().all(|p| p.from_list_item));
        assert!(paras[0].text.contains("outer item"));
        assert!(paras[1].text.contains("inner item"));
    }

    #[test]
    fn empty_list_item_produces_no_paragraph() {
        let md = "- \n- still empty\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let paras: Vec<_> = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .collect();
        // Only the second item carries text.
        assert_eq!(paras.len(), 1);
        assert!(paras[0].text.contains("still empty"));
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
