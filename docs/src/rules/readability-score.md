# `readability-score`

## What it flags

A document-level readability index. Readability formulas are the historical synthetic signal for text complexity — simple, reproducible, recognized by US/UK government guidelines and WCAG. Treat it like cyclomatic complexity: a metric first, a warning second.

## At a glance

| | |
|---|---|
| **Category** | `readability` |
| **Default severity** | `info` (always reported) · `warning` when above `max_grade_level` |
| **Default weight** | `5` |
| **Languages** | EN — Flesch-Kincaid · FR — Kandel-Moles (auto-selected per detected language; v0.2+) |
| **Source** | [`src/rules/readability_score.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/readability_score.rs) |

## Detection (v0.2 — per-language formula)

The formula is selected by the document's detected language:

**English — Flesch-Kincaid Grade Level:**

```
0.39 × (words / sentences) + 11.8 × (syllables / words) − 15.59
```

The result is a US-school grade. Compared directly to `max_grade_level`.

**French — Kandel & Moles (1958):**

```
207 − 1.015 × (words / sentences) − 73.6 × (syllables / words)
```

The result is an ease score on roughly `0..100` (higher = easier), Flesch-style. To stay comparable across languages, the rule converts it to a grade-equivalent with the standard linear approximation `(100 − score) / 10`, and compares that against `max_grade_level`. The diagnostic message surfaces both the native ease score and the grade-equivalent.

**Unknown language** falls back to Flesch-Kincaid.

| Grade | US school equivalent |
|---|---|
| < 6 | Elementary |
| 6–9 | Middle school |
| 9–12 | High school |
| 12–16 | College |
| > 16 | Expert |

The `--readability-formula` flag (shipped with F11 in v0.2) pins a formula regardless of detected language: `--readability-formula flesch-kincaid` or `--readability-formula kandel-moles`. Default `auto` keeps the per-language behaviour. Additional formulas (`Gunning Fog`, `SMOG`, `Dale-Chall`, `Scolarius`) and multi-formula `--readability-verbose` reports remain on the [roadmap](../roadmap.md).

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_grade_level` | `float` | 14 | 9 | 6 |
| `always_report` | `bool` | `true` | `true` | `true` |
| `formula` | `auto` \| `flesch-kincaid` \| `kandel-moles` | `auto` | `auto` | `auto` |

Override `formula` via `--readability-formula` on the CLI; `auto` uses the detected language, other values pin the formula.

## Output modes

- Always reported as `info` (for observability, even when under the threshold).
- Reported as `warning` when the grade level exceeds `max_grade_level`.

## Suppression

Suppressing a document-level metric is rarely the right answer; adjust `max_grade_level` in `lucid-lint.toml` instead. See [Configuration](../guide/configuration.md).
