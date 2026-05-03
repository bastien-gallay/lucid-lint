# `lexicon.weasel-words`

## What it flags

Vague qualifiers that weaken a statement. A weasel word adds an invisible cognitive load: the reader has to decide whether the claim matters, is true, or measurable. References: Wikipedia style guide (*Avoid weasel words*), Strunk & White, FALC.

## At a glance

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (separate lists) |
| **Source** | [`src/rules/weasel_words.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/weasel_words.rs) |

## Detection

Word-boundary match against a per-language list. Case-insensitive. One diagnostic per occurrence.

<aside class="since-version" aria-label="New in v0.2">

<span class="since-version__tag">Since v0.2</span> — Two context-aware
exclusions land as the first slice of **[F23](../roadmap.md)**.

</aside>

- **Inline code spans.** A hit inside `` `…` `` is skipped. Wrap a weasel term in backticks when you are discussing the word itself.
- **Directional pairings.** `rather than` (EN) and `plutôt que` (FR) are conjunctions meaning "instead of" — not hedges — and are skipped.

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

Two patterns still fire in v0.2: straight-quoted terms (`"many X"` without backticks) and `"many X"` where X is a concrete noun. Both are queued under **[F23](../roadmap.md)** on the [roadmap](../roadmap.md). Wrap the quoted term in backticks, or use an inline disable comment, to opt out.

## Suppression

Use `<!-- lucid-lint disable-next-line lexicon.weasel-words -->` when the weasel is intentional (quotation, legitimate subset reference, meta-discussion). See [Suppressing diagnostics](../guide/suppression.md).

## References

- [Strunk & White (1999)](../references.md#strunk-white-1999)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

See [References](../references.md) for the full bibliography.
