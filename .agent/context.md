# Agent context

This file captures project-specific context useful for coding agents working on `lucid-lint`.

The canonical entry point is [AGENTS.md](../AGENTS.md) at the repository root. This file adds project state and conventions that might not be obvious from the main docs.

## Current state (v0.2 — scoring)

The project was bootstrapped in April 2026 with 17 rules in v0.1. The
v0.2 cycle landed the hybrid scoring model (F14):

- Rust-only core, 17 rules, Markdown + plain text + stdin inputs.
- Bilingual EN/FR from day one.
- Deterministic rules only (LLM-based rules remain a future v0.3 plugin).
- **v0.2 additions**: hybrid scoring model (`X/max` global + 5 category
  sub-scores), `Diagnostic.weight` field, `--min-score` CLI gate,
  `[scoring]` config table, category taxonomy remapped to
  `Structure · Rhythm · Lexicon · Syntax · Readability`.
- Output is still linter-style *and* now scored; the JSON schema is at
  `version = 2`.

The reference rule implementation is `sentence-too-long`. The reference
cross-cutting module is `src/scoring.rs`.

## Backlog and implementation order

The authoritative v0.1 backlog (rules + cross-cutting features, with current status) lives in [ROADMAP.md — v0.1 — In progress](../ROADMAP.md#v01--in-progress). Consult it before picking up work, and update the checkbox when a rule lands.

## Design context

Brand voice, palette, typography shortlist, audience, and the WCAG AAA accessibility bar live in [.impeccable.md](../.impeccable.md) at the repo root. Consult it before any frontend, mdBook, branding, or marketing-surface work.

## Design decisions to respect

See also [ROADMAP.md](../ROADMAP.md) section "Design decisions from v0.1 session".

### Diagnostic struct (v0.2)

```rust
pub struct Diagnostic {
    pub rule_id: String,
    pub severity: Severity,
    pub location: Location,
    pub section: Option<String>,
    pub message: String,
    pub weight: u32,   // v0.2: seeded from scoring::default_weight_for(rule_id)
}
```

- `category` is NOT stored — derivable from `rule_id` via `Category::for_rule`.
- `weight` IS stored (v0.2): so that `with_weight()` overrides and user
  config overrides propagate without a second lookup. Rules almost never
  need to override; the default table is tuned centrally in `scoring.rs`.
- `suggestion` is NOT stored. Still deferred past v0.2.
- `section` IS stored because recomputing it requires re-parsing the document.

The v0.2 category taxonomy is **fixed at 5 variants**:
`Structure · Rhythm · Lexicon · Syntax · Readability`. The pre-v0.2
`Length`, `Lexical`, `Style`, `Global` variants are gone. Do not
re-introduce them.

### No premature abstraction

- No trait for a single implementation.
- No plugin system yet (v0.3 feature).
- Parser is concrete `MarkdownParser` until a second format needs adding.

### Deterministic core

- No network calls.
- No LLM calls.
- No environment-dependent behavior.

### Bilingual from day one

- Language detection in `src/language/detect.rs`.
- Language-specific data in `src/language/{en,fr}/`.
- Every language-dependent rule takes a `Language` parameter.

## Coding patterns

### Rule implementation pattern

Each rule implements the `Rule` trait and lives in its own file under `src/rules/`. See `src/rules/sentence_too_long.rs` for the canonical template.

### Testing pattern

Each rule file includes:

- `#[cfg(test)] mod tests` with unit tests
- A snapshot test in the same module
- Corpus fixtures referenced from `tests/corpus/`

### Configuration pattern

Rule configuration uses `serde` with profile-based defaults. The user's `lucid-lint.toml` overrides the profile defaults per rule.

## Useful commands during development

```bash
just test-one sentence_too_long    # Test one rule
just snapshot                       # Review pending snapshots
just dogfood                        # Lint lucid-lint's own docs
just docs-serve                     # Preview mdBook with hot reload
```

## What NOT to do

- Do not add `unwrap()` or `expect()` in library code.
- Do not use regex to parse Markdown (use `pulldown-cmark`).
- Do not split sentences on `.` alone (use the tokenizer).
- Do not hardcode language (accept `Language` as parameter).
- Do not store derivable data in structs.
