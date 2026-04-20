# `excessive-commas`

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/excessive_commas.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/excessive_commas.rs) |

## What it flags

Sentences whose comma count exceeds a per-profile ceiling. The comma is the most frequent marker of syntactic complexity; rather than disentangle the cause (subordination, apposition, enumeration, parenthetical), the rule uses density as a leading indicator of overload.

## Detection

Count commas per sentence, report those above `max_commas`.

**Interaction.** When [`long-enumeration`](./long-enumeration.md) fires on the same sentence, this rule is suppressed for that sentence to avoid double-reporting. The shared enumeration detector also discounts Oxford-style enumeration commas (3+ short items).

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_commas` | `int` | 4 | 3 | 2 |

## Known false positives

Technical prose enumerating short items still fires; context-aware relaxations (parentheticals, post-colon short lists, non-Oxford enumerations) are tracked as **[F22](../roadmap.md)** in the [roadmap](../roadmap.md).

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`long-enumeration`](./long-enumeration.md)
- [`deep-subordination`](./deep-subordination.md)
