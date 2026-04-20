//! Rule: `readability-score`.
//!
//! Computes a per-document Flesch-Kincaid grade level and reports it as
//! an observability signal. Emitted as `info` under threshold and
//! `warning` above — this is "cyclomatic complexity for prose": a
//! metric first, a warning second.
//!
//! The formula is calibrated for English. Applied to French it
//! over-estimates by roughly one to two grades; language-specific
//! calibration (Kandel-Moles, Scolarius) is tracked as F10.
//!
//! See [`RULES.md`](../../RULES.md#readability-score) for the rule's
//! rationale and thresholds.

use unicode_segmentation::UnicodeSegmentation;

use crate::config::Profile;
use crate::parser::{split_sentences, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity};

/// Configuration for [`ReadabilityScore`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// Flesch-Kincaid grade level at or below which the document is
    /// considered acceptable. Scores strictly above this become a
    /// `warning`.
    pub max_grade_level: f64,

    /// When true, always emit an `info` diagnostic with the score, even
    /// when the document is within the target range.
    pub always_report: bool,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub const fn for_profile(profile: Profile) -> Self {
        let max_grade_level = match profile {
            Profile::DevDoc => 14.0,
            Profile::Public => 9.0,
            Profile::Falc => 6.0,
        };
        Self {
            max_grade_level,
            always_report: true,
        }
    }
}

/// The [`ReadabilityScore`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ReadabilityScore {
    config: Config,
}

impl ReadabilityScore {
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
    pub const ID: &'static str = "readability-score";
}

impl Rule for ReadabilityScore {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let mut words: u64 = 0;
        let mut syllables: u64 = 0;
        let mut sentences: u64 = 0;

        for (paragraph, _section) in document.paragraphs_with_section() {
            for token in paragraph.text.unicode_words() {
                words += 1;
                syllables += u64::from(count_syllables(token));
            }
            sentences +=
                u64::try_from(split_sentences(&paragraph.text, paragraph.start_line, 1).len())
                    .unwrap_or(u64::MAX);
        }

        if words == 0 || sentences == 0 {
            return Vec::new();
        }

        let words_f = words as f64;
        let sentences_f = sentences as f64;
        let syllables_f = syllables as f64;
        let grade = 0.39f64.mul_add(words_f / sentences_f, 11.8 * (syllables_f / words_f)) - 15.59;

        let above_threshold = grade > self.config.max_grade_level;
        if !above_threshold && !self.config.always_report {
            return Vec::new();
        }

        let severity = if above_threshold {
            Severity::Warning
        } else {
            Severity::Info
        };
        let message = if above_threshold {
            format!(
                "Flesch-Kincaid grade {:.1} exceeds target {:.1}. Shorten sentences or choose \
                 simpler words.",
                grade, self.config.max_grade_level,
            )
        } else {
            format!(
                "Flesch-Kincaid grade {:.1} (target ≤ {:.1}).",
                grade, self.config.max_grade_level,
            )
        };
        let location = Location::new(document.source.clone(), 1, 1, 1);
        vec![Diagnostic::new(Self::ID, severity, location, message)]
    }
}

/// Rough syllable count for a single unicode word. Counts runs of vowel
/// characters (ASCII + common Latin-1 accented vowels) and subtracts a
/// trailing silent `e` when it would not take the count below one.
fn count_syllables(word: &str) -> u32 {
    let lower = word.to_lowercase();
    let mut count: u32 = 0;
    let mut in_vowel_run = false;
    for ch in lower.chars() {
        if is_vowel(ch) {
            if !in_vowel_run {
                count += 1;
                in_vowel_run = true;
            }
        } else {
            in_vowel_run = false;
        }
    }
    // Silent trailing `e` (English and French both).
    if count > 1 && lower.ends_with('e') {
        count -= 1;
    }
    count.max(1)
}

const fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'e'
            | 'i'
            | 'o'
            | 'u'
            | 'y'
            | 'à'
            | 'â'
            | 'ä'
            | 'é'
            | 'è'
            | 'ê'
            | 'ë'
            | 'î'
            | 'ï'
            | 'ô'
            | 'ö'
            | 'ù'
            | 'û'
            | 'ü'
            | 'ÿ'
            | 'œ'
            | 'æ'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_plain;
    use crate::types::SourceFile;

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        ReadabilityScore::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(ReadabilityScore::ID, "readability-score");
    }

    #[test]
    fn empty_document_produces_no_diagnostic() {
        assert!(lint("", Profile::Public).is_empty());
    }

    #[test]
    fn simple_prose_reports_info_under_threshold() {
        let text = "The cat sat on the mat. The dog ran.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Info);
        assert!(diags[0].message.starts_with("Flesch-Kincaid grade"));
    }

    #[test]
    fn dense_prose_reports_warning_above_threshold() {
        // Long polysyllabic sentence on purpose.
        let text = "The implementation of this particular configuration unfortunately \
                    requires comprehensive understanding of several interdependent \
                    architectural subcomponents, constraining downstream collaborators.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
        assert!(diags[0].message.contains("exceeds target"));
    }

    #[test]
    fn always_report_false_suppresses_info() {
        let cfg = Config {
            max_grade_level: 9.0,
            always_report: false,
        };
        let doc = parse_plain("The cat sat on the mat.", SourceFile::Anonymous);
        let diags = ReadabilityScore::new(cfg).check(&doc, Language::En);
        assert!(diags.is_empty());
    }

    #[test]
    fn always_report_false_still_warns_above_threshold() {
        let cfg = Config {
            max_grade_level: -10.0,
            always_report: false,
        };
        let doc = parse_plain("The cat sat on the mat.", SourceFile::Anonymous);
        let diags = ReadabilityScore::new(cfg).check(&doc, Language::En);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
    }

    #[test]
    fn syllable_counter_handles_simple_cases() {
        assert_eq!(count_syllables("cat"), 1);
        assert_eq!(count_syllables("sitting"), 2);
        assert_eq!(count_syllables("cake"), 1); // silent e
        assert_eq!(count_syllables("implementation"), 5);
        // FALC edge-case word with trailing "e" but single vowel run.
        assert_eq!(count_syllables("the"), 1);
    }

    #[test]
    fn syllable_counter_handles_french_accents() {
        assert_eq!(count_syllables("été"), 2);
        // The silent-e heuristic strips the final `e` of `œuvre`. That's
        // the intended approximation — not all French readers would count
        // the schwa.
        assert_eq!(count_syllables("œuvre"), 1);
        assert_eq!(count_syllables("maison"), 2);
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        assert!((Config::for_profile(Profile::DevDoc).max_grade_level - 14.0).abs() < 1e-9);
        assert!((Config::for_profile(Profile::Public).max_grade_level - 9.0).abs() < 1e-9);
        assert!((Config::for_profile(Profile::Falc).max_grade_level - 6.0).abs() < 1e-9);
    }

    #[test]
    fn category_is_global() {
        let text = "Short. Prose.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Global);
    }

    #[test]
    fn one_diagnostic_per_document() {
        let text = "First short one. Second short one.\n\nThird short one. Fourth short one.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "The cat sat on the mat. The dog ran quickly. The bird flew away.";
        let diags = lint(text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
