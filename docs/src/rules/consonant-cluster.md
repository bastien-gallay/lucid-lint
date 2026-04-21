# `consonant-cluster`

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Condition tags** | `dyslexia`, `general` |
| **Languages** | EN · FR |
| **Source** | [`src/rules/consonant_cluster.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/consonant_cluster.rs) |

## What it flags

Words whose longest run of consecutive consonants meets or exceeds a per-profile threshold. Dense consonant clusters are a known decoding barrier for dyslexic readers (BDA Dyslexia Style Guide): the reader must hold more phonemes in working memory before the next vowel "releases" the syllable.

Typical English offenders at the `public` threshold of 5 include `strengths` (n-g-t-h-s), `twelfths` (l-f-t-h-s), `sixths` (x-t-h-s in a 4-run plus context). Typical French offenders at the `falc` threshold of 4 include `constructions` (n-s-t-r).

## Detection

Per source line, walk the grapheme stream once. A word is a maximal run of alphabetic characters; hyphens, apostrophes, and whitespace close the word (so `dys-lexic` is two words, not one ten-letter cluster). Within a word, track the longest run of consecutive consonants. Emit one diagnostic per word whose longest run meets `min_run_length`.

Vowels are language-aware — French accented forms (`é`, `è`, `ê`, `à`, `â`, `î`, `ï`, `ô`, `ö`, `ù`, `û`, `ü`, `ÿ`, `œ`, `æ`) count as vowels. The English fallback still accepts common latin-1 accented vowels so borrowed words (`café`, `naïve`) decode correctly. `y` is treated as a vowel in every language (lenient), which avoids awkward false positives on words like `fly`, `rhythm`.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_run_length` | `int` | 6 | 5 | 4 |

`dev-doc` is tolerant — technical prose regularly names things like `strengths` and `benchmarks`. `falc` (plain-language audience) catches any 4-consonant run.

## Known caveats

- The rule is blind to syllable structure: it counts raw consonant graphemes, not phonemes. A word like `hatching` (5 letters: t-c-h-n-g — a run of 5) reads fluently to most readers because `tch` is a single English digraph. Suppress with an inline directive when a hit is unavoidable.
- Script-agnostic for any alphabetic script, but the vowel lists are tuned for Latin scripts only. Words in Cyrillic, Greek, Arabic, etc., will likely trigger whenever the language flag is `en` or `fr` — in practice such content is out of scope for a bilingual EN/FR linter.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`all-caps-shouting`](./all-caps-shouting.md)
- [`readability-score`](./readability-score.md)
- [Conditions](../guide/conditions.md)
