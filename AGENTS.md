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

State and reference pointers live in [`.agent/context.md`](.agent/context.md). Brand voice, palette, typography, and the WCAG AAA accessibility bar live in [`.impeccable.md`](.impeccable.md) — consult before any frontend, mdBook, branding, or marketing work; `/impeccable` reads it automatically.

This repo follows the cross-tool `AGENTS.md` standard at the root and uses `.agent/` for shared agent state. `CLAUDE.md` is a stub that imports `AGENTS.md`.

## Principles

### 1. YAGNI

No abstractions for hypothetical futures. A trait with one impl, a field with no reader, a plugin system with no plugin — red flags.

Already removed by past design rounds; confirm with a human before reintroducing:

- `category` on `Diagnostic` — derive via `Category::for_rule`.
- `suggestion` on `Diagnostic` — deferred past v0.2.
- A pluggable `Parser` trait around the single `MarkdownParser`.

`weight` on `Diagnostic` is intentional (v0.2, F14 hybrid scoring). Seeded from `scoring::default_weight_for(rule_id)`; rules rarely override via `with_weight`. Keep it; do not recompute on the fly.

### 2. Typing discipline

Make impossible states impossible: newtypes for domain primitives, enums with associated data, `NonZeroU32` for bounded counts. Extend the enum rather than weaken the type.

### 3. Deterministic core

No network, no LLM, no env-dependent behavior in core. Such logic lives in plugin crates (`lucid-lint-llm`, `lucid-lint-nlp`).

### 4. Bilingual from day one

Language-specific data lives under `src/language/{en,fr}/`. Rules accept `Language` as a parameter — never hardcode. Tests cover FR and EN. Ship bilingual or ship neither.

### 5. Tests required

Every new rule: unit tests in the rule file (`#[cfg(test)] mod tests`) + at least one `insta` snapshot + corpus fixture under `tests/corpus/{en,fr}/`. For bug fixes, write the regression test before the fix.

### 6. Atomic rules and the acceptance filter

One signal per rule file under `src/rules/`. A rule that "would also like to detect X" makes X a new rule.

A new rule lands in core only if it passes every point:

1. **Atomic** — one detection goal.
2. **Cognitive-load-grounded** — cites research or a recognised standard (WCAG, RGAA, FALC, BDA, IFLA, CDC Clear Communication Index, plainlanguage.gov). Aesthetic-only rules go to a future `lucid-lint-style` plugin.
3. **Deterministic in core** — pattern, counter, or window. POS / dependency tree / LLM goes to a plugin.
4. **Bilingual-viable** — language-agnostic, or has a concrete FR + EN path at proposal time.
5. **Category-coherent** — fits one of `structure · syntax · rhythm · lexicon · readability` cleanly. Fits three? Probably two rules.
6. **Balance is monitored, not enforced** — an uneven category is a taxonomy signal, not a quota.

Rules may carry condition tags (F71) from a fixed ontology — see [RULES.md](RULES.md).

## Contracts (CI-enforced)

- **Rule documentation contract** — adding or modifying a rule means updating five surfaces: source, wiring (`default_rules`, `Category::for_rule`, `scoring::WEIGHTED_RULE_IDS`), `docs/src/rules/<rule-id>.md`, tests, `CHANGELOG.md` `## [Unreleased]`. See [CONTRIBUTING.md](CONTRIBUTING.md). Enforced by `tests/rule_docs_coverage.rs`.
- **Docs links stay inside `docs/src/`** — mdBook only serves `docs/src/`. Relative links like `](../../…)` from `docs/src/**/*.md` render as 404. Use absolute `https://github.com/...` URLs to reach repo files outside the book. Enforced by `docs_links_stay_inside_docs`.
- **`examples/local/` is opaque on public surfaces** — treat like `.env`. Tools may parse it; nothing committed to git may name files, slugs, titles, URLs, or aggregate counts that betray local-only existence (e.g. `4 / 7` reveals "3 local-only" — publish only the `public_ok` count). The `redistribution` field in `examples/texts.yaml` is the tripwire: only `public_ok` entries are safe to name. Reference pattern: `scripts/texts_common.py::load_sources()`.
- **FR pages declare their EN source SHA** — every page under `docs/src/fr/**/*.md` (except intentionally-asymmetric pages: `docs/src/fr/roadmap.md` is a stub pointer; mappings tracked in `scripts/check_lang_staleness.py::ASYMMETRIC_FR_PAGES`) carries an `en-source-sha` HTML-comment stamp on the first line, recording the commit SHA of the EN counterpart at translation time. Shape: `<!-- en-source-sha: 5e24f614b0378a6ff57d9f497d052361e2cfcec3 -->`. mdBook passes HTML comments through unchanged so the stamp is invisible in the rendered page. (YAML front-matter would have been more idiomatic but mdBook does not strip it — `---` renders as `<hr>` and the body lines render as text.) `scripts/check_lang_staleness.py` walks every FR page, compares the stored SHA to the EN counterpart's last-touching commit (`git log -n1 --pretty=%H -- <EN counterpart>`), and reports drift. Soft on PRs via `just docs-lang-staleness`; strict on `main` (set `STRICT=1`) once the existing stale backlog clears. Pair-locked with F92's filename-parity gate: filename parity catches *missing* FR pages, content-SHA parity catches *stale* FR pages. When you update an EN page that has an FR counterpart, either translate the change in the same commit or accept the staleness warning until the FR catches up.

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

## Pitfalls

- **Markdown**: parse with `pulldown-cmark`; never with regex. Most rules must exclude code blocks — use the helper in `src/parser/markdown.rs`.
- **Sentence splitting**: use the project `Tokenizer` in `src/parser/tokenizer.rs`. Splitting on `.` alone breaks abbreviations (`Dr.`, `e.g.`), decimals, and ellipses; the tokenizer handles them.
<!-- lucid-lint disable-next-line lexicon.weasel-words -->
- **Language detection**: stop-words ratio heuristic in `src/language/detect.rs`. Returns `Language::Unknown` when confidence is low — respect it and skip language-specific rules rather than guess.
- **Unicode**: prose is Unicode. Use `unicode-segmentation` for graphemes and words; index by grapheme, never by byte offset.
- **Performance**: reuse the shared AST, avoid string clones, prefer direct scans over regex in hot paths.
- **Safety**: keep `unwrap()` and `expect()` out of library code; surface a typed error instead.

## Prose style in agent answers

We dogfood the tool on ourselves. Write chat replies, commit messages, PR descriptions, and docs to pass our own rules where reasonable.

### Match the language to the user

- User writes in French → answer in French, target the **FALC** profile (Facile À Lire et à Comprendre).
- User writes in English → answer in English, target the spirit of **plainlanguage.gov** and the **CDC Clear Communication Index**.

Keep one language per answer. Technical identifiers, file paths, and quoted output are exempt.

### Two profiles, by surface

| Surface | Target profile |
| --- | --- |
| Chat replies, explanations, docs prose | `falc` (FR) / plain-language (EN) |
| Commit messages, PR titles + bodies, code review | `dev-doc` |

Commits and reviews carry identifiers, flags, and logic that fight FALC thresholds — `dev-doc` is the right target there. Threshold tables: [`docs/src/guide/profiles.md`](docs/src/guide/profiles.md).

### Concrete moves for the strict profile

- One idea per sentence. Aim for ~15 words, hard cap around 20.
- Active voice. Concrete verbs over nominalizations (*decide*, not *make a decision*).
- Define a technical term the first time it appears, or pick a simpler word.
- Prefer short lists over long paragraphs when enumerating.
- Keep weasel words, redundant intensifiers, and all-caps shouting out.
- Keep clauses shallow and punctuation light.

Code blocks, file paths, identifiers, command names, flag names, quoted tool output, and error messages stay verbatim. The profile applies to the prose around them.

This is a soft target, not a lint gate. When a rule would force unclear or misleading phrasing on technical content, break the rule.
