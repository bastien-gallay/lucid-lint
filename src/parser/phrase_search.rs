//! Shared word-bounded phrase search used by lexical rules.
//!
//! Several rules (`weasel-words`, `jargon-undefined`,
//! `repetitive-connectors`) scan prose for case-insensitive, word-bounded
//! occurrences of a fixed phrase and, for some of them, need the
//! `(line, column)` at which each hit starts. Before this module, every
//! rule carried its own copy; the copies drifted in subtle ways and all
//! shared the same latent panic on non-ASCII input (byte offsets that
//! land mid-UTF-8).
//!
//! Callers pass a pre-lowercased haystack. Returned byte offsets index
//! into that haystack; when converting to a `(line, column)` in the
//! original text, [`line_column_at`] snaps to the nearest valid char
//! boundary so multi-byte characters can never trigger a mid-char slice
//! panic.

/// Find every word-bounded occurrence of `needle` in `haystack_lower`.
///
/// Both arguments MUST already be lowercase. `needle` may not be empty;
/// if it is, the result is an empty `Vec`.
///
/// A "word boundary" here treats alphabetic characters and `'` as word
/// characters; everything else breaks a word. The definition matches
/// the behavior expected by the phrase-based lexical rules.
#[must_use]
pub fn find_word_bounded(haystack_lower: &str, needle_lower: &str) -> Vec<usize> {
    if needle_lower.is_empty() {
        return Vec::new();
    }
    let mut hits = Vec::new();
    let mut start = 0;
    while let Some(found) = haystack_lower[start..].find(needle_lower) {
        let abs = start + found;
        if is_word_boundary(haystack_lower, abs, abs + needle_lower.len()) {
            hits.push(abs);
        }
        start = abs + needle_lower.len();
        if start > haystack_lower.len() {
            break;
        }
    }
    hits
}

/// Count word-bounded occurrences of `needle` in `haystack_lower`.
///
/// Equivalent to `find_word_bounded(..).len()` but without the
/// intermediate `Vec` allocation.
#[must_use]
pub fn count_word_bounded(haystack_lower: &str, needle_lower: &str) -> usize {
    if needle_lower.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut start = 0;
    while let Some(found) = haystack_lower[start..].find(needle_lower) {
        let abs = start + found;
        if is_word_boundary(haystack_lower, abs, abs + needle_lower.len()) {
            count += 1;
        }
        start = abs + needle_lower.len();
        if start > haystack_lower.len() {
            break;
        }
    }
    count
}

/// Compute `(line_offset, column)` for a byte offset into a paragraph.
///
/// `line_offset` is 0-based (0 = same line as paragraph start); `column`
/// is 1-based and counted in `char`s within the current line.
///
/// Safety: `byte_offset` is clamped to `text.len()` and snapped down to
/// the nearest char boundary before slicing, so passing an offset that
/// came from a lowercased variant (where byte positions may diverge
/// from the original for a few exotic characters) cannot panic.
#[must_use]
pub fn line_column_at(text: &str, byte_offset: usize) -> (u32, u32) {
    let mut capped = byte_offset.min(text.len());
    while capped > 0 && !text.is_char_boundary(capped) {
        capped -= 1;
    }
    let prefix = &text[..capped];
    #[allow(clippy::naive_bytecount)]
    let line_offset =
        u32::try_from(prefix.bytes().filter(|&b| b == b'\n').count()).unwrap_or(u32::MAX);
    let current_line_start = prefix.rfind('\n').map_or(0, |pos| pos + 1);
    let column =
        u32::try_from(text[current_line_start..capped].chars().count() + 1).unwrap_or(u32::MAX);
    (line_offset, column)
}

fn is_word_boundary(s: &str, start: usize, end: usize) -> bool {
    let before_ok = start == 0 || !s[..start].chars().next_back().is_some_and(is_word_char);
    let after_ok = end >= s.len() || !s[end..].chars().next().is_some_and(is_word_char);
    before_ok && after_ok
}

fn is_word_char(c: char) -> bool {
    c.is_alphabetic() || c == '\''
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_needle_yields_no_hits() {
        assert!(find_word_bounded("hello world", "").is_empty());
        assert_eq!(count_word_bounded("hello world", ""), 0);
    }

    #[test]
    fn word_bounded_match_ignores_substrings() {
        // "cat" should not match inside "category".
        assert_eq!(find_word_bounded("category of cat", "cat"), vec![12]);
        assert_eq!(count_word_bounded("category of cat", "cat"), 1);
    }

    #[test]
    fn line_column_at_handles_multi_byte_prefix() {
        // "é" is 2 bytes in UTF-8. A hit at the word "test" starts at
        // byte 3, but column-wise it's the 3rd char (1-based).
        let text = "é, test";
        let (line, col) = line_column_at(text, 4); // byte offset of 't'
        assert_eq!(line, 0);
        assert_eq!(col, 4);
    }

    #[test]
    fn line_column_at_snaps_to_char_boundary() {
        // Offset lands mid-UTF-8: used to panic, now snaps down.
        let text = "é";
        let (_, col) = line_column_at(text, 1);
        assert_eq!(col, 1);
    }

    #[test]
    fn line_column_at_counts_lines() {
        let text = "first line\nsecond line\nthird";
        let (line, col) = line_column_at(text, 23); // 't' of "third"
        assert_eq!(line, 2);
        assert_eq!(col, 1);
    }
}
