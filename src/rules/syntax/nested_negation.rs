//! Rule: `nested-negation`.
//!
//! Flags sentences that stack multiple negations. Two or more negations in
//! the same sentence force the reader to mentally toggle truth values, a
//! known burden for readers with aphasia, ADHD, and (more broadly) anyone
//! reading under cognitive load. Plain-language guidelines (FALC, CDC
//! Clear Communication Index, plainlanguage.gov) recommend rewriting
//! double negatives as positives.
//!
//! Counting strategy:
//!
//! - **English** — sum of word-boundary matches against the language's
//!   `NEGATIONS` list plus occurrences of the contracted `n't` suffix
//!   (`don't`, `won't`, `isn't`, `doesn't`, …).
//! - **French** — pair-based bipartite counting. Each `ne` / `n'` clitic
//!   contributes one negation and is paired with its nearest
//!   second-position particle (`pas`, `rien`, `jamais`, `plus`,
//!   `personne`, `aucun`, `aucune`, `guère`, `nulle part`) within a short
//!   window; the pairing just consumes the particle to avoid
//!   double-counting. Any second-position particle left unpaired in a
//!   sentence that contains a `ne` clitic contributes an additional
//!   negation — this catches forms like `rien` used as a nominal negative
//!   subject (`… que rien n'est jamais possible`). Guards: `pas` and
//!   `plus` never count when unpaired (too ambiguous outside the `ne …`
//!   construction); `rien` preceded by `de` is treated as the idiom
//!   `de rien` and skipped. Standalone negators (`sans`, `non`) always
//!   count.
//!
//! See [`RULES.md`](../../RULES.md#nested-negation) for the threshold
//! reference.

use std::num::NonZeroU32;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::en;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`NestedNegation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Maximum allowed negations per sentence.
    pub max_negations: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 3,
            Profile::Public => 2,
            Profile::Falc => 1,
        };
        Self {
            max_negations: NonZeroU32::new(max).expect("non-zero literal"),
        }
    }
}

/// The [`NestedNegation`] rule.
#[derive(Debug, Clone, Copy)]
pub struct NestedNegation {
    config: Config,
}

impl NestedNegation {
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
    pub const ID: &'static str = "syntax.nested-negation";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[
        ConditionTag::Aphasia,
        ConditionTag::Adhd,
        ConditionTag::General,
    ];
}

impl Rule for NestedNegation {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let max = self.config.max_negations.get();
        let counter = match language {
            Language::Fr => count_french,
            Language::En | Language::Unknown => count_english,
        };

        document
            .paragraphs_with_section()
            .flat_map(|(paragraph, section_title)| {
                paragraph.sentences.iter().filter_map(move |sentence| {
                    let count = counter(&sentence.text);
                    if count > max {
                        Some(build_diagnostic(
                            &document.source,
                            &sentence.text,
                            sentence.line,
                            sentence.column,
                            count,
                            max,
                            section_title,
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

fn count_english(sentence: &str) -> u32 {
    let lowered = sentence.to_lowercase();
    let mut count: u32 = 0;
    for token in lowered.split(|c: char| !c.is_alphanumeric() && c != '\'') {
        if token.is_empty() {
            continue;
        }
        if en::NEGATIONS.contains(&token) {
            count = count.saturating_add(1);
            continue;
        }
        // Contracted forms: don't, can't, won't, isn't, doesn't, …
        // Bare "n't" doesn't occur as a token in well-formed text, so
        // require at least one preceding letter.
        if token.len() > 2 && token.ends_with("n't") {
            count = count.saturating_add(1);
        }
    }
    count
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrKind {
    NeClitic,
    Particle,
    Sans,
    Non,
    Other,
}

fn is_ne_clitic(token: &str) -> bool {
    token == "ne" || token.starts_with("n'") || token.starts_with("n\u{2019}")
}

fn bare(token: &str) -> &str {
    token.trim_matches(|c: char| !c.is_alphanumeric())
}

/// Pairing window for FR ne-clitic ↔ second-position particle.
const FR_PAIRING_WINDOW: usize = 6;

fn count_french(sentence: &str) -> u32 {
    let lowered = sentence.to_lowercase();
    let raw: Vec<&str> = lowered
        .split(|c: char| {
            c.is_whitespace() || matches!(c, ',' | ';' | ':' | '.' | '!' | '?' | '(' | ')' | '"')
        })
        .filter(|t| !t.is_empty())
        .collect();

    let mut kinds: Vec<FrKind> = Vec::with_capacity(raw.len());
    let mut skip_next = false;
    for (i, tok) in raw.iter().enumerate() {
        if skip_next {
            skip_next = false;
            kinds.push(FrKind::Other);
            continue;
        }
        if is_ne_clitic(tok) {
            kinds.push(FrKind::NeClitic);
            continue;
        }
        let b = bare(tok);
        if b == "sans" {
            kinds.push(FrKind::Sans);
            continue;
        }
        if b == "non" {
            kinds.push(FrKind::Non);
            continue;
        }
        // Multi-word "nulle part": treat as one particle at position i.
        if b == "nulle" && raw.get(i + 1).is_some_and(|n| bare(n) == "part") {
            kinds.push(FrKind::Particle);
            skip_next = true;
            continue;
        }
        let is_particle = matches!(
            b,
            "pas" | "rien" | "jamais" | "plus" | "personne" | "aucun" | "aucune" | "guère"
        );
        if is_particle {
            // `de rien` idiom guard.
            if b == "rien" && i > 0 && bare(raw[i - 1]) == "de" {
                kinds.push(FrKind::Other);
            } else {
                kinds.push(FrKind::Particle);
            }
            continue;
        }
        kinds.push(FrKind::Other);
    }

    // Pair each ne-clitic with the nearest unclaimed particle within a
    // short window (forward preferred, then backward). Pairing consumes
    // the particle so we don't double-count the `ne … X` construction;
    // each ne-clitic still contributes 1 negation whether paired or not.
    let n = kinds.len();
    let mut claimed = vec![false; n];
    let mut ne_count: u32 = 0;
    let mut has_ne = false;
    for i in 0..n {
        if kinds[i] != FrKind::NeClitic {
            continue;
        }
        has_ne = true;
        ne_count = ne_count.saturating_add(1);
        let mut paired: Option<usize> = None;
        let fwd_end = (i + 1 + FR_PAIRING_WINDOW).min(n);
        for j in (i + 1)..fwd_end {
            if !claimed[j] && kinds[j] == FrKind::Particle {
                paired = Some(j);
                break;
            }
        }
        if paired.is_none() {
            let lo = i.saturating_sub(FR_PAIRING_WINDOW);
            for j in (lo..i).rev() {
                if !claimed[j] && kinds[j] == FrKind::Particle {
                    paired = Some(j);
                    break;
                }
            }
        }
        if let Some(j) = paired {
            claimed[j] = true;
        }
    }

    // Unpaired second-position particles in a ne-sentence contribute an
    // extra negation, except `pas` and `plus` which are too ambiguous
    // outside a direct `ne … X` pair.
    let mut unpaired: u32 = 0;
    if has_ne {
        for (i, k) in kinds.iter().enumerate() {
            if *k == FrKind::Particle && !claimed[i] {
                let b = bare(raw[i]);
                if !matches!(b, "pas" | "plus") {
                    unpaired = unpaired.saturating_add(1);
                }
            }
        }
    }

    // Standalone negators always count.
    let standalones: u32 = kinds
        .iter()
        .filter(|k| matches!(**k, FrKind::Sans | FrKind::Non))
        .count()
        .try_into()
        .unwrap_or(u32::MAX);

    ne_count
        .saturating_add(unpaired)
        .saturating_add(standalones)
}

fn build_diagnostic(
    source: &SourceFile,
    sentence_text: &str,
    line: u32,
    column: u32,
    actual: u32,
    max: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(sentence_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "Sentence stacks {actual} negations (maximum {max}). Rewrite as a positive statement \
         or split the negations across separate sentences."
    );
    let diag = Diagnostic::new(NestedNegation::ID, Severity::Warning, location, message);
    match section {
        Some(title) => diag.with_section(title),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_plain;
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        NestedNegation::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(NestedNegation::ID, "syntax.nested-negation");
    }

    #[test]
    fn tags_include_general_so_rule_runs_by_default() {
        assert!(NestedNegation::TAGS.contains(&ConditionTag::General));
        assert!(NestedNegation::TAGS.contains(&ConditionTag::Aphasia));
        assert!(NestedNegation::TAGS.contains(&ConditionTag::Adhd));
    }

    #[test]
    fn category_is_syntax() {
        // Two negations under Public threshold (2) — bump to three.
        let text = "We do not say nothing is never possible.";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].category(), Category::Syntax);
    }

    #[test]
    fn english_single_negation_does_not_trigger() {
        assert!(lint("This is not allowed.", Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn english_double_negation_does_not_trigger_under_public() {
        // Public threshold is 2; two negations = at threshold = no diagnostic.
        let text = "We do not allow nothing here.";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn english_triple_negation_triggers_under_public() {
        let text = "We do not say nothing is never possible.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 negations"));
    }

    #[test]
    fn english_contracted_nt_counts_as_negation() {
        // "don't" (1) + "isn't" (1) + "never" (1) = 3 → triggers Public.
        let text = "I don't think this isn't never going to happen.";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn falc_profile_flags_a_single_double_negation() {
        // FALC max is 1; two negations triggers.
        let text = "We do not allow nothing.";
        let diags = lint(text, Profile::Falc, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn dev_doc_profile_tolerates_more_stacking() {
        // 3 negations: triggers Public (max 2), passes DevDoc (max 3).
        let text = "We do not say nothing is never wrong.";
        assert!(!lint(text, Profile::Public, Language::En).is_empty());
        assert!(lint(text, Profile::DevDoc, Language::En).is_empty());
    }

    #[test]
    fn french_bipartite_negation_counts_as_one() {
        // `ne ... pas` is ONE negation, not two.
        let text = "Nous ne sommes pas prêts.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_clitic_apostrophe_counts() {
        // `n'est ... jamais` — `n'` is the clitic, counts as one.
        let text = "Il n'est jamais venu.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_three_clitics_trigger_under_public() {
        // Three independent ne ... X clauses; "ne" appears 3 times.
        let text = "Il ne dit rien, elle ne fait rien et nous ne savons pas.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 negations"));
    }

    #[test]
    fn french_sans_counts_as_standalone_negation() {
        // `sans X sans Y` = 2 standalone negations → at Public threshold (2),
        // does not trigger; FALC (max 1) does.
        let text = "Sans plan, sans budget.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
        assert_eq!(lint(text, Profile::Falc, Language::Fr).len(), 1);
    }

    #[test]
    fn french_unpaired_rien_in_ne_sentence_counts() {
        // F87 pedagogical target: ne…pas (1) + rien (1) + n'…jamais (1) = 3.
        let text = "Nous ne disons pas que rien n'est jamais possible.";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 negations"));
    }

    #[test]
    fn french_rien_ne_pairs_as_one() {
        // `Rien ne marche` is ONE bipartite negation, not two.
        assert!(lint("Rien ne marche.", Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_ne_plus_pairs_as_one() {
        // `ne … plus` (no longer) pairs, so `plus` is consumed.
        assert!(lint("Je ne veux plus parler.", Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_personne_ne_pairs_as_one() {
        assert!(lint("Personne ne m'a parlé.", Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_de_rien_idiom_is_skipped() {
        // `de rien` must not count; sentence has 1 bipartite `n'…rien`.
        let text = "De rien — je n'ai rien entendu.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn french_nulle_part_counts_as_one_particle() {
        // `ne … nulle part` pairs → one negation.
        assert!(lint(
            "Il ne trouve nulle part de solution.",
            Profile::Public,
            Language::Fr
        )
        .is_empty());
    }

    #[test]
    fn french_plus_alone_is_not_counted() {
        // `plus` outside `ne ... plus` means "more" — must not be flagged.
        let text = "Il faut plus de plus de plus de courage.";
        assert!(lint(text, Profile::Public, Language::Fr).is_empty());
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).max_negations.get(), 3);
        assert_eq!(Config::for_profile(Profile::Public).max_negations.get(), 2);
        assert_eq!(Config::for_profile(Profile::Falc).max_negations.get(), 1);
    }

    #[test]
    fn french_corpus_fixture_triggers_only_on_expected_lines() {
        // `tests/corpus/fr/nested-negation.md` mixes the F87 pedagogical
        // target, the single-negation guards, and a triple-standalone
        // case. Under `public`, exactly three lines should trip the rule:
        // the two F87-style triples and `Sans plan, sans budget, sans
        // équipe.` (three standalones).
        let text = include_str!("../../../tests/corpus/fr/nested-negation.md");
        let diags = lint(text, Profile::Public, Language::Fr);
        let lines: Vec<u32> = diags.iter().map(|d| d.location.line).collect();
        assert_eq!(lines, vec![9, 39, 43], "got diags: {diags:#?}");
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Short and clean. We do not say nothing is never possible. Fine again.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
