# lucid-lint — Roadmap

> Future rules, refinements, and platform extensions tracked from v0.1 design discussions.

This document captures ideas that emerged while designing v0.1. They are intentionally deferred.

## Legend

| Priority | Meaning |
|---|---|
| 🔴 High | Expected to land in v0.2 |
| 🟡 Medium | Likely v0.3 or plugin |
| 🟢 Low | Nice-to-have, speculative |

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
