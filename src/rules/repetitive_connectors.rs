//! Rule: `repetitive-connectors`.
//!
//! Flags overuse of the same logical connector within a sliding window of
//! sentences. Connectors (`however`, `then`, `because`, …) are
//! attentional anchors when varied and noise when repeated.
//!
//! See [`RULES.md`](../../RULES.md#repetitive-connectors) for references
//! (Sanders & Noordman 2000; Graesser et al. 2004).

use std::collections::HashSet;
use std::num::NonZeroU32;
use std::sync::LazyLock;

use crate::config::Profile;
use crate::parser::{split_sentences, Document, Sentence};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

static EN_CONNECTORS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        // Opposition
        "however",
        "nevertheless",
        "yet",
        "although",
        "but",
        // Cause
        "because",
        "since",
        "as",
        "for",
        // Consequence
        "therefore",
        "thus",
        "consequently",
        "hence",
        "so",
        // Sequence
        "first",
        "then",
        "next",
        "finally",
        // Illustration
        "for example",
        "notably",
        "in particular",
        "such as",
        // Addition
        "moreover",
        "furthermore",
        "also",
        "additionally",
    ]
});

static FR_CONNECTORS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        // Opposition
        "cependant",
        "toutefois",
        "en revanche",
        "néanmoins",
        "pourtant",
        "mais",
        // Cause
        "parce que",
        "car",
        "puisque",
        "en effet",
        // Consequence
        "donc",
        "ainsi",
        "par conséquent",
        "c'est pourquoi",
        // Sequence
        "d'abord",
        "ensuite",
        "puis",
        "enfin",
        "premièrement",
        // Illustration
        "par exemple",
        "notamment",
        "en particulier",
        // Addition
        "de plus",
        "en outre",
        "également",
        "par ailleurs",
    ]
});

/// Configuration for [`RepetitiveConnectors`].
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum occurrences of a single connector allowed within
    /// `window_size` consecutive sentences. Strictly exceeding this
    /// triggers.
    pub max_per_window: NonZeroU32,

    /// Sliding window size, in sentences.
    pub window_size: NonZeroU32,

    /// User-provided additional connectors (lowercase).
    pub custom_connectors: Vec<String>,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 4,
            Profile::Public => 3,
            Profile::Falc => 2,
        };
        Self {
            max_per_window: NonZeroU32::new(max).expect("non-zero literal"),
            window_size: NonZeroU32::new(5).expect("non-zero literal"),
            custom_connectors: Vec::new(),
        }
    }
}

/// The [`RepetitiveConnectors`] rule.
#[derive(Debug, Clone)]
pub struct RepetitiveConnectors {
    config: Config,
}

impl RepetitiveConnectors {
    /// Build the rule from explicit config.
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Build the rule using the preset for the given profile.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        Self::new(Config::for_profile(profile))
    }

    /// The rule identifier.
    pub const ID: &'static str = "repetitive-connectors";

    fn connectors_for(&self, language: Language) -> Vec<String> {
        let base: Vec<&'static str> = match language {
            Language::En => EN_CONNECTORS.iter().copied().collect(),
            Language::Fr => FR_CONNECTORS.iter().copied().collect(),
            Language::Unknown => return Vec::new(),
        };
        let mut seen: HashSet<String> = HashSet::new();
        base.into_iter()
            .map(str::to_string)
            .chain(self.config.custom_connectors.iter().cloned())
            .filter(|c| seen.insert(c.clone()))
            .collect()
    }
}

impl Rule for RepetitiveConnectors {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let connectors = self.connectors_for(language);
        if connectors.is_empty() {
            return Vec::new();
        }

        // Collect every sentence in document order, paired with its section.
        let mut sentences: Vec<(Sentence, Option<&str>)> = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            for s in split_sentences(&paragraph.text, paragraph.start_line, 1) {
                sentences.push((s, section_title));
            }
        }
        if sentences.is_empty() {
            return Vec::new();
        }

        let threshold = self.config.max_per_window.get() as usize;
        let window = self.config.window_size.get() as usize;

        let mut diagnostics = Vec::new();
        for connector in &connectors {
            // Indices (within `sentences`) of every hit for this connector.
            let mut hits: Vec<usize> = Vec::new();
            for (idx, (sentence, _)) in sentences.iter().enumerate() {
                let lowered = sentence.text.to_lowercase();
                let occurrences = count_matches(&lowered, connector);
                for _ in 0..occurrences {
                    hits.push(idx);
                }
            }
            if hits.len() <= threshold {
                continue;
            }
            // Walk hits and emit one diagnostic per cluster.
            let mut k = 0;
            while k + threshold < hits.len() {
                let cluster_start = hits[k];
                let cluster_end = hits[k + threshold];
                if cluster_end < cluster_start + window {
                    let (sentence, section) = &sentences[cluster_start];
                    let count = threshold + 1;
                    diagnostics.push(build_diagnostic(
                        &document.source,
                        sentence.line,
                        sentence.column,
                        &sentence.text,
                        connector,
                        u32::try_from(count).unwrap_or(u32::MAX),
                        self.config.max_per_window.get(),
                        window,
                        *section,
                    ));
                    // Skip past this cluster so we don't re-report overlapping
                    // windows. Advance to the hit after the last one in the
                    // cluster.
                    k += threshold + 1;
                } else {
                    k += 1;
                }
            }
        }

        diagnostics.sort_by_key(|d| (d.location.line, d.location.column));
        diagnostics
    }
}

/// Count word-bounded case-insensitive occurrences of `needle` in `haystack`.
/// `haystack` must already be lowercased.
fn count_matches(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut start = 0;
    while let Some(found) = haystack[start..].find(needle) {
        let abs = start + found;
        if is_word_boundary(haystack, abs, abs + needle.len()) {
            count += 1;
        }
        start = abs + needle.len();
        if start > haystack.len() {
            break;
        }
    }
    count
}

fn is_word_boundary(s: &str, start: usize, end: usize) -> bool {
    let before_ok = start == 0 || !s[..start].chars().next_back().is_some_and(is_word_char);
    let after_ok = end >= s.len() || !s[end..].chars().next().is_some_and(is_word_char);
    before_ok && after_ok
}

fn is_word_char(c: char) -> bool {
    c.is_alphabetic() || c == '\''
}

#[allow(clippy::too_many_arguments)]
fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    sentence_text: &str,
    connector: &str,
    count: u32,
    max: u32,
    window: usize,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(sentence_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Connector \"{connector}\" appears {count} times within {window} consecutive sentences \
         (max {max}). Vary the connector or restructure the passage."
    );
    let diag = Diagnostic::new(
        RepetitiveConnectors::ID,
        Severity::Warning,
        location,
        message,
    );
    match section {
        Some(title) => diag.with_section(title),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_plain;
    use crate::types::SourceFile;

    fn lint(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        RepetitiveConnectors::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(RepetitiveConnectors::ID, "repetitive-connectors");
    }

    #[test]
    fn varied_connectors_do_not_trigger() {
        let text = "First we act. Then we think. However, we also pause. Therefore we improve.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn repeated_then_triggers() {
        // Public: max 3 → 4 "then" within 5 sentences triggers.
        let text = "We analyzed the data. Then we built the model. Then we validated it. \
                    Then we shipped it. Then we archived it.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"then\""));
        assert!(diags[0].message.contains("4 times"));
    }

    #[test]
    fn at_threshold_does_not_trigger() {
        // Public: max 3 → exactly 3 allowed.
        let text = "Then a. Then b. Then c.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn spread_out_repetition_does_not_trigger() {
        // 4 "then" but spread across 8 sentences — window of 5 never contains more than 3.
        let text = "Then a. b. Then b. c. Then c. d. Then d. e.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // 3 "then": passes Public (3), fails FALC (2).
        let text = "Then a. Then b. Then c.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
        assert!(!lint(text, Profile::Falc, Language::En).is_empty());
    }

    #[test]
    fn multi_word_connector_matches() {
        // "in particular" × 4 in 5 sentences. Picked over "for example" because
        // "for" on its own is also a cause connector and would trigger twice.
        let text = "In particular the first. In particular the second. In particular the \
                    third. In particular the fourth. Mid.";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(
            diags.iter().any(|d| d.message.contains("in particular")),
            "expected an in-particular diagnostic: {diags:?}"
        );
    }

    #[test]
    fn french_connector_matches() {
        // "puis" × 4 in 5 sentences.
        let text = "Puis nous avons lu. Puis nous avons écrit. Puis nous avons révisé. \
                    Puis nous avons publié. Fin.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("puis"));
    }

    #[test]
    fn case_insensitive_match() {
        let text = "Then a. THEN b. then c. Then d. Mid.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn word_boundary_prevents_partial_match() {
        // "therein" must not match "then".
        let text = "Therein a. Therein b. Therein c. Therein d. Therein e.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn unknown_language_skips_rule() {
        let text = "Then a. Then b. Then c. Then d.";
        assert!(lint(text, Profile::Public, Language::Unknown).is_empty());
    }

    #[test]
    fn one_diagnostic_per_cluster() {
        // Six "then" in 6 sentences — one cluster, one diagnostic.
        let text = "Then a. Then b. Then c. Then d. Then e. Then f.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_per_window.get(), 4);
        assert_eq!(Config::for_profile(Profile::Public).max_per_window.get(), 3);
        assert_eq!(Config::for_profile(Profile::Falc).max_per_window.get(), 2);
        assert_eq!(Config::for_profile(Profile::Public).window_size.get(), 5);
    }

    #[test]
    fn category_is_style() {
        let text = "Then a. Then b. Then c. Then d. Mid.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Style);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "We analyzed the data. Then we built the model. Then we validated it. \
                    Then we shipped it. Then we archived it.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
