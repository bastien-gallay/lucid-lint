# `repetitive-connectors`

| | |
|---|---|
| **Category** | `rhythm` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (separate lists) |
| **Source** | [`src/rules/repetitive_connectors.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/repetitive_connectors.rs) |

## What it flags

Overuse of a single logical connector inside a short window of sentences. Connectors (opposition, cause, consequence, sequence, illustration, addition) are attentional anchors; repeated, they flatten the sense of progression. Sanders & Noordman (2000), *Connectives as processing signals*; Graesser et al. (2004), local cohesion.

## Detection

Sliding window of `window_size` sentences. Per connector, count occurrences in the window. Fire once per cluster that crosses `max_per_window`.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_per_window` | `int` | 4 | 3 | 2 |
| `window_size` | `int` | 5 | 5 | 5 |
| `custom_connectors` | `list` | `[]` | `[]` | `[]` |

## Default connector lists

- 🇫🇷 Opposition: *cependant, toutefois, en revanche, néanmoins, pourtant, mais*
- 🇫🇷 Cause: *parce que, car, puisque, en effet*
- 🇫🇷 Consequence: *donc, ainsi, par conséquent, c'est pourquoi*
- 🇫🇷 Sequence: *d'abord, ensuite, puis, enfin, premièrement*
- 🇫🇷 Illustration: *par exemple, notamment, en particulier*
- 🇫🇷 Addition: *de plus, en outre, également, par ailleurs*
- 🇬🇧 Opposition: *however, nevertheless, yet, although, but*
- 🇬🇧 Cause: *because, since, as, for*
- 🇬🇧 Consequence: *therefore, thus, consequently, hence, so*
- 🇬🇧 Sequence: *first, then, next, finally*
- 🇬🇧 Illustration: *for example, notably, in particular, such as*
- 🇬🇧 Addition: *moreover, furthermore, also, additionally*

## Example

<!-- lucid-lint-disable repetitive-connectors -->

> We analysed the data. Then we built the model. Then we validated the results. Then we published the report.

<!-- lucid-lint-enable -->

Four *then* — no progression felt.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).
