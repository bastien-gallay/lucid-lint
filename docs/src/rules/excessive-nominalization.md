# `excessive-nominalization`

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (overlapping suffix lists) |
| **Source** | [`src/rules/excessive_nominalization.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/excessive_nominalization.rs) |

## What it flags

Sentences densely packed with nominalizations — verbs turned into abstract nouns. Two problems compound: nominalized text is more abstract (costlier to process) and hides the agent ("who does what" is obscured). FALC and the US Plain Writing Act both recommend strong verbs over nominalizations.

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

Technical vocabulary (`function`, `implementation`, `configuration`) contains many legitimate nominalizations, which is why `dev-doc` relaxes the threshold. The `-al` suffix in English is too broad (flags `crucial`, `horizontal`, `positional` despite these not being abstract nouns) and is tracked for review in **[F24](../roadmap.md)** on the [roadmap](../roadmap.md).

## Example

Before (heavy):

<!-- lucid-lint disable-next-line excessive-nominalization -->

> La réalisation de l'analyse de la conformité permettra l'identification des axes d'amélioration.

After (lighter):

> Nous analyserons la conformité. Cela permettra d'identifier les axes à améliorer.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).
