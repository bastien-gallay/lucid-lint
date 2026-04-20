# `paragraph-too-long`

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `2` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/paragraph_too_long.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/paragraph_too_long.rs) |

## What it flags

Paragraphs that overrun either a sentence-count or a word-count threshold. A paragraph is a visual reprise unit: long paragraphs dilute the reprise point for readers who interrupt often. Both metrics are checked so that a short-but-dense paragraph (one 80-word sentence) is still caught — [`sentence-too-long`](./sentence-too-long.md) covers the complementary case.

## Detection

Split on blank lines (Markdown paragraph convention). Count sentences and words per paragraph. Flag paragraphs exceeding *either* threshold.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_sentences` | `int` | 7 | 5 | 3 |
| `max_words` | `int` | 150 | 100 | 60 |

## Examples

A paragraph of eight medium sentences under the `public` profile will fire on `max_sentences`. A paragraph containing a single 120-word sentence will fire on `max_words` (and also on `sentence-too-long`).

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`sentence-too-long`](./sentence-too-long.md)
- [`consecutive-long-sentences`](./consecutive-long-sentences.md)
