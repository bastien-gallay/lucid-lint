# `unexplained-abbreviation`

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (whitelists differ) |
| **Source** | [`src/rules/unexplained_abbreviation.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/unexplained_abbreviation.rs) |

## What it flags

Acronyms used without a nearby definition. Each forced interruption to guess or look up an acronym breaks the flow and raises the risk of losing attention.

**References.** WCAG 2.1 SC 3.1.4 (Abbreviations); RGAA 9.4.

## Detection (v0.1 simplified)

1. Match sequences of 2+ consecutive uppercase letters (optionally with digits).
2. Subtract the whitelist.
3. Flag each remaining occurrence.

A two-pass definition-aware version (check whether the acronym is defined anywhere in the document) is tracked as **F9** on the [roadmap](../roadmap.md).

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_length` | `int` | 3 | 2 | 2 |
| `whitelist` | `list` | extended | minimal | empty |

**Default whitelist (v0.1):** general IT (`URL, HTML, CSS, JSON, XML, HTTP, HTTPS, API, CLI, GUI, OS, CPU, RAM, SSD, USB, WiFi`) plus common FR/EN (`PDF, SMS, GPS, ID, OK, FAQ`). Narrower project-scoped overrides are tracked as **F31**.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`jargon-undefined`](./jargon-undefined.md) — the content-word equivalent.
