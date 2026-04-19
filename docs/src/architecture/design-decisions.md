# Design decisions

This page records design decisions made during v0.1 that are worth revisiting before changing.

## Linter model vs scoring model

**Decision**: v0.1 ships as a classic linter with `info` / `warning` severities. A hybrid scoring model (global score + per-category sub-scores + diagnostics) is planned for v0.2.

**Rationale**: shipping the linter form first lets us validate detection quality on real corpora before investing in a scoring model.

## Diagnostic struct is minimal

**Decision**: a `Diagnostic` carries only `rule_id`, `severity`, `location`, `section`, `message`.

**What's NOT stored and why**:

- **`category`** — derivable from `rule_id` via `Category::for_rule`. Storing it would duplicate information and risk drift.
- **`weight`** — a v0.2 feature for the scoring model. Not useful yet.
- **`suggestion`** — a v0.2 feature. Current messages are actionable on their own.

**What IS stored and why**:

- **`section`** — recomputing it after the fact would require re-parsing the document to walk headings and match locations. The storage cost is an `Option<String>` per diagnostic; the recompute cost is a second full parse.

## Deterministic core, plugins for the rest

**Decision**: the core ships only deterministic rules. LLM-based rules, network-backed rules, or ML-model-backed rules live in optional plugin crates (planned v0.3).

**Rationale**: a pre-commit hook that takes 5 seconds and varies between runs is worse than no hook. Determinism is non-negotiable in the happy path.

## Bilingual EN/FR from day one

**Decision**: every language-dependent rule supports English and French from v0.1.

**Rationale**: most French-speaking OSS developers write docs in English. Targeting French only would miss the majority. Supporting both from day one is cheap and signals the ambition.

## Single readability formula in v0.1

**Decision**: v0.1 uses Flesch-Kincaid Grade Level for all languages. Language-specific formulas (Kandel-Moles for French, SMOG, Coleman-Liau) are deferred to v0.2.

**Rationale**: Flesch-Kincaid is understood, reproducible, and well-behaved. Adding three more formulas before validating the basics would be premature optimization.

## Markdown + plain text + stdin, Pandoc for the rest

**Decision**: native support for `.md`, `.markdown`, `.txt`, and stdin in v0.1. Other formats (AsciiDoc, HTML, docx, PDF) use Pandoc as a pre-processor.

**Rationale**: Markdown covers the overwhelming majority of open-source and technical writing. Pandoc is free, ubiquitous, and removes the burden of maintaining multiple parsers.

## One file per rule

**Decision**: each rule lives in its own file under `src/rules/` with a consistent structure (struct, config, Rule impl, tests).

**Rationale**: makes adding a rule a well-defined operation (new file from template), and makes reviewing easy (one rule, one PR, one file to read).

## Stop-word heuristic for language detection

**Decision**: v0.1 detects language by stop-word ratio. No external dependency.

**Rationale**: short, deterministic, no runtime cost. For the cases where it fails (very short texts, code-heavy docs), the `unknown` fallback is safe.

## Profile presets as enum variants

**Decision**: profiles are `Profile::DevDoc | Public | Falc`. They cannot be defined in user config in v0.1.

**Rationale**: adding custom profiles is a speculative abstraction until someone asks for it. Per-rule overrides are enough to cover 95% of the "I want a slightly different preset" cases.

## References to follow before changing these

- [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) — the authoritative rule reference
- [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md) — future work tracked
- [`CODING_STANDARDS.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/CODING_STANDARDS.md) — day-to-day conventions
