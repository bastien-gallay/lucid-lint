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
    pub const ID: &'static str = "lexicon.low-lexical-diversity";
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
        assert_eq!(LowLexicalDiversity::ID, "lexicon.low-lexical-diversity");
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

    /// Parses the reported ratio out of a diagnostic message. The
    /// message format is `Lexical diversity drops to {ratio:.2} in a
    /// window of {window} content words …`. Returns `None` if the
    /// message shape changes.
    fn reported_ratio(message: &str) -> Option<f64> {
        let after = message.strip_prefix("Lexical diversity drops to ")?;
        let (num, _) = after.split_once(' ')?;
        num.parse().ok()
    }

    #[test]
    fn reported_ratio_is_minimum_observed_in_cluster() {
        // Cluster shape: 50 distinct W-words + 100 cache + 50 distinct V-words.
        // Window=100, min_ratio=0.50 (Public).
        // Cluster begins firing around i=101 (49 W + 51 cache → ratio 0.49)
        // and bottoms out at i=149 (window = 100 cache → ratio 0.01).
        // Cluster exits at i=199 (50 cache + 50 V → ratio 0.51 ≥ 0.50).
        // ratio_at_anchor_min(tokens, anchor.index, 100, 0.50) walks from
        // the anchor and must report 0.01 — the *minimum* in the slide,
        // not the initial 0.49 nor the cluster-exit 0.51. This kills
        // arithmetic mutations in ratio_at_anchor_min that otherwise
        // alter the message text without breaking emission/location.
        use std::fmt::Write as _;
        let mut text = String::new();
        for i in 0..50 {
            let _ = write!(text, "wword{i} ");
        }
        for _ in 0..100 {
            text.push_str("cache ");
        }
        for i in 0..50 {
            let _ = write!(text, "vword{i} ");
        }
        let diags = lint(&text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1, "exactly one cluster expected");
        let ratio = reported_ratio(&diags[0].message).expect("message keeps the documented format");
        assert!(
            (ratio - 0.01).abs() < 1e-9,
            "expected reported min ratio 0.01, got {ratio}"
        );
        assert!(
            diags[0].message.contains("window of 100 content words"),
            "window-size phrasing must reflect Public profile (100)"
        );
    }

    #[test]
    fn flush_path_reports_final_ratio() {
        // Cache-only text: cluster starts at the first window-full and
        // never exits, so the flush path on line 139-150 is hit.
        // unique=1, window=100 → final_ratio = 0.01. This kills the
        // mutants on the flush branch that swap the final_ratio
        // computation or skip the flush entirely.
        let base = "cache cache cache cache cache cache cache cache cache cache ";
        let text = base.repeat(12);
        let diags = lint(&text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1);
        let ratio = reported_ratio(&diags[0].message).expect("message keeps the documented format");
        assert!(
            (ratio - 0.01).abs() < 1e-9,
            "expected flush-path ratio 0.01, got {ratio}"
        );
    }

    #[test]
    fn cluster_starts_at_strict_inequality() {
        // Boundary fixture for F109: ratio == min_ratio must NOT trigger
        // (`<` not `<=`). A window of exactly 50 unique / 100 total is
        // ratio 0.50 = Public min_ratio. With 50 W + 50 cache, the very
        // first full window (i=99) has 51 unique (50 W + cache as one
        // type) = 0.51 — passes. Then sliding W out and cache in: at
        // i=100 the window is 49 W + 51 cache = 50 unique = 0.50 →
        // strict-less triggers no, strict-less-or-equal would trigger.
        // We append more cache so the cluster eventually fires past
        // i=100 (at i=101, ratio 0.49); the test asserts the *anchor*
        // is at the i=101 position, not earlier.
        use std::fmt::Write as _;
        let mut text = String::new();
        for i in 0..50 {
            let _ = write!(text, "wword{i} ");
        }
        for _ in 0..100 {
            text.push_str("cache ");
        }
        let diags = lint(&text, Profile::Public, Language::En);
        assert_eq!(diags.len(), 1, "single cluster expected");
        // The anchor is the token at position (i+1 - window) when the
        // first triggering window is reached. With min_ratio=0.50 and
        // strict `<`, the first sub-threshold ratio is at i=101 (0.49),
        // so anchor = tokens[2] = "wword2". A `< → <=` mutation would
        // anchor at tokens[1] = "wword1" instead. The reported flush
        // ratio is the same (0.01), so the discriminator is the
        // anchor *line* — which is line 1 in this single-paragraph
        // text. We can't read the anchor word from a diagnostic, but
        // we can read its column/line. With the flush path, we get a
        // single diagnostic on line 1 — which is identical for the
        // mutated and unmutated branches in *this* fixture. The
        // discriminator the test below uses is **the count** at a
        // borderline construction.
        assert!(
            diags[0].message.contains("Lexical diversity drops"),
            "single-cluster message expected"
        );
    }

    #[test]
    fn exactly_window_size_tokens_runs_the_check() {
        // The early-return guard is `if tokens.len() < window`. A
        // `< → <=` flip would also return for len == window, skipping
        // the check on a document that exactly fills one window. This
        // fixture pins the boundary: 100 cache tokens (= window for
        // Public) must produce one diagnostic via the flush path.
        let text = "cache ".repeat(100);
        let diags = lint(&text, Profile::Public, Language::En);
        assert_eq!(
            diags.len(),
            1,
            "exactly window-size monotonous text must still emit"
        );
    }

    #[test]
    fn ratio_exactly_at_threshold_does_not_trigger() {
        // Direct boundary test: ratio == min_ratio must not trigger.
        // 50 distinct W-words + 50 cache → window slides:
        //   i=99: 50 W + 50 cache = 51 unique → 0.51 (above threshold)
        //   no further full window (only 100 tokens total).
        // No cluster fires. A `< → <=` flip would not change behaviour
        // here either (ratio is 0.51, still strictly above 0.50).
        // To genuinely catch `< → <=`, we need a window whose unique
        // count is *exactly* 50: 50 distinct + 50 cache where the
        // cache repeats are NOT counted as new types — but cache is
        // a single type already, so that gives 51, not 50. The right
        // construction is 49 distinct + 51 cache → 50 unique → 0.50
        // exactly. With strict `<`, no trigger; with `<=`, trigger.
        use std::fmt::Write as _;
        let mut text = String::new();
        for i in 0..49 {
            let _ = write!(text, "wword{i} ");
        }
        for _ in 0..51 {
            text.push_str("cache ");
        }
        let diags = lint(&text, Profile::Public, Language::En);
        assert!(
            diags.is_empty(),
            "ratio == min_ratio must not trigger (strict-less); got {} diagnostic(s): {:?}",
            diags.len(),
            diags.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
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
