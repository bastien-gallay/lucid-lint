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

### Rules (4 / 16)

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
| ☐ | `excessive-commas` | |
| ☐ | `consecutive-long-sentences` | |

#### Phase 3 — Lexical rules with word lists

| Status | Rule | Notes |
|---|---|---|
| ☐ | `weasel-words` | |
| ☐ | `unexplained-abbreviation` | v0.1 simplified, no definition awareness (see F9) |
| ☐ | `jargon-undefined` | |
| ☐ | `excessive-nominalization` | |
| ☐ | `repetitive-connectors` | |

#### Phase 4 — Global metric

| Status | Rule | Notes |
|---|---|---|
| ☐ | `readability-score` | EN/FR share Flesch variant in v0.1 (see F10) |

#### Phase 5 — Heuristic rules (hardest)

| Status | Rule | Notes |
|---|---|---|
| ☐ | `long-enumeration` | |
| ☐ | `deep-subordination` | |
| ☐ | `passive-voice` | Heuristic only in v0.1; POS-based detection is a `lucid-lint-nlp` plugin candidate |
| ☐ | `unclear-antecedent` | |
| ☐ | `low-lexical-diversity` | |

### Cross-cutting features

| Status | Feature | Notes |
|---|---|---|
| ✅ | Minimal inline-disable | `<!-- lucid-lint disable-next-line <rule-id> -->` for Markdown inputs, single rule id, optional reason. See [RULES.md → Suppressing diagnostics](RULES.md#suppressing-diagnostics). Block form, config ignores, file-level scope and required `reason=` are tracked as F18–F21 below. |

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
