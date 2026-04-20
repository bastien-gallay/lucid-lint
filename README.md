# lucid-lint

> A cognitive accessibility linter for prose. Built on cognitive load research. Bilingual EN/FR with equal care. Plugin-first, CI-native.

[![CI](https://github.com/bastien-gallay/lucid-lint/actions/workflows/ci.yml/badge.svg)](https://github.com/bastien-gallay/lucid-lint/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/lucid-lint.svg)](https://crates.io/crates/lucid-lint)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

**Status:** 🚧 v0.1 under active development. Not ready for production use. Current backlog and rule-by-rule progress: [ROADMAP.md — v0.1 — In progress](ROADMAP.md#v01--in-progress).

---

## What is lucid-lint?

`lucid-lint` reads your Markdown, plain-text, or piped prose and reports issues that increase **cognitive load** — the mental effort a reader spends to understand you.

<!-- lucid-lint disable-next-line excessive-commas -->

Most prose linters measure style (`write-good`), grammar (`Antidote`), or surface readability (Flesch score). `lucid-lint` focuses on one thing: **how hard is this text to read for someone whose attention is already stretched**. Stretched attention could mean ADHD, fatigue, a second language, a noisy environment, or an accessibility-sensitive context (EAA, RGAA, FALC).

It's fast, CI-native, bilingual EN/FR, and designed to be extended.

## How it compares

- **Vale** is a great prose linter with a plugin ecosystem. It's English-first and style-focused. `lucid-lint` is bilingual-first and cognitive-load focused.
- **textlint** has many plugins but no opinion on accessibility. `lucid-lint` takes a stance grounded in cognitive load research (Sweller, Gibson, Graesser).
- **Hemingway** and **Grammarly** are closed products for authors. `lucid-lint` is open source, CLI-first, CI-ready.
- **Coh-Metrix** is the academic reference for discourse cohesion metrics. It's rigorous but hard to integrate. `lucid-lint` borrows its conceptual model and ships it in a modern dev workflow.
- **readability-score libraries** compute Flesch, SMOG, etc. `lucid-lint` includes them as one signal among many.

## Install

Once released to crates.io:

```bash
cargo install lucid-lint
```

From source:

```bash
git clone https://github.com/bastien-gallay/lucid-lint
cd lucid-lint
cargo install --path .
```

## Quick start

```bash
# Lint a Markdown file with the default profile
lucid-lint check README.md

# Use a specific profile
lucid-lint check --profile=public docs/*.md

# Pipe from stdin
cat draft.md | lucid-lint check -

# JSON output for CI
lucid-lint check --format=json docs/

# Fail CI if the aggregate score drops below 85 / 100
lucid-lint check --min-score=85 docs/
```

## Scoring (v0.2)

Every run now emits a global `X / max` score plus five per-category
sub-scores (Structure · Rhythm · Lexicon · Syntax · Readability), in
addition to the existing diagnostics list.

```text
warning /tmp/draft.md:12:1 Sentence is 27 words long (maximum 22).
  rule: sentence-too-long

Summary: 1 warnings.
score: 88/100 · structure 8/20 · rhythm 20/20 · lexicon 20/20 · syntax 20/20 · readability 20/20
```

Use `--min-score=N` to gate CI on the aggregate score (the gate stacks
with `--fail-on-warning`). Tune category caps and per-rule weights via
the `[scoring]` and `[scoring.weights]` tables in `lucid-lint.toml`.

Full documentation in the user guide:
[Scoring](https://bastien-gallay.github.io/lucid-lint/guide/scoring.html).

## Supported formats

- Markdown (`.md`, `.markdown`)
- Plain text (`.txt`)
- stdin

<!-- lucid-lint disable-next-line excessive-commas -->

For other formats (AsciiDoc, reStructuredText, HTML, docx, PDF), convert first with [Pandoc](https://pandoc.org):

```bash
pandoc document.docx -t markdown | lucid-lint check -
```

Native support for more formats is on the [roadmap](ROADMAP.md).

## Profiles

| Profile | Target audience |
|---|---|
| `dev-doc` | Technical docs, API references, ADRs |
| `public` | General audience, marketing, product copy |
| `falc` | Easy-to-Read / Facile À Lire et à Comprendre |

See [RULES.md](RULES.md) for per-rule thresholds.

## Rules

<!-- lucid-lint disable-next-line excessive-commas -->
<!-- lucid-lint disable-next-line long-enumeration -->

17 rules grouped into 5 scoring categories — **Structure · Rhythm · Lexicon · Syntax · Readability**. Full reference in [RULES.md](RULES.md).

## Configuration

Create a `lucid-lint.toml` in your project root:

```toml
[default]
profile = "public"

[rules.sentence-too-long]
max_words = 20

[rules.passive-voice]
enabled = false

[scoring]
category_max = 20
category_cap = 15

[scoring.weights]
sentence-too-long = 3
```

## Editor integration

Planned for v0.2:

- VS Code extension
- Neovim LSP support
- pre-commit hook

## Documentation

Full documentation at [https://bastien-gallay.github.io/lucid-lint](https://bastien-gallay.github.io/lucid-lint) (built with mdBook).

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

Especially welcome:

- Rule proposals (open an issue first)
- Language-specific word lists (connectors, weasel words, jargon)
- Documentation improvements
- Benchmarks and corpus contributions

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

<!-- lucid-lint disable-next-line sentence-too-long -->

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
