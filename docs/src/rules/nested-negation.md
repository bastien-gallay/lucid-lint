# `syntax.nested-negation`

## What it flags

Sentences that stack multiple negations. Two or more negations in the same sentence force the reader to mentally toggle truth values — a known burden for readers with aphasia and attention-fragile readers (ADHD), and a load multiplier for everyone reading under cognitive pressure. Plain-language guidelines (FALC, CDC Clear Communication Index, plainlanguage.gov) recommend rewriting double negatives as positives.

## At a glance

| | |
|---|---|
| **Category** | `syntax` |
| **Default severity** | `warning` |
| **Default weight** | `2` |
| **Condition tags** | `aphasia`, `adhd`, `general` |
| **Languages** | EN · FR (language-specific counting) |
| **Source** | [`src/rules/nested_negation.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/nested_negation.rs) |

## Detection

Count the negations per sentence; report sentences whose count exceeds `max_negations`.

- **English** — sum of word-boundary matches against the language's negation list (`not`, `no`, `never`, `none`, `nothing`, `nobody`, `nowhere`, `neither`, `nor`, `cannot`, `without`) plus occurrences of the contracted `n't` suffix (`don't`, `won't`, `isn't`, `doesn't`, …).
- **French** — bipartite negation: each `ne` / `n'` clitic counts as one negation, plus standalone negators (`sans`, `non`). Counting the second-position particle (`pas`, `jamais`, `plus`, …) directly would trigger false positives because many of those forms are ambiguous outside the `ne ... X` construction.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_negations` | `int` | 3 | 2 | 1 |

## Examples

**EN — triggers under `public`:**

> We do not say nothing is never possible.

Three negations (`not`, `nothing`, `never`). Rewrite as: *We say everything is possible.*

**FR — passes under `public`:**

> Nous ne sommes pas prêts.

Bipartite `ne ... pas` counts as one negation.

**FR — triggers under `public`:**

> Il ne dit rien, elle ne fait rien et nous ne savons pas.

Three independent `ne` clitics → three negations.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`syntax.passive-voice`](./passive-voice.md)
- [`structure.deep-subordination`](./deep-subordination.md)
- [Conditions](../guide/conditions.md)
