# Roadmap

The full roadmap lives in [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md) at the repository root.

## Summary

### v0.1 — In progress

Current status: **1 / 16 rules implemented** (`sentence-too-long`, used as the template for the others). The 15 remaining rules are grouped in 5 phases — structural, simple text, lexical, global metric, then heuristics. Full checklist in [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#v01--in-progress).

Also in flight for v0.1:

- Minimal inline-disable — next-line and block suppression comments for Markdown

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
