# `excessive-commas`

## What it flags

Sentences whose comma count exceeds a per-profile ceiling. The comma is the most frequent marker of syntactic complexity; rather than disentangle the cause (subordination, apposition, enumeration, parenthetical), the rule uses density as a leading indicator of overload.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/excessive_commas.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/excessive_commas.rs) |

## Detection

Count commas per sentence, report those above `max_commas`.

**Interaction.** When [`long-enumeration`](./long-enumeration.md) fires on the same sentence, this rule is suppressed for that sentence to avoid double-reporting. The shared enumeration detector also discounts Oxford-style enumeration commas (3+ short items) and commas inside `(A, B, C, …)` parenthesised token lists (3+ short comma-separated segments inside balanced parens) — both discounts are language-agnostic.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_commas` | `int` | 4 | 3 | 2 |

## Known false positives

Parenthesised token lists `(A, B, C, …)` are now discounted (F22 first slice). Remaining false positives mostly come from bare comma-separated lists with 3+-word items (`as long as`, `as soon as`) and non-Oxford or "plus"-closed lists; these are tracked as **[F22](../roadmap.md)** in the [roadmap](../roadmap.md) for a v0.3 slice.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`long-enumeration`](./long-enumeration.md)
- [`deep-subordination`](./deep-subordination.md)
