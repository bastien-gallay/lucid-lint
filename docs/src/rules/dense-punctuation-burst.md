# `dense-punctuation-burst`

## What it flags

*Local* bursts of punctuation: a sliding window of grapheme clusters that contains too many qualifying marks (`,`, `;`, `:`, `—`, `–`). Tight clusters of marks signal layered subordination, parenthetical interjections, or list-within-list constructions that are hard to parse for readers with cognitive or attentional difficulties (IFLA easy-to-read guidelines).

Distinct from [`excessive-commas`](./excessive-commas.md), which counts commas across an entire sentence. A sentence with 8 commas spread evenly across 200 characters does not trigger here, while a sentence with 3 commas inside a 30-character span does.

## At a glance

| | |
|---|---|
| **Category** | `syntax` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Condition tags** | `general` |
| **Languages** | EN · FR (script-agnostic) |
| **Source** | [`src/rules/dense_punctuation_burst.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/dense_punctuation_burst.rs) |

## Detection

Per source line, walk the grapheme stream once and collect the column of every qualifying mark. When a window of `window_graphemes` graphemes holds `min_marks` or more marks, emit a burst spanning the first to the last mark in the window, then advance past that last mark so overlapping windows do not double-fire on the same cluster.

Code blocks (fenced and indented) are excluded upstream by the Markdown parser. Sentence terminators (`.`, `!`, `?`) and brackets do not count toward the burst.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_marks` | `int` | 4 | 3 | 3 |
| `window_graphemes` | `int` | 30 | 30 | 40 |

`dev-doc` tolerates a 3-mark cluster (often unavoidable in technical lists adjacent to prose). FALC keeps the same density floor as `public` but widens the window to catch slightly looser bursts.

## Known caveats

- The rule operates per source line. A burst that wraps across a hard line break in the source is not detected; in practice this is rare because dense punctuation is also dense in source bytes.
- Em dash (`—`, U+2014) and en dash (`–`, U+2013) qualify; the ASCII double-hyphen surrogate (`--`) does not, on the assumption that authors who care about readability use the proper Unicode forms.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`excessive-commas`](./excessive-commas.md)
- [`sentence-too-long`](./sentence-too-long.md)
- [Conditions](../guide/conditions.md)
