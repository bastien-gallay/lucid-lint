# AGENTS.md

This file provides guidance to coding agents (Claude Code, Cursor, GitHub Copilot Workspace, etc.) when working on this repository.

## Project overview

`lucid-lint` is a cognitive accessibility linter for prose, written in Rust, bilingual EN/FR.

Read in order:

1. [README.md](README.md) — what it is and how it's used
2. [RULES.md](RULES.md) — the 17 rules shipped in v0.1
3. [CODING_STANDARDS.md](CODING_STANDARDS.md) — design principles
4. [ROADMAP.md](ROADMAP.md) — what's planned
5. [CONTRIBUTING.md](CONTRIBUTING.md) — contribution workflow

## Prime directives for agents

### 1. Respect YAGNI

Do not add abstractions speculatively. A trait for a single implementation, a field for an unimplemented feature, a plugin system with no plugin — these are red flags.

Previous design discussions explicitly removed:

- `category` from the `Diagnostic` struct (derivable from `rule_id` via `Category::for_rule`)
- `suggestion` from the `Diagnostic` struct (still deferred past v0.2)
- A pluggable `Parser` trait with a single `MarkdownParser` implementation

`weight` was added to `Diagnostic` in v0.2 as part of the hybrid scoring
model (F14). Seeded at emission from `scoring::default_weight_for(rule_id)`;
rules rarely need to override via `with_weight`. Do not remove the field
or re-derive it on the fly.

If you find yourself wanting to add one of the removed items, stop and
confirm with a human.

### 2. Preserve typing discipline

The codebase follows "make impossible states impossible":

- Newtype wrappers for domain primitives.
- Enums with associated data.
- `NonZeroU32` or similar for bounded primitives.

Do not weaken these types. If you need flexibility, extend the enum or add a variant.

### 3. Atomic rules

Each rule file under `src/rules/` implements one signal. Do not merge detection logic across rules. If a rule "would like to also detect X", file X is a new rule.

### 4. Deterministic core

No network, no LLM, no environment-dependent behavior in the core. Such logic must live in an optional plugin crate.

### 5. Bilingual from day one

When adding a language-dependent rule:

- Language-specific data goes in `src/language/{fr,en}/` or equivalent.
- The rule itself should accept the language as a parameter, not hardcode one.
- Tests must cover FR and EN cases.

### 6. Tests are not optional

Every new rule requires:

- Unit tests in the rule file (`#[cfg(test)] mod tests`)
- At least one snapshot test (`insta`)
- Corpus fixtures in `tests/corpus/{en,fr}/`

Bug fixes require a regression test before the fix.

### 7. New-rule acceptance filter

A new rule earns its place in core only if it passes every point:

1. **Atomic** — one detection goal. (Restates directive 3.)
2. **Cognitive-load-grounded** — traceable to a cited source
   (cognitive / linguistic research) or a recognised standard: WCAG,
   RGAA, FALC, BDA, IFLA, CDC Clear Communication Index,
   plainlanguage.gov. No aesthetic-only rules — those belong in a
   future `lucid-lint-style` plugin.
3. **Deterministic in core** — pattern, counter, or window.
   LLM / POS / dependency-tree goes to a plugin (`lucid-lint-llm`,
   `lucid-lint-nlp`).
4. **Bilingual-viable** — language-agnostic, or has a concrete FR + EN
   implementation path at proposal time. No "EN now, FR later".
5. **Category-coherent** — fits one of the five categories
   (`structure`, `syntax`, `rhythm`, `lexicon`, `readability`)
   cleanly. If a rule fits three, it is probably two rules.
6. **Balance is monitored, not enforced** — a category growing or
   staying thin is a signal to revisit the taxonomy, not a reason to
   invent or reject rules.

Rules may additionally declare condition tags (F71) from the fixed
ontology `a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`,
`non-native`, `general`. Most rules are `general`; some carry multiple
tags.

## Project-specific conventions

### Naming

- Rule IDs: `kebab-case` (`sentence-too-long`). Match the filename.
- Rule struct: `PascalCase` matching rule ID (`struct SentenceTooLong`).
- Rust files: `snake_case` (`sentence_too_long.rs`).

### Rule structure

Every rule implements a common `Rule` trait, exposes a default config, and lives in its own file under `src/rules/`. See `src/rules/sentence_too_long.rs` as the canonical example.

### Configuration

- Global config: `lucid-lint.toml` in the project root.
- Per-rule overrides under `[rules.<rule-id>]`.
- Profiles (`dev-doc`, `public`, `falc`) are presets that set all thresholds at once.

### Output

- Default: human-readable TTY with colors when stdout is a tty. Appends a
  `score:` summary line (v0.2+) with optional per-category breakdown.
- `--format=json`: stable JSON schema for CI integration. `version = 2`
  as of v0.2 — carries `score`, `category_scores`, and per-diagnostic
  `weight`. See `docs/src/guide/scoring.md`.
- SARIF v2.1.0 for GitHub Code Scanning is planned for v0.2 (see `ROADMAP.md`).

## Definition of "done" for a change

A change is done when:

- [ ] Code compiles with zero warnings.
- [ ] `just check` passes (fmt, clippy, test, coverage).
- [ ] New rules have unit + snapshot + corpus tests.
- [ ] Public API has doc comments.
- [ ] If behavior changed, documentation (`RULES.md`, `docs/`) is updated.
- [ ] Commit messages follow Conventional Commits.

## Known pitfalls

### Markdown parsing

Use `pulldown-cmark`. Do NOT parse Markdown with regexes. Code blocks must be excluded from most rules; see `src/parser/markdown.rs` for the extraction helper.

### Sentence splitting

Use the project's `Tokenizer` in `src/parser/tokenizer.rs`. Do NOT split on `.` alone: abbreviations (`Dr.`, `e.g.`), decimals, and ellipses are edge cases handled there.

### Language detection

<!-- lucid-lint disable-next-line weasel-words -->

Heuristic based on stop-words ratio. See `src/language/detect.rs`. Returns `Language::Unknown` if confidence is low — respect that and skip language-specific rules rather than guessing.

### Unicode

Prose is Unicode. Use `unicode-segmentation` for graphemes and words. Never index strings by byte offset.

### Performance

`lucid-lint` is marketed as fast. Avoid:

- Cloning strings unnecessarily.
- Re-parsing the document per rule (use the shared AST from the parser phase).
- Regex in the hot path if a direct scan works.

## When in doubt

- Open an issue before large refactors.
- Ask a maintainer before adding a dependency.
- Prefer small PRs over sweeping ones.

## Design context

<!-- lucid-lint disable-next-line long-enumeration -->

Brand voice, palette, typography shortlist, audience, and the WCAG AAA accessibility bar live in [.impeccable.md](.impeccable.md). Read it before any frontend, mdBook, branding, or marketing-surface work. The `/impeccable` skill also reads it automatically.

## For Claude specifically

- The project owner uses `.agent/` instead of `.claude/` for agent-specific files.
- This file (`AGENTS.md`) replaces `CLAUDE.md`.
- Project-specific agent context lives in `.agent/context.md`.
- Design context lives in [.impeccable.md](.impeccable.md) (see above).
