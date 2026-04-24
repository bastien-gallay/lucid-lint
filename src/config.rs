//! Configuration: profiles and per-rule settings.
//!
//! Users pick a [`Profile`] close to their intent and optionally override
//! specific rules via a `lucid-lint.toml` file at the project root.

use std::fmt;
use std::fs;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::condition::ConditionTag;
use crate::rules::readability::score::FormulaChoice;

/// Canonical filename the loader looks for when walking up from the
/// current working directory.
pub const CONFIG_FILENAME: &str = "lucid-lint.toml";

/// A preset bundle of rule thresholds tuned for a specific audience.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    /// Technical documentation, API docs, developer-facing content.
    DevDoc,
    /// Content for a general audience.
    Public,
    /// Facile À Lire et à Comprendre — European Easy-to-Read standard.
    Falc,
}

impl Profile {
    /// Default profile used when none is specified.
    pub const DEFAULT: Self = Self::Public;

    /// Parse a profile name from a string.
    ///
    /// Accepts `dev-doc`, `public`, `falc`. Case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::UnknownProfile`] if the input does not match a known profile.
    pub fn from_name(name: &str) -> Result<Self, ConfigError> {
        match name.to_lowercase().as_str() {
            "dev-doc" | "dev_doc" | "devdoc" => Ok(Self::DevDoc),
            "public" => Ok(Self::Public),
            "falc" => Ok(Self::Falc),
            other => Err(ConfigError::UnknownProfile(other.to_string())),
        }
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DevDoc => f.write_str("dev-doc"),
            Self::Public => f.write_str("public"),
            Self::Falc => f.write_str("falc"),
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Top-level configuration loaded from `lucid-lint.toml`.
///
/// All fields are optional. Missing fields fall back to profile defaults.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Default profile applied when no CLI override is provided.
    #[serde(default)]
    pub default: DefaultConfig,

    /// Per-rule configuration overrides.
    #[serde(default)]
    pub rules: RulesConfig,

    /// Scoring tunables (category caps and per-rule weight overrides).
    #[serde(default)]
    pub scoring: ScoringFileConfig,

    /// Config-based diagnostic ignores (F19). Each `[[ignore]]` entry
    /// silences every diagnostic with the matching `rule_id`, across
    /// all input sources (Markdown, plain text, stdin). Complements
    /// inline-disable directives, which remain the recommended local
    /// escape hatch when you only want to silence one spot.
    ///
    /// ```toml
    /// [[ignore]]
    /// rule_id = "lexicon.unexplained-abbreviation"
    ///
    /// [[ignore]]
    /// rule_id = "lexicon.weasel-words"
    /// ```
    #[serde(default, rename = "ignore")]
    pub ignores: Vec<IgnoreSpec>,
}

impl Config {
    /// Load a configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if the file cannot be read,
    /// or [`ConfigError::Parse`] if the contents are not valid TOML.
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path).map_err(ConfigError::Io)?;
        Self::from_toml_str(&contents)
    }

    /// Parse a configuration from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] if the string is not valid TOML.
    pub fn from_toml_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(|e| ConfigError::Parse(e.to_string()))
    }

    /// Walk up from `start` (inclusive) looking for [`CONFIG_FILENAME`].
    ///
    /// Returns the resolved path of the first match, or `None` if none
    /// was found up to the filesystem root. The walk stops at the first
    /// `.git` directory parent as well — a reasonable "repo boundary"
    /// heuristic that prevents the loader from leaking into unrelated
    /// parent projects on shared machines.
    #[must_use]
    pub fn discover_from(start: &Path) -> Option<PathBuf> {
        let mut current = if start.is_file() {
            start.parent()?
        } else {
            start
        };
        loop {
            let candidate = current.join(CONFIG_FILENAME);
            if candidate.is_file() {
                return Some(candidate);
            }
            // Repo boundary: stop if we just inspected a directory that
            // contains a `.git` entry (file or directory).
            if current.join(".git").exists() {
                return None;
            }
            match current.parent() {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    /// Extract the `[rules."lexicon.unexplained-abbreviation"].whitelist` field
    /// when present. Returns an empty list if the sub-table or field is
    /// missing, and an error if the field exists but is not an array
    /// of strings (typo-guarding).
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] if `whitelist` is present but not
    /// an array of strings.
    pub fn unexplained_abbreviation_whitelist(&self) -> Result<Vec<String>, ConfigError> {
        let Some(sub) = self.rules.entries.get("lexicon.unexplained-abbreviation") else {
            return Ok(Vec::new());
        };
        let Some(value) = sub.get("whitelist") else {
            return Ok(Vec::new());
        };
        let Some(array) = value.as_array() else {
            return Err(ConfigError::Parse(format!(
                "[rules.\"lexicon.unexplained-abbreviation\"].whitelist must be an array of strings, got {}",
                value.type_str()
            )));
        };
        let mut out = Vec::with_capacity(array.len());
        for (idx, entry) in array.iter().enumerate() {
            let Some(s) = entry.as_str() else {
                return Err(ConfigError::Parse(format!(
                    "[rules.\"lexicon.unexplained-abbreviation\"].whitelist[{idx}] must be a string, got {}",
                    entry.type_str()
                )));
            };
            // The detector only matches runs of ASCII uppercase + digits
            // (see `is_acronym_byte` in the rule). Any other shape would
            // silently never match — fail loud at load time instead.
            if s.is_empty()
                || !s
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
            {
                return Err(ConfigError::Parse(format!(
                    "[rules.\"lexicon.unexplained-abbreviation\"].whitelist[{idx}] = {s:?} must be \
                     a non-empty string of ASCII uppercase letters and digits (e.g. \"WCAG\", \"ARIA\", \"LLM\")"
                )));
            }
            out.push(s.to_string());
        }
        Ok(out)
    }

    /// Extract the `[rules."structure.excessive-commas"].max_commas`
    /// field when present. Returns `None` if the sub-table or field is
    /// missing, and an error if the field exists but is not a positive
    /// integer (zero or negative rejected, since the runtime threshold
    /// is `NonZeroU32`).
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] if `max_commas` is present but
    /// not a positive integer.
    pub fn excessive_commas_max_commas(&self) -> Result<Option<NonZeroU32>, ConfigError> {
        let Some(sub) = self.rules.entries.get("structure.excessive-commas") else {
            return Ok(None);
        };
        let Some(value) = sub.get("max_commas") else {
            return Ok(None);
        };
        let Some(raw) = value.as_integer() else {
            return Err(ConfigError::Parse(format!(
                "[rules.\"structure.excessive-commas\"].max_commas must be a positive integer, got {}",
                value.type_str()
            )));
        };
        let parsed = u32::try_from(raw).ok().and_then(NonZeroU32::new);
        parsed.map(Some).ok_or_else(|| {
            ConfigError::Parse(format!(
                "[rules.\"structure.excessive-commas\"].max_commas = {raw} must be a positive integer \
                 (minimum 1)"
            ))
        })
    }

    /// Extract the `[rules."readability.score"].formula` field when
    /// present. Returns `None` if the sub-table or field is missing,
    /// and an error if the field exists but is not a recognised string
    /// value (typo-guarding).
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] if `formula` is present but not
    /// a recognised value.
    pub fn readability_formula(&self) -> Result<Option<FormulaChoice>, ConfigError> {
        let Some(sub) = self.rules.entries.get("readability.score") else {
            return Ok(None);
        };
        let Some(value) = sub.get("formula") else {
            return Ok(None);
        };
        let Some(raw) = value.as_str() else {
            return Err(ConfigError::Parse(format!(
                "[rules.\"readability.score\"].formula must be a string, got {}",
                value.type_str()
            )));
        };
        FormulaChoice::from_cli(raw).map(Some).map_err(|bad| {
            ConfigError::Parse(format!(
                "[rules.\"readability.score\"].formula = {bad:?} is not a recognised value \
                 (expected one of: auto, flesch-kincaid, kandel-moles)"
            ))
        })
    }
}

/// Top-level `[default]` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    /// Default profile. `public` when omitted.
    #[serde(default)]
    pub profile: Profile,

    /// Active condition tags (F72). Enables tagged rules on top of the
    /// chosen profile. Rules tagged `general` are always active; tagged
    /// rules without `general` only run when their tag appears here.
    ///
    /// ```toml
    /// [default]
    /// profile = "falc"
    /// conditions = ["dyslexia", "aphasia"]
    /// ```
    #[serde(default)]
    pub conditions: Vec<ConditionTag>,

    /// Glob patterns of paths to skip at discovery time (F78). Matched
    /// against the slash-normalised path of each candidate file and
    /// every directory encountered during recursion. Matching
    /// directories are not descended into.
    ///
    /// ```toml
    /// [default]
    /// exclude = ["vendor/**", "**/fixtures/**", "CHANGELOG.md"]
    /// ```
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            profile: Profile::DEFAULT,
            conditions: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

/// A single `[[ignore]]` entry from `lucid-lint.toml`.
///
/// Unknown rule ids are tolerated at load time so removing a rule in a
/// future release does not break older configs — the same precedent
/// as `[scoring.weights]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreSpec {
    /// Rule id to silence (e.g. `"structure.sentence-too-long"`).
    pub rule_id: String,
}

/// Per-rule settings, keyed by rule id.
///
/// Example:
/// ```toml
/// [rules."structure.sentence-too-long"]
/// max_words = 25
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RulesConfig {
    /// Raw map of rule id to TOML table. Rules parse their own sub-table.
    pub entries: std::collections::BTreeMap<String, toml::Value>,
}

/// `[scoring]` table in `lucid-lint.toml`.
///
/// All fields are optional. Missing fields fall back to the defaults
/// defined in [`crate::scoring`].
///
/// Example:
///
/// ```toml
/// [scoring]
/// category_max = 25
/// category_cap = 20
///
/// [scoring.weights]
/// "structure.sentence-too-long" = 3
/// "lexicon.weasel-words" = 2
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScoringFileConfig {
    /// Override the per-category maximum (default 20).
    #[serde(default)]
    pub category_max: Option<u32>,

    /// Override the per-category cap (default 15).
    #[serde(default)]
    pub category_cap: Option<u32>,

    /// Per-rule weight overrides, keyed by rule id.
    #[serde(default)]
    pub weights: std::collections::BTreeMap<String, u32>,
}

impl ScoringFileConfig {
    /// Materialize into a runtime [`crate::scoring::ScoringConfig`].
    #[must_use]
    pub fn into_scoring_config(self) -> crate::scoring::ScoringConfig {
        let defaults = crate::scoring::ScoringConfig::default();
        crate::scoring::ScoringConfig {
            category_max: self.category_max.unwrap_or(defaults.category_max),
            category_cap: self.category_cap.unwrap_or(defaults.category_cap),
            weight_overrides: self.weights,
        }
    }
}

/// Errors returned by config loading and parsing.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Unknown profile name.
    #[error("unknown profile `{0}` (expected one of: dev-doc, public, falc)")]
    UnknownProfile(String),

    /// I/O error reading a config file.
    #[error("failed to read config file")]
    Io(#[source] std::io::Error),

    /// TOML parse error.
    #[error("failed to parse config: {0}")]
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_from_name_accepts_canonical_forms() {
        assert_eq!(Profile::from_name("dev-doc").unwrap(), Profile::DevDoc);
        assert_eq!(Profile::from_name("public").unwrap(), Profile::Public);
        assert_eq!(Profile::from_name("falc").unwrap(), Profile::Falc);
    }

    #[test]
    fn profile_from_name_is_case_insensitive() {
        assert_eq!(Profile::from_name("PUBLIC").unwrap(), Profile::Public);
        assert_eq!(Profile::from_name("Dev-Doc").unwrap(), Profile::DevDoc);
    }

    #[test]
    fn profile_from_name_rejects_unknown() {
        assert!(matches!(
            Profile::from_name("nonexistent"),
            Err(ConfigError::UnknownProfile(_))
        ));
    }

    #[test]
    fn profile_default_is_public() {
        assert_eq!(Profile::default(), Profile::Public);
    }

    #[test]
    fn config_parses_empty_toml() {
        let config = Config::from_toml_str("").unwrap();
        assert_eq!(config.default.profile, Profile::Public);
        assert!(config.rules.entries.is_empty());
    }

    #[test]
    fn config_parses_conditions_list() {
        let config = Config::from_toml_str(
            r#"[default]
profile = "falc"
conditions = ["dyslexia", "aphasia"]
"#,
        )
        .unwrap();
        assert_eq!(config.default.profile, Profile::Falc);
        assert_eq!(
            config.default.conditions,
            vec![ConditionTag::Dyslexia, ConditionTag::Aphasia]
        );
    }

    #[test]
    fn config_rejects_unknown_condition_tag() {
        assert!(matches!(
            Config::from_toml_str(
                r#"[default]
conditions = ["autism"]
"#,
            ),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn config_parses_ignore_entries() {
        let config = Config::from_toml_str(
            r#"
[[ignore]]
rule_id = "structure.sentence-too-long"

[[ignore]]
rule_id = "lexicon.weasel-words"
"#,
        )
        .unwrap();
        assert_eq!(config.ignores.len(), 2);
        assert_eq!(config.ignores[0].rule_id, "structure.sentence-too-long");
        assert_eq!(config.ignores[1].rule_id, "lexicon.weasel-words");
    }

    #[test]
    fn config_ignores_default_to_empty() {
        let config = Config::from_toml_str("").unwrap();
        assert!(config.ignores.is_empty());
    }

    #[test]
    fn config_ignore_tolerates_unknown_rule_id() {
        // Same precedent as [scoring.weights]: removing a rule in a
        // future release must not break older configs.
        let config = Config::from_toml_str(
            r#"
[[ignore]]
rule_id = "rule-that-does-not-exist"
"#,
        )
        .unwrap();
        assert_eq!(config.ignores.len(), 1);
    }

    #[test]
    fn config_parses_exclude_list() {
        let config = Config::from_toml_str(
            r#"[default]
exclude = ["vendor/**", "CHANGELOG.md"]
"#,
        )
        .unwrap();
        assert_eq!(
            config.default.exclude,
            vec!["vendor/**".to_string(), "CHANGELOG.md".to_string()]
        );
    }

    #[test]
    fn config_parses_default_section() {
        let config = Config::from_toml_str(
            r#"[default]
profile = "falc"
"#,
        )
        .unwrap();
        assert_eq!(config.default.profile, Profile::Falc);
    }

    #[test]
    fn config_parses_rule_overrides() {
        let config = Config::from_toml_str(
            r#"
[rules."structure.sentence-too-long"]
max_words = 25
"#,
        )
        .unwrap();
        assert!(config
            .rules
            .entries
            .contains_key("structure.sentence-too-long"));
    }

    #[test]
    fn config_rejects_invalid_toml() {
        assert!(matches!(
            Config::from_toml_str("not valid toml ="),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn config_parses_scoring_overrides() {
        let config = Config::from_toml_str(
            r#"
[scoring]
category_max = 25
category_cap = 18

[scoring.weights]
"structure.sentence-too-long" = 4
"lexicon.weasel-words" = 2
"#,
        )
        .unwrap();
        assert_eq!(config.scoring.category_max, Some(25));
        assert_eq!(config.scoring.category_cap, Some(18));
        assert_eq!(
            config.scoring.weights.get("structure.sentence-too-long"),
            Some(&4)
        );
        assert_eq!(config.scoring.weights.get("lexicon.weasel-words"), Some(&2));

        let runtime = config.scoring.into_scoring_config();
        assert_eq!(runtime.category_max, 25);
        assert_eq!(runtime.category_cap, 18);
        assert_eq!(
            runtime.weight_overrides.get("structure.sentence-too-long"),
            Some(&4)
        );
    }

    #[test]
    fn missing_scoring_table_falls_back_to_defaults() {
        let config = Config::from_toml_str("").unwrap();
        let runtime = config.scoring.into_scoring_config();
        assert_eq!(runtime.category_max, crate::scoring::DEFAULT_CATEGORY_MAX);
        assert_eq!(runtime.category_cap, crate::scoring::DEFAULT_CATEGORY_CAP);
        assert!(runtime.weight_overrides.is_empty());
    }

    #[test]
    fn unexplained_whitelist_defaults_to_empty() {
        let config = Config::from_toml_str("").unwrap();
        assert!(config
            .unexplained_abbreviation_whitelist()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn unexplained_whitelist_parses_array_of_strings() {
        let config = Config::from_toml_str(
            r#"
[rules."lexicon.unexplained-abbreviation"]
whitelist = ["WCAG", "ARIA", "ADHD"]
"#,
        )
        .unwrap();
        let list = config.unexplained_abbreviation_whitelist().unwrap();
        assert_eq!(list, vec!["WCAG", "ARIA", "ADHD"]);
    }

    #[test]
    fn unexplained_whitelist_rejects_non_array() {
        let config = Config::from_toml_str(
            r#"
[rules."lexicon.unexplained-abbreviation"]
whitelist = "WCAG"
"#,
        )
        .unwrap();
        assert!(matches!(
            config.unexplained_abbreviation_whitelist(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn unexplained_whitelist_rejects_non_string_entry() {
        let config = Config::from_toml_str(
            r#"
[rules."lexicon.unexplained-abbreviation"]
whitelist = ["WCAG", 42]
"#,
        )
        .unwrap();
        assert!(matches!(
            config.unexplained_abbreviation_whitelist(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn unexplained_whitelist_rejects_lowercase_entry() {
        // The detector only matches runs of ASCII uppercase + digits, so a
        // lowercase entry would silently never fire. Fail loud at load time.
        let config = Config::from_toml_str(
            r#"
[rules."lexicon.unexplained-abbreviation"]
whitelist = ["wcag"]
"#,
        )
        .unwrap();
        let err = config.unexplained_abbreviation_whitelist().unwrap_err();
        assert!(matches!(err, ConfigError::Parse(_)));
        let msg = err.to_string();
        assert!(
            msg.contains("\"wcag\""),
            "error should name the offending entry, got: {msg}"
        );
    }

    #[test]
    fn unexplained_whitelist_rejects_mixed_case_entry() {
        let config = Config::from_toml_str(
            r#"
[rules."lexicon.unexplained-abbreviation"]
whitelist = ["Wcag"]
"#,
        )
        .unwrap();
        assert!(matches!(
            config.unexplained_abbreviation_whitelist(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn unexplained_whitelist_rejects_empty_string_entry() {
        let config = Config::from_toml_str(
            r#"
[rules."lexicon.unexplained-abbreviation"]
whitelist = ["WCAG", ""]
"#,
        )
        .unwrap();
        assert!(matches!(
            config.unexplained_abbreviation_whitelist(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn unexplained_whitelist_accepts_digits_in_entries() {
        // `WCAG21`, `HTML5`, `UTF8` are valid — the detector accepts digits.
        let config = Config::from_toml_str(
            r#"
[rules."lexicon.unexplained-abbreviation"]
whitelist = ["WCAG21", "HTML5", "UTF8"]
"#,
        )
        .unwrap();
        let list = config.unexplained_abbreviation_whitelist().unwrap();
        assert_eq!(list, vec!["WCAG21", "HTML5", "UTF8"]);
    }

    #[test]
    fn excessive_commas_max_commas_absent_when_unset() {
        let config = Config::from_toml_str("").unwrap();
        assert_eq!(config.excessive_commas_max_commas().unwrap(), None);
    }

    #[test]
    fn excessive_commas_max_commas_parses_positive_integer() {
        let config = Config::from_toml_str(
            r#"
[rules."structure.excessive-commas"]
max_commas = 5
"#,
        )
        .unwrap();
        assert_eq!(
            config.excessive_commas_max_commas().unwrap(),
            Some(NonZeroU32::new(5).unwrap())
        );
    }

    #[test]
    fn excessive_commas_max_commas_rejects_zero() {
        let config = Config::from_toml_str(
            r#"
[rules."structure.excessive-commas"]
max_commas = 0
"#,
        )
        .unwrap();
        assert!(matches!(
            config.excessive_commas_max_commas(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn excessive_commas_max_commas_rejects_negative() {
        let config = Config::from_toml_str(
            r#"
[rules."structure.excessive-commas"]
max_commas = -1
"#,
        )
        .unwrap();
        assert!(matches!(
            config.excessive_commas_max_commas(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn excessive_commas_max_commas_rejects_non_integer() {
        let config = Config::from_toml_str(
            r#"
[rules."structure.excessive-commas"]
max_commas = "three"
"#,
        )
        .unwrap();
        assert!(matches!(
            config.excessive_commas_max_commas(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn readability_formula_absent_when_unset() {
        let config = Config::from_toml_str("").unwrap();
        assert_eq!(config.readability_formula().unwrap(), None);
    }

    #[test]
    fn readability_formula_reads_from_rule_table() {
        let config = Config::from_toml_str(
            r#"
[rules."readability.score"]
formula = "kandel-moles"
"#,
        )
        .unwrap();
        assert_eq!(
            config.readability_formula().unwrap(),
            Some(FormulaChoice::KandelMoles)
        );
    }

    #[test]
    fn readability_formula_rejects_unknown_value() {
        let config = Config::from_toml_str(
            r#"
[rules."readability.score"]
formula = "gunning-fog"
"#,
        )
        .unwrap();
        assert!(matches!(
            config.readability_formula(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn readability_formula_rejects_non_string() {
        let config = Config::from_toml_str(
            r#"
[rules."readability.score"]
formula = 42
"#,
        )
        .unwrap();
        assert!(matches!(
            config.readability_formula(),
            Err(ConfigError::Parse(_))
        ));
    }

    #[test]
    fn discover_walks_up_and_stops_at_repo_boundary() {
        use std::fs::File;

        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path();
        // Simulate a parent "other project" and a nested repo inside.
        let outer_config = root.join(CONFIG_FILENAME);
        File::create(&outer_config).unwrap();

        let repo = root.join("inner-repo");
        fs::create_dir_all(&repo).unwrap();
        fs::create_dir_all(repo.join(".git")).unwrap();
        let deep = repo.join("src").join("nested");
        fs::create_dir_all(&deep).unwrap();

        // From a deep dir inside the repo: since no `lucid-lint.toml`
        // exists at or below the `.git` boundary, discovery returns None
        // — not the outer-project config above the boundary.
        assert!(Config::discover_from(&deep).is_none());

        // Drop a config inside the repo → discovery picks it up.
        let repo_config = repo.join(CONFIG_FILENAME);
        File::create(&repo_config).unwrap();
        let found = Config::discover_from(&deep).expect("expected repo config");
        assert_eq!(found, repo_config);
    }

    #[test]
    fn discover_returns_none_when_no_file_exists() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(Config::discover_from(tmp.path()).is_none());
    }
}
