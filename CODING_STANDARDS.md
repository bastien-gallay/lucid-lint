# Coding Standards

This document describes the design principles and code conventions applied in `lucid-lint`.

It is meant to guide contributions and to keep the codebase coherent as it grows.

## Design principles

The house framework is **CUPID** (Dan North): write code that is joyful to live with. Five properties, one anti-speculation rule.

### C — Composable

Pieces plug together without special knowledge of each other.

- `Engine` is decoupled from `cli.rs`; the library can be embedded anywhere.
- Rules go through a single `Rule` trait (`Box<dyn Rule>`); add, remove, swap without touching the engine.
- Parser output (`Document`) is the contract between parsing and rules — rules consume it read-only.

**Watch for**: rules that reach past `Document` into the raw input or re-parse themselves. If several rules want the same derived view (lowercased sentence, token stream), hoist it into `Document` once rather than recomputing per rule.

### U — Unix philosophy

Do one thing well.

- One binary, one job: lint prose for cognitive accessibility.
- Reads files, stdin (`-`), writes stdout, non-zero exit on findings. Pipe-friendly by design.
- One file, one rule, one signal. If a rule grows a second detection strategy, split it (see the v0.1 decomposition of rule 3 into `excessive-commas`, `long-enumeration`, `deep-subordination`).

**Watch for**: rules that "while we're here" emit an unrelated diagnostic. File the second signal as a new rule.

### P — Predictable

Same input, same output, on any machine.

- No network, no LLM, no time or environment dependency in the core.
- Deterministic iteration: if order matters, sort explicitly — don't rely on `HashMap`/`HashSet` iteration.
- Language-sensitive rules return empty for `Language::Unknown` unless they have a well-defined language-agnostic fallback.
- Snapshot tests (`insta`) make any change in output intentional and reviewable.

Non-deterministic rules (LLM-backed, network-backed) live in optional plugin crates, never the core.

### I — Idiomatic

Feels like modern Rust to a Rust reader.

- Newtype wrappers and enums to make impossible states impossible (`NonZeroU32` for positive thresholds, `Severity`/`Category`/`Language` as enums, not strings).
- Iterator chains (`map`, `filter`, `fold`) over accumulating in a mut `Vec` — when it reads cleaner. Use a `for` loop when it doesn't.
- `Result<T, E>` and `Option<T>` over sentinel values. Pattern matching over nested `if let`.
- `thiserror` for library errors, `anyhow` for binary-level terminal errors. No `unwrap`/`expect` in library code unless provably unreachable and commented.

**Example**

```rust
// Bad: nothing prevents a negative or absurd threshold at runtime.
pub struct RuleConfig {
    pub max_words: i32,
    pub profile: String,
}

// Good: parsing happens once, at construction.
pub struct RuleConfig {
    pub max_words: NonZeroU32,
    pub profile: Profile, // enum
}
```

### D — Domain-based

The code speaks the vocabulary of linguistics and accessibility.

- Types map to the domain: `Sentence`, `Paragraph`, `Diagnostic`, `Severity`, `Profile`, `Language`.
- Rule filenames name the signal, not the data structure: `sentence_too_long.rs`, `deep_subordination.rs`, `low_lexical_diversity.rs`.
- Profile names (`DevDoc`, `Public`, `Falc`) are grounded in real user personas, not technical tiers.

**Watch for**: helpers named after their implementation (`ratio_at_anchor_min`) leaking into public API. Rename toward the domain when promoting.

### YAGNI (anti-speculation rule)

CUPID describes what good code *is*; YAGNI protects against building what you don't need yet.

- No abstraction for a second use case that doesn't exist. A concrete `MarkdownParser` is fine until `AsciiDocParser` actually lands.
- No trait with a single implementation.
- No field that duplicates information derivable from another (`category` from `rule_id`; don't store it, derive it with a helper).
- No proc macros to remove scaffolding when the scaffolding *is* the contract. Seventeen small, identical rule skeletons beat one magical macro.

When a refactor toward CUPID would require speculative work, stop and wait for the second use case.

## Code conventions

### Module layout

```
src/
├── lib.rs                  # Library crate root
├── main.rs                 # Binary crate entry point
├── cli.rs                  # CLI argument parsing
├── config/                 # Configuration loading and profiles
├── parser/                 # Input parsing (Markdown, plain text)
├── language/               # Language detection and language-specific data
├── rules/                  # Individual rule implementations
│   ├── mod.rs              # Rule registry and trait
│   └── <rule_name>.rs      # One file per rule
└── output/                 # Output formatters (TTY, JSON, SARIF)
```

### Naming

- Types: `PascalCase`
- Functions and variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Rule IDs: `kebab-case` (`sentence-too-long`)
- Files: `snake_case.rs`

### Error handling

- Use `thiserror` for library error types.
- Use `anyhow` in binary code for terminal error handling.
- Never `unwrap()` or `expect()` in library code unless a panic is genuinely unreachable and commented.

### Documentation

- Every public item has a doc comment.
- Doc comments start with a one-line summary, followed by details and examples.
- Link to related items with intra-doc links.

```rust
/// Configuration for a single lint rule.
///
/// Rules are configured via [`RuleConfig`], which is typically loaded
/// from a [`Profile`] preset and optionally overridden by the user's
/// `lucid-lint.toml` file.
///
/// # Examples
///
/// ```
/// use lucid_lint::config::{RuleConfig, Profile};
/// let config = RuleConfig::for_profile(Profile::Public);
/// ```
pub struct RuleConfig {
    // ...
}
```

### Testing

- **Unit tests** live in the same file, under `#[cfg(test)] mod tests`.
- **Integration tests** live in `tests/`.
- **Snapshot tests** use `insta` and live alongside their target.
- **Fixtures** live in `tests/corpus/{en,fr}/`.

Aim for:

- Every rule has at least 3 unit tests: positive case, negative case, edge case.
- Every rule has at least 1 snapshot test.
- Parser logic has snapshot tests for representative Markdown features.

### Clippy

We run `clippy` with `-W clippy::pedantic -W clippy::nursery`, then selectively allow noisy lints in `clippy.toml` or at the item level.

Goals:

- Prefer compiler feedback over stylistic nitpicks.
- Not every `pedantic` warning needs to be fixed, but every one should be considered.

## Commit style

Commits follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<optional scope>): <short summary>

<optional body>

<optional footer>
```

Types:

- `feat`: new rule, new feature
- `fix`: bug fix
- `docs`: documentation only
- `refactor`: code change that neither fixes a bug nor adds a feature
- `test`: adding or fixing tests
- `chore`: build tooling, dependencies
- `perf`: performance improvement

Breaking changes include `BREAKING CHANGE:` in the footer or `!` after the type.

## Toolchain policy

- `rust-toolchain.toml` pins `channel = "stable"`. Do NOT pin a specific
  Rust minor version here — a frozen pin silently ages and third-party
  actions (e.g. `rustsec/audit-check`, `peaceiris/actions-mdbook` when
  running `cargo doc`) will fail as their ecosystem moves forward.
- `Cargo.toml` declares `rust-version` as the MSRV. The CI `msrv` job
  MUST match this exact value. Bumping MSRV requires a dedicated commit
  updating both places together.
- Jobs that invoke `cargo` in CI MUST install a toolchain explicitly via
  `dtolnay/rust-toolchain@master`. Don't rely on the runner's
  preinstalled Rust — it drifts without warning.

## Review mindset

When reviewing a PR, ask:

1. Does it meet the design principles above?
2. Are there impossible states the types now allow?
3. Is the logic tested?
4. Is the documentation up to date?
5. Would a future maintainer understand why, not just what?

Kindness over pedantry. The goal is a better codebase, not a perfect one.
