# `lexicon.low-lexical-diversity`

## What it flags

Passages with excessive repetition of content words. A monotonous text loses reader attention and often signals unstructured thinking. The rule is *not* an anti-jargon detector: technical terms (`API`, `request`, `cache`) are expected to repeat — the signal targets non-technical content words.

**Reference.** Type-Token Ratio (TTR), a classical corpus-linguistics metric (Herdan, 1960).

## At a glance

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `info` |
| **Default weight** | `1` |
| **Languages** | EN · FR (separate stoplists) |
| **Source** | [`src/rules/low_lexical_diversity.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/low_lexical_diversity.rs) |

## Detection

Sliding window of `window_size` words. Within the window, compute `unique_words / total_words` over non-stopword, non-code-block tokens. Fire when the ratio falls below `min_ratio`.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `window_size` | `int` | 100 | 100 | 80 |
| `min_ratio` | `float` | 0.40 | 0.50 | 0.55 |
| `use_stoplist` | `bool` | `true` | `true` | `true` |

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## References

- [Herdan (1960)](../references.md#herdan-1960)
- [McCarthy & Jarvis (2010)](../references.md#mccarthy-jarvis-2010)
- [Graesser et al. (2004)](../references.md#graesser-2004)

See [References](../references.md) for the full bibliography.
