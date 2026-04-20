# `long-enumeration`

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/long_enumeration.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/long_enumeration.rs), shared helper [`src/rules/enumeration.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/enumeration.rs) |

## What it flags

Inline prose enumerations that would be clearer as a bulleted list — 5+ comma-separated items closed by a coordinator (`and`, `or`, `et`, `ou`).

## Detection

Sequence of `min_items` or more short comma-separated segments ending with `, et` / `, ou` / `, and` / `, or` (Oxford comma optional). Shared detector also informs [`excessive-commas`](./excessive-commas.md).

## Parameters

| Key | Type | Default |
|---|---|---|
| `min_items` | `int` | `5` |

## Diagnostic message

Suggests converting the enumeration to a bulleted list.

## Example

Flagged:

> The library supports JSON, YAML, TOML, CSV, INI, and XML.

Rewrite:

> The library supports the following formats:
>
> - JSON
> - YAML
> - TOML
> - CSV
> - INI
> - XML

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).
