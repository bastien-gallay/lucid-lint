# `weasel-words`

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (separate lists) |
| **Source** | [`src/rules/weasel_words.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/weasel_words.rs) |

## What it flags

Vague qualifiers that weaken a statement. A weasel word adds an invisible cognitive load: the reader has to decide whether the claim matters, is true, or measurable. References: Wikipedia style guide (*Avoid weasel words*), Strunk & White, FALC.

## Detection

Word-boundary match against a per-language list. Case-insensitive. One diagnostic per occurrence.

## Parameters

| Key | Type | Default |
|---|---|---|
| `custom_weasels_fr` | `list` | `[]` |
| `custom_weasels_en` | `list` | `[]` |
| `disable_weasels` | `list` | `[]` |

## Default lists (v0.1)

- 🇫🇷 *quelques, certains, parfois, plutôt, assez, globalement, généralement, souvent, en général, la plupart, il semble que, il semblerait que, on pourrait dire que, on dit souvent, beaucoup de, peu de, presque, quasiment, environ, à peu près*
- 🇬🇧 *some, many, often, just, simply, clearly, obviously, seemingly, arguably, basically, essentially, virtually, various, numerous, sort of, kind of, a bit, rather, quite, fairly, relatively, mostly, generally*

## Known false positives

The v0.1 detector is context-blind. Quoted terms, "many X" with concrete X, and meta-discussion of the words themselves still fire. Context-awareness is tracked as **F23** on the [roadmap](../roadmap.md).

## Suppression

Use `<!-- lucid-lint disable-next-line weasel-words -->` when the weasel is intentional (quotation, legitimate subset reference, meta-discussion). See [Suppressing diagnostics](../guide/suppression.md).
