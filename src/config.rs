//! Configuration: profiles and per-rule settings.
//!
//! Users pick a [`Profile`] close to their intent and optionally override
//! specific rules via a `lucid-lint.toml` file at the project root.

use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

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
}

/// Top-level `[default]` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    /// Default profile. `public` when omitted.
    #[serde(default)]
    pub profile: Profile,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            profile: Profile::DEFAULT,
        }
    }
}

/// Per-rule settings, keyed by rule id.
///
/// Example:
/// ```toml
/// [rules.sentence-too-long]
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
/// sentence-too-long = 3
/// weasel-words = 2
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
            r"
[rules.sentence-too-long]
max_words = 25
",
        )
        .unwrap();
        assert!(config.rules.entries.contains_key("sentence-too-long"));
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
sentence-too-long = 4
weasel-words = 2
"#,
        )
        .unwrap();
        assert_eq!(config.scoring.category_max, Some(25));
        assert_eq!(config.scoring.category_cap, Some(18));
        assert_eq!(config.scoring.weights.get("sentence-too-long"), Some(&4));
        assert_eq!(config.scoring.weights.get("weasel-words"), Some(&2));

        let runtime = config.scoring.into_scoring_config();
        assert_eq!(runtime.category_max, 25);
        assert_eq!(runtime.category_cap, 18);
        assert_eq!(runtime.weight_overrides.get("sentence-too-long"), Some(&4));
    }

    #[test]
    fn missing_scoring_table_falls_back_to_defaults() {
        let config = Config::from_toml_str("").unwrap();
        let runtime = config.scoring.into_scoring_config();
        assert_eq!(runtime.category_max, crate::scoring::DEFAULT_CATEGORY_MAX);
        assert_eq!(runtime.category_cap, crate::scoring::DEFAULT_CATEGORY_CAP);
        assert!(runtime.weight_overrides.is_empty());
    }
}
