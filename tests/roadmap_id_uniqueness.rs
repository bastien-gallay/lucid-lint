//! F-roadmap-slug-ids: enforce slug-as-ID uniqueness in ROADMAP + CHANGELOG.
//!
//! Three invariants:
//! 1. Every `F-<slug>` definition site (`<a id="f-<slug>"></a>`) appears
//!    at most once per source file.
//! 2. Slugs are kebab-case and start with an ASCII letter — guarantees
//!    the slug body cannot collide with the legacy `F<number>` namespace.
//! 3. Every reference `[F-<slug>](#f-<slug>)` matches text-to-anchor and
//!    resolves to a definition site in the same file.

#![allow(clippy::panic)] // Test panics on I/O / convention violations are intentional.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

const SOURCES: &[&str] = &["ROADMAP.md", "CHANGELOG.md"];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(name: &str) -> String {
    let path = workspace_root().join(name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// Strip code spans and fenced code blocks from the body so documentation
/// prose mentions don't trip the substring matchers — only real markdown
/// anchors and links should.
///
/// Per-line scope:
/// - A line whose first non-whitespace characters are three backticks
///   toggles fenced-block state; lines inside a fenced block are dropped.
/// - On non-fenced lines, single-backtick spans are stripped (toggle on
///   each backtick encountered).
fn strip_code(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut in_fence = false;
    for line in body.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            out.push('\n');
            continue;
        }
        if in_fence {
            out.push('\n');
            continue;
        }
        let mut in_span = false;
        for c in line.chars() {
            if c == '`' {
                in_span = !in_span;
                continue;
            }
            if !in_span {
                out.push(c);
            }
        }
        out.push('\n');
    }
    out
}

/// Pull every `<a id="f-..."></a>` slug-anchor body from the file.
fn slug_definitions(body: &str) -> Vec<String> {
    let needle = "<a id=\"f-";
    body.match_indices(needle)
        .filter_map(|(i, _)| {
            let after = &body[i + needle.len()..];
            after.find('"').map(|end| after[..end].to_string())
        })
        .collect()
}

/// Pull every `[F-...](#f-...)` reference; returns `(text_slug, anchor_slug)` pairs.
fn slug_references(body: &str) -> Vec<(String, String)> {
    let needle = "[F-";
    body.match_indices(needle)
        .filter_map(|(i, _)| {
            let after = &body[i + needle.len()..];
            let text_end = after.find(']')?;
            let rest = &after[text_end + 1..];
            let anchor_part = rest.strip_prefix("(#f-")?;
            let anchor_end = anchor_part.find(')')?;
            Some((
                after[..text_end].to_lowercase(),
                anchor_part[..anchor_end].to_string(),
            ))
        })
        .collect()
}

#[test]
fn slug_definitions_are_unique_per_file() {
    for source in SOURCES {
        let body = strip_code(&read(source));
        let mut counts: HashMap<String, usize> = HashMap::new();
        for slug in slug_definitions(&body) {
            *counts.entry(slug).or_insert(0) += 1;
        }
        let dupes: Vec<_> = counts.iter().filter(|(_, n)| **n > 1).collect();
        assert!(
            dupes.is_empty(),
            "duplicate F-slug definition sites in {source}: {dupes:?}",
        );
    }
}

#[test]
fn slugs_use_kebab_form_starting_with_letter() {
    // Starting with an ASCII letter prevents collision with the legacy
    // `F<number>` namespace (`F-143` would shadow `F143`).
    for source in SOURCES {
        let body = strip_code(&read(source));
        for slug in slug_definitions(&body) {
            let first = slug.chars().next().unwrap_or('-');
            assert!(
                first.is_ascii_lowercase(),
                "F-slug `{slug}` in {source} must start with an ASCII letter (kebab-case)",
            );
            assert!(
                slug.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "F-slug `{slug}` in {source} contains non-kebab characters (allowed: a-z, 0-9, hyphen)",
            );
        }
    }
}

#[test]
fn slug_references_resolve_to_local_definitions() {
    for source in SOURCES {
        let body = strip_code(&read(source));
        let defs: HashSet<String> = slug_definitions(&body).into_iter().collect();
        for (text, anchor) in slug_references(&body) {
            assert_eq!(
                text, anchor,
                "in {source}: `[F-{text}]` text does not match `#f-{anchor}` anchor",
            );
            assert!(
                defs.contains(&anchor),
                "in {source}: `[F-{text}](#f-{anchor})` has no `<a id=\"f-{anchor}\"></a>` definition site",
            );
        }
    }
}
