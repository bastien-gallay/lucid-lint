# `structure.long-enumeration`

## What it flags

Inline prose enumerations that would be clearer as a bulleted list — 5+ comma-separated items closed by a coordinator (`and`, `or`, `et`, `ou`).

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/long_enumeration.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/long_enumeration.rs), shared helper [`src/rules/enumeration.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/enumeration.rs) |

## Detection

Sequence of `min_items` or more short comma-separated segments ending with `, et` / `, ou` / `, and` / `, or` (Oxford comma optional). Shared detector also informs [`structure.excessive-commas`](./excessive-commas.md).

<aside class="since-version" aria-label="New in v0.3">

<span class="since-version__tag">Since v0.3</span> — The shared detector
also recognises rhythmically-regular runs of 1–4-word items (5+ items,
word-count spread ≤ 2, no clause boundary inside the run). Sentences
like `Each rule has a dedicated page below with category, severity,
default weight, thresholds per profile, examples, and suppression
guidance.` now surface as bullet-list candidates
([F22](../roadmap.md), second slice).

</aside>

## Parameters

| Key | Type | Default |
|---|---|---|
| `min_items` | `int` | `5` |

## Diagnostic message

Suggests converting the enumeration to a bulleted list.

## Examples

`lucid-lint` reports; the rewrite is always yours.

### English

Six items, colour-matched across the rewrite — each inline term lines up with its bullet.

<!-- lucid-lint-disable structure.long-enumeration -->
<!-- lucid-lint-disable syntax.dense-punctuation-burst -->

**Before** (flagged):

> The dish contains <span class="lucid-idea" data-idea="1">tomato</span>, <span class="lucid-idea" data-idea="2">onion</span>, <span class="lucid-idea" data-idea="3">garlic</span>, <span class="lucid-idea" data-idea="4">basil</span>, <span class="lucid-idea" data-idea="5">parsley</span>, and <span class="lucid-idea" data-idea="1">thyme</span>.

<!-- lucid-lint-enable -->

What `lucid-lint check --profile public` reports:

```text
warning input.md:1:1 Inline enumeration of 5 items. Consider converting it into a bulleted list so readers can scan the items. [structure.long-enumeration]
```

**After** (your rewrite):

> The dish contains:
>
> - <span class="lucid-idea" data-idea="1">tomato</span>
> - <span class="lucid-idea" data-idea="2">onion</span>
> - <span class="lucid-idea" data-idea="3">garlic</span>
> - <span class="lucid-idea" data-idea="4">basil</span>
> - <span class="lucid-idea" data-idea="5">parsley</span>
> - <span class="lucid-idea" data-idea="1">thyme</span>

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## References

- [Plain Language US (2011)](../references.md#plain-language-us-2011)

See [References](../references.md) for the full bibliography.
