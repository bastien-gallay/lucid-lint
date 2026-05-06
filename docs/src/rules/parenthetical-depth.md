# `syntax.parenthetical-depth`

> *Experimental in v0.2.x.* Off by default; opt in via
> `--experimental syntax.parenthetical-depth` or
> `[experimental] enabled = ["syntax.parenthetical-depth"]` in
> `lucid-lint.toml`. Flips to `Stable` at the v0.3 cut as part of the
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status)
> cohort flip. See [Conditions](../guide/conditions.md) for the
> `adhd` and `general` condition tags.

## What it flags

A sentence whose maximum balanced-bracket nesting depth across `()` and `[]` reaches the profile threshold. Stacked parentheticals force the reader to track multiple suspended frames at once — a recognised "hard sentence" signal in the plainlanguage.gov and Hemingway editing traditions, and a particular cost for ADHD readers, who carry the working-memory load first.

The rule complements `structure.excessive-commas`, which already discounts flat `(A, B, C)` enumerations at depth 1. This rule fires only at depth 2 or more, so the two rules are mechanically orthogonal: one flat parenthesised list never trips this rule.

## At a glance

| | |
|---|---|
| **Category** | `syntax` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Status** | `experimental` (v0.2.x) → `stable` at v0.3 cut |
| **Condition tags** | `adhd`, `general` (gated; runs only under matching `--conditions`) |
| **Languages** | EN · FR (language-agnostic — bracket families are the same in both) |
| **Source** | [`src/rules/syntax/parenthetical_depth.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/syntax/parenthetical_depth.rs) |

## Detection

For each sentence, the rule walks the post-flattening paragraph text (so fenced code blocks are already excluded by the parser) and tracks a single running depth counter.

### Algorithm

1. Walk the sentence one character at a time.
2. Increment depth on `(` or `[`; decrement on `)` or `]`.
3. A close that would push depth below zero resets depth to zero — the rule fails open on unbalanced markup, mirroring the posture of the `parenthesised_list_comma_count` helper used by `structure.excessive-commas`.
4. Track the maximum depth reached and the position of the opener that achieved it.
5. Emit one diagnostic per sentence when `max_depth ≥` the profile threshold, anchored at the deepest opener.

### Skips (false-positive guards)

- **Code spans / fenced code blocks**: already excluded upstream by the Markdown parser.
- **Unbalanced brackets**: the depth-floor reset prevents stray closes from inflating later depths.

### Deferred (not in MVP)

Em-dash pairs (`— … —`), curly braces (`{}`), and comma-flanked appositives are intentionally out of scope at v0.2.x. Em-dash pair detection is fragile (en/em-dash confusion, hyphen ambiguity) and would smuggle scope back in.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_depth` | `int` | 4 | 3 | 2 |

`max_depth` is the inclusive nesting depth at which the rule fires. A sentence whose deepest bracket frame is one level shallower stays silent.

Tune via `lucid-lint.toml`:

```toml
[rules."syntax.parenthetical-depth"]
max_depth = 3
```

## Examples

### English

**Before** (flagged):

> The migration tool (which now supports rollbacks (see `--reverse`, added in 0.4.2 [tracked in #312])) is opt-in.

What `lucid-lint check --profile public --experimental syntax.parenthetical-depth --conditions adhd` reports:

```text
warning input.md:1:21 Nested parentheticals reach depth 3; readers must hold 3 suspended thoughts to reach the close. Split the sentence or unnest the inner bracket (plainlanguage.gov, Hemingway). [syntax.parenthetical-depth]
```

**After** (your rewrite):

> The migration tool is opt-in. It now supports rollbacks via `--reverse`, added in 0.4.2 (tracked in #312).

The two top-level parentheticals are gone; the remaining one sits flat at depth 1. A reader no longer has to push three suspended thoughts on the stack to reach the close.

### French

**Before** (flagged):

> Le module (qui dépend du noyau (chargé au démarrage [voir le manuel])) est facultatif.

What `lucid-lint check --profile public --experimental syntax.parenthetical-depth --conditions adhd` reports:

```text
warning input.md:1:23 Nested parentheticals reach depth 3; readers must hold 3 suspended thoughts to reach the close. Split the sentence or unnest the inner bracket (plainlanguage.gov, Hemingway). [syntax.parenthetical-depth]
```

**After** (your rewrite):

> Le module est facultatif. Il dépend du noyau, chargé au démarrage. Voir le manuel pour les détails.

Three sentences, no nested brackets. The dependency chain is now linear and the reader recovers each fact in the order it appears.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md) for the inline and block forms. Inline disable also works on this rule:

```markdown
<!-- lucid-lint disable-next-line syntax.parenthetical-depth -->
The migration tool (which now supports rollbacks (see `--reverse`, added in 0.4.2 [tracked in #312])) is opt-in.
```

## See also

- [Conditions](../guide/conditions.md) — the `adhd` and `general` tags that gate this rule.
- [`structure.excessive-commas`](./excessive-commas.md) — sibling rule on flat parenthesised enumerations. Atomic split: `excessive-commas` discounts depth-1 `(A, B, C)` lists; this rule fires only at depth ≥ 2.
- [`syntax.dense-punctuation-burst`](./dense-punctuation-burst.md) — sibling rule on local punctuation density. Both signal hard-to-parse sentences from different angles.
- [F-experimental-rule-status — experimental rule status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrate that lets this rule ship in v0.2.x without affecting default scores.

## References

- [plainlanguage.gov — Write short sentences](https://www.plainlanguage.gov/guidelines/concise/write-short-sentences/). Plain-language guidance treats stacked qualifiers and nested parentheticals as the canonical "long sentence" symptom.
- Hemingway editing tradition — surfaces "hard to read" sentences when they layer multiple suspended ideas; nested parentheticals are the cleanest mechanical reading of that signal.

See [References](../references.md) for the full bibliography.
