# AGENTS.md

Guidance for coding agents (Claude Code, Cursor, Copilot, …) working on this repository.

## Project

`lucid-lint` is a cognitive accessibility linter for prose. Rust, bilingual EN/FR.

Read in order:

1. [README.md](README.md) — what it is and how it's used
2. [RULES.md](RULES.md) — the 17 rules shipped in v0.1
3. [CODING_STANDARDS.md](CODING_STANDARDS.md) — design principles
4. [ROADMAP.md](ROADMAP.md) — what's planned
5. [CONTRIBUTING.md](CONTRIBUTING.md) — contribution workflow

Current project state, design decisions, and "what NOT to do" specifics live in [`.agent/context.md`](.agent/context.md). Brand voice, palette, typography, and the WCAG AAA accessibility bar live in [`.impeccable.md`](.impeccable.md) — read before any frontend, mdBook, branding, or marketing work; `/impeccable` reads it automatically.

Claude-specific: this repo uses `.agent/` (not `.claude/`); `AGENTS.md` replaces `CLAUDE.md`.

## Prime directives

### 1. YAGNI

No abstractions for hypothetical futures. A trait with one impl, a field with no reader, a plugin system with no plugin — red flags.

Already removed by past design rounds. Do not reintroduce without confirming with a human:

- `category` on `Diagnostic` — derive via `Category::for_rule`.
- `suggestion` on `Diagnostic` — deferred past v0.2.
- A pluggable `Parser` trait around the single `MarkdownParser`.

`weight` on `Diagnostic` is intentional (v0.2, F14 hybrid scoring). Seeded from `scoring::default_weight_for(rule_id)`; rules rarely override via `with_weight`. Do not remove it or recompute on the fly.

### 2. Typing discipline

Make impossible states impossible: newtypes for domain primitives, enums with associated data, `NonZeroU32` for bounded counts. Extend the enum rather than weaken the type.

### 3. Deterministic core

No network, no LLM, no env-dependent behavior in core. Such logic lives in plugin crates (`lucid-lint-llm`, `lucid-lint-nlp`).

### 4. Bilingual from day one

Language-specific data goes under `src/language/{en,fr}/`. Rules accept `Language` as a parameter — never hardcode. Tests cover FR and EN. No "EN now, FR later".

### 5. Tests required

Every new rule: unit tests in the rule file (`#[cfg(test)] mod tests`) + at least one `insta` snapshot + corpus fixture under `tests/corpus/{en,fr}/`. Bug fixes: regression test before the fix.

### 6. Atomic rules and the acceptance filter

One signal per rule file under `src/rules/`. A rule that "would also like to detect X" makes X a new rule.

A new rule lands in core only if it passes every point:

1. **Atomic** — one detection goal.
2. **Cognitive-load-grounded** — cites research or a recognised standard (WCAG, RGAA, FALC, BDA, IFLA, CDC Clear Communication Index, plainlanguage.gov). No aesthetic-only rules; those go to a future `lucid-lint-style` plugin.
3. **Deterministic in core** — pattern, counter, or window. POS / dependency tree / LLM goes to a plugin.
4. **Bilingual-viable** — language-agnostic, or has a concrete FR + EN path at proposal time.
5. **Category-coherent** — fits one of `structure · syntax · rhythm · lexicon · readability` cleanly. Fits three? Probably two rules.
6. **Balance is monitored, not enforced** — an uneven category is a taxonomy signal, not a quota.

Rules may carry condition tags (F71) from a fixed ontology: `a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`, `non-native`, `general`. Most are `general`.

### 7. Rule documentation contract

Adding *or modifying* a rule means updating five surfaces: source, wiring (`default_rules`, `Category::for_rule`, `scoring::WEIGHTED_RULE_IDS`), `docs/src/rules/<rule-id>.md`, tests, and `CHANGELOG.md` `## [Unreleased]`. Full checklist and CI gating in [CONTRIBUTING.md](CONTRIBUTING.md). The coverage test `tests/rule_docs_coverage.rs` enforces surfaces 1–4.

### 8. Docs links stay inside `docs/src/`

mdBook only serves `docs/src/`. A relative link from any `docs/src/**` page must resolve under `docs/src/` — `../../RULES.md` renders as 404.

When the canonical target is missing, create a short page under `docs/src/guide/` or `docs/src/architecture/` (for stable content) or a placeholder + roadmap entry (otherwise). Absolute `https://github.com/...` URLs remain fine for explicit "see the repo file" references.

The test `docs_links_stay_inside_docs` fails on any `](../../…)` pattern in `docs/src/**/*.md`.

### 9. `examples/local/` is opaque to public surfaces

`examples/local/` is a local-only scratch space (gitignored except its `README.md`). Treat it like `.env`: tools may read and parse it, but **nothing committed to git may reveal what lives inside**.

In source, docs, `CHANGELOG.md`, `ROADMAP.md`, `examples/texts.md`, commit messages, PR descriptions, generated reports:

- No filenames, folder names, slugs, titles, URLs, or identifying snippets from `examples/local/`.
- No aggregate counts that betray local-only existence by subtraction (e.g. `4 / 7` reveals "3 local-only" — publish only the `public_ok` count).
- No "we also have X under local" phrasings.

When a tool needs local-only output (target list, gap map, audit), write it under `examples/local/` so gitignore keeps it off GitHub. The texts pipeline is the reference pattern: `examples/texts.yaml` (tracked, `public_ok` only) ↔ `examples/local/texts.yaml` (gitignored, everything else); merge logic in `scripts/texts_common.py::load_sources()`. The `redistribution` field is the tripwire: only `public_ok` entries are safe to name.

## Conventions

- **Rule IDs**: `category.rule-name` in kebab-case. Prefix matches the `src/rules/` subdir, name matches the filename. Struct is `PascalCase` of the name half. Example: `structure.sentence-too-long` → `src/rules/structure/sentence_too_long.rs` → `struct SentenceTooLong`. Reference rule: that file.
- **Config**: global `lucid-lint.toml`; per-rule `[rules.<rule-id>]`; profiles (`dev-doc`, `public`, `falc`) preset every threshold at once.
- **Output**: human TTY by default with a `score:` summary (v0.2+); `--format=json` is the stable schema (`version = 2`, carries `score`, `category_scores`, per-diagnostic `weight` — see `docs/src/guide/scoring.md`); SARIF v2.1.0 planned.

## Definition of "done"

- [ ] Compiles with zero warnings.
- [ ] `just check` passes (fmt, clippy, test, coverage).
- [ ] New rules have unit + snapshot + corpus tests.
- [ ] Public API has doc comments.
- [ ] Behavior changes are documented (`RULES.md`, `docs/`).
- [ ] Commit messages follow Conventional Commits.

## Known pitfalls

- **Markdown**: parse with `pulldown-cmark`. Never with regex. Most rules must exclude code blocks — see the helper in `src/parser/markdown.rs`.
- **Sentence splitting**: use the project `Tokenizer` in `src/parser/tokenizer.rs`. Never split on `.` alone — abbreviations (`Dr.`, `e.g.`), decimals, and ellipses are handled there.
<!-- lucid-lint disable-next-line lexicon.weasel-words -->
- **Language detection**: stop-words ratio heuristic in `src/language/detect.rs`. Returns `Language::Unknown` when confidence is low — respect it and skip language-specific rules rather than guess.
- **Unicode**: prose is Unicode. Use `unicode-segmentation` for graphemes and words. Never index strings by byte offset.
- **Performance**: avoid string clones, per-rule re-parsing (use the shared AST), and regex in the hot path when a direct scan works.

## When in doubt

Open an issue before large refactors. Ask a maintainer before adding a dependency. Prefer small PRs over sweeping ones.

## Prose style in agent answers

We dogfood the tool on ourselves. Write your own prose — chat replies, commit messages, PR descriptions, docs contributions — to pass our own rules as much as reasonably possible.

### Match the language to the user

- User writes in French → answer in French, targeting the **FALC** profile (Facile À Lire et à Comprendre).
- User writes in English → answer in English, targeting the spirit of **plainlanguage.gov** and the **CDC Clear Communication Index** — the closest EN equivalents to FALC, both already cited in directive 6.

Do not mix languages inside a single answer (technical identifiers, file paths, and quoted output excepted).

### Two profiles, by surface

| Surface                                          | Target profile                    |
| ------------------------------------------------ | --------------------------------- |
| Chat replies, explanations, docs prose           | `falc` (FR) / plain-language (EN) |
| Commit messages, PR titles + bodies, code review | `dev-doc`                         |

Conversational prose tolerates the strict profile; commits and reviews carry identifiers, flags, and logic that fight FALC thresholds, so `dev-doc` stays the right target there. Threshold tables: [`docs/src/guide/profiles.md`](docs/src/guide/profiles.md).

### Concrete moves for the strict profile

The label alone is too vague. When targeting FALC / plain-language:

- One idea per sentence. Aim for ~15 words, hard cap around 20.
- Active voice. Concrete verbs over nominalizations (*decide*, not *make a decision*).
- Define a technical term the first time it appears, or pick a simpler word.
- Prefer short lists over long paragraphs when enumerating.
- No weasel words, no redundant intensifiers, no all-caps shouting.
- Avoid deep subordination and dense punctuation bursts.

### Exemptions and escape hatch

Code blocks, file paths, identifiers, command names, flag names, quoted tool output, and error messages stay verbatim — the profile applies to the prose around them, not to them.

This is a soft target, not a lint gate. When a rule would force unclear or misleading phrasing on technical content, break the rule. Before sending a long answer, re-read it once against the checklist above; do not regress to default verbosity mid-response.
