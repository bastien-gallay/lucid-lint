//! Rule: `lexicon.homophone-density`.
//!
//! Flags paragraphs whose share of homophone words exceeds a configurable
//! density. Homophones force a phonological-then-orthographic
//! disambiguation pass: dyslexic readers decode by sound first, so a
//! cluster of "to / too / two", "their / there / they're", or
//! "amande / amende" forces them to fall back on context word by word.
//! The British Dyslexia Association style guide and FALC orthographic
//! clarity guidelines both recommend rephrasing dense homophone runs.
//!
//! Cohort sibling of [F49](../../../ROADMAP.md#f-italic-span-long) under
//! the v0.3 condition-tag rules cohort
//! ([F46](../../../ROADMAP.md#f-homophone-density) /
//! [F49](../../../ROADMAP.md#f-italic-span-long) /
//! [F51](../../../ROADMAP.md#f51) / [F53](../../../ROADMAP.md#f53) /
//! [F57](../../../ROADMAP.md#f57)). Ships as
//! [`Status::Experimental`] in v0.2.x via the
//! [F139](../../../ROADMAP.md#f139) substrate; flips to `Stable` at
//! the v0.3 cut.

use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::Document;
use crate::rules::{Rule, Status};
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Minimum content-word count before a paragraph is evaluated.
///
/// Below this floor, a single homophone produces a misleading double-digit
/// percentage. The rule short-circuits short paragraphs rather than
/// pretending to score them.
const MIN_CONTENT_WORDS: u32 = 20;

/// Configuration for [`HomophoneDensity`].
///
/// Threshold is a percentage in the closed range `[0, 100]`. The rule
/// fires when the observed density is *strictly greater* than the
/// threshold (mirrors the `>` semantics used across the other density
/// rules in this category).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// Maximum allowed homophone share, expressed as a percentage of
    /// content words in the paragraph.
    pub max_density_percent: f64,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let max = match profile {
            Profile::DevDoc => 8.0,
            Profile::Public => 5.0,
            Profile::Falc => 3.0,
        };
        Self {
            max_density_percent: max,
        }
    }
}

/// The [`HomophoneDensity`] rule.
#[derive(Debug, Clone, Copy)]
pub struct HomophoneDensity {
    config: Config,
}

impl HomophoneDensity {
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
    pub const ID: &'static str = "lexicon.homophone-density";
}

impl Rule for HomophoneDensity {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let Some(homophones) = lookup_for(language) else {
            return Vec::new();
        };
        let max = self.config.max_density_percent;

        let mut diagnostics = Vec::new();
        for (paragraph, _section) in document.paragraphs_with_section() {
            let (total, hits, examples) = scan_paragraph(&paragraph.text, &homophones);
            if total < MIN_CONTENT_WORDS {
                continue;
            }
            let density = (f64::from(hits) / f64::from(total)) * 100.0;
            if density > max {
                diagnostics.push(build_diagnostic(
                    &document.source,
                    paragraph.start_line,
                    hits,
                    total,
                    density,
                    max,
                    &examples,
                ));
            }
        }
        diagnostics
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        // Dyslexia is the primary grounding (BDA); aphasic readers
        // share the phonological-decoding load that homophone clusters
        // amplify (FALC).
        &[ConditionTag::Dyslexia, ConditionTag::Aphasia]
    }

    fn status(&self) -> Status {
        // Cohort sibling of F49 (`structure.italic-span-long`).
        // Experimental in v0.2.x via F139; flips to Stable at v0.3 cut.
        Status::Experimental
    }
}

/// Build a flat lookup set from the per-language group table.
///
/// Returns `None` for [`Language::Unknown`] — the rule has no homophone
/// data to apply and skips the document rather than guessing.
fn lookup_for(language: Language) -> Option<HashSet<&'static str>> {
    let groups: &[&[&str]] = match language {
        Language::En => en::HOMOPHONE_GROUPS_EN,
        Language::Fr => fr::HOMOPHONE_GROUPS_FR,
        Language::Unknown => return None,
    };
    Some(groups.iter().copied().flatten().copied().collect())
}

/// Walk a paragraph and return `(total_content_words, hits, examples)`.
///
/// `examples` carries up to two distinct homophone surface forms
/// observed in the paragraph, in encounter order — used in the
/// diagnostic message to point the reader at concrete instances.
fn scan_paragraph(text: &str, homophones: &HashSet<&'static str>) -> (u32, u32, Vec<&'static str>) {
    let mut total: u32 = 0;
    let mut hits: u32 = 0;
    let mut examples: Vec<&'static str> = Vec::new();
    for raw in text.unicode_words() {
        let lower = raw.to_lowercase();
        if !lower.chars().any(char::is_alphabetic) {
            continue;
        }
        total = total.saturating_add(1);
        if let Some(canonical) = homophones.get(lower.as_str()) {
            hits = hits.saturating_add(1);
            if examples.len() < 2 && !examples.contains(canonical) {
                examples.push(*canonical);
            }
        }
    }
    (total, hits, examples)
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    hits: u32,
    total: u32,
    density: f64,
    max: f64,
    examples: &[&str],
) -> Diagnostic {
    let location = Location::new(source.clone(), line, 1, 1);
    let example_clause = if examples.is_empty() {
        String::new()
    } else {
        format!(" (e.g. {})", examples.join(", "))
    };
    let message = format!(
        "Paragraph density of homophones is {density:.1}% ({hits} of {total} content words{example_clause}); maximum {max:.1}%. Dense homophone runs raise the phonological-decoding load for dyslexic and aphasic readers; rephrase to disambiguate."
    );
    Diagnostic::new(HomophoneDensity::ID, Severity::Warning, location, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_plain;

    fn lint(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        HomophoneDensity::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case_and_category_prefixed() {
        assert_eq!(HomophoneDensity::ID, "lexicon.homophone-density");
        assert_eq!(
            HomophoneDensity::for_profile(Profile::Public).id(),
            "lexicon.homophone-density"
        );
    }

    #[test]
    fn ships_as_experimental() {
        // Cohort sibling of F49; flips to Stable at v0.3 cut.
        assert_eq!(
            HomophoneDensity::for_profile(Profile::Public).status(),
            Status::Experimental
        );
    }

    #[test]
    fn carries_dyslexia_and_aphasia_condition_tags() {
        // BDA grounds the dyslexia tag; FALC grounds the aphasia tag.
        let rule = HomophoneDensity::for_profile(Profile::Public);
        assert_eq!(
            rule.condition_tags(),
            &[ConditionTag::Dyslexia, ConditionTag::Aphasia]
        );
    }

    #[test]
    fn category_is_lexicon() {
        // 30 distinct words, 5 homophones from the EN list → density 16.7%.
        let text = "their there toys grow under affect bright clouds and lose vivid colors quickly while you wander gently between meadows and rivers across the calm valley below the open hills";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].category(), crate::types::Category::Lexicon);
    }

    #[test]
    fn paragraph_below_min_word_floor_is_skipped() {
        // 7 content words — well under MIN_CONTENT_WORDS — even though
        // every word is a homophone, the rule must short-circuit and
        // emit nothing rather than report a misleading 100%.
        let text = "their there to too two its lose";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn unknown_language_skips_silently() {
        // No lookup table for Unknown → rule returns no diagnostics
        // even on text full of would-be homophones.
        let text = "their there to too two its lose loose principal principle weather \
                    whether affect effect they're you're your there their too two its";
        let diags = lint(text, Profile::Public, Language::Unknown);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn clean_paragraph_does_not_trigger() {
        // 30 distinct content words with zero homophones → density 0%.
        let text = "Cognitive accessibility benefits every reader who scans documentation \
                    quickly between meetings tomorrow morning while juggling several open \
                    browser tabs across multiple monitors near a sunny window today.";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn dense_homophone_paragraph_triggers_on_public() {
        // 30 content words; hits = `their`, `there`, `to`, `too`, `two`,
        // `affect`, `lose` → 7 hits / 30 total ≈ 23.3% (above Public 5%).
        let text = "their report shows there were too many decisions to make and two \
                    teams could not affect the launch nor lose the schedule despite \
                    careful planning across each region and product line every quarter";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule_id, HomophoneDensity::ID);
        assert_eq!(diags[0].severity, Severity::Warning);
        assert!(diags[0].message.contains("homophones"));
        assert!(diags[0].message.contains("maximum 5.0%"));
    }

    #[test]
    fn density_at_threshold_does_not_trigger() {
        // `>` semantics: a paragraph at *exactly* the threshold passes.
        // 20 content words, 1 homophone (`their`) → density = 5.0% =
        // Public max. The floor (`>= MIN_CONTENT_WORDS`) is met because
        // the floor is inclusive.
        let text = "their bright clouds gently drift across calm meadows while travellers \
                    climb gentle hills near quiet villages every season under blue skies";
        let diags = lint(text, Profile::Public, Language::En);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn devdoc_profile_is_more_tolerant() {
        // Density ~13% triggers Public (5%) but not DevDoc (8%) when
        // density falls between the two. Build a 30-word paragraph with
        // 2 homophones → density ≈ 6.7% (above 5, below 8).
        let text = "their bright clouds gently drift across calm meadows while travellers \
                    climb gentle hills near quiet villages every season and watch the \
                    river run softly between forests there beyond";
        assert!(!lint(text, Profile::Public, Language::En).is_empty());
        assert!(lint(text, Profile::DevDoc, Language::En).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // Density ≈ 4% triggers FALC (3%) but not Public (5%).
        // 25 content words, 1 homophone → 4.0%.
        let text = "their bright clouds gently drift across calm meadows while travellers \
                    climb gentle hills near quiet villages every season and admire each \
                    rising star above";
        assert!(lint(text, Profile::Public, Language::En).is_empty());
        assert!(!lint(text, Profile::Falc, Language::En).is_empty());
    }

    #[test]
    fn french_paragraph_with_homophone_cluster_triggers() {
        // 30 mots, 4 homophones (`cours`, `foie`, `tâche`, `pause`) →
        // densité ≈ 13.3 %, au-dessus du seuil Public (5 %).
        let text = "Pendant le cours du matin la cuisinière prépare le foie de veau \
                    avant la pause de midi puis revient à sa tâche après avoir rangé \
                    les ustensiles sur la grande table en bois clair";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1, "got {diags:?}");
        assert!(diags[0].message.contains("homophones"));
    }

    #[test]
    fn french_clean_paragraph_does_not_trigger() {
        // 30 mots, aucun homophone listé → densité 0 %.
        let text = "Le développement durable devient une exigence partagée par toutes \
                    les équipes ingénieures responsables qui planifient soigneusement \
                    chaque déploiement progressif sur le réseau régional pendant cette \
                    année charnière";
        let diags = lint(text, Profile::Public, Language::Fr);
        assert!(diags.is_empty(), "got {diags:?}");
    }

    #[test]
    fn message_lists_concrete_example_words() {
        // The diagnostic should surface the first homophones it saw so
        // the reader can locate the cluster, not just see a percentage.
        let text = "their report shows there were too many decisions to make and two \
                    teams could not affect the launch nor lose the schedule despite \
                    careful planning across each region and product line every quarter";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        // First two distinct hits in encounter order are `their` and
        // `there`; both must surface in the example clause.
        assert!(
            diags[0].message.contains("their"),
            "message lacked example `their`: {}",
            diags[0].message
        );
        assert!(
            diags[0].message.contains("there"),
            "message lacked example `there`: {}",
            diags[0].message
        );
    }

    #[test]
    fn diagnostic_anchors_at_paragraph_start_line() {
        // Two paragraphs separated by a blank line; only the second
        // crosses the threshold. Anchor must report line 3.
        let text = "Cognitive accessibility benefits every reader who scans documentation \
                    quickly between meetings tomorrow morning while juggling several open \
                    browser tabs across multiple monitors near a sunny window today.\n\n\
                    their report shows there were too many decisions to make and two \
                    teams could not affect the launch nor lose the schedule despite \
                    careful planning across each region and product line every quarter";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].location.line, 3);
    }

    #[test]
    fn each_paragraph_evaluated_independently() {
        // Two dense paragraphs → two diagnostics, one per paragraph.
        let text = "their report shows there were too many decisions to make and two \
                    teams could not affect the launch nor lose the schedule despite \
                    careful planning across each region and product line every quarter\n\n\
                    Their plan to redirect there ensures too many losses cannot affect \
                    its schedule, and the principal author hopes to lose neither weather \
                    forecasts nor whether deadlines slip past two cycles ahead steadily";
        let diags = lint(text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert!(
            (Config::for_profile(Profile::DevDoc).max_density_percent - 8.0).abs() < f64::EPSILON
        );
        assert!(
            (Config::for_profile(Profile::Public).max_density_percent - 5.0).abs() < f64::EPSILON
        );
        assert!(
            (Config::for_profile(Profile::Falc).max_density_percent - 3.0).abs() < f64::EPSILON
        );
    }

    #[test]
    fn snapshot_fixture() {
        let text = "their report shows there were too many decisions to make and two \
                    teams could not affect the launch nor lose the schedule despite \
                    careful planning across each region and product line every quarter";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
