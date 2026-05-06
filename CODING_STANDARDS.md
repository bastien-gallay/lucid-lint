# Coding Standards

This file is the working agreement for code in this repo. It is meant to be re-read on a slow day, not skimmed once. Four pillars, in the order you usually apply them:

1. **Tidy First** — separate behaviour changes from clean-ups.
2. **CUPID & YAGNI** — properties to aim for in design and refactoring.
3. **TDD (Red → Green → Refactor → Reflect)** — the loop that keeps the above honest.
4. **Clean Code** — local taste rules that survive automation.

Repo-specific rules in `AGENTS.md` take precedence when they collide.

---

## 1. Tidy First (Kent Beck)

> *Make the change easy, then make the easy change.*

Behaviour changes and structural changes are **two different commits**.

- **Tidying** — renames, extractions, dead-code removal, reformatting, splitting a long function, adding a missing test that pins existing behaviour. Never alters observable output.
- **Behaviour change** — the actual feature, fix, or contract change.

Rules of thumb:

- If the diff to add a feature feels too big, stop. Tidy the surrounding code first (in its own commit), then come back. The feature commit shrinks.
- Tests that pin existing behaviour are **Must-have**, not Could-have. Land them *before* the behaviour change. The point of pinning is to make the behaviour change reviewable as a small, intentional diff.
- If a tidy ends up changing observable behaviour, it wasn't a tidy. Revert and split.

Acceptable commit shapes:

```text
✅  refactor(rules): extract check_bounds helper        (tidy)
    feat(rules): add structure.sentence-too-long        (behaviour)

❌  feat(rules): add sentence-too-long + cleanup
```

---

## 2. CUPID & YAGNI

Five properties to optimise for, in roughly this order:

| Property            | One-liner                                               | Smell when violated                               |
| ------------------- | ------------------------------------------------------- | ------------------------------------------------- |
| **Composable**      | Plays well with others; small surface, no surprises.    | "I have to mock half the world to test this."     |
| **Unix philosophy** | Does one thing well.                                    | Module/trait with `and` in its purpose statement. |
| **Predictable**     | Behaves as expected; no hidden state, no spooky action. | "Works on my machine" / order-dependent tests.    |
| **Idiomatic**       | Reads like the language and the codebase.               | Reviewer says "this is clever" with a sigh.       |
| **Domain-based**    | Names match the user's vocabulary.                      | Generic `Manager`/`Helper`/`Util` names.          |

### C — Composable

Pieces plug together without special knowledge of each other.

- `Engine` is decoupled from `cli.rs`; the library can be embedded anywhere.
- Rules go through a single `Rule` trait (`Box<dyn Rule>`); add, remove, swap without touching the engine.
- Parser output (`Document`) is the contract between parsing and rules — rules consume it read-only.

**Watch for**: rules that reach past `Document` into the raw input or re-parse themselves. If several rules want the same derived view (lowercased sentence, token stream), hoist it into `Document` once instead of recomputing per rule.

### U — Unix philosophy

Do one thing well.

- One binary, one job: lint prose for cognitive accessibility.
- Reads files, stdin (`-`), writes stdout, non-zero exit on findings. Pipe-friendly by design.
- One file, one rule, one signal. If a rule grows a second detection strategy, split it (see the v0.1 decomposition of rule 3 into `structure.excessive-commas`, `structure.long-enumeration`, `structure.deep-subordination`).

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

### Heuristic-rule off-ramp

Heuristic rules in `src/rules/` (pattern, counter, window — the deterministic-core kind) accumulate exception cases over time. A rule that keeps growing exceptions for edge cases has hit its ceiling: each new exception adds complexity for diminishing precision gains, and the next class of mistake usually needs structural information the heuristic does not have.

When that happens, do not add another exception. Three moves instead:

1. **Freeze the rule.** Document the ceiling in the rule's rustdoc — name the class of cases the heuristic cannot reach without becoming a parser.
2. **Queue the precise version for the NLP plugin.** POS, dependency-tree, and anaphora-grade precision belongs in `lucid-lint-nlp` (see [F-nlp-plugin](ROADMAP.md#f-nlp-plugin)). The deterministic-core stance (principle 3 in [AGENTS.md](AGENTS.md)) keeps the precise replacement out of `src/rules/`.
3. **Ship at-risk new rules `Status::Experimental` from day one.** When proposing a rule whose exception tail is hard to bound up front, opt into the lifecycle gate (see [F-experimental-rule-status](ROADMAP.md#f-experimental-rule-status)) so users opt in and the freeze decision is cheap later.

The signal is qualitative — there is no fixed exception count. If two reviewers in a row reach for "let's also exempt X," that is the cue.

---

## 3. TDD with a fourth step — Reflect

The standard Red → Green → Refactor loop, with a deliberate **Reflect** beat at the end of each cycle. The reflect step is what keeps the loop from grinding out lots of small green tests that don't add up to a coherent design.

```text
   ┌──────────┐
   │   RED    │   Write the smallest failing test that names the
   │          │   behaviour you want. Run it. Confirm it fails for
   │          │   the right reason (not a typo, not an import).
   └────┬─────┘
        │
        ▼
   ┌──────────┐
   │  GREEN   │   Write the least code that makes the test pass.
   │          │   Ugly is fine here. Don't generalise yet.
   └────┬─────┘
        │
        ▼
   ┌──────────┐
   │ REFACTOR │   With the test green, clean up — names, duplication,
   │          │   shape. Tests stay green between every keystroke.
   │          │   This is a TIDY (see §1); commit it separately.
   └────┬─────┘
        │
        ▼
   ┌──────────┐
   │ REFLECT  │   Pause. Ask:
   │          │     • What did this cycle teach me?
   │          │     • What surprised me (red took longer? green was
   │          │       trivial? refactor revealed a missing concept)?
   │          │     • Is the *next* test on my list still the right
   │          │       one, or did this cycle change the plan?
   │          │     • Is there a test I should retire because it now
   │          │       overlaps with a stronger one?
   │          │     • Did I learn a domain rule worth pinning in
   │          │       another test, separate from the one I just
   │          │       wrote?
   │          │   Update the test list. Then loop.
   └────┬─────┘
        │
        ▼
       (next test)
```

Reflect rules:

- **Reflect is short.** A minute, sometimes thirty seconds. If it becomes a meeting, do it asynchronously between cycles.
- **Reflect updates the plan, not the code.** If reflection reveals code that should change, that's the *next* RED test, not an edit you smuggle into the current cycle.
- **Reflect after Green-but-no-Refactor cycles too.** "There was nothing to clean" is itself a signal — either the design is good or you're not looking hard enough.
- **Always surface findings to the user with a recommendation.** Every reflection that produces a finding (a new test worth pinning, a surprise that suggests a missing concept, a smell you noticed) gets a one-line decision prompt: *"apply now / add to today / add to the roadmap / forget it"*. Do not silently carry findings forward and do not silently apply them.

  Recommend the best move per the principles, and say *why* in one short clause. Heuristics:
  - **Apply now** — the finding closes a still-open hole from the cycle just finished, the fix is small, and skipping it would leave the work half-done. (e.g. forward contract test landed → reverse test is ~30 LOC → apply now closes the lesson.)
  - **Add to today** — same-session work, but it would derail the current task; better as the next discrete cycle.
  - **Add to the roadmap** — useful but not on the critical path; capturing it here so it's not lost.
  - **Forget it** — speculative, low-leverage, or you're not sure it's real. Recording every passing thought is its own debt.

  Default leans toward *apply now* when the finding is small and directly tied to the cycle that surfaced it (Tidy First: keep the diff coherent). Lean toward *roadmap* when the finding is larger than the cycle it interrupted (CUPID-Composable: don't bundle unrelated work).

**Testing in `lucid-lint`**:

- **Unit tests** live in the same file, under `#[cfg(test)] mod tests`. Every rule needs at least 3: positive case, negative case, edge case.
- **Integration tests** live in `tests/`.
- **Snapshot tests** use `insta` and live alongside their target. Every rule has at least 1 snapshot test. Parser logic has snapshot tests for representative Markdown features.
- **Fixtures** live in `tests/corpus/{en,fr}/`.

---

## 4. Clean Code

Local taste rules. None of these are absolute; they exist to be broken *on purpose*, not by accident.

### Names

- A name should let a reader skip the implementation. If they have to read the body to understand the name, rename it.
- Domain words beat generic ones.
- Boolean names read as predicates: `is_stale`, `has_today`, `should_carry`. Not `flag`, not `status` (unless it's an enum).
- Types: `PascalCase`, Functions/vars: `snake_case`, Constants: `SCREAMING_SNAKE_CASE`, Rule IDs: `kebab-case`, Files: `snake_case.rs`.

### Functions

- One purpose per function. If you'd need "and" to describe it, split.
- Short by default — long when the alternative is a tangle of helpers no one will read in order.
- Arguments: 0–3 is fine; 4+ wants a struct or builder pattern.
- No flag arguments that change *what* the function does. `do(x, dry_run: bool)` is fine (toggles a side-effect); `do(x, mode: Mode)` is usually two functions.

### Comments & Documentation

- Default to **no inline comments**. Code says *what*; commit messages and PR descriptions say *why*.
- Write an inline comment only when the *why* is non-obvious from the code: a hidden constraint, a surprising invariant, a workaround for a specific bug.
- Every public item has a doc comment (starting with a one-line summary, followed by details and examples, using intra-doc links).

```rust
/// Configuration for a single lint rule.
///
/// Rules are configured via [`RuleConfig`], which is typically loaded
/// from a [`Profile`] preset and optionally overridden by the user's
/// `lucid-lint.toml` file.
pub struct RuleConfig {
    // ...
}
```

### Errors

- Validate at boundaries (CLI args, file I/O). Trust internal callers.
- Fail loudly and early. A silent fallback is a future bug report.
- Exit codes matter for the CLI: `0` ok, `1` operational failure, `2` user error.
- Use `thiserror` for library error types, and `anyhow` in binary code for terminal error handling.

---

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

---

## Toolchain & Layout

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

### Toolchain policy

- `rust-toolchain.toml` pins `channel = "stable"`. Do NOT pin a specific Rust minor version here.
- `Cargo.toml` declares `rust-version` as the MSRV. The CI `msrv` job MUST match this exact value. Bumping MSRV requires a dedicated commit updating both places together.
- Jobs that invoke `cargo` in CI MUST install a toolchain explicitly via `dtolnay/rust-toolchain@master`.

### Clippy

We run `clippy` with `-W clippy::pedantic -W clippy::nursery`, then selectively allow noisy lints in `clippy.toml` or at the item level.

Goals:

- Prefer compiler feedback over stylistic nitpicks.
- Not every `pedantic` warning needs to be fixed, but every one should be considered.

---

## Review mindset

When reviewing a PR, ask:

1. Does it meet the design principles above?
2. Are there impossible states the types now allow?
3. Is the logic tested?
4. Is the documentation up to date?
5. Would a future maintainer understand why, not just what?

Kindness over pedantry. The goal is a better codebase, not a perfect one.
