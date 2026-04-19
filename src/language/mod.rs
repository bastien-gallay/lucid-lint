//! Language detection and language-specific data.
//!
//! Language detection uses a simple stop-word ratio heuristic in v0.1.
//! See [`ROADMAP.md`] for the planned upgrade to a dedicated crate like `whatlang`.

use crate::types::Language;

mod detect;
pub mod en;
pub mod fr;

pub use detect::detect_language;

/// Returns the default language when detection is inconclusive.
///
/// Current policy: fall back to English, because most prose linters' training
/// data and word lists are English-biased, and English-first behavior is the
/// safer default for mixed corpora.
#[must_use]
pub const fn default_language() -> Language {
    Language::En
}
