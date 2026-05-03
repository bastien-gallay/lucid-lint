# `structure.excessive-commas`

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

**Interaction.** When [`structure.long-enumeration`](./long-enumeration.md) fires on the same sentence, this rule is suppressed for that sentence to avoid double-reporting. The shared enumeration detector discounts Oxford-style enumeration commas (3+ short items, plus a relaxed rhythmic pass for 1–4-word items, plus runs closed by `plus` as well as `and` / `or` — see "Known false positives" below) and commas inside `(A, B, C, …)` parenthesised token lists (3+ short comma-separated segments inside balanced parens) — all language-agnostic.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_commas` | `int` | 4 | 3 | 2 |

## Known false positives

<aside class="since-version" aria-label="New in v0.2.x">

<span class="since-version__tag">Since v0.2.x</span> — Parenthesised token
lists `(A, B, C, …)` no longer count toward the comma total
([F22](../roadmap.md), first slice).

</aside>

<aside class="since-version" aria-label="New in v0.3">

<span class="since-version__tag">Since v0.3</span> — Oxford runs of
1–4-word items that share a regular rhythm are now discounted too — for
example `category, severity, default weight, parameters per profile,
EN/FR examples, and suppression`. The detector requires at least 5
items, a word-count spread ≤ 2, and walks no further back than the
nearest clause boundary ([F22](../roadmap.md), second slice).

</aside>

<aside class="since-version" aria-label="New in v0.3">

<span class="since-version__tag">Since v0.3</span> — `plus` is now
recognised as an Oxford-style terminator alongside `and` / `or` /
`et` / `ou`, so a list like `profile, format, min-score, plus
working-directory and args` is detected and discounted. Same
connector word in both EN and FR ([F22](../roadmap.md), third slice).

</aside>

Remaining false positives mostly come from bare lists with no terminal connector (e.g. `Rules touched: A, B, C`) and Oxford runs interrupted by an interleaved parenthetical; these are tracked as **[F22](../roadmap.md)** in the [roadmap](../roadmap.md) for further v0.3 sub-slices.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`structure.long-enumeration`](./long-enumeration.md)
- [`structure.deep-subordination`](./deep-subordination.md)

## References

- [Gibson (1998)](../references.md#gibson-1998)

See [References](../references.md) for the full bibliography.
