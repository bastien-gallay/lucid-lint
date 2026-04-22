# `lexicon.all-caps-shouting`

## What it flags

Runs of consecutive ALL-CAPS words. ALL-CAPS prose strips the shape cues (ascenders, descenders, x-height contrast) that dyslexic readers rely on to disambiguate words, and it triggers many screen readers to spell out the run letter by letter unless the surrounding markup says otherwise. WCAG 3.1.5 and the BDA Dyslexia Style Guide both recommend lowercase or sentence case for emphasis.

## At a glance

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Condition tags** | `a11y-markup`, `dyslexia`, `general` |
| **Languages** | EN · FR (script-only detection — language-agnostic) |
| **Source** | [`src/rules/all_caps_shouting.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/all_caps_shouting.rs) |

## Detection

Per paragraph, scan for runs of consecutive ALL-CAPS words. Minor connectors (`,`, `;`, `:`, `-`, whitespace) keep a run alive; a lowercase word, a period, or paragraph break ends it.

A word is ALL-CAPS when it is at least 2 letters long and contains no lowercase letter. Single ALL-CAPS tokens are treated as abbreviations and are the responsibility of [`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md).

Code blocks are excluded by the Markdown parser before the rule runs.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_run_length` | `int` | 3 | 2 | 2 |

`dev-doc` tolerates a 2-word emphasis run (`DO NOT`) common in technical docs.

## Examples

**Triggers under `public`:**

> Please DO NOT touch this.

`DO NOT` reads as shouting. Use bold or italics instead: *do not touch this*.

## Known false positives

A chain of three or more acronyms in prose (`API HTTP TLS`) is structurally indistinguishable from shouting and will fire. Suppress on the line if the chain is intentional, or restructure the prose.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md)
- [Conditions](../guide/conditions.md)
