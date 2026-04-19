# Introduction

`lucid-lint` is a cognitive accessibility linter for prose.

It reads Markdown or plain text and reports issues that increase the **cognitive load** a reader must spend to understand the content — especially readers whose attention is already stretched by ADHD, fatigue, a second language, a noisy environment, or an accessibility-sensitive context.

## What makes it different

Most prose linters measure style (`write-good`), grammar (`Antidote`), or surface readability (Flesch). `lucid-lint` focuses on **cognitive load**, grounded in research from Sweller, Gibson, Graesser, and others.

- **Bilingual EN/FR** from day one, with equal quality.
- **Deterministic** by default: identical input, identical output. LLM-based rules live in optional plugins.
- **CI-native**: plain-text and JSON outputs, exit codes that integrate cleanly with pre-commit and GitHub Actions.
- **Profile-based**: pick `dev-doc`, `public`, or `falc` (Easy-to-Read), then override per rule if you want.

## Project status

`lucid-lint` is in v0.1 active development. The 16 rules from [`RULES.md`](https://github.com/YOUR_USER/lucid-lint/blob/main/RULES.md) are the planned scope for the first release.

## Quick taste

```bash
cargo install lucid-lint

# Lint a single file
lucid-lint check README.md

# Use the FALC profile for maximum strictness
lucid-lint check --profile=falc docs/

# Pipe from stdin
echo "This is a test sentence." | lucid-lint check -

# JSON output for CI
lucid-lint check --format=json docs/
```

## Where to next

- [Installation](./guide/installation.md) — how to install
- [Quick start](./guide/quick-start.md) — a 5-minute walkthrough
- [Profiles](./guide/profiles.md) — pick the one that fits
- [Rules reference](./rules/index.md) — all 16 rules explained

## License

Dual-licensed under MIT or Apache-2.0, at your option.
