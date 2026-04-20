//! Rule: `heading-jump`.
//!
//! Flags broken heading hierarchies — each heading must follow the
//! previous one by at most +1 level. A reader navigating by headings
//! loses the mental map of the document when levels are skipped.
//!
//! See [`RULES.md`](../../RULES.md#heading-jump) for the rule's
//! rationale and references.

use crate::config::Profile;
use crate::parser::Document;
use crate::rules::Rule;
use crate::types::{Diagnostic, Language, Location, Severity, SourceFile};

/// Configuration for [`HeadingJump`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Whether the first heading is allowed to start at any level.
    /// When `false`, the first heading must be H1.
    pub allow_first_heading_any_level: bool,

    /// Whether the document must contain an H1.
    pub require_h1: bool,
}

impl Config {
    /// Build a config from a profile preset.
    #[must_use]
    pub const fn for_profile(_profile: Profile) -> Self {
        // The rule applies equally across profiles; thresholds are booleans,
        // not graduated. Defaults match RULES.md.
        Self {
            allow_first_heading_any_level: true,
            require_h1: false,
        }
    }
}

/// The [`HeadingJump`] rule.
#[derive(Debug, Clone, Copy)]
pub struct HeadingJump {
    config: Config,
}

impl HeadingJump {
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
    pub const ID: &'static str = "heading-jump";
}

impl Rule for HeadingJump {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn check(&self, document: &Document, _language: Language) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut previous_depth: Option<u32> = None;
        let mut seen_any_heading = false;
        let mut seen_h1 = false;

        for section in &document.sections {
            let Some(line) = section.heading_line else {
                continue;
            };
            let depth = section.depth;
            if depth == 1 {
                seen_h1 = true;
            }

            match previous_depth {
                None => {
                    if !self.config.allow_first_heading_any_level && depth > 1 {
                        diagnostics.push(build_diagnostic(
                            &document.source,
                            line,
                            section.title.as_deref(),
                            format!(
                                "First heading is H{depth}, not H1. Start the document with an \
                                 H1 heading before any lower-level heading."
                            ),
                        ));
                    }
                },
                Some(prev) => {
                    if depth > prev + 1 {
                        diagnostics.push(build_diagnostic(
                            &document.source,
                            line,
                            section.title.as_deref(),
                            format!(
                                "Heading jumps from H{prev} to H{depth}. Use H{} instead so the \
                                 hierarchy grows by one level at a time.",
                                prev + 1
                            ),
                        ));
                    }
                },
            }
            previous_depth = Some(depth);
            seen_any_heading = true;
        }

        if self.config.require_h1 && seen_any_heading && !seen_h1 {
            diagnostics.push(build_diagnostic(
                &document.source,
                1,
                None,
                "Document has no H1 heading. Add an H1 at the top to anchor the outline."
                    .to_string(),
            ));
        }

        diagnostics
    }
}

fn build_diagnostic(
    source: &SourceFile,
    line: u32,
    title: Option<&str>,
    message: String,
) -> Diagnostic {
    let length = title.map_or(1, |t| u32::try_from(t.chars().count()).unwrap_or(u32::MAX));
    let location = Location::new(source.clone(), line, 1, length);
    let diag = Diagnostic::new(HeadingJump::ID, Severity::Warning, location, message);
    match title {
        Some(t) => diag.with_section(t),
        None => diag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_markdown;
    use crate::types::SourceFile;

    fn lint(md: &str) -> Vec<Diagnostic> {
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let rule = HeadingJump::for_profile(Profile::Public);
        rule.check(&doc, Language::En)
    }

    #[test]
    fn id_is_kebab_case() {
        assert_eq!(HeadingJump::ID, "heading-jump");
    }

    #[test]
    fn well_formed_hierarchy_does_not_trigger() {
        let md = "# Top\n\n## Sub\n\n### Deep\n\n## Another\n";
        assert!(lint(md).is_empty());
    }

    #[test]
    fn jump_h2_to_h4_triggers() {
        let md = "# Top\n\n## Sub\n\n#### Skipped\n";
        let diags = lint(md);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("H2 to H4"));
    }

    #[test]
    fn jump_at_start_h1_to_h3_triggers() {
        let md = "# Top\n\n### Jumped\n";
        let diags = lint(md);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("H1 to H3"));
    }

    #[test]
    fn same_level_siblings_do_not_trigger() {
        let md = "## First\n\n## Second\n\n## Third\n";
        assert!(lint(md).is_empty());
    }

    #[test]
    fn climbing_back_up_does_not_trigger() {
        // Coming back up any number of levels is fine.
        let md = "# Top\n\n## Sub\n\n### Deep\n\n# Back to top\n";
        assert!(lint(md).is_empty());
    }

    #[test]
    fn first_heading_not_h1_is_allowed_by_default() {
        let md = "## Starts at H2\n\n### Deeper\n";
        assert!(lint(md).is_empty());
    }

    #[test]
    fn require_h1_flags_missing_h1() {
        let md = "## No H1\n\n### Deeper\n";
        let cfg = Config {
            allow_first_heading_any_level: true,
            require_h1: true,
        };
        let doc = parse_markdown(md, SourceFile::Anonymous);
        let diags = HeadingJump::new(cfg).check(&doc, Language::En);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("no H1"));
    }

    #[test]
    fn category_is_structure() {
        let md = "# Top\n\n#### Jumped\n";
        let diags = lint(md);
        assert_eq!(diags[0].category(), crate::types::Category::Structure);
    }

    #[test]
    fn diagnostic_points_at_offending_heading_line() {
        let md = "# Top\n\n## Sub\n\n#### Skipped\n";
        let diags = lint(md);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].location.line, 5);
    }

    #[test]
    fn snapshot_fixture() {
        let md = "# Top\n\n## Sub\n\n#### Skipped\n\n### Back\n";
        let diags = lint(md);
        insta::assert_yaml_snapshot!(diags, {
            ".*.location.file" => "<input>",
        });
    }
}
