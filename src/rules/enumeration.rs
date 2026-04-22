//! Shared helper: detect inline prose enumerations.
//!
//! Used by two rules with different thresholds:
//!
//! - [`crate::rules::structure::long_enumeration`] flags enumerations
//!   of 5+ items as "convert to a bulleted list" suggestions.
//! - [`crate::rules::structure::excessive_commas`] discounts commas
//!   inside detected enumerations so that sentences like "red, green,
//!   blue, and yellow" don't trip the comma-density threshold.
//!
//! Detection recognizes the Oxford-comma pattern documented in
//! `RULES.md` for `long-enumeration`: a run of comma-separated short
//! segments ending with a connector segment (`, and X` / `, or X` /
//! `, et X` / `, ou X`). Non-Oxford enumerations ("A, B, C and D" with
//! no comma before the connector) are deliberately out of scope for
//! v0.1.

use crate::types::Language;

/// Maximum number of words per segment for it to count as "short".
/// Longer segments disqualify the enumeration pattern. Kept deliberately
/// tight (2) so subordinate clauses and non-enumerable prose fragments
/// do not get swallowed into a false enumeration when walking backwards
/// from a trailing connector. Real enumerations with 3+ word items
/// ("red cherries, green apples, blue jays") are missed — acceptable
/// for v0.1; a smarter detector can lift the limit later.
const MAX_SEGMENT_WORDS: usize = 2;

/// Minimum number of items before we recognize a sequence as an
/// enumeration at all (used for comma-discounting). `long-enumeration`
/// layers its own `min_items` on top.
const MIN_ITEMS_FOR_DETECTION: u32 = 3;

/// A detected prose enumeration within a sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumeration {
    /// Byte offset (in the sentence) where the enumeration starts.
    pub start: usize,

    /// Byte offset (exclusive) where the enumeration ends.
    pub end: usize,

    /// Number of items (comma-separated segments, including the trailing
    /// connector segment).
    pub items: u32,

    /// Number of commas that separate the items.
    pub commas: u32,
}

/// Detect Oxford-style prose enumerations in a sentence.
///
/// Returns at most one enumeration per run; overlapping enumerations are
/// collapsed into the widest one seen.
#[must_use]
pub fn detect_enumerations(sentence: &str, language: Language) -> Vec<Enumeration> {
    let connectors: &[&str] = match language {
        Language::En => &["and", "or"],
        Language::Fr => &["et", "ou"],
        Language::Unknown => return Vec::new(),
    };

    let segments = split_comma_segments(sentence);
    let mut result: Vec<Enumeration> = Vec::new();

    for (i, seg) in segments.iter().enumerate() {
        if !segment_starts_with_connector(&sentence[seg.range.clone()], connectors) {
            continue;
        }
        // Walk backwards over consecutive short segments. Stop at another
        // connector segment, so a second enumeration anchor doesn't scoop
        // up a preceding one ("..., green, and blue, carefully, and quietly"
        // must not be detected as a single 4-item enumeration).
        let mut start_idx = i;
        while start_idx > 0 {
            let prev_text = &sentence[segments[start_idx - 1].range.clone()];
            if !is_short_segment(prev_text) {
                break;
            }
            if segment_starts_with_connector(prev_text, connectors) {
                break;
            }
            start_idx -= 1;
        }
        let items = u32::try_from(i - start_idx + 1).unwrap_or(u32::MAX);
        if items < MIN_ITEMS_FOR_DETECTION {
            continue;
        }
        let start_byte = segments[start_idx].range.start;
        let end_byte = seg.range.end;
        // Commas between items: for N items there are N-1 separators.
        let commas = items.saturating_sub(1);
        push_or_merge(
            &mut result,
            Enumeration {
                start: start_byte,
                end: end_byte,
                items,
                commas,
            },
        );
    }
    result
}

/// Total number of commas that fall inside any detected enumeration.
/// Used by `excessive-commas` to discount enumeration-commas.
#[must_use]
pub fn enumeration_comma_count(sentence: &str, language: Language) -> u32 {
    detect_enumerations(sentence, language)
        .iter()
        .map(|e| e.commas)
        .sum()
}

/// Count commas inside `(A, B, C, …)` parenthesised token lists.
///
/// Recognises three or more short comma-separated segments inside a
/// balanced pair of parentheses. Language-agnostic: parens behave the
/// same in EN and FR, and no connector word is required.
///
/// Used by `excessive-commas` as a second discount source alongside
/// [`enumeration_comma_count`], to suppress the dominant FP driver on
/// doc prose (F22): backticked identifier lists, example enumerations,
/// and other "aside" lists that the Oxford-only detector never sees.
///
/// Only the outermost parenthesised run is considered — nested parens
/// disqualify the containing span to keep the heuristic conservative.
#[must_use]
pub fn parenthesised_list_comma_count(sentence: &str) -> u32 {
    let bytes = sentence.as_bytes();
    let mut total: u32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'(' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut depth = 1usize;
        let mut j = start;
        while j < bytes.len() {
            match bytes[j] {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                },
                _ => {},
            }
            j += 1;
        }
        if depth != 0 {
            // Unbalanced parenthesis: give up on the rest of the sentence.
            break;
        }
        let inner = &sentence[start..j];
        if !inner.contains('(') {
            total = total.saturating_add(parenthesised_run_comma_count(inner));
        }
        i = j + 1;
    }
    total
}

fn parenthesised_run_comma_count(inner: &str) -> u32 {
    let segments: Vec<&str> = inner.split(',').collect();
    if segments.len() < MIN_ITEMS_FOR_DETECTION as usize {
        return 0;
    }
    // Empty segments are accepted: the markdown parser strips inline
    // code contents, so `(`a`, `b`, `c`)` reaches us as `(, , )`. A
    // surviving non-empty segment would still need to be short.
    if !segments
        .iter()
        .all(|s| s.split_whitespace().count() <= MAX_SEGMENT_WORDS)
    {
        return 0;
    }
    u32::try_from(segments.len() - 1).unwrap_or(u32::MAX)
}

struct Segment {
    range: std::ops::Range<usize>,
}

/// Split a sentence into comma-separated segments, preserving byte
/// ranges. Commas are excluded from the segment ranges.
fn split_comma_segments(sentence: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let bytes = sentence.as_bytes();
    let mut start = 0;
    for (idx, &b) in bytes.iter().enumerate() {
        if b == b',' {
            segments.push(Segment { range: start..idx });
            start = idx + 1;
        }
    }
    segments.push(Segment {
        range: start..bytes.len(),
    });
    segments
}

/// Whether a segment starts (after trimming) with one of the given
/// connector words followed by whitespace.
fn segment_starts_with_connector(segment: &str, connectors: &[&str]) -> bool {
    let trimmed = segment.trim_start();
    let lower = trimmed.to_lowercase();
    for connector in connectors {
        if let Some(rest) = lower.strip_prefix(connector) {
            if rest.chars().next().is_some_and(char::is_whitespace) {
                return true;
            }
        }
    }
    false
}

/// Whether a segment is "short enough" to be an enumeration item.
fn is_short_segment(segment: &str) -> bool {
    let words = segment.split_whitespace().count();
    words > 0 && words <= MAX_SEGMENT_WORDS
}

/// Push a new enumeration onto the result, merging with the last one if
/// they overlap or touch (the rightmost anchor absorbs the leftmost).
fn push_or_merge(out: &mut Vec<Enumeration>, candidate: Enumeration) {
    if let Some(last) = out.last_mut() {
        if candidate.start <= last.end {
            // Extend the existing enumeration rightwards.
            last.end = candidate.end.max(last.end);
            last.items = last.items.max(candidate.items);
            last.commas = last.commas.max(candidate.commas);
            return;
        }
    }
    out.push(candidate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_english_oxford_enumeration() {
        let s = "red, green, blue, and yellow";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
        assert_eq!(enums[0].commas, 3);
    }

    #[test]
    fn detects_french_oxford_enumeration() {
        let s = "rouge, vert, bleu, et jaune";
        let enums = detect_enumerations(s, Language::Fr);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
    }

    #[test]
    fn detects_or_as_connector() {
        let s = "a, b, c, or d";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
    }

    #[test]
    fn ignores_non_oxford_form() {
        // No comma before "and" → out of scope for v0.1.
        let s = "red, green, blue and yellow";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn needs_at_least_three_items() {
        let s = "a, and b";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn rejects_long_segments() {
        // Second segment has 8 words — too long for the pattern.
        let s =
            "first, the second segment is way too long to count as a short item, third, and fourth";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn does_not_swallow_surrounding_subordinates() {
        // The real enumeration here is only 2 items ("green, and blue files"),
        // below the minimum, so no enumeration should be reported.
        let s = "Note, although we agreed, to pack the red, green, and blue files, carefully";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn enumeration_comma_count_sums_all_enumerations() {
        let s = "red, green, blue, and yellow";
        assert_eq!(enumeration_comma_count(s, Language::En), 3);
    }

    #[test]
    fn unknown_language_disables_detection() {
        let s = "red, green, blue, and yellow";
        assert!(detect_enumerations(s, Language::Unknown).is_empty());
    }

    #[test]
    fn parenthesised_list_counts_inner_commas() {
        let s = "Pick a colour (red, green, blue, yellow) for the frame.";
        assert_eq!(parenthesised_list_comma_count(s), 3);
    }

    #[test]
    fn parenthesised_list_needs_three_segments() {
        // Two-item parens are asides, not lists.
        let s = "The pair (foo, bar) matters.";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_rejects_long_segments() {
        // One segment has > MAX_SEGMENT_WORDS words.
        let s = "See (red, a very long qualifying clause here, blue, yellow).";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_ignores_nested_parens() {
        // Nested parens disqualify the containing span.
        let s = "See (red, green (emerald), blue, yellow) here.";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_ignores_unbalanced_parens() {
        let s = "See (red, green, blue here.";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_handles_multiple_runs() {
        let s = "Digits (`1`, `2`, `3`) and spellings (`one`, `two`, `three`) differ.";
        assert_eq!(parenthesised_list_comma_count(s), 4);
    }

    #[test]
    fn parenthesised_list_counts_empty_segments_from_stripped_code() {
        // The markdown parser drops inline code contents, so
        // `(`a`, `b`, `c`, `d`)` reaches rules as `(, , , )`.
        let s = "The tokens (, , , ) are listed.";
        assert_eq!(parenthesised_list_comma_count(s), 3);
    }

    #[test]
    fn parenthesised_list_is_language_agnostic() {
        // Same sentence shape, no connector word — both languages see it.
        let s = "Voyelles (`a`, `e`, `i`, `o`, `u`) courantes.";
        assert_eq!(parenthesised_list_comma_count(s), 4);
    }

    #[test]
    fn case_insensitive_connector() {
        let s = "red, green, blue, And yellow";
        assert_eq!(detect_enumerations(s, Language::En).len(), 1);
    }
}
