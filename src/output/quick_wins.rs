//! Actionable hint hooks rendered after the diagnostic list on the TTY
//! surface (F-report-quick-wins).
//!
//! The block is purely additive: JSON / SARIF stay structural until a CI
//! consumer asks. Two seed shapes share the same threshold-fire pattern:
//!
//! 1. **Acronym whitelist hint** — when ≥ [`ACRONYM_HINT_THRESHOLD`]
//!    occurrences of `lexicon.unexplained-abbreviation` share one token,
//!    surface `add "X" to whitelist (N hits suppressed)`. Top
//!    [`ACRONYM_HINT_TOP_N`] tokens by count.
//! 2. **Single-rule hot-spot hint** — when one rule fires ≥
//!    [`HOTSPOT_HINT_THRESHOLD`] times in one file, surface
//!    `<rule-id> dominates this file — see <docs URL>`.
//!
//! The block caps at [`MAX_HINTS`] lines so it never crowds the score
//! banner. The acronym path also suppresses a hot-spot hint for the same
//! `(file, lexicon.unexplained-abbreviation)` pair, since the whitelist
//! advice is the more actionable one.
//!
//! Thresholds live next to the hint definitions; no central config knob
//! until a second consumer asks.

use std::collections::{BTreeMap, HashSet};

use crate::explain::DOCS_BASE;
use crate::types::Diagnostic;

/// Maximum number of hint lines the block can emit. Keeps the block from
/// crowding the score banner that follows.
pub(super) const MAX_HINTS: usize = 5;

/// Minimum shared-token occurrences before the acronym hint fires.
pub(super) const ACRONYM_HINT_THRESHOLD: usize = 3;

/// Minimum same-rule-in-one-file occurrences before the hot-spot hint fires.
pub(super) const HOTSPOT_HINT_THRESHOLD: usize = 10;

/// Top-N tokens surfaced when several acronyms qualify.
pub(super) const ACRONYM_HINT_TOP_N: usize = 3;

const UNEXPLAINED_ABBREVIATION: &str = "lexicon.unexplained-abbreviation";

/// Compute the quick-wins hint lines for `diagnostics`.
///
/// Returns plain-text bodies (without the leading `→ ` arrow and without
/// color); the renderer is responsible for the prefix and any dimming.
pub(super) fn hints(diagnostics: &[Diagnostic]) -> Vec<String> {
    let mut out = Vec::new();

    let (acronym_lines, covered_pairs) = acronym_hints(diagnostics);
    out.extend(acronym_lines);

    for line in hotspot_hints(diagnostics, &covered_pairs) {
        if out.len() >= MAX_HINTS {
            break;
        }
        out.push(line);
    }

    out.truncate(MAX_HINTS);
    out
}

/// Acronym whitelist hints. Returns the rendered lines and the set of
/// `(file, rule_id)` pairs the caller should skip when emitting hot-spot
/// hints — the whitelist advice already covers them.
fn acronym_hints(diagnostics: &[Diagnostic]) -> (Vec<String>, HashSet<(String, String)>) {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut files_for_token: BTreeMap<String, HashSet<String>> = BTreeMap::new();

    for diag in diagnostics {
        if diag.rule_id != UNEXPLAINED_ABBREVIATION {
            continue;
        }
        let Some(token) = extract_acronym_token(&diag.message) else {
            continue;
        };
        *counts.entry(token.clone()).or_insert(0) += 1;
        files_for_token
            .entry(token)
            .or_default()
            .insert(diag.location.file.to_string());
    }

    let mut ranked: Vec<(String, usize)> = counts
        .into_iter()
        .filter(|(_, n)| *n >= ACRONYM_HINT_THRESHOLD)
        .collect();
    // Most hits first; ties broken by token (BTreeMap iteration is alpha,
    // so `sort_by` only needs to invert by count).
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut covered: HashSet<(String, String)> = HashSet::new();
    let mut lines = Vec::new();
    for (token, n) in ranked.into_iter().take(ACRONYM_HINT_TOP_N) {
        lines.push(format!(
            "add \"{token}\" to [rules.{UNEXPLAINED_ABBREVIATION}].whitelist ({n} hits suppressed)"
        ));
        if let Some(files) = files_for_token.get(&token) {
            for file in files {
                covered.insert((file.clone(), UNEXPLAINED_ABBREVIATION.to_string()));
            }
        }
    }
    (lines, covered)
}

/// Single-rule hot-spot hints. `covered` holds `(file, rule_id)` pairs
/// already addressed by an upstream hint (currently the acronym path).
fn hotspot_hints(diagnostics: &[Diagnostic], covered: &HashSet<(String, String)>) -> Vec<String> {
    // Preserve first-occurrence order so output is stable when several
    // rules tie at threshold.
    let mut counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut order: Vec<(String, String)> = Vec::new();

    for diag in diagnostics {
        let key = (diag.location.file.to_string(), diag.rule_id.clone());
        if !counts.contains_key(&key) {
            order.push(key.clone());
        }
        *counts.entry(key).or_insert(0) += 1;
    }

    let mut lines = Vec::new();
    for key in order {
        let n = counts[&key];
        if n < HOTSPOT_HINT_THRESHOLD {
            continue;
        }
        if covered.contains(&key) {
            continue;
        }
        let (file, rule_id) = key;
        lines.push(format!(
            "{rule_id} dominates {file} ({n} hits) — see {DOCS_BASE}/rules/{rule_id}"
        ));
    }
    lines
}

/// Extract the acronym token from an `unexplained-abbreviation` message.
///
/// Messages emit as `Acronym "TOKEN" is not defined on first use.`; the
/// token sits between the first pair of ASCII double quotes. Returns
/// `None` if the message has been reshaped — the caller skips that
/// diagnostic rather than guessing.
fn extract_acronym_token(message: &str) -> Option<String> {
    let start = message.find('"')?;
    let rest = &message[start + 1..];
    let end = rest.find('"')?;
    let token = &rest[..end];
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Location, Severity, SourceFile};

    fn diag(rule_id: &str, file: SourceFile, message: &str) -> Diagnostic {
        Diagnostic::new(
            rule_id,
            Severity::Warning,
            Location::new(file, 1, 1, 1),
            message,
        )
    }

    fn acronym(file: SourceFile, token: &str) -> Diagnostic {
        diag(
            UNEXPLAINED_ABBREVIATION,
            file,
            &format!("Acronym \"{token}\" is not defined on first use."),
        )
    }

    fn anon_acronym(token: &str) -> Diagnostic {
        acronym(SourceFile::Anonymous, token)
    }

    #[test]
    fn silent_when_no_diagnostics_qualify() {
        let diags: Vec<Diagnostic> = (0..2).map(|_| anon_acronym("HTTP")).collect();
        assert!(hints(&diags).is_empty());
    }

    #[test]
    fn acronym_hint_fires_at_threshold() {
        let diags: Vec<Diagnostic> = (0..ACRONYM_HINT_THRESHOLD)
            .map(|_| anon_acronym("HTTP"))
            .collect();
        let lines = hints(&diags);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("\"HTTP\""));
        assert!(lines[0].contains("3 hits suppressed"));
    }

    #[test]
    fn acronym_hint_caps_at_top_n_and_orders_by_count() {
        let mut diags = Vec::new();
        diags.extend((0..5).map(|_| anon_acronym("HTTP")));
        diags.extend((0..4).map(|_| anon_acronym("WCAG")));
        diags.extend((0..3).map(|_| anon_acronym("RGAA")));
        diags.extend((0..3).map(|_| anon_acronym("FALC")));
        let lines = hints(&diags);
        assert_eq!(lines.len(), ACRONYM_HINT_TOP_N);
        // HTTP (5) before WCAG (4) before the alpha-first 3-hit token (FALC).
        assert!(lines[0].contains("\"HTTP\""));
        assert!(lines[1].contains("\"WCAG\""));
        assert!(lines[2].contains("\"FALC\""));
    }

    #[test]
    fn hotspot_hint_fires_at_threshold() {
        let path = SourceFile::Path(std::path::PathBuf::from("README.md"));
        let diags: Vec<Diagnostic> = (0..HOTSPOT_HINT_THRESHOLD)
            .map(|_| diag("structure.sentence-too-long", path.clone(), "msg"))
            .collect();
        let lines = hints(&diags);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("structure.sentence-too-long dominates"));
        assert!(lines[0].contains("README.md"));
        assert!(lines[0].contains("10 hits"));
    }

    #[test]
    fn hotspot_hint_silent_below_threshold() {
        let path = SourceFile::Path(std::path::PathBuf::from("README.md"));
        let diags: Vec<Diagnostic> = (0..HOTSPOT_HINT_THRESHOLD - 1)
            .map(|_| diag("structure.sentence-too-long", path.clone(), "msg"))
            .collect();
        assert!(hints(&diags).is_empty());
    }

    #[test]
    fn acronym_path_suppresses_hotspot_for_same_file_rule_pair() {
        // 12 acronym hits in one file would also trip the hot-spot
        // threshold — the acronym hint takes precedence.
        let path = SourceFile::Path(std::path::PathBuf::from("notes.md"));
        let diags: Vec<Diagnostic> = (0..12).map(|_| acronym(path.clone(), "HTTP")).collect();
        let lines = hints(&diags);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("whitelist"));
    }

    #[test]
    fn block_caps_at_max_hints() {
        let mut diags = Vec::new();
        // Three acronym hints (HTTP, WCAG, RGAA) plus three hot-spots in
        // distinct files — that is 6 candidates, must trim to MAX_HINTS.
        diags.extend((0..3).map(|_| anon_acronym("HTTP")));
        diags.extend((0..3).map(|_| anon_acronym("WCAG")));
        diags.extend((0..3).map(|_| anon_acronym("RGAA")));
        for name in ["a.md", "b.md", "c.md"] {
            let path = SourceFile::Path(std::path::PathBuf::from(name));
            diags.extend(
                (0..HOTSPOT_HINT_THRESHOLD)
                    .map(|_| diag("structure.sentence-too-long", path.clone(), "msg")),
            );
        }
        let lines = hints(&diags);
        assert_eq!(lines.len(), MAX_HINTS);
    }

    #[test]
    fn extract_acronym_token_handles_canonical_message() {
        assert_eq!(
            extract_acronym_token("Acronym \"WCAG\" is not defined on first use."),
            Some("WCAG".to_string())
        );
    }

    #[test]
    fn extract_acronym_token_returns_none_on_unexpected_shape() {
        assert!(extract_acronym_token("Acronym WCAG not defined.").is_none());
        assert!(extract_acronym_token("Acronym \"\" is not defined.").is_none());
    }
}
