# `structure.deeply-nested-lists`

## What it flags

Bulleted list items nested beyond a reasonable depth. A deeply nested list forces the reader to reconstruct a complex mental hierarchy — horizontal indentation stops being a positional cue and becomes noise. Four levels of indent are too many for readers with attentional difficulties to track.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | language-agnostic |
| **Source** | [`src/rules/deeply_nested_lists.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/deeply_nested_lists.rs) |

## Detection

Parse Markdown via `pulldown-cmark`; extract list items with their indentation level; flag items deeper than `max_depth`. Deterministic, no false positives.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_depth` | `int` | 4 | 3 | 2 |

## Example

Under `public` (max depth 3):

```markdown
- Level 1
  - Level 2
    - Level 3
      - Level 4    ← flagged
```

## Diagnostic message

Includes repair guidance: flatten the structure, split into multiple lists, or promote sub-items to subsections with headings.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).
