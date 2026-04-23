# `rhythm.repetitive-connectors`

## What it flags

Overuse of a single logical connector inside a short window of sentences. Connectors (opposition, cause, consequence, sequence, illustration, addition) are attentional anchors; repeated, they flatten the sense of progression. Sanders & Noordman (2000), *Connectives as processing signals*; Graesser et al. (2004), local cohesion.

## At a glance

| | |
|---|---|
| **Category** | `rhythm` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (separate lists) |
| **Source** | [`src/rules/repetitive_connectors.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/repetitive_connectors.rs) |

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

## Examples

`lucid-lint` reports; the rewrite is always yours.

### English

Five actions, colour-matched across the rewrite — only the connectors change.

<!-- lucid-lint-disable rhythm.repetitive-connectors -->

**Before** (flagged):

> <span class="lucid-idea" data-idea="1">We analysed the data.</span> Then <span class="lucid-idea" data-idea="2">we built the model.</span> Then <span class="lucid-idea" data-idea="3">we validated the results.</span> Then <span class="lucid-idea" data-idea="4">we published the report.</span> Then <span class="lucid-idea" data-idea="5">we archived the raw data.</span>

Four *then* in five sentences — no progression felt.

What `lucid-lint check --profile public` reports:

```text
warning input.md:1:1 Connector "then" appears 4 times within 5 consecutive sentences (max 3). Vary the connector or restructure the passage. [rhythm.repetitive-connectors]
```

**After** (your rewrite):

> <span class="lucid-idea" data-idea="1">We analysed the data.</span> From it <span class="lucid-idea" data-idea="2">we built the model.</span> <span class="lucid-idea" data-idea="3">Validation followed,</span> and once the results held up <span class="lucid-idea" data-idea="4">we published the report.</span> <span class="lucid-idea" data-idea="5">The raw data was archived last.</span>

<!-- lucid-lint-enable -->

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).
