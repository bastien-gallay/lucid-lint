# `rhythm.consecutive-long-sentences`

## What it flags

Streaks of long sentences within the same paragraph. An isolated long sentence is manageable; several in a row fatigue attention even when each individual sentence is under the [`structure.sentence-too-long`](./sentence-too-long.md) ceiling. This rule catches the *rhythm*.

## At a glance

| | |
|---|---|
| **Category** | `rhythm` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (identical detection) |
| **Source** | [`src/rules/consecutive_long_sentences.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/consecutive_long_sentences.rs) |

## Detection

Walk sentences sequentially inside each paragraph. Maintain a running count of consecutive sentences above `word_threshold`. Fire once per streak that reaches `max_consecutive`.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `word_threshold` | `int` | 20 | 15 | 10 |
| `max_consecutive` | `int` | 3 | 2 | 2 |

## Relation to `structure.sentence-too-long`

Both rules look at sentence length but catch different problems:

| Rule | Threshold (`dev-doc` / `public` / `falc`) | Fires on |
|---|---|---|
| [`structure.sentence-too-long`](./sentence-too-long.md) | `max_words` 30 / 22 / 15 | a single sentence past the ceiling |
| `rhythm.consecutive-long-sentences` | `word_threshold` 20 / 15 / 10 | a streak of `max_consecutive` sentences each above the lower threshold |

Because `word_threshold` sits below `max_words`, this rule catches the rhythm even when no individual sentence trips `sentence-too-long`. The invariant `word_threshold < max_words` (per profile) keeps the two from co-firing on the same sentence.

## Examples

Five ideas, colour-matched across the rewrite — only the rhythm changes. `lucid-lint` reports; the rewrite is always yours.

### English

**Before** (flagged):

> <span class="lucid-idea" data-idea="1">The migration introduced a caching layer that sits in front of every read from the primary database.</span> <span class="lucid-idea" data-idea="2">The team observed unexpected latency spikes whenever the cache invalidated under sustained write load.</span> <span class="lucid-idea" data-idea="3">A subsequent investigation traced the regression to a thundering-herd pattern that fired on every cold key.</span> <span class="lucid-idea" data-idea="4">The metrics dashboard misreported the issue as a generic timeout because the trace propagation was incomplete.</span> <span class="lucid-idea" data-idea="5">The fix coalesced concurrent fills, added jittered TTLs, and instrumented the cache layer with a dedicated span emitter.</span>

Five sentences, each over 20 words — the streak fatigues attention.

What `lucid-lint check --profile dev-doc` reports:

```text
warning input.md:1:1 5 consecutive sentences exceed 20 words (max 3). Vary sentence length or split the streak. [rhythm.consecutive-long-sentences]
```

**After** (your rewrite):

> <span class="lucid-idea" data-idea="1">The migration introduced a caching layer in front of the primary database.</span> <span class="lucid-idea" data-idea="2">Latency spiked whenever the cache invalidated under heavy writes.</span> <span class="lucid-idea" data-idea="3">The cause was a thundering-herd pattern on cold keys.</span> <span class="lucid-idea" data-idea="4">Metrics misreported it as a generic timeout — trace propagation was broken.</span> <span class="lucid-idea" data-idea="5">The fix coalesced concurrent fills, added jittered TTLs, and emitted a dedicated span.</span>

### French

**Before** (flagged):

> <span class="lucid-idea" data-idea="1">La migration a introduit une couche de cache qui se place devant chaque lecture de la base primaire.</span> <span class="lucid-idea" data-idea="2">L'équipe a observé des pics de latence inattendus chaque fois que le cache s'invalidait sous une charge d'écriture soutenue.</span> <span class="lucid-idea" data-idea="3">Une enquête ultérieure a relié la régression à un motif de troupeau tonnant qui se déclenchait sur chaque clé froide.</span> <span class="lucid-idea" data-idea="4">Le tableau de bord des métriques signalait à tort un délai d'attente générique parce que la propagation de la trace était incomplète.</span> <span class="lucid-idea" data-idea="5">Le correctif a fusionné les remplissages concurrents, ajouté un TTL avec gigue, et instrumenté la couche de cache avec un émetteur de span dédié.</span>

What `lucid-lint check --profile dev-doc` reports:

```text
warning input.md:1:1 5 consecutive sentences exceed 20 words (max 3). Vary sentence length or split the streak. [rhythm.consecutive-long-sentences]
```

**After** (your rewrite):

> <span class="lucid-idea" data-idea="1">La migration a introduit une couche de cache devant la base primaire.</span> <span class="lucid-idea" data-idea="2">La latence montait dès que le cache s'invalidait sous écritures soutenues.</span> <span class="lucid-idea" data-idea="3">Le coupable : un troupeau tonnant sur les clés froides.</span> <span class="lucid-idea" data-idea="4">Les métriques signalaient un délai générique — la trace était cassée.</span> <span class="lucid-idea" data-idea="5">Le correctif fusionne les remplissages, ajoute un TTL avec gigue et émet un span dédié.</span>

## Suppression

See [Suppressing diagnostics](../guide/suppression.md) for the inline and block forms.

## See also

- [`structure.sentence-too-long`](./sentence-too-long.md) — catches individual long sentences; this rule catches the streak even when each sentence is under that ceiling.
- [Scoring model](../guide/scoring.md) — `rhythm.consecutive-long-sentences` carries the default weight `1`; the cognitive cost is the cumulative streak, not any single sentence.
