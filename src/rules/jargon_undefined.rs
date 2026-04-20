//! Rule: `jargon-undefined`.
//!
//! Flags domain-specific jargon used without definition. Like acronyms,
//! jargon creates reading interruptions for non-specialists; unlike
//! acronyms it is content words rather than uppercase sequences.
//!
//! v0.1 is a simplified, pattern-based flagger: maintain lists per
//! domain, let users activate the lists that match their audience, and
//! warn on every hit. A definition-aware two-pass version is tracked as
//! part of the same research backlog as F9 in `ROADMAP.md`.
//!
//! See [`RULES.md`](../../RULES.md#jargon-undefined) for references
//! (US Plain Language, FALC, WCAG 3.1.3).

use std::collections::HashSet;
use std::sync::LazyLock;

use crate::config::Profile;
use crate::parser::phrase_search::{find_word_bounded, line_column_at};
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Jargon categories shipped in v0.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JargonList {
    /// Software engineering jargon (EN-dominant).
    Tech,
    /// Legal jargon (FR-dominant).
    Legal,
    /// Medical jargon (FR-dominant).
    Medical,
    /// Administrative jargon (FR-dominant).
    Admin,
}

impl JargonList {
    fn entries(self) -> &'static [&'static str] {
        match self {
            Self::Tech => &TECH,
            Self::Legal => &LEGAL,
            Self::Medical => &MEDICAL,
            Self::Admin => &ADMIN,
        }
    }
}

static TECH: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "idempotent",
        "orthogonal",
        "deterministic",
        "polymorphic",
        "serialization",
        "deserialization",
        "synchronous",
        "asynchronous",
        "concurrency",
        "thread-safe",
        "side-effect",
        "referential transparency",
        "memoization",
        "currying",
        "hoisting",
        "closure",
        "monad",
        "immutable",
        "stateless",
        "refactoring",
    ]
});

static LEGAL: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "apériteur",
        "clause résolutoire",
        "force majeure",
        "cessation de paiement",
        "préjudice subi",
        "onéreux",
        "nonobstant",
        "préalablement",
        "susmentionné",
        "infra",
        "supra",
        "ad hoc",
        "de facto",
        "in fine",
        "subséquemment",
    ]
});

static MEDICAL: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "anamnèse",
        "étiologie",
        "pathognomonique",
        "iatrogène",
        "nosocomial",
        "pronostic vital engagé",
        "décompensation",
        "récidive",
        "rémission",
        "syndromique",
    ]
});

static ADMIN: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "attributaire",
        "solliciter",
        "diligenter",
        "instruction du dossier",
        "pièces justificatives",
        "circulaire",
        "délibération",
        "arrêté préfectoral",
        "transmission des pièces",
        "ayant droit",
    ]
});

/// Configuration for [`JargonUndefined`].
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Jargon categories active for the current profile.
    pub active_lists: Vec<JargonList>,

    /// Extra jargon terms (lowercase) layered on top of the active lists.
    pub custom: Vec<String>,

    /// Terms to silence from the active lists (exact lowercase match).
    pub whitelist: Vec<String>,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let active_lists = match profile {
            // Developers know their domain jargon — don't flag it.
            Profile::DevDoc => Vec::new(),
            Profile::Public | Profile::Falc => vec![
                JargonList::Tech,
                JargonList::Legal,
                JargonList::Medical,
                JargonList::Admin,
            ],
        };
        Self {
            active_lists,
            ..Self::default()
        }
    }
}

/// The [`JargonUndefined`] rule.
#[derive(Debug, Clone)]
pub struct JargonUndefined {
    config: Config,
}

impl JargonUndefined {
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
    pub const ID: &'static str = "jargon-undefined";

    fn phrases(&self) -> Vec<String> {
        let whitelist: HashSet<&str> = self.config.whitelist.iter().map(String::as_str).collect();
        self.config
            .active_lists
            .iter()
            .flat_map(|list| list.entries().iter().copied().map(str::to_string))
            .chain(self.config.custom.iter().cloned())
            .filter(|term| !whitelist.contains(term.as_str()))
            .collect()
    }
}

impl Rule for JargonUndefined {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let phrases = self.phrases();
        if phrases.is_empty() {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for (paragraph, section_title) in document.paragraphs_with_section() {
            let lowered = paragraph.text.to_lowercase();
            for phrase in &phrases {
                for byte_offset in find_word_bounded(&lowered, phrase) {
                    let (line_offset, column) = line_column_at(&paragraph.text, byte_offset);
                    let line = paragraph.start_line.saturating_add(line_offset);
                    diagnostics.push(build_diagnostic(
                        &document.source,
                        line,
                        column,
                        phrase,
                        section_title,
                    ));
                }
            }
        }
        diagnostics.sort_by_key(|d| (d.location.line, d.location.column));
        diagnostics
    }
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    term: &str,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(term.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Jargon term \"{term}\" may be unfamiliar to non-specialists. Define it on first use or \
         replace with a simpler phrase."
    );
    let diag = Diagnostic::new(JargonUndefined::ID, Severity::Warning, location, message);
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

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        JargonUndefined::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(JargonUndefined::ID, "jargon-undefined");
    }

    #[test]
    fn dev_doc_does_not_flag_anything() {
        let text = "The function is idempotent and thread-safe.";
        assert!(lint(text, Profile::DevDoc).is_empty());
    }

    #[test]
    fn public_flags_tech_jargon() {
        let diags = lint("The function is idempotent.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("idempotent"));
    }

    #[test]
    fn hyphenated_term_matches_as_phrase() {
        let diags = lint("Use a thread-safe queue here.", Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("thread-safe"));
    }

    #[test]
    fn multi_word_term_matches() {
        let diags = lint(
            "Invoke memoization or referential transparency.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn french_legal_term_matches() {
        let diags = lint(
            "La force majeure a entraîné une cessation de paiement.",
            Profile::Public,
        );
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn case_insensitive_match() {
        let diags = lint("Deterministic and Idempotent behaviour.", Profile::Public);
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn word_boundary_prevents_partial_match() {
        // "monads" should not match "monad" (trailing 's' is a word char).
        assert!(lint("He studies monads here.", Profile::Public).is_empty());
    }

    #[test]
    fn whitelist_silences_specific_term() {
        let cfg = Config {
            active_lists: vec![JargonList::Tech],
            whitelist: vec!["idempotent".to_string()],
            ..Config::default()
        };
        let doc = parse_plain(
            "The function is idempotent and deterministic.",
            SourceFile::Anonymous,
        );
        let diags = JargonUndefined::new(cfg).check(&doc, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("deterministic"));
    }

    #[test]
    fn custom_term_is_added() {
        let cfg = Config {
            active_lists: vec![JargonList::Tech],
            custom: vec!["yak shaving".to_string()],
            ..Config::default()
        };
        let doc = parse_plain("Too much yak shaving today.", SourceFile::Anonymous);
        let diags = JargonUndefined::new(cfg).check(&doc, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("yak shaving"));
    }

    #[test]
    fn category_is_lexicon() {
        let diags = lint("A polymorphic wrapper.", Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Lexicon);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "The wrapper is idempotent. It ensures thread-safe serialization of records.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
