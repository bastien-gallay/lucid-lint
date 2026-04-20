//! Rule: `readability-score`.
//!
//! Computes a per-document readability score and reports it as an
//! observability signal. Emitted as `info` under threshold and `warning`
//! above — this is "cyclomatic complexity for prose": a metric first, a
//! warning second.
//!
//! The formula is selected per detected document language (F10):
//!
//! - **English** — Flesch-Kincaid grade level.
//! - **French** — Kandel-Moles ease score, converted to a grade
//!   equivalent so the per-profile `max_grade_level` thresholds remain
//!   meaningful across languages.
//! - **Unknown** — falls back to Flesch-Kincaid.
//!
//! User-configurable formula choice (F11), `--readability-verbose`
//! multi-formula reports, and the should-ship alternatives (Gunning Fog,
//! SMOG, Dale-Chall, Scolarius) are tracked separately on the roadmap.
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

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
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

        #[allow(clippy::cast_precision_loss)]
        let words_f = words as f64;
        #[allow(clippy::cast_precision_loss)]
        let sentences_f = sentences as f64;
        #[allow(clippy::cast_precision_loss)]
        let syllables_f = syllables as f64;

        let formula = Formula::for_language(language);
        let report = formula.compute(words_f, sentences_f, syllables_f);

        let above_threshold = report.grade_equivalent > self.config.max_grade_level;
        if !above_threshold && !self.config.always_report {
            return Vec::new();
        }

        let severity = if above_threshold {
            Severity::Warning
        } else {
            Severity::Info
        };
        let message = report.format_message(self.config.max_grade_level, above_threshold);
        let location = Location::new(document.source.clone(), 1, 1, 1);
        vec![Diagnostic::new(Self::ID, severity, location, message)]
    }
}

/// Per-language readability formula (F10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Formula {
    /// Flesch-Kincaid grade level — English.
    FleschKincaid,
    /// Kandel-Moles ease score — French.
    KandelMoles,
}

impl Formula {
    fn for_language(language: Language) -> Self {
        match language {
            Language::Fr => Self::KandelMoles,
            Language::En | Language::Unknown => Self::FleschKincaid,
        }
    }

    fn compute(self, words: f64, sentences: f64, syllables: f64) -> ScoreReport {
        match self {
            Self::FleschKincaid => {
                // Standard Flesch-Kincaid grade level.
                let grade = 0.39f64.mul_add(words / sentences, 11.8 * (syllables / words)) - 15.59;
                ScoreReport {
                    formula_name: "Flesch-Kincaid",
                    native_value: grade,
                    native_unit: "grade",
                    grade_equivalent: grade,
                }
            },
            Self::KandelMoles => {
                // Kandel & Moles (1958) — Flesch reading-ease adapted for
                // French. Range ≈ 0..100, higher = easier. Convert to a
                // grade equivalent with the same linear approximation
                // commonly used for the English Reading Ease score so the
                // per-profile `max_grade_level` threshold stays meaningful
                // across languages: grade ≈ (100 - score) / 10.
                let ease = 207.0 - 1.015f64.mul_add(words / sentences, 73.6 * (syllables / words));
                let grade_equivalent = (100.0 - ease) / 10.0;
                ScoreReport {
                    formula_name: "Kandel-Moles",
                    native_value: ease,
                    native_unit: "ease score",
                    grade_equivalent,
                }
            },
        }
    }
}

/// One formula's computation result.
#[derive(Debug, Clone, Copy)]
struct ScoreReport {
    formula_name: &'static str,
    native_value: f64,
    native_unit: &'static str,
    grade_equivalent: f64,
}

impl ScoreReport {
    fn format_message(&self, target_grade: f64, above_threshold: bool) -> String {
        // Always show the formula's native value so the user can verify
        // the metric. When the native unit is not a grade, also surface
        // the grade-equivalent that the threshold compares against.
        let native_block = if self.native_unit == "grade" {
            format!("{} grade {:.1}", self.formula_name, self.native_value)
        } else {
            format!(
                "{} {} {:.1} (≈ grade {:.1})",
                self.formula_name, self.native_unit, self.native_value, self.grade_equivalent,
            )
        };
        if above_threshold {
            format!(
                "{native_block} exceeds target {target_grade:.1}. Shorten sentences or choose \
                 simpler words.",
            )
        } else {
            format!("{native_block} (target ≤ {target_grade:.1}).")
        }
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
        lint_lang(text, profile, Language::En)
    }

    fn lint_lang(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        ReadabilityScore::for_profile(profile).check(&document, language)
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
    fn category_is_readability() {
        let text = "Short. Prose.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Readability);
    }

    #[test]
    fn one_diagnostic_per_document() {
        let text = "First short one. Second short one.\n\nThird short one. Fourth short one.";
        let diags = lint(text, Profile::Public);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn french_uses_kandel_moles() {
        let text = "Le chat est sur le tapis. Le chien court.";
        let diags = lint_lang(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
        assert!(
            diags[0].message.starts_with("Kandel-Moles"),
            "expected Kandel-Moles formula for FR, got: {}",
            diags[0].message
        );
        // Easy text — info under threshold.
        assert_eq!(diags[0].severity, Severity::Info);
    }

    #[test]
    fn french_dense_prose_warns_above_threshold() {
        let text = "L'implémentation de cette configuration particulière nécessite \
                    malheureusement une compréhension approfondie de plusieurs sous-composants \
                    architecturaux interdépendants, contraignant ainsi les collaborateurs en aval.";
        let diags = lint_lang(text, Profile::Public, Language::Fr);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warning);
        assert!(diags[0].message.starts_with("Kandel-Moles"));
        assert!(diags[0].message.contains("≈ grade"));
    }

    #[test]
    fn english_message_remains_grade_only_for_flesch_kincaid() {
        let text = "The cat sat on the mat.";
        let diags = lint_lang(text, Profile::Public, Language::En);
        assert!(diags[0].message.starts_with("Flesch-Kincaid grade"));
        assert!(!diags[0].message.contains("≈"));
    }

    #[test]
    fn unknown_language_falls_back_to_flesch_kincaid() {
        let text = "Short. Prose.";
        let diags = lint_lang(text, Profile::Public, Language::Unknown);
        assert!(diags[0].message.starts_with("Flesch-Kincaid"));
    }

    #[test]
    fn formula_for_language_dispatches_correctly() {
        assert_eq!(Formula::for_language(Language::En), Formula::FleschKincaid);
        assert_eq!(Formula::for_language(Language::Fr), Formula::KandelMoles);
        assert_eq!(
            Formula::for_language(Language::Unknown),
            Formula::FleschKincaid
        );
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
