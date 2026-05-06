# `structure.number-run`

> *Experimental in v0.2.x.* Off by default; opt in via
> `--experimental structure.number-run` or
> `[experimental] enabled = ["structure.number-run"]` in
> `lucid-lint.toml`. Flips to `Stable` at the v0.3 cut as part of the
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status)
> cohort flip. See [Conditions](../guide/conditions.md) for the
> `dyscalculia` condition tag that gates this rule under user-active
> conditions.

## What it flags

Sentences that pack more than a configurable number of numeric tokens together. plainlanguage.gov is explicit on the framing — *"Don't put a lot of numbers together in one sentence"* and *"Avoid placing too many statistics close together"* — and readers with dyscalculia carry the cost first: each numeric token forces a quantity-to-symbol re-anchoring that does not benefit from running prose context the way ordinary words do. Citation salads (`(Smith 2020, Jones 2021, Wei 2022, Park 2023)`), benchmark tables flattened into prose, and statistic-heavy paragraphs are the typical hits.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Status** | `experimental` (v0.2.x) → `stable` at v0.3 cut |
| **Condition tag** | `dyscalculia` (gated; runs only under matching `--conditions`) |
| **Languages** | EN · FR (identical detection — digits are language-agnostic) |
| **Source** | [`src/rules/structure/number_run.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/structure/number_run.rs) |

## Detection

Walks each paragraph's sentence stream (post-flattening, so fenced code blocks are already excluded by the parser) and counts numeric tokens per sentence. A numeric token is a contiguous run of ASCII digits, optionally containing one decimal separator (`.` or `,`) followed by more digits. Hyphen, colon, slash, and whitespace split tokens.

| Input | Tokens counted | Note |
|---|---|---|
| `42` | 1 | Bare integer |
| `3.14` | 1 | Decimal separator kept |
| `1,000` | 1 | Comma separator kept |
| `2026-05-04` | 3 | Hyphens split — a date is three numbers from a load standpoint |
| `$3.50` | 1 | Currency prefix is non-digit and ignored |
| `1st` | 1 | Trailing letters split; the digits still count |

The diagnostic location points at the *first* numeric token in the offending sentence, so the squiggle in your editor lands on the visible cluster rather than the start of the sentence.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_numbers` | `int` | 6 | 4 | 3 |

Tune via `lucid-lint.toml`:

```toml
[rules."structure.number-run"]
max_numbers = 5
```

## Examples

### English

**Before** (flagged):

> The 2024 cohort sat 1,200 students across 4 campuses, posted a 92.5% pass rate on the 3 reviewed papers, and improved 18 points over the prior year.

What `lucid-lint check --profile public --experimental structure.number-run --conditions dyscalculia` reports:

```text
warning input.md:1:5 Sentence packs 8 numeric tokens (maximum 4). plain-language guidance recommends not placing many numbers or statistics together in one sentence; split the sentence or move some figures to a list or table. [structure.number-run]
```

**After** (your rewrite):

> The 2024 cohort sat 1,200 students across 4 campuses. They posted a 92.5% pass rate on the reviewed papers and improved 18 points over the prior year.

The figures still travel together, but each sentence carries a load a dyscalculic reader can re-anchor without losing the running referent.

### French

**Before** (flagged):

> La promotion 2024 a réuni 1 200 étudiants sur 4 campus, affiché un taux de réussite de 92,5 % sur les 3 copies revues, et progressé de 18 points par rapport à l'année précédente.

**After** (your rewrite):

> La promotion 2024 a réuni 1 200 étudiants sur 4 campus. Le taux de réussite atteint 92,5 % sur les copies revues et progresse de 18 points par rapport à l'année précédente.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md) for the inline and block forms. Inline disable also works on this rule:

```markdown
<!-- lucid-lint disable-next-line structure.number-run -->
The 2024 cohort sat 1,200 students across 4 campuses, posted a 92.5% pass rate on the 3 reviewed papers, and improved 18 points.
```

## See also

- [Conditions](../guide/conditions.md) — the `dyscalculia` tag that gates this rule.
- [`structure.mixed-numeric-format`](./mixed-numeric-format.md) — sibling rule on numeric *form* consistency. Atomic split: `mixed-numeric-format` cares whether digits and spelled-out numerals share one sentence; `number-run` cares about how many numeric tokens cluster regardless of form.
- [F-experimental-rule-status — experimental rule status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrate that lets this rule ship in v0.2.x without affecting default scores.

## References

- [plainlanguage.gov — Use short, simple sentences](https://www.plainlanguage.gov/guidelines/concise/use-short-simple-sentences/). *"Don't put a lot of numbers together in one sentence."*
- [plainlanguage.gov — Use numerals](https://www.plainlanguage.gov/guidelines/words/use-numerals/). Companion guidance on consistent numeric form (the grounding for `mixed-numeric-format`).

See [References](../references.md) for the full bibliography.
