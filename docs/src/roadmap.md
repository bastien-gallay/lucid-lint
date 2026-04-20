# Roadmap

The full roadmap lives in [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md) at the repository root.

## Summary

### v0.1 — Complete

All **17 / 17 rules** shipped, grouped by category: length, structure, rhythm, lexical, style, and global readability. Full per-rule reference in [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) and the authoritative checklist in [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#v01--in-progress).

Cross-cutting features shipped:

- Minimal inline-disable directive (`<!-- lucid-lint disable-next-line <rule-id> -->`) for Markdown inputs.

### v0.2 — First major iteration

**Shipped in this cycle**:

- ✅ **Hybrid scoring model (F14)** — global `X / max` score plus 5
  per-category sub-scores (Structure · Rhythm · Lexicon · Syntax ·
  Readability), `--min-score` CLI flag, `[scoring]` / `[scoring.weights]`
  config tables. See the [Scoring](./guide/scoring.md) guide page.
- 🚧 **Document-level scoring granularity (F15)** — CLI aggregates
  multi-path runs as a single document; per-file + project roll-up
  still open.

**Still in flight**:

- SARIF v2.1.0 output for GitHub Code Scanning (F32)
- Rule-message clarity audit (F37) — gates the v0.2 release
- Language-specific readability formulas (Kandel-Moles FR, SMOG, Coleman-Liau)
- Definition-aware `unexplained-abbreviation`
- Custom stoplist for `low-lexical-diversity`
- Pandoc companion script
- `missing-connectors` rule
- Native AsciiDoc and HTML support

### v0.3+ — Advanced plugins

- `lucid-lint-llm` plugin for LLM-as-Judge rules (disabled by default)
- `lucid-lint-nlp` plugin for deeper linguistic analysis (POS-based passive voice, anaphora resolution, discourse cohesion)
- VS Code extension and Neovim LSP support
