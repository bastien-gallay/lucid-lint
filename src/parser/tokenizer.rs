//! Sentence and word tokenization.
//!
//! Sentence splitting is NOT a naive split on `.`. It respects:
//! - Abbreviations (`Dr.`, `e.g.`, `etc.`, `cf.`, `M.`)
//! - Decimal numbers (`3.14`)
//! - Ellipses (`...`)
//! - Initials (`J. R. R. Tolkien`) — within reason
//!
//! Word counting uses Unicode grapheme-aware word segmentation.

use std::sync::LazyLock;

use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

use super::document::Sentence;

/// Abbreviations that end with a period but do NOT end a sentence.
///
/// Case is ignored during matching.
static ABBREVIATIONS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // Titles
        "mr", "mrs", "ms", "dr", "prof", "sr", "jr", "m", "mme", "mlle",
        // Common Latin abbreviations
        "e.g", "i.e", "etc", "cf", "vs", "e", "g", "i", // Measurement and misc
        "no", "vol", "pp", "fig", "ca", // French
        "ex", "av", "apr", "p",
    ]
    .into_iter()
    .collect()
});

/// Split a text into sentences.
///
/// Returns an empty vector for empty or whitespace-only input.
///
/// # Examples
///
/// ```
/// use lucid_lint::parser::split_sentences;
/// let sentences = split_sentences("Hello world. How are you?", 1, 1);
/// assert_eq!(sentences.len(), 2);
/// ```
#[must_use]
pub fn split_sentences(text: &str, start_line: u32, start_column: u32) -> Vec<Sentence> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let mut sentences = Vec::new();
    let mut current = String::new();
    let mut line = start_line;
    let mut column = start_column;
    let mut sentence_line = line;
    let mut sentence_column = column;

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        if current.is_empty() && !c.is_whitespace() {
            sentence_line = line;
            sentence_column = column;
        }

        current.push(c);

        if is_sentence_terminator(c) {
            // Handle ellipsis: consume remaining dots as one terminator.
            if c == '.' {
                while i + 1 < len && chars[i + 1] == '.' {
                    i += 1;
                    current.push('.');
                    column += 1;
                }
            }

            // Decide whether this is a real sentence end.
            let is_end = is_real_sentence_end(&chars, i, &current);

            if is_end {
                let text = current.trim().to_string();
                if !text.is_empty() {
                    sentences.push(Sentence::new(text, sentence_line, sentence_column));
                }
                current.clear();
            }
        }

        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }

        i += 1;
    }

    let trailing = current.trim();
    if !trailing.is_empty() {
        sentences.push(Sentence::new(
            trailing.to_string(),
            sentence_line,
            sentence_column,
        ));
    }

    sentences
}

/// Count words in a text using Unicode word segmentation.
///
/// English contractions (`don't`) and French article/preposition elisions
/// (`l'accessibilité`, `d'une`) are counted as one word. French
/// pronoun/conjunction elisions (`c'est`, `j'ai`, `qu'il`) count as two,
/// because the elided clitic stands in for a full grammatical word.
#[must_use]
pub fn word_count(text: &str) -> u32 {
    text.unicode_words().map(count_unicode_word).sum()
}

/// Return how many grammatical words a single Unicode-segmented word is worth.
fn count_unicode_word(word: &str) -> u32 {
    let lower = word.to_lowercase();
    let Some((head, tail)) = lower.split_once('\'') else {
        return 1;
    };
    let tail_starts_with_letter = tail.chars().next().is_some_and(char::is_alphabetic);
    if tail_starts_with_letter && matches!(head, "c" | "j" | "m" | "n" | "qu" | "s" | "t") {
        2
    } else {
        1
    }
}

const fn is_sentence_terminator(c: char) -> bool {
    matches!(c, '.' | '!' | '?' | '…')
}

fn is_real_sentence_end(chars: &[char], idx: usize, current: &str) -> bool {
    let c = chars[idx];

    // Non-period terminators always end a sentence in v0.1.
    if c != '.' {
        return true;
    }

    // Check for decimal number: digit before AND digit after.
    if idx > 0
        && chars[idx - 1].is_ascii_digit()
        && idx + 1 < chars.len()
        && chars[idx + 1].is_ascii_digit()
    {
        return false;
    }

    // Ellipsis followed by lowercase: the thought continues, not a real end.
    // `current` already contains all the dots we just consumed.
    if current.ends_with("..") {
        if let Some(next) = chars[idx + 1..].iter().find(|c| !c.is_whitespace()) {
            if next.is_lowercase() {
                return false;
            }
        }
    }

    // Check for abbreviation: token before the period matches.
    let trimmed = current.trim_end_matches('.');
    if let Some(last_token) = trimmed.rsplit(|c: char| c.is_whitespace()).next() {
        let normalized = last_token.to_lowercase();
        // Abbreviations can appear with internal dots ("e.g"); keep the trailing
        // form intact for lookup.
        if ABBREVIATIONS.contains(normalized.trim_end_matches('.')) {
            return false;
        }
    }

    // Single-letter initial: not a sentence end when followed by whitespace then uppercase.
    if trimmed.len() <= 1
        && idx + 2 < chars.len()
        && chars[idx + 1].is_whitespace()
        && chars[idx + 2].is_uppercase()
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_simple_sentences() {
        let s = split_sentences("Hello world. How are you? Fine.", 1, 1);
        assert_eq!(s.len(), 3);
        assert_eq!(s[0].text, "Hello world.");
        assert_eq!(s[1].text, "How are you?");
        assert_eq!(s[2].text, "Fine.");
    }

    #[test]
    fn does_not_split_on_decimals() {
        let s = split_sentences("Pi is about 3.14 according to math.", 1, 1);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn does_not_split_on_common_abbreviations() {
        let s = split_sentences("Dr. Smith arrived. He was late.", 1, 1);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].text, "Dr. Smith arrived.");
    }

    #[test]
    fn does_not_split_on_eg() {
        let s = split_sentences("Use a browser, e.g. Firefox or Chrome. It works.", 1, 1);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn handles_ellipsis() {
        let s = split_sentences("Wait... something is wrong. Indeed.", 1, 1);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn handles_question_and_exclamation() {
        let s = split_sentences("Really? Yes! Absolutely.", 1, 1);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn empty_input_yields_empty() {
        assert!(split_sentences("", 1, 1).is_empty());
        assert!(split_sentences("   \n  ", 1, 1).is_empty());
    }

    #[test]
    fn sentence_without_terminator_is_still_returned() {
        let s = split_sentences("Unterminated sentence", 1, 1);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].text, "Unterminated sentence");
    }

    #[test]
    fn french_abbreviation_dr() {
        let s = split_sentences("Le Dr. Dupont est arrivé. Il est en retard.", 1, 1);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn word_count_basic() {
        assert_eq!(word_count("Hello world"), 2);
        assert_eq!(word_count("one two three four"), 4);
    }

    #[test]
    fn word_count_contractions() {
        // "don't" counts as one word per Unicode word segmentation.
        assert_eq!(word_count("I don't know"), 3);
    }

    #[test]
    fn word_count_french_elision() {
        // "l'accessibilité" counts as one word.
        assert_eq!(word_count("C'est l'accessibilité numérique"), 4);
    }

    #[test]
    fn word_count_empty() {
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   "), 0);
    }
}
