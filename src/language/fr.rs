//! French-specific language data.
//!
//! Used by [`crate::language::detect_language`] for language detection,
//! and by lexical rules to exclude function words from content analysis.

use std::sync::LazyLock;

use std::collections::HashSet;

/// Common French stop-words (mots outils).
///
/// Kept focused on highly frequent function words for reliable detection
/// on short texts. Elisions (`l'`, `d'`, `qu'`) are included because
/// `unicode_words` keeps the apostrophe attached.
pub static STOPWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // Articles
        "le", "la", "les", "un", "une", "des", "du", "au", "aux", "l'", "d'",
        // Contracted articles and elisions appearing as standalone tokens
        "l", "d", "j", "m", "t", "s", "n", "c", "qu", // Pronouns
        "je", "tu", "il", "elle", "on", "nous", "vous", "ils", "elles", "me", "te", "se", "lui",
        "leur", "leurs", "mon", "ton", "son", "ma", "ta", "sa", "mes", "tes", "ses", "notre",
        "votre", "nos", "vos", "ce", "cet", "cette", "ces", "celui", "celle", "ceux", "celles",
        "ça", "cela", "ceci", // Common verbs: être
        "suis", "es", "est", "sommes", "êtes", "sont", "était", "étaient", "étais", "étions",
        "étiez", "été", // Common verbs: avoir
        "ai", "as", "a", "avons", "avez", "ont", "avait", "avaient", "avais", "avions", "aviez",
        "eu", // Modal-ish verbs
        "peut", "peux", "pouvez", "pouvons", "peuvent", "doit", "dois", "devez", "devons",
        "doivent", "va", "vais", "vas", "allons", "allez", "vont", // Prepositions
        "de", "à", "en", "dans", "sur", "sous", "par", "pour", "avec", "sans", "entre", "vers",
        "chez", "contre", "depuis", // Conjunctions
        "et", "ou", "mais", "donc", "or", "ni", "car", "que", "qui", "quoi", "dont", "où", "si",
        "comme", "parce", "puisque", "lorsque", "quand", // Common adverbs and fillers
        "pas", "ne", "non", "oui", "plus", "moins", "très", "trop", "ici", "là", "bien", "mal",
        "aussi", "encore", "déjà", // Common frequent words
        "tout", "tous", "toute", "toutes", "même", "mêmes", "autre", "autres",
    ]
    .into_iter()
    .collect()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_common_articles() {
        assert!(STOPWORDS.contains("le"));
        assert!(STOPWORDS.contains("la"));
        assert!(STOPWORDS.contains("les"));
        assert!(STOPWORDS.contains("un"));
    }

    #[test]
    fn contains_common_auxiliaries() {
        assert!(STOPWORDS.contains("est"));
        assert!(STOPWORDS.contains("sont"));
        assert!(STOPWORDS.contains("avait"));
    }

    #[test]
    fn contains_contracted_forms() {
        assert!(STOPWORDS.contains("du"));
        assert!(STOPWORDS.contains("au"));
        assert!(STOPWORDS.contains("aux"));
    }

    #[test]
    fn does_not_contain_content_words() {
        assert!(!STOPWORDS.contains("accessibilité"));
        assert!(!STOPWORDS.contains("linter"));
    }
}
