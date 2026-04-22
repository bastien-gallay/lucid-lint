# `structure.sentence-too-long`

## What it flags

Sentences whose length exceeds a per-profile ceiling. The intrinsic cognitive load of a sentence grows non-linearly with its word count (Graesser et al. 2004, *Coh-Metrix*); FALC caps at 15 words, Plain English at 20. Long sentences increase the probability of a reader under attentional load losing the thread mid-read.

## At a glance

| | |
|---|---|
| **Category** | `structure` |
| **Default severity** | `warning` |
| **Default weight** | `2` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/sentence_too_long.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/sentence_too_long.rs) |

## Detection

Split text into sentences via strong punctuation (`.`, `!`, `?`, `…`, paragraph breaks). Count Unicode word tokens, excluding punctuation. Contractions (`don't`) and elisions (`l'accessibilité`) count as one word when the apostrophe sits between two letters. Code blocks are skipped.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_words` | `int` | 30 | 22 | 15 |
| `exclude_code_blocks` | `bool` | `true` | `true` | `true` |

## Examples

**Before** (EN, flagged):

> The caching subsystem, which was introduced in an earlier milestone, turned out to interact poorly with the new request pipeline under sustained load, and the investigation that followed required multiple rounds of profiling.

**After:**

> The caching subsystem was introduced earlier. It interacts poorly with the new request pipeline under sustained load. The investigation required several rounds of profiling.

**Before** (FR, flagged):

> Le sous-système de cache introduit lors d'un jalon précédent interagit mal avec le nouveau pipeline de requêtes sous charge soutenue, et l'enquête a nécessité plusieurs rondes de profilage.

**After:**

> Le cache a été introduit lors d'un jalon précédent. Il interagit mal avec le nouveau pipeline sous charge soutenue. L'enquête a nécessité plusieurs rondes de profilage.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md) for the inline and block forms.

## See also

- [`rhythm.consecutive-long-sentences`](./consecutive-long-sentences.md) — catches rhythm; its threshold must stay lower than `max_words` here.
- [Scoring model](../guide/scoring.md) — `structure.sentence-too-long` carries weight `2` because the cognitive cost compounds with length.
