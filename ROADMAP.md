# lucid-lint — Roadmap

> Future rules, refinements, and platform extensions tracked from v0.1 onwards.

**Status as of 2026-04-24:** v0.1 shipped 2026-04-20 (17 rules). v0.2.0
must-ship bundle shipped 2026-04-22 (25 rules + hybrid scoring + SARIF +
condition tags). v0.2.1 shipped 2026-04-23 and v0.2.2 shipped 2026-04-23
(FR nested-negation pair counter). The **v0.2.x patch cycle is active**;
v0.3 is scoped but not started; v0.4 is a horizon bet list.

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
| — | Shipped; priority meaningless once the item has landed |

---

## At a glance

Version-centric and topic-centric summary views. The sections below
this one are the authoritative topic-centric tables; use them when you
need origin, rationale, or full history. Use this section when you
need to answer "what's next?" or "what's the 0.3 shape?" in a glance.

### Version snapshot

| Version | State | Breaking? | Headline content |
|---|---|---|---|
| v0.1 | ✅ Released 2026-04-20 | — | 17 rules across 5 phases, minimal inline-disable, mdBook site with FR stub |
| v0.2.0 | ✅ Released 2026-04-22 | Yes (rule-id harmonisation) | Hybrid scoring (F14), SARIF (F32), condition tags (F71/F72), 8 new rules (25 total), F10 EN/FR auto-formula |
| v0.2.1 | ✅ Released 2026-04-23 | No | Localhost 404.html fix, 3rd per-rule TOML override, fixtures pipeline, TTY GIFs, v0.1/v0.2 prose sweep |
| v0.2.2 | ✅ Released 2026-04-23 | No | FR `syntax.nested-negation` pair-based counting |
| **v0.2.x** | 🚧 **In progress** | No | FR per-rule pages (F25), responsive (F34), P2 a11y (F35b/c), F84 part 2, perf (F102/F103 — F93/F94 refuted by profile 2026-04-25), hygiene (F95/F96), F15 project roll-up |
| **v0.3** | ☐ Scoped | **Yes** | F22 v0.3 slice, F10 remainder, 5 condition-tag rules (F46/F49/F51/F53/F57) |
| v0.4 | ☐ Horizon | Varies | LLM plugin (F16), alternative formats (F5–F8), feedback-driven items |

### Active work (🔴 Next)

Items actively queued — flattened across every topic table below so you
don't have to scan. Grouped by version for planning; each row links to
the authoritative entry in its topic section.

**v0.2.x (patch cycle — no breaking changes):**

| ID | Topic | Item |
|---|---|---|
| F15 | Architecture | Project-level scoring roll-up (per-file + summary) |
| F22 | Rules refinement | Parenthesised-list slice shipped; next slice deferred to 0.3 |
| F25 | Docs — bilingual | FR per-rule pages (11 / 25 shipped — `structure` category 100 % FR-complete) |
| F30 | Docs — content | Rule-mention linking pass |
| F34 | Docs — reading prefs | Responsive / mobile adaptation |
| F35b | Docs — reading prefs | Drop `role="radiogroup"` on reading chips (P2 a11y) |
| F35c | Docs — reading prefs | Reduced-motion colour-tint preservation (P2 a11y) |
| F84 | Example-text fixtures | Part 2 — redistributable replacements |
| F95 | Perf / hygiene | `.unwrap()` / `.expect(` audit |
| F96 | Perf / hygiene | Document `scoring.rs:203` cast invariant |
| F102 | Perf / hygiene | `detect_language` cost (7.5% of engine) |
| F103 | Perf / hygiene | Per-rule `split_sentences` re-parse (8 rules) |
| F104 | Docs — site | Per-category sidebar grouping in `SUMMARY.md` |
| F105 | Docs — content | Consolidated references page (cited sources, one click) |
| F107 | Docs — bilingual | FR rule labels (page subtitle + index gloss) |

**v0.3 (breaking boundary):**

| ID | Topic | Item |
|---|---|---|
| F10 | Rules refinement | SMOG / Dale-Chall / Scolarius / `--readability-verbose` |
| F22 | Rules refinement | v0.3 slice (3–4-word Oxford, non-Oxford, interleaved) |
| F46 | New rules (v0.3) | `lexicon.homophone-density` (slip-flag: FR corpus > 2 d → 0.3.x) |
| F49 | New rules (v0.3) | `structure.italic-span-long` |
| F51 | New rules (v0.3) | `structure.number-run` |
| F53 | New rules (v0.3) | `readability.large-number-unanchored` |
| F57 | New rules (v0.3) | `syntax.parenthetical-depth` |

### Topic heatmap

Where the active energy is. Counts include 🔴 Next only; shipped items
excluded.

| Topic | v0.2.x 🔴 | v0.3 🔴 | v0.4 bets | Later 🟡 | Speculative 🟢 |
|---|---|---|---|---|---|
| Rules (refinement) | 1 (F22 follow-up) | 2 (F10, F22) | — | F1, F13, F24 | F2, F3 |
| New rules | — | 5 (F46, F49, F51, F53, F57) | F65–F69, F63 | F58, F59, F60, F61 | F64, F70 |
| Architecture / scoring | 1 (F15) | — | F38, F41 | F17, F38, F39, F40 | F41 |
| Docs site (bilingual / content / theming / reading) | 4 (F25, F30, F34, F35b/c) | — | — | F36, F43, F44, F73, F89, F90/F91 | — |
| Example-text fixtures | 1 (F84 part 2) | — | F85, F86 | F81, F82, F83 | F86 |
| Performance / hygiene | 4 (F95, F96, F102, F103) | F97 | — | — | — |
| Suppression / config | — | F97 | — | F20, F21 | — |
| Formats | — | — | F5–F8 (single pick) | F5–F8 | — |
| Ecosystem interop | — | F76 | — | F76 | — |
| Plugins / NLP / LLM | — | F75 (Should) | F16, F75 | F75 | F16 |
| Developer experience | — | F88 (narrow `--fix`) | LSP server | F73-dx (`--compare`), F79 | F12 |
| Research track | — | — | F74, F101 (user feedback) | — | F64, F70, F74 |

### Cadence and gating

- **v0.2.x** is a **rolling patch cycle**, not a single release target.
  Each Must or Should ships as soon as it's green on `just check` + CI;
  any 🔴-tagged row is eligible to ride the next patch cut.
- **v0.3** opens only when the v0.2.x Must queue is empty and at least
  one breaking change is justified. Until then, breaking changes are
  held — non-breaking items that would otherwise fit 0.3 (e.g. F39
  letter grade) can slide into 0.2.x if they mature first.
- **v0.4** items do not progress by tenure. Each carries an **unlock
  signal** — a concrete event that promotes it from horizon to
  scheduled. See "v0.4 — horizon" at the bottom of this document.

---

## v0.1 — Released 2026-04-20

Shipped in the tag: all 17 rules across 5 phases, the minimal inline-disable directive, and the mdBook documentation site (Lucid light / Lucid dark themes, Atkinson Hyperlegible Next / Literata / Commit Mono / OpenDyslexic typography layer, reading-preferences demonstrator, accessibility page, EN/FR header switch with v0.2 FR-stub). See [`CHANGELOG.md`](CHANGELOG.md) for the full release notes.

### Rules (17 / 17) ✅

#### Phase 1 — Deterministic structural rules

| Status | Rule | Notes |
|---|---|---|
| ✅ | `structure.paragraph-too-long` | Sentence-count + word-count thresholds per profile (`src/rules/paragraph_too_long.rs`) |
| ✅ | `structure.deeply-nested-lists` | Flags list items nested beyond profile depth (`src/rules/deeply_nested_lists.rs`) |
| ✅ | `structure.heading-jump` | Walks section depths, flags jumps > +1 level (`src/rules/heading_jump.rs`) |

#### Phase 2 — Simple text rules

| Status | Rule | Notes |
|---|---|---|
| ✅ | `structure.sentence-too-long` | Reference implementation — template for the 15 others (`src/rules/sentence_too_long.rs`) |
| ✅ | `structure.excessive-commas` | Per-profile comma-per-sentence threshold (`src/rules/excessive_commas.rs`) |
| ✅ | `rhythm.consecutive-long-sentences` | Intra-paragraph streak of long sentences (`src/rules/consecutive_long_sentences.rs`) |

#### Phase 3 — Lexical rules with word lists

| Status | Rule | Notes |
|---|---|---|
| ✅ | `lexicon.weasel-words` | Per-language phrase list, word-boundary match (`src/rules/weasel_words.rs`) |
| ✅ | `lexicon.unexplained-abbreviation` | Pattern-based (v0.1); definition-awareness tracked as F9 (`src/rules/unexplained_abbreviation.rs`) |
| ✅ | `lexicon.jargon-undefined` | Pattern-based, profile-activated category lists (`src/rules/jargon_undefined.rs`) |
| ✅ | `lexicon.excessive-nominalization` | Per-sentence suffix-based density check (`src/rules/excessive_nominalization.rs`) |
| ✅ | `rhythm.repetitive-connectors` | Sliding-window connector frequency, one diagnostic per cluster (`src/rules/repetitive_connectors.rs`) |

#### Phase 4 — Global metric

| Status | Rule | Notes |
|---|---|---|
| ✅ | `readability.score` | Per-document Flesch-Kincaid grade; info under threshold, warning above (`src/rules/readability_score.rs`) |

#### Phase 5 — Heuristic rules (hardest)

| Status | Rule | Notes |
|---|---|---|
| ✅ | `structure.long-enumeration` | Shared enumeration detector with `structure.excessive-commas`; suggests list conversion (`src/rules/long_enumeration.rs`, `src/rules/enumeration.rs`) |
| ✅ | `structure.deep-subordination` | Counts subordinators between strong-punct breaks; skips pronoun enumerations (`src/rules/deep_subordination.rs`) |
| ✅ | `syntax.passive-voice` | Heuristic `be/être`+past-participle detector; POS-based detection remains a `lucid-lint-nlp` plugin candidate (`src/rules/passive_voice.rs`) |
| ✅ | `syntax.unclear-antecedent` | Info-level heuristic: bare demonstrative + verb, or paragraph-start personal pronoun (`src/rules/unclear_antecedent.rs`) |
| ✅ | `lexicon.low-lexical-diversity` | Sliding-window TTR over non-stopword content tokens (`src/rules/low_lexical_diversity.rs`) |

### Cross-cutting features

| Status | Feature | Notes |
|---|---|---|
| ✅ | Minimal inline-disable | `<!-- lucid-lint disable-next-line <rule-id> -->` for Markdown inputs, single rule id, optional reason. See [RULES.md → Suppressing diagnostics](RULES.md#suppressing-diagnostics). Block form, config ignores, file-level scope and required `reason=` are tracked as F18–F21 below. |
| ✅ | Accessibility page in the docs | `docs/src/accessibility.md` covers the WCAG 2.2 AAA bar, the reading-preferences control, typography credits (Atkinson Hyperlegible Next — Braille Institute; OpenDyslexic — Abelardo Gonzalez; Literata — TypeTogether), keyboard shortcuts, and how the site dogfoods the project's mission. Linked from the sidebar and the footer. |

---

## v0.2 / v0.2.x — Must-ship shipped, patch cycle in progress

### Release cadence

The 2026-04-22 reprioritisation favoured a **tight 0.2.0 cut over a
fat one**: anything non-blocking slides to 0.2.x patch releases, which
exist precisely to absorb per-rule polish and per-surface slices.
v0.2.0, v0.2.1, and v0.2.2 are shipped; v0.2.x remains open as a
rolling patch cycle. 0.2.x routing was reviewed on 2026-04-24
in `.personal/brainstorm/20260424-next-cycles.md` (not tracked;
`.personal/` is gitignored).

### v0.2.0 — Blocking items (all ✅ shipped 2026-04-22)

| ID | Summary |
|---|---|
| F29-slim | Rule IDs moved to `category.rule-name` form (25 rules); `src/rules/<cat>/` subdirectories; `Category::for_rule` derives from prefix. Hard break — suppression directives, `[rules.<id>]` TOML keys, JSON/SARIF `ruleId` all use the new form. |
| F35a | `theme/index.hbs` forked from upstream mdBook; skip link + EN / FR switch server-rendered. WCAG 2.4.1 Bypass Blocks passes with JS disabled. |
| F35d | Accessibility statement page (`docs/src/accessibility.md` + FR counterpart). |
| F80 | `--fail-on-warning` accepts optional boolean; hidden mirror `--no-fail-on-warning`. `--min-score` now testable in isolation on documents with warnings. |

### v0.2.1 — ✅ Released 2026-04-23

Localhost 404.html rendering fix (F84 part 1), per-rule TOML override
for `structure.excessive-commas` (third rule wired after
`readability.score.formula` and `lexicon.unexplained-abbreviation.whitelist`),
scraped-prose fixtures pipeline (`examples/texts.yaml` + `just texts`),
TTY-capture GIFs via `vhs` tapes, v0.1 / v0.2 staleness sweep, idea-highlight
motif extended to the `structure.sentence-too-long` rule page. First
crates.io publish since v0.1.1 — packaging switched from `exclude` to
an explicit `include` list so `docs/src/rules/*.md` reach the tarball
(needed by `src/explain.rs`'s `include_str!`).

### v0.2.2 — ✅ Released 2026-04-23

F87 — FR `syntax.nested-negation` pair-based counting over `ne` / `n'`
clitics and second-position particles (`pas`, `rien`, `jamais`, …).

### v0.2.x — MoSCoW routing (patch cycle, post-release)

Routed 2026-04-24 from the active-work view. Each row here has a
full entry under a topic section below; priority column reflects the
routing decision.

#### Must — 🔴 Next

| ID | Topic | Item |
|---|---|---|
| F25 | Docs — bilingual | Per-rule FR pages + guides (11 / 25 shipped — `structure` category 100 % FR-complete) |
| F34 | Docs — reading prefs | Responsive / mobile adaptation |
| F35b | Docs — reading prefs | Drop `role="radiogroup"` on reading chips (P2 a11y) |
| F35c | Docs — reading prefs | Reduced-motion colour-tint preservation (P2 a11y) |
| F84 part 2 | Example-text fixtures | Redistributable replacements for load-bearing slots |
| F95 | Perf / hygiene | `.unwrap()` / `.expect(` audit in library code |
| F96 | Perf / hygiene | Document `scoring.rs:203` cast invariant |
| F102 | Perf / hygiene | `detect_language` cost (7.5% of engine, samply 2026-04-25) |
| F103 | Perf / hygiene | Per-rule `split_sentences` re-parse — move to parser phase, share across 8 rules |

#### Should — ships as the next patch absorbs it

| ID | Topic | Item |
|---|---|---|
| F15 | Architecture | Project-level scoring roll-up (per-file + summary) |
| — | Suppression / config | Per-rule TOML plumbing, rule-by-rule as each `Config` gains `Deserialize` |
| F20 | Suppression / config | `reason="..."` field on suppression directives |
| F30 | Docs — content | Rule-mention linking audit + coverage test (F44) |

#### Could — nice-to-have

F24 (nominalization suffix refine), F43 (RULES.md drift cleanup), F73
(font-leak CI gate), F36 (final polish pass), F79 (fancy `explain`
rendering), F21 (`disable-file`), F81 / F82 / F83 (fixture hygiene).

#### Won't (pushed to 0.3)

F39 letter grade, F40 traffic light, F17 per-family sub-scores, F89
`.lucid-stance` unify, F88 `--fix` mode (narrow).

### v0.3 and later (already scoped)

Detail under "New rules (v0.3 candidates)" and the `## v0.4 — horizon`
section below.

- **F22 v0.3 slice** — 3–4-word Oxford items, non-Oxford / "plus"-closed
  lists, interleaved parentheticals (first slice shipped in 0.2.x).
- **F10 remainder** — SMOG, Dale-Chall, Scolarius,
  `--readability-verbose`.
- **Five condition-tag rules** — F46, F49, F51, F53, F57. F46 carries a
  slip-flag: if FR corpus tuning for homophone density exceeds ~2 days,
  it slides to 0.3.x.
- **Full F29** — demoted to 🟢 Speculative on 2026-04-24. F29-slim
  already fixed the category-drift problem by construction; numeric
  codes (`STR-001`) only earn their cost on a real rename, and there are
  zero scheduled renames. Revisit when one actually happens.

### Architecture

| ID | Item | Priority | Origin |
|---|---|---|---|
| F14 | ✅ Hybrid scoring model shipped in v0.2 (global score + per-category sub-scores + diagnostics). `X/max` arbitrary-max at both levels, 5 fixed categories (Structure · Rhythm · Lexicon · Syntax · Readability), composition = weighted sum × density-normalization × per-category cap, `weight` field added to `Diagnostic`, `--min-score=N` CLI flag. See [`docs/src/guide/scoring.md`](docs/src/guide/scoring.md). Letter-grade / traffic-light / reading-time decorations deferred (F39–F41). | — | Architecture decision discussion |
| F15 | 🚧 Document-level scoring shipped in v0.2 (multi-path runs are aggregated as one document). Project-level roll-up (per-file breakdown + project summary) still open. Section-level deferred → F38. | 🔴 Next | Linked to F14 |
| F17 | Per-family sub-scores | 🟡 Later | Linked to F14 |
| F32 | ✅ Shipped in v0.2 — `lucid-lint check --format=sarif` emits a SARIF v2.1.0 log compatible with GitHub Code Scanning. One rule descriptor per observed rule id (category, default severity, default weight, `helpUri` to the per-rule mdBook page); per-result properties carry weight + section. Workflow snippet in [`docs/src/guide/ci-integration.md`](docs/src/guide/ci-integration.md#github-code-scanning-sarif). | — | v0.1 AGENTS.md audit |
| F37 | ✅ Rule-message clarity audit completed: all 17 rules reviewed against "what do I change?" bar. 15 rules already actionable; `structure.heading-jump` updated (first-heading-not-H1 and missing-H1 variants now include repair guidance). `readability.score` info variant left observational by design (fires only when `always_report` is set). | — | F14 `brainstorm/20260420-score-semantics.md` |
| F38 | Section-level granularity for scoring (deferred from F15) — per-heading sub-scores once document + project are proven in the wild. | 🟡 Later | F14 `brainstorm/20260420-score-semantics.md` |
| F39 | Letter-grade decoration (A–F) on the `X/max` score — promote when user feedback shows the numbers feel noisy or hard to compare across docs. | 🟡 Later | F14 `brainstorm/20260420-score-semantics.md` |
| F40 | Traffic-light (🔴🟡🟢) + pass/fail margin in the TTY output — promote when CI users ask for a stronger glance signal than the number alone. | 🟡 Later | F14 `brainstorm/20260420-score-semantics.md` |
| F41 | Reading-time-seconds as an alternative score unit — ties score to concrete user outcome. Requires validated heuristic + companion metrics (comfort, fatigue, understandability) so the time unit doesn't monopolize the read. | 🟢 Speculative | F14 `brainstorm/20260420-score-semantics.md` |
| F71 | ✅ Shipped in v0.2 — `ConditionTag` enum (fixed 7-variant ontology: `a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`, `non-native`, `general`) plus `Rule::condition_tags()` trait method (default `&[General]`). All 17 v0.2 rules are `general`; future tagged rules (F48, F55, F56) opt in by overriding. See [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md). | — | Rule-system-growth brainstorm (2026-04-20) |
| F72 | ✅ Shipped in v0.2 — `[default] conditions = [...]` config field and `--conditions` CLI flag (comma-separated). Filter semantics: rules tagged `general` always run; tagged-only rules run iff their tags intersect the active list. Profiles unchanged; FALC retains its regulatory meaning. See [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md). | — | Rule-system-growth brainstorm (2026-04-20) |

### Rules refinement

| ID | Item | Priority | Origin |
|---|---|---|---|
| F9 | ✅ Shipped in v0.2 — definition-aware `lexicon.unexplained-abbreviation` is now two-pass. A pre-scan collects acronyms defined anywhere in the document in either canonical form (`Expansion (ACRONYM)` or `ACRONYM (Expansion)`; expansion side ≥ 2 alphabetic words to reject `(TBD)`-shaped noise), and a single definition silences every occurrence of that token. Silencing precedence: defined-in-doc → user whitelist → baseline. See [`docs/src/rules/unexplained-abbreviation.md`](docs/src/rules/unexplained-abbreviation.md). | — | Rule 10 simplified in v0.1 |
| F10 | 🚧 Must-ship slice shipped in v0.2 — `readability.score` auto-selects the formula by detected language: Flesch-Kincaid for EN (kept), Kandel & Moles (1958) for FR. Kandel-Moles ease scores are converted to a grade-equivalent so per-profile `max_grade_level` stays comparable across languages. Unknown language → Flesch-Kincaid. See [`docs/src/rules/readability-score.md`](docs/src/rules/readability-score.md). Still open: Gunning Fog / SMOG / Dale-Chall (EN), Scolarius / Flesch-Kandel (FR), `--readability-verbose` multi-formula reports, per-file override (covered by F11). | 🟡 Later | Rule 11 simplified in v0.1; scope expanded in rule-system-growth brainstorm (2026-04-20) |
| F11 | ✅ Shipped in v0.2 — `--readability-formula {auto,flesch-kincaid,kandel-moles}` CLI flag + `FormulaChoice` enum on `readability_score::Config` + `Engine::with_readability_formula(choice)`. `auto` (default) keeps F10 per-language selection; `flesch-kincaid` / `kandel-moles` pin a formula for cross-document comparison. TOML config wiring is tracked separately as F77. | 🟡 Later | Rule 11 |
| F13 | `missing-connectors` rule (15b not shipped in v0.1) | 🟡 Later | Rule 15 decomposition |
| F1 | Custom stoplist parameter for `lexicon.low-lexical-diversity` | 🟡 Later | Rule 5 |
| F2 | Sentence-level low-lexical-diversity density | 🟢 Speculative | Rule 5 |
| F3 | Comma density metric (relative) for `structure.excessive-commas` | 🟢 Speculative | Rule 3a |
| F22 | 🚧 First slice shipped in v0.2.x — `structure.excessive-commas` now discounts commas inside `(A, B, C, …)` parenthesised token lists (3+ short comma-separated segments inside balanced parens, language-agnostic). Sibling helper `parenthesised_list_comma_count` in `src/rules/enumeration.rs`. Dogfood drops from 25 → 15 hits (10 FPs killed, ~40% reduction). Deferred to v0.3: relaxing `MAX_SEGMENT_WORDS = 2` for 3–4-word Oxford items, non-Oxford / "plus"-closed lists, interleaved parentheticals inside Oxford runs. See research note in `.personal/research/F22.md`. | 🔴 Next | v0.1 dogfood: 5 false-ish positives on technical docs |
| F23 | ✅ Shipped in v0.2 — false-positive cleanup complete for v0.2. Hits inside inline code spans, straight `"..."` quotes, paired curly `"..."` quotes, and directional `rather than` / `plutôt que` pairings are now skipped. Single quotes / apostrophes are deliberately not recognised (possessives, contractions, FR elisions). The "concrete noun" semantic check (`"many X"` where X is a concrete noun) stays unshipped — needs POS data and belongs in the `lucid-lint-nlp` plugin (F75) rather than the deterministic core. | — | v0.1 dogfood: 11 false-ish positives on this repo's own docs |
| F24 | Refine `lexicon.excessive-nominalization` suffix list (drop or gate `-al`; many adjectives — `crucial`, `horizontal`, `positional`, `attentional` — are flagged despite not being abstract nouns) | 🟡 Later | v0.1 dogfood |
| F87 | ✅ Shipped in 0.2.x — FR `syntax.nested-negation` now uses pair-based counting over `ne` / `n'` clitics and the second-position particles `pas`, `rien`, `jamais`, `plus`, `personne`, `aucun`, `aucune`, `guère`, `nulle part`. Each clitic contributes one negation and consumes its nearest particle within a 6-token window; unpaired particles in a `ne`-sentence contribute one more — so `Nous ne disons pas que rien n'est jamais possible` now counts as 3 (was 2). Guards: `pas` / `plus` never count when unpaired, `de rien` idiom is skipped, particles in ne-less sentences are skipped. Fixture at `tests/corpus/fr/nested-negation.md` anchors the behaviour. | — | 2026-04-23 docs clarity session — FR pedagogical example surfaced the detection gap |
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
- **Interaction with `structure.long-enumeration`**: the shared
  `enumeration::detect_enumerations` helper already discounts Oxford-
  style enumeration commas from `structure.excessive-commas` (3+ short items).
  F22 is specifically about the cases that helper still misses:
  parentheticals, post-colon lists, and non-Oxford enumerations
  ("A, B, C and D" without the final comma).

Research inputs to gather before deciding: FR/EN corpus samples of
technical docs, a handful of real false positives from dogfooding and
downstream projects, how `textlint` / Vale / `write-good` handle
parentheticals. Decide between relaxation parameters vs. a smarter
token-aware counter.

### Performance / hygiene (0.2.x)

Findings filed from the 2026-04-24 code-review stream-2 pass on
`src/`. Each has a concrete source reference so it survives past the
`.personal/<date>-today.md` scratchpad.

| ID | Item | Priority | Origin |
|---|---|---|---|
| F93 | **Parser hot-path allocations.** `src/parser/mod.rs:43` (`Paragraph::new(trimmed.to_string(), …)`) and `src/parser/tokenizer.rs:~88/109` (`current.trim().to_string()` per sentence) allocate in hot loops. ~~Confirm constructors accept `impl Into<String>`; pass the already-owned buffer where possible.~~ **Refuted by samply profile 2026-04-25**: `Paragraph::new` does not appear in the profile; `to_string()` in tokenizer = 3 samples / 0.03%. Real hot spots are F102 (`detect_language` 7.5%) and F103 (per-rule `split_sentences`). | ✅ Done (refuted) | 2026-04-24 code review (stream-2 #3); refuted 2026-04-25 |
| F94 | **Tokenizer `Vec<char>` per sentence.** `src/parser/tokenizer.rs:~60` collects a full `Vec<char>` for lookahead. ~~Swap to `Peekable<CharIndices>`.~~ **Refuted by samply profile 2026-04-25**: `Vec<char>` drop = 3 samples / 0.03% on the engine path. Yesterday's "low ceiling" note (~5%) was generous; real ceiling is ~0.1%. Skip. | ✅ Done (refuted) | 2026-04-24 code review (stream-2 #5); refuted 2026-04-25 |
| F102 | **`detect_language` cost.** Single function showed 7.5% inclusive in samply profile 2026-04-25. Rewrote as single-pass, alloc-light: scalar counters, `to_lowercase()` only for words containing an uppercase character, no intermediate vectors. Bench delta on `engine_lint_str/en_long_devdoc` vs `stream2-noisy`: **−0.56 % (p = 0.00, ~20 µs)** — smaller than profile suggested because most of the inclusive cost is `unicode_words()` itself, which the rewrite cannot touch. | ✅ Done | 2026-04-25 samply profile; landed 2026-04-25 |
| F103 | **Per-rule `split_sentences` re-parse.** 8 rules called `split_sentences(&paragraph.text, …)` directly. Moved sentence splitting into `Paragraph::new`; rules now read `&paragraph.sentences`. Bench delta vs `stream2-noisy`: **`engine_lint_str/en_long_devdoc` −11.58 % (~394 µs)**; `parse_markdown/en_long` +17.67 % (~38 µs, intentional — split cost moved into the parser phase, where it pays for itself across the eight consumers). Net user-facing win ~360 µs. New baseline saved as `stream2-after-f103`. | ✅ Done | 2026-04-25 samply profile; landed 2026-04-25 |
| F95 | **Audit `.unwrap()` / `.expect(` in library code.** Grep `src/rules/` and `src/parser/` (flagged candidate: `parser/tokenizer.rs:177`). Replace dictionary-lookup unwraps with `.contains()` / `.get().is_some_and()` per AGENTS.md. | 🔴 Next | 2026-04-24 code review (stream-2 #2) |
| F96 | **Document the `scoring.rs:203` cast invariant.** Replace `#[allow(clippy::cast_possible_truncation, …)]` with a one-liner stating the `[0, cap]` clamp that makes the cast safe, so a future edit can't loosen the clamp silently. | 🔴 Next | 2026-04-24 code review (stream-2 #1) |
| F97 | **Config whitelist normalization at load time.** `src/config.rs` — normalize (trim, case-fold per rule needs) on load instead of per invocation; catches user typos early. Small win; fits a v0.3 config-plumbing pass rather than a 0.2.x patch. | 🟡 Later | 2026-04-24 code review (stream-2 #6) |

### New rules (v0.2)

New rule candidates raised in the rule-system-growth brainstorm
(2026-04-20). Naming uses a provisional `category.rule-name` prefix
pending F29 harmonisation. Grounding column points at the standard or
research that justifies the rule.

**Must-ship v0.2 (blocking release):**

| ID | Rule | Category | Tags | Grounding | Priority |
|---|---|---|---|---|---|
| F48 | ✅ `lexicon.all-caps-shouting` shipped in v0.2 — see [`docs/src/rules/all-caps-shouting.md`](docs/src/rules/all-caps-shouting.md) | Lexicon | `a11y-markup`, `dyslexia`, `general` | WCAG 3.1.5, BDA Dyslexia Style Guide | — |
| F55 | ✅ `syntax.nested-negation` shipped in v0.2 — see [`docs/src/rules/nested-negation.md`](docs/src/rules/nested-negation.md) | Syntax | `aphasia`, `adhd`, `general` | FALC, CDC Clear Communication Index | — |
| F56 | ✅ `syntax.conditional-stacking` shipped in v0.2 — see [`docs/src/rules/conditional-stacking.md`](docs/src/rules/conditional-stacking.md) | Syntax | `aphasia`, `adhd`, `general` | FALC, plainlanguage.gov | — |

**Should-ship v0.2 (cuttable under time pressure, in suggested cut order):**

| ID | Rule | Category | Tags | Grounding | Priority |
|---|---|---|---|---|---|
| F62 | ✅ `lexicon.redundant-intensifier` shipped in v0.2 — see [`docs/src/rules/redundant-intensifier.md`](docs/src/rules/redundant-intensifier.md) | Lexicon | `general` | Plain-language guides | 🟡 Later |
| F52 | ✅ `structure.mixed-numeric-format` shipped in v0.2 — see [`docs/src/rules/mixed-numeric-format.md`](docs/src/rules/mixed-numeric-format.md) | Structure | `dyscalculia`, `general` | CDC Clear Communication Index | 🟡 Later |
| F50 | ✅ `structure.line-length-wide` shipped in v0.2 — see [`docs/src/rules/line-length-wide.md`](docs/src/rules/line-length-wide.md) | Structure | `dyslexia`, `general` | WCAG 1.4.8 (AAA) | 🟡 Later |
| F47 | ✅ `lexicon.consonant-cluster` shipped in v0.2 — see [`docs/src/rules/consonant-cluster.md`](docs/src/rules/consonant-cluster.md) | Lexicon | `dyslexia`, `general` | BDA Dyslexia Style Guide | 🟡 Later |
| F54 | ✅ `syntax.dense-punctuation-burst` shipped in v0.2 — see [`docs/src/rules/dense-punctuation-burst.md`](docs/src/rules/dense-punctuation-burst.md) | Syntax | `general` | IFLA easy-to-read guidelines | 🟡 Later |

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

### Example-text fixtures

Scraper + cleaner + converter triplet under `scripts/texts_*.py`
populates `examples/public/` (committable `public_ok` sources) from
`examples/texts.yaml`. First batch landed 21 fixtures. The follow-ups
below close the remaining rough edges.

| ID | Item | Priority | Origin |
|---|---|---|---|
| F81 | Per-source adapters for git-cloned upstreams. The generic `clean` / `convert` path doesn't know how to extract text from shallow-cloned repos (proselint checks, Vale style packs, write-good / alex / retext / textlint-rule fixtures, ASSET / OneStopEnglish / EASSE / CLEAR-corpus datasets). Each needs a small extractor that walks the repo and emits one or more `.md` files per rule / excerpt. | 🟡 Later | First scraper batch, 2026-04-22 |
| F82 | Refine `texts_convert._split_before_after`. The current heuristic looks for literal `## Before` / `## After` (EN/FR) headings; no upstream page in the current batch uses that shape, so every `before_after` source fell back to a single `content.md` with a warning. Replace with a per-source pair-extraction rule (plainlanguage.gov, EC *How to write clearly*, Canada.ca, OneStopEnglish, ASSET, Inclusion Europe) that emits `before.md` + `after.md`. | 🟡 Later | First scraper batch, 2026-04-22 |
| F83 | Maintenance pass on `examples/texts.yaml` URLs. 12 sources failed on the first batch — 404s from moved landing pages (canada.ca × 2, BDA Dyslexia, Center for Plain Language, Newsela, HuggingFace wiki_auto), UA-/bot-blocking (Légifrance 403, Orthodidacte 403, ADHD Foundation 400), and a DNS error for the specific 18F post. Audit and update entries; for sources that genuinely require a browser-flavoured UA, add a per-source override in the fetcher. Fold in the opportunistic hygiene tasks from the 2026-04-23 brainstorm: (a) dedupe overlapping canada.ca / plainlanguage.gov entries, (b) add a licence-drift guard that flags when a source's `redistribution` changes between fetches. | 🟡 Later | First scraper batch, 2026-04-22 + referential brainstorm, 2026-04-23 |
| F84 | Desired-fixture-shapes coverage table + replacements for high-value local-only entries. **Part 1 — coverage tables:** ✅ Shipped (2026-04-23) — `scripts/texts_coverage.py` splits output by audience: the committed `examples/texts.md` shows `public_ok` counts only (no totals, no names that would leak local-only existence), spliced between `<!-- coverage:begin/end -->` markers; the gitignored `examples/local/COVERAGE.md` carries the full matrices plus the load-bearing local-only list. Wired as `just texts-coverage` / `just texts-coverage-check`. **Part 2 — replacement hunting:** 🔴 Next (promoted 2026-04-24). The load-bearing list flags four slots with zero redistributable sources — hunt for open-licence equivalents so the coverage honesty at `public` is not a hollow victory. | 🔴 Next (part 2) | Referential brainstorm, 2026-04-23 |
| F85 | Bidirectional rule ↔ fixture coverage map. Generate `examples/COVERAGE.md` from each `content.md`'s `rules_relevant` frontmatter, rendered as two views: rule → fixtures that exercise it (surfaces under-fixtured rules) and fixture → rules it covers (surfaces untagged or mis-tagged fixtures). Once stable, embed or link the canonical fixture per rule from `docs/src/rules/<rule-id>.md`. Optional follow-up: calibrated snapshot tests that lock expected lint output per canonical fixture. | 🟡 Later | Referential brainstorm, 2026-04-23 |
| F86 | Auto-discovery of new references with triage queue. Crawler (sitemaps, RSS, GitHub search, ACL Anthology API) surfaces candidate sources against a relevance filter derived from `rules_relevant` keywords; a lightweight triage file lists candidates with accept / ignore / defer. Mini-product — revisit post-v0.3 once the referential has stabilised. | 🟢 Speculative | Referential brainstorm, 2026-04-23 |

### Documentation rules plugin

| ID | Item | Priority | Origin |
|---|---|---|---|
| F4 | `code-block-without-lang` rule | 🟡 Later | Rule 8 dropped from v0.1, candidate for `lucid-lint-docs` plugin |

### Docs site — bilingual

| ID | Item | Priority | Origin |
|---|---|---|---|
| F25 | French mirror of the mdBook docs (`/fr/` tree). First slice shipped 2026-04-22: translated `introduction` + `rules-index`, short FR `accessibility` and `roadmap` pages pointing at EN, SUMMARY sidebar entry. Second slice shipped post-0.2.1 (2026-04-23): `fr/rules-index.md` renamed to `fr/rules/index.md` for EN-parity, first FR per-rule page landed (`structure.sentence-too-long`), parallel-version sidebar and EN↔FR deep-link toggle (F90 plan slot A, F92). Third slice shipped 2026-04-24: four more FR per-rule pages landed (`structure.excessive-commas`, `structure.long-enumeration`, `lexicon.weasel-words`, `lexicon.unexplained-abbreviation`), locked template honoured, `SUMMARY.md` + `fr/rules/index.md` rewired to point at the local FR versions. Fourth slice shipped 2026-04-25: six more FR per-rule pages landed (`structure.paragraph-too-long`, `structure.line-length-wide`, `structure.mixed-numeric-format`, `structure.deeply-nested-lists`, `structure.heading-jump`, `structure.deep-subordination`), closing out the `structure` category (9 / 9 rules FR-complete). Remaining: 14 per-rule FR pages (rhythm 2, lexicon 6, syntax 5, readability 1) + FR guide translations. | 🟡 In progress | v0.1 docs `/shape` session, bilingual-equality prime directive |
| F90 | Split `SUMMARY.md` per locale (EN + FR) via a small preprocessor. v0.2.1 ships the single-`SUMMARY.md` + CSS `:has()` locale-hiding approach (1.A); both language trees coexist in the built HTML and each viewer only sees theirs. A clean separation would maintain `SUMMARY.en.md` + `SUMMARY.fr.md` and stitch them at build. Benefit: smaller per-page sidebar payload; clearer authoring story; no `:has()` browser-support floor. Cost: build-time stitcher, tooling to keep the two files in pair-sync. File when the FR tree outgrows the hide-via-CSS approach. | 🟢 Speculative | 2026-04-23 FR per-rule pages session |
| F91 | Multi-book mdBook layout (one book per locale). The truest "parallel version" — `/` redirects to `/en/`, `/fr/` is its own mdBook with its own theme inheritance. Benefit: each locale has its own table of contents, its own search index, its own navigation neighbour hints; no cross-locale bleed in any surface. Cost: biggest surgery — book.toml per locale, build orchestration, shared theme / asset de-duplication, sitemap updates, redirects. Revisit only if F90 isn't enough. | 🟢 Speculative | 2026-04-23 FR per-rule pages session |
| F92 | ✅ Shipped post-0.2.1 (2026-04-23) — `scripts/sync_lang_counterparts.py` walks `docs/book/**/*.html` after `mdbook build` and rewrites both `hreflang="en"` and `hreflang="fr"` anchors so the lang-switch deep-links to the matching page (e.g. `/fr/rules/sentence-too-long.html` ↔ `/rules/sentence-too-long.html`). Wired into `just docs-build`, the Deploy-docs workflow, and a new `just docs-lang-check` CI gate that runs with `--check` and fails on orphaned FR pages (FR without EN counterpart). The invariant is asymmetric by design: EN is canonical, FR is a translation layer — untranslated EN pages are informational and tracked as F25, not gated. No front-matter flag yet; add a `counterpart: none` flag only when a truly asymmetric page appears. | — | 2026-04-23 FR per-rule pages session, option 2.B |
| F107 | **FR rule labels (page subtitle + index gloss).** Every FR rule page opens with the H1 `structure.sentence-too-long` (rule ID in backticks) and the FR sidebar shows the same English ID — a FR-only reader sees no FR label anywhere. Hard constraint: the rule ID is a stable contract (CLI flags, `--format=json`, config keys, citations); it must not be renamed or aliased. Two-part fix: (1) **page subtitle** — keep the ID as the H1, add a small FR human-readable subtitle directly below (e.g. via a `<p class="lucid-rule-subtitle">` or italic gloss line, like `*Phrase trop longue.*`); (2) **index gloss** — extend the FR `rules/index.md` per-category table with a `Libellé FR` column (or inline gloss) so the catalog reads naturally. Skip translating the sidebar TOC labels — that would force a per-locale `SUMMARY.md` (which is F90, parked Speculative) and create drift risk between sidebar and H1. Effort: medium — define the subtitle convention once, apply to the 11 FR pages already shipped, propagate to the remaining 14 as they land. | 🟡 Later | 2026-04-25 docs UX critique (Block E) |

### Docs site — content

| ID | Item | Priority | Origin |
|---|---|---|---|
| F27 | ✅ Shipped in v0.2 — `docs/src/roadmap.md` is auto-generated from the root `ROADMAP.md` by [`scripts/sync-roadmap.py`](scripts/sync-roadmap.py). `just docs-build` / `just docs-serve` run the sync first, so the mdBook site always ships the current roadmap. Relative links are rewritten (targets under `docs/src/` become docs-relative; others become absolute GitHub URLs) so the `docs_links_stay_inside_docs` gate still passes. | — | v0.1 docs review |
| F28 | ✅ Shipped in v0.2 — one page per rule under `docs/src/rules/`, wired into `docs/src/SUMMARY.md`, enforced by [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs). Each page carries category, severity, default weight, parameters per profile, EN/FR examples where applicable, and suppression guidance. | — | v0.1 docs review |
| F29 | Rule ID harmonisation. **F29-slim** ✅ shipped 2026-04-22 in v0.2.0: the 25 rule IDs now use `category.rule-name` form (`structure.excessive-commas`, `lexicon.weasel-words`, `readability.score`, …) and rule source files moved into category subdirectories under `src/rules/<cat>/`. `Category::for_rule` derives the category from the id prefix rather than a hand-maintained match arm (F43-style drift now impossible by construction). Hard break — suppression directives, `[rules.<id>]` TOML keys, JSON/SARIF `ruleId` fields all use the new form; no alias layer. mdBook filenames and docs URLs still use the flat kebab slug; docs-tree rearchitecture into category subdirs is a separate slice. **F29-full** (parked 2026-04-24) would add a stable category-numbered code (`STR-001`, `LEX-002`, `SYN-003`) that survives renames — slim already makes drift impossible by construction, and numeric codes only earn their cost on a real rename. Revisit only when a rename actually happens. | — (slim) / 🟢 Speculative (full) | v0.1 docs review; 2026-04-22 reprioritisation; 2026-04-24 brainstorm-next-cycles |
| F30 | Audit every rule mention across the docs and link it to its reference page (F28). Requires F28 to land first. | 🟡 Later | v0.1 docs review |
| F42 | ✅ Shipped in v0.2 — rule documentation coverage gate. [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs) cross-checks every shipped rule id against its mdBook page, `Category::for_rule`, `scoring::WEIGHTED_RULE_IDS`, and (on CI, gated by `RULE_DOCS_GATE_GIT=1`) the `## [Unreleased]` section of `CHANGELOG.md`. Contract documented in [`CONTRIBUTING.md`](CONTRIBUTING.md#adding-or-modifying-a-rule--documentation-contract). | — | v0.2 interlude |
| F43 | ✅ Shipped in v0.2 — `RULES.md` category drift fixed. Per-rule `**Category**` lines and the Categories table now match `Category::for_rule`: `structure.excessive-commas` and `structure.deep-subordination` are `structure`, `rhythm.repetitive-connectors` is `rhythm`, `syntax.unclear-antecedent` is `syntax`. The drift banners on the four per-rule mdBook pages are removed. | 🟡 Later | Surfaced by F42 interlude |
| F44 | Coverage test for F30 rule-mention linking — assert each rule id mentioned in `docs/src/**/*.md` is linked on first-per-section occurrence. Follow-up from F30. | 🟡 Later | F30 follow-up |
| F104 | **Per-category sidebar grouping in `SUMMARY.md`.** Today the sidebar shows the 25 rules as one flat list under "Overview" — category-grouped in *order* but with no visible grouping. Readers scanning for rhythm or readability rules wade past 9 structure entries first. The `rules/index.md` page already shows the per-category table; the sidebar should mirror it. Add 5 sub-headings (`structure` · `rhythm` · `lexicon` · `syntax` · `readability`) with their rules nested below in `SUMMARY.md`. Mirror the same shape in the FR `Version française` block once each FR rule lands. Cost: small (one SUMMARY edit per locale; no new pages). Picked over (B) "one sub-page per category" — B doubles the page count without adding clarity the index table doesn't already provide. | 🟡 Later | 2026-04-25 docs UX critique (Block E) |
| F105 | **Consolidated references page.** Today citations are scattered across 10+ rule pages — WCAG 1.3.1 / 1.4.8 / 2.4.6 / 3.1.4, RGAA 9.1 / 9.4, Sweller, Gibson 1998, Graesser, Coh-Metrix, BDA Dyslexia Style Guide, IFLA easy-to-read guidelines, FALC, plainlanguage.gov, Kandel-Moles. A reader who wants to know "what does this tool stand on?" has no single surface. New `docs/src/references.md` under the "Project" section: flat list of cited sources, each with a one-line context and the rules that cite it. Rule pages keep their inline citations but link to the anchor here. FR mirror at `docs/src/fr/references.md`. Cost: medium — one-pass scan of every rule page + draft + cross-link. | 🟡 Later | 2026-04-25 docs UX critique (Block E) |
| F106 | **Landing-page polish.** `docs/src/introduction.md` already plays both roles today: lens-motif hero, before/after figure, "what makes it different", quick-taste terminal capture, "where to next". A real landing-page push only earns its cost when there's a *first consumer outside the maintainer* (project gets adopted, traffic shows up). Until then, polishing is design work without a forcing function. Notes for when triggered: more positioning above the fold, demo grid for the rule families (one canonical example per category), CTA toward profiles + quick-start, lens-motif extension already validated for use across the page. | 🟢 Speculative | 2026-04-25 docs UX critique (Block E) |

### Docs site — theming

| ID | Item | Priority | Origin |
|---|---|---|---|
| F26 | ✅ MVP shipped in v0.2 via DOM-level trim in `lucid-navigation.js` — the picker now shows three honest items (`Auto · Lucid light · Lucid dark`); the stock Rust / Navy / Ayu `<li>`s are marked `hidden` so they're inert for keyboard and screen-reader. CSS class mapping is unchanged (`.light` / `.rust` → lucid-light, `.coal` / `.navy` / `.ayu` → lucid-dark), so pre-existing localStorage selections still render correctly. Follow-up (optional): a full `index.hbs` override to drop the stock markup entirely rather than hide it; preferred once the mdBook upgrade cadence settles. | 🟡 Later | v0.1 docs `/colorize` session; mdBook stock limitation |
| F73 | ✅ Pre-deploy font-leak gate shipped in v0.2 — `just docs-check-clean` rebuilds the book, runs `scripts/sanitize-stock-css.py`, and greps the output for active `font-family` / `--*-font` / `local()` references to `Open Sans` or `Source Code Pro`. Not wired into `just check` (mdbook build is too slow for the dev loop); wire it into the docs-publish CI workflow before any release-candidate goes live. | 🟡 Later | v0.2 `/critique` polish pass follow-up |
| F84 | ✅ Shipped in v0.2.1 — fixed localhost 404.html rendering under `mdbook serve`. `book.toml` sets `site-url = "/lucid-lint/"` for GitHub Pages, and mdBook emits `<base href="/lucid-lint/">` into 404.html (only there). On localhost that prefix doesn't exist, so the browser's preload scanner fired 18 stylesheet/script requests with the wrong prefix before the page recovered via a second fetch. The previous JS workaround in `docs/theme/head.hbs` rewrote `<base>` at parse time, but ran after the preload scanner. Fix: `just docs-serve` now sets `MDBOOK_OUTPUT__HTML__SITE_URL=/` for the serve process, so 404.html carries `<base href="/">` on localhost and the correct `<base href="/lucid-lint/">` in production builds; the JS workaround is removed. | — | 2026-04-23 Block A |

### Docs site — reading preferences

| ID | Item | Priority | Origin |
|---|---|---|---|
| F33 | Full reading-preferences popover UI — cog button in the header opens a popover with font radio (Atkinson / Standard / OpenDyslexic), line-spacing slider (1.4–2.0, 0.05 step) and text-size slider (90–130 %, 5 % step). v0.1 ships only the Introduction-page demonstrator; the CSS-variable plumbing (`--reading-scale`, `--reading-line-height`, `[data-font]`) is already in place, so this is UI work only. | 🟡 Later | v0.1 docs `/shape` + `/typeset` sessions |
| F34 | Responsive / mobile adaptation — right-rail page TOC and header controls collapse gracefully below 700 px; touch targets verified ≥ 44 × 44 px; sidebar drawer behaviour polished. | 🔴 Next | v0.1 docs `/layout` session, deferred to `/adapt` |
| F35 | Accessibility audit sweep — full AAA pass on both themes (contrast, focus order, `prefers-reduced-motion` coverage, keyboard-only walk-through, skip-link), plus a published accessibility statement page. First audit pass ran 2026-04-22 (17/20, 0 P0, 2 P1, 3 P2); findings filed as F35a–F35d below. F35 stays open until the statement page ships and P1s are cleared. | 🟡 In progress | v0.1 docs `/audit` plan |
| F35a | ✅ Shipped 2026-04-22 — `theme/index.hbs` is now forked from mdBook v0.5.2's upstream template (minimal-diff approach, documented so future mdBook upgrades stay a mechanical re-sync). The skip link and EN / FR language switch are emitted as server-rendered HTML inside `<body>` and inside `.right-buttons`; both language variants are rendered and CSS in `lucid-layout.css` hides the wrong-locale copy based on `html[lang]` (which `head.hbs` sets synchronously before first paint on `/fr/` pages). The previous `skipLink()` and `langSwitch()` IIFEs in `lucid-navigation.js` are gone; the only remaining JS on the skip-link path is a progressive-enhancement smooth-scroll handler. WCAG 2.4.1 Bypass Blocks now passes with JS disabled. Unblocks F26 (stock theme labels can be collapsed at the markup level). | — | F35 audit 2026-04-22 |
| F35b | **Drop `role="radiogroup"`/`role="radio"` on reading-demo chips** (P2 from F35 audit). Current markup declares radiogroup semantics but the JS only binds `click` — arrow-key traversal is missing, so the ARIA contract is broken. Simpler fix is to switch to plain buttons with `aria-pressed` (the chips are preset toggles, not radios) rather than add a keyboard handler. Promoted to 🔴 Next on 2026-04-24 (brainstorm-next-cycles). | 🔴 Next | F35 audit 2026-04-22 |
| F35c | **`.lucid-stance__idea` reduced-motion rule strips the colour tint entirely** (P2 from F35 audit). `prefers-reduced-motion` users lose the pair-comparison idea-highlight feature, not just its animation. Move any transition/animation properties into the reduced-motion block and keep the static `background-color` in the base rule. Same pattern probably worth scanning across the other seven reduced-motion blocks. Promoted to 🔴 Next on 2026-04-24. | 🔴 Next | F35 audit 2026-04-22 |
| F35d | **Publish an accessibility statement page** (`docs/src/accessibility.md`, FR counterpart at `docs/src/fr/accessibility.md`). EN page carries the stated bar (WCAG 2.2 AAA), first audit pass result (2026-04-22, 17/20), a "Known limitations" block listing F35a/b/c pending, report route, and audit cadence. FR stub mirrors the limitations block. Shipped 2026-04-22. | 🟢 Shipped | F35 audit 2026-04-22 |
| F36 | Final polish pass — optical alignment, spacing rhythm, edge-state copy, favicon PNG fallback, social-card refinement, re-running `/critique` to verify the score moves above 30/40. | 🟡 Later | v0.1 docs `/polish` plan |

### Quality features

| ID | Item | Priority | Origin |
|---|---|---|---|
| F12 | Score evolution dashboard across runs | 🟢 Speculative | Rule 11, inspired by coverage reports |
| F98 | **Mutation testing via `cargo-mutants`.** ✅ Baseline shipped 2026-04-25 — dev-tool installed, `just mutants <file>` recipe added (timeout 60 s, no-shuffle for reproducibility), four-file probe run: `sentence_too_long.rs` 6 caught / 0 missed / 4 unviable (100 %), `scoring.rs` 18 / 0 / 2 (100 %), `engine.rs` 5 / 0 / 12 (100 %), `low_lexical_diversity.rs` 29 / 47 / 5 (36 %). Canonical reference rule + cross-cutting layer score perfectly; the lexical-diversity rule has two clear test gaps surfaced as F108 + F109. Triage methodology: cluster missed mutants by site → one ROADMAP entry per root cause, not per mutant. | ✅ Done | Stream-2 testing brainstorm, 2026-04-24 |
| F108 | **`low_lexical_diversity::ratio_at_anchor_min` — assert reported ratio in tests.** ✅ Shipped 2026-04-25. Added `reported_ratio()` helper (parses the documented message format) and three new test fixtures: `reported_ratio_is_minimum_observed_in_cluster` (50 W + 100 cache + 50 V → cluster-exit path with min ratio 0.01 deep mid-slide, not at anchor), `flush_path_reports_final_ratio` (cache-only doc → flush path), and `exactly_window_size_tokens_runs_the_check` (boundary on the early-return guard). Ratio assertion uses `(ratio - 0.01).abs() < 1e-9` so floating-point shifts from arithmetic mutations are caught. Bonus refactor (typed-ratio field on `Diagnostic`) deferred — string parsing is fine for the test-only consumer. | ✅ Done | F98 baseline 2026-04-25 |
| F109 | **`low_lexical_diversity::check` — borderline-cluster fixtures.** ✅ Shipped 2026-04-25 alongside F108. Added `cluster_starts_at_strict_inequality` and `ratio_exactly_at_threshold_does_not_trigger` — the latter uses 49 W + 51 cache so the only full window has unique=50 → ratio exactly 0.50 = `min_ratio`. With strict `<` the rule must not trigger; a `< → <=` flip would emit a diagnostic and fail the test. Combined effect: the rule's mutation score moved from 36 % (29 / 47 / 5) at F98 baseline to **89 %** (68 / 8 / 5). The remaining 8 missed mutants are equivalent under the current rule logic — defensive guards (`start_index + window > tokens.len()` is unreachable in normal flow because `anchor.index ≤ len − window`), or initial values the slide loop unconditionally overwrites (`let mut best = unique / window` is replaced as soon as a lower ratio appears, which it always does in a real cluster). Closing those would require rule refactoring (e.g. starting `best` at `f64::INFINITY` to prove the initial computation is dead) — diminishing returns; deferred. | ✅ Done | F98 baseline 2026-04-25 |
| F99 | **Property-based tests via `proptest`** (dep already in `[dev-dependencies]`, zero call sites today — paid for, unused). Four invariants in `tests/properties.rs`, deliberately small: (1) `split_sentences` never drops a non-whitespace character on round-trip, (2) re-linting an identical string yields identical diagnostics (engine idempotence), (3) for threshold-driven rules, `public`-profile diagnostics are a superset of `dev-doc`-profile diagnostics on the same input (profile monotonicity), (4) `Engine::lint_str` never panics on arbitrary valid UTF-8 ≤ 10KB. Goal: fortify tokenizer / engine seams, not rewrite the suite. | 🟡 Later | Stream-2 testing brainstorm, 2026-04-24 |
| F100 | **LLM false-positive miner via Claude Code.** Dev-only audit script (not a test, not a CI gate) that runs lucid-lint across the CC corpus, asks Claude to flag diagnostics that look wrong, writes a triage report to `.personal/audits/`. Reframed from the original "LLM-as-Judge harness" after Devil's Advocate surfaced three blockers on the gating form: non-determinism across Claude model versions, ambiguity about whether a disagreement indicts the rule or the judge, cost / wall-clock at 600×N scale. The miner form sheds all three — human triages, Claude suggests. Respects prime directive #4 (deterministic core, no LLM) because it lives entirely outside the library crate and never blocks `just check`. Wait until v0.3 `lucid-lint-nlp` plugin work surfaces the need for correctness review at scale. | 🟢 Speculative | Stream-2 testing brainstorm, 2026-04-24 |
| F93 | Tokenizer `split_sentences` `Vec\<char\>` allocation. The helper collects the full input into a `Vec\<char\>` per call to support lookbehind (`chars[idx-1]`) and arbitrary lookahead (`chars[idx+1..].find(!ws)` for ellipsis-continuation). Nominal waste on real corpus is ~5% of the `split_sentences` budget (bench shows 35µs total, `Vec\<char\>` alloc ~1–2µs). Refactor to a small ring-buffer + `Peekable\<CharIndices\>` is feasible but high-churn for low ceiling. Revisit only if profiling pins the tokenizer as a bottleneck. | 🟢 Speculative | Stream-2 code review 2026-04-24 (measured; deferred) |
| F89 | Unify rule-page example figures on the `.lucid-stance` component. Today the intro page uses a custom `.lucid-stance` figure (Before / After side-by-side, colour-matched ideas, diagnostic in the figcaption), while rule pages use plain H3 + blockquote + fenced `text` for the diagnostic (see `docs/src/rules/sentence-too-long.md`). The H3 form works and is cheap to roll out, but wide screens could show stronger Before↔After pairing with the side-by-side figure. Scope: extract `.lucid-stance` into a reusable component (mdBook include or raw HTML pattern), tune the styling for in-content width (rule pages sit inside the narrower content column, not the landing-page hero), one figure per language, drop the H3 subsections in favour of a `data-lang` attribute surfaced as a chip on the figure. Ship only after the H3-based rollout has landed across all example-bearing rule pages and the unified pairing is confirmed as the dominant reader complaint. | 🟢 Speculative | 2026-04-23 docs clarity session — H3 subsections landed as the lightweight option; F89 parks the heavier unify-the-components path |
| F88 | `--fix` mode for the mechanical subset of rules — promoted to 🟡 Later on 2026-04-24 (brainstorm-next-cycles, 0.3 Should). Narrow scope locked: `lexicon.all-caps-shouting` (lowercase the run), `lexicon.redundant-intensifier` (drop the intensifier), `structure.mixed-numeric-format` (normalise to the detected majority style), `structure.line-length-wide` (rewrap to `max_chars`). All other rules stay report-only — cognitive-load judgments need the author to choose the rewrite. Borderline `structure.heading-jump` stays out of the initial cut. Design: per-rule `fixable: bool` metadata on the `Rule` trait, `--fix` flag walks diagnostics in document order applying only those with concrete replacements, writes files in place (or emits a unified diff with `--fix=print`), exits with count of fixes applied. Conservative default: `--fix` only touches the explicitly-fixable set, never guesses. | 🟡 Later | 2026-04-23 docs clarity session — framing "lucid-lint reports, you rewrite" surfaced the question |

### Scope control

File/directory discovery. Distinct from suppression (below): scope
control excludes inputs before they are scanned; suppression hides
diagnostics after scanning.

| ID | Item | Priority | Origin |
|---|---|---|---|
| F78 | ✅ Shipped in v0.2 — `exclude = [...]` glob list in `[default]` of `lucid-lint.toml` and `--exclude <GLOB>` CLI flag (comma-delimited, repeatable). Patterns match against paths relative to the walked root; matching directories are pruned, not descended. Explicit file args bypass exclusion. Backed by `globset`. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#excluding-paths). `.lucidignore` (gitignore-style file) deferred to F78b if user demand surfaces. | — | Dogfood feedback 2026-04-21 |
| F78b | `.lucidignore` file (gitignore-style, with negations and nested files). Different crate (`ignore`) and a larger test matrix than the glob-list MVP. Ship only if users ask — the `exclude` list in `lucid-lint.toml` covers the dominant use case. | 🟢 Speculative | F78 deferral, 2026-04-21 |

### Suppression mechanism

v0.1 ships the minimal inline-disable directive (see brainstorm
`brainstorm/20260419-inline-disable-feature.md`). Extensions deferred:

| ID | Item | Priority | Origin |
|---|---|---|---|
| F18 | ✅ Block form shipped in v0.2: `<!-- lucid-lint-disable <rule-id> -->` … `<!-- lucid-lint-enable -->` silences one rule across every line in the scope. `enable` with no argument closes every open scope; with a rule id, closes only that rule's scope (so overlapping disables for different rules can nest). Unterminated `disable` extends to end-of-document. See [RULES.md → Suppressing diagnostics](RULES.md#suppressing-diagnostics). | — | v0.1 inline-disable brainstorm |
| F19 | ✅ Shipped in v0.2 — top-level `[[ignore]]` array-of-tables in `lucid-lint.toml`, each entry with a required `rule_id` silences every diagnostic for that rule across Markdown, plain text, and stdin. Unknown ids tolerated. Applied post-engine, pre-scoring, so scoring / rendering / exit-code logic all see the filtered view. Scope broadened from the roadmap's original "`.txt` and stdin" wording because a global filter is simpler and more useful; Markdown users can still prefer inline directives for local silencing. `reason` field tracked as F20. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#silencing-rules-globally). | — | v0.1 inline-disable brainstorm |
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
| F75 | `lucid-lint-nlp` plugin specification and scaffolding (Python subprocess or WASM-based). Replaces heuristic rules with POS- / dependency-tree- / anaphora-backed precise versions. **Ship only when the first plugin rule is concretely scheduled** — scaffolding-without-consumer is the red flag from AGENTS.md directive #1 (2026-04-24 brainstorm-next-cycles). | 🟡 Later | Rule-system-growth brainstorm (2026-04-20) |

Candidate rules for the plugin:

- POS-based `syntax.passive-voice` detection (replaces v0.1 heuristic)
- Full anaphora resolution for `syntax.unclear-antecedent`
- Dependency-tree-based `structure.deep-subordination`
- Semantic similarity between adjacent sentences (discourse cohesion signal inspired by Coh-Metrix)

### New rules (v0.3 candidates)

Deferred from v0.2 because they require corpus work, lexicon builds, or
depend on earlier features (F9, F14). Naming uses the provisional
`category.rule-name` prefix pending F29.

| ID | Rule | Category | Tags | Grounding | Depends on |
|---|---|---|---|---|---|
| F46 | `lexicon.homophone-density` | Lexicon | `dyslexia` | BDA (dyslexia) | FR corpus tuning; ships as `info`. Slip-flag (2026-04-24): if FR corpus tuning exceeds ~2 days, slides to 0.3.x |
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
| F79 | Fancy terminal rendering for `lucid-lint explain` — pipe the bundled markdown through `termimad` (or a custom `pulldown-cmark` + `owo-colors` walker) so headings, tables, code fences, bullets, and inline `code` render with proper typography instead of raw markdown. Ship a toned `Skin` that matches the existing warning-yellow / info-cyan palette rather than termimad's magenta defaults — the brand direction is calm, typographic, not "rich CLI". Defer past v0.2 so the `check` output polish (F?) lands first. | 🟡 Later | TTY-output critique (2026-04-22) |

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

Scope audit at 2026-04-20: after the `structure.heading-jump` reframing (cognitive
"comprehension cliff" at skip ≥ 2 levels, distinct from MD001's strict
+1 rule), **`structure.deeply-nested-lists` is the only lucid-lint rule that
remains functionally equivalent to a markdownlint rule (MD007)**. The
mechanism below is designed to scale — Vale, proselint, textlint
overlaps are likely as the rule set grows — rather than to solve a
single-rule problem.

| ID | Item | Priority | Origin |
|---|---|---|---|
| F77 | ✅ Shipped in v0.2 — `main.rs` now auto-discovers `lucid-lint.toml` walking up from the CWD (stopping at the nearest `.git` boundary) and applies `[default].profile`, `[default].conditions`, `[scoring]` via `ScoringFileConfig::into_scoring_config`, and `[rules.readability-score].formula`. New `--config <path>` flag overrides discovery. Precedence: built-in profile defaults → TOML → CLI flags. Per-rule TOML overrides beyond `readability.score` extend rule-by-rule as each `Config` gains `Deserialize`. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md). | — | F11 follow-up (2026-04-21) |
| F76 | Interop suppression mechanism. Rules declare overlapping external linter rules in their metadata (e.g. `Rule::external_overlaps() -> &[(Linter, &'static str)]`, enum `Linter::Markdownlint \| Vale \| Proselint \| Textlint`). Users opt in via `[interop] suppress_when = ["markdownlint"]` in `lucid-lint.toml` (CLI equivalent: `--interop-suppress=markdownlint`); opt-out is default, so coverage never silently drops. When active, affected rules are skipped at emission time with an info-level trace in `--verbose`. Ships CLI + LSP (the LSP path is the real motivator: two servers squiggling the same span with different severities and wording erodes trust in both). Only `structure.deeply-nested-lists` qualifies at time of writing (MD007); framework is designed to scale to future overlaps. Non-goal: detecting whether the external linter is actually installed or configured — the config field is the signal. | 🟡 Later | Markdownlint-overlap scan (2026-04-20) |

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
  `structure.long-enumeration` → concrete list skeleton). Controversial for
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

## v0.4 — horizon (bets, not commitments)

Routed 2026-04-24 in `.personal/brainstorm/20260424-next-cycles.md`.
Each bet lists the **signal that unlocks it**, so horizon items don't
drift into Must by tenure alone. No commitments; this is "what could
be true in ~6 months if 0.2 and 0.3 land cleanly".

| Bet | Unlock signal |
|---|---|
| F16 — `lucid-lint-llm` plugin | ≥ 2 concrete LLM-as-Judge rules designed on paper; deterministic-core base stable enough that non-determinism is a clear opt-in |
| F5 / F6 / F7 / F8 — alternative formats (AsciiDoc / HTML / .docx / pandoc bridge) | External user requests; pick the single format with most pull and ship it alone, not the set |
| F85 + F86 — fixture coverage maps + auto-discovery | Referential has stabilised (F84 part 2 done) and rule set stops churning |
| F63 — vocabulary-rarity | Lexique.org + COCA frequency lexicons built and licence-cleared |
| F65 – F69 — remaining condition-tag rules | F46 / F49 / F51 / F53 / F57 validated in the wild at 0.3 |
| F38 — section-level scoring | Document + project level proven; users ask "which H2 is the problem?" |
| F41 — reading-time unit | Validated heuristic exists; companion metrics (comfort, fatigue, understandability) defined |
| F12 — score-evolution dashboard | CI users explicitly ask for trend view (not delta — delta is F73-dx / `--compare`) |
| F76 — interop suppression (if not shipped in 0.3) | A second rule joins `deeply-nested-lists` as a markdownlint overlap |
| F74 — rule-discovery corpus mining | Student / intern resource available; separate research track |
| LSP server | Editor demand visible (Cursor / VSCode issues); would change the deployment story |
| F70 / F64 — research-track rules | Only if someone codes them for fun |
| **F101 — top 3 items from first-10-external-users feedback (TBD)** | 0.2.0 ships and ≥ 10 non-maintainer users exist — placeholder reserved so the horizon isn't 100 % maintainer bets (renumbered from F98 post-collision with stream-2 cargo-mutants) |

**Deliberately off the 0.4 list:**

- F39 / F40 letter grade + traffic light — routed to 0.3 Should; if
  they slip they go to 0.3.x, not 0.4.
- Full F29 numeric codes — parked until a rename actually happens.
- F2, F3 speculative rule refinements — stay speculative until a
  concrete dogfood case surfaces.
- F17 per-family sub-scores — category sub-scores (F14) already ship;
  unclear what "family" adds beyond that.

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

Future rules and plugins can be proposed by the community. The default jargon and stoplists (`lexicon.jargon-undefined`, `lexicon.weasel-words`, `lexicon.low-lexical-diversity`) are especially welcome targets for community pull requests to expand coverage across domains and languages.
