# `readability-score`

| | |
|---|---|
| **Category** | `readability` |
| **Default severity** | `info` (always reported) · `warning` when above `max_grade_level` |
| **Default weight** | `5` |
| **Languages** | formula calibrated for English; applied as-is to French in v0.1 (overestimates by +1–2 grades) |
| **Source** | [`src/rules/readability_score.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/readability_score.rs) |

## What it flags

A document-level readability index. Readability formulas are the historical synthetic signal for text complexity — simple, reproducible, recognized by US/UK government guidelines and WCAG. Treat it like cyclomatic complexity: a metric first, a warning second.

## Detection (v0.1 unified formula)

**Flesch–Kincaid Grade Level**, applied regardless of language:

```
0.39 × (words / sentences) + 11.8 × (syllables / words) − 15.59
```

| Grade | US school equivalent |
|---|---|
| < 6 | Elementary |
| 6–9 | Middle school |
| 9–12 | High school |
| 12–16 | College |
| > 16 | Expert |

Language-specific formulas (Kandel-Moles, Scolarius, SMOG, Coleman-Liau) and a user-configurable formula choice are tracked as **[F10](../roadmap.md)** / **[F11](../roadmap.md)** on the [roadmap](../roadmap.md).

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_grade_level` | `float` | 14 | 9 | 6 |
| `always_report` | `bool` | `true` | `true` | `true` |

## Output modes

- Always reported as `info` (for observability, even when under the threshold).
- Reported as `warning` when the grade level exceeds `max_grade_level`.

## Suppression

Suppressing a document-level metric is rarely the right answer; adjust `max_grade_level` in `lucid-lint.toml` instead. See [Configuration](../guide/configuration.md).
