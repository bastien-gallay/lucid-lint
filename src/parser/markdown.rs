//! Markdown parser using `pulldown-cmark`.
//!
//! Extracts headings and paragraphs while excluding:
//! - Fenced and indented code blocks
//! - Inline code
//! - HTML blocks
//!
//! Strong (`**…**`) and inline links have their visible text preserved but
//! their Markdown markup stripped from [`Paragraph::text`]. Emphasis
//! (`*…*` / `_…_`) is also flattened into the visible-text string, *and*
//! captured structurally as [`Inline::Emphasis`] nodes on
//! [`Paragraph::inline`] (F143 substrate) — so rules that need span
//! boundaries (e.g. `structure.italic-span-long`) can walk a typed inline
//! tree without re-parsing.

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use super::document::{Directive, Document, EmphasisSpan, Inline, ListItem, Paragraph, Section};
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
    // Inline tree (F143, lazy-build path B): a stack of children-vectors,
    // *only* allocated when the first emphasis fires inside a paragraph.
    // For the common case (no emphasis), the stack stays empty and the
    // paragraph ships with `inline: Vec::new()` — no double-allocation
    // alongside `buf`. The bottom frame, when present, is the paragraph-
    // level inline list; each open emphasis span pushes another frame.
    let mut inline_stack: Vec<Vec<Inline>> = Vec::new();
    // Whether the current paragraph has seen at least one emphasis
    // event. Drives lazy initialisation of `inline_stack`: stays false
    // for emphasis-free paragraphs (the bench-relevant common case),
    // flips true on the first `Tag::Emphasis` start and stays true
    // until paragraph end.
    let mut lazy_inline_active = false;
    // Source positions (line, column) of currently-open emphasis spans,
    // popped when their `End(Emphasis)` fires so the resulting
    // [`EmphasisSpan`] carries the opening-delimiter location.
    let mut emphasis_opens: Vec<(u32, u32)> = Vec::new();

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
                    inline_stack.clear();
                    lazy_inline_active = false;
                    emphasis_opens.clear();
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
                    &mut inline_stack,
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
                inline_stack.clear();
                lazy_inline_active = false;
                emphasis_opens.clear();
                paragraph_start_line = offset_to_line(text, range.start);
            },
            Event::End(TagEnd::Paragraph) => {
                finish_paragraph(
                    &mut in_paragraph,
                    list_item_depth,
                    &mut buf,
                    &mut inline_stack,
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
                    &mut inline_stack,
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
                    &mut inline_stack,
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
            Event::Start(Tag::Emphasis) if in_paragraph => {
                if !lazy_inline_active {
                    // First emphasis in this paragraph: rewind and
                    // commit the visible-text prefix (everything in
                    // `buf` so far) as a single Text node, then enter
                    // lazy mode for the rest of the paragraph. Empty
                    // prefix (paragraph opens with emphasis) → no
                    // prefix node, just an empty bottom frame.
                    let mut bottom = Vec::with_capacity(2);
                    if !buf.is_empty() {
                        bottom.push(Inline::Text(buf.clone()));
                    }
                    inline_stack.push(bottom);
                    lazy_inline_active = true;
                }
                let (line, col) = offset_to_line_col(text, range.start);
                emphasis_opens.push((line, col));
                inline_stack.push(Vec::new());
            },
            Event::End(TagEnd::Emphasis) if in_paragraph => {
                if let Some(children) = inline_stack.pop() {
                    let (start_line, start_column) =
                        emphasis_opens.pop().unwrap_or((paragraph_start_line, 1));
                    let span = EmphasisSpan {
                        children,
                        start_line,
                        start_column,
                    };
                    if let Some(parent) = inline_stack.last_mut() {
                        parent.push(Inline::Emphasis(span));
                    }
                }
            },
            Event::Html(s) | Event::InlineHtml(s) => {
                // `<br>` is an authorial line break the renderer respects;
                // map it to `\n` so paragraph-level rules that care about
                // author-chosen wrap (e.g. `structure.line-length-wide`)
                // see it the same way as a Markdown HardBreak. Comments
                // carrying suppression directives flow through unchanged.
                if (in_heading.is_some() || in_paragraph) && html_is_br_tag(&s) {
                    buf.push('\n');
                    if in_paragraph && lazy_inline_active {
                        push_inline_text(&mut inline_stack, "\n");
                    }
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
                if in_paragraph && lazy_inline_active {
                    push_inline_text(&mut inline_stack, &s);
                }
            },
            Event::SoftBreak if in_heading.is_some() || in_paragraph => {
                buf.push(' ');
                if in_paragraph && lazy_inline_active {
                    push_inline_text(&mut inline_stack, " ");
                }
            },
            Event::HardBreak if in_heading.is_some() || in_paragraph => {
                buf.push('\n');
                if in_paragraph && lazy_inline_active {
                    push_inline_text(&mut inline_stack, "\n");
                }
            },
            _ => {},
        }
    }
    // Flush any remaining content.
    finish_paragraph(
        &mut in_paragraph,
        list_item_depth,
        &mut buf,
        &mut inline_stack,
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
    inline_stack: &mut Vec<Vec<Inline>>,
    paragraphs: &mut Vec<Paragraph>,
    start_line: u32,
) {
    if !*in_paragraph {
        return;
    }
    let text = buf.trim().to_string();
    // Drop any open inline frames first (defensive: an unclosed
    // emphasis would otherwise leave orphan frames on the stack for
    // the next paragraph). Collapse them into the bottom frame so the
    // visible text and the inline tree stay in sync.
    while inline_stack.len() > 1 {
        let frame = inline_stack.pop().unwrap_or_default();
        if let Some(parent) = inline_stack.last_mut() {
            parent.extend(frame);
        }
    }
    let inline = inline_stack.pop().unwrap_or_default();
    if !text.is_empty() {
        let para = if list_item_depth > 0 {
            Paragraph::from_list_item_with_inline(text, start_line, inline)
        } else {
            Paragraph::with_inline(text, start_line, inline)
        };
        paragraphs.push(para);
    }
    buf.clear();
    *in_paragraph = false;
}

/// Append plain text to the current inline frame, merging with the
/// trailing [`Inline::Text`] node when possible. Keeps the inline tree
/// from accumulating tiny adjacent text nodes from soft breaks and
/// multi-event runs.
///
/// **Defensive empty-stack early-return.** Callers all gate on
/// `lazy_inline_active`, which is only ever set true after the first
/// frame is pushed onto `inline_stack`. The empty-stack early-return
/// is therefore unreachable under correct usage — but it is *also*
/// the reason `cargo-mutants` reports the surrounding
/// `&& lazy_inline_active` guards in callers as survivors: mutating
/// them to `||` is a no-op precisely because this function falls
/// through silently. Keeping the defense costs one branch and makes
/// the function safe to call from any future code path that hasn't
/// yet established the lazy-build invariant; do not remove it.
fn push_inline_text(inline_stack: &mut [Vec<Inline>], s: &str) {
    if s.is_empty() {
        return;
    }
    let Some(frame) = inline_stack.last_mut() else {
        return;
    };
    if let Some(Inline::Text(existing)) = frame.last_mut() {
        existing.push_str(s);
    } else {
        frame.push(Inline::Text(s.to_string()));
    }
}

/// 1-based (line, column) for a byte offset. Column counts characters
/// (not bytes) within the line, matching the project's existing
/// approximate-column convention used for diagnostic positions.
fn offset_to_line_col(text: &str, offset: usize) -> (u32, u32) {
    let capped = offset.min(text.len());
    let line_start = text.as_bytes()[..capped]
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(0, |p| p + 1);
    let line = offset_to_line(text, capped);
    let col = text[line_start..capped].chars().count() + 1;
    (line, u32::try_from(col).unwrap_or(u32::MAX))
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

    // ---- F143: inline AST capture ----

    fn paragraph_inline(md: &str) -> Vec<Inline> {
        let doc = parse_markdown(md, SourceFile::Anonymous);
        doc.sections
            .into_iter()
            .flat_map(|s| s.paragraphs)
            .next()
            .map(|p| p.inline)
            .unwrap_or_default()
    }

    #[test]
    fn paragraph_without_emphasis_has_empty_inline() {
        // F143 lazy-build (path B): paragraphs with no emphasis ship
        // with `inline: Vec::new()`. Rules wanting to know "no spans
        // worth modeling" check `inline.is_empty()`.
        let inline = paragraph_inline("Plain prose, nothing fancy.");
        assert!(inline.is_empty(), "got {inline:?}");
    }

    #[test]
    fn emphasis_span_is_captured() {
        let inline = paragraph_inline("Some *italic words* in the middle.");
        assert_eq!(inline.len(), 3, "got {inline:?}");
        assert_eq!(inline[0], Inline::Text("Some ".to_string()));
        let span = match &inline[1] {
            Inline::Emphasis(s) => s,
            Inline::Text(t) => unreachable!("expected Emphasis at index 1, got Text({t:?})"),
        };
        assert_eq!(
            span.children,
            vec![Inline::Text("italic words".to_string())]
        );
        assert_eq!(span.start_line, 1);
        // Column points at the opening `*` (1-based, char-counted).
        assert_eq!(span.start_column, 6);
        assert_eq!(inline[2], Inline::Text(" in the middle.".to_string()));
    }

    #[test]
    fn underscore_emphasis_is_captured_too() {
        let inline = paragraph_inline("An _underscore italic_ here.");
        let has_emphasis = inline
            .iter()
            .any(|n| matches!(n, Inline::Emphasis(s) if s.children == vec![Inline::Text("underscore italic".to_string())]));
        assert!(has_emphasis, "got {inline:?}");
    }

    #[test]
    fn strong_does_not_create_an_emphasis_node() {
        // F143 substrate is intentionally narrow: only Text + Emphasis
        // are modeled today. Strong (**bold**) flattens into Text and,
        // because no `Tag::Emphasis` event fires, the lazy-build path
        // (B) leaves `inline` empty altogether.
        let inline = paragraph_inline("Some **bold words** here.");
        assert!(
            inline.iter().all(|n| !matches!(n, Inline::Emphasis(_))),
            "got {inline:?}"
        );
        // Path B contract: emphasis-free paragraphs (strong-only,
        // link-only, plain) ship with `inline.is_empty()`.
        assert!(inline.is_empty(), "got {inline:?}");
    }

    #[test]
    fn nested_emphasis_is_preserved() {
        // Outer `*…*` contains an inner `_…_`. Nested emphasis should
        // round-trip with both spans visible in the tree.
        let inline = paragraph_inline("Outer *one _two_ three* end.");
        let outer = inline
            .iter()
            .find_map(|n| match n {
                Inline::Emphasis(s) => Some(s),
                Inline::Text(_) => None,
            })
            .expect("outer emphasis present");
        let inner = outer
            .children
            .iter()
            .find_map(|n| match n {
                Inline::Emphasis(s) => Some(s),
                Inline::Text(_) => None,
            })
            .expect("inner emphasis present");
        assert_eq!(inner.children, vec![Inline::Text("two".to_string())]);
    }

    #[test]
    fn multiple_emphases_in_one_paragraph_are_all_captured() {
        let inline = paragraph_inline("First *one* then *two* then *three*.");
        let count = inline
            .iter()
            .filter(|n| matches!(n, Inline::Emphasis(_)))
            .count();
        assert_eq!(count, 3, "got {inline:?}");
    }

    #[test]
    fn emphasis_in_code_block_does_not_appear_in_inline_tree() {
        // Code fences are excluded from paragraph buffers; the inline
        // tree must inherit that exclusion.
        let md = "Before.\n\n```\nignored *not italic*\n```\n\nAfter.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        for para in doc.sections.iter().flat_map(|s| s.paragraphs.iter()) {
            assert!(
                para.inline
                    .iter()
                    .all(|n| !matches!(n, Inline::Emphasis(_))),
                "code-block emphasis leaked into {para:?}"
            );
        }
    }

    #[test]
    fn inline_tree_is_empty_for_plain_text_input() {
        // `parse_plain` does not capture an inline tree — Markdown
        // semantics do not apply. The field should default to empty.
        let doc = super::super::parse_plain("A plain *paragraph* of text.", SourceFile::Anonymous);
        let para = &doc.sections[0].paragraphs[0];
        assert!(para.inline.is_empty(), "got {:?}", para.inline);
    }

    #[test]
    fn inline_tree_text_concatenation_matches_paragraph_text() {
        // Round-trip invariant: flattening the inline tree (text +
        // children of emphasis, recursively) should reproduce the
        // paragraph's `text` field. Lets future rules trust the tree
        // and the string in lock-step.
        fn flatten(nodes: &[Inline], out: &mut String) {
            for node in nodes {
                match node {
                    Inline::Text(t) => out.push_str(t),
                    Inline::Emphasis(span) => flatten(&span.children, out),
                }
            }
        }
        let md = "Before *italic with _nested_ inside* and after.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let para = &doc.sections[0].paragraphs[0];
        let mut flat = String::new();
        flatten(&para.inline, &mut flat);
        assert_eq!(flat, para.text, "tree {:?}", para.inline);
    }

    #[test]
    fn emphasis_position_points_at_opening_delimiter_on_later_line() {
        // Multi-line paragraph: the opening `*` of the emphasis is on
        // line 3 (after a blank line + a setup line). Confirm the
        // captured `start_line` matches and column counts from the
        // line start.
        let md = "Intro paragraph.\n\nFollow-up paragraph with *important* word.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let span = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .flat_map(|p| p.inline.iter())
            .find_map(|n| match n {
                Inline::Emphasis(s) => Some(s),
                Inline::Text(_) => None,
            })
            .expect("emphasis present");
        assert_eq!(span.start_line, 3);
        // "Follow-up paragraph with " is 25 chars; opening `*` at col 26.
        assert_eq!(span.start_column, 26);
    }

    // ---- F143 property tests ----
    //
    // These run before *and* after the lazy-build refactor (path B).
    // They pin invariants any correct inline-capture implementation
    // must satisfy, regardless of whether the tree is built eagerly
    // or lazily, and regardless of node-boundary choices the
    // implementation makes.

    fn flatten(nodes: &[Inline], out: &mut String) {
        for node in nodes {
            match node {
                Inline::Text(t) => out.push_str(t),
                Inline::Emphasis(span) => flatten(&span.children, out),
            }
        }
    }

    fn flatten_to_string(nodes: &[Inline]) -> String {
        let mut s = String::new();
        flatten(nodes, &mut s);
        s
    }

    /// Generate Markdown-ish input with optional emphasis spans
    /// sprinkled into prose. Alphabet constrained to ASCII
    /// alphanumerics + spaces so generated text rarely interacts with
    /// other Markdown constructs (headings, lists, code) — this
    /// strategy targets the inline-capture path specifically.
    fn proptest_md_input() -> impl proptest::strategy::Strategy<Value = String> {
        use proptest::prelude::*;
        prop::collection::vec(proptest_segment(), 1..6).prop_map(|segs| segs.join(" "))
    }

    fn proptest_segment() -> impl proptest::strategy::Strategy<Value = String> {
        use proptest::prelude::*;
        prop_oneof![
            // Plain word run.
            "[a-z]{1,8}( [a-z]{1,8}){0,5}".prop_map(String::from),
            // Asterisk-delimited emphasis.
            "[a-z]{1,8}( [a-z]{1,8}){0,3}".prop_map(|s| format!("*{s}*")),
            // Underscore-delimited emphasis.
            "[a-z]{1,8}( [a-z]{1,8}){0,3}".prop_map(|s| format!("_{s}_")),
        ]
    }

    fn proptest_plain_text() -> impl proptest::strategy::Strategy<Value = String> {
        use proptest::prelude::*;
        // No emphasis delimiters at all — for properties that quantify
        // over delimiter-free input (P2, P5).
        "[a-z ]{0,80}".prop_map(String::from)
    }

    proptest::proptest! {
        // Bound case count to keep the test run snappy; 256 is plenty
        // to catch the regressions we care about (allocation-shape
        // changes, node-boundary drift).
        #![proptest_config(proptest::prelude::ProptestConfig {
            cases: 256,
            ..proptest::prelude::ProptestConfig::default()
        })]

        // P1 — flatten invariant (path B contract). When the inline
        // tree is non-empty (i.e. emphasis was captured for this
        // paragraph), recursively flattening it must reproduce the
        // paragraph's `text` field. An empty inline tree is the
        // lazy-build path's "no spans worth modeling" signal and is
        // exempt from the flatten check — rules wanting paragraph
        // text walk `text` directly in that case. Load-bearing for
        // the condition that holds: rules walking `inline` and rules
        // walking `text` agree whenever both have content.
        #[test]
        fn prop_flatten_inline_equals_paragraph_text(
            input in proptest_md_input()
        ) {
            let doc = parse_markdown(&input, SourceFile::Anonymous);
            for para in doc.sections.iter().flat_map(|s| s.paragraphs.iter()) {
                if !para.inline.is_empty() {
                    let flat = flatten_to_string(&para.inline);
                    proptest::prop_assert_eq!(&flat, &para.text);
                }
            }
        }

        // P1b — lazy-build emptiness contract (new with path B). The
        // inline tree is empty *exactly when* the paragraph contained
        // no emphasis. Catches the regression where lazy-build leaves
        // an empty bottom frame for an emphasis-free paragraph (which
        // would still satisfy P1 but break the "is_empty == no spans"
        // contract F49 relies on).
        #[test]
        fn prop_inline_empty_iff_no_emphasis(
            input in proptest_md_input()
        ) {
            let doc = parse_markdown(&input, SourceFile::Anonymous);
            for para in doc.sections.iter().flat_map(|s| s.paragraphs.iter()) {
                let has_emphasis_in_tree = para
                    .inline
                    .iter()
                    .any(|n| matches!(n, Inline::Emphasis(_)));
                proptest::prop_assert_eq!(
                    !para.inline.is_empty(),
                    has_emphasis_in_tree,
                    "para.inline non-emptiness {} disagrees with emphasis presence {} for text {:?}",
                    !para.inline.is_empty(),
                    has_emphasis_in_tree,
                    para.text
                );
            }
        }

        // P2 — no-emphasis absence. Input without `*` or `_` must
        // produce no `Inline::Emphasis` node. Catches false-positive
        // emphasis capture (e.g. emitting Emphasis from Strong / Link).
        #[test]
        fn prop_no_delimiters_implies_no_emphasis(
            input in proptest_plain_text()
        ) {
            proptest::prop_assume!(!input.contains('*') && !input.contains('_'));
            let doc = parse_markdown(&input, SourceFile::Anonymous);
            for para in doc.sections.iter().flat_map(|s| s.paragraphs.iter()) {
                let any_emphasis = para
                    .inline
                    .iter()
                    .any(|n| matches!(n, Inline::Emphasis(_)));
                proptest::prop_assert!(!any_emphasis, "got {:?}", para.inline);
            }
        }

        // P3 — emphasis subset. Every emphasis span's flattened text
        // must appear as a contiguous substring of the parent
        // paragraph. Catches inline-tree drift where emphasis children
        // acquire or lose characters relative to the visible text.
        #[test]
        fn prop_emphasis_text_is_substring_of_paragraph(
            input in proptest_md_input()
        ) {
            for para in parse_markdown(&input, SourceFile::Anonymous)
                .sections
                .iter()
                .flat_map(|s| s.paragraphs.iter())
            {
                for node in &para.inline {
                    if let Inline::Emphasis(span) = node {
                        let inner = flatten_to_string(&span.children);
                        if !inner.is_empty() {
                            proptest::prop_assert!(
                                para.text.contains(&inner),
                                "emphasis {:?} not in paragraph {:?}",
                                inner,
                                para.text
                            );
                        }
                    }
                }
            }
        }

        // P4 — emphasis position fidelity. Every captured emphasis
        // `start_line` must fall within the paragraph's line range.
        // Catches off-by-one errors and stale positions leaking
        // across paragraph boundaries.
        #[test]
        fn prop_emphasis_position_within_paragraph(
            input in proptest_md_input()
        ) {
            for para in parse_markdown(&input, SourceFile::Anonymous)
                .sections
                .iter()
                .flat_map(|s| s.paragraphs.iter())
            {
                let nl = u32::try_from(para.text.matches('\n').count())
                    .unwrap_or(u32::MAX);
                let max_line = para.start_line.saturating_add(nl);
                for node in &para.inline {
                    if let Inline::Emphasis(span) = node {
                        proptest::prop_assert!(
                            span.start_line >= para.start_line
                                && span.start_line <= max_line,
                            "span line {} outside [{}, {}] for para {:?}",
                            span.start_line,
                            para.start_line,
                            max_line,
                            para.text
                        );
                        proptest::prop_assert!(
                            span.start_column >= 1,
                            "span column {} is not 1-based",
                            span.start_column
                        );
                    }
                }
            }
        }

        // P5 — plain-text empty inline. `parse_plain` never produces
        // an inline tree, regardless of input shape. Pins the
        // contract that Markdown semantics do not bleed into the
        // plain-text path.
        #[test]
        fn prop_parse_plain_has_empty_inline(
            input in proptest_plain_text()
        ) {
            let doc = super::super::parse_plain(&input, SourceFile::Anonymous);
            for para in doc.sections.iter().flat_map(|s| s.paragraphs.iter()) {
                proptest::prop_assert!(
                    para.inline.is_empty(),
                    "got {:?}",
                    para.inline
                );
            }
        }
    }

    // ---- F143 golden-fixture snapshots ----
    //
    // Locked in on path A. After the lazy-build refactor (path B),
    // these must match byte-for-byte — that's the fixture-level
    // equivalence check. Property tests guarantee invariant
    // preservation; these guarantee node-boundary preservation.
    //
    // Bilingual corpus (EN + FR) since the substrate is language-
    // agnostic and AGENTS.md §6.4 mandates bilingual coverage.
    // Debug-snapshot avoids the serde::Serialize derive on Inline
    // (production type stays minimal).

    fn snapshot_inline(md: &str) -> Vec<Vec<Inline>> {
        parse_markdown(md, SourceFile::Anonymous)
            .sections
            .into_iter()
            .flat_map(|s| s.paragraphs)
            .map(|p| p.inline)
            .collect()
    }

    #[test]
    fn snapshot_en_plain_paragraph() {
        // Baseline: no emphasis, no Markdown decoration. Inline tree
        // collapses to a single Text node containing the paragraph
        // verbatim. Sets the "common case" reference.
        let trees = snapshot_inline("Plain prose, nothing fancy here.");
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_en_single_emphasis() {
        let trees = snapshot_inline("Some *italic words* in the middle of prose.");
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_en_strong_does_not_create_emphasis() {
        // Path A flattens **bold** into Text. Path B must do the same
        // (the substrate's narrow-by-design contract).
        let trees = snapshot_inline("A line with **strong words** but no italic.");
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_en_multi_paragraph_mixed() {
        // Three paragraphs: plain → emphasis → strong + emphasis.
        // Pins paragraph-boundary state reset (no inline-stack
        // bleed-through across paragraphs).
        let md = "First paragraph, plain.\n\n\
                  Second has *italic* in it.\n\n\
                  Third has **bold** and *italic both*.";
        let trees = snapshot_inline(md);
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_en_nested_emphasis() {
        // Outer asterisk wraps inner underscore. Tree round-trips
        // both spans with nested children.
        let trees = snapshot_inline("Outer *one _two_ three* end.");
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_en_tight_list_item_with_emphasis() {
        // F129 synthesis path + F143 capture combined.
        let trees = snapshot_inline("- bullet with *italic phrase* inside.\n");
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_en_code_block_excluded() {
        // Emphasis inside a fenced code block must not appear in any
        // paragraph's inline tree.
        let md = "Before italics.\n\n\
                  ```\n\
                  not *italic* here\n\
                  ```\n\n\
                  After *real italics* end.";
        let trees = snapshot_inline(md);
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_fr_single_emphasis() {
        // FR mirror of the single-emphasis case. The substrate is
        // language-agnostic; the snapshot proves it.
        let trees = snapshot_inline("Une phrase avec *des mots en italique* au milieu.");
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_fr_nested_emphasis_with_accents() {
        // Accented characters touch the column-counting path
        // (offset_to_line_col uses chars().count(), not bytes).
        // Locks the FR positions in too.
        let trees = snapshot_inline("Élève *attentif _très_ concentré* ici.");
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn snapshot_fr_multi_paragraph_with_lists() {
        let md = "Paragraphe simple.\n\n\
                  - item avec *italique court* ici\n\
                  - autre item avec **gras** et _souligné_\n\n\
                  Conclusion en *italique final*.";
        let trees = snapshot_inline(md);
        insta::assert_debug_snapshot!(trees);
    }

    #[test]
    fn emphasis_inside_heading_does_not_bleed_into_next_paragraph() {
        // Mutation-test gap (cargo-mutants 2026-05-02 on path A):
        // `Event::Start/End(Emphasis) if in_paragraph` survived its
        // guard-removal mutation because the inline-stack cleanup on
        // every `Tag::Paragraph` start papered over heading-leaked
        // frames. Path B's lazy-build flips the contract: emphasis-
        // free paragraphs ship with `inline.is_empty()`. The leak
        // would manifest as the body paragraph carrying a stale
        // `Emphasis` frame from the heading, which would fail this
        // assertion.
        let md = "# Heading with *italic title*\n\nA body paragraph, no emphasis here.";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let para = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .next()
            .expect("body paragraph present");
        assert_eq!(para.text, "A body paragraph, no emphasis here.");
        // Strongest available assertion under path B: the body
        // paragraph carries no inline tree at all — no emphasis
        // captured, lazy-build never activated.
        assert!(
            para.inline.is_empty(),
            "heading emphasis leaked into body paragraph: {:?}",
            para.inline
        );
    }

    #[test]
    fn emphasis_inside_tight_list_item_is_captured() {
        // F129's tight-list paragraph synthesis must also seed the
        // inline-stack frame, otherwise emphasis in a single-bullet
        // list goes unmodeled.
        let md = "- bullet with *italic phrase* inside.\n";
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let para = doc
            .sections
            .iter()
            .flat_map(|s| s.paragraphs.iter())
            .next()
            .expect("paragraph synthesized");
        assert!(para.from_list_item);
        let has_emphasis = para.inline.iter().any(|n| matches!(n, Inline::Emphasis(_)));
        assert!(has_emphasis, "got {:?}", para.inline);
    }
}
