# `unclear-antecedent`

| | |
|---|---|
| **Category** | `syntax` |
| **Default severity** | `info` |
| **Default weight** | `2` |
| **Languages** | EN · FR (separate pronoun lists) |
| **Source** | [`src/rules/unclear_antecedent.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/unclear_antecedent.rs) |

> **Note.** `RULES.md` currently lists this rule under `rhythm`; `Category::for_rule` classifies it as `syntax`. Code is authoritative for scoring.

## What it flags

Pronouns whose antecedent is not obvious in the immediate context. Ambiguous pronominal reference is one of the costliest comprehension breaks for readers with attentional difficulties: each ambiguity forces a conscious return-and-search.

**References.** Strunk & White; FALC ("prefer name repetition over pronouns"); Graesser et al. *Coh-Metrix* (referential cohesion).

## Detection (v0.1 heuristic)

Exact detection requires anaphora resolution (advanced NLP). v0.1 catches the two most frequent patterns:

1. Bare demonstrative pronouns at sentence start (`This`/`That`/`These`/`Those`, `Ceci`/`Cela`/`Ce`) **not** followed by a noun.
2. Personal pronouns at paragraph start (no antecedent in the preceding context).

Severity is `info` because the heuristic is approximate — the noise level warrants a soft signal.

## Parameters

| Key | Type | Default |
|---|---|---|
| `check_demonstratives` | `bool` | `true` |
| `check_paragraph_start_pronouns` | `bool` | `true` |

## Pronoun lists

- 🇫🇷 *ce, cela, ceci, ça, celui-ci, celle-ci, il, elle, ils, elles*
- 🇬🇧 *this, that, these, those, it, they, them*

## Example

> Les performances étaient médiocres avec le cache LRU. **Cela** a motivé le changement.

*Cela* refers to the performance? The cache? Ambiguous.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).
