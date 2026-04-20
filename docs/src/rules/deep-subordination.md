# `deep-subordination`

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `2` |
| **Languages** | EN · FR (separate lists) |
| **Source** | [`src/rules/deep_subordination.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/deep_subordination.rs) |

> **Note.** `RULES.md` currently lists this rule under `syntax`; the running `Category::for_rule` classifies it as `structure`. Code is authoritative for scoring.

## What it flags

Cascading subordinate clauses: multiple relative pronouns or subordinating conjunctions chained without a strong-punctuation break. Each open referent has to sit in working memory until it closes — Gibson's *Dependency Locality Theory* (1998) ties processing cost directly to that distance.

## Detection

Walk the sentence between strong-punctuation breaks; count consecutive subordinators. Flag when the count exceeds `max_consecutive_subordinators`. Pronoun *enumerations* (`qui, que, dont, où`) are skipped — the detector recognises the list form and does not treat it as cascading.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_consecutive_subordinators` | `int` | 3 | 2 | 2 |

## Language lists

- 🇫🇷 Relative pronouns: *qui, que, dont, où, lequel, laquelle, lesquels, lesquelles*
- 🇫🇷 Subordinators: *parce que, afin que, bien que, quoique, puisque, pour que, tandis que*
- 🇬🇧 Relative pronouns: *which, that, who, whom, whose*
- 🇬🇧 Subordinators: *because, although, while, since, whereas, unless, until*

## Example

Flagged (FR):

<!-- lucid-lint disable-next-line deep-subordination -->
<!-- lucid-lint disable-next-line passive-voice -->

> Le document qui a été rédigé par l'équipe que nous avons constituée et qui couvre les points que nous avions discutés…

Not flagged:

> Les pronoms relatifs en français sont : qui, que, dont, où.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).
