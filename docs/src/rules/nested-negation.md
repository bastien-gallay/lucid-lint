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

`lucid-lint` reports; the rewrite is always yours.

### English

Three negations → three affirmatives, colour-matched across the rewrite. The `not` simply drops — the simplification shows.

**Before** (flagged):

> We do <span class="lucid-idea" data-idea="1">not say</span> <span class="lucid-idea" data-idea="2">nothing</span> is <span class="lucid-idea" data-idea="3">never possible</span>.

Three negations (`not`, `nothing`, `never`).

What `lucid-lint check --profile public` reports:

```text
warning input.md:1:1 Sentence stacks 3 negations (maximum 2). Rewrite as a positive statement or split the negations across separate sentences. [syntax.nested-negation]
```

**After** (your rewrite):

> We <span class="lucid-idea" data-idea="1">say</span> <span class="lucid-idea" data-idea="2">everything</span> <span class="lucid-idea" data-idea="3">is possible</span>.

### French

**Passes under `public`:**

> Nous ne sommes pas prêts.

Bipartite `ne ... pas` counts as one negation.

**Before** (the pedagogical target — see note below):

> Nous <span class="lucid-idea" data-idea="1">ne disons pas</span> que <span class="lucid-idea" data-idea="2">rien</span> <span class="lucid-idea" data-idea="3">n'est jamais possible</span>.

Three negations (`ne…pas`, `rien`, `jamais`).

> **Note — not yet flagged.** The current FR detector counts only `ne` / `n'` clitics (plus the standalones `sans` and `non`), so this sentence registers as 2 negations and stays below the `public` threshold. Extending FR detection to second-position negators (`rien`, `jamais`, `plus`, `personne`, `aucun`) is tracked as **[F87](../roadmap.md)** in the [roadmap](../roadmap.md).

**After** (your rewrite):

> Nous <span class="lucid-idea" data-idea="1">disons</span> que <span class="lucid-idea" data-idea="2">tout</span> <span class="lucid-idea" data-idea="3">est possible</span>.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`syntax.passive-voice`](./passive-voice.md)
- [`structure.deep-subordination`](./deep-subordination.md)
- [Conditions](../guide/conditions.md)
