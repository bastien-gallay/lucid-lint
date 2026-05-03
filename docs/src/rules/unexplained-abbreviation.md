# `lexicon.unexplained-abbreviation`

## What it flags

Acronyms used without a nearby definition. Each forced interruption to guess or look up an acronym breaks the flow and raises the risk of losing attention.

**References.** WCAG 2.1 SC 3.1.4 (Abbreviations); RGAA 9.4.

## At a glance

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (whitelists differ) |
| **Source** | [`src/rules/unexplained_abbreviation.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/unexplained_abbreviation.rs) |

## Detection (v0.2, two-pass — F9)

1. **Pre-scan** the whole document for acronyms defined in either canonical form:
   - `Full Expansion (ACRONYM)` — example: `World Wide Web (WWW)`
   - `ACRONYM (Full Expansion)` — example: `WWW (World Wide Web)`

   The "expansion" side must contain at least two alphabetic words, so short parenthetical notes like `(TBD)` or `(check later)` do not count as definitions.

2. **Match** sequences of 2+ consecutive uppercase letters (optionally with digits) in the main text.
3. **Filter** each candidate against three layers, in order:
   1. Defined in document (from the pre-scan) — strongest.
   2. User whitelist from `[rules.unexplained-abbreviation].whitelist`.
   3. Baseline whitelist (profile-driven).
4. **Flag** each remaining occurrence.

A single definition anywhere in the document silences every occurrence of the same acronym — matching how readers actually use documentation (scroll back once to find the expansion, remember it thereafter).

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_length` | `int` | 3 | 2 | 2 |
| `whitelist` | `list` | extended | minimal | empty |

**Default whitelist (v0.2, narrowed by F31):** the infrastructure stack — `URL, HTML, CSS, JSON, XML, HTTP, HTTPS, UTF, IO, API, CLI, GUI, OS, CPU, RAM, SSD, USB, IDE, SDK, CI, CD` — plus common FR/EN acronyms and RFC 2119 emphasis keywords (`PDF, SMS, GPS, ID, OK, FAQ`, `MUST, SHALL, SHOULD, …`).

<aside class="since-version" aria-label="New in v0.2">

<span class="since-version__tag">Since v0.2</span> — Accessibility
standards (`WCAG`, `ARIA`, `RGAA`, …), AI/language-tech initialisms
(`LLM`, `NLP`), and engineering-practice acronyms
(`YAGNI`, `DRY`, `TDD`, …) are no longer in the shipped baseline.
Projects that use these should add them to
`[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml` — see
the [configuration guide](../guide/configuration.md#per-rule-overrides).

</aside>

```toml
[rules.unexplained-abbreviation]
whitelist = ["WCAG", "ARIA", "ADHD", "LLM"]
```

User-whitelist entries are **additive** over the baseline — they extend it, never replace it.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`lexicon.jargon-undefined`](./jargon-undefined.md) — the content-word equivalent.

## References

- [WCAG 2.1 — 3.1.4](../references.md#wcag-2-1)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

See [References](../references.md) for the full bibliography.
