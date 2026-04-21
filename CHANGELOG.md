# Changelog

All notable changes to `lucid-lint` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`consonant-cluster` rule (F47)** — flags words whose longest run of
  consecutive consonants meets or exceeds a per-profile threshold (BDA
  Dyslexia Style Guide). Language-aware vowel sets: French accented
  forms (`é`, `è`, `ê`, `à`, `œ`, …) count as vowels; English fallback
  still accepts common latin-1 accented vowels for borrowed words
  (`café`, `naïve`); `y` is a vowel everywhere (lenient). Hyphens and
  apostrophes close the word. Profile thresholds: `min_run_length` 6 /
  5 / 4 (`dev-doc` / `public` / `falc`). Condition tags: `dyslexia`,
  `general`. See [`docs/src/rules/consonant-cluster.md`](docs/src/rules/consonant-cluster.md).
- **`dense-punctuation-burst` rule (F54)** — flags *local* bursts of
  punctuation: windows where ≥ N qualifying marks (`,`, `;`, `:`, `—`,
  `–`) cluster within W grapheme clusters (IFLA easy-to-read
  guidelines). Distinct from `excessive-commas` (per-sentence count):
  this rule fires on local density, not total count. Per-source-line
  sliding window with greedy-extend; emits one diagnostic per burst,
  never overlapping. Profile thresholds: `dev-doc` 4/30, `public` 3/30,
  `falc` 3/40 — `dev-doc` tolerates a 3-mark cluster, FALC widens the
  window. Tag: `general`. See [`docs/src/rules/dense-punctuation-burst.md`](docs/src/rules/dense-punctuation-burst.md).
- **`redundant-intensifier` rule (F62)** — flags intensifiers
  (`very`, `really`, `extremely`, `absolutely`, … / FR `très`,
  `vraiment`, `extrêmement`, `absolument`, …) that try to upgrade the
  confidence of a statement without adding information
  (plainlanguage.gov Chapter 4, CDC Clear Communication Index).
  Deliberate sibling of `weasel-words`: weasel words downgrade
  confidence, intensifiers upgrade it — the two lexical lists are
  disjoint by construction. Per-language config supports
  `custom_intensifiers_{en,fr}` and `disable` for per-phrase
  suppression. Fenced / inline code spans are ignored; `Unknown`
  language skips the rule. See [`docs/src/rules/redundant-intensifier.md`](docs/src/rules/redundant-intensifier.md).
- **`mixed-numeric-format` rule (F52)** — flags sentences that mix
  digit numerals (`42`, `3.14`, `1,000`, `1 000`) with spelled-out
  numerals (`two`, `trois`, `twenty`, `cent`) in the same sentence
  (CDC Clear Communication Index 3.5, plainlanguage.gov Chapter 4).
  Per-sentence scan using the shared tokenizer; code blocks excluded
  upstream by the Markdown parser. No configurable threshold — a
  single co-occurrence suffices. EN `one` and FR `un` / `une` are
  excluded from the spelled-numeral lists because they double as
  indefinite pronouns / articles. First `dyscalculia`-tagged
  `structure` rule. See [`docs/src/rules/mixed-numeric-format.md`](docs/src/rules/mixed-numeric-format.md).
- **`line-length-wide` rule (F50)** — flags source lines wider than
  the per-profile ceiling (WCAG 1.4.8 AAA / BDA Dyslexia Style Guide
  grounding). Per-paragraph grapheme-cluster scan; fenced code blocks
  excluded upstream by the Markdown parser. Profile thresholds:
  `max_line_length` 120 / 100 / 80 (`dev-doc` / `public` / `falc`);
  FALC matches the WCAG 1.4.8 AAA recommendation. Condition tags:
  `dyslexia`, `general`. See [`docs/src/rules/line-length-wide.md`](docs/src/rules/line-length-wide.md).
- **Per-language readability formula (F10 must-ship slice)** —
  `readability-score` now selects its formula from the detected document
  language: Flesch-Kincaid for English (kept), Kandel & Moles (1958) for
  French. The Kandel-Moles ease score is converted to a grade-equivalent
  via the standard `(100 − score) / 10` linear approximation so
  per-profile `max_grade_level` thresholds remain comparable across
  languages. Unknown language falls back to Flesch-Kincaid. Diagnostic
  messages now surface the formula name and, for FR, both the native
  ease score and the grade-equivalent. User-configurable formula choice
  (F11) and the `Gunning Fog` / `SMOG` / `Dale-Chall` / `Scolarius`
  alternatives (F10 should-ship) are still pending.
- **`all-caps-shouting` rule (F48)** — flags runs of two or more
  consecutive ALL-CAPS words (WCAG 3.1.5 / BDA Dyslexia Style Guide
  grounding). Per-profile thresholds: `min_run_length` 3 / 2 / 2
  (`dev-doc` / `public` / `falc`); `dev-doc` tolerates a 2-word `DO NOT`
  emphasis run. Single ALL-CAPS tokens stay with `unexplained-abbreviation`.
  First `lexicon` rule to declare the `a11y-markup` tag (also `dyslexia`,
  `general`). See [`docs/src/rules/all-caps-shouting.md`](docs/src/rules/all-caps-shouting.md).
- **`conditional-stacking` rule (F56)** — flags sentences chaining
  multiple conditional connectors (FALC / plainlanguage.gov grounding).
  Per-profile thresholds: 3 / 2 / 1 (`dev-doc` / `public` / `falc`).
  Bilingual lists in `language::{en,fr}::CONDITIONALS`; FR also counts
  the `s'il` / `s'ils` clitics. Condition tags: `aphasia`, `adhd`,
  `general`. See [`docs/src/rules/conditional-stacking.md`](docs/src/rules/conditional-stacking.md).
- **`nested-negation` rule (F55)** — flags sentences that stack multiple
  negations (FALC / CDC Clear Communication Index grounding). Per-profile
  thresholds: 3 / 2 / 1 (`dev-doc` / `public` / `falc`). Bilingual: EN
  counts the negation list plus contracted `n't`; FR counts `ne` / `n'`
  clitics plus standalone `sans` / `non` (avoids the `plus` / `personne`
  ambiguity outside `ne ... X`). Condition tags: `aphasia`, `adhd`,
  `general`. See [`docs/src/rules/nested-negation.md`](docs/src/rules/nested-negation.md).
- **Condition-tag ontology (F71 + F72)** — new `ConditionTag` enum
  (`a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`,
  `non-native`, `general`) plus `Rule::condition_tags()` trait method
  (default `&[General]`). User-facing surface: `[default] conditions =
  [...]` in `lucid-lint.toml` and the `--conditions` CLI flag
  (comma-separated). Filter semantics: rules tagged `general` always
  run; tagged-only rules opt in via the active list. All 17 v0.2 rules
  are `general`, so default behavior is unchanged. Tagged rules (F48,
  F55, F56) ride this infrastructure. See
  [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md).
- **Hybrid scoring model (F14)** — every run now emits a global `X / max`
  score plus five per-category sub-scores (Structure · Rhythm · Lexicon ·
  Syntax · Readability). Composition stacks a weighted sum, density
  normalization (per 1 000 words, floored at 200 words) and a per-category
  cap so no single rule dominates. See [`docs/src/guide/scoring.md`](docs/src/guide/scoring.md).
- **`--min-score=N` CLI flag** — optional gate that exits `1` when the
  aggregate document score falls below `N`. Stacks with the existing
  severity gate.
- **`[scoring]` / `[scoring.weights]` in `lucid-lint.toml`** — override
  `category_max`, `category_cap`, and per-rule weights without touching
  code.
- **`Diagnostic.weight` field** — populated at emission from
  `scoring::default_weight_for`; overridable per-diagnostic via
  `with_weight`.
- **Public `scoring` module** — `Score`, `CategoryScore`, `Scorecard`,
  `ScoringConfig`, `compute`, `default_weight_for`, `severity_multiplier`,
  plus the `Category::ALL` constant for fixed iteration order.
- **Per-rule mdBook pages (F28)** — every one of the 17 rules now has
  a dedicated page under `docs/src/rules/<rule-id>.md` covering
  category, severity, default weight, parameters per profile, EN/FR
  examples, and suppression. Wired into `docs/src/SUMMARY.md`;
  `docs/src/rules/index.md` turns into a categorised link index.
  Rules touched: `sentence-too-long`, `paragraph-too-long`,
  `heading-jump`, `deeply-nested-lists`, `excessive-commas`,
  `long-enumeration`, `deep-subordination`,
  `consecutive-long-sentences`, `repetitive-connectors`,
  `low-lexical-diversity`, `excessive-nominalization`,
  `unexplained-abbreviation`, `weasel-words`, `jargon-undefined`,
  `passive-voice`, `unclear-antecedent`, `readability-score`.
- **Rule documentation coverage gate (F42)** — new integration test
  `tests/rule_docs_coverage.rs` keeps five surfaces in lock-step: rule
  source, `rules::default_rules`, `Category::for_rule`,
  `scoring::WEIGHTED_RULE_IDS` (new public const), and the per-rule
  mdBook pages. When `RULE_DOCS_GATE_GIT=1` (set by the CI `test` job),
  any rule source modified versus `origin/main` must also be mentioned
  in the `## [Unreleased]` section of this file. Contract documented
  in `CONTRIBUTING.md` §"Adding or modifying a rule — documentation
  contract" and mirrored as directive 8 in `AGENTS.md`.
- **`docs/src/guide/suppression.md`** — dedicated page covering the
  inline and block disable directives, common properties, and the
  F19–F21 deferred extensions. The 17 per-rule pages now link to it
  via `../guide/suppression.md` (previously pointed at the
  repo-root `RULES.md`, which renders as a 404 inside mdBook).
- **SARIF v2.1.0 output (F32)** — `lucid-lint check --format=sarif` now
  emits a SARIF log compatible with GitHub Code Scanning (and any
  SARIF v2.1.0 consumer). Each rule appears once under
  `runs[0].tool.driver.rules` with its category, default severity,
  default scoring weight, and a `helpUri` to the per-rule mdBook page;
  per-result `properties` carry the diagnostic's scoring weight and
  the heading it was found under.
  [`docs/src/guide/ci-integration.md`](docs/src/guide/ci-integration.md)
  ships a full GitHub Actions workflow that uploads the SARIF to
  Code Scanning via `github/codeql-action/upload-sarif@v3`.
- **Context-aware `weasel-words` — first slice of F23.** Hits inside
  inline code spans (`` `…` ``) are skipped so an author discussing a
  weasel term by name is no longer flagged for using it. Directional
  pairings `rather than` (EN) and `plutôt que` (FR) are recognised as
  conjunctions and skipped. Dogfood on this repo drops from 9 to 5
  weasel hits. Follow-up work on straight-quoted terms and `"many X"`
  with concrete X remains queued under F23. Rule touched:
  `weasel-words`.
- **Roadmap mirror in the mdBook site (F27)** — `docs/src/roadmap.md`
  is auto-generated from the root `ROADMAP.md` by
  [`scripts/sync-roadmap.py`](scripts/sync-roadmap.py), invoked as a
  dependency of `just docs-build` / `just docs-serve`. Relative links
  are rewritten on the fly: targets under `docs/src/` become
  docs-relative, everything else becomes an absolute GitHub URL, so
  the `docs_links_stay_inside_docs` gate still passes.
- **Rule + feature cross-links across the mdBook site (F30)** — first
  mention of a rule id or feature id (in backticks / plain text) per
  H2/H3 section now links to its canonical page
  (`docs/src/rules/<id>.md` or `docs/src/roadmap.md`). Touches
  `accessibility.md`, `architecture/design-decisions.md`,
  `guide/scoring.md`, `guide/suppression.md`, `roadmap.md`, and eight
  per-rule pages.
- **`RULES.md` category drift fixed (F43)** — per-rule `**Category**`
  lines and the Categories table now match `Category::for_rule`:
  `excessive-commas` and `deep-subordination` are `structure`,
  `repetitive-connectors` is `rhythm`, `unclear-antecedent` is
  `syntax`. The drift banners on the four affected per-rule mdBook
  pages are removed.
- **In-docs link convention + gate.** A new test
  `docs_links_stay_inside_docs` scans `docs/src/**/*.md` and fails on
  any `](../../…)` pattern that escapes the mdBook tree. Convention
  written up as `AGENTS.md` directive 9 and the "Docs links stay
  inside `docs/src/`" section of `CONTRIBUTING.md`. Also fixed a
  pre-existing broken `../../ROADMAP.md` link in
  `docs/src/guide/ci-integration.md`.

### Changed

- **Breaking — `Category` remap.** The six v0.1 variants collapse to
  five: `length` / `structure` → `Structure`, `rhythm` → `Rhythm`,
  `lexical` → `Lexicon`, `style` / `repetitive-connectors` → `Syntax` /
  `Rhythm`, `global` → `Readability`. JSON `category` values and
  `Category::for_rule` are updated accordingly.
- **Breaking — `Engine::lint_str` / `lint_stdin` / `lint_file` return
  `Report`** (`diagnostics` + `scorecard` + `word_count`) instead of
  `Vec<Diagnostic>`. Call sites consume `report.diagnostics` and
  `report.scorecard`.
- **Breaking — JSON schema `version = 2`.** Adds `score`,
  `category_scores`, and per-diagnostic `weight`. Consumers on v0.1
  should bump the expected version and remap the category names listed
  above.
- **TTY output** appends one colorized `score:` line after the existing
  summary, followed by all five per-category scores in fixed order.

### Still in flight (v0.2)

- **SARIF v2.1.0 output** for GitHub Code Scanning (F32).
- **Rule refinement** — definition-aware `unexplained-abbreviation`,
  language-specific readability formulas (Kandel-Moles FR, SMOG,
  Coleman-Liau), context-aware relaxations for `excessive-commas` and
  `weasel-words` (F9, F10, F22, F23).
- **Rule-message clarity audit (F37)** — every diagnostic must answer
  "what do I change?"; gates the v0.2 release because score
  actionability depends on diagnostic actionability.
- **Docs site** — French mirror of the docs (`/fr/`), full reading-
  preferences popover UI, responsive / mobile adaptation, brand-owned
  theme-picker labels, accessibility-audit sweep (F25, F26, F33–F36).
- **Format support** — native AsciiDoc and HTML, a Pandoc companion
  script for docx (F5, F6, F7, F8).
- **Project-level scoring roll-up (F15)** — document-level lands in this
  cycle; multi-file roll-up still open.

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
