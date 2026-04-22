# Changelog

All notable changes to `lucid-lint` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Server-rendered skip link and language switch (F35a)** — the
  mdBook theme now forks upstream `index.hbs` (minimal-diff against
  mdBook v0.5.2) and emits the WCAG 2.4.1 "skip to content" anchor
  and the EN / FR switch inside the server-rendered HTML, not via
  post-paint JS injection. Both language variants are emitted; CSS
  in `lucid-layout.css` hides the wrong-locale copy based on
  `html[lang]`, which `theme/head.hbs` sets synchronously before
  first paint on `/fr/` pages. The skip link and lang switch now
  work with JavaScript disabled, closing the P1 findings from the
  2026-04-22 accessibility audit. A progressive-enhancement
  smooth-scroll handler remains in `lucid-navigation.js`; the
  previous `skipLink()` and `langSwitch()` IIFEs are gone. Unblocks
  F26 (stock theme labels can now be collapsed at the markup
  level).
- **Accessibility statement page fleshed out (F35d)** — the EN
  `docs/src/accessibility.md` now carries the first AAA audit
  pass (2026-04-22, 17/20) and a "Known limitations" section
  that lists every open finding in the F35 family (F35a–F35c)
  with a roadmap link and a plain-language explanation. Report
  route and audit cadence are spelled out. The FR
  `docs/src/fr/accessibility.md` stub gains a short parallel
  "Écarts connus" section so bilingual readers see the same
  gaps. F35d moves to 🟢 Shipped; F35 stays 🟡 In progress
  until F35a clears.
- **French docs mirror — skeleton + home (F25 first slice)** — the
  `docs/src/fr/` tree now carries a translated `introduction` (the
  prior stub is gone) plus a translated `rules-index` that mirrors
  the EN rules overview. Short `accessibility.md` and `roadmap.md`
  FR pages point readers to the full EN versions for now. A new
  "Version française" section in `SUMMARY.md` surfaces the FR pages
  in the sidebar. The `fr/` tree stays depth-2 to satisfy the
  `docs_links_stay_inside_docs` coverage rule. Per-rule FR pages
  stay in F25 for later slices.
- **`lexicon.weasel-words` false-positive cleanup (F23 completion)** —
  hits inside straight `"..."` or paired curly `"..."` quotes are
  now skipped, joining the inline-code-span and directional-pair
  exceptions shipped earlier in the F23 cycle. The author is
  *mentioning* the word in those spans rather than using it as a
  hedge (`Use "many" sparingly`, `The word "rather" weakens the
  sentence`). Scan is per-line, so an unclosed quote cannot leak
  across a newline and silence later hits. Single quotes /
  apostrophes are deliberately NOT recognised — they collide with
  possessives, contractions, and French elisions. The "concrete
  noun" semantic slice (`"many X"` with a concrete noun) remains
  unshipped by design: it needs POS data and belongs in the future
  `lucid-lint-nlp` plugin (F75).
- **Definition-aware `lexicon.unexplained-abbreviation` + project whitelist
  (F9 + F31)** — the rule is now two-pass. A pre-scan collects
  acronyms defined in the document in either canonical form
  (`Full Expansion (ACRONYM)` or `ACRONYM (Full Expansion)`) and a
  single definition anywhere in the document silences every
  occurrence of the same acronym. The parenthesised half of a
  definition must contain at least two alphabetic words, so
  throwaway notes like `(TBD)` or `(check later)` do not count.
  Paired with F31: the shipped `dev-doc` baseline whitelist is
  narrowed to the ubiquitous infrastructure stack (`URL`, `HTML`,
  `API`, `CPU`, `IDE`, …). Accessibility standards (`WCAG`, `ARIA`,
  `RGAA`, …), engineering-practice initialisms (`YAGNI`, `DRY`,
  `TDD`, …), and AI/language-tech terms (`LLM`, `NLP`) are no longer
  part of the baseline. Projects that use them add them to
  `[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml`
  — user entries are additive over the baseline. Silencing
  precedence is now `defined-in-doc → user whitelist → baseline`.
  See [`docs/src/rules/unexplained-abbreviation.md`](docs/src/rules/unexplained-abbreviation.md).

- **Config-based diagnostic ignores (F19)** — new top-level
  `[[ignore]]` array-of-tables in `lucid-lint.toml` silences every
  diagnostic with the matching `rule_id`, across Markdown, plain
  text, and stdin. Fills the gap for formats that have no inline-
  disable escape hatch, and gives users a project-wide alternative
  to sprinkling inline directives when a rule is globally noisy.
  Each entry requires a `rule_id` string; unknown ids are tolerated
  silently so removing a rule in a future release never breaks an
  older config. The filter runs after rule emission and before
  scoring, so score, render, and exit-code logic all see the
  filtered view. A `reason = "..."` field is tracked separately as
  F20. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#silencing-rules-globally-v02).
- **Path exclusion (F78)** — new `--exclude <GLOB>` CLI flag
  (comma-delimited, repeatable) and `exclude = [...]` field in the
  `[default]` section of `lucid-lint.toml`. Both lists are unioned and
  applied during directory recursion: matching files are skipped,
  matching directories are pruned without being entered. Patterns are
  matched against the path relative to the walked root, so
  `vendor/**` behaves the same whether the user passes a relative or
  absolute root. Explicit file arguments bypass exclusion — naming a
  path on the command line is treated as intent. Backed by the
  `globset` crate. Unblocks adoption on large documentation
  repositories. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#excluding-paths-v02).
- **TOML config loader wiring (F77)** — `lucid-lint check` now
  auto-discovers a `lucid-lint.toml` at or above the current working
  directory (stopping at the nearest `.git` boundary) and applies its
  `[default].profile`, `[default].conditions`, `[scoring]` weights /
  caps, and `[rules.readability-score].formula` fields. A new
  `--config <path>` flag overrides discovery; a missing explicit path
  is an error, a missing auto-discovered file is not. Precedence:
  built-in profile defaults → TOML → CLI flags (unset CLI defers to
  TOML; unset TOML defers to the preset). Per-rule TOML overrides
  beyond `readability.score` will land as each rule's `Config` gains a
  `Deserialize` impl. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md).
- **User-configurable readability formula (F11)** — new
  `--readability-formula` CLI flag and `FormulaChoice` enum exposed on
  `readability_score::Config`. `auto` (default) preserves the F10
  per-language behaviour; `flesch-kincaid` and `kandel-moles` pin a
  concrete formula regardless of detected language, which is useful
  for cross-document comparison on mixed corpora. Wired into the
  engine via `Engine::with_readability_formula(choice)`. See
  [`docs/src/rules/readability-score.md`](docs/src/rules/readability-score.md).
- **`lexicon.consonant-cluster` rule (F47)** — flags words whose longest run of
  consecutive consonants meets or exceeds a per-profile threshold (BDA
  Dyslexia Style Guide). Language-aware vowel sets: French accented
  forms (`é`, `è`, `ê`, `à`, `œ`, …) count as vowels; English fallback
  still accepts common latin-1 accented vowels for borrowed words
  (`café`, `naïve`); `y` is a vowel everywhere (lenient). Hyphens and
  apostrophes close the word. Profile thresholds: `min_run_length` 6 /
  5 / 4 (`dev-doc` / `public` / `falc`). Condition tags: `dyslexia`,
  `general`. See [`docs/src/rules/consonant-cluster.md`](docs/src/rules/consonant-cluster.md).
- **`syntax.dense-punctuation-burst` rule (F54)** — flags *local* bursts of
  punctuation: windows where ≥ N qualifying marks (`,`, `;`, `:`, `—`,
  `–`) cluster within W grapheme clusters (IFLA easy-to-read
  guidelines). Distinct from `structure.excessive-commas` (per-sentence count):
  this rule fires on local density, not total count. Per-source-line
  sliding window with greedy-extend; emits one diagnostic per burst,
  never overlapping. Profile thresholds: `dev-doc` 4/30, `public` 3/30,
  `falc` 3/40 — `dev-doc` tolerates a 3-mark cluster, FALC widens the
  window. Tag: `general`. See [`docs/src/rules/dense-punctuation-burst.md`](docs/src/rules/dense-punctuation-burst.md).
- **`lexicon.redundant-intensifier` rule (F62)** — flags intensifiers
  (`very`, `really`, `extremely`, `absolutely`, … / FR `très`,
  `vraiment`, `extrêmement`, `absolument`, …) that try to upgrade the
  confidence of a statement without adding information
  (plainlanguage.gov Chapter 4, CDC Clear Communication Index).
  Deliberate sibling of `lexicon.weasel-words`: weasel words downgrade
  confidence, intensifiers upgrade it — the two lexical lists are
  disjoint by construction. Per-language config supports
  `custom_intensifiers_{en,fr}` and `disable` for per-phrase
  suppression. Fenced / inline code spans are ignored; `Unknown`
  language skips the rule. See [`docs/src/rules/redundant-intensifier.md`](docs/src/rules/redundant-intensifier.md).
- **`structure.mixed-numeric-format` rule (F52)** — flags sentences that mix
  digit numerals (`42`, `3.14`, `1,000`, `1 000`) with spelled-out
  numerals (`two`, `trois`, `twenty`, `cent`) in the same sentence
  (CDC Clear Communication Index 3.5, plainlanguage.gov Chapter 4).
  Per-sentence scan using the shared tokenizer; code blocks excluded
  upstream by the Markdown parser. No configurable threshold — a
  single co-occurrence suffices. EN `one` and FR `un` / `une` are
  excluded from the spelled-numeral lists because they double as
  indefinite pronouns / articles. First `dyscalculia`-tagged
  `structure` rule. See [`docs/src/rules/mixed-numeric-format.md`](docs/src/rules/mixed-numeric-format.md).
- **`structure.line-length-wide` rule (F50)** — flags source lines wider than
  the per-profile ceiling (WCAG 1.4.8 AAA / BDA Dyslexia Style Guide
  grounding). Per-paragraph grapheme-cluster scan; fenced code blocks
  excluded upstream by the Markdown parser. Profile thresholds:
  `max_line_length` 120 / 100 / 80 (`dev-doc` / `public` / `falc`);
  FALC matches the WCAG 1.4.8 AAA recommendation. Condition tags:
  `dyslexia`, `general`. See [`docs/src/rules/line-length-wide.md`](docs/src/rules/line-length-wide.md).
- **Per-language readability formula (F10 must-ship slice)** —
  `readability.score` now selects its formula from the detected document
  language: Flesch-Kincaid for English (kept), Kandel & Moles (1958) for
  French. The Kandel-Moles ease score is converted to a grade-equivalent
  via the standard `(100 − score) / 10` linear approximation so
  per-profile `max_grade_level` thresholds remain comparable across
  languages. Unknown language falls back to Flesch-Kincaid. Diagnostic
  messages now surface the formula name and, for FR, both the native
  ease score and the grade-equivalent. User-configurable formula choice
  (F11) and the `Gunning Fog` / `SMOG` / `Dale-Chall` / `Scolarius`
  alternatives (F10 should-ship) are still pending.
- **`lexicon.all-caps-shouting` rule (F48)** — flags runs of two or more
  consecutive ALL-CAPS words (WCAG 3.1.5 / BDA Dyslexia Style Guide
  grounding). Per-profile thresholds: `min_run_length` 3 / 2 / 2
  (`dev-doc` / `public` / `falc`); `dev-doc` tolerates a 2-word `DO NOT`
  emphasis run. Single ALL-CAPS tokens stay with `lexicon.unexplained-abbreviation`.
  First `lexicon` rule to declare the `a11y-markup` tag (also `dyslexia`,
  `general`). See [`docs/src/rules/all-caps-shouting.md`](docs/src/rules/all-caps-shouting.md).
- **`syntax.conditional-stacking` rule (F56)** — flags sentences chaining
  multiple conditional connectors (FALC / plainlanguage.gov grounding).
  Per-profile thresholds: 3 / 2 / 1 (`dev-doc` / `public` / `falc`).
  Bilingual lists in `language::{en,fr}::CONDITIONALS`; FR also counts
  the `s'il` / `s'ils` clitics. Condition tags: `aphasia`, `adhd`,
  `general`. See [`docs/src/rules/conditional-stacking.md`](docs/src/rules/conditional-stacking.md).
- **`syntax.nested-negation` rule (F55)** — flags sentences that stack multiple
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
  Rules touched: `structure.sentence-too-long`, `structure.paragraph-too-long`,
  `structure.heading-jump`, `structure.deeply-nested-lists`, `structure.excessive-commas`,
  `structure.long-enumeration`, `structure.deep-subordination`,
  `rhythm.consecutive-long-sentences`, `rhythm.repetitive-connectors`,
  `lexicon.low-lexical-diversity`, `lexicon.excessive-nominalization`,
  `lexicon.unexplained-abbreviation`, `lexicon.weasel-words`, `lexicon.jargon-undefined`,
  `syntax.passive-voice`, `syntax.unclear-antecedent`, `readability.score`.
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
- **Context-aware `lexicon.weasel-words` — first slice of F23.** Hits inside
  inline code spans (`` `…` ``) are skipped so an author discussing a
  weasel term by name is no longer flagged for using it. Directional
  pairings `rather than` (EN) and `plutôt que` (FR) are recognised as
  conjunctions and skipped. Dogfood on this repo drops from 9 to 5
  weasel hits. Follow-up work on straight-quoted terms and `"many X"`
  with concrete X remains queued under F23. Rule touched:
  `lexicon.weasel-words`.
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
  `structure.excessive-commas` and `structure.deep-subordination` are `structure`,
  `rhythm.repetitive-connectors` is `rhythm`, `syntax.unclear-antecedent` is
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

- **Breaking — rule IDs now use `category.rule-name` form (F29-slim).**
  Every rule ID has been renamed from flat kebab-case to a
  category-prefixed form: `sentence-too-long` →
  `structure.sentence-too-long`, `weasel-words` →
  `lexicon.weasel-words`, `readability-score` → `readability.score`,
  and so on for the other 22 rules. Source files moved into category
  subdirectories under `src/rules/<category>/`. `Category::for_rule`
  now derives the category from the id prefix instead of a
  hand-maintained match arm (F43-style drift is impossible by
  construction). Pre-1.0 is the last cheap moment to break IDs
  before they propagate through downstream tooling. **Hard break —
  no alias layer.** Downstream callers must update every occurrence:
  - Markdown suppression directives:
    `<!-- lucid-lint disable-next-line sentence-too-long -->` →
    `<!-- lucid-lint disable-next-line structure.sentence-too-long -->`
    (same for `lucid-lint-disable` / `lucid-lint-enable` block forms).
  - `lucid-lint.toml` per-rule tables:
    `[rules.sentence-too-long]` → `[rules."structure.sentence-too-long"]`
    (quotes are required because of the `.`).
  - `lucid-lint.toml` scoring weights:
    `[scoring.weights] sentence-too-long = 3` →
    `[scoring.weights] "structure.sentence-too-long" = 3`.
  - JSON output (`version = 2`) and SARIF `ruleId` fields carry the
    new ids — CI consumers parsing these streams need to update any
    hard-coded id matches.
  - `lucid-lint explain <rule-id>` and `--disable <rule-id>` CLI
    args expect the new form.

  mdBook filenames and published docs URLs still use the flat kebab
  slug (`docs/src/rules/sentence-too-long.md`,
  `/rules/sentence-too-long.html`) — the docs-tree rearchitecture
  into category subdirectories is tracked as a separate slice. The
  explain module carries a slug mapping so `canonical_url()`
  resolves correctly.

- **Breaking — `lexicon.unexplained-abbreviation` baseline whitelist (F31).**
  Removed from the shipped `dev-doc` defaults: `WCAG`, `WAI`, `ARIA`,
  `RGAA`, `EAA`, `FALC`, `AA`, `AAA`, `ADHD`, `LLM`, `NLP`, `YAGNI`,
  `DRY`, `KISS`, `SOLID`, `TDD`, `BDD`, `MVP`, `WASM`, `MIT`, `LRU`.
  Projects that relied on these being built-in must add them to
  `[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml`
  to restore the previous behaviour. The diagnostic message now
  names the TOML field so the fix is self-evident. See
  [`lucid-lint.toml`](lucid-lint.toml) at the repo root for the
  full dogfood list lucid-lint uses on its own docs.
- **Breaking — `Category` remap.** The six v0.1 variants collapse to
  five: `length` / `structure` → `Structure`, `rhythm` → `Rhythm`,
  `lexical` → `Lexicon`, `style` / `rhythm.repetitive-connectors` → `Syntax` /
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

### Fixed

- **`structure.excessive-commas` parenthesised-list discount (F22 first
  slice)** — the rule now discounts commas inside `(A, B, C, …)`
  parenthesised token lists in addition to Oxford enumerations. A
  new sibling helper `parenthesised_list_comma_count` in
  `src/rules/enumeration.rs` recognises any balanced paren span
  holding three or more short comma-separated segments, accepting
  empty segments too so that markdown inline code (stripped by the
  parser) stays recognisable as `(, , , )`. Language-agnostic, no
  config change, no threshold change. Drops this repo's own dogfood
  `structure.excessive-commas` hit count from 25 to 15 (about 40% FP
  reduction). Non-parenthesised bare lists with 3+-word items
  (`as long as`, `as soon as`) and other surfaces stay flagged —
  they are explicitly deferred to v0.3 per
  `.personal/research/F22.md`.

### Still in flight (v0.2)

- **SARIF v2.1.0 output** for GitHub Code Scanning (F32).
- **Rule refinement** — definition-aware `lexicon.unexplained-abbreviation`,
  language-specific readability formulas (Kandel-Moles FR, SMOG,
  Coleman-Liau), context-aware relaxations for `structure.excessive-commas` and
  `lexicon.weasel-words` (F9, F10, F22, F23).
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

- `structure.sentence-too-long` — reference implementation; per-profile word-count threshold
- `structure.paragraph-too-long` — sentence-count and word-count thresholds
- `structure.excessive-commas` — per-profile comma-per-sentence threshold, aware of enumerations
- `structure.long-enumeration` — flags 5+ comma-separated items; suggests list conversion
- `structure.deep-subordination` — counts subordinators between strong-punctuation breaks
- `structure.deeply-nested-lists` — flags list items nested beyond profile depth
- `structure.heading-jump` — flags section-level jumps greater than +1

Rhythm:

- `rhythm.consecutive-long-sentences` — intra-paragraph streaks of long sentences

Lexical:

- `lexicon.weasel-words` — per-language vague-qualifier phrase list
- `lexicon.unexplained-abbreviation` — acronym detection with baseline + per-profile whitelists
- `lexicon.jargon-undefined` — profile-activated jargon lists (tech, legal, medical, accessibility)
- `lexicon.excessive-nominalization` — per-sentence suffix-based density check
- `rhythm.repetitive-connectors` — sliding-window connector frequency
- `lexicon.low-lexical-diversity` — sliding-window type-token ratio over non-stopword tokens

Style and global:

- `syntax.passive-voice` — EN/FR heuristic `be`/`être` + past participle detector
- `syntax.unclear-antecedent` — bare demonstrative + verb, or paragraph-opening pronoun
- `readability.score` — Flesch–Kincaid grade per document

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
