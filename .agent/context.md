# Agent context

This file captures project state and reference pointers useful for coding agents working on `lucid-lint`. The directives live in [AGENTS.md](../AGENTS.md) at the repository root — read it first.

## Current state (v0.2)

Bootstrapped April 2026 with 17 rules in v0.1. The v0.2 cycle landed the hybrid scoring model (F14): `Diagnostic.weight`, `--min-score` CLI gate, `[scoring]` config table, JSON schema `version = 2`, fixed 5-variant taxonomy.

For shipped vs in-flight detail, see [CHANGELOG.md](../CHANGELOG.md) and [ROADMAP.md](../ROADMAP.md).

Reference rule implementation: `sentence-too-long`. Reference cross-cutting module: `src/scoring.rs`. The `Diagnostic` struct lives in `src/diagnostic.rs` — read the source rather than restating it.

## Backlog

The authoritative v0.1 backlog (rules + cross-cutting features, with current status) lives in [ROADMAP.md — v0.1 — In progress](../ROADMAP.md#v01--in-progress). Update the checkbox when a rule lands.

## Design context

Brand voice, palette, typography shortlist, audience, and the WCAG AAA accessibility bar live in [.impeccable.md](../.impeccable.md). Consult before any frontend, mdBook, branding, or marketing-surface work.

## Taxonomy (do not regress)

The v0.2 category taxonomy is fixed at 5 variants: `Structure · Rhythm · Lexicon · Syntax · Readability`. The pre-v0.2 `Length`, `Lexical`, `Style`, `Global` variants are gone — do not re-introduce them. `Diagnostic.category` is not stored; derive via `Category::for_rule`.

## Coding patterns

- **Rule implementation** — each rule implements `Rule` and lives in its own file under `src/rules/`. Canonical template: `src/rules/structure/sentence_too_long.rs`.
- **Testing** — per rule: `#[cfg(test)] mod tests` with unit tests, an `insta` snapshot, corpus fixtures referenced from `tests/corpus/{en,fr}/`.
- **Configuration** — `serde` with profile-based defaults. `lucid-lint.toml` overrides profile defaults per rule.

## Useful commands during development

```bash
just test-one sentence_too_long    # Test one rule
just snapshot                       # Review pending snapshots
just dogfood                        # Lint lucid-lint's own docs
just docs-serve                     # Preview mdBook with hot reload
just texts-plan                     # Preview the texts.yaml → examples/ routing
just texts                          # Fetch + clean + convert text fixtures
```

The `texts*` recipes pull the sources listed in `examples/texts.yaml` (filtered to `markdownable >= 3`) into `examples/public/` (safe to commit) or `examples/local/` (gitignored). See [`scripts/README.md`](../scripts/README.md) for details.
