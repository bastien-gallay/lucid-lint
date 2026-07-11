# Contributing to lucid-lint

Thank you for your interest in contributing.

This document describes how to set up your environment, how to propose changes, and the quality bar we apply to contributions.

## Ground rules

- Be respectful. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
- Open an issue before large changes. A quick discussion saves everyone time.
- Small focused PRs are easier to review than sprawling ones.
- All contributions are licensed under the project's dual MIT / Apache-2.0 license.

## Ways to contribute

We welcome all of the following:

- 🐛 **Bug reports** via GitHub issues
- 💡 **Rule proposals** via GitHub discussions or issues labeled `rule-proposal`
- 📝 **Documentation improvements** including rule explanations, examples, and translations
- 🌍 **Language word lists** (connectors, weasel words, jargon, acronyms) for languages we support or want to support
- 🧪 **Corpus contributions** — real-world text samples that help us validate rules
- 🔬 **Research references** — if you know a paper we should cite, open an issue
- 💻 **Code contributions** — rules, parsers, output formats, performance improvements

## Environment setup

### Prerequisites

- Rust stable (latest recommended, check `rust-toolchain.toml` for the pinned version)
- [just](https://github.com/casey/just) for task running
- [pre-commit](https://pre-commit.com) for git hooks

### First time setup

```bash
git clone https://github.com/bastien-gallay/lucid-lint
cd lucid-lint
just setup
```

<!-- lucid-lint disable-next-line structure.excessive-commas -->

`just setup` installs required Cargo components (`rustfmt`, `clippy`, `cargo-insta`, `cargo-llvm-cov`, `agnix-cli`), installs pre-commit hooks, and runs a sanity check. `agnix` validates `AGENTS.md` / `.agent/` / `CLAUDE.md` and runs as part of `just check` via the `lint-agents` recipe; config lives in `.agnix.toml`.

### Common commands

```bash
just test           # Run all tests
just test-watch     # Re-run tests on file change
just lint           # Run clippy with project-level deny list
just fmt            # Format code
just coverage       # Generate coverage report
just docs           # Build the mdBook documentation
just docs-serve     # Serve docs locally with hot reload
just check          # Run all checks (format, lint, test, coverage)
just snapshot       # Update insta snapshots after intentional changes
```

## Quality bar

Every contribution should meet the following standards. CI enforces most of this automatically.

### Correctness

- Code compiles with zero warnings (`-D warnings`).
- Clippy passes with zero warnings at our configured level.
- All tests pass on Linux, macOS, and Windows.

### Tests

- New rules require unit tests and at least one integration snapshot.
- Bug fixes require a regression test.
- Aim for high coverage on rule logic. We track coverage with `cargo-llvm-cov`.

### Style

- Formatting follows `rustfmt` with project settings in `rustfmt.toml`.
- Clippy rules follow `clippy.toml` plus the deny-list in `Cargo.toml`.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):
  - `feat: add new rule X`
  - `fix: correct false positive in rule Y`
  - `docs: clarify rule Z threshold`
  - `refactor: extract parser helpers`
  - `test: cover edge case W`

### Design principles

See [CODING_STANDARDS.md](CODING_STANDARDS.md) for detailed guidance. Highlights:

- **Make impossible states impossible** via rich typing.
- **Prefer functional style** when it improves clarity.
- **Atomic rules** — one rule, one signal.
- **Deterministic by default** — LLM-based rules live in plugins.
- **YAGNI** — don't add abstractions until a second use case appears.

## Proposing a new rule

Before writing code:

1. Open an issue with the `rule-proposal` label.
2. Include:
   - The cognitive load problem the rule addresses
   - Detection approach (deterministic if possible)
   - False positive risks and mitigations
   - Research references if any
   - Proposed default thresholds per profile
3. Wait for maintainer feedback before investing time.

## Adding or modifying a rule — documentation contract

Every rule lands on five surfaces. CI (`tests/rule_docs_coverage.rs`) enforces the first four; the fifth is manual.

1. **Source** — `src/rules/<category>/<rule-name>.rs`. Rule IDs use the `category.rule-name` form (F29-slim): the category prefix matches the parent subdirectory, and the rule-name part matches the filename (kebab-case in the ID, snake_case on disk). Use [`src/rules/structure/sentence_too_long.rs`](src/rules/structure/sentence_too_long.rs) as the template.
2. **Wiring** — register the rule in three places, and keep them in sync:
   - `rules::default_rules` in [`src/rules/mod.rs`](src/rules/mod.rs), plus the `pub mod` line in the matching [`src/rules/<category>/mod.rs`](src/rules/)
   - `Category::for_rule` in [`src/types.rs`](src/types.rs) derives the category from the id prefix, so no match arm needed — but the id must use a known category prefix (`structure.`, `rhythm.`, `lexicon.`, `syntax.`, `readability.`).
   - `scoring::WEIGHTED_RULE_IDS` (and `default_weight_for` if non-default) in [`src/scoring.rs`](src/scoring.rs), plus a matching `doc!()` line in [`src/explain.rs`](src/explain.rs) mapping the id to its docs slug.
3. **Docs page** — `docs/src/rules/<slug>.md` (kebab slug today; docs tree rearchitecture into category subdirs is a later slice). The H1 must be `` `<rule-id>` `` with the full `category.rule-name` form, and the page must declare `| **Category** | `` `<category>` `` |` matching the id prefix. Add an entry to [`docs/src/SUMMARY.md`](docs/src/SUMMARY.md).
4. **Tests** — unit tests inside the rule file, one `insta` snapshot, and a corpus fixture under `tests/corpus/{en,fr}/` (both if the rule is language-dependent).
5. **Changelog** — add a line to the `## [Unreleased]` section of [`CHANGELOG.md`](CHANGELOG.md) mentioning the rule ID. CI diffs rule files against `origin/main` and fails the build if the rule ID is missing from Unreleased.

The same contract applies when you **modify** a shipped rule (new parameter, changed threshold, refined detection). Only step 1 is optional in that case — the other four are still required.

### Marking version-gated changes (`.since-version`)

When a rule's behavior changes in a release, mark the affected paragraph with the `since-version` callout so readers know what shifted and when. The styling lives in [`docs/theme/css/lucid-layout.css`](docs/theme/css/lucid-layout.css) (search `.since-version`).

EN:

```html
<aside class="since-version" aria-label="New in v0.3">

<span class="since-version__tag">Since v0.3</span> — One-line summary
of what changed, in body voice.

</aside>
```

FR mirror:

```html
<aside class="since-version" aria-label="Nouveauté en v0.3">

<span class="since-version__tag">Depuis v0.3</span> — Résumé en une
ligne, voix corps de texte.

</aside>
```

Notes:

- Blank lines around the inner content are required so mdBook re-enables markdown parsing inside the `<aside>`.
- Ship the callout to `main` *with* the code slice that introduces the behavior — it's safe ahead of the tagged release because the badge itself dates the change.
- Remove the callout one minor cycle after the release lands (e.g. drop `Since v0.3` notes during the `v0.4` polish pass) so rule pages don't accumulate version archaeology. Long-term history belongs in `CHANGELOG.md`.

### Docs links stay inside `docs/src/`

mdBook only renders files under `docs/src/`. Any relative link written from a page inside `docs/src/` must resolve to another page inside `docs/src/` — a `(../../RULES.md)` or `(../../ROADMAP.md)` link points outside the mdBook tree and renders as a 404 on the published site.

When a canonical target is missing:

1. **Stable, high-confidence content** (a shipped feature, a settled convention) → create a short page under `docs/src/guide/` (or `docs/src/architecture/`) and link to it. See [`docs/src/guide/suppression.md`](docs/src/guide/suppression.md) as an example.
2. **Future content** → create a placeholder page and add a roadmap entry so future contributors know where the full version should land.

Absolute `https://github.com/…` URLs remain acceptable for deliberate "see the repo file" references (LICENSE, root-level `RULES.md` / `ROADMAP.md`). The test `docs_links_stay_inside_docs` in [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs) fails on any `](../../…)` pattern in a `docs/src/**/*.md` file.

## Language word lists

Lists for `lexicon.weasel-words`, `rhythm.repetitive-connectors`, `lexicon.jargon-undefined`, and stoplists live in `src/language/`. PRs are very welcome to:

- Add missing items
- Refine existing entries
- Add support for new languages

Include a short rationale in the PR description and cite a style guide or reference where possible.

## Pull request checklist

Before opening a PR:

- [ ] `just check` passes locally
- [ ] New code has tests
- [ ] Snapshots are up to date (`just snapshot` if you changed output)
- [ ] Documentation updated if behavior changed
- [ ] Commit messages follow Conventional Commits
- [ ] PR description explains the why, not just the what

## Review process

- A maintainer will review within a week (usually faster).
- Feedback is meant to improve the contribution, not criticize the contributor.
- Once approved, a maintainer merges. Squash-merge is the default.

### Continuous integration

A PR gates on a **single required check, `CI success`** — an aggregator job that
passes only when every real gate (format, clippy, Ubuntu tests, MSRV, docs,
`cargo-deny`, actionlint) passed or was path-skipped. This lets us rename or add
jobs without editing branch protection, which matches required checks by exact
name.

Some checks are **signals, not gates** — they run on the PR and show a red check
if they fail, but never block a merge, because branch protection requires only
`CI success` and these jobs are excluded from it (they are *not* required
checks):

- **Spell check (typos)** and **Link check (lychee)** — a failure is a visible
  red check you can merge past. Fix it when it's a real typo or a genuinely
  broken internal link; otherwise it's noise (external rot, a flaky host).
- **Cross-platform tests (macOS/Windows)** run only *after* merge on `main` and
  on `release/*` branches — never on PRs. A red run there flags a platform
  regression to the maintainer without slowing PRs.
- **CodeQL** (SAST) runs on `main` + weekly, never on PRs. **Coverage**
  (Codecov) and the **dogfood** self-lint are informational.

## Roadmap conventions

`ROADMAP.md` is a **generated artifact** — never edit it by hand. The source of truth lives under `.roadmap/` (gitignored; symlinked to the maintainer's `.personal/` repo): one Markdown file per feature in `.roadmap/features/<slug>.md`, plus a `config.toml` defining the version-bucket order. See the [design decision](docs/src/architecture/design-decisions.md) and the [F-roadmap-toml-source](ROADMAP.md#f-roadmap-toml-source) row for *why* the file is generated.

### Feature IDs

- New entries use `F-<kebab-slug>` (e.g. `F-roadmap-slug-ids`).
- Slugs are coined locally — **no central counter, no reservation**.
- Uniqueness is enforced by [`tests/roadmap_id_uniqueness.rs`](tests/roadmap_id_uniqueness.rs) (runs offline in `cargo test`, re-runs in CI as a backstop).
- Each ID gets an HTML anchor on first definition: `<a id="f-<slug>"></a>`.
- The `F-` prefix is **optional** in branch names and commit subjects — prefer plain `feat/<slug>` for branches and Conventional-Commit scope syntax (`feat(parser): …`) for commits.
- Legacy `F<number>` IDs (F1–F146) are unchanged.

### Splitting a feature into sub-features

- **Default: don't split** — keep one ROADMAP row with a checklist.
- Split only when sub-items ship on independent timelines, get cited from elsewhere, or get prioritised differently in MoSCoW.
- When a split is warranted, coin sub-slugs as `<parent-slug>-<descriptor>` (e.g. `F-fix-mode-redundant-intensifier` under `F-fix-mode`). The parent stays as the umbrella narrative entry with cross-refs to children; each child gets its own `<a id="f-<sub-slug>"></a>` anchor and the standard surface contract (ROADMAP row + CHANGELOG entry on land).
- Legacy numeric splits (`F35a` / `F35b` / `F78b` / `F105b`) are grandfathered — the letter-suffix form is closed for new entries.

Regenerating the artifact is a release-prep step — see [Maintainer release prep](#maintainer-release-prep).

## Maintainer release prep

`ROADMAP.md` is regenerated only at release-prep time (source layout and ID rules: [Roadmap conventions](#roadmap-conventions)).

On the **release-prep PR** (the one that bumps `Cargo.toml` and CHANGELOG ahead of a tag), regenerate the artifact and commit it with the prep changes:

```bash
just validate-roadmap   # schema, slug uniqueness, anchor drift
just regen-roadmap      # writes ROADMAP.md
git add ROADMAP.md
```

Both recipes silent-pass on checkouts without a `.roadmap/` source, so they're safe to run on contributor branches and in CI — they only do work on the maintainer's machine.

CI does **not** regenerate `ROADMAP.md`. The release-prep step is the only point at which the artifact is refreshed; in between, it lags by up to one release cycle (accepted trade-off, see the ADR).

The `just regen-roadmap` / `just validate-roadmap` recipes call the `roadmap` binary, which lives in its own repository — [bastien-gallay/roadmap-cli](https://github.com/bastien-gallay/roadmap-cli). Install it once:

```bash
cargo install --git https://github.com/bastien-gallay/roadmap-cli
roadmap validate
```

## Release cadence

Patch releases are **trigger-based**, not calendar-based. Cut a patch
when **user-facing** work has landed on `main`. Chores and Experimental
rules ride along on the same release; a quiet period without
user-facing work is a signal to *not* cut, not to ship a noise patch.
Rationale recorded in
[F-release-policy-codify](ROADMAP.md#f-release-policy-codify).

User-facing triggers — any one is sufficient to cut a patch:

- Bug fix or correctness fix.
- Performance win (initial threshold: ≥ 5 % wall-clock improvement on
  the project's representative bench corpus, justified per case).
- UX, TTY, or render/output change visible to a default-config run.
- Security fix.

Items that **do not** trigger a patch on their own — they ride along
on the next user-facing patch:

- New `Status::Experimental` rules (opt-in, dormant by default — see
  [Rule lifecycle and SemVer](RULES.md#rule-lifecycle-and-semver)).
- CI / pre-commit / lint / dep-bump chores.
- Docs-only changes (including FR translations).
- ROADMAP / CONTRIBUTING / internal-policy updates.
- A new dependency is classified as a *chore* unless it changes
  runtime behavior visible to a default-config run, in which case
  it counts as the relevant user-facing trigger above.

Quiet release periods are normal — no patch is better than a patch
with nothing for users. The README states this explicitly so quiet
periods do not read as project abandonment.

The lifecycle policy that governs which rule changes are allowed in
patch vs minor vs major releases lives in
[`RULES.md` — Rule lifecycle and SemVer](RULES.md#rule-lifecycle-and-semver).
The `CHANGELOG.md` "Rule lifecycle changes" subsection template
documents the per-rule entry shape required at each `Experimental →
Stable` flip.

## Getting help

- GitHub Discussions for questions
- GitHub Issues for bugs and proposals
- Open a draft PR early if you want early feedback

Thank you for making `lucid-lint` better.
