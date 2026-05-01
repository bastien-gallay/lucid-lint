//! Shared helper: detect inline prose enumerations.
//!
//! Used by two rules with different thresholds:
//!
//! - [`crate::rules::structure::long_enumeration`] flags enumerations
//!   of 5+ items as "convert to a bulleted list" suggestions.
//! - [`crate::rules::structure::excessive_commas`] discounts commas
//!   inside detected enumerations so that sentences like "red, green,
//!   blue, and yellow" don't trip the comma-density threshold.
//!
//! Detection recognizes the Oxford-comma pattern documented in
//! `RULES.md` for `long-enumeration`: a run of comma-separated short
//! segments ending with a connector segment (`, and X` / `, or X` /
//! `, plus X` / `, et X` / `, ou X`). Non-Oxford enumerations
//! ("A, B, C and D" with no comma before the connector) are
//! deliberately out of scope for v0.1.
//!
//! `plus` is treated as an honorary Oxford terminator (F22 v0.3 second
//! tranche). Syntactically it occupies the same slot as `and` / `et` —
//! `profile, format, min-score, plus working-directory and args` is a
//! list, not a clause pile — and the existing connector-stop guard in
//! the walk-back already rejects `…, and X, plus we did Y` shapes
//! where `plus` would otherwise extend the run.

use crate::types::Language;

/// Maximum number of words per segment for it to count as "short" under
/// the tight pass. Kept deliberately tight (2) so subordinate clauses and
/// non-enumerable prose fragments do not get swallowed into a false
/// enumeration when walking backwards from a trailing connector. The
/// relaxed pass below lifts this for rhythmically-regular runs.
const MAX_SEGMENT_WORDS: usize = 2;

/// Upper bound on segment length under the relaxed pass (F22 v0.3 slice).
/// A run of segments each ≤ 4 words is accepted only when it also passes
/// [`run_is_rhythmically_regular`] — segments cluster around one length.
/// Subordination tends to mix short and long clauses, so this rhythm
/// constraint is what separates "list of items" from "stack of clauses".
const MAX_RHYTHMIC_SEGMENT_WORDS: usize = 4;

/// Maximum permitted spread (max − min word count) inside a relaxed-pass
/// run. ≤ 2 lets `category, severity, default weight, parameters per
/// profile, EN/FR examples, and suppression` (counts {1,1,2,3,2,1})
/// through while rejecting heavy subordination piles.
const MAX_RHYTHMIC_SPREAD: usize = 2;

/// Minimum item count for the relaxed pass to fire. Higher than the
/// tight pass's [`MIN_ITEMS_FOR_DETECTION`] (=3) because rhythm alone
/// could not distinguish a 3–4-item list of short noun phrases from
/// a 3–4-clause subordinate pile. Five is the smallest floor that
/// clears every Oxford hit in the F22 corpus this slice targets
/// (#12 / #24 / #25 — list-intro prose absorbs the first item, so the
/// algorithmic item count tops out at 5).
const MIN_ITEMS_FOR_RHYTHMIC_DETECTION: usize = 5;

/// EN clause-onset markers. The relaxed walk-back stops at any segment
/// whose first word matches one of these — they signal a clause
/// boundary, not a noun-phrase item, and the tight word-count limit
/// was doing double-duty as a clause-boundary stop that the relaxed
/// pass forfeits. Conservative shortlist: subject pronouns and the
/// most common subordinators / coordinators that introduce a clause.
/// Function words like "the" / "a" are deliberately excluded because
/// a noun phrase can legitimately lead with them ("the team, the
/// manager, the engineer"). Case-insensitive match.
const EN_CLAUSE_ONSET_MARKERS: &[&str] = &[
    "we",
    "i",
    "you",
    "he",
    "she",
    "it",
    "they",
    "this",
    "that",
    "these",
    "those",
    "although",
    "though",
    "while",
    "when",
    "since",
    "because",
    "if",
    "however",
    "moreover",
    "furthermore",
    "but",
    "yet",
    "so",
];

/// FR clause-onset markers — same role as [`EN_CLAUSE_ONSET_MARKERS`].
const FR_CLAUSE_ONSET_MARKERS: &[&str] = &[
    "nous",
    "je",
    "tu",
    "il",
    "elle",
    "on",
    "ils",
    "elles",
    "ce",
    "cet",
    "cette",
    "ces",
    "bien",
    "alors",
    "tandis",
    "quand",
    "puisque",
    "parce",
    "si",
    "mais",
    "or",
    "donc",
    "car",
    "cependant",
    "toutefois",
    "néanmoins",
];

/// Minimum number of items before we recognize a sequence as an
/// enumeration at all (used for comma-discounting). `long-enumeration`
/// layers its own `min_items` on top.
const MIN_ITEMS_FOR_DETECTION: u32 = 3;

/// A detected prose enumeration within a sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumeration {
    /// Byte offset (in the sentence) where the enumeration starts.
    pub start: usize,

    /// Byte offset (exclusive) where the enumeration ends.
    pub end: usize,

    /// Number of items (comma-separated segments, including the trailing
    /// connector segment).
    pub items: u32,

    /// Number of commas that separate the items.
    pub commas: u32,
}

/// Detect Oxford-style prose enumerations in a sentence.
///
/// Returns at most one enumeration per run; overlapping enumerations are
/// collapsed into the widest one seen.
#[must_use]
pub fn detect_enumerations(sentence: &str, language: Language) -> Vec<Enumeration> {
    let connectors: &[&str] = match language {
        Language::En => &["and", "or", "plus"],
        Language::Fr => &["et", "ou", "plus"],
        Language::Unknown => return Vec::new(),
    };

    let segments = split_comma_segments(sentence);
    let mut result: Vec<Enumeration> = Vec::new();

    let clause_onsets: &[&str] = match language {
        Language::En => EN_CLAUSE_ONSET_MARKERS,
        Language::Fr => FR_CLAUSE_ONSET_MARKERS,
        Language::Unknown => return Vec::new(),
    };
    for (i, seg) in segments.iter().enumerate() {
        let connector_text = &sentence[seg.range.clone()];
        if !segment_starts_with_connector(connector_text, connectors) {
            continue;
        }
        // Two-pass walk-back: try the existing tight limit first
        // (subordinate-resistant), then a relaxed pass capped at 4 words
        // per segment but gated on rhythmic regularity (F22 v0.3 slice).
        // Tight pass preserved verbatim so existing behavior cannot
        // regress on inputs the v0.1/v0.2 detector already accepted.
        let start_idx =
            walk_back_under_tight_limit(&segments, sentence, i, connectors).or_else(|| {
                walk_back_under_rhythmic_limit(&segments, sentence, i, connectors, clause_onsets)
            });
        let Some(start_idx) = start_idx else {
            continue;
        };
        let items = u32::try_from(i - start_idx + 1).unwrap_or(u32::MAX);
        if items < MIN_ITEMS_FOR_DETECTION {
            continue;
        }
        let start_byte = segments[start_idx].range.start;
        let end_byte = seg.range.end;
        // Commas between items: for N items there are N-1 separators.
        let commas = items.saturating_sub(1);
        push_or_merge(
            &mut result,
            Enumeration {
                start: start_byte,
                end: end_byte,
                items,
                commas,
            },
        );
    }
    result
}

/// Walk backwards from a connector segment under the strict word limit
/// (`MAX_SEGMENT_WORDS = 2`). Returns the run's starting segment index if
/// it covers at least [`MIN_ITEMS_FOR_DETECTION`] items, else `None`.
fn walk_back_under_tight_limit(
    segments: &[Segment],
    sentence: &str,
    connector_idx: usize,
    connectors: &[&str],
) -> Option<usize> {
    let mut start_idx = connector_idx;
    while start_idx > 0 {
        let prev_text = &sentence[segments[start_idx - 1].range.clone()];
        if word_count(prev_text) == 0 || word_count(prev_text) > MAX_SEGMENT_WORDS {
            break;
        }
        if segment_starts_with_connector(prev_text, connectors) {
            break;
        }
        start_idx -= 1;
    }
    let items = connector_idx - start_idx + 1;
    if items >= MIN_ITEMS_FOR_DETECTION as usize {
        Some(start_idx)
    } else {
        None
    }
}

/// Relaxed walk-back: allows up to [`MAX_RHYTHMIC_SEGMENT_WORDS`] per
/// segment, but only accepts the resulting run if its word counts are
/// rhythmically regular (see [`run_is_rhythmically_regular`]) and no
/// segment opens with a clause-onset marker.
fn walk_back_under_rhythmic_limit(
    segments: &[Segment],
    sentence: &str,
    connector_idx: usize,
    connectors: &[&str],
    clause_onsets: &[&str],
) -> Option<usize> {
    let mut start_idx = connector_idx;
    while start_idx > 0 {
        let prev_text = &sentence[segments[start_idx - 1].range.clone()];
        let count = word_count(prev_text);
        if count == 0 || count > MAX_RHYTHMIC_SEGMENT_WORDS {
            break;
        }
        if segment_starts_with_connector(prev_text, connectors) {
            break;
        }
        if segment_starts_with_word_in(prev_text, clause_onsets) {
            break;
        }
        start_idx -= 1;
    }
    let items = connector_idx - start_idx + 1;
    if items < MIN_ITEMS_FOR_RHYTHMIC_DETECTION {
        return None;
    }
    // Collect word counts for the rhythm check. Non-connector segments
    // count as-is; the connector segment contributes its final-item word
    // count (i.e. with the leading "and"/"et"/… stripped) so a one-word
    // tail item doesn't fall outside the spread it would have shown
    // without the connector word.
    let mut counts: Vec<usize> = (start_idx..connector_idx)
        .map(|k| word_count(&sentence[segments[k].range.clone()]))
        .collect();
    let connector_text = &sentence[segments[connector_idx].range.clone()];
    counts.push(connector_segment_item_word_count(
        connector_text,
        connectors,
    ));
    if !run_is_rhythmically_regular(&counts) {
        return None;
    }
    Some(start_idx)
}

/// Word counts cluster tightly enough to read as a list of items (rather
/// than a stack of subordinate clauses). All counts must be ≥ 1 and
/// ≤ [`MAX_RHYTHMIC_SEGMENT_WORDS`], with spread ≤ [`MAX_RHYTHMIC_SPREAD`].
/// Item count is enforced by the caller, not here.
fn run_is_rhythmically_regular(counts: &[usize]) -> bool {
    let (Some(&min), Some(&max)) = (counts.iter().min(), counts.iter().max()) else {
        return false;
    };
    if min == 0 || max > MAX_RHYTHMIC_SEGMENT_WORDS {
        return false;
    }
    max - min <= MAX_RHYTHMIC_SPREAD
}

/// Number of words in a segment (whitespace-split, non-empty filter).
fn word_count(segment: &str) -> usize {
    segment.split_whitespace().count()
}

/// Word count of the item carried by a connector segment ("and X, Y" →
/// counts the words after "and"). Returns 0 if the connector word stands
/// alone (degenerate input).
fn connector_segment_item_word_count(segment: &str, connectors: &[&str]) -> usize {
    let trimmed = segment.trim_start();
    let lower = trimmed.to_lowercase();
    for connector in connectors {
        if let Some(rest) = lower.strip_prefix(connector) {
            if rest.chars().next().is_some_and(char::is_whitespace) {
                return word_count(rest);
            }
        }
    }
    word_count(segment)
}

/// Whether the segment's first word matches any of the markers.
/// Case-insensitive. Strips a leading punctuation/whitespace run before
/// inspecting the first word so that a segment like ` "we packed red"`
/// (with leading whitespace + opening quote) still matches `we`.
fn segment_starts_with_word_in(segment: &str, markers: &[&str]) -> bool {
    let trimmed = segment.trim_start_matches(|c: char| !c.is_alphabetic());
    let first_word = trimmed
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .next()
        .unwrap_or("");
    if first_word.is_empty() {
        return false;
    }
    let lower = first_word.to_lowercase();
    markers.iter().any(|m| *m == lower)
}

/// Total number of commas that fall inside any detected enumeration.
/// Used by `excessive-commas` to discount enumeration-commas.
#[must_use]
pub fn enumeration_comma_count(sentence: &str, language: Language) -> u32 {
    detect_enumerations(sentence, language)
        .iter()
        .map(|e| e.commas)
        .sum()
}

/// Count commas inside `(A, B, C, …)` parenthesised token lists.
///
/// Recognises three or more short comma-separated segments inside a
/// balanced pair of parentheses. Language-agnostic: parens behave the
/// same in EN and FR, and no connector word is required.
///
/// Used by `excessive-commas` as a second discount source alongside
/// [`enumeration_comma_count`], to suppress the dominant FP driver on
/// doc prose (F22): backticked identifier lists, example enumerations,
/// and other "aside" lists that the Oxford-only detector never sees.
///
/// Only the outermost parenthesised run is considered — nested parens
/// disqualify the containing span to keep the heuristic conservative.
#[must_use]
pub fn parenthesised_list_comma_count(sentence: &str) -> u32 {
    let bytes = sentence.as_bytes();
    let mut total: u32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'(' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut depth = 1usize;
        let mut j = start;
        while j < bytes.len() {
            match bytes[j] {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                },
                _ => {},
            }
            j += 1;
        }
        if depth != 0 {
            // Unbalanced parenthesis: give up on the rest of the sentence.
            break;
        }
        let inner = &sentence[start..j];
        if !inner.contains('(') {
            total = total.saturating_add(parenthesised_run_comma_count(inner));
        }
        i = j + 1;
    }
    total
}

fn parenthesised_run_comma_count(inner: &str) -> u32 {
    let segments: Vec<&str> = inner.split(',').collect();
    if segments.len() < MIN_ITEMS_FOR_DETECTION as usize {
        return 0;
    }
    // Empty segments are accepted: the markdown parser strips inline
    // code contents, so `(`a`, `b`, `c`)` reaches us as `(, , )`. A
    // surviving non-empty segment would still need to be short.
    if !segments
        .iter()
        .all(|s| s.split_whitespace().count() <= MAX_SEGMENT_WORDS)
    {
        return 0;
    }
    u32::try_from(segments.len() - 1).unwrap_or(u32::MAX)
}

struct Segment {
    range: std::ops::Range<usize>,
}

/// Split a sentence into comma-separated segments, preserving byte
/// ranges. Commas are excluded from the segment ranges.
fn split_comma_segments(sentence: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let bytes = sentence.as_bytes();
    let mut start = 0;
    for (idx, &b) in bytes.iter().enumerate() {
        if b == b',' {
            segments.push(Segment { range: start..idx });
            start = idx + 1;
        }
    }
    segments.push(Segment {
        range: start..bytes.len(),
    });
    segments
}

/// Whether a segment starts (after trimming) with one of the given
/// connector words followed by whitespace.
fn segment_starts_with_connector(segment: &str, connectors: &[&str]) -> bool {
    let trimmed = segment.trim_start();
    let lower = trimmed.to_lowercase();
    for connector in connectors {
        if let Some(rest) = lower.strip_prefix(connector) {
            if rest.chars().next().is_some_and(char::is_whitespace) {
                return true;
            }
        }
    }
    false
}

/// Push a new enumeration onto the result, merging with the last one if
/// they overlap or touch (the rightmost anchor absorbs the leftmost).
fn push_or_merge(out: &mut Vec<Enumeration>, candidate: Enumeration) {
    if let Some(last) = out.last_mut() {
        if candidate.start <= last.end {
            // Extend the existing enumeration rightwards.
            last.end = candidate.end.max(last.end);
            last.items = last.items.max(candidate.items);
            last.commas = last.commas.max(candidate.commas);
            return;
        }
    }
    out.push(candidate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_english_oxford_enumeration() {
        let s = "red, green, blue, and yellow";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
        assert_eq!(enums[0].commas, 3);
    }

    #[test]
    fn detects_french_oxford_enumeration() {
        let s = "rouge, vert, bleu, et jaune";
        let enums = detect_enumerations(s, Language::Fr);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
    }

    #[test]
    fn detects_or_as_connector() {
        let s = "a, b, c, or d";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
    }

    #[test]
    fn ignores_non_oxford_form() {
        // No comma before "and" → out of scope for v0.1.
        let s = "red, green, blue and yellow";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn needs_at_least_three_items() {
        let s = "a, and b";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn rejects_long_segments() {
        // Second segment has 8 words — too long for the pattern.
        let s =
            "first, the second segment is way too long to count as a short item, third, and fourth";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn does_not_swallow_surrounding_subordinates() {
        // The real enumeration here is only 2 items ("green, and blue files"),
        // below the minimum, so no enumeration should be reported.
        let s = "Note, although we agreed, to pack the red, green, and blue files, carefully";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn enumeration_comma_count_sums_all_enumerations() {
        let s = "red, green, blue, and yellow";
        assert_eq!(enumeration_comma_count(s, Language::En), 3);
    }

    #[test]
    fn unknown_language_disables_detection() {
        let s = "red, green, blue, and yellow";
        assert!(detect_enumerations(s, Language::Unknown).is_empty());
    }

    #[test]
    fn parenthesised_list_counts_inner_commas() {
        let s = "Pick a colour (red, green, blue, yellow) for the frame.";
        assert_eq!(parenthesised_list_comma_count(s), 3);
    }

    #[test]
    fn parenthesised_list_needs_three_segments() {
        // Two-item parens are asides, not lists.
        let s = "The pair (foo, bar) matters.";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_rejects_long_segments() {
        // One segment has > MAX_SEGMENT_WORDS words.
        let s = "See (red, a very long qualifying clause here, blue, yellow).";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_ignores_nested_parens() {
        // Nested parens disqualify the containing span.
        let s = "See (red, green (emerald), blue, yellow) here.";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_ignores_unbalanced_parens() {
        let s = "See (red, green, blue here.";
        assert_eq!(parenthesised_list_comma_count(s), 0);
    }

    #[test]
    fn parenthesised_list_handles_multiple_runs() {
        let s = "Digits (`1`, `2`, `3`) and spellings (`one`, `two`, `three`) differ.";
        assert_eq!(parenthesised_list_comma_count(s), 4);
    }

    #[test]
    fn parenthesised_list_counts_empty_segments_from_stripped_code() {
        // The markdown parser drops inline code contents, so
        // `(`a`, `b`, `c`, `d`)` reaches rules as `(, , , )`.
        let s = "The tokens (, , , ) are listed.";
        assert_eq!(parenthesised_list_comma_count(s), 3);
    }

    #[test]
    fn parenthesised_list_is_language_agnostic() {
        // Same sentence shape, no connector word — both languages see it.
        let s = "Voyelles (`a`, `e`, `i`, `o`, `u`) courantes.";
        assert_eq!(parenthesised_list_comma_count(s), 4);
    }

    #[test]
    fn case_insensitive_connector() {
        let s = "red, green, blue, And yellow";
        assert_eq!(detect_enumerations(s, Language::En).len(), 1);
    }

    // ---- F22 v0.3 slice — rhythmic relaxation ----

    #[test]
    fn rhythmic_three_to_four_word_oxford_run_is_detected() {
        // F22 corpus #12 / #24 shape. Word counts {1,1,2,3,2,1} with the
        // connector segment's item ("suppression"). Spread = 2, all ≤ 4.
        let s = "category, severity, default weight, parameters per profile, EN/FR examples, \
                 and suppression";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 6);
        assert_eq!(enums[0].commas, 5);
    }

    #[test]
    fn rhythmic_run_in_french_is_detected() {
        // FR mirror — "categorie, severite, poids par defaut, seuils par
        // profil, exemples, et neutralisation" — counts {1,1,3,3,1,1}.
        let s =
            "categorie, severite, poids par defaut, seuils par profil, exemples, et neutralisation";
        let enums = detect_enumerations(s, Language::Fr);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 6);
    }

    #[test]
    fn arrhythmic_subordination_pile_is_rejected() {
        // Three-clause subordinate pile. The relaxed walk-back captures
        // only `to revise the palette` + `before shipping` (the 5-word
        // `after much debate among stakeholders` exceeds the relaxed
        // word cap), giving 3 items — short of MIN_ITEMS_FOR_RHYTHMIC.
        // Tight pass also fails (4-word segment too long). No detection.
        let s = "the team decided, after much debate among stakeholders, to revise the palette, \
                 before shipping, and despite the tight deadline";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn five_item_run_at_rhythmic_word_cap_is_accepted() {
        // Five segments at 4 words each, spread = 0. At the item-count floor.
        let s = "alpha beta gamma delta, epsilon zeta eta theta, iota kappa lambda mu, \
                 nu xi omicron pi, and rho sigma tau upsilon";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 5);
    }

    #[test]
    fn rhythmic_run_above_word_cap_is_rejected() {
        // One segment has 5 words, exceeding MAX_RHYTHMIC_SEGMENT_WORDS=4.
        // The relaxed walk-back stops there.
        let s = "first item, second item, the third item is five words, fourth item, \
                 and fifth";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn rhythmic_spread_above_two_is_rejected() {
        // Five items but counts include a 4 next to a 1 — spread = 3 fails
        // the rhythm guard even though item count clears the floor.
        let s = "alpha, beta gamma delta epsilon, zeta, eta theta iota kappa, and mu";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn rhythmic_run_below_five_items_is_rejected() {
        // Four items, all 3 words, spread = 0 — perfect rhythm but below
        // MIN_ITEMS_FOR_RHYTHMIC_DETECTION. The relaxed pass forfeits the
        // tight word-count's clause-boundary signal, so we demand more
        // confidence (item count) before accepting.
        let s = "first new item, second new item, third new item, and fourth new item";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn clause_onset_marker_stops_relaxed_walk_back() {
        // The relaxed pass must NOT walk past a segment opening with a
        // clause-onset pronoun ("we"). Existing-test fixture pattern:
        // "Note, although we agreed, we packed red, green, and blue,
        // carefully, and quietly." — the run for "and blue" must stop at
        // "we packed red", giving < 3 items. Same for "although".
        let s = "Note, although we agreed, we packed red, green, and blue, carefully, and quietly";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn clause_onset_marker_works_in_french() {
        // FR mirror — "nous" (we) is a clause-onset marker.
        let s = "Note, alors que nous avions accepté, nous avons emballé rouge, vert, et bleu";
        assert!(detect_enumerations(s, Language::Fr).is_empty());
    }

    #[test]
    fn tight_pass_still_accepts_pure_one_word_run() {
        // Sanity: the tight pass is the first try, so a one-word run is
        // accepted unconditionally — no rhythm check, no item-count floor
        // beyond the standard MIN_ITEMS_FOR_DETECTION=3.
        let s = "red, green, blue, and yellow";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
    }

    #[test]
    fn rhythmic_relaxation_does_not_break_subordinate_guard() {
        // Same fixture as `does_not_swallow_surrounding_subordinates`.
        // Tight pass fails (4w segment). Relaxed walk-back stops at the
        // clause-onset "although" → only 1 item collected, well below the
        // floor. Subordinate-pile rejected.
        let s = "Note, although we agreed, to pack the red, green, and blue files, carefully";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    // ---- F22 v0.3 second tranche — `plus`-closed enumerations ----

    #[test]
    fn plus_closes_an_english_enumeration() {
        // F22 corpus #11 shape: `profile, format, min-score, plus
        // working-directory and args`. Tight pass walks back over three
        // 1-word segments, terminator is "plus".
        let s = "profile, format, min-score, plus working-directory and args";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
        assert_eq!(enums[0].commas, 3);
    }

    #[test]
    fn plus_closes_a_french_enumeration() {
        let s = "profil, format, score minimal, plus repertoire et arguments";
        let enums = detect_enumerations(s, Language::Fr);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 4);
    }

    #[test]
    fn plus_after_oxford_does_not_extend_the_oxford_run() {
        // Two terminator segments. The "and" segment closes a 3-item
        // Oxford run; the "plus" segment that follows must not absorb
        // it — the connector-stop guard makes walk-back refuse to fold
        // a prior connector segment into a new run.
        let s = "apples, oranges, and bananas, plus laughed";
        let enums = detect_enumerations(s, Language::En);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].items, 3);
    }

    #[test]
    fn plus_without_a_preceding_run_does_not_trigger() {
        // Only one comma-separated segment before "plus" → below the
        // 3-item floor. "plus" alone never invents an enumeration.
        let s = "we shopped today, plus we ordered grapes";
        assert!(detect_enumerations(s, Language::En).is_empty());
    }

    #[test]
    fn run_is_rhythmically_regular_unit() {
        // The helper checks word-count distribution only; item-count is
        // the caller's responsibility.
        assert!(run_is_rhythmically_regular(&[1, 1, 2, 3, 2, 1])); // F22 corpus #12
        assert!(run_is_rhythmically_regular(&[2, 2, 2]));
        assert!(run_is_rhythmically_regular(&[1, 2, 3]));
        assert!(!run_is_rhythmically_regular(&[])); // empty
        assert!(!run_is_rhythmically_regular(&[1, 2, 4])); // spread = 3
        assert!(!run_is_rhythmically_regular(&[0, 1, 2])); // empty segment
        assert!(!run_is_rhythmically_regular(&[1, 2, 5])); // > MAX cap
    }
}
