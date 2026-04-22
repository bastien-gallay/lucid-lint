//! Rule: `consecutive-long-sentences`.
//!
//! Flags runs of long sentences. An isolated long sentence is manageable;
//! three in a row almost guarantees an attention-fragile reader loses
//! their place. This rule catches the *rhythm* complement to
//! [`crate::rules::SentenceTooLong`].
//!
//! Streaks are counted within a paragraph: a paragraph break is a
//! natural rhythm reset.
//!
//! See [`RULES.md`](../../RULES.md#consecutive-long-sentences) for
//! rationale and thresholds.

use std::num::NonZeroU32;

use crate::config::Profile;
use crate::parser::{split_sentences, word_count, Document};
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`ConsecutiveLongSentences`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Word count above which a sentence is "long" for rhythm purposes.
    /// Intentionally lower than `sentence-too-long`'s `max_words`.
    pub word_threshold: NonZeroU32,

    /// Maximum allowed run of consecutive long sentences within a paragraph.
    /// A streak strictly exceeding this triggers.
    pub max_consecutive: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let (threshold, consecutive) = match profile {
            Profile::DevDoc => (20, 3),
            Profile::Public => (15, 2),
            Profile::Falc => (10, 2),
        };
        Self {
            word_threshold: NonZeroU32::new(threshold).expect("non-zero literal"),
            max_consecutive: NonZeroU32::new(consecutive).expect("non-zero literal"),
        }
    }
}

/// The [`ConsecutiveLongSentences`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ConsecutiveLongSentences {
    config: Config,
}

impl ConsecutiveLongSentences {
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
    pub const ID: &'static str = "rhythm.consecutive-long-sentences";
}

impl Rule for ConsecutiveLongSentences {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let threshold = self.config.word_threshold.get();
        let max = self.config.max_consecutive.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            let sentences = split_sentences(&paragraph.text, paragraph.start_line, 1);
            let mut streak_start: Option<usize> = None;
            let mut streak_len: u32 = 0;
            let mut already_flagged = false;

            for (idx, sentence) in sentences.iter().enumerate() {
                if word_count(&sentence.text) > threshold {
                    if streak_start.is_none() {
                        streak_start = Some(idx);
                        already_flagged = false;
                    }
                    streak_len += 1;
                    if streak_len > max && !already_flagged {
                        let first =
                            &sentences[streak_start.expect("streak_start set on first long")];
                        diagnostics.push(build_diagnostic(
                            &document.source,
                            first.line,
                            first.column,
                            &first.text,
                            streak_len,
                            threshold,
                            max,
                            section_title,
                        ));
                        already_flagged = true;
                    }
                } else {
                    streak_start = None;
                    streak_len = 0;
                    already_flagged = false;
                }
            }
        }

        diagnostics
    }
}

#[allow(clippy::too_many_arguments)]
fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    column: u32,
    anchor_text: &str,
    streak_len: u32,
    threshold: u32,
    max: u32,
    section: Option<&str>,
) -> Diagnostic {
    let length = u32::try_from(anchor_text.chars().count()).unwrap_or(u32::MAX);
    let location = Location::new(source.clone(), line, column, length);
    let message = format!(
        "{streak_len} consecutive sentences each exceed {threshold} words (max run allowed: \
         {max}). Break the rhythm with a shorter sentence."
    );
    let diag = Diagnostic::new(
        ConsecutiveLongSentences::ID,
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

    /// 17-word sentence: exceeds `Public` threshold (15), under `DevDoc` (20).
    const LONG_PUBLIC: &str = "This particular sentence is carefully built to reach exactly \
                               eighteen words for consistent rhythm rule testing here.";

    /// Confirm the fixture really is 17 words so the tests read honestly.
    #[test]
    fn fixture_long_public_is_17_words() {
        assert_eq!(word_count(LONG_PUBLIC), 17);
    }

    fn lint(text: &str, profile: Profile) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        ConsecutiveLongSentences::for_profile(profile).check(&document, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(
            ConsecutiveLongSentences::ID,
            "rhythm.consecutive-long-sentences"
        );
    }

    #[test]
    fn short_sentences_do_not_trigger() {
        let text = "A short one. Another one. Fine.";
        assert!(lint(text, Profile::Public).is_empty());
    }

    #[test]
    fn isolated_long_sentence_does_not_trigger() {
        let text = format!("Short. {LONG_PUBLIC} Short again.");
        assert!(lint(&text, Profile::Public).is_empty());
    }

    #[test]
    fn at_max_consecutive_does_not_trigger() {
        // Public max_consecutive is 2; exactly 2 long in a row is allowed.
        let text = format!("{LONG_PUBLIC} {LONG_PUBLIC} Short.");
        assert!(lint(&text, Profile::Public).is_empty());
    }

    #[test]
    fn exceeding_max_consecutive_triggers_once() {
        // 3 long in a row under Public (max 2) → trigger.
        let text = format!("{LONG_PUBLIC} {LONG_PUBLIC} {LONG_PUBLIC} Short.");
        let diags = lint(&text, Profile::Public);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("3 consecutive"));
    }

    #[test]
    fn streak_resets_between_paragraphs() {
        // Two long sentences, paragraph break, two more long sentences.
        // Each paragraph is within its own budget (max 2 consecutive).
        let text = format!("{LONG_PUBLIC} {LONG_PUBLIC}\n\n{LONG_PUBLIC} {LONG_PUBLIC}");
        assert!(lint(&text, Profile::Public).is_empty());
    }

    #[test]
    fn streak_resets_after_short_sentence() {
        let text = format!("{LONG_PUBLIC} {LONG_PUBLIC} Short. {LONG_PUBLIC} {LONG_PUBLIC}");
        assert!(lint(&text, Profile::Public).is_empty());
    }

    #[test]
    fn falc_profile_is_stricter() {
        // FALC word_threshold is 10; a 12-word sentence counts as "long".
        let twelve = "This simple sentence contains a tally that sums up to about twelve.";
        assert_eq!(word_count(twelve), 12);
        let text = format!("{twelve} {twelve} {twelve}");
        assert!(!lint(&text, Profile::Falc).is_empty());
    }

    #[test]
    fn config_thresholds_match_rules_md() {
        let dev = Config::for_profile(Profile::DevDoc);
        assert_eq!(dev.word_threshold.get(), 20);
        assert_eq!(dev.max_consecutive.get(), 3);
        let pub_ = Config::for_profile(Profile::Public);
        assert_eq!(pub_.word_threshold.get(), 15);
        assert_eq!(pub_.max_consecutive.get(), 2);
        let falc = Config::for_profile(Profile::Falc);
        assert_eq!(falc.word_threshold.get(), 10);
        assert_eq!(falc.max_consecutive.get(), 2);
    }

    #[test]
    fn category_is_rhythm() {
        let text = format!("{LONG_PUBLIC} {LONG_PUBLIC} {LONG_PUBLIC}");
        let diags = lint(&text, Profile::Public);
        assert_eq!(diags[0].category(), crate::types::Category::Rhythm);
    }

    #[test]
    fn snapshot_fixture() {
        let text = format!("Lead-in. {LONG_PUBLIC} {LONG_PUBLIC} {LONG_PUBLIC} Wrap-up.");
        let diags = lint(&text, Profile::Public);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
