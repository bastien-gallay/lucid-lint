//! `lucid-lint explain <rule-id>` — print the bundled rule documentation.
//!
//! Rule docs live in `docs/src/rules/<rule-id>.md` on disk (the mdBook
//! source). They are compiled into the binary via `include_str!` so
//! `explain` works offline with zero I/O beyond stdout.
//!
//! The `tests/rule_docs_coverage.rs` test suite guarantees every rule
//! in `default_rules()` has a matching page. A companion test in this
//! module guarantees every such page is also bundled here.

/// Base URL of the published docs site.
///
/// Matches `documentation` in `Cargo.toml`. One place to change if the
/// host moves; callers must never hard-code the host anywhere else.
pub const DOCS_BASE: &str = "https://bastien-gallay.github.io/lucid-lint";

/// Bundled rule documentation, keyed by rule id.
///
/// Keep the list in kebab-sorted order so a missing entry is easy to
/// spot in review. Adding a new rule means:
///   1. create `docs/src/rules/<rule-id>.md`
///   2. append a `doc!("<rule-id>")` line here
///   3. `cargo test every_rule_has_a_bundled_doc` to confirm
// The rule-id uses the `category.rule-name` form (F29-slim), while the
// mdBook source file currently still lives at `docs/src/rules/<kebab>.md`
// — docs rearchitecture into category subdirs is a later slice. The
// macro therefore carries both the id and the URL slug (filename) so
// `canonical_url` can resolve to the live mdBook URL.
macro_rules! doc {
    ($id:literal, $slug:literal) => {
        (
            $id,
            $slug,
            include_str!(concat!("../docs/src/rules/", $slug, ".md")),
        )
    };
}

/// `(rule_id, docs_url_slug, bundled_body)` tuples, sorted by `rule_id`.
pub(crate) const RULE_DOCS: &[(&str, &str, &str)] = &[
    doc!("lexicon.all-caps-shouting", "all-caps-shouting"),
    doc!("lexicon.consonant-cluster", "consonant-cluster"),
    doc!(
        "lexicon.excessive-nominalization",
        "excessive-nominalization"
    ),
    doc!("lexicon.jargon-undefined", "jargon-undefined"),
    doc!("lexicon.low-lexical-diversity", "low-lexical-diversity"),
    doc!("lexicon.redundant-intensifier", "redundant-intensifier"),
    doc!(
        "lexicon.unexplained-abbreviation",
        "unexplained-abbreviation"
    ),
    doc!("lexicon.weasel-words", "weasel-words"),
    doc!("readability.score", "readability-score"),
    doc!(
        "rhythm.consecutive-long-sentences",
        "consecutive-long-sentences"
    ),
    doc!("rhythm.repetitive-connectors", "repetitive-connectors"),
    doc!("structure.deep-subordination", "deep-subordination"),
    doc!("structure.deeply-nested-lists", "deeply-nested-lists"),
    doc!("structure.excessive-commas", "excessive-commas"),
    doc!("structure.heading-jump", "heading-jump"),
    doc!("structure.line-length-wide", "line-length-wide"),
    doc!("structure.long-enumeration", "long-enumeration"),
    doc!("structure.mixed-numeric-format", "mixed-numeric-format"),
    doc!("structure.paragraph-too-long", "paragraph-too-long"),
    doc!("structure.sentence-too-long", "sentence-too-long"),
    doc!("syntax.conditional-stacking", "conditional-stacking"),
    doc!("syntax.dense-punctuation-burst", "dense-punctuation-burst"),
    doc!("syntax.nested-negation", "nested-negation"),
    doc!("syntax.passive-voice", "passive-voice"),
    doc!("syntax.unclear-antecedent", "unclear-antecedent"),
];

/// Result of looking up a single rule id.
pub enum Lookup<'a> {
    /// The rule id is known; `body` is the bundled markdown.
    Found {
        /// Echoed id, unchanged from the caller's input.
        id: &'a str,
        /// Raw markdown body bundled at compile time.
        body: &'static str,
    },
    /// The rule id is unknown; `suggestions` lists the closest known ids.
    NotFound {
        /// Echoed id, unchanged from the caller's input.
        id: &'a str,
        /// Up to three nearest known ids (Levenshtein), empty if none.
        suggestions: Vec<&'static str>,
    },
}

/// Look up the bundled doc for a rule id.
#[must_use]
pub fn lookup(id: &str) -> Lookup<'_> {
    if let Some(&(_, _, body)) = RULE_DOCS.iter().find(|(k, _, _)| *k == id) {
        return Lookup::Found { id, body };
    }
    let mut scored: Vec<(usize, &'static str)> = RULE_DOCS
        .iter()
        .map(|(k, _, _)| (levenshtein(id, k), *k))
        .collect();
    scored.sort_by_key(|(d, _)| *d);
    let suggestions = scored.into_iter().take(3).map(|(_, k)| k).collect();
    Lookup::NotFound { id, suggestions }
}

/// Render explanations for one or more rule ids as a single string.
///
/// Relative markdown links are rewritten to absolute mdBook URLs rooted
/// at [`DOCS_BASE`] unless `keep_relative` is set. Each rule doc is
/// followed by a canonical URL footer so readers can share a stable link
/// without grep-ing the body. Unknown ids are printed with suggestions
/// but do not abort the batch. Returns `true` if every id resolved.
#[must_use]
pub fn render_many(ids: &[String], keep_relative: bool) -> (String, bool) {
    use std::fmt::Write as _;

    let mut out = String::new();
    let mut all_found = true;

    for (i, id) in ids.iter().enumerate() {
        if i > 0 {
            out.push('\n');
            out.push_str(&"─".repeat(60));
            out.push_str("\n\n");
        }
        match lookup(id) {
            Lookup::Found { id, body } => {
                let body = strip_mdbook_artifacts(body);
                let body = if keep_relative {
                    body
                } else {
                    rewrite_links(&body)
                };
                out.push_str(&body);
                if !out.ends_with('\n') {
                    out.push('\n');
                }
                let _ = writeln!(out, "\nFull page: {}", canonical_url(id));
            },
            Lookup::NotFound { id, suggestions } => {
                all_found = false;
                let _ = writeln!(out, "Unknown rule id: `{id}`.");
                if suggestions.is_empty() {
                    out.push_str("No known rules.\n");
                } else {
                    out.push_str("Did you mean:\n");
                    for s in suggestions {
                        let _ = writeln!(out, "  - {s}");
                    }
                }
                out.push_str("\nRun `lucid-lint explain --list` to see every known id.\n");
            },
        }
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    (out, all_found)
}

/// Canonical published URL for a rule's doc page.
///
/// Resolves the rule id to its mdBook URL slug (filename stem) and
/// composes an absolute URL rooted at [`DOCS_BASE`]. Unknown ids fall
/// back to using the id itself as the slug, which matches the eventual
/// category-prefixed filenames once the docs tree is rearchitected.
#[must_use]
pub fn canonical_url(rule_id: &str) -> String {
    let slug = RULE_DOCS
        .iter()
        .find_map(|(k, s, _)| (*k == rule_id).then_some(*s))
        .unwrap_or(rule_id);
    format!("{DOCS_BASE}/rules/{slug}.html")
}

/// Rewrite every relative markdown link target in `body` to an absolute
/// mdBook URL. Absolute URLs (http, https, mailto) and pure anchors stay
/// untouched. Rule pages live at `docs/src/rules/<id>.md`, so `./x.md`
/// means a sibling rule page and `../guide/x.md` jumps up one directory.
fn rewrite_links(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut rest = body;
    while let Some(idx) = rest.find("](") {
        out.push_str(&rest[..idx + 2]);
        let after = &rest[idx + 2..];
        let Some(end) = after.find(')') else {
            out.push_str(after);
            return out;
        };
        let target = &after[..end];
        out.push_str(&rewrite_target(target));
        out.push(')');
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

fn rewrite_target(target: &str) -> String {
    // Absolute URLs and pure-anchor links stay as-is.
    if target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
        || target.starts_with('#')
    {
        return target.to_string();
    }

    // Split off an optional `#anchor` suffix.
    let (path, anchor) = target
        .find('#')
        .map_or((target, ""), |i| (&target[..i], &target[i..]));

    // Rule pages live at docs/src/rules/<id>.md; resolve the target path
    // against that directory, then flatten `./` / `../` segments.
    let joined = path.strip_prefix("./").map_or_else(
        || {
            path.strip_prefix("../").map_or_else(
                || {
                    if path.is_empty() {
                        String::new()
                    } else {
                        format!("rules/{path}")
                    }
                },
                ToString::to_string,
            )
        },
        |rest| format!("rules/{rest}"),
    );

    // Rewrite the on-disk `.md` extension to the served `.html`.
    let served = joined
        .strip_suffix(".md")
        .map_or_else(|| joined.clone(), |stem| format!("{stem}.html"));

    format!("{DOCS_BASE}/{served}{anchor}")
}

/// Return the sorted list of bundled rule ids.
#[must_use]
pub fn known_ids() -> Vec<&'static str> {
    RULE_DOCS.iter().map(|(k, _, _)| *k).collect()
}

/// Resolve a rule id to its mdBook URL slug (the `<slug>.html` file name
/// on the published docs site, and the `<slug>.md` stem on disk).
///
/// Returns `None` for unknown ids. Exposed so coverage tests and
/// external tooling can locate the on-disk doc page during the F29-slim
/// transition where rule ids use `category.rule-name` form but doc
/// filenames have not yet been rearchitected.
#[must_use]
pub fn docs_slug(rule_id: &str) -> Option<&'static str> {
    RULE_DOCS
        .iter()
        .find_map(|(k, s, _)| (*k == rule_id).then_some(*s))
}

/// Return every bundled rule id paired with a one-line description.
///
/// The description is the first non-empty paragraph under the rule page's
/// `## What it flags` section, collapsed to a single line and truncated
/// to `max_len` graphemes (UTF-8 safe). If the section cannot be located,
/// the description is an empty string so the caller can still line-align.
#[must_use]
pub fn known_ids_with_descriptions(max_len: usize) -> Vec<(&'static str, String)> {
    RULE_DOCS
        .iter()
        .map(|(id, _, body)| (*id, describe(body, max_len)))
        .collect()
}

fn describe(body: &str, max_len: usize) -> String {
    let mut in_section = false;
    let mut buf = String::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            if in_section {
                break;
            }
            if trimmed.eq_ignore_ascii_case("## What it flags") {
                in_section = true;
            }
            continue;
        }
        if !in_section {
            continue;
        }
        if trimmed.is_empty() {
            if !buf.is_empty() {
                break;
            }
            continue;
        }
        if !buf.is_empty() {
            buf.push(' ');
        }
        buf.push_str(trimmed);
    }
    truncate(&buf, max_len)
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max_len.saturating_sub(1)).collect();
    // Prefer cutting at the last space so words aren't split mid-glyph.
    if let Some(last_space) = out.rfind(' ') {
        out.truncate(last_space);
    }
    out.push('…');
    out
}

/// Remove HTML comments that only exist to silence lucid-lint on its own
/// docs (`<!-- lucid-lint disable-next-line ... -->`). They are meaningful
/// when linting the mdBook source but noise in the terminal view.
fn strip_mdbook_artifacts(body: &str) -> String {
    body.lines()
        .filter(|line| !line.trim_start().starts_with("<!-- lucid-lint "))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Classic Levenshtein edit distance for suggestion ranking.
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr: Vec<usize> = vec![0; b.len() + 1];
    for i in 1..=a.len() {
        curr[0] = i;
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_ids_are_sorted_and_unique() {
        let ids = known_ids();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted, "RULE_DOCS must stay kebab-sorted");
        let dedup: std::collections::BTreeSet<&str> = ids.iter().copied().collect();
        assert_eq!(dedup.len(), ids.len(), "RULE_DOCS has duplicate rule ids");
    }

    #[test]
    fn lookup_finds_known_id() {
        let Lookup::Found { id, body } = lookup("structure.sentence-too-long") else {
            unreachable!("expected Found");
        };
        assert_eq!(id, "structure.sentence-too-long");
        assert!(body.contains("structure.sentence-too-long"));
    }

    #[test]
    fn lookup_suggests_on_typo() {
        let Lookup::NotFound { suggestions, .. } = lookup("sentence-too-logn") else {
            unreachable!("typo should not resolve");
        };
        assert!(
            suggestions.contains(&"structure.sentence-too-long"),
            "expected sentence-too-long in suggestions, got {suggestions:?}"
        );
    }

    #[test]
    fn render_many_emits_separator_between_entries() {
        let ids = vec![
            "structure.sentence-too-long".to_string(),
            "lexicon.weasel-words".to_string(),
        ];
        let (rendered, all_found) = render_many(&ids, false);
        assert!(all_found);
        assert!(rendered.contains("structure.sentence-too-long"));
        assert!(rendered.contains("lexicon.weasel-words"));
        assert!(rendered.contains(&"─".repeat(60)));
    }

    #[test]
    fn render_many_includes_canonical_url_per_rule() {
        let ids = vec!["structure.sentence-too-long".to_string()];
        let (rendered, _) = render_many(&ids, false);
        assert!(rendered.contains(&format!(
            "Full page: {DOCS_BASE}/rules/sentence-too-long.html"
        )));
    }

    #[test]
    fn render_many_rewrites_relative_links_by_default() {
        let ids = vec!["structure.sentence-too-long".to_string()];
        let (rendered, _) = render_many(&ids, false);
        // The page has `[Suppressing diagnostics](../guide/suppression.md)`.
        assert!(
            rendered.contains(&format!("{DOCS_BASE}/guide/suppression.html")),
            "expected relative guide link to be rewritten to an absolute URL"
        );
        assert!(
            !rendered.contains("](../guide/"),
            "no relative `../guide/` link should survive the default rewrite"
        );
    }

    #[test]
    fn render_many_keeps_relative_when_requested() {
        let ids = vec!["structure.sentence-too-long".to_string()];
        let (rendered, _) = render_many(&ids, true);
        assert!(
            rendered.contains("](../guide/suppression.md)"),
            "--keep-relative must preserve the markdown-native link target"
        );
    }

    #[test]
    fn render_many_reports_failure_but_continues() {
        let ids = vec!["bogus-rule".to_string(), "lexicon.weasel-words".to_string()];
        let (rendered, all_found) = render_many(&ids, false);
        assert!(!all_found);
        assert!(rendered.contains("Unknown rule id"));
        assert!(rendered.contains("lexicon.weasel-words"));
    }

    #[test]
    fn rewrite_target_leaves_absolute_urls_alone() {
        assert_eq!(
            rewrite_target("https://example.com/x"),
            "https://example.com/x"
        );
        assert_eq!(rewrite_target("#anchor"), "#anchor");
        assert_eq!(rewrite_target("mailto:a@b.c"), "mailto:a@b.c");
    }

    #[test]
    fn rewrite_target_resolves_sibling_rule_link() {
        assert_eq!(
            rewrite_target("./consecutive-long-sentences.md"),
            format!("{DOCS_BASE}/rules/consecutive-long-sentences.html"),
        );
    }

    #[test]
    fn rewrite_target_resolves_parent_guide_link() {
        assert_eq!(
            rewrite_target("../guide/scoring.md#weights"),
            format!("{DOCS_BASE}/guide/scoring.html#weights"),
        );
    }

    #[test]
    fn every_bundled_rule_has_a_non_empty_description() {
        let entries = known_ids_with_descriptions(120);
        for (id, desc) in entries {
            assert!(
                !desc.is_empty(),
                "rule `{id}` has no extractable `## What it flags` paragraph"
            );
        }
    }

    #[test]
    fn describe_collapses_multiline_paragraph() {
        let body = "# `x`\n\n## What it flags\n\nOne sentence.\nSecond line.\n\n## At a glance\n";
        let d = describe(body, 200);
        assert_eq!(d, "One sentence. Second line.");
    }

    #[test]
    fn describe_truncates_at_word_boundary() {
        let body = "## What it flags\n\nOne two three four five six seven.\n";
        let d = describe(body, 15);
        assert!(d.ends_with('…'));
        assert!(!d.contains("seven"));
    }

    #[test]
    fn strip_mdbook_artifacts_removes_disable_comments() {
        let input = "line one\n<!-- lucid-lint disable-next-line foo -->\nline two";
        let out = strip_mdbook_artifacts(input);
        assert!(!out.contains("lucid-lint disable"));
        assert!(out.contains("line one"));
        assert!(out.contains("line two"));
    }
}
