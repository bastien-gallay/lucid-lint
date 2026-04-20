//! # lucid-lint
//!
//! A cognitive accessibility linter for prose. Built on cognitive load research.
//! Bilingual EN/FR. CI-native.
//!
//! ## Quick overview
//!
//! - [`config`] — profiles and rule configuration
//! - [`parser`] — Markdown and plain text parsing
//! - [`language`] — language detection
//! - [`rules`] — lint rules
//! - [`output`] — diagnostic formatters
//! - [`types`] — core domain types ([`Diagnostic`], [`Severity`], [`Location`], etc.)
//!
//! ## Example
//!
//! ```no_run
//! use lucid_lint::{Engine, Profile};
//!
//! let engine = Engine::with_profile(Profile::Public);
//! let report = engine.lint_str("Your text here.");
//! println!("score: {}/{}", report.scorecard.global.value, report.scorecard.global.max);
//! for diag in &report.diagnostics {
//!     println!("{}", diag.message);
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![forbid(unsafe_code)]

pub mod condition;
pub mod config;
pub mod language;
pub mod output;
pub mod parser;
pub mod rules;
pub mod scoring;
pub mod types;

mod engine;

pub use condition::ConditionTag;
pub use config::Profile;
pub use engine::{Engine, EngineError, Report};
pub use scoring::{CategoryScore, Score, Scorecard, ScoringConfig};
pub use types::{Category, Diagnostic, Language, Location, Severity};
