//! Rule documentation & wiring coverage.
//!
//! Keeps the four surfaces that describe a rule in lock-step:
//!
//! 1. The mdBook page `docs/src/rules/<rule-id>.md` exists and starts with
//!    a level-1 heading matching the rule id.
//! 2. `Category::for_rule` maps the rule id to the same category the page
//!    declares (so the scoring model and the docs cannot drift).
//! 3. `scoring::WEIGHTED_RULE_IDS` lists the rule id, proving weighting
//!    was considered (the uniform-1 fallback is a valid weight, but
//!    forgetting to think about it is not).
//! 4. When `RULE_DOCS_GATE_GIT=1` (set by CI), any rule source file
//!    changed versus `origin/main` must be mentioned in the
//!    `## [Unreleased]` section of `CHANGELOG.md`.
//!
//! Also enforces a cross-cutting mdBook invariant:
//!
//! 5. Every relative link from a `docs/src/**/*.md` page must resolve to
//!    another page under `docs/src/`. `](../../…)` patterns escape the
//!    tree — mdBook does not serve them.
//!
//! Failure messages name the missing artifact so the fix is obvious.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use lucid_lint::explain;
use lucid_lint::rules::default_rules;
use lucid_lint::scoring::{default_weight_for, WEIGHTED_RULE_IDS};
use lucid_lint::{Category, Profile};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rule_ids() -> Vec<String> {
    default_rules(Profile::Public)
        .iter()
        .map(|r| r.id().to_string())
        .collect()
}

fn read_rule_page(rule_id: &str) -> Option<(PathBuf, String)> {
    // Rule ids use the `category.rule-name` form (F29-slim); doc filenames
    // currently still use the flat kebab slug. The mapping lives in
    // `explain::docs_slug`.
    let slug = explain::docs_slug(rule_id)?;
    let path = workspace_root()
        .join("docs/src/rules")
        .join(format!("{slug}.md"));
    let body = fs::read_to_string(&path).ok()?;
    Some((path, body))
}

/// Every rule listed in `default_rules` has a mdBook page whose H1 is the rule id.
#[test]
fn every_rule_has_a_docs_page() {
    let mut missing = Vec::new();
    let mut mismatched = Vec::new();

    for id in rule_ids() {
        match read_rule_page(&id) {
            None => missing.push(id.clone()),
            Some((path, body)) => {
                let first_heading = body
                    .lines()
                    .find(|l| l.starts_with("# "))
                    .map(|l| l.trim_start_matches('#').trim().to_string());
                let expected = format!("`{id}`");
                if first_heading.as_deref() != Some(expected.as_str()) {
                    mismatched.push(format!(
                        "{}: expected H1 `{expected}`, got `{:?}`",
                        path.display(),
                        first_heading
                    ));
                }
            },
        }
    }

    assert!(
        missing.is_empty() && mismatched.is_empty(),
        "rule documentation pages out of sync:\n  missing pages: {missing:?}\n  mismatched H1: {mismatched:?}\n\
         Fix: create docs/src/rules/<rule-id>.md (H1 `\\`<rule-id>\\``) and wire it into docs/src/SUMMARY.md."
    );
}

/// Every shipped rule maps to a specific category — never the fallback branch.
#[test]
fn every_rule_is_categorized() {
    // Build the set of categories each page declares via its "| **Category** | `x` |"
    // table row, and assert it matches Category::for_rule.
    let mut drift = Vec::new();
    let mut unmapped = Vec::new();

    for id in rule_ids() {
        let code_category = Category::for_rule(&id);
        // Sanity: the fallback is Syntax, so flag any rule that lands there
        // *unless* it is one of the two rules we expect to be there.
        if matches!(code_category, Category::Syntax)
            && !matches!(
                id.as_str(),
                "syntax.passive-voice"
                    | "syntax.unclear-antecedent"
                    | "syntax.nested-negation"
                    | "syntax.conditional-stacking"
                    | "syntax.dense-punctuation-burst"
            )
        {
            unmapped.push(id.clone());
        }

        let Some((_, body)) = read_rule_page(&id) else {
            continue; // Handled by every_rule_has_a_docs_page.
        };

        let declared = body
            .lines()
            .find(|l| l.contains("**Category**"))
            .and_then(extract_backticked);

        let code_category_str = code_category.to_string();
        if declared.as_deref() != Some(code_category_str.as_str()) {
            drift.push(format!(
                "{id}: page declares `{}`, Category::for_rule says `{}`",
                declared.as_deref().unwrap_or("<missing>"),
                code_category_str
            ));
        }
    }

    assert!(
        unmapped.is_empty(),
        "rule ids falling through to the Syntax fallback in Category::for_rule: {unmapped:?}\n\
         Fix: add an explicit arm in src/types.rs Category::for_rule."
    );
    assert!(
        drift.is_empty(),
        "category drift between docs and code:\n  {}\n\
         Fix: update the `**Category**` row in the per-rule page or the arm in Category::for_rule.",
        drift.join("\n  ")
    );
}

/// Every shipped rule appears in `WEIGHTED_RULE_IDS`.
#[test]
fn every_rule_has_a_default_weight_registered() {
    let registered: BTreeSet<&str> = WEIGHTED_RULE_IDS.iter().copied().collect();
    let mut missing = Vec::new();

    for id in rule_ids() {
        // Touch default_weight_for to guarantee the mapping still compiles
        // and returns a non-zero weight for every rule.
        let w = default_weight_for(&id);
        assert!(w >= 1, "default_weight_for({id}) returned 0");
        if !registered.contains(id.as_str()) {
            missing.push(id);
        }
    }

    assert!(
        missing.is_empty(),
        "rules missing from scoring::WEIGHTED_RULE_IDS: {missing:?}\n\
         Fix: append the id to the const in src/scoring.rs (even if the intended weight is the uniform 1)."
    );
}

/// Every shipped rule has a bundled doc page reachable from `lucid-lint explain`.
#[test]
fn every_rule_has_a_bundled_doc() {
    let bundled: BTreeSet<&str> = explain::known_ids().into_iter().collect();
    let mut missing = Vec::new();
    for id in rule_ids() {
        if !bundled.contains(id.as_str()) {
            missing.push(id);
        }
    }
    assert!(
        missing.is_empty(),
        "rules missing from src/explain.rs RULE_DOCS: {missing:?}\n\
         Fix: append a `doc!(\"<rule-id>\")` line in kebab-sorted order in src/explain.rs."
    );
}

/// When `RULE_DOCS_GATE_GIT=1` (CI opt-in), any modified rule source must
/// appear in the `CHANGELOG.md` Unreleased section.
#[test]
fn changed_rules_appear_in_changelog_unreleased() {
    if std::env::var_os("RULE_DOCS_GATE_GIT").is_none() {
        eprintln!("skipping: RULE_DOCS_GATE_GIT not set");
        return;
    }

    let base = std::env::var("RULE_DOCS_GATE_BASE").unwrap_or_else(|_| "origin/main".to_string());

    let output = Command::new("git")
        .args([
            "diff",
            "--name-only",
            &format!("{base}...HEAD"),
            "--",
            "src/rules/",
        ])
        .current_dir(workspace_root())
        .output()
        .expect("git diff failed to execute");

    assert!(
        output.status.success(),
        "git diff returned non-zero: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    let changed: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|path| {
            let p = Path::new(path);
            // Only Rust sources are rules. Insta snapshot files
            // (`*.snap` under `snapshots/`) and other non-source
            // changes in the rules tree must not be parsed as rule
            // ids — the file stem of a snapshot has the shape
            // `lucid_lint__rules__structure__<rule>__tests__<test>`,
            // which would otherwise produce nonsensical "rule ids"
            // like `lucid-lint--rules--structure--…`.
            if p.extension().and_then(|e| e.to_str()) != Some("rs") {
                return None;
            }
            // Skip files inside any `snapshots/` directory (defensive:
            // covers test-side fixtures that happen to live as `.rs`
            // alongside snapshots).
            if p.components().any(|c| c.as_os_str() == "snapshots") {
                return None;
            }
            let stem = p.file_stem()?.to_str()?;
            if stem == "mod" || stem == "enumeration" {
                return None;
            }
            Some(stem.replace('_', "-"))
        })
        .collect();

    if changed.is_empty() {
        return;
    }

    let changelog =
        fs::read_to_string(workspace_root().join("CHANGELOG.md")).expect("CHANGELOG.md missing");
    // Accept mentions in `[Unreleased]` OR the topmost dated release
    // section. On main the next-release home is `[Unreleased]`; on a
    // release branch we rename it to `[X.Y.Z] — DATE` and add a fresh
    // empty `[Unreleased]` above it. Both surfaces are legitimate
    // documentation homes during the release window.
    let documented = extract_release_window(&changelog)
        .expect("CHANGELOG.md has no `## [Unreleased]` or dated release section");

    let mut missing = Vec::new();
    for id in &changed {
        if !documented.contains(id.as_str()) {
            missing.push(id.clone());
        }
    }

    assert!(
        missing.is_empty(),
        "rules touched without a CHANGELOG Unreleased / current-release mention: {missing:?}\n\
         Fix: add a line under `## [Unreleased]` (or the topmost dated section on a release branch) in CHANGELOG.md naming each rule id."
    );
}

/// Every relative link inside `docs/src/` must resolve inside `docs/src/`.
///
/// mdBook only serves pages under its `src/` tree. A link escapes when
/// its `../` chain pops above `docs/src/` — which depends on the
/// source page's depth, not a fixed `../` count (the FR tree adds a
/// third level at `docs/src/fr/<section>/<page>.md`). For deliberate
/// "see the repo file" references use an absolute `https://github.com/…`
/// URL instead.
#[test]
fn docs_links_stay_inside_docs() {
    let root = workspace_root().join("docs/src");
    let mut offenders = Vec::new();
    walk_markdown(&root, &mut |path, body| {
        // Depth of the file's parent directory relative to docs/src/.
        // `docs/src/introduction.md` → depth 0
        // `docs/src/rules/sentence-too-long.md` → depth 1
        // `docs/src/fr/rules/sentence-too-long.md` → depth 2
        let Some(parent) = path.parent() else {
            return;
        };
        let Ok(rel_parent) = parent.strip_prefix(&root) else {
            return;
        };
        let depth = rel_parent.components().count();

        for (lineno, line) in body.lines().enumerate() {
            // Only inspect link targets — patterns of the form `](...)`.
            let mut rest = line;
            while let Some(idx) = rest.find("](") {
                let after = &rest[idx + 2..];
                let Some(end) = after.find(')') else {
                    break;
                };
                let target = &after[..end];
                rest = &after[end..];

                // Count leading `../` segments; skip anything else
                // (`./`, a bare filename, `http(s)://`, `#anchor`, …).
                let mut remainder = target;
                let mut ups = 0usize;
                while let Some(rest) = remainder.strip_prefix("../") {
                    ups += 1;
                    remainder = rest;
                }
                if ups == 0 || ups <= depth {
                    continue;
                }
                offenders.push(format!(
                    "{}:{}: link target `{target}` escapes docs/src/",
                    path.strip_prefix(&root).unwrap_or(path).display(),
                    lineno + 1,
                ));
            }
        }
    });
    assert!(
        offenders.is_empty(),
        "docs/src/ pages contain relative links that escape the mdBook tree:\n  {}\n\n\
         Fix: point the link at a page under docs/src/ (create one if missing), or use an absolute https://github.com/… URL for a deliberate repo-file reference. See AGENTS.md directive 9.",
        offenders.join("\n  ")
    );
}

fn walk_markdown(root: &Path, visit: &mut dyn FnMut(&Path, &str)) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_markdown(&path, visit);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            if let Ok(body) = fs::read_to_string(&path) {
                visit(&path, &body);
            }
        }
    }
}

fn extract_backticked(line: &str) -> Option<String> {
    let mut parts = line.splitn(3, '`');
    parts.next()?; // before first backtick
    let inside = parts.next()?; // between first and second
    Some(inside.to_string())
}

/// Extract the contiguous "release window" at the top of the changelog:
/// the `[Unreleased]` section plus the first dated `[X.Y.Z] — date`
/// section that follows it. Stops at the second dated section.
///
/// On main this collapses to `[Unreleased]` only (the dated section is
/// already shipped, but accepting it costs nothing and keeps the rule
/// from tripping if a hot-fix mentions a rule already in the most
/// recent release notes). On a release branch, where the entries have
/// been renamed from `[Unreleased]` to `[X.Y.Z] — date` and a fresh
/// empty `[Unreleased]` sits above, this captures both — so rules
/// documented in the named release still satisfy the gate.
fn extract_release_window(changelog: &str) -> Option<String> {
    let mut out = String::new();
    let mut in_window = false;
    let mut dated_seen = 0usize;
    for line in changelog.lines() {
        if line.starts_with("## ") {
            let header = line.to_ascii_lowercase();
            let is_unreleased = header.contains("unreleased");
            let is_dated = !is_unreleased && header.contains('[');
            if is_unreleased {
                in_window = true;
                continue;
            }
            if is_dated {
                dated_seen += 1;
                // Accept the FIRST dated section that follows
                // `[Unreleased]`; stop at the second.
                if dated_seen >= 2 {
                    break;
                }
                in_window = true;
                continue;
            }
            // Some other `## ` heading — leave window unchanged.
            continue;
        }
        if in_window {
            out.push_str(line);
            out.push('\n');
        }
    }
    if in_window || !out.is_empty() {
        Some(out)
    } else {
        None
    }
}
