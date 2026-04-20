# Roadmap

The full roadmap lives in [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md) at the repository root.

## Summary

### v0.1 — Complete

All **17 / 17 rules** shipped, grouped by category: length, structure, rhythm, lexical, style, and global readability. Full per-rule reference in [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) and the authoritative checklist in [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#v01--in-progress).

Cross-cutting features shipped:

- Minimal inline-disable directive (`<!-- lucid-lint disable-next-line <rule-id> -->`) for Markdown inputs.

### v0.2 — First major iteration

- Hybrid scoring model (global score + per-category sub-scores + diagnostics)
- Language-specific readability formulas (Kandel-Moles for French, SMOG, Coleman-Liau)
- Definition-aware `unexplained-abbreviation`
- Custom stoplist for `low-lexical-diversity`
- Pandoc companion script
- `missing-connectors` rule
- Native AsciiDoc and HTML support

### v0.3+ — Advanced plugins

- `lucid-lint-llm` plugin for LLM-as-Judge rules (disabled by default)
- `lucid-lint-nlp` plugin for deeper linguistic analysis (POS-based passive voice, anaphora resolution, discourse cohesion)
- VS Code extension and Neovim LSP support
