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

<!-- lucid-lint disable-next-line excessive-commas -->

`just setup` installs required Cargo components (`rustfmt`, `clippy`, `cargo-insta`, `cargo-llvm-cov`), installs pre-commit hooks, and runs a sanity check.

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

1. **Source** — `src/rules/<rule-id>.rs`. Use [`sentence_too_long.rs`](src/rules/sentence_too_long.rs) as the template. The rule ID must match the filename (kebab-case).
2. **Wiring** — register the rule in three places, and keep them in sync:
   - `rules::default_rules` in [`src/rules/mod.rs`](src/rules/mod.rs)
   - `Category::for_rule` in [`src/types.rs`](src/types.rs)
   - `scoring::WEIGHTED_RULE_IDS` (and `default_weight_for` if non-default) in [`src/scoring.rs`](src/scoring.rs)
3. **Docs page** — `docs/src/rules/<rule-id>.md`. Copy the template from an existing rule ([`sentence-too-long.md`](docs/src/rules/sentence-too-long.md) is canonical). The H1 must be `` `<rule-id>` `` and the page must declare `| **Category** | `` `<category>` `` |` matching `Category::for_rule`. Add an entry to [`docs/src/SUMMARY.md`](docs/src/SUMMARY.md).
4. **Tests** — unit tests inside the rule file, one `insta` snapshot, and a corpus fixture under `tests/corpus/{en,fr}/` (both if the rule is language-dependent).
5. **Changelog** — add a line to the `## [Unreleased]` section of [`CHANGELOG.md`](CHANGELOG.md) mentioning the rule ID. CI diffs rule files against `origin/main` and fails the build if the rule ID is missing from Unreleased.

The same contract applies when you **modify** a shipped rule (new parameter, changed threshold, refined detection). Only step 1 is optional in that case — the other four are still required.

### Docs links stay inside `docs/src/`

mdBook only renders files under `docs/src/`. Any relative link written from a page inside `docs/src/` must resolve to another page inside `docs/src/` — a `(../../RULES.md)` or `(../../ROADMAP.md)` link points outside the mdBook tree and renders as a 404 on the published site.

When a canonical target is missing:

1. **Stable, high-confidence content** (a shipped feature, a settled convention) → create a short page under `docs/src/guide/` (or `docs/src/architecture/`) and link to it. See [`docs/src/guide/suppression.md`](docs/src/guide/suppression.md) as an example.
2. **Future content** → create a placeholder page and add a roadmap entry so future contributors know where the full version should land.

Absolute `https://github.com/…` URLs remain acceptable for deliberate "see the repo file" references (LICENSE, root-level `RULES.md` / `ROADMAP.md`). The test `docs_links_stay_inside_docs` in [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs) fails on any `](../../…)` pattern in a `docs/src/**/*.md` file.

## Language word lists

Lists for `weasel-words`, `repetitive-connectors`, `jargon-undefined`, and stoplists live in `src/language/`. PRs are very welcome to:

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

## Getting help

- GitHub Discussions for questions
- GitHub Issues for bugs and proposals
- Open a draft PR early if you want early feedback

Thank you for making `lucid-lint` better.
