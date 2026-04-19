//! English-specific language data.
//!
//! Used by [`crate::language::detect_language`] for language detection,
//! and by lexical rules to exclude function words from content analysis.

use std::sync::LazyLock;

use std::collections::HashSet;

/// Common English stop-words (function words).
///
/// This list is deliberately kept to highly frequent function words to minimize
/// false detection of English in mixed-language texts. It is NOT a full stop-word
/// list suitable for information retrieval.
pub static STOPWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // Articles
        "the",
        "a",
        "an",
        // Pronouns
        "i",
        "you",
        "he",
        "she",
        "it",
        "we",
        "they",
        "me",
        "him",
        "her",
        "us",
        "them",
        "my",
        "your",
        "his",
        "its",
        "our",
        "their",
        "this",
        "that",
        "these",
        "those",
        // Auxiliaries and be-forms
        "is",
        "are",
        "was",
        "were",
        "be",
        "been",
        "being",
        "have",
        "has",
        "had",
        "having",
        "do",
        "does",
        "did",
        "done",
        "will",
        "would",
        "shall",
        "should",
        "can",
        "could",
        "may",
        "might",
        "must",
        // Prepositions
        "of",
        "in",
        "on",
        "at",
        "to",
        "from",
        "for",
        "by",
        "with",
        "about",
        "into",
        "through",
        "during",
        "before",
        "after",
        "above",
        "below",
        "between",
        "under",
        "over",
        // Conjunctions
        "and",
        "or",
        "but",
        "nor",
        "so",
        "yet",
        "if",
        "because",
        "as",
        "than",
        "that",
        "while",
        "when",
        "where",
        "whether",
        "although",
        // Common adverbs and fillers
        "not",
        "no",
        "yes",
        "only",
        "just",
        "also",
        "very",
        "too",
        "here",
        "there",
        "now",
        "then",
        // Common contractions (lowercased)
        "don't",
        "doesn't",
        "didn't",
        "won't",
        "wouldn't",
        "can't",
        "couldn't",
        "shouldn't",
        "isn't",
        "aren't",
        "wasn't",
        "weren't",
        "haven't",
        "hasn't",
        "hadn't",
        "i'm",
        "you're",
        "he's",
        "she's",
        "it's",
        "we're",
        "they're",
        "i've",
        "you've",
        "we've",
        "they've",
        "i'll",
        "you'll",
        "he'll",
        "she'll",
        "we'll",
        "they'll",
    ]
    .into_iter()
    .collect()
});

/// Default English weasel words and phrases (lowercased).
///
/// A weasel word weakens a statement without informing the reader: the
/// reader must silently decide whether the qualification matters. See
/// [`RULES.md`](../../RULES.md#weasel-words).
pub static WEASELS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "some",
        "many",
        "often",
        "just",
        "simply",
        "clearly",
        "obviously",
        "seemingly",
        "arguably",
        "basically",
        "essentially",
        "virtually",
        "various",
        "numerous",
        "rather",
        "quite",
        "sort of",
        "kind of",
        "a bit",
    ]
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_common_articles() {
        assert!(STOPWORDS.contains("the"));
        assert!(STOPWORDS.contains("a"));
        assert!(STOPWORDS.contains("an"));
    }

    #[test]
    fn contains_common_auxiliaries() {
        assert!(STOPWORDS.contains("is"));
        assert!(STOPWORDS.contains("have"));
        assert!(STOPWORDS.contains("will"));
    }

    #[test]
    fn does_not_contain_content_words() {
        assert!(!STOPWORDS.contains("accessibility"));
        assert!(!STOPWORDS.contains("linter"));
    }
}
