//! SARIF v2.1.0 output for GitHub Code Scanning and compatible consumers.
//!
//! The emitted document is intentionally minimal: a single `run` describing
//! the `lucid-lint` driver, one rule descriptor per rule id observed in the
//! diagnostics (category / severity / default weight exposed as
//! `properties`), and one `result` per diagnostic with a `physicalLocation`
//! whose `region` carries 1-based line + column as the JSON schema
//! expects.
//!
//! SARIF severity levels collapse our three-variant `Severity` to the
//! three-variant `level` field: `info` → `note`, `warning` → `warning`,
//! `error` → `error`.
//!
//! Reference: SARIF v2.1.0 specification,
//! <https://docs.oasis-open.org/sarif/sarif/v2.1.0/os/sarif-v2.1.0-os.html>.

use std::collections::BTreeSet;
use std::path::Path;

use serde::Serialize;

use crate::scoring::Scorecard;
use crate::types::{Category, Diagnostic, Severity, SourceFile};

const SCHEMA: &str = "https://json.schemastore.org/sarif-2.1.0.json";
const SARIF_VERSION: &str = "2.1.0";
const DRIVER_NAME: &str = "lucid-lint";
const INFORMATION_URI: &str = "https://github.com/bastien-gallay/lucid-lint";
const HELP_URI_BASE: &str = "https://bastien-gallay.github.io/lucid-lint/rules";

#[derive(Debug, Serialize)]
struct Sarif<'a> {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<Run<'a>>,
}

#[derive(Debug, Serialize)]
struct Run<'a> {
    tool: Tool,
    results: Vec<Result_<'a>>,
}

#[derive(Debug, Serialize)]
struct Tool {
    driver: Driver,
}

#[derive(Debug, Serialize)]
struct Driver {
    name: &'static str,
    version: &'static str,
    #[serde(rename = "informationUri")]
    information_uri: &'static str,
    rules: Vec<RuleDescriptor>,
}

#[derive(Debug, Serialize)]
struct RuleDescriptor {
    id: String,
    name: String,
    #[serde(rename = "shortDescription")]
    short_description: MultiformatMessage,
    #[serde(rename = "helpUri")]
    help_uri: String,
    properties: RuleProperties,
}

#[derive(Debug, Serialize)]
struct RuleProperties {
    category: &'static str,
    #[serde(rename = "default-severity")]
    default_severity: &'static str,
    #[serde(rename = "default-weight")]
    default_weight: u32,
    tags: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct Result_<'a> {
    #[serde(rename = "ruleId")]
    rule_id: &'a str,
    level: &'static str,
    message: MultiformatMessage,
    locations: Vec<Location>,
    properties: ResultProperties,
}

#[derive(Debug, Serialize)]
struct ResultProperties {
    weight: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    section: Option<String>,
}

#[derive(Debug, Serialize)]
struct Location {
    #[serde(rename = "physicalLocation")]
    physical_location: PhysicalLocation,
}

#[derive(Debug, Serialize)]
struct PhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: ArtifactLocation,
    region: Region,
}

#[derive(Debug, Serialize)]
struct ArtifactLocation {
    uri: String,
}

#[derive(Debug, Serialize)]
struct Region {
    #[serde(rename = "startLine")]
    start_line: u32,
    #[serde(rename = "startColumn")]
    start_column: u32,
    #[serde(rename = "endColumn")]
    end_column: u32,
}

#[derive(Debug, Serialize)]
struct MultiformatMessage {
    text: String,
}

const fn sarif_level(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "note",
        Severity::Warning => "warning",
        Severity::Error => "error",
    }
}

const fn category_str(category: Category) -> &'static str {
    match category {
        Category::Structure => "structure",
        Category::Rhythm => "rhythm",
        Category::Lexicon => "lexicon",
        Category::Syntax => "syntax",
        Category::Readability => "readability",
    }
}

fn artifact_uri(source: &SourceFile) -> String {
    match source {
        SourceFile::Path(p) => path_to_uri(p),
        SourceFile::Stdin => "stdin".to_string(),
        SourceFile::Anonymous => "input".to_string(),
    }
}

fn path_to_uri(path: &Path) -> String {
    // SARIF expects a URI reference. Keep it simple: forward-slashed
    // relative path. Absolute paths pass through unchanged.
    path.to_string_lossy().replace('\\', "/")
}

fn rule_descriptor(rule_id: &str) -> RuleDescriptor {
    let category = Category::for_rule(rule_id);
    let default_weight = crate::scoring::default_weight_for(rule_id);
    // Use the rule's declared severity in `default_rules`? That would
    // require iterating rules to find the one matching rule_id. The
    // per-rule severity is stable at emission time; callers get the
    // concrete severity on each `result`. Here we surface a coarse
    // default so code-scanning consumers can filter on the rule itself.
    let default_severity = default_severity_for(rule_id);
    RuleDescriptor {
        id: rule_id.to_string(),
        name: rule_id.to_string(),
        short_description: MultiformatMessage {
            text: format!(
                "{} — cognitive accessibility check (category: {}).",
                rule_id,
                category_str(category),
            ),
        },
        help_uri: format!("{HELP_URI_BASE}/{rule_id}.html"),
        properties: RuleProperties {
            category: category_str(category),
            default_severity: sarif_level(default_severity),
            default_weight,
            tags: vec![category_str(category), "accessibility", "prose"],
        },
    }
}

fn default_severity_for(rule_id: &str) -> Severity {
    match rule_id {
        "lexicon.low-lexical-diversity" | "syntax.unclear-antecedent" | "readability.score" => {
            Severity::Info
        },
        _ => Severity::Warning,
    }
}

/// Render diagnostics + scorecard as a pretty-printed SARIF v2.1.0 log.
///
/// The `scorecard` argument is accepted for API symmetry with other
/// formatters; SARIF has no first-class score concept, so the
/// scorecard is not emitted. (A future extension could surface it under
/// `run.properties` — tracked behind the existing JSON schema for now.)
#[must_use]
pub fn render(diagnostics: &[Diagnostic], _scorecard: &Scorecard) -> String {
    let seen_rules: BTreeSet<&str> = diagnostics.iter().map(|d| d.rule_id.as_str()).collect();
    let rules: Vec<RuleDescriptor> = seen_rules.into_iter().map(rule_descriptor).collect();

    let results: Vec<Result_<'_>> = diagnostics
        .iter()
        .map(|d| {
            let end_column = d.location.column.saturating_add(d.location.length);
            Result_ {
                rule_id: &d.rule_id,
                level: sarif_level(d.severity),
                message: MultiformatMessage {
                    text: d.message.clone(),
                },
                locations: vec![Location {
                    physical_location: PhysicalLocation {
                        artifact_location: ArtifactLocation {
                            uri: artifact_uri(&d.location.file),
                        },
                        region: Region {
                            start_line: d.location.line,
                            start_column: d.location.column,
                            end_column,
                        },
                    },
                }],
                properties: ResultProperties {
                    weight: d.weight,
                    section: d.section.clone(),
                },
            }
        })
        .collect();

    let doc = Sarif {
        schema: SCHEMA,
        version: SARIF_VERSION,
        runs: vec![Run {
            tool: Tool {
                driver: Driver {
                    name: DRIVER_NAME,
                    version: env!("CARGO_PKG_VERSION"),
                    information_uri: INFORMATION_URI,
                    rules,
                },
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&doc).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::scoring::{self, ScoringConfig};
    use crate::types::{Location as Loc, SourceFile};

    fn sample_diag() -> Diagnostic {
        Diagnostic::new(
            "structure.sentence-too-long",
            Severity::Warning,
            Loc::new(SourceFile::Path(PathBuf::from("docs/foo.md")), 3, 1, 42),
            "Sentence is too long.",
        )
    }

    fn scorecard(diags: &[Diagnostic]) -> Scorecard {
        scoring::compute(diags, 1000, &ScoringConfig::default())
    }

    #[test]
    fn render_is_valid_json() {
        let diag = sample_diag();
        let card = scorecard(std::slice::from_ref(&diag));
        let sarif = render(&[diag], &card);
        let parsed: serde_json::Value = serde_json::from_str(&sarif).unwrap();
        assert_eq!(parsed["version"], "2.1.0");
        assert!(parsed["$schema"].as_str().unwrap().contains("sarif"));
    }

    #[test]
    fn render_carries_tool_metadata() {
        let diag = sample_diag();
        let card = scorecard(std::slice::from_ref(&diag));
        let sarif = render(&[diag], &card);
        let parsed: serde_json::Value = serde_json::from_str(&sarif).unwrap();
        let driver = &parsed["runs"][0]["tool"]["driver"];
        assert_eq!(driver["name"], "lucid-lint");
        assert_eq!(driver["version"], env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn render_emits_rule_descriptor_per_unique_rule() {
        let diags = vec![
            sample_diag(),
            Diagnostic::new(
                "lexicon.weasel-words",
                Severity::Warning,
                Loc::new(SourceFile::Path(PathBuf::from("a.md")), 1, 1, 5),
                "m",
            ),
            Diagnostic::new(
                "lexicon.weasel-words",
                Severity::Warning,
                Loc::new(SourceFile::Path(PathBuf::from("a.md")), 2, 1, 5),
                "m",
            ),
        ];
        let card = scorecard(&diags);
        let sarif = render(&diags, &card);
        let parsed: serde_json::Value = serde_json::from_str(&sarif).unwrap();
        let rules = parsed["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn render_emits_one_result_per_diagnostic() {
        let diags = vec![
            sample_diag(),
            Diagnostic::new(
                "lexicon.weasel-words",
                Severity::Info,
                Loc::new(SourceFile::Path(PathBuf::from("a.md")), 2, 3, 4),
                "m",
            ),
        ];
        let card = scorecard(&diags);
        let sarif = render(&diags, &card);
        let parsed: serde_json::Value = serde_json::from_str(&sarif).unwrap();
        let results = parsed["runs"][0]["results"].as_array().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["level"], "warning");
        assert_eq!(results[1]["level"], "note");
        let region = &results[0]["locations"][0]["physicalLocation"]["region"];
        assert_eq!(region["startLine"], 3);
        assert_eq!(region["startColumn"], 1);
        assert_eq!(region["endColumn"], 43);
    }

    #[test]
    fn snapshot_shape() {
        let diags = vec![sample_diag()];
        let card = scorecard(&diags);
        let sarif = render(&diags, &card);
        // Normalize the crate version so the snapshot is stable across
        // releases; the dedicated `render_carries_tool_metadata` test
        // pins the version value itself.
        let normalized = sarif.replace(env!("CARGO_PKG_VERSION"), "X.Y.Z");
        insta::assert_snapshot!(normalized);
    }
}
