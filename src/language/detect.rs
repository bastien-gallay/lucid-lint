//! Language detection by stop-word ratio heuristic.
//!
//! This is a pragmatic v0.1 implementation. For better accuracy on short or
//! ambiguous texts, the `whatlang` crate is on the roadmap.

use unicode_segmentation::UnicodeSegmentation;

use crate::language::{en, fr};
use crate::types::Language;

/// Minimum ratio difference between top candidates to return a confident result.
///
/// If the gap between the best and second-best language is below this,
/// we return [`Language::Unknown`].
const CONFIDENCE_GAP: f64 = 0.02;

/// Minimum number of words required for detection to be meaningful.
///
/// Below this, detection is unreliable and we return [`Language::Unknown`].
const MIN_WORDS: usize = 10;

/// Detect the language of a text using stop-word frequency.
///
/// Returns [`Language::Unknown`] when the text is too short or confidence is low.
///
/// # Examples
///
/// ```
/// use lucid_lint::language::detect_language;
/// use lucid_lint::Language;
///
/// let text = "The quick brown fox jumps over the lazy dog. The dog was not amused.";
/// assert_eq!(detect_language(text), Language::En);
/// ```
#[must_use]
pub fn detect_language(text: &str) -> Language {
    let words: Vec<&str> = text.unicode_words().collect();

    if words.len() < MIN_WORDS {
        return Language::Unknown;
    }

    let total = words.len() as f64;
    let lower: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();

    let en_hits = lower.iter().filter(|w| en::STOPWORDS.contains(w.as_str())).count() as f64;
    let fr_hits = lower.iter().filter(|w| fr::STOPWORDS.contains(w.as_str())).count() as f64;

    let en_ratio = en_hits / total;
    let fr_ratio = fr_hits / total;

    if (en_ratio - fr_ratio).abs() < CONFIDENCE_GAP {
        return Language::Unknown;
    }

    if en_ratio > fr_ratio {
        Language::En
    } else {
        Language::Fr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_english_text() {
        let text = "The quick brown fox jumps over the lazy dog. \
                    The dog was not amused by this turn of events.";
        assert_eq!(detect_language(text), Language::En);
    }

    #[test]
    fn detect_french_text() {
        let text = "Le renard rapide saute par-dessus le chien paresseux. \
                    Le chien n'est pas content de cette situation.";
        assert_eq!(detect_language(text), Language::Fr);
    }

    #[test]
    fn too_short_returns_unknown() {
        assert_eq!(detect_language("Hi there"), Language::Unknown);
    }

    #[test]
    fn empty_text_returns_unknown() {
        assert_eq!(detect_language(""), Language::Unknown);
    }

    #[test]
    fn ambiguous_text_may_return_unknown() {
        // A text with no stop words in either language.
        let text = "xenon krypton argon helium neon radon carbon silicon sulfur phosphorus";
        // Without stop-word matches, ratios are both 0, so within CONFIDENCE_GAP.
        assert_eq!(detect_language(text), Language::Unknown);
    }
}
