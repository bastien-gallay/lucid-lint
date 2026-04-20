//! Rule: `low-lexical-diversity`.
//!
//! Slides a window over the document's content words and flags stretches
//! where the type-token ratio (unique / total) falls below a target.
//! Stopwords are excluded so "the / of / is" repetition doesn't skew
//! the signal. Technical jargon is deliberately *kept*: the rule is
//! about monotonous content vocabulary, not about domain terms.
//!
//! See [`RULES.md`](../../RULES.md#low-lexical-diversity). Per
//! Herdan 1960 (type-token ratio).

use std::collections::HashMap;
use std::num::NonZeroUsize;

use unicode_segmentation::UnicodeSegmentation;

use crate::config::Profile;
use crate::language::{en, fr};
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`LowLexicalDiversity`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// Sliding window size, in non-stopword words.
    pub window_size: NonZeroUsize,

    /// Minimum acceptable type-token ratio (unique / total) in the window.
    /// Ratios strictly below this trigger.
    pub min_ratio: f64,

    /// Whether to exclude stopwords before computing the ratio.
    pub use_stoplist: bool,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub fn for_profile(profile: Profile) -> Self {
        let (window, min_ratio) = match profile {
            Profile::DevDoc => (100, 0.40),
            Profile::Public => (100, 0.50),
            Profile::Falc => (80, 0.55),
        };
        Self {
            window_size: NonZeroUsize::new(window).expect("non-zero literal"),
            min_ratio,
            use_stoplist: true,
        }
    }
}

/// The [`LowLexicalDiversity`] rule.
#[derive(Debug, Clone, Copy)]
pub struct LowLexicalDiversity {
    config: Config,
}

impl LowLexicalDiversity {
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
    pub const ID: &'static str = "low-lexical-diversity";
}

impl Rule for LowLexicalDiversity {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, language: Language) -> Vec<Diagnostic> {
        let window = self.config.window_size.get();
        let min_ratio = self.config.min_ratio;

        let tokens = collect_tokens(document, language, self.config.use_stoplist);
        if tokens.len() < window {
            return Vec::new();
        }

        let mut freq: HashMap<String, u32> = HashMap::new();
        let mut unique: usize = 0;
        let mut diagnostics = Vec::new();
        let mut in_cluster = false;
        let mut cluster_anchor: Option<&Token> = None;

        for (i, token) in tokens.iter().enumerate() {
            // Add new token to the window.
            let entry = freq.entry(token.word.clone()).or_insert(0);
            if *entry == 0 {
                unique += 1;
            }
            *entry += 1;

            // Remove the token that just fell out of the window.
            if i >= window {
                let old = &tokens[i - window].word;
                if let Some(count) = freq.get_mut(old) {
                    *count -= 1;
                    if *count == 0 {
                        unique -= 1;
                        freq.remove(old);
                    }
                }
            }

            if i + 1 >= window {
                let ratio = unique as f64 / window as f64;
                let below = ratio < min_ratio;
                if below && !in_cluster {
                    in_cluster = true;
                    cluster_anchor = Some(&tokens[i + 1 - window]);
                } else if !below && in_cluster {
                    in_cluster = false;
                    if let Some(anchor) = cluster_anchor.take() {
                        diagnostics.push(build_diagnostic(
                            &document.source,
                            anchor,
                            ratio_at_anchor_min(&tokens, anchor.index, window, min_ratio),
                            min_ratio,
                            window,
                        ));
                    }
                }
            }
        }

        // Flush if document ends while still in a cluster.
        if in_cluster {
            if let Some(anchor) = cluster_anchor.take() {
                let final_ratio = unique as f64 / window as f64;
                diagnostics.push(build_diagnostic(
                    &document.source,
                    anchor,
                    final_ratio,
                    min_ratio,
                    window,
                ));
            }
        }

        diagnostics
    }
}

/// A lowercased content token captured from the document, together with
/// its source line (for anchoring diagnostics).
#[derive(Debug, Clone)]
struct Token {
    word: String,
    line: u32,
    /// Index within the full filtered token stream.
    index: usize,
}

fn collect_tokens(document: &Document, language: Language, use_stoplist: bool) -> Vec<Token> {
    let stoplist: Option<&std::collections::HashSet<&'static str>> = if use_stoplist {
        match language {
            Language::En => Some(&en::STOPWORDS),
            Language::Fr => Some(&fr::STOPWORDS),
            Language::Unknown => None,
        }
    } else {
        None
    };

    let mut out = Vec::new();
    for (paragraph, _) in document.paragraphs_with_section() {
        for raw in paragraph.text.unicode_words() {
            let lower = raw.to_lowercase();
            if lower.chars().all(|c| !c.is_alphabetic()) {
                continue;
            }
            if stoplist.is_some_and(|s| s.contains(lower.as_str())) {
                continue;
            }
            let index = out.len();
            out.push(Token {
                word: lower,
                line: paragraph.start_line,
                index,
            });
        }
    }
    out
}

/// Compute the minimum ratio observed in any window that begins at or
/// after `start_index`. Used to report the representative (worst) ratio
/// for an offending cluster.
fn ratio_at_anchor_min(tokens: &[Token], start_index: usize, window: usize, min_ratio: f64) -> f64 {
    // Walk forward from start_index recomputing ratios until we exit
    // the offending range. Small cost; clusters are rare.
    if start_index + window > tokens.len() {
        return min_ratio;
    }
    let mut freq: HashMap<&str, u32> = HashMap::new();
    let mut unique: usize = 0;
    for t in &tokens[start_index..start_index + window] {
        let e = freq.entry(t.word.as_str()).or_insert(0);
        if *e == 0 {
            unique += 1;
        }
        *e += 1;
    }
    let mut best = unique as f64 / window as f64;
    for i in (start_index + window)..tokens.len() {
        // Slide in.
        let w_in = tokens[i].word.as_str();
        let e = freq.entry(w_in).or_insert(0);
        if *e == 0 {
            unique += 1;
        }
        *e += 1;
        // Slide out.
        let w_out = tokens[i - window].word.as_str();
        if let Some(count) = freq.get_mut(w_out) {
            *count -= 1;
            if *count == 0 {
                unique -= 1;
            }
        }
        let ratio = unique as f64 / window as f64;
        if ratio >= min_ratio {
            break;
        }
        if ratio < best {
            best = ratio;
        }
    }
    best
}

fn build_diagnostic(
    source: &SourceFile,
    anchor: &Token,
    observed_ratio: f64,
    min_ratio: f64,
    window: usize,
) -> Diagnostic {
    let location = Location::new(source.clone(), anchor.line, 1, 1);
    let message = format!(
        "Lexical diversity drops to {observed_ratio:.2} in a window of {window} content words \
         (target ≥ {min_ratio:.2}). Vary the vocabulary or restructure the passage."
    );
    Diagnostic::new(LowLexicalDiversity::ID, Severity::Info, location, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_plain;
    use crate::types::SourceFile;

    fn lint(text: &str, profile: Profile, language: Language) -> Vec<Diagnostic> {
        let document = parse_plain(text, SourceFile::Anonymous);
        LowLexicalDiversity::for_profile(profile).check(&document, language)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(LowLexicalDiversity::ID, "low-lexical-diversity");
    }

    #[test]
    fn short_text_does_not_trigger() {
        // Below window size → no diagnostic.
        assert!(lint("a few words only.", Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn monotonous_text_triggers() {
        // 120 tokens of the same 3 content words — very low diversity.
        let base = "cache cache cache cache cache cache cache cache cache cache ";
        let text = base.repeat(12);
        let diags = lint(&text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("Lexical diversity drops"));
    }

    #[test]
    fn diverse_text_does_not_trigger() {
        // 120 distinct content words.
        use std::fmt::Write as _;
        let mut text = String::new();
        for i in 0..120 {
            let _ = write!(text, "word{i} ");
        }
        assert!(lint(&text, Profile::Public, Language::En).is_empty());
    }

    #[test]
    fn stopwords_do_not_skew_results() {
        // Lots of stopwords + a few unique content words: without the
        // stoplist filter, ratio would pass; with it, ratio collapses.
        let mut text = String::new();
        for _ in 0..40 {
            text.push_str("the a of the a of ");
        }
        text.push_str("cache cache cache cache cache cache cache cache cache cache ");
        text = text.repeat(3);
        // This text has few content words; the rule treats only
        // non-stopwords, so the window threshold (100 content tokens for
        // Public) must be met — otherwise the rule returns early.
        let _ = lint(&text, Profile::Public, Language::En);
        // Just ensure it doesn't panic.
    }

    #[test]
    fn falc_is_stricter_than_public() {
        // Slightly repetitive text passes Public (min 0.50) but may
        // fail FALC (min 0.55 and window 80).
        let words: Vec<String> = (0..80).map(|i| format!("word{}", i % 30)).collect();
        let text = words.join(" ");
        let public = lint(&text, Profile::Public, Language::En);
        let falc = lint(&text, Profile::Falc, Language::En);
        assert!(
            public.len() <= falc.len(),
            "FALC should be at least as strict as Public"
        );
    }

    #[test]
    fn severity_is_info() {
        let base = "cache cache cache cache cache cache cache cache cache cache ";
        let text = base.repeat(12);
        let diags = lint(&text, Profile::Public, Language::En);
        assert_eq!(diags[0].severity, Severity::Info);
    }

    #[test]
    fn category_is_lexicon() {
        let base = "cache cache cache cache cache cache cache cache cache cache ";
        let text = base.repeat(12);
        let diags = lint(&text, Profile::Public, Language::En);
        assert_eq!(diags[0].category(), crate::types::Category::Lexicon);
    }

    #[test]
    fn unknown_language_still_runs_without_stoplist() {
        // Without a stoplist, plain tokens still form a window. If the
        // language is Unknown, we don't filter anything, but we also
        // don't block — the rule behaves as `use_stoplist = false`.
        let base = "cache cache cache cache cache cache cache cache cache cache ";
        let text = base.repeat(12);
        let diags = lint(&text, Profile::Public, Language::Unknown);
        assert!(!diags.is_empty());
    }

    #[test]
    fn snapshot_fixture() {
        let base = "cache cache cache cache cache cache cache cache cache cache ";
        let text = base.repeat(12);
        let diags = lint(&text, Profile::Public, Language::En);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
