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
