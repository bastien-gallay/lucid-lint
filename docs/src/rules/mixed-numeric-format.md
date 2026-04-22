# `structure.mixed-numeric-format`

## What it flags

Sentences that mix digit numerals (`42`, `3.14`, `1,000`, `1 000`) with spelled-out numerals (`two`, `trois`, `twenty`, `cent`) within the same sentence. Presenting numbers inconsistently forces the reader to switch surface forms mid-clause and re-anchor the referent — a known load for readers with dyscalculia and a plain-language anti-pattern.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Condition tags** | `dyscalculia`, `general` |
| **Languages** | EN · FR |
| **Source** | [`src/rules/mixed_numeric_format.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/mixed_numeric_format.rs) |

## Detection

For each sentence emitted by the tokenizer, scan for digit-numeric tokens and for entries in the per-language spelled-numeral list. If at least one of each kind co-occurs, emit a single diagnostic for the sentence citing one representative token of each kind.

Digit tokens accept ASCII digits plus an optional decimal (`.`) or thousands separator (`,`, narrow space `U+0020`) when flanked by digits on both sides. Spelled-out matches are case-insensitive ASCII compares against [`en::SPELLED_NUMERALS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/en.rs) and [`fr::SPELLED_NUMERALS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/fr.rs).

The ambiguous forms `one` (EN) and `un` / `une` (FR) are **excluded** from the spelled-numeral list because they double as indefinite pronouns and articles. This keeps the false-positive rate manageable at the cost of missing genuine mixed-format cases whose only spelled-out numeral is `one`. Metropolitan French and Swiss / Belgian regional forms (`septante`, `huitante`, `octante`, `nonante`) are all included.

Sentences are produced by the shared tokenizer (see [`src/parser/tokenizer.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/parser/tokenizer.rs)), so abbreviations, decimals, and ellipses do not spuriously split sentences. Fenced and indented code blocks are excluded upstream by the Markdown parser.

## Parameters

None. The rule has no configurable threshold — a single co-occurrence of the two surface forms is sufficient.

## Known caveats

- Sentences whose only spelled-out numeral is `one` / `un` / `une` are not flagged, by design (see *Detection*).
- Ordinals (`first`, `premier`, `2nd`, `3e`) are out of scope. `2nd` currently reads as a digit token (`2`) followed by a word (`nd`), which does not match the spelled-numeral list — no false positive.
- Roman numerals (`IV`, `XIV`) are neither digits nor spelled-out numerals for this rule.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`readability.score`](./readability-score.md)
- [Conditions](../guide/conditions.md)
