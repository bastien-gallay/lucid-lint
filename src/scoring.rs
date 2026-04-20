//! Hybrid scoring model (F14).
//!
//! Aggregates diagnostics into a global `X / max` score plus five
//! per-category sub-scores. Composition stacks three mechanics so no
//! single rule or noisy section can dominate the signal:
//!
//! 1. **Weighted sum** — each hit costs `weight × severity_multiplier`.
//! 2. **Density normalization** — costs are divided by `words / 1000`,
//!    floored at `0.2` (i.e. treat docs shorter than 200 words as 200-word
//!    documents) so tiny fixtures aren't punished.
//! 3. **Per-category cap** — a single runaway rule can eat at most
//!    [`ScoringConfig::category_cap`] out of [`ScoringConfig::category_max`].
//!
//! The taxonomy is fixed at 5 categories ([`Category::ALL`]). Letter grades,
//! traffic lights, and reading-time are deferred (see `ROADMAP.md` F39–F41).

use serde::Serialize;

use crate::types::{Category, Diagnostic, Severity};

/// A single `X / max` score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Score {
    /// Current value, clamped to `0..=max`.
    pub value: u32,
    /// Maximum achievable value.
    pub max: u32,
}

impl Score {
    /// Build a score, clamping `value` to `max`.
    #[must_use]
    pub const fn new(value: u32, max: u32) -> Self {
        Self {
            value: if value > max { max } else { value },
            max,
        }
    }

    /// A perfect score of `max / max`.
    #[must_use]
    pub const fn perfect(max: u32) -> Self {
        Self { value: max, max }
    }
}

/// A category paired with its sub-score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CategoryScore {
    /// Which category this score covers.
    pub category: Category,
    /// The sub-score.
    pub score: Score,
}

/// A full scoring report for a single document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Scorecard {
    /// Aggregate score = sum of per-category values, with shared max.
    pub global: Score,
    /// Per-category sub-scores in the fixed [`Category::ALL`] order.
    pub per_category: [CategoryScore; 5],
}

/// Tunable scoring parameters.
///
/// Defaults are calibrated during v0.2 dogfood. Users override via the
/// `[scoring]` table in `lucid-lint.toml`.
#[derive(Debug, Clone)]
pub struct ScoringConfig {
    /// Maximum score per category. Default `20` — five categories sum to `100`.
    pub category_max: u32,
    /// Maximum cost a single category can accrue. Default `15`, i.e. one
    /// noisy rule can eat at most 75 % of its category.
    pub category_cap: u32,
    /// Per-rule weight overrides. Missing keys fall back to [`default_weight_for`].
    pub weight_overrides: std::collections::BTreeMap<String, u32>,
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            category_max: DEFAULT_CATEGORY_MAX,
            category_cap: DEFAULT_CATEGORY_CAP,
            weight_overrides: std::collections::BTreeMap::new(),
        }
    }
}

/// Default per-category maximum.
pub const DEFAULT_CATEGORY_MAX: u32 = 20;
/// Default per-category cap.
pub const DEFAULT_CATEGORY_CAP: u32 = 15;
/// Density floor: documents under 200 words are scored as 200-word documents.
pub const DENSITY_FLOOR_WORDS: u32 = 200;

/// Cost multiplier per severity level.
#[must_use]
pub const fn severity_multiplier(severity: Severity) -> u32 {
    match severity {
        Severity::Info => 1,
        Severity::Warning => 3,
        Severity::Error => 5,
    }
}

/// Rule ids explicitly registered in [`default_weight_for`].
///
/// Used by the documentation coverage test (`tests/rule_docs_coverage.rs`)
/// to assert that every shipped rule has been considered for weighting,
/// rather than silently inheriting the uniform fallback.
pub const WEIGHTED_RULE_IDS: &[&str] = &[
    "readability-score",
    "sentence-too-long",
    "paragraph-too-long",
    "deep-subordination",
    "passive-voice",
    "unclear-antecedent",
    "heading-jump",
    "deeply-nested-lists",
    "excessive-commas",
    "long-enumeration",
    "consecutive-long-sentences",
    "repetitive-connectors",
    "low-lexical-diversity",
    "excessive-nominalization",
    "unexplained-abbreviation",
    "weasel-words",
    "jargon-undefined",
    "nested-negation",
    "conditional-stacking",
    "all-caps-shouting",
];

/// Default weight for a rule, keyed by `rule_id`.
///
/// Rules that cost more cognitive effort to land on a reader get higher
/// weights. Unknown ids fall back to `1` (the uniform floor).
///
/// A rule id deliberately returning `1` must still appear in
/// [`WEIGHTED_RULE_IDS`] so the coverage test can distinguish "considered
/// and kept at the floor" from "forgotten".
#[must_use]
pub fn default_weight_for(rule_id: &str) -> u32 {
    match rule_id {
        "readability-score" => 5,
        "sentence-too-long"
        | "paragraph-too-long"
        | "deep-subordination"
        | "passive-voice"
        | "unclear-antecedent"
        | "nested-negation"
        | "conditional-stacking" => 2,
        _ => 1,
    }
}

/// Compute a [`Scorecard`] from a set of diagnostics and the document's word count.
///
/// The composition formula is:
///
/// ```text
/// per_cat_cost  = min( Σ(weight × severity_multiplier) / (words/1000),
///                      category_cap )
/// cat_score     = category_max − per_cat_cost    (clamped ≥ 0)
/// global_score  = Σ cat_score
/// ```
///
/// With `words` clamped to [`DENSITY_FLOOR_WORDS`] before the division.
#[must_use]
pub fn compute(diagnostics: &[Diagnostic], word_count: u32, config: &ScoringConfig) -> Scorecard {
    let density_words = word_count.max(DENSITY_FLOOR_WORDS);
    let density_divisor = f64::from(density_words) / 1000.0;

    let mut costs: [f64; 5] = [0.0; 5];
    for diag in diagnostics {
        // Config overrides win; otherwise reuse the weight the rule emitted
        // (which is already seeded from `default_weight_for` in `Diagnostic::new`).
        let weight = config
            .weight_overrides
            .get(&diag.rule_id)
            .copied()
            .unwrap_or(diag.weight);
        let hit_cost = f64::from(weight * severity_multiplier(diag.severity));
        let idx = category_index(diag.category());
        costs[idx] += hit_cost;
    }

    let cap = f64::from(config.category_cap);
    let max = config.category_max;

    let per_category: [CategoryScore; 5] = core::array::from_fn(|i| {
        let normalized = (costs[i] / density_divisor).min(cap).max(0.0);
        // Normalized is bounded to `[0, category_cap]` above, so casting the
        // rounded value to u32 neither truncates nor loses a sign. Round
        // half-up for the human-facing score.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let cost_u32 = normalized.round() as u32;
        let score_value = max.saturating_sub(cost_u32);
        CategoryScore {
            category: Category::ALL[i],
            score: Score::new(score_value, max),
        }
    });

    let global_value: u32 = per_category.iter().map(|c| c.score.value).sum();
    let global_max = max * 5;

    Scorecard {
        global: Score::new(global_value, global_max),
        per_category,
    }
}

const fn category_index(c: Category) -> usize {
    match c {
        Category::Structure => 0,
        Category::Rhythm => 1,
        Category::Lexicon => 2,
        Category::Syntax => 3,
        Category::Readability => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Location, Severity, SourceFile};

    fn diag(rule: &str, severity: Severity) -> Diagnostic {
        Diagnostic::new(
            rule,
            severity,
            Location::new(SourceFile::Anonymous, 1, 1, 1),
            "m",
        )
    }

    #[test]
    fn zero_diagnostics_scores_perfect() {
        let config = ScoringConfig::default();
        let card = compute(&[], 1000, &config);
        assert_eq!(card.global.value, card.global.max);
        assert_eq!(card.global.max, DEFAULT_CATEGORY_MAX * 5);
        for cs in &card.per_category {
            assert_eq!(cs.score.value, cs.score.max);
        }
    }

    #[test]
    fn empty_document_does_not_panic() {
        let config = ScoringConfig::default();
        let card = compute(&[], 0, &config);
        assert_eq!(card.global.value, card.global.max);
    }

    #[test]
    fn single_warning_costs_weight_times_severity_normalized() {
        // sentence-too-long: weight 2, severity Warning: multiplier 3 → cost 6
        // 1000 words → density divisor 1.0 → per_cat_cost = 6
        // Structure score = 20 - 6 = 14
        let config = ScoringConfig::default();
        let diags = [diag("sentence-too-long", Severity::Warning)];
        let card = compute(&diags, 1000, &config);
        let structure = card
            .per_category
            .iter()
            .find(|c| c.category == Category::Structure)
            .unwrap();
        assert_eq!(structure.score.value, 14);
        // Other categories untouched.
        for cs in &card.per_category {
            if cs.category != Category::Structure {
                assert_eq!(cs.score.value, DEFAULT_CATEGORY_MAX);
            }
        }
    }

    #[test]
    fn category_cap_limits_runaway_rule() {
        // 20 warnings of sentence-too-long at 1000 words:
        // raw cost = 20 × 2 × 3 = 120 → well above cap 15.
        // Structure score = 20 - 15 = 5 regardless of further hits.
        let config = ScoringConfig::default();
        let diags: Vec<_> = (0..20)
            .map(|_| diag("sentence-too-long", Severity::Warning))
            .collect();
        let card = compute(&diags, 1000, &config);
        let structure = card
            .per_category
            .iter()
            .find(|c| c.category == Category::Structure)
            .unwrap();
        assert_eq!(
            structure.score.value,
            DEFAULT_CATEGORY_MAX - DEFAULT_CATEGORY_CAP
        );
    }

    #[test]
    fn density_floor_protects_short_documents() {
        // A 10-word doc with one warning still divides by 200/1000 = 0.2,
        // not 10/1000 = 0.01 (which would massively inflate cost).
        let config = ScoringConfig::default();
        let diags = [diag("weasel-words", Severity::Warning)];
        let card = compute(&diags, 10, &config);
        // weasel-words: weight 1 × severity 3 = 3; divided by 0.2 = 15; capped at 15.
        // Lexicon = 20 - 15 = 5.
        let lex = card
            .per_category
            .iter()
            .find(|c| c.category == Category::Lexicon)
            .unwrap();
        assert_eq!(lex.score.value, 5);
    }

    #[test]
    fn weight_override_takes_effect() {
        let mut config = ScoringConfig::default();
        config.weight_overrides.insert("weasel-words".into(), 10);
        let diags = [diag("weasel-words", Severity::Info)];
        // 10 × 1 (info) = 10 at 1000 words → Lexicon = 20 - 10 = 10.
        let card = compute(&diags, 1000, &config);
        let lex = card
            .per_category
            .iter()
            .find(|c| c.category == Category::Lexicon)
            .unwrap();
        assert_eq!(lex.score.value, 10);
    }

    #[test]
    fn default_weight_for_prioritises_costly_rules() {
        assert_eq!(default_weight_for("readability-score"), 5);
        assert_eq!(default_weight_for("sentence-too-long"), 2);
        assert_eq!(default_weight_for("weasel-words"), 1);
        assert_eq!(default_weight_for("unknown"), 1);
    }

    #[test]
    fn severity_multiplier_is_monotonic() {
        assert!(severity_multiplier(Severity::Info) < severity_multiplier(Severity::Warning));
        assert!(severity_multiplier(Severity::Warning) < severity_multiplier(Severity::Error));
    }
}
