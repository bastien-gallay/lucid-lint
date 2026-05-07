//! `validate` subcommand: schema, slug uniqueness, anchor drift.
//!
//! Pure read-only — never mutates the source tree. Collects all
//! issues into a `ValidationReport` instead of bailing on the first
//! parse error, so a single run surfaces every problem.

use crate::{load_config, parse_feature, render, sort_features};
use anyhow::{bail, Context, Result};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct ValidationReport {
    /// `.roadmap/` source tree is absent on this checkout (e.g. CI, or a
    /// fresh main worktree before A9 migration). Skipped, not failed.
    pub source_missing: bool,
    pub schema_errors: Vec<SchemaError>,
    pub duplicate_ids: Vec<String>,
    pub anchor_collisions: Vec<AnchorCollision>,
    /// Anchors present in `ROADMAP.md` but absent from a fresh regen
    /// — inbound links from EN/FR docs would 404 after the next regen.
    pub anchors_missing_from_regen: Vec<String>,
    /// Anchors present in regen but absent from on-disk `ROADMAP.md`
    /// — release-prep regen never ran (or wasn't committed).
    pub anchors_missing_from_disk: Vec<String>,
}

#[derive(Debug)]
pub struct SchemaError {
    pub path: PathBuf,
    pub message: String,
}

#[derive(Debug)]
pub struct AnchorCollision {
    pub anchor: String,
    pub ids: Vec<String>,
}

impl ValidationReport {
    pub fn is_clean(&self) -> bool {
        self.source_missing
            || (self.schema_errors.is_empty()
                && self.duplicate_ids.is_empty()
                && self.anchor_collisions.is_empty()
                && !self.has_drift())
    }

    pub fn has_drift(&self) -> bool {
        !self.anchors_missing_from_regen.is_empty() || !self.anchors_missing_from_disk.is_empty()
    }

    pub fn has_hard_errors(&self) -> bool {
        !self.schema_errors.is_empty()
            || !self.duplicate_ids.is_empty()
            || !self.anchor_collisions.is_empty()
    }

    pub fn to_text(&self) -> String {
        use std::fmt::Write;
        let mut out = String::new();
        if self.source_missing {
            out.push_str("validate: skipped (no `.roadmap/` source on this checkout)\n");
            return out;
        }
        if self.is_clean() {
            out.push_str("validate: clean\n");
            return out;
        }
        if !self.schema_errors.is_empty() {
            let _ = writeln!(out, "schema errors ({}):", self.schema_errors.len());
            for e in &self.schema_errors {
                let _ = writeln!(out, "  {}: {}", e.path.display(), e.message);
            }
        }
        if !self.duplicate_ids.is_empty() {
            let _ = writeln!(out, "duplicate ids ({}):", self.duplicate_ids.len());
            for id in &self.duplicate_ids {
                let _ = writeln!(out, "  {id}");
            }
        }
        if !self.anchor_collisions.is_empty() {
            let _ = writeln!(out, "anchor collisions ({}):", self.anchor_collisions.len());
            for c in &self.anchor_collisions {
                let _ = writeln!(out, "  anchor `{}` ← ids {:?}", c.anchor, c.ids);
            }
        }
        if !self.anchors_missing_from_regen.is_empty() {
            let _ = writeln!(
                out,
                "anchors on disk but not in regen ({}) — broken inbound links after regen:",
                self.anchors_missing_from_regen.len()
            );
            for a in &self.anchors_missing_from_regen {
                let _ = writeln!(out, "  {a}");
            }
        }
        if !self.anchors_missing_from_disk.is_empty() {
            let _ = writeln!(
                out,
                "anchors in regen but not on disk ({}) — `ROADMAP.md` needs regen:",
                self.anchors_missing_from_disk.len()
            );
            for a in &self.anchors_missing_from_disk {
                let _ = writeln!(out, "  {a}");
            }
        }
        out
    }
}

pub fn validate(root: &Path, roadmap_md: &Path) -> Result<ValidationReport> {
    let mut report = ValidationReport::default();

    let features_dir = root.join("features");
    if !features_dir.is_dir() {
        // No source on this checkout — silent-pass. Lets the recipe
        // run on main (where `.roadmap/` is gitignored) and in CI
        // without manufacturing an error.
        report.source_missing = true;
        return Ok(report);
    }

    let mut features = Vec::new();
    for entry in walkdir::WalkDir::new(&features_dir)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
    {
        let entry = entry.context("walking features dir")?;
        let path = entry.path();
        if !(entry.file_type().is_file() && path.extension().is_some_and(|e| e == "md")) {
            continue;
        }
        match std::fs::read_to_string(path) {
            Ok(src) => match parse_feature(&src) {
                Ok(f) => features.push(f),
                Err(e) => report.schema_errors.push(SchemaError {
                    path: path.to_path_buf(),
                    message: format!("{e:#}"),
                }),
            },
            Err(e) => report.schema_errors.push(SchemaError {
                path: path.to_path_buf(),
                message: format!("read failed: {e}"),
            }),
        }
    }

    let mut id_counts: HashMap<String, usize> = HashMap::new();
    for f in &features {
        *id_counts.entry(f.frontmatter.id.clone()).or_default() += 1;
    }
    for (id, n) in &id_counts {
        if *n > 1 {
            report.duplicate_ids.push(id.clone());
        }
    }
    report.duplicate_ids.sort();

    let mut anchor_to_ids: HashMap<String, BTreeSet<String>> = HashMap::new();
    for f in &features {
        anchor_to_ids
            .entry(f.frontmatter.id.to_lowercase())
            .or_default()
            .insert(f.frontmatter.id.clone());
    }
    for (anchor, ids) in anchor_to_ids {
        if ids.len() > 1 {
            report.anchor_collisions.push(AnchorCollision {
                anchor,
                ids: ids.into_iter().collect(),
            });
        }
    }
    report
        .anchor_collisions
        .sort_by(|a, b| a.anchor.cmp(&b.anchor));

    if !roadmap_md.is_file() {
        bail!("ROADMAP.md not found at: {}", roadmap_md.display());
    }
    let on_disk = std::fs::read_to_string(roadmap_md)
        .with_context(|| format!("reading {}", roadmap_md.display()))?;
    let on_disk_anchors = extract_anchors(&on_disk);

    let config = load_config(root).context("loading config.toml")?;
    let mut sorted = features;
    sort_features(&mut sorted, &config);
    let regen = render(&sorted);
    let regen_anchors = extract_anchors(&regen);

    report.anchors_missing_from_regen = on_disk_anchors
        .difference(&regen_anchors)
        .cloned()
        .collect();
    report.anchors_missing_from_disk = regen_anchors
        .difference(&on_disk_anchors)
        .cloned()
        .collect();

    Ok(report)
}

/// Extract the contents of every `<a id="…">` in markdown.
/// Manual scanner — the shape is fixed and narrow, doesn't justify a regex dep.
pub fn extract_anchors(md: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let needle = "<a id=\"";
    let mut rest = md;
    while let Some(start) = rest.find(needle) {
        let after = &rest[start + needle.len()..];
        match after.find('"') {
            Some(end) => {
                out.insert(after[..end].to_string());
                rest = &after[end + 1..];
            },
            None => break,
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_anchors_basic() {
        let md = r#"<a id="f22"></a> ... <a id="f-foo"></a> ..."#;
        let got = extract_anchors(md);
        let want: BTreeSet<String> = ["f22", "f-foo"].iter().map(|s| s.to_string()).collect();
        assert_eq!(got, want);
    }

    #[test]
    fn extract_anchors_ignores_other_html() {
        let md = r##"<div id="x"></div> <a href="#y">z</a> <a id="ok"></a>"##;
        let got = extract_anchors(md);
        let want: BTreeSet<String> = ["ok"].iter().map(|s| s.to_string()).collect();
        assert_eq!(got, want);
    }

    #[test]
    fn extract_anchors_unterminated_is_safe() {
        let md = r#"<a id="oops"#;
        assert!(extract_anchors(md).is_empty());
    }

    #[test]
    fn report_clean_when_empty() {
        let r = ValidationReport::default();
        assert!(r.is_clean());
        assert!(!r.has_drift());
        assert!(!r.has_hard_errors());
    }

    #[test]
    fn validate_skips_when_source_missing() {
        // Pointing `root` at any non-existent `features/` parent should
        // silent-pass — the recipe runs on main worktrees too.
        let tmp = std::env::temp_dir().join("roadmap-cli-skip-missing");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let roadmap_md = tmp.join("ROADMAP.md");
        std::fs::write(&roadmap_md, "").unwrap();
        let r = validate(&tmp, &roadmap_md).unwrap();
        assert!(r.source_missing);
        assert!(r.is_clean());
        assert!(r.to_text().contains("skipped"));
    }

    #[test]
    fn report_drift_only() {
        let mut r = ValidationReport::default();
        r.anchors_missing_from_disk.push("f-new".into());
        assert!(!r.is_clean());
        assert!(r.has_drift());
        assert!(!r.has_hard_errors());
    }
}
