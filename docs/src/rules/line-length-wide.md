# `line-length-wide`

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Condition tags** | `dyslexia`, `general` |
| **Languages** | EN · FR (script-agnostic) |
| **Source** | [`src/rules/line_length_wide.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/line_length_wide.rs) |

## What it flags

Source lines wider than the per-profile ceiling. WCAG 1.4.8 (AAA) caps rendered text at roughly 80 characters per line because longer lines force the eye to track further between saccades and increase re-reading on return sweep — a known difficulty for dyslexic readers (BDA Dyslexia Style Guide).

## Detection

Per paragraph, scan each line's width in grapheme clusters and report lines above `max_line_length`. Fenced and indented code blocks are excluded by the Markdown parser; `paragraph.text` retains hard breaks from the source (Markdown soft-break reflow means a markdown-source paragraph wrapped as the author intended counts as one reflowed line for this rule — plain-text and stdin inputs measure the raw source lines directly).

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_line_length` | `int` | 120 | 100 | 80 |

FALC matches the WCAG 1.4.8 AAA recommendation of 80 characters.

## Known caveats

Markdown soft breaks in the source are reflowed to spaces during parsing; the rule therefore sees the *reflowed* paragraph text. A markdown paragraph whose source lines are each under 80 characters but whose reflowed text is 400 characters will still fire. For hard control over wrap width, lint the rendered output or use a plain-text input.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`paragraph-too-long`](./paragraph-too-long.md)
- [`sentence-too-long`](./sentence-too-long.md)
- [Conditions](../guide/conditions.md)
