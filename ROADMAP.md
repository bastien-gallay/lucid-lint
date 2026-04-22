# lucid-lint — Roadmap

> Future rules, refinements, and platform extensions tracked from v0.1 onwards.

v0.1 shipped on 2026-04-20 with 17 rules. The **v0.2 must-ship bundle** — hybrid scoring (F14), SARIF output (F32), condition-tag ontology (F71+F72), F10 per-language readability auto-select, F48 `all-caps-shouting`, F55 `nested-negation`, F56 `conditional-stacking`, F50 `line-length-wide`, F52 `mixed-numeric-format`, F62 `redundant-intensifier`, F54 `dense-punctuation-burst`, F47 `consonant-cluster`, and F11 user-configurable readability formula — is **complete** (rule count 17 → 25). This document now tracks v0.2 should-ship polish, v0.3 candidates, and longer-horizon research.

## Legend

| Status | Meaning |
|---|---|
| ✅ | Done (merged on `main`) |
| 🚧 | In progress |
| ☐ | Not started |

| Priority | Meaning |
|---|---|
| 🔴 Next | Actively queued for the next cut |
| 🟡 Later | Likely someday, not scheduled |
| 🟢 Speculative | Nice-to-have, exploratory |

Shipped items (✅) have their priority column set to `—` — the tag is meaningless once the item has landed.

---

## v0.1 — Released 2026-04-20

Shipped in the tag: all 17 rules across 5 phases, the minimal inline-disable directive, and the mdBook documentation site (Lucid light / Lucid dark themes, Atkinson Hyperlegible Next / Literata / Commit Mono / OpenDyslexic typography layer, reading-preferences demonstrator, accessibility page, EN/FR header switch with v0.2 FR-stub). See [`CHANGELOG.md`](CHANGELOG.md) for the full release notes.

### Rules (17 / 17) ✅

#### Phase 1 — Deterministic structural rules

| Status | Rule | Notes |
|---|---|---|
| ✅ | `paragraph-too-long` | Sentence-count + word-count thresholds per profile (`src/rules/paragraph_too_long.rs`) |
| ✅ | `deeply-nested-lists` | Flags list items nested beyond profile depth (`src/rules/deeply_nested_lists.rs`) |
| ✅ | `heading-jump` | Walks section depths, flags jumps > +1 level (`src/rules/heading_jump.rs`) |

#### Phase 2 — Simple text rules

| Status | Rule | Notes |
|---|---|---|
| ✅ | `sentence-too-long` | Reference implementation — template for the 15 others (`src/rules/sentence_too_long.rs`) |
| ✅ | `excessive-commas` | Per-profile comma-per-sentence threshold (`src/rules/excessive_commas.rs`) |
| ✅ | `consecutive-long-sentences` | Intra-paragraph streak of long sentences (`src/rules/consecutive_long_sentences.rs`) |

#### Phase 3 — Lexical rules with word lists

| Status | Rule | Notes |
|---|---|---|
| ✅ | `weasel-words` | Per-language phrase list, word-boundary match (`src/rules/weasel_words.rs`) |
| ✅ | `unexplained-abbreviation` | Pattern-based (v0.1); definition-awareness tracked as F9 (`src/rules/unexplained_abbreviation.rs`) |
| ✅ | `jargon-undefined` | Pattern-based, profile-activated category lists (`src/rules/jargon_undefined.rs`) |
| ✅ | `excessive-nominalization` | Per-sentence suffix-based density check (`src/rules/excessive_nominalization.rs`) |
| ✅ | `repetitive-connectors` | Sliding-window connector frequency, one diagnostic per cluster (`src/rules/repetitive_connectors.rs`) |

#### Phase 4 — Global metric

| Status | Rule | Notes |
|---|---|---|
| ✅ | `readability-score` | Per-document Flesch-Kincaid grade; info under threshold, warning above (`src/rules/readability_score.rs`) |

#### Phase 5 — Heuristic rules (hardest)

| Status | Rule | Notes |
|---|---|---|
| ✅ | `long-enumeration` | Shared enumeration detector with `excessive-commas`; suggests list conversion (`src/rules/long_enumeration.rs`, `src/rules/enumeration.rs`) |
| ✅ | `deep-subordination` | Counts subordinators between strong-punct breaks; skips pronoun enumerations (`src/rules/deep_subordination.rs`) |
| ✅ | `passive-voice` | Heuristic `be/être`+past-participle detector; POS-based detection remains a `lucid-lint-nlp` plugin candidate (`src/rules/passive_voice.rs`) |
| ✅ | `unclear-antecedent` | Info-level heuristic: bare demonstrative + verb, or paragraph-start personal pronoun (`src/rules/unclear_antecedent.rs`) |
| ✅ | `low-lexical-diversity` | Sliding-window TTR over non-stopword content tokens (`src/rules/low_lexical_diversity.rs`) |

### Cross-cutting features

| Status | Feature | Notes |
|---|---|---|
| ✅ | Minimal inline-disable | `<!-- lucid-lint disable-next-line <rule-id> -->` for Markdown inputs, single rule id, optional reason. See [RULES.md → Suppressing diagnostics](RULES.md#suppressing-diagnostics). Block form, config ignores, file-level scope and required `reason=` are tracked as F18–F21 below. |
| ✅ | Accessibility page in the docs | `docs/src/accessibility.md` covers the WCAG 2.2 AAA bar, the reading-preferences control, typography credits (Atkinson Hyperlegible Next — Braille Institute; OpenDyslexic — Abelardo Gonzalez; Literata — TypeTogether), keyboard shortcuts, and how the site dogfoods the project's mission. Linked from the sidebar and the footer. |

---

## v0.2 — First major iteration

### Next up (v0.2.x polish bundle)

Current 🔴 queue, grouped by theme:

- **False-positive cleanup** — F22 (`excessive-commas`)
- **Bilingual directive** — F25 (French docs mirror)
- **Docs-site finishing** — F34 (responsive), F35 (a11y audit sweep)

### Architecture

| ID | Item | Priority | Origin |
|---|---|---|---|
| F14 | ✅ Hybrid scoring model shipped in v0.2 (global score + per-category sub-scores + diagnostics). `X/max` arbitrary-max at both levels, 5 fixed categories (Structure · Rhythm · Lexicon · Syntax · Readability), composition = weighted sum × density-normalization × per-category cap, `weight` field added to `Diagnostic`, `--min-score=N` CLI flag. See [`docs/src/guide/scoring.md`](docs/src/guide/scoring.md). Letter-grade / traffic-light / reading-time decorations deferred (F39–F41). | 🔴 Next | Architecture decision discussion |
| F15 | 🚧 Document-level scoring shipped in v0.2 (multi-path runs are aggregated as one document). Project-level roll-up (per-file breakdown + project summary) still open. Section-level deferred → F38. | 🔴 Next | Linked to F14 |
| F17 | Per-family sub-scores | 🟡 Later | Linked to F14 |
| F32 | ✅ Shipped in v0.2 — `lucid-lint check --format=sarif` emits a SARIF v2.1.0 log compatible with GitHub Code Scanning. One rule descriptor per observed rule id (category, default severity, default weight, `helpUri` to the per-rule mdBook page); per-result properties carry weight + section. Workflow snippet in [`docs/src/guide/ci-integration.md`](docs/src/guide/ci-integration.md#github-code-scanning-sarif). | 🔴 Next | v0.1 AGENTS.md audit |
| F37 | ✅ Rule-message clarity audit completed: all 17 rules reviewed against "what do I change?" bar. 15 rules already actionable; `heading-jump` updated (first-heading-not-H1 and missing-H1 variants now include repair guidance). `readability-score` info variant left observational by design (fires only when `always_report` is set). | 🔴 Next | F14 `brainstorm/20260420-score-semantics.md` |
| F38 | Section-level granularity for scoring (deferred from F15) — per-heading sub-scores once document + project are proven in the wild. | 🟡 Later | F14 `brainstorm/20260420-score-semantics.md` |
| F39 | Letter-grade decoration (A–F) on the `X/max` score — promote when user feedback shows the numbers feel noisy or hard to compare across docs. | 🟡 Later | F14 `brainstorm/20260420-score-semantics.md` |
| F40 | Traffic-light (🔴🟡🟢) + pass/fail margin in the TTY output — promote when CI users ask for a stronger glance signal than the number alone. | 🟡 Later | F14 `brainstorm/20260420-score-semantics.md` |
| F41 | Reading-time-seconds as an alternative score unit — ties score to concrete user outcome. Requires validated heuristic + companion metrics (comfort, fatigue, understandability) so the time unit doesn't monopolize the read. | 🟢 Speculative | F14 `brainstorm/20260420-score-semantics.md` |
| F71 | ✅ Shipped in v0.2 — `ConditionTag` enum (fixed 7-variant ontology: `a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`, `non-native`, `general`) plus `Rule::condition_tags()` trait method (default `&[General]`). All 17 v0.2 rules are `general`; future tagged rules (F48, F55, F56) opt in by overriding. See [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md). | 🔴 Next | Rule-system-growth brainstorm (2026-04-20) |
| F72 | ✅ Shipped in v0.2 — `[default] conditions = [...]` config field and `--conditions` CLI flag (comma-separated). Filter semantics: rules tagged `general` always run; tagged-only rules run iff their tags intersect the active list. Profiles unchanged; FALC retains its regulatory meaning. See [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md). | 🔴 Next | Rule-system-growth brainstorm (2026-04-20) |

### Rules refinement

| ID | Item | Priority | Origin |
|---|---|---|---|
| F9 | ✅ Shipped in v0.2 — definition-aware `unexplained-abbreviation` is now two-pass. A pre-scan collects acronyms defined anywhere in the document in either canonical form (`Expansion (ACRONYM)` or `ACRONYM (Expansion)`; expansion side ≥ 2 alphabetic words to reject `(TBD)`-shaped noise), and a single definition silences every occurrence of that token. Silencing precedence: defined-in-doc → user whitelist → baseline. See [`docs/src/rules/unexplained-abbreviation.md`](docs/src/rules/unexplained-abbreviation.md). | — | Rule 10 simplified in v0.1 |
| F10 | 🚧 Must-ship slice shipped in v0.2 — `readability-score` auto-selects the formula by detected language: Flesch-Kincaid for EN (kept), Kandel & Moles (1958) for FR. Kandel-Moles ease scores are converted to a grade-equivalent so per-profile `max_grade_level` stays comparable across languages. Unknown language → Flesch-Kincaid. See [`docs/src/rules/readability-score.md`](docs/src/rules/readability-score.md). Still open: Gunning Fog / SMOG / Dale-Chall (EN), Scolarius / Flesch-Kandel (FR), `--readability-verbose` multi-formula reports, per-file override (covered by F11). | 🟡 Later | Rule 11 simplified in v0.1; scope expanded in rule-system-growth brainstorm (2026-04-20) |
| F11 | ✅ Shipped in v0.2 — `--readability-formula {auto,flesch-kincaid,kandel-moles}` CLI flag + `FormulaChoice` enum on `readability_score::Config` + `Engine::with_readability_formula(choice)`. `auto` (default) keeps F10 per-language selection; `flesch-kincaid` / `kandel-moles` pin a formula for cross-document comparison. TOML config wiring is tracked separately as F77. | 🟡 Later | Rule 11 |
| F13 | `missing-connectors` rule (15b not shipped in v0.1) | 🟡 Later | Rule 15 decomposition |
| F1 | Custom stoplist parameter for `low-lexical-diversity` | 🟡 Later | Rule 5 |
| F2 | Sentence-level low-lexical-diversity density | 🟢 Speculative | Rule 5 |
| F3 | Comma density metric (relative) for `excessive-commas` | 🟢 Speculative | Rule 3a |
| F22 | 🚧 First slice shipped in v0.2.x — `excessive-commas` now discounts commas inside `(A, B, C, …)` parenthesised token lists (3+ short comma-separated segments inside balanced parens, language-agnostic). Sibling helper `parenthesised_list_comma_count` in `src/rules/enumeration.rs`. Dogfood drops from 25 → 15 hits (10 FPs killed, ~40% reduction). Deferred to v0.3: relaxing `MAX_SEGMENT_WORDS = 2` for 3–4-word Oxford items, non-Oxford / "plus"-closed lists, interleaved parentheticals inside Oxford runs. See research note in `.personal/research/F22.md`. | 🔴 Next | v0.1 dogfood: 5 false-ish positives on technical docs |
| F23 | ✅ Shipped in v0.2 — false-positive cleanup complete for v0.2. Hits inside inline code spans, straight `"..."` quotes, paired curly `"..."` quotes, and directional `rather than` / `plutôt que` pairings are now skipped. Single quotes / apostrophes are deliberately not recognised (possessives, contractions, FR elisions). The "concrete noun" semantic check (`"many X"` where X is a concrete noun) stays unshipped — needs POS data and belongs in the `lucid-lint-nlp` plugin (F75) rather than the deterministic core. | — | v0.1 dogfood: 11 false-ish positives on this repo's own docs |
| F24 | Refine `excessive-nominalization` suffix list (drop or gate `-al`; many adjectives — `crucial`, `horizontal`, `positional`, `attentional` — are flagged despite not being abstract nouns) | 🟡 Later | v0.1 dogfood |
| F31 | ✅ Shipped in v0.2 — `dev-doc` baseline narrowed to the infrastructure stack (`URL`, `HTML`, `CSS`, `JSON`, `XML`, `HTTP`, `HTTPS`, `UTF`, `IO`, `API`, `CLI`, `GUI`, `OS`, `CPU`, `RAM`, `SSD`, `USB`, `IDE`, `SDK`, `CI`, `CD`). Accessibility standards, engineering-practice initialisms, and AI/language-tech terms moved to project config via new `[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml` (additive over baseline). Breaking change for downstream users, flagged in CHANGELOG with the recovery snippet. Dogfooded in this repo's own [`lucid-lint.toml`](lucid-lint.toml). | — | v0.1 review feedback |

<!-- lucid-lint disable-next-line weasel-words -->

**F22 context.** The v0.1 rule is a flat comma-per-sentence threshold.
In technical docs that routinely enumerate short items, this fires
often even when the sentence is perfectly scannable. Candidate
relaxations to evaluate (needs corpus research — don't pick blindly):

- **Discount commas inside parenthesis-like elements** (`(...)`,
  `[...]`, en/em-dash pairs). A parenthetical enumeration is already
  visually bracketed; its commas are not adding subordination load.
- **Discount commas after a colon `:`** when what follows is a list of
  short items. Colon + short items is idiomatic prose-enumeration and
  reads well.
- **Short-item enumeration exemption**: if all comma-separated
  segments are 1–2 words, treat the enumeration as a single
  "flattened list" token for counting purposes (a
  `max_short_enum_items` parameter, or implicit).
- **Interaction with `long-enumeration`**: the shared
  `enumeration::detect_enumerations` helper already discounts Oxford-
  style enumeration commas from `excessive-commas` (3+ short items).
  F22 is specifically about the cases that helper still misses:
  parentheticals, post-colon lists, and non-Oxford enumerations
  ("A, B, C and D" without the final comma).

Research inputs to gather before deciding: FR/EN corpus samples of
technical docs, a handful of real false positives from dogfooding and
downstream projects, how `textlint` / Vale / `write-good` handle
parentheticals. Decide between relaxation parameters vs. a smarter
token-aware counter.

### New rules (v0.2)

New rule candidates raised in the rule-system-growth brainstorm
(2026-04-20). Naming uses a provisional `category.rule-name` prefix
pending F29 harmonisation. Grounding column points at the standard or
research that justifies the rule.

**Must-ship v0.2 (blocking release):**

| ID | Rule | Category | Tags | Grounding | Priority |
|---|---|---|---|---|---|
| F48 | ✅ `all-caps-shouting` shipped in v0.2 — see [`docs/src/rules/all-caps-shouting.md`](docs/src/rules/all-caps-shouting.md) | Lexicon | `a11y-markup`, `dyslexia`, `general` | WCAG 3.1.5, BDA Dyslexia Style Guide | 🔴 Next |
| F55 | ✅ `nested-negation` shipped in v0.2 — see [`docs/src/rules/nested-negation.md`](docs/src/rules/nested-negation.md) | Syntax | `aphasia`, `adhd`, `general` | FALC, CDC Clear Communication Index | 🔴 Next |
| F56 | ✅ `conditional-stacking` shipped in v0.2 — see [`docs/src/rules/conditional-stacking.md`](docs/src/rules/conditional-stacking.md) | Syntax | `aphasia`, `adhd`, `general` | FALC, plainlanguage.gov | 🔴 Next |

**Should-ship v0.2 (cuttable under time pressure, in suggested cut order):**

| ID | Rule | Category | Tags | Grounding | Priority |
|---|---|---|---|---|---|
| F62 | ✅ `redundant-intensifier` shipped in v0.2 — see [`docs/src/rules/redundant-intensifier.md`](docs/src/rules/redundant-intensifier.md) | Lexicon | `general` | Plain-language guides | 🟡 Later |
| F52 | ✅ `mixed-numeric-format` shipped in v0.2 — see [`docs/src/rules/mixed-numeric-format.md`](docs/src/rules/mixed-numeric-format.md) | Structure | `dyscalculia`, `general` | CDC Clear Communication Index | 🟡 Later |
| F50 | ✅ `line-length-wide` shipped in v0.2 — see [`docs/src/rules/line-length-wide.md`](docs/src/rules/line-length-wide.md) | Structure | `dyslexia`, `general` | WCAG 1.4.8 (AAA) | 🟡 Later |
| F47 | ✅ `consonant-cluster` shipped in v0.2 — see [`docs/src/rules/consonant-cluster.md`](docs/src/rules/consonant-cluster.md) | Lexicon | `dyslexia`, `general` | BDA Dyslexia Style Guide | 🟡 Later |
| F54 | ✅ `dense-punctuation-burst` shipped in v0.2 — see [`docs/src/rules/dense-punctuation-burst.md`](docs/src/rules/dense-punctuation-burst.md) | Syntax | `general` | IFLA easy-to-read guidelines | 🟡 Later |

**Cut order if schedule slips:** F47 → F54 → F62 → F52 → F50 → F11. F55
and F56 are non-negotiable (trivial implementation cost, strong
grounding).

### Format support

| ID | Item | Priority | Origin |
|---|---|---|---|
| F5 | Native AsciiDoc support | 🟡 Later | Format scope v0.1 |
| F6 | Native HTML support | 🟡 Later | Relevant for EAA compliance |
| F7 | `.docx` support via Pandoc integration | 🟡 Later | FALC institutional target |
| F8 | Companion script `pandoc → lucid-lint` | 🟡 Later | Documented in v0.1 README |

### Documentation rules plugin

| ID | Item | Priority | Origin |
|---|---|---|---|
| F4 | `code-block-without-lang` rule | 🟡 Later | Rule 8 dropped from v0.1, candidate for `lucid-lint-docs` plugin |

### Docs site — bilingual

| ID | Item | Priority | Origin |
|---|---|---|---|
| F25 | French mirror of the mdBook docs (`/fr/` tree). First slice shipped 2026-04-22: translated `introduction` + `rules-index`, short FR `accessibility` and `roadmap` pages pointing at EN, SUMMARY sidebar entry. Per-rule FR pages still pending. | 🟡 In progress | v0.1 docs `/shape` session, bilingual-equality prime directive |

### Docs site — content

| ID | Item | Priority | Origin |
|---|---|---|---|
| F27 | ✅ Shipped in v0.2 — `docs/src/roadmap.md` is auto-generated from the root `ROADMAP.md` by [`scripts/sync-roadmap.py`](scripts/sync-roadmap.py). `just docs-build` / `just docs-serve` run the sync first, so the mdBook site always ships the current roadmap. Relative links are rewritten (targets under `docs/src/` become docs-relative; others become absolute GitHub URLs) so the `docs_links_stay_inside_docs` gate still passes. | 🔴 Next | v0.1 docs review |
| F28 | ✅ Shipped in v0.2 — one page per rule under `docs/src/rules/`, wired into `docs/src/SUMMARY.md`, enforced by [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs). Each page carries category, severity, default weight, parameters per profile, EN/FR examples where applicable, and suppression guidance. | 🔴 Next | v0.1 docs review |
| F29 | Rule numbering scheme based on category (e.g. `STR-001` for structural, `LEX-002` for lexical, `SYN-003` for syntactic). Stable IDs that survive renames, referenced from both diagnostics output and the docs. | 🟡 Later | v0.1 docs review |
| F30 | Audit every rule mention across the docs and link it to its reference page (F28). Requires F28 to land first. | 🟡 Later | v0.1 docs review |
| F42 | ✅ Shipped in v0.2 — rule documentation coverage gate. [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs) cross-checks every shipped rule id against its mdBook page, `Category::for_rule`, `scoring::WEIGHTED_RULE_IDS`, and (on CI, gated by `RULE_DOCS_GATE_GIT=1`) the `## [Unreleased]` section of `CHANGELOG.md`. Contract documented in [`CONTRIBUTING.md`](CONTRIBUTING.md#adding-or-modifying-a-rule--documentation-contract). | 🔴 Next | v0.2 interlude |
| F43 | ✅ Shipped in v0.2 — `RULES.md` category drift fixed. Per-rule `**Category**` lines and the Categories table now match `Category::for_rule`: `excessive-commas` and `deep-subordination` are `structure`, `repetitive-connectors` is `rhythm`, `unclear-antecedent` is `syntax`. The drift banners on the four per-rule mdBook pages are removed. | 🟡 Later | Surfaced by F42 interlude |
| F44 | Coverage test for F30 rule-mention linking — assert each rule id mentioned in `docs/src/**/*.md` is linked on first-per-section occurrence. Follow-up from F30. | 🟡 Later | F30 follow-up |

### Docs site — theming

| ID | Item | Priority | Origin |
|---|---|---|---|
| F26 | ✅ MVP shipped in v0.2 via DOM-level trim in `lucid-navigation.js` — the picker now shows three honest items (`Auto · Lucid light · Lucid dark`); the stock Rust / Navy / Ayu `<li>`s are marked `hidden` so they're inert for keyboard and screen-reader. CSS class mapping is unchanged (`.light` / `.rust` → lucid-light, `.coal` / `.navy` / `.ayu` → lucid-dark), so pre-existing localStorage selections still render correctly. Follow-up (optional): a full `index.hbs` override to drop the stock markup entirely rather than hide it; preferred once the mdBook upgrade cadence settles. | 🟡 Later | v0.1 docs `/colorize` session; mdBook stock limitation |
| F73 | ✅ Pre-deploy font-leak gate shipped in v0.2 — `just docs-check-clean` rebuilds the book, runs `scripts/sanitize-stock-css.py`, and greps the output for active `font-family` / `--*-font` / `local()` references to `Open Sans` or `Source Code Pro`. Not wired into `just check` (mdbook build is too slow for the dev loop); wire it into the docs-publish CI workflow before any release-candidate goes live. | 🟡 Later | v0.2 `/critique` polish pass follow-up |

### Docs site — reading preferences

| ID | Item | Priority | Origin |
|---|---|---|---|
| F33 | Full reading-preferences popover UI — cog button in the header opens a popover with font radio (Atkinson / Standard / OpenDyslexic), line-spacing slider (1.4–2.0, 0.05 step) and text-size slider (90–130 %, 5 % step). v0.1 ships only the Introduction-page demonstrator; the CSS-variable plumbing (`--reading-scale`, `--reading-line-height`, `[data-font]`) is already in place, so this is UI work only. | 🟡 Later | v0.1 docs `/shape` + `/typeset` sessions |
| F34 | Responsive / mobile adaptation — right-rail page TOC and header controls collapse gracefully below 700 px; touch targets verified ≥ 44 × 44 px; sidebar drawer behaviour polished. | 🔴 Next | v0.1 docs `/layout` session, deferred to `/adapt` |
| F35 | Accessibility audit sweep — full AAA pass on both themes (contrast, focus order, `prefers-reduced-motion` coverage, keyboard-only walk-through, skip-link), plus a published accessibility statement page. First audit pass ran 2026-04-22 (17/20, 0 P0, 2 P1, 3 P2); findings filed as F35a–F35d below. F35 stays open until the statement page ships and P1s are cleared. | 🟡 In progress | v0.1 docs `/audit` plan |
| F35a | **Server-render skip link + language switch in `theme/index.hbs`** (P1 × 2 from F35 audit). Today both live in `lucid-navigation.js` and only appear post-paint; `.impeccable.md` AAA bar requires the lang switch in the header markup, and WCAG 2.4.1 Bypass Blocks needs the skip link available without JS. Overriding `index.hbs` also unblocks the final part of F26 (collapse stock theme labels at the markup level rather than via DOM hide). | 🔴 Next | F35 audit 2026-04-22 |
| F35b | **Drop `role="radiogroup"`/`role="radio"` on reading-demo chips** (P2 from F35 audit). Current markup declares radiogroup semantics but the JS only binds `click` — arrow-key traversal is missing, so the ARIA contract is broken. Simpler fix is to switch to plain buttons with `aria-pressed` (the chips are preset toggles, not radios) rather than add a keyboard handler. | 🟡 Later | F35 audit 2026-04-22 |
| F35c | **`.lucid-stance__idea` reduced-motion rule strips the colour tint entirely** (P2 from F35 audit). `prefers-reduced-motion` users lose the pair-comparison idea-highlight feature, not just its animation. Move any transition/animation properties into the reduced-motion block and keep the static `background-color` in the base rule. Same pattern probably worth scanning across the other seven reduced-motion blocks. | 🟡 Later | F35 audit 2026-04-22 |
| F35d | **Publish an accessibility statement page** (`docs/src/accessibility.md` is an EN stub today, FR counterpart added 2026-04-22). Flesh out: the stated bar (WCAG 2.2 AAA), what's covered, known shortfalls (F35a/b/c pending), report route. Required by RGAA/EAA for accessibility-claiming sites. | 🔴 Next | F35 audit 2026-04-22 |
| F36 | Final polish pass — optical alignment, spacing rhythm, edge-state copy, favicon PNG fallback, social-card refinement, re-running `/critique` to verify the score moves above 30/40. | 🟡 Later | v0.1 docs `/polish` plan |

### Quality features

| ID | Item | Priority | Origin |
|---|---|---|---|
| F12 | Score evolution dashboard across runs | 🟢 Speculative | Rule 11, inspired by coverage reports |

### Scope control

File/directory discovery. Distinct from suppression (below): scope
control excludes inputs before they are scanned; suppression hides
diagnostics after scanning.

| ID | Item | Priority | Origin |
|---|---|---|---|
| F78 | ✅ Shipped in v0.2 — `exclude = [...]` glob list in `[default]` of `lucid-lint.toml` and `--exclude <GLOB>` CLI flag (comma-delimited, repeatable). Patterns match against paths relative to the walked root; matching directories are pruned, not descended. Explicit file args bypass exclusion. Backed by `globset`. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#excluding-paths-v02). `.lucidignore` (gitignore-style file) deferred to F78b if user demand surfaces. | — | Dogfood feedback 2026-04-21 |
| F78b | `.lucidignore` file (gitignore-style, with negations and nested files). Different crate (`ignore`) and a larger test matrix than the glob-list MVP. Ship only if users ask — the `exclude` list in `lucid-lint.toml` covers the dominant use case. | 🟢 Speculative | F78 deferral, 2026-04-21 |

### Suppression mechanism

v0.1 ships the minimal inline-disable directive (see brainstorm
`brainstorm/20260419-inline-disable-feature.md`). Extensions deferred:

| ID | Item | Priority | Origin |
|---|---|---|---|
| F18 | ✅ Block form shipped in v0.2: `<!-- lucid-lint-disable <rule-id> -->` … `<!-- lucid-lint-enable -->` silences one rule across every line in the scope. `enable` with no argument closes every open scope; with a rule id, closes only that rule's scope (so overlapping disables for different rules can nest). Unterminated `disable` extends to end-of-document. See [RULES.md → Suppressing diagnostics](RULES.md#suppressing-diagnostics). | 🔴 Next | v0.1 inline-disable brainstorm |
| F19 | ✅ Shipped in v0.2 — top-level `[[ignore]]` array-of-tables in `lucid-lint.toml`, each entry with a required `rule_id` silences every diagnostic for that rule across Markdown, plain text, and stdin. Unknown ids tolerated. Applied post-engine, pre-scoring, so scoring / rendering / exit-code logic all see the filtered view. Scope broadened from the roadmap's original "`.txt` and stdin" wording because a global filter is simpler and more useful; Markdown users can still prefer inline directives for local silencing. `reason` field tracked as F20. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#silencing-rules-globally-v02). | — | v0.1 inline-disable brainstorm |
| F20 | `reason="..."` field, optional in v0.1, surfaced in reports and optionally required via config | 🟡 Later | v0.1 inline-disable brainstorm |
| F21 | File-level directive (`disable-file`) and multi-rule lists | 🟡 Later | v0.1 inline-disable brainstorm |

---

## v0.3+ — Advanced plugins

### LLM-enhanced detection

| ID | Item | Priority | Origin |
|---|---|---|---|
| F16 | `lucid-lint-llm` plugin (LLM-as-Judge rules) | 🟢 Speculative | Research on existing tools |

The plugin would add rules like `unclear-antecedent-semantic` that use an LLM to detect semantic ambiguities the pattern-based heuristics miss.

Disabled by default due to non-determinism, API cost, and latency incompatible with pre-commit hooks.

### Advanced NLP

| ID | Item | Priority | Origin |
|---|---|---|---|
| F75 | `lucid-lint-nlp` plugin specification and scaffolding (Python subprocess or WASM-based). Replaces heuristic rules with POS- / dependency-tree- / anaphora-backed precise versions. | 🟡 Later | Rule-system-growth brainstorm (2026-04-20) |

Candidate rules for the plugin:

- POS-based `passive-voice` detection (replaces v0.1 heuristic)
- Full anaphora resolution for `unclear-antecedent`
- Dependency-tree-based `deep-subordination`
- Semantic similarity between adjacent sentences (discourse cohesion signal inspired by Coh-Metrix)

### New rules (v0.3 candidates)

Deferred from v0.2 because they require corpus work, lexicon builds, or
depend on earlier features (F9, F14). Naming uses the provisional
`category.rule-name` prefix pending F29.

| ID | Rule | Category | Tags | Grounding | Depends on |
|---|---|---|---|---|---|
| F46 | `lexicon.homophone-density` | Lexicon | `dyslexia` | BDA (dyslexia) | FR corpus tuning; ships as `info` |
| F49 | `structure.italic-span-long` | Structure | `dyslexia` | BDA | — |
| F51 | `structure.number-run` | Structure | `dyscalculia` | plainlanguage.gov | — |
| F53 | `readability.large-number-unanchored` | Readability | `dyscalculia`, `general` | CDC CCI | — |
| F57 | `syntax.parenthetical-depth` | Syntax | `adhd`, `general` | plainlanguage.gov, Hemingway | — |
| F58 | `syntax.front-loaded-subject-delay` | Syntax | `adhd`, `general` | plainlanguage.gov | FR corpus validation (dislocation FP risk) |
| F59 | `rhythm.pronoun-density` | Rhythm | `aphasia`, `general` | FALC | — |
| F60 | `rhythm.topic-shift-cluster` | Rhythm | `adhd`, `general` | Hemingway | May merge into F13 after corpus review |
| F61 | `lexicon.falc-idiom` | Lexicon | `aphasia`, `non-native` | IFLA, FALC | Curated bilingual idiom lexicon |
| F63 | `lexicon.vocabulary-rarity` | Lexicon | `non-native`, `general` | — | Frequency lexicon per language (Lexique.org for FR, COCA / Google-Books for EN). Tiered weights: `common` / `context-dependent` / `expert`. LLM-built fallback only. |
| F65 | `rhythm.forward-reference-heavy` | Rhythm | `adhd`, `general` | Working-memory load | — |
| F66 | `lexicon.acronym-distance-from-definition` | Lexicon | `adhd`, `non-native` | Memory decay | F9 (definition-aware abbreviation) |
| F67 | `syntax.complex-tense` | Syntax | `non-native`, `aphasia` | FALC tense restrictions | FR morphology primary; EN lighter |
| F68 | `syntax.impersonal-voice-heavy` | Syntax | `aphasia` | FALC direct-address rule | — |
| F69 | `syntax.address-inconsistency` | Syntax | `non-native`, `general` | Register consistency | FR primary (tu / vous); EN weaker (you / one) |

### Developer experience (v0.3)

| ID | Item | Priority | Origin |
|---|---|---|---|
| F73 | Differential diagnostics — `--compare=<ref>` CLI mode. Runs against two revisions of the same text(s) and reports score-delta + diagnostic-delta. Pitch: CI/PR comment framing ("this PR adds 2 warnings, removes 5, net −3"), inverting alarm fatigue the way coverage tools do. CLI + JSON + SARIF-run-comparison. No dashboard (that is F12). | 🟡 Later | Rule-system-growth brainstorm (2026-04-20). Depends on F14 stabilising. |

### Ecosystem interop

Motivation: lucid-lint and Markdown-syntax linters (markdownlint, Vale,
proselint, textlint) can flag the same line from different angles.
Cognitive-load rules that happen to share a substrate with a structural
check should stay shipped in core — users without markdownlint, users
who disabled the matching markdownlint rule, and users feeding
non-Markdown input (plain text, .docx via F7, HTML via F6) all rely on
lucid-lint for that coverage. The pain point is editor LSP sessions
where two servers report the same span with different severities and
different wording, not CLI pipelines where tools run sequentially.

Scope audit at 2026-04-20: after the `heading-jump` reframing (cognitive
"comprehension cliff" at skip ≥ 2 levels, distinct from MD001's strict
+1 rule), **`deeply-nested-lists` is the only lucid-lint rule that
remains functionally equivalent to a markdownlint rule (MD007)**. The
mechanism below is designed to scale — Vale, proselint, textlint
overlaps are likely as the rule set grows — rather than to solve a
single-rule problem.

| ID | Item | Priority | Origin |
|---|---|---|---|
| F77 | ✅ Shipped in v0.2 — `main.rs` now auto-discovers `lucid-lint.toml` walking up from the CWD (stopping at the nearest `.git` boundary) and applies `[default].profile`, `[default].conditions`, `[scoring]` via `ScoringFileConfig::into_scoring_config`, and `[rules.readability-score].formula`. New `--config <path>` flag overrides discovery. Precedence: built-in profile defaults → TOML → CLI flags. Per-rule TOML overrides beyond `readability-score` extend rule-by-rule as each `Config` gains `Deserialize`. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md). | 🔴 Next | F11 follow-up (2026-04-21) |
| F76 | Interop suppression mechanism. Rules declare overlapping external linter rules in their metadata (e.g. `Rule::external_overlaps() -> &[(Linter, &'static str)]`, enum `Linter::Markdownlint \| Vale \| Proselint \| Textlint`). Users opt in via `[interop] suppress_when = ["markdownlint"]` in `lucid-lint.toml` (CLI equivalent: `--interop-suppress=markdownlint`); opt-out is default, so coverage never silently drops. When active, affected rules are skipped at emission time with an info-level trace in `--verbose`. Ships CLI + LSP (the LSP path is the real motivator: two servers squiggling the same span with different severities and wording erodes trust in both). Only `deeply-nested-lists` qualifies at time of writing (MD007); framework is designed to scale to future overlaps. Non-goal: detecting whether the external linter is actually installed or configured — the config field is the signal. | 🟡 Later | Markdownlint-overlap scan (2026-04-20) |

### Research track

Bets that don't commit to a ship date. Tracked to ensure they're not
forgotten.

| ID | Item | Priority | Origin |
|---|---|---|---|
| F64 | `structure.paragraph-landmark-density` — reprise-points for attention-fragile readers. Research needed to define "landmark" (bold / italic / headers / list-starts / code spans?). | 🟢 Speculative | Rule-system-growth brainstorm (2026-04-20) |
| F70 | `structure.lede-buried` — journalistic inverted-pyramid check. Strong candidate for a future `lucid-lint-journalism` plugin rather than core. | 🟢 Speculative | Rule-system-growth brainstorm (2026-04-20) |
| F74 | Rule-discovery corpus project — mine writer-heavy git histories for patterns that authors repeatedly rewrite. Source of evidence-grounded rule proposals. Intern / student project scale. | 🟢 Speculative | Rule-system-growth brainstorm (2026-04-20) |

Additional research directions captured for posterity but not yet ID'd:

- **Reader-model scoring** — tiny local model predicts processing time
  and accuracy per paragraph; output is a cognitive-load heatmap.
  Deterministic at inference, data-hungry at training.
- **TTS / screen-reader prosody** rules — detect prosody breakdown
  (mid-sentence acronyms, awkward punctuation cadence). Needs a TTS
  corpus.
- **Cross-document terminology drift** — same concept named three ways
  across a corpus ("user" / "customer" / "client"). Requires
  multi-file analysis infrastructure; performance implications.
- **Eye-tracking corpus collaboration** — partnership with a reading
  lab to ground thresholds in behavioural data.
- **LSP server** — live diagnostics in editors; same core, different
  frontend.
- **`--fix` / quickfix suggestions** — safe rules only (e.g.
  `long-enumeration` → concrete list skeleton). Controversial for
  prose; needs guardrails.
- **`lucid-lint baseline`** — record per-project medians; rules flag
  regressions rather than absolutes (ESLint-style).
- **Profile composition** (`extends = "falc"`) — reduce duplication
  across projects.
- **Community rule-pack registry** — cargo-style publication of domain
  packs (medical, legal, edu, journalism).
- **`lucid-lint-style` plugin** — adverb overuse, show-don't-tell, and
  other aesthetic rules excluded from core by design.
- **`lucid-lint-a11y` plugin** — alternative home for `a11y-markup`-
  tagged rules if the tag proves insufficient to separate them from
  prose rules.

---

## Design decisions from v0.1 session

### Diagnostic structure

Decided: v0.1 diagnostics carry only what cannot be trivially recomputed.

```rust
pub struct Diagnostic {
    pub rule_id: String,
    pub severity: Severity,
    pub location: Location,
    pub section: Option<String>,  // H2 (or configured level) containing the diagnostic
    pub message: String,
}
```

**Kept** : `section` is stored at emission. Recomputing it a posteriori would require re-parsing the Markdown to walk headings and match locations. Expensive. Storing it is cheap.

**Omitted** : `category` is a pure function of `rule_id`. A `category_of(rule_id) -> Category` utility derives it in O(1). No duplication in diagnostics.

**Omitted** : `weight` and `suggestion` are not used in v0.1 and will be introduced when the hybrid scoring model (F14) lands.

This aligns with the "open to change, not abstracted for change" principle applied earlier to format handling: struct fields can be added later without breaking JSON serialization compatibility.

---

## Points deferred from v0.1 session

A number of configuration and ergonomics questions were raised but postponed. They will be addressed before or during v0.2:

### Configuration

- Config file format decision: TOML (recommended), YAML, or JSON
- Config filename convention
- Profile name finalization (`dev-doc`, `public`, `falc` confirmed)
- Naming convention for rules (kebab-case confirmed, flat vs. hierarchical namespace)
- Rule codes (short codes like `LL001` vs. name-only)
- Suppression mechanism (`# lucid-lint disable-next`, block disable/enable, ignore file)

### Output

- TTY format (colors, snippets, condensed report)
- Structured format: JSON schema, SARIF exactness, native format
- Exit code granularity (0/1 vs. graduated)

### Architecture

- Language detection: simple heuristic (stop-words) vs. dedicated crate (`whatlang`)
- Parallelism: `rayon` for multi-file processing
- Glob patterns and `.lucidignore` (now tracked as **F78**)
- Core library exposed as `lucid-lint-core` for third-party integration

### Project

- Repo structure: single crate vs. Cargo workspace
- Reference corpus for testing
- README v0.1 content and positioning
- Tagline and visual identity

---

## Contribution invitation

Future rules and plugins can be proposed by the community. The default jargon and stoplists (`jargon-undefined`, `weasel-words`, `low-lexical-diversity`) are especially welcome targets for community pull requests to expand coverage across domains and languages.
