# Design decisions

This page records design decisions made during v0.1 that are worth revisiting before changing.

## Linter model vs scoring model

**Decision**: v0.1 shipped as a classic linter with `info` / `warning`
severities. v0.2 added a hybrid scoring model (global score +
per-category sub-scores + diagnostics) on top, without removing the
linter form.

**Rationale**: shipping the linter form first let us validate detection
quality on real corpora before adding the aggregation layer. The scoring
layer is additive — consumers that only care about diagnostics ignore
the scorecard.

## Hybrid scoring model (v0.2)

**Decision**: global + 5 per-category sub-scores, all in `X / max` form.
Composition stacks a weighted sum, density normalization (per 1 000
words, floored at 200), and a per-category cap. 5 fixed categories:
`Structure · Rhythm · Lexicon · Syntax · Readability`.
New `Diagnostic.weight` field, new `--min-score=N` CLI flag.

**Rationale** (full brainstorm at [`brainstorm/20260420-score-semantics.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/brainstorm/20260420-score-semantics.md)):

- `X / max` over 0–100: arbitrary max lets us re-tune without claiming
  the 80 we ship today is the same 80 next release. The `/impeccable`
  skill already uses this convention.
- 5 fixed categories: couples nothing to a rule rename; uses the
  `category_of(rule_id)` helper already decided in v0.1. Derive-from-
  prefix (plan B) was rejected because it would require renaming 17
  rules for [F14](../roadmap.md) alone.
- Three composition mechanics stacked: no single one covers every
  failure mode. Density alone punishes short docs; weights alone lose
  to a runaway rule; caps alone can't reflect cost magnitude.
- Letter grades, traffic lights, pass/fail margin, reading-time-seconds
  were cut from the v0.2 design after a first-principles pass ([F-score-letter-grade](../roadmap.md#f-score-letter-grade)–[F-reading-time-score](../roadmap.md#f-reading-time-score)
  in ROADMAP). They duplicate function-1 (at-a-glance) that the number
  already serves.
- Actionability (function-2) is delivered by the diagnostics list, not
  the score. So sub-scores can afford to be minimal — [F37](../roadmap.md) makes sure
  diagnostic messages hold up the actionability side of the contract.

## Diagnostic struct

**Decision**: a `Diagnostic` carries `rule_id`, `severity`, `location`,
`section`, `message`, and (as of v0.2) `weight`.

**What's NOT stored and why**:

- **`category`** — derivable from `rule_id` via `Category::for_rule`. Storing it would duplicate information and risk drift.
- **`suggestion`** — still deferred; current messages are actionable on their own.

**What IS stored and why**:

- **`section`** — recomputing it after the fact would require re-parsing the document to walk headings and match locations. The storage cost is an `Option<String>` per diagnostic; the recompute cost is a second full parse.
- **`weight`** (v0.2) — seeded at emission from `scoring::default_weight_for`
  so that user overrides (via config) and rule-level overrides (via
  `with_weight`) both flow through aggregation without a second lookup.

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

## ROADMAP source-of-truth pipeline (v0.2.x+)

**Decision**: `ROADMAP.md` is demoted from edited source to *generated artifact*. The source-of-truth becomes a structured set of files under `.roadmap/` (gitignored), one markdown file per feature with TOML front-matter, plus narrative chunks. A small Rust workspace member (`crates/roadmap-cli`) provides `add` / `generate` / `validate` / `rename` subcommands. The generator is invoked locally during release prep; the regenerated `ROADMAP.md` is committed on the release-prep PR. CI does not regenerate. Scoped under [F-roadmap-toml-source](../roadmap.md#f-roadmap-toml-source).

**Rationale**:

- Branch protection on `main` (in place since 2026-05-03 via [F-repo-config-hardening](../roadmap.md#f-repo-config-hardening)) forces every `ROADMAP.md` tweak through the worktree → branch → PR → CI → merge → cleanup cycle. Forecast steady-state was 10–30 ROADMAP-only edits per week. The PR review value on those edits is null (solo author), so the ceremony was pure overhead.
- Path-scoped ruleset bypass on `ROADMAP.md` would weaken the branch-protection signals tracked by the OpenSSF Scorecard / Best Practices badges. Demoting the file from `main` source preserves those signals untouched.
- Per-feature files give per-feature git diffs, kill schema lock-in (front-matter is per-file optional), and let narrative sections live as plain markdown rather than TOML strings.
- Rust over Python for the generator: reuses `pulldown-cmark` already in dependencies, folds tests into `cargo test`, single-toolchain maintenance, and stays extractable as a standalone crate if the tool matures.
- Local generator (not CI) avoids granting CI any access to `.roadmap/` (gitignored and machine-local). Release cadence — not real-time — was an accepted trade-off; the public `ROADMAP.md` artifact updates per `v*` tag.
- Day-1 blockers on landing: deterministic `<a id="…">` anchor emission (so existing `[F46](#f46)`-style cross-links from PRs and commits keep resolving), an `add` templating subcommand (so creating a feature is one keystroke, not a regression), and a round-trip determinism test (regenerate the artifact, diff against committed, fail on drift).

**Emergency fallback**: if `crates/roadmap-cli` work overruns budget, the file moves instead to a `roadmap` orphan branch with direct push and the same `.md` shape — preserves Scorecard signals via a different mechanism, at the cost of a non-standard branch layout. Documented as the escape hatch but not the chosen path.

## References to follow before changing these

- [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) — the authoritative rule reference
- [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md) — future work tracked
- [`CODING_STANDARDS.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/CODING_STANDARDS.md) — day-to-day conventions
