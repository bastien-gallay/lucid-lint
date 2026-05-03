# `structure.deep-subordination`

## What it flags

Cascading subordinate clauses: multiple relative pronouns or subordinating conjunctions chained without a strong-punctuation break. Each open referent has to sit in working memory until it closes — Gibson's *Dependency Locality Theory* (1998) ties processing cost directly to that distance.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `2` |
| **Languages** | EN · FR (separate lists) |
| **Source** | [`src/rules/deep_subordination.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/deep_subordination.rs) |

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

## Examples

Each highlighted token is one subordinator counted by the rule. Four in a row triggers the `dev-doc` threshold (3); two in a row triggers `public` and `falc`.

Flagged (FR):

<!-- lucid-lint disable-next-line structure.deep-subordination -->
<!-- lucid-lint disable-next-line syntax.passive-voice -->

> Le document <span class="lucid-idea" data-idea="1">qui</span> a été rédigé par l'équipe <span class="lucid-idea" data-idea="2">que</span> nous avons constituée et <span class="lucid-idea" data-idea="3">qui</span> couvre les points <span class="lucid-idea" data-idea="4">que</span> nous avions discutés…

Flagged (EN):

<!-- lucid-lint disable-next-line structure.deep-subordination -->
<!-- lucid-lint disable-next-line syntax.passive-voice -->

> The report <span class="lucid-idea" data-idea="1">that</span> was drafted by the team <span class="lucid-idea" data-idea="2">which</span> we formed last month and <span class="lucid-idea" data-idea="3">which</span> covers the topics <span class="lucid-idea" data-idea="4">that</span> we had discussed…

Not flagged (enumeration form, recognised by the detector):

> Les pronoms relatifs en français sont : qui, que, dont, où.

And the matching English form:

> The English relative pronouns are: which, that, who, whom, whose.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## References

- [Gibson (1998)](../references.md#gibson-1998)

See [References](../references.md) for the full bibliography.
