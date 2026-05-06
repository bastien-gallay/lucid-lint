# `readability.large-number-unanchored`

> *Experimental in v0.2.x.* Off by default; opt in via
> `--experimental readability.large-number-unanchored` or
> `[experimental] enabled = ["readability.large-number-unanchored"]` in
> `lucid-lint.toml`. Flips to `Stable` at the v0.3 cut as part of the
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status)
> cohort flip. See [Conditions](../guide/conditions.md) for the
> `dyscalculia` and `general` condition tags.

## What it flags

A large numeral or magnitude word that appears in a sentence with no nearby anchor — no unit, no percentage, no currency symbol, no ratio, no comparator phrase. The CDC Clear Communication Index asks whether numbers are *clear and meaningful for the primary audience*; plainlanguage.gov is more direct on the mechanism — *"Use Numbers Effectively"* recommends giving every large figure a comparison or denominator the reader can ground. Readers with dyscalculia carry the cost first: a context-free *"4.8 milliards"* forces an unaided magnitude estimate that ordinary prose context does not provide.

The rule complements `structure.number-run`, which fires on numeric *clusters* (≥ N tokens in one sentence). This rule fires on a *single* large or magnitude-word numeral that lacks anchoring context.

## At a glance

| | |
|---|---|
| **Category** | `readability` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Status** | `experimental` (v0.2.x) → `stable` at v0.3 cut |
| **Condition tags** | `dyscalculia`, `general` (gated; runs only under matching `--conditions`) |
| **Languages** | EN · FR (per-language comparator and figure-ref lexicons) |
| **Source** | [`src/rules/readability/large_number_unanchored.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/readability/large_number_unanchored.rs) |

## Detection

For each sentence, the rule walks the post-flattening paragraph text (so fenced code blocks are already excluded by the parser) and searches for unanchored candidates.

### Candidate definition

A sentence-level candidate is either:

1. A **numeric token** whose digit count is ≥ 4 *and* whose integer value is ≥ the profile threshold. The scanner collapses common thousands separators (`,`, `.`, ASCII space, NBSP, thin space, narrow NBSP) between digit groups so `1 000` (FR) and `1,000` (EN) both count as one 4-digit token with value 1000.
2. A **magnitude word** — `million`(s), `billion`(s), `trillion`(s) in EN; `million`(s), `milliard`(s), `billion`(s), `trillion`(s) in FR. Whole-word, case-insensitive.

### Skips (false-positive guards)

- **Year-shaped**: exactly 4 contiguous digits with no thousands or decimal separators and value in `1000..=2999`. `2024` and `1789` are years, not magnitudes.
- **Ordinal**: digit run immediately followed by a letter (`1st`, `12th`).
- **Figure / page / section reference**: candidate preceded (within 16 bytes, same sentence) by `Figure`, `Fig.`, `Page`, `Section`, `§`, `p.`, `pp.`, `#`, or the FR equivalents (`figure`, `page`, `section`, `tableau`, `chapitre`, `annexe`).

### Anchor types (sentence-scoped)

Any of the following anywhere in the sentence anchors *all* candidates in that sentence:

- Percent sign (`%`).
- Currency symbol (`$`, `€`, `£`, `¥`).
- Unit token from a small curated list (`km`, `kg`, `m²`, `°C`, `L`, `Hz`, `MB`, `Mo`, …).
- Ratio pattern: `X out of Y`, `X sur Y`, or `X / Y` between digits.
- Comparator phrase from the per-language lexicon (EN: `roughly`, `approximately`, `more than`, `the size of`, …; FR: `soit environ`, `équivalent à`, `environ`, `plus de`, `par rapport à`, …).

The diagnostic location points at the *first* surviving candidate in the offending sentence, so the squiggle in your editor lands on the visible numeral rather than the start of the sentence.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_value` | `int` | 100000 | 10000 | 1000 |

`min_value` is the inclusive lower bound on the integer value of a numeric candidate. Tokens that meet the digit-count gate but fall below `min_value` are skipped — page-number-like quantities already get the figure-ref skip; this is a second safety net.

Tune via `lucid-lint.toml`:

```toml
[rules."readability.large-number-unanchored"]
min_value = 50000
```

## Examples

### English

**Before** (flagged):

> The proposal mentions several billion in vague spending across regions.

What `lucid-lint check --profile public --experimental readability.large-number-unanchored --conditions dyscalculia` reports:

```text
warning input.md:1:32 Magnitude word appears with no anchor in this sentence (no unit, percentage, ratio, or comparison phrase). plain-language guidance recommends pairing magnitude words with a unit or a comparison the reader can ground. [readability.large-number-unanchored]
```

**After** (your rewrite):

> The proposal mentions several billion dollars in vague spending across regions, roughly the annual budget of a mid-sized state agency.

The figure now sits next to a unit (`dollars`) and a comparator phrase (`roughly the annual budget`); both anchor the magnitude for a reader who cannot ground it from raw scale.

### French

**Before** (flagged):

> Le budget atteint 4 800 000 000 selon le rapport final.

What `lucid-lint check --profile public --experimental readability.large-number-unanchored --conditions dyscalculia` reports:

```text
warning input.md:1:19 Large numeral (10-digit, value ≈ 4800000000) appears with no anchor in this sentence (no unit, percentage, ratio, or comparison phrase). plain-language guidance recommends giving large numbers a comparison or denominator the reader can ground. [readability.large-number-unanchored]
```

**After** (your rewrite):

> Le budget atteint 4,8 milliards d'euros, soit environ 6 % du PIB selon le rapport final.

The figure is now accompanied by a unit (`euros`), a percentage (`6 %`), and a comparator phrase (`soit environ`). A reader who cannot estimate "4,8 milliards" raw now has three independent anchors.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md) for the inline and block forms. Inline disable also works on this rule:

```markdown
<!-- lucid-lint disable-next-line readability.large-number-unanchored -->
The proposal mentions several billion in vague spending across regions.
```

## See also

- [Conditions](../guide/conditions.md) — the `dyscalculia` and `general` tags that gate this rule.
- [`structure.number-run`](./number-run.md) — sibling rule on numeric clustering. Atomic split: `number-run` fires on clusters of numeric tokens; this rule fires on a single unanchored large numeral.
- [`structure.mixed-numeric-format`](./mixed-numeric-format.md) — another sibling on numeric *form* consistency (digits vs spelled-out).
- [F-experimental-rule-status — experimental rule status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrate that lets this rule ship in v0.2.x without affecting default scores.

## References

- [plainlanguage.gov — Use numbers effectively](https://www.plainlanguage.gov/guidelines/words/use-numbers-effectively/). *"Help your reader visualize numbers… Compare numbers to something the reader is familiar with."*
- [CDC Clear Communication Index — Numbers](https://www.cdc.gov/ccindex/). Item 6 asks whether numbers are clear and meaningful for the primary audience.

See [References](../references.md) for the full bibliography.
