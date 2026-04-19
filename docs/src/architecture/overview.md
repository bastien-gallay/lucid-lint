# Architecture overview

`lucid-lint` is a small Rust crate with a deliberately simple pipeline.

## Pipeline

```
 input text
     │
     ▼
┌──────────────────────────┐
│ Language detection       │   stop-word ratio heuristic
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Parser                   │   pulldown-cmark or plain text
│ (Markdown | plain)       │
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Document model           │   Section > Paragraph > Sentence
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Rules                    │   Each rule gets the document + language
│ (sentence-too-long, ...) │
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Diagnostics              │   rule_id, severity, location, section, message
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Output formatter         │   TTY (default) or JSON
└──────────────────────────┘
```

## Key types

- **`Diagnostic`** — the output unit. Minimal and stable.
- **`Rule`** (trait) — `fn check(document, language) -> Vec<Diagnostic>`.
- **`Document`** — the parser's output. Section-aware.
- **`Engine`** — bundles a profile and a set of rules, exposes `lint_str`, `lint_file`, `lint_stdin`.

## Design principles

These principles are enforced in code review. See [Design decisions](./design-decisions.md) for background.

1. **Make impossible states impossible** — newtypes, enums with data, `NonZeroU32`.
2. **Functional style** where it helps — iterator chains, pure rule functions.
3. **Atomic rules** — one rule, one signal.
4. **Deterministic core** — no network, no LLM, no env-dependent behavior.
5. **YAGNI** — no speculative abstractions.

## Module layout

```
src/
├── lib.rs             — library root
├── main.rs            — binary entry point
├── cli.rs             — clap CLI
├── config.rs          — profile presets, config file parsing
├── engine.rs          — orchestration
├── language/          — detection + per-language data
├── parser/            — Markdown + plain + tokenizer + document model
├── rules/             — one file per rule
├── output/            — TTY + JSON formatters
└── types.rs           — domain types (Diagnostic, Severity, Location, ...)
```
