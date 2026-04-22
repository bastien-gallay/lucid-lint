# `rhythm.consecutive-long-sentences`

## What it flags

Streaks of long sentences within the same paragraph. An isolated long sentence is manageable; several in a row fatigue attention even when each individual sentence is under the [`structure.sentence-too-long`](./sentence-too-long.md) ceiling. This rule catches the *rhythm*.

## At a glance

| | |
|---|---|
| **Category** | `rhythm` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/consecutive_long_sentences.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/consecutive_long_sentences.rs) |

## Detection

Walk sentences sequentially inside each paragraph. Maintain a running count of consecutive sentences above `word_threshold`. Fire once per streak that reaches `max_consecutive`.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `word_threshold` | `int` | 20 | 15 | 10 |
| `max_consecutive` | `int` | 3 | 2 | 2 |

**Invariant.** `word_threshold` must stay below `sentence-too-long.max_words` for the same profile. Otherwise both rules fire on the same sentences.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`structure.sentence-too-long`](./sentence-too-long.md)
