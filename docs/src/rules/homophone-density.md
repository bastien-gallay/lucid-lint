# `lexicon.homophone-density`

> *Experimental in v0.2.x.* Off by default; opt in via
> `--experimental lexicon.homophone-density` or
> `[experimental] enabled = ["lexicon.homophone-density"]` in
> `lucid-lint.toml`. Flips to `Stable` at the v0.3 cut as part of the
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status)
> cohort flip. See [Conditions](../guide/conditions.md) for the
> `dyslexia` and `aphasia` condition tags that gate this rule under
> user-active conditions.

## What it flags

Paragraphs whose share of homophones — words that sound alike but spell differently (`their` / `there` / `they're`, `to` / `too` / `two`, `cours` / `court`, `amande` / `amende`) — exceeds a configurable percentage. Homophones force a phonological-then-orthographic disambiguation pass: the ear resolves the word, the eye must then pick the right spelling from context. That extra hop is cheap on its own and expensive in a cluster. The British Dyslexia Association style guide flags homophones as a known friction point for dyslexic readers, and the FALC orthographic-clarity guidelines recommend rephrasing dense homophone runs for aphasic and plain-language audiences.

## At a glance

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Status** | `experimental` (v0.2.x) → `stable` at v0.3 cut |
| **Condition tags** | `dyslexia`, `aphasia` (gated; runs only under matching `--conditions`) |
| **Languages** | EN · FR (curated per-language homophone lists) |
| **Source** | [`src/rules/lexicon/homophone_density.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/lexicon/homophone_density.rs) |

## Detection

For each paragraph, walk the word stream once, count alphabetic words as the denominator, and count words that appear in the per-language homophone table as hits. If `hits / total` strictly exceeds the per-profile threshold, emit one diagnostic anchored at the paragraph's start line. Paragraphs with fewer than 20 content words are skipped — below that floor, a single homophone produces a misleading double-digit percentage. The diagnostic message names up to two example homophones the rule actually saw, so the location is the paragraph but the fix candidates are concrete.

The homophone tables (`HOMOPHONE_GROUPS_EN`, `HOMOPHONE_GROUPS_FR` in `src/language/`) lean toward content-word pairs whose orthographic confusion genuinely distorts meaning. Ultra-frequent French function-word homophones (`et` / `est`, `a` / `à`, `ou` / `où`) are intentionally excluded: they appear in nearly every sentence and would push baseline density past every threshold, drowning out the signal the rule is meant to catch.

When the document's detected language is `Unknown` the rule has no table to apply and skips silently rather than guessing.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_density_percent` | `float` | 8.0 | 5.0 | 3.0 |

Tune via `lucid-lint.toml`:

```toml
[rules."lexicon.homophone-density"]
max_density_percent = 4.0
```

## Examples

### English

**Before** (flagged):

> Their report shows there were too many decisions to make and two teams could not affect the launch nor lose the schedule despite careful planning across each region and product line every quarter.

What `lucid-lint check --profile public --experimental lexicon.homophone-density --conditions dyslexia` reports:

```text
warning input.md:1:1 Paragraph density of homophones is 21.2% (7 of 33 content words (e.g. their, there)); maximum 5.0%. Dense homophone runs raise the phonological-decoding load for dyslexic and aphasic readers; rephrase to disambiguate. [lexicon.homophone-density]
```

**After** (your rewrite):

> The report shows that the team made many decisions and that the two squads kept the launch on schedule despite careful planning across each region and product line every quarter.

The rephrase swaps `their` / `there` / `to` / `too` / `two` for context-anchored alternatives (`the report`, `that`, `the team`, `kept`, `the two squads`), bringing density well below the threshold.

### French

**Before** (flagged):

> Pendant le cours du matin la cuisinière prépare le foie de veau avant la pause de midi puis revient à sa tâche après avoir rangé les ustensiles sur la grande table en bois clair.

What `lucid-lint check --profile public --experimental lexicon.homophone-density --conditions dyslexia` reports:

```text
warning input.md:1:1 Paragraph density of homophones is 11.8% (4 of 34 content words (e.g. cours, foie)); maximum 5.0%. Dense homophone runs raise the phonological-decoding load for dyslexic and aphasic readers; rephrase to disambiguate. [lexicon.homophone-density]
```

**After** (your rewrite):

> Pendant la séance du matin la cuisinière prépare le foie de veau avant la coupure de midi puis reprend son travail après avoir rangé les ustensiles sur la grande table en bois clair.

`cours` becomes `séance`, `pause` becomes `coupure`, `tâche` becomes `travail` — three of the four homophone hits disappear without losing meaning.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md) for the inline and block forms. Inline disable also works on this rule:

```markdown
<!-- lucid-lint disable-next-line lexicon.homophone-density -->
Their report shows there were too many decisions to make and two teams could not lose the launch.
```

## See also

- [Conditions](../guide/conditions.md) — the `dyslexia` and `aphasia` tags that gate this rule.
- [F-experimental-rule-status — experimental rule status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrate that lets this rule ship in v0.2.x without affecting default scores.

## References

- [British Dyslexia Association — Dyslexia Style Guide (2018)](https://www.bdadyslexia.org.uk/advice/employers/creating-a-dyslexia-friendly-workplace/dyslexia-friendly-style-guide). Flags homophones as a friction point for dyslexic readers.
- [Falc — Information pour tous (2009)](../references.md). FALC orthographic-clarity guidelines for aphasic and plain-language audiences.

See [References](../references.md) for the full bibliography.
