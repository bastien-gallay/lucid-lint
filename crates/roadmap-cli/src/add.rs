//! `add` subcommand: scaffold a new feature file from a template.
//!
//! Mitigates pre-mortem F2 (friction regression): one command beats
//! hand-crafting frontmatter every time.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlugShape {
    /// `f-<kebab-name>` — the canonical shape for new features.
    KebabPrefixed,
    /// `f<digits>` — the legacy numeric form. Only accepted at `add`
    /// time when an explicit migration flag is set.
    LegacyNumeric,
}

pub fn classify_slug(slug: &str) -> Result<SlugShape> {
    if let Some(rest) = slug.strip_prefix("f-") {
        if rest.is_empty() {
            bail!("slug `{slug}` has empty body after `f-`");
        }
        if !rest
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            bail!("slug `{slug}` must match `^f-[a-z0-9-]+$` (lowercase kebab)");
        }
        if rest.starts_with('-') || rest.ends_with('-') || rest.contains("--") {
            bail!("slug `{slug}` must not have leading, trailing, or double hyphens after `f-`");
        }
        return Ok(SlugShape::KebabPrefixed);
    }
    if let Some(rest) = slug.strip_prefix('f') {
        if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
            return Ok(SlugShape::LegacyNumeric);
        }
    }
    bail!("slug `{slug}` did not match `^f-[a-z0-9-]+$` or `^f\\d+$`")
}

/// Outcome of `add`. `legacy_numeric_warning` lets the CLI emit the
/// deprecation warning without the lib forcing a stderr write.
#[derive(Debug)]
pub struct AddOutcome {
    pub path: PathBuf,
    pub legacy_numeric_warning: bool,
}

pub fn add(root: &Path, slug: &str, allow_legacy_numeric: bool) -> Result<AddOutcome> {
    let shape = classify_slug(slug)?;
    if shape == SlugShape::LegacyNumeric && !allow_legacy_numeric {
        bail!(
            "slug `{slug}` is the legacy numeric form (`f<digits>`). New features must use \
             `f-<kebab-name>`. If this is part of a one-shot migration, pass \
             `--allow-legacy-numeric`."
        );
    }

    let features_dir = root.join("features");
    std::fs::create_dir_all(&features_dir)
        .with_context(|| format!("creating {}", features_dir.display()))?;

    let path = features_dir.join(format!("{slug}.md"));
    if path.exists() {
        bail!("refusing to overwrite existing file: {}", path.display());
    }

    let id = derive_id(slug);
    std::fs::write(&path, render_template(&id))
        .with_context(|| format!("writing {}", path.display()))?;

    Ok(AddOutcome {
        path,
        legacy_numeric_warning: shape == SlugShape::LegacyNumeric,
    })
}

/// Capitalize the leading `f` of the slug to produce the canonical id.
/// `f-foo` → `F-foo`, `f139` → `F139`.
fn derive_id(slug: &str) -> String {
    let mut chars = slug.chars();
    match chars.next() {
        Some(_) => format!("F{}", chars.as_str()),
        None => String::new(),
    }
}

fn render_template(id: &str) -> String {
    format!(
        r#"+++
id = "{id}"
topic = "<TODO>"
status = "todo"
priority = "next"
target = ["<TODO>"]
+++

<TODO: one-paragraph summary — first non-empty line becomes the catalog row's Summary column.>
"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn unique_tmp(label: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        std::env::temp_dir().join(format!("roadmap-cli-add-{label}-{pid}-{n}"))
    }

    #[test]
    fn kebab_slugs_classify() {
        assert_eq!(classify_slug("f-foo").unwrap(), SlugShape::KebabPrefixed);
        assert_eq!(
            classify_slug("f-foo-bar-baz").unwrap(),
            SlugShape::KebabPrefixed
        );
        assert_eq!(classify_slug("f-foo123").unwrap(), SlugShape::KebabPrefixed);
    }

    #[test]
    fn legacy_slugs_classify() {
        assert_eq!(classify_slug("f139").unwrap(), SlugShape::LegacyNumeric);
        assert_eq!(classify_slug("f1").unwrap(), SlugShape::LegacyNumeric);
    }

    #[test]
    fn rejects_uppercase() {
        assert!(classify_slug("F-foo").is_err());
        assert!(classify_slug("F139").is_err());
    }

    #[test]
    fn rejects_no_prefix() {
        assert!(classify_slug("foo-bar").is_err());
        assert!(classify_slug("xyz").is_err());
        assert!(classify_slug("").is_err());
    }

    #[test]
    fn rejects_malformed_kebab() {
        for bad in ["f-", "f--foo", "f-foo-", "f-foo--bar", "f-foo_bar", "f-FOO"] {
            assert!(classify_slug(bad).is_err(), "expected reject: {bad}");
        }
    }

    #[test]
    fn derive_id_capitalizes() {
        assert_eq!(derive_id("f-foo"), "F-foo");
        assert_eq!(derive_id("f139"), "F139");
    }

    #[test]
    fn add_writes_template_at_expected_path() {
        let root = unique_tmp("ok");
        let out = add(&root, "f-new-thing", false).unwrap();
        assert!(out.path.ends_with("features/f-new-thing.md"));
        assert!(!out.legacy_numeric_warning);
        let body = std::fs::read_to_string(&out.path).unwrap();
        assert!(body.contains(r#"id = "F-new-thing""#));
        assert!(body.contains(r#"status = "todo""#));
        assert!(body.contains("<TODO"));
    }

    #[test]
    fn add_refuses_clobber() {
        let root = unique_tmp("clobber");
        add(&root, "f-twice", false).unwrap();
        let err = add(&root, "f-twice", false).unwrap_err();
        assert!(format!("{err:#}").contains("refusing to overwrite"));
    }

    #[test]
    fn add_rejects_legacy_without_flag() {
        let root = unique_tmp("legacy-no-flag");
        let err = add(&root, "f139", false).unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("legacy numeric"));
        assert!(msg.contains("--allow-legacy-numeric"));
    }

    #[test]
    fn add_accepts_legacy_with_flag_and_signals_warning() {
        let root = unique_tmp("legacy-flag");
        let out = add(&root, "f200", true).unwrap();
        assert!(out.legacy_numeric_warning);
        let body = std::fs::read_to_string(&out.path).unwrap();
        assert!(body.contains(r#"id = "F200""#));
    }
}
