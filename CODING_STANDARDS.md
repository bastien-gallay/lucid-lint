# Coding Standards

This document describes the design principles and code conventions applied in `lucid-lint`.

It is meant to guide contributions and to keep the codebase coherent as it grows.

## Design principles

### 1. Make impossible states impossible

<!-- lucid-lint disable-next-line weasel-words -->

Rust's type system is a tool for correctness, not just performance.

**Prefer**:

- Newtype wrappers over raw primitives (`Words(u32)`, not `u32` for a word count).
- Enums with associated data over flags + optional fields.
- `NonZeroU32` when a value must be positive.
- Builder patterns that require all mandatory fields before `.build()`.

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

### 2. Prefer functional style

Transformations over mutations, when it doesn't hurt clarity.

**Prefer**:

- `Iterator` chains (`map`, `filter`, `fold`) over accumulating in a mut `Vec`.
- Pure functions with clear inputs and outputs.
- `Result<T, E>` and `Option<T>` instead of sentinel values.
- Pattern matching over nested `if let`.

**Example**

```rust
// Prefer
let long_sentences: Vec<_> = sentences
    .iter()
    .filter(|s| s.word_count() > threshold.get())
    .collect();

// Avoid, when the functional form is just as clear
let mut long_sentences = Vec::new();
for s in &sentences {
    if s.word_count() > threshold.get() {
        long_sentences.push(s);
    }
}
```

Don't go overboard. If a `for` loop reads better than a chain, use the loop.

### 3. Atomic rules

Each rule checks one signal.

If a rule needs three independent parameters and three different detection strategies, it is actually three rules. See the v0.1 history of rule 3 (`excessive-commas`, `long-enumeration`, `deep-subordination`) for a concrete decomposition.

### 4. Deterministic by default

The core `lucid-lint` binary must produce the same output for the same input on any machine.

- No network calls in core rules.
- No LLM calls in core rules.
- No time-dependent or environment-dependent behavior.

LLM-based or other non-deterministic rules live in optional plugins.

### 5. YAGNI

Don't add an abstraction for a second use case that doesn't exist yet. Add it when the second use case appears.

Corollary: avoid traits with a single implementation until there is a clear need for polymorphism. A concrete `MarkdownParser` is fine until we actually add `AsciiDocParser`.

**Counter-example**: if a field on a struct would duplicate information already derivable from another field (`category` from `rule_id`, for instance), don't store it. Derive it with a helper function.

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
