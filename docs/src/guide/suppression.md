# Suppressing diagnostics

`lucid-lint` supports two inline directives for silencing diagnostics in Markdown input. They are intended for the rare cases where a rule fires on intentional prose (a quoted weasel word, a didactic heavy-nominalization example, a legitimate passive). Prefer rewriting the prose first; reach for a directive when the detection is a known false positive or when the author has considered the warning and chosen to keep the text.

## Line form

```markdown
<!-- lucid-lint disable-next-line structure.sentence-too-long -->

A long sentence that is intentional and should not be flagged.
```

- **Syntax.** HTML comment, one rule id per directive. Multiple line directives may precede the same target line.
- **Scope.** The next non-blank line in the source.
- **Optional reason.** `<!-- lucid-lint disable-next-line lexicon.weasel-words reason="quoting the style guide" -->` — surfaced in JSON output; will be *required* via config in a future release (tracked as [F20](../roadmap.md) in the [roadmap](../roadmap.md)).

## Block form (v0.2, F18)

```markdown
<!-- lucid-lint-disable structure.sentence-too-long -->

A long sentence.

Another long sentence in the same scope.

<!-- lucid-lint-enable -->
```

- **Opening.** `<!-- lucid-lint-disable <rule-id> -->` opens a scope for one rule.
- **Closing.** `<!-- lucid-lint-enable -->` closes **every** currently-open scope. Passing a rule id (`<!-- lucid-lint-enable <rule-id> -->`) closes only that rule's scope, which lets overlapping disables for different rules nest cleanly.
- **Scope.** Every line between the two comments (inclusive).
- **Unterminated disable.** Extends to end-of-document — useful for whole-file opt-outs, but prefer the planned `disable-file` directive ([F21](../roadmap.md)) once it lands.
- **One rule per comment.** Multi-rule lists are tracked as F21.

## Common properties

- **Applies to Markdown only.** Plain text and stdin cannot carry HTML comments. Config-based ignores (`[[ignore]]` in `lucid-lint.toml`) covering `.txt` and stdin are tracked as **[F19](../roadmap.md)**.
- **Unknown rule ids are silently ignored.** This keeps directives forward-compatible across lint versions.
- **Suppressed diagnostics cost zero score.** The suppression and [scoring](./scoring.md) models are consistent — silencing a diagnostic removes it from the weighted-sum cost. No hidden double-penalty.

## Deferred

The following extensions are tracked on the [roadmap](../roadmap.md):

| ID | Item |
|---|---|
| [F19](../roadmap.md) | Config-based ignores (`[[ignore]]` in `lucid-lint.toml`) for `.txt` and stdin inputs |
| [F20](../roadmap.md) | Optional-then-required `reason="..."` field, surfaced in reports |
| [F21](../roadmap.md) | File-level directive (`disable-file`) and multi-rule comma lists |

## See also

- [Configuration](./configuration.md) — TOML-level thresholds and profile overrides.
- [Scoring](./scoring.md) — how suppressed diagnostics affect the global and per-category scores.
- Rule-specific notes on when a suppression is idiomatic — see the `## Suppression` section on any rule page under [Rules reference](../rules/index.md).
