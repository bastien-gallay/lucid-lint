# Changelog

All notable changes to `lucid-lint` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

The v0.2 cycle extends the v0.1 foundations. Work in flight:

- **Scoring** — hybrid model (global score + per-category sub-scores) and SARIF v2.1.0 output for GitHub Code Scanning (F14, F15, F17, F32).
- **Rule refinement** — definition-aware `unexplained-abbreviation`, language-specific readability formulas (Kandel-Moles FR, SMOG, Coleman-Liau), context-aware relaxations for `excessive-commas` and `weasel-words` (F9, F10, F22, F23).
- **Docs site** — French mirror of the docs (`/fr/`), full reading-preferences popover UI (font, line-spacing, text-size sliders — the v0.1 demonstrator is the seed), responsive / mobile adaptation, brand-owned theme-picker labels, and the accessibility-audit sweep (F25, F26, F33–F36).
- **Format support** — native AsciiDoc and HTML, a Pandoc companion script for docx (F5, F6, F7, F8).

See [`ROADMAP.md`](ROADMAP.md) for the full backlog with priorities.

## [0.1.1] — 2026-04-20

### Fixed

- **Windows release packaging** — the `Package (Windows)` job now runs under `bash` so the Unix-style `cp` invocation is parsed by Git Bash instead of PowerShell; v0.1.0 shipped without a Windows archive because PowerShell rejected `cp README.md LICENSE-MIT LICENSE-APACHE dist/` as too many positional arguments.

## [0.1.0] — 2026-04-20

First public release. Seventeen rules, bilingual EN/FR, deterministic core, Markdown + plain-text + stdin inputs.

### Added

#### Rules (17)

Length and structure:

- `sentence-too-long` — reference implementation; per-profile word-count threshold
- `paragraph-too-long` — sentence-count and word-count thresholds
- `excessive-commas` — per-profile comma-per-sentence threshold, aware of enumerations
- `long-enumeration` — flags 5+ comma-separated items; suggests list conversion
- `deep-subordination` — counts subordinators between strong-punctuation breaks
- `deeply-nested-lists` — flags list items nested beyond profile depth
- `heading-jump` — flags section-level jumps greater than +1

Rhythm:

- `consecutive-long-sentences` — intra-paragraph streaks of long sentences

Lexical:

- `weasel-words` — per-language vague-qualifier phrase list
- `unexplained-abbreviation` — acronym detection with baseline + per-profile whitelists
- `jargon-undefined` — profile-activated jargon lists (tech, legal, medical, accessibility)
- `excessive-nominalization` — per-sentence suffix-based density check
- `repetitive-connectors` — sliding-window connector frequency
- `low-lexical-diversity` — sliding-window type-token ratio over non-stopword tokens

Style and global:

- `passive-voice` — EN/FR heuristic `be`/`être` + past participle detector
- `unclear-antecedent` — bare demonstrative + verb, or paragraph-opening pronoun
- `readability-score` — Flesch–Kincaid grade per document

#### Engine and parser

- `Engine` with `with_profile` and `with_rules` constructors; `lint_str`, `lint_stdin`, `lint_file`.
- Pulldown-cmark-backed Markdown parser extracting paragraphs, sentences, sections, and list items.
- Plain-text parser with blank-line paragraph splitting and CRLF normalization.
- Shared `parser::phrase_search` helper: word-bounded, char-boundary-safe phrase search used by three lexical rules.
- Inline-disable directive: `<!-- lucid-lint disable-next-line <rule-id> -->` (Markdown only).

#### Profiles

- `dev-doc`, `public`, `falc` presets with per-rule threshold sets tuned to each audience.

#### Output

- TTY with color (auto-detected for terminals).
- `--format=json` with a stable schema.

#### CLI

- `lucid-lint check <path>…` accepting Markdown files, plain-text files, and stdin (`-`).
- `--profile` selects the threshold preset.
- Exit code 0 on clean input, 1 when warnings are present.

#### Documentation

- mdBook site (`docs/`) with installation, quick-start, profiles, configuration, CI integration, architecture, accessibility, and roadmap pages.
- WCAG 2.2 AAA typography layer (Atkinson Hyperlegible Next, Literata, OpenDyslexic, Commit Mono) with self-hosted fonts and a reading-preferences control.
- Design principles captured in `CODING_STANDARDS.md` around CUPID (Composable, Unix philosophy, Predictable, Idiomatic, Domain-based) plus YAGNI as the anti-speculation rule.
- Rule reference in `RULES.md`; living roadmap in `ROADMAP.md`.

#### Quality

- Pre-commit hooks: trailing whitespace, EOF, cargo fmt, cargo clippy (`-D warnings`), markdownlint.
- CI matrix: format, clippy, tests on Linux/macOS/Windows, MSRV build, rustdoc, security audit, dogfood.
- MSRV pinned to Rust 1.80.

[Unreleased]: https://github.com/bastien-gallay/lucid-lint/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/bastien-gallay/lucid-lint/releases/tag/v0.1.0
