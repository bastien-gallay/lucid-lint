# `redundant-intensifier`

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Condition tags** | `general` |
| **Languages** | EN · FR |
| **Source** | [`src/rules/redundant_intensifier.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/redundant_intensifier.rs) |

## What it flags

Intensifiers — adverbs that try to *upgrade* the confidence of a statement without adding information. `very important` reduces to `important`, or better, to a quantified claim. plainlanguage.gov (Chapter 4) and the CDC Clear Communication Index flag intensifiers as a plain-language anti-pattern.

The rule is a deliberate sibling of [`weasel-words`](./weasel-words.md): weasel words *downgrade* confidence (hedges, qualifiers); redundant intensifiers *upgrade* it. The two lists are disjoint by construction.

## Detection

Per paragraph, lowercase the text and look for each intensifier phrase in the per-language list ([`en::INTENSIFIERS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/en.rs), [`fr::INTENSIFIERS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/fr.rs)) using the shared word-bounded search. Hits inside fenced or inline code spans are ignored. Documents whose language is `Unknown` are skipped rather than guessed, matching `weasel-words`.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `custom_intensifiers_en` | `list<string>` | `[]` | `[]` | `[]` |
| `custom_intensifiers_fr` | `list<string>` | `[]` | `[]` | `[]` |
| `disable` | `list<string>` | `[]` | `[]` | `[]` |

`custom_intensifiers_en` / `_fr` add phrases to the defaults. `disable` removes phrases from them (exact lowercase match).

## Known caveats

- `very` in the fixed phrase `very well` (as acknowledgment) still triggers — plain-language guides flag it anyway, so the rule does not carve out an exception. Suppress via inline directive if the context genuinely calls for it.
- Metalinguistic references ("the word 'very' is an intensifier") trigger unless the target word is in backticks. Use inline code spans for such references.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`weasel-words`](./weasel-words.md)
- [`jargon-undefined`](./jargon-undefined.md)
- [Conditions](../guide/conditions.md)
