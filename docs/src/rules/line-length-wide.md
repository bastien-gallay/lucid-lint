# `structure.line-length-wide`

## What it flags

Author-chosen lines wider than the per-profile ceiling. WCAG 1.4.8 (AAA) caps rendered text at roughly 80 characters per line because longer lines force the eye to track further between saccades and increase re-reading on return sweep — a known difficulty for dyslexic readers (BDA Dyslexia Style Guide).

"Author-chosen" matters: in Markdown, soft-wrapped lines collapse to spaces at parse time because the renderer reflows them to fit the viewport. Their source length tells us nothing about what the reader sees. Only line breaks the author kept on purpose are checked here — Markdown hard breaks (`<br>` or two trailing spaces) and explicit newlines in plain-text input. A soft-wrapped Markdown paragraph is exempt no matter how long its joined text is. Use [`structure.paragraph-too-long`](./paragraph-too-long.md) to bound paragraph density.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Condition tags** | `dyslexia`, `general` |
| **Languages** | EN · FR (script-agnostic) |
| **Source** | [`src/rules/line_length_wide.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/line_length_wide.rs) |

## Detection

For every paragraph that carries an authorial line break, scan each line's width in grapheme clusters and report lines above `max_line_length`.

A Markdown paragraph with no hard break inside it (the common case for prose) is exempt — the parser collapses its soft breaks to spaces, so what remains is one logical line whose source length tracks the viewport, not the rendered width WCAG 1.4.8 targets. Plain-text input is treated symmetrically: a paragraph with no inner `\n` is exempt; one with internal newlines is checked line by line.

Fenced and indented code blocks are excluded upstream by the Markdown parser. Headings, list items, and table cells are out of scope by construction — `paragraph-too-long`, `sentence-too-long`, and the heading rules cover the cognitive-load concerns that apply to those blocks.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_line_length` | `int` | 120 | 100 | 80 |

FALC matches the WCAG 1.4.8 AAA recommendation of 80 characters.

## Known caveats

Long single-line prose paragraphs in Markdown source are intentionally exempt. The rule used to fire on them and produced large amounts of noise on real prose; v0.2.x narrows the rule to author-chosen breaks only. Pair this rule with [`structure.paragraph-too-long`](./paragraph-too-long.md) if you also want a ceiling on the joined paragraph length.

Headings and list items are not measured by this rule. Their wrap behavior depends on the renderer (display type, list indent), and the underlying cognitive concerns are covered by other rules.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`structure.paragraph-too-long`](./paragraph-too-long.md)
- [`structure.sentence-too-long`](./sentence-too-long.md)
- [Conditions](../guide/conditions.md)
