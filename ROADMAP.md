# lucid-lint — Roadmap

> Future rules, refinements, and platform extensions tracked from v0.1 design discussions.

This document captures ideas that emerged while designing v0.1. They are intentionally deferred.

## Legend

| Status | Meaning |
|---|---|
| ✅ | Done (merged on `main`) |
| 🚧 | In progress |
| ☐ | Not started |

| Priority | Meaning |
|---|---|
| 🔴 High | Expected to land in v0.2 |
| 🟡 Medium | Likely v0.3 or plugin |
| 🟢 Low | Nice-to-have, speculative |

---

## v0.1 — In progress

Backlog of everything that must ship before tagging `v0.1.0`. The implementation order mirrors the phases agreed during the bootstrap session: start with the cheapest, most deterministic signals, work up to the trickiest heuristics.

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
| ☐ | Accessibility page in the docs | Dedicated `accessibility.md` page covering: the stated WCAG 2.2 AAA bar, the reading-preferences control, typography choices with credits (Atkinson Hyperlegible Next — Braille Institute; OpenDyslexic — Abelardo Gonzalez; Literata — TypeTogether), keyboard shortcuts, and a brief explanation of how the site itself dogfoods the project's mission. Linked from the sidebar and the footer. |

---

## v0.2 — First major iteration

### Architecture

| ID | Item | Priority | Origin |
|---|---|---|---|
| F14 | Hybrid scoring model (global score + per-category sub-scores + diagnostics) | 🔴 High | Architecture decision discussion |
| F15 | Scoring granularity: document, project, section | 🔴 High | Linked to F14 |
| F17 | Per-family sub-scores | 🟡 Medium | Linked to F14 |

### Rules refinement

| ID | Item | Priority | Origin |
|---|---|---|---|
| F9 | Definition-aware `unexplained-abbreviation` (two-pass) | 🔴 High | Rule 10 simplified in v0.1 |
| F10 | Language-specific readability formulas (Kandel-Moles FR, SMOG, Coleman-Liau) | 🔴 High | Rule 11 simplified in v0.1 |
| F11 | User-configurable readability formula choice | 🟡 Medium | Rule 11 |
| F13 | `missing-connectors` rule (15b not shipped in v0.1) | 🟡 Medium | Rule 15 decomposition |
| F1 | Custom stoplist parameter for `low-lexical-diversity` | 🟡 Medium | Rule 5 |
| F2 | Sentence-level low-lexical-diversity density | 🟢 Low | Rule 5 |
| F3 | Comma density metric (relative) for `excessive-commas` | 🟢 Low | Rule 3a |
| F22 | Context-aware relaxation for `excessive-commas` (research needed before design) | 🔴 High | v0.1 dogfood: 5 false-ish positives on technical docs |
| F23 | Context-aware `weasel-words` (quoted terms, "many X" with concrete X, meta-discussion of the words themselves) | 🔴 High | v0.1 dogfood: 11 false-ish positives on this repo's own docs |
| F24 | Refine `excessive-nominalization` suffix list (drop or gate `-al`; many adjectives — `crucial`, `horizontal`, `positional`, `attentional` — are flagged despite not being abstract nouns) | 🟡 Medium | v0.1 dogfood |
| F31 | Split `unexplained-abbreviation` built-in whitelist: the accessibility (`WCAG`, `WAI`, `ARIA`, `RGAA`, `EAA`, `FALC`, `AA`, `AAA`, `ADHD`) and AI (`LLM`, `NLP`) initialisms are well-known inside `lucid-lint` but narrower for a generic tech audience. Move them into a project-scoped whitelist once F19 lands CLI-level config loading, keeping only truly ubiquitous tech acronyms (`URL`, `HTML`, `API`, `CPU`, …) in the shipped `dev-doc` baseline. | 🟡 Medium | v0.1 review feedback |

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

### Format support

| ID | Item | Priority | Origin |
|---|---|---|---|
| F5 | Native AsciiDoc support | 🟡 Medium | Format scope v0.1 |
| F6 | Native HTML support | 🟡 Medium | Relevant for EAA compliance |
| F7 | `.docx` support via Pandoc integration | 🟡 Medium | FALC institutional target |
| F8 | Companion script `pandoc → lucid-lint` | 🔴 High | Documented in v0.1 README |

### Documentation rules plugin

| ID | Item | Priority | Origin |
|---|---|---|---|
| F4 | `code-block-without-lang` rule | 🟡 Medium | Rule 8 dropped from v0.1, candidate for `lucid-lint-docs` plugin |

### Docs site — bilingual

| ID | Item | Priority | Origin |
|---|---|---|---|
| F25 | French mirror of the mdBook docs (`/fr/` tree) — until then, the header EN/FR switch links to a "French version — coming in v0.2" stub | 🔴 High | v0.1 docs `/shape` session, bilingual-equality prime directive |

### Docs site — content

| ID | Item | Priority | Origin |
|---|---|---|---|
| F27 | Mirror the full `ROADMAP.md` in the mdBook site (synced from the repo file, not a hand-edited copy) so readers can browse v0.2+ plans without leaving the docs. | 🔴 High | v0.1 docs review |
| F28 | Dedicated rule reference in the docs: one page per rule, covering rationale, thresholds per profile, EN/FR examples, and suppression guidance. | 🔴 High | v0.1 docs review |
| F29 | Rule numbering scheme based on category (e.g. `STR-001` for structural, `LEX-002` for lexical, `SYN-003` for syntactic). Stable IDs that survive renames, referenced from both diagnostics output and the docs. | 🟡 Medium | v0.1 docs review |
| F30 | Audit every rule mention across the docs and link it to its reference page (F28). Requires F28 to land first. | 🟡 Medium | v0.1 docs review |

### Docs site — theming

| ID | Item | Priority | Origin |
|---|---|---|---|
| F26 | Override `index.hbs` (or `book.js`) to replace the mdBook theme picker with a two-option toggle labelled "Lucid light / Lucid dark". In v0.1 the `.light` + `.rust` classes both resolve to lucid-light and `.coal` + `.navy` + `.ayu` to lucid-dark — the palette is consistent but the menu labels still read `Light / Rust / Coal / Navy / Ayu`. | 🟡 Medium | v0.1 docs `/colorize` session; mdBook stock limitation |

### Quality features

| ID | Item | Priority | Origin |
|---|---|---|---|
| F12 | Score evolution dashboard across runs | 🟢 Low | Rule 11, inspired by coverage reports |

### Suppression mechanism

v0.1 ships the minimal inline-disable directive (see brainstorm
`brainstorm/20260419-inline-disable-feature.md`). Extensions deferred:

| ID | Item | Priority | Origin |
|---|---|---|---|
| F18 | Block form: `<!-- lucid-lint-disable <rule-id> -->` … `<!-- lucid-lint-enable -->` | 🔴 High | v0.1 inline-disable brainstorm |
| F19 | Config-based ignores (`[[ignore]]` in `lucid-lint.toml`) covering `.txt` and stdin | 🔴 High | v0.1 inline-disable brainstorm |
| F20 | `reason="..."` field, optional in v0.1, surfaced in reports and optionally required via config | 🟡 Medium | v0.1 inline-disable brainstorm |
| F21 | File-level directive (`disable-file`) and multi-rule lists | 🟡 Medium | v0.1 inline-disable brainstorm |

---

## v0.3+ — Advanced plugins

### LLM-enhanced detection

| ID | Item | Priority | Origin |
|---|---|---|---|
| F16 | `lucid-lint-llm` plugin (LLM-as-Judge rules) | 🟢 Low | Research on existing tools |

The plugin would add rules like `unclear-antecedent-semantic` that use an LLM to detect semantic ambiguities the pattern-based heuristics miss.

Disabled by default due to non-determinism, API cost, and latency incompatible with pre-commit hooks.

### Advanced NLP

Candidates for a `lucid-lint-nlp` plugin (Python subprocess or WASM-based):

- POS-based `passive-voice` detection (replaces v0.1 heuristic)
- Full anaphora resolution for `unclear-antecedent`
- Dependency-tree-based `deep-subordination`
- Semantic similarity between adjacent sentences (discourse cohesion signal inspired by Coh-Metrix)

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
- Glob patterns and `.lucidignore`
- Core library exposed as `lucid-lint-core` for third-party integration

### Project

- Repo structure: single crate vs. Cargo workspace
- Reference corpus for testing
- README v0.1 content and positioning
- Tagline and visual identity

---

## Contribution invitation

Future rules and plugins can be proposed by the community. The default jargon and stoplists (`jargon-undefined`, `weasel-words`, `low-lexical-diversity`) are especially welcome targets for community pull requests to expand coverage across domains and languages.
