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

/// Default French weasel words and phrases (lowercased).
///
/// See [`RULES.md`](../../RULES.md#weasel-words). Phrases are matched at
/// word boundaries, so `beaucoup` as a standalone token is not flagged —
/// only `beaucoup de` is.
pub static WEASELS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "quelques",
        "certains",
        "parfois",
        "plutôt",
        "assez",
        "globalement",
        "généralement",
        "souvent",
        "en général",
        "la plupart",
        "il semble que",
        "il semblerait que",
        "on pourrait dire que",
        "on dit souvent",
        "beaucoup de",
        "peu de",
        "presque",
        "quasiment",
        "environ",
        "à peu près",
    ]
});

/// French standalone negation markers (lowercased word forms).
///
/// French negation is bipartite (`ne ... pas`, `ne ... jamais`, …). The
/// `nested-negation` rule counts each `ne` / `n'` occurrence as one
/// negation rather than enumerating second-position particles, because
/// many of those particles (`plus`, `personne`, `point`, …) are ambiguous
/// outside the bipartite construction.
///
/// This list captures negators that stand on their own, without `ne`:
/// `sans` (without), `non` (no/not in elliptic answers and prefixes).
pub static STANDALONE_NEGATIONS: &[&str] = &["sans", "non"];

/// French pre-verbal negation clitic forms.
///
/// One occurrence of `ne` or `n'` marks one bipartite negation.
pub static NEGATION_CLITICS: &[&str] = &["ne", "n'"];

/// French conditional / temporal-conditional connectors (lowercased).
///
/// Used by the `conditional-stacking` rule. Bare `si` covers both the
/// canonical conditional and the elliptic `s'` form (see [`SI_CLITICS`]).
pub static CONDITIONALS: &[&str] = &[
    "si",
    "sauf si",
    "à moins que",
    "à moins de",
    "quand",
    "lorsque",
    "lorsqu'",
    "dès que",
    "tant que",
    "pourvu que",
    "à condition que",
    "à condition de",
    "au cas où",
    "même si",
    "en cas de",
];

/// French elliptic forms of `si` that attach to a vowel-initial word
/// (`s'il`, `s'ils`). Counted as one occurrence each.
pub static SI_CLITICS: &[&str] = &["s'il", "s'ils"];

/// French redundant intensifiers (lowercased).
///
/// Mirror of [`crate::language::en::INTENSIFIERS`]: words whose role
/// is to *upgrade* the confidence of a statement without adding
/// information. plainlanguage.gov, the CDC Clear Communication Index,
/// and the FALC guidelines all flag intensifiers as a plain-language
/// anti-pattern.
///
/// Disjoint from [`WEASELS`] (which captures hedges that *downgrade*
/// confidence: `plutôt`, `assez`, `presque`).
pub static INTENSIFIERS: &[&str] = &[
    "très",
    "vraiment",
    "extrêmement",
    "absolument",
    "totalement",
    "complètement",
    "terriblement",
    "incroyablement",
    "profondément",
    "super",
    "hyper",
];

/// French spelled-out cardinal numerals (lowercased).
///
/// Used by the `mixed-numeric-format` rule to detect a sentence that
/// mixes digits with spelled-out numerals (CDC Clear Communication Index,
/// plainlanguage.gov: present numbers consistently throughout).
///
/// `un` and `une` are deliberately excluded — they double as indefinite
/// articles (`un lecteur`, `une page`) and the false-positive rate is
/// prohibitive. The list therefore starts at `deux`. Regional variants
/// for 70/80/90 (`septante`, `huitante`, `octante`, `nonante`) are
/// included alongside the metropolitan compounds.
pub static SPELLED_NUMERALS: &[&str] = &[
    "deux",
    "trois",
    "quatre",
    "cinq",
    "six",
    "sept",
    "huit",
    "neuf",
    "dix",
    "onze",
    "douze",
    "treize",
    "quatorze",
    "quinze",
    "seize",
    "vingt",
    "vingts",
    "trente",
    "quarante",
    "cinquante",
    "soixante",
    "septante",
    "huitante",
    "octante",
    "nonante",
    "cent",
    "cents",
    "mille",
    "million",
    "millions",
    "milliard",
    "milliards",
];

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
