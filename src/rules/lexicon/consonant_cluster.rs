//! Rule: `consonant-cluster`.
//!
//! Flags tokens whose longest run of consecutive consonants meets or
//! exceeds a per-profile threshold. Dense consonant clusters are a
//! known decoding barrier for dyslexic readers (BDA Dyslexia Style
//! Guide): the reader must hold more phonemes in working memory
//! before the next vowel "releases" the syllable.
//!
//! The vowel set is language-aware — French accented vowels
//! (`é`, `è`, `ê`, `ï`, …) count as vowels, otherwise a word like
//! `étranger` would read as a six-consonant cluster. `y` is treated
//! as a vowel everywhere (lenient). Hyphens, apostrophes, and
//! whitespace break the run so compound forms are not falsely merged.
//!
//! See [`RULES.md`](../../RULES.md#consonant-cluster) for the reference
//! entry.

use std::num::NonZeroU32;

use unicode_segmentation::UnicodeSegmentation;

use crate::condition::ConditionTag;
use crate::config::Profile;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity};

/// Configuration for [`ConsonantCluster`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Minimum run of consecutive consonants that triggers a warning.
    pub min_run_length: NonZeroU32,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let min = match profile {
            Profile::DevDoc => 6,
            Profile::Public => 5,
            Profile::Falc => 4,
        };
        Self {
            min_run_length: NonZeroU32::new(min).expect("non-zero literal"),
        }
    }
}

/// The [`ConsonantCluster`] rule.
#[derive(Debug, Clone, Copy)]
pub struct ConsonantCluster {
    config: Config,
}

impl ConsonantCluster {
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
    pub const ID: &'static str = "lexicon.consonant-cluster";

    /// Condition tags this rule targets.
    pub const TAGS: &'static [ConditionTag] = &[ConditionTag::Dyslexia, ConditionTag::General];
}

impl Rule for ConsonantCluster {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn condition_tags(&self) -> &'static [ConditionTag] {
        Self::TAGS
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let min = self.config.min_run_length.get();
        let mut diagnostics = Vec::new();

        for (paragraph, section_title) in document.paragraphs_with_section() {
            for (line_offset, line) in paragraph.text.lines().enumerate() {
                for hit in find_clusters(line, language, min) {
                    let line_number = paragraph
                        .start_line
                        .saturating_add(u32::try_from(line_offset).unwrap_or(u32::MAX));
                    let column = u32::try_from(hit.column).unwrap_or(u32::MAX);
                    let length = u32::try_from(hit.word.chars().count()).unwrap_or(u32::MAX);
                    let location =
                        Location::new(document.source.clone(), line_number, column, length);
                    let message = format!(
                        "Word \"{}\" contains a run of {} consecutive consonants. Dense \
                         consonant clusters are a decoding barrier for dyslexic readers \
                         (BDA Dyslexia Style Guide). Consider a shorter or more common \
                         synonym.",
                        hit.word, hit.run_length
                    );
                    let mut diag = Diagnostic::new(Self::ID, Severity::Warning, location, message);
                    if let Some(title) = section_title {
                        diag = diag.with_section(title);
                    }
                    diagnostics.push(diag);
                }
            }
        }

        diagnostics
    }
}

#[derive(Debug)]
struct Hit {
    word: String,
    run_length: u32,
    /// 1-based grapheme column of the first character of the word.
    column: usize,
}

/// Scan one line for words whose longest consonant run reaches `min`.
fn find_clusters(line: &str, language: Language, min: u32) -> Vec<Hit> {
    let mut hits = Vec::new();
    let mut current_word = String::new();
    let mut current_start_col: Option<usize> = None;
    let mut run = 0u32;
    let mut max_run = 0u32;

    // Walk graphemes with their 1-based column.
    for (idx, grapheme) in line.graphemes(true).enumerate() {
        let col = idx + 1;
        let first = grapheme.chars().next().unwrap_or(' ');
        if is_word_char(first) {
            if current_start_col.is_none() {
                current_start_col = Some(col);
            }
            current_word.push_str(grapheme);
            if is_consonant(first, language) {
                run += 1;
                if run > max_run {
                    max_run = run;
                }
            } else {
                run = 0;
            }
        } else {
            // Non-word grapheme: close the current word.
            if let Some(start) = current_start_col.take() {
                if max_run >= min {
                    hits.push(Hit {
                        word: std::mem::take(&mut current_word),
                        run_length: max_run,
                        column: start,
                    });
                } else {
                    current_word.clear();
                }
            }
            run = 0;
            max_run = 0;
        }
    }

    if let Some(start) = current_start_col {
        if max_run >= min {
            hits.push(Hit {
                word: current_word,
                run_length: max_run,
                column: start,
            });
        }
    }

    hits
}

/// A word character is a letter in any script. Digits, apostrophes,
/// and hyphens are deliberately excluded so they break the word and
/// reset the consonant run (compound forms like `dys-lexic` do not
/// falsely merge into a long cluster).
fn is_word_char(c: char) -> bool {
    c.is_alphabetic()
}

/// A character is a consonant when it is alphabetic, not in the
/// vowel set, and not a digit. `y` is always treated as a vowel
/// (lenient fallback that avoids edge cases like English `fly`,
/// `rhythm` where `y` carries the syllable).
fn is_consonant(c: char, language: Language) -> bool {
    if !c.is_alphabetic() {
        return false;
    }
    !is_vowel(c, language)
}

fn is_vowel(c: char, language: Language) -> bool {
    let lower = c.to_lowercase().next().unwrap_or(c);
    if matches!(lower, 'a' | 'e' | 'i' | 'o' | 'u' | 'y') {
        return true;
    }
    match language {
        Language::Fr => matches!(
            lower,
            'à' | 'â'
                | 'ä'
                | 'æ'
                | 'é'
                | 'è'
                | 'ê'
                | 'ë'
                | 'î'
                | 'ï'
                | 'ô'
                | 'ö'
                | 'œ'
                | 'ù'
                | 'û'
                | 'ü'
                | 'ÿ'
        ),
        // English + Unknown: no accented vowels in the default
        // lexicon, but borrowed words (`café`, `naïve`) should still
        // decode correctly. Accept the common latin-1 accented vowels
        // as vowels in all languages.
        Language::En | Language::Unknown => matches!(
            lower,
            'à' | 'â' | 'ä' | 'é' | 'è' | 'ê' | 'ë' | 'î' | 'ï' | 'ô' | 'ö' | 'ù' | 'û' | 'ü'
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_markdown, parse_plain};
    use crate::types::{Category, SourceFile};

    fn lint(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        ConsonantCluster::for_profile(profile).check(&document, language)
    }

    fn lint_md(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_markdown(text, SourceFile::Anonymous);
        ConsonantCluster::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(ConsonantCluster::ID, "lexicon.consonant-cluster");
    }

    #[test]
    fn tags_carry_dyslexia_and_general() {
        assert!(ConsonantCluster::TAGS.contains(&ConditionTag::Dyslexia));
        assert!(ConsonantCluster::TAGS.contains(&ConditionTag::General));
    }

    #[test]
    fn category_is_lexicon() {
        // "strengths" has n-g-t-h-s → 5 consecutive consonants.
        let diags = lint("The strengths matter.", Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].category(), Category::Lexicon);
    }

    #[test]
    fn common_words_do_not_trigger_under_public() {
        assert!(lint(
            "The quick brown fox jumps over the lazy dog.",
            Profile::Public,
            Language::En
        )
        .is_empty());
    }

    #[test]
    fn strengths_triggers_under_public() {
        // s-t-r-e-n-g-t-h-s → longest run is n-g-t-h-s = 5.
        let diags = lint("Our strengths matter.", Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"strengths\""));
        assert!(diags[0].message.contains("5 consecutive"));
    }

    #[test]
    fn falc_catches_shorter_runs() {
        // "strict" has s-t-r run = 3 and c-t run = 2; max = 3 — not flagged.
        assert!(lint("A strict test.", Profile::Falc, Language::En).is_empty());
        // "shrinks" has s-h-r (3) and n-k-s (3); max = 3. Not flagged.
        assert!(lint("It shrinks.", Profile::Falc, Language::En).is_empty());
        // "twelfths" has t-w (2), l-f-t-h-s (5). FALC threshold = 4 → flag.
        let diags = lint("The twelfths shifted.", Profile::Falc, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"twelfths\""));
    }

    #[test]
    fn dev_doc_tolerates_five_run() {
        // "strengths" run of 5: flagged under Public, passes DevDoc (min 6).
        assert!(!lint("Strengths.", Profile::Public, Language::En).is_empty());
        assert!(lint("Strengths.", Profile::DevDoc, Language::En).is_empty());
    }

    #[test]
    fn hyphen_breaks_the_word() {
        // `dys-lexic` = "dys" (1 run of 2) and "lexic" (no long run).
        // Without the hyphen-break, you could imagine d-y-s-l-x-c = 6,
        // but the hyphen closes the word.
        assert!(
            lint("A dys-lexic reader.", Profile::Falc, Language::En).is_empty(),
            "hyphen must break the word"
        );
    }

    #[test]
    fn apostrophe_breaks_the_word() {
        assert!(lint("rock'n'roll", Profile::Falc, Language::En).is_empty());
    }

    #[test]
    fn fr_accented_vowel_is_not_a_consonant() {
        // "étranger" would be e-t-r-n-g-r = 6 consonants if é were a
        // consonant. With FR vowels it becomes é-t-r-a-n-g-e-r → max
        // run 2.
        assert!(lint("C'est étranger.", Profile::Falc, Language::Fr).is_empty());
    }

    #[test]
    fn fr_real_cluster_triggers() {
        // "constructions" in FR: c-n-s-t-r-c-t-n-s → longest run is
        // n-s-t-r = 4 (then vowel u) and n-s = 2. FALC (4) flags it.
        let diags = lint(
            "Les constructions sont claires.",
            Profile::Falc,
            Language::Fr,
        );
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("\"constructions\""));
    }

    #[test]
    fn uppercase_words_also_trigger() {
        let diags = lint("STRENGTHS.", Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
    }

    #[test]
    fn y_is_treated_as_a_vowel() {
        // "rhythm" = r-h-y-t-h-m. With y as vowel: r-h (2), t-h-m (3).
        // No flag at FALC (4).
        assert!(lint("The rhythm.", Profile::Falc, Language::En).is_empty());
    }

    #[test]
    fn fenced_code_block_content_is_ignored() {
        let md = "Intro.\n\n```\nstrengths twelfths\n```\n\nPlain prose.\n";
        assert!(lint_md(md, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn config_thresholds_are_as_documented() {
        assert_eq!(Config::for_profile(Profile::DevDoc).min_run_length.get(), 6);
        assert_eq!(Config::for_profile(Profile::Public).min_run_length.get(), 5);
        assert_eq!(Config::for_profile(Profile::Falc).min_run_length.get(), 4);
    }

    #[test]
    fn snapshot_fixture() {
        let text = "Our strengths and twelfths shifted. All clear.";
        let diags = lint(text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
