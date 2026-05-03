# `lexicon.excessive-nominalization`

## What it flags

Sentences densely packed with nominalizations — verbs turned into abstract nouns. Two problems compound: nominalized text is more abstract (costlier to process) and hides the agent ("who does what" is obscured). FALC and the US Plain Writing Act both recommend strong verbs over nominalizations.

## At a glance

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (overlapping suffix lists) |
| **Source** | [`src/rules/excessive_nominalization.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/excessive_nominalization.rs) |

## Detection

Walk the sentence. Flag words whose suffix matches the language's nominalization list. Fire when the count per sentence crosses `max_per_sentence`.

- 🇫🇷 Suffixes: `-tion`, `-sion`, `-ment`, `-ance`, `-ence`, `-age`, `-ité`, `-isme`, `-ure`
- 🇬🇧 Suffixes: `-tion`, `-sion`, `-ment`, `-ance`, `-ence`, `-ity`, `-ism`, `-ness`, `-al`

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_per_sentence` | `int` | 4 | 3 | 2 |
| `suffixes` | `list` | language defaults | language defaults | language defaults |

## Known false positives

Technical vocabulary (`function`, `implementation`, `configuration`) contains many legitimate nominalizations, which is why `dev-doc` relaxes the threshold. The `-al` suffix in English is too broad (flags `crucial`, `horizontal`, `positional` despite these not being abstract nouns) and is tracked for review in **[F-excessive-nominalization-suffix-refine](../roadmap.md#f-excessive-nominalization-suffix-refine)** on the [roadmap](../roadmap.md).

## Example

Nominalizations colour-matched to their active-verb counterparts in the rewrite.

Before (heavy):

<!-- lucid-lint disable-next-line lexicon.excessive-nominalization -->

> La <span class="lucid-idea" data-idea="1">réalisation</span> de l'<span class="lucid-idea" data-idea="2">analyse</span> de la conformité permettra l'<span class="lucid-idea" data-idea="3">identification</span> des axes d'<span class="lucid-idea" data-idea="4">amélioration</span>.

After (lighter):

> Nous <span class="lucid-idea" data-idea="2">analyserons</span> la conformité. Cela permettra d'<span class="lucid-idea" data-idea="3">identifier</span> les axes à <span class="lucid-idea" data-idea="4">améliorer</span>.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## References

- [Plain Language US (2011)](../references.md#plain-language-us-2011)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

See [References](../references.md) for the full bibliography.
