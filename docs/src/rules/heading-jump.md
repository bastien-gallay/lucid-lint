# `structure.heading-jump`

## What it flags

Heading-level jumps that break the document's mental map (e.g. H2 → H4). Each level must follow the previous by at most +1. Readers with attentional difficulties lean heavily on heading hierarchy to reposition after an interruption; a broken hierarchy destroys that cue. Also flags the first heading being deeper than H2 when `allow_first_heading_any_level` is `false`, and missing H1 when `require_h1` is `true`.

**References.** WCAG 2.1 SC 1.3.1 (Info and Relationships) and 2.4.6 (Headings and Labels); RGAA 9.1.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | language-agnostic |
| **Source** | [`src/rules/heading_jump.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/heading_jump.rs) |

## Detection

Parse Markdown headings (`#`, `##`, …). Walk them in source order; report each heading whose level exceeds the previous by more than one. Deterministic, no false positives.

## Parameters

| Key | Type | Default |
|---|---|---|
| `allow_first_heading_any_level` | `bool` | `true` |
| `require_h1` | `bool` | `false` |

A binary rule — no per-profile thresholds.

## Examples

Flagged:

```markdown
# Overview
#### Details    ← jumps from H1 to H4
```

Clean:

```markdown
# Overview
## Section
### Subsection
```

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`structure.deeply-nested-lists`](./deeply-nested-lists.md) — the list-level equivalent signal.
