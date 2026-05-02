# `structure.italic-span-long`

> *Experimental in v0.2.x.* Off by default; opt in via
> `--experimental structure.italic-span-long` or
> `[experimental] enabled = ["structure.italic-span-long"]` in
> `lucid-lint.toml`. Flips to `Stable` at the v0.3 cut as part of the
> [F139](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f139)
> cohort flip. See [Conditions](../guide/conditions.md) for the
> `dyslexia` condition tag that gates this rule under user-active
> conditions.

## What it flags

Italic spans (`*…*` / `_…_`) longer than a configurable word threshold. Slanted glyphs degrade letter-shape recognition for readers with dyslexia — a robust finding behind the British Dyslexia Association's recommendation to keep italic emphasis to a short phrase rather than running a full sentence in italics. Long italic runs also harm scanability for readers whose attention is already taxed (fatigue, second-language reading, low-vision conditions).

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Status** | `experimental` (v0.2.x) → `stable` at v0.3 cut |
| **Condition tag** | `dyslexia` (gated; runs only under matching `--conditions`) |
| **Languages** | EN · FR (identical detection — substrate is language-agnostic) |
| **Source** | [`src/rules/structure/italic_span_long.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/structure/italic_span_long.rs) |

## Detection

Walks the typed inline tree captured on each `Paragraph` ([F143](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f143) substrate) and flags every `Inline::Emphasis` span whose visible word count exceeds the per-profile threshold. Code blocks and inline code are excluded by the parser, so an italic span inside a code fence never fires. Strong (`**bold**`) does not trigger this rule — only emphasis (`*italic*` / `_italic_`).

The diagnostic location points at the *opening* delimiter, so the squiggle in your editor lands on the visible `*` or `_` rather than an arbitrary column inside the paragraph.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_words` | `int` | 12 | 8 | 5 |

Tune via `lucid-lint.toml`:

```toml
[rules."structure.italic-span-long"]
max_words = 6
```

## Examples

### English

**Before** (flagged):

> The team eventually concluded that *the proposed migration plan would require careful coordination across three regional offices and an extended freeze window* before any deployment could begin.

What `lucid-lint check --profile public --experimental structure.italic-span-long --conditions dyslexia` reports:

```text
warning input.md:1:36 Italic span is 17 words long (maximum 8). Long italic runs strain dyslexic readers; consider shortening the emphasized phrase or removing the italics. [structure.italic-span-long]
```

**After** (your rewrite):

> The team eventually concluded that the proposed migration plan would require careful coordination. Three regional offices and an extended freeze window are *prerequisites* before any deployment.

The italics now mark a single load-bearing word — the kind of emphasis the BDA style guide endorses.

### French

**Before** (flagged):

> L'équipe a fini par conclure que *le plan de migration proposé nécessiterait une coordination soignée entre trois bureaux régionaux et une fenêtre de gel prolongée* avant tout déploiement.

What `lucid-lint check --profile public --experimental structure.italic-span-long --conditions dyslexia` reports:

```text
warning input.md:1:35 Italic span is 18 words long (maximum 8). Long italic runs strain dyslexic readers; consider shortening the emphasized phrase or removing the italics. [structure.italic-span-long]
```

**After** (your rewrite):

> L'équipe a fini par conclure que le plan de migration nécessiterait une coordination soignée. Trois bureaux régionaux et une fenêtre de gel prolongée sont *indispensables* avant tout déploiement.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md) for the inline and block forms. Inline disable also works on this rule:

```markdown
<!-- lucid-lint disable-next-line structure.italic-span-long -->
A *deliberately long italic span that the rule would normally flag* lives here.
```

## See also

- [Conditions](../guide/conditions.md) — the `dyslexia` tag that gates this rule.
- [F139 — experimental rule status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f139) — substrate that lets this rule ship in v0.2.x without affecting default scores.
- [F143 — inline AST layer](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f143) — substrate that exposes emphasis-span boundaries to this rule.

## References

- [British Dyslexia Association — Dyslexia Style Guide (2018)](https://www.bdadyslexia.org.uk/advice/employers/creating-a-dyslexia-friendly-workplace/dyslexia-friendly-style-guide). Recommends keeping italics to short phrases for letter-shape recognition.
- [Rello & Baeza-Yates (2013)](../references.md#rello-baeza-yates-2013) — broader academic context on dyslexia-friendly typography.

See [References](../references.md) for the full bibliography.
