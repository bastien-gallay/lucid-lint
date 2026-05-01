<!-- en-source-sha: 9e6be0c432915d364d877b302e70186928303088 -->
# `rhythm.consecutive-long-sentences`

*Phrases longues consécutives.*

## Ce que cette règle signale

Des séries de phrases longues à l'intérieur d'un même paragraphe. Une
phrase longue isolée reste gérable ; plusieurs d'affilée fatiguent
l'attention même si chaque phrase reste sous le plafond de
[`structure.sentence-too-long`](./sentence-too-long.md). Cette règle
capte le *rythme*.

## En bref

| | |
|---|---|
| **Catégorie** | `rhythm` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (détection identique) |
| **Source** | [`src/rules/consecutive_long_sentences.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/consecutive_long_sentences.rs) |

## Détection

Parcourir les phrases dans l'ordre à l'intérieur de chaque paragraphe.
Maintenir un compteur de phrases consécutives au-dessus de
`word_threshold`. Émettre un seul diagnostic par série atteignant
`max_consecutive`.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `word_threshold` | `int` | 20 | 15 | 10 |
| `max_consecutive` | `int` | 3 | 2 | 2 |

## Relation à `structure.sentence-too-long`

Les deux règles regardent la longueur des phrases mais signalent des
problèmes différents :

| Règle | Seuil (`dev-doc` / `public` / `falc`) | Se déclenche sur |
|---|---|---|
| [`structure.sentence-too-long`](./sentence-too-long.md) | `max_words` 30 / 22 / 15 | une phrase isolée au-delà du plafond |
| `rhythm.consecutive-long-sentences` | `word_threshold` 20 / 15 / 10 | une série de `max_consecutive` phrases chacune au-dessus du seuil inférieur |

Comme `word_threshold` reste sous `max_words`, cette règle capte le
rythme même quand aucune phrase isolée ne franchit
`sentence-too-long`. L'invariant `word_threshold < max_words` (par
profil) empêche les deux règles de se déclencher ensemble sur la
même phrase.

## Exemples

Cinq idées, teintes assorties d'un bout à l'autre de la réécriture —
seul le rythme change. `lucid-lint` signale ; la réécriture vous
appartient.

### Français

**Avant** (signalée) :

> <span class="lucid-idea" data-idea="1">La migration a introduit une couche de cache qui se place devant chaque lecture de la base de données primaire.</span> <span class="lucid-idea" data-idea="2">L'équipe a observé des pics de latence inattendus chaque fois que le cache s'invalidait sous une charge d'écriture soutenue.</span> <span class="lucid-idea" data-idea="3">Une enquête ultérieure a relié la régression à un effet *thundering-herd* qui se déclenchait sur chaque clé froide.</span> <span class="lucid-idea" data-idea="4">Le tableau de bord des métriques signalait à tort un délai d'attente générique parce que la propagation de la trace était incomplète.</span> <span class="lucid-idea" data-idea="5">Le correctif a fusionné les remplissages concurrents, randomisé les TTL, et instrumenté la couche de cache avec un émetteur de span dédié.</span>

Cinq phrases, chacune au-delà de 20 mots — la série fatigue l'attention.

Ce que `lucid-lint check --profile dev-doc` rapporte :

```text
warning input.md:1:1 5 consecutive sentences exceed 20 words (max 3). Vary sentence length or split the streak. [rhythm.consecutive-long-sentences]
```

**Après** (votre réécriture) :

> <span class="lucid-idea" data-idea="1">La migration a introduit une couche de cache devant la base de données primaire.</span> <span class="lucid-idea" data-idea="2">La latence montait dès que le cache s'invalidait sous écritures soutenues.</span> <span class="lucid-idea" data-idea="3">Le coupable : un *thundering-herd* sur les clés froides.</span> <span class="lucid-idea" data-idea="4">Les métriques signalaient un délai générique — la trace était cassée.</span> <span class="lucid-idea" data-idea="5">Le correctif fusionne les remplissages, randomise les TTL et émet un span dédié.</span>

### Anglais

**Avant** (signalée) :

> <span class="lucid-idea" data-idea="1">The migration introduced a caching layer that sits in front of every read from the primary database.</span> <span class="lucid-idea" data-idea="2">The team observed unexpected latency spikes whenever the cache invalidated under sustained write load.</span> <span class="lucid-idea" data-idea="3">A subsequent investigation traced the regression to a thundering-herd pattern that fired on every cold key.</span> <span class="lucid-idea" data-idea="4">The metrics dashboard misreported the issue as a generic timeout because the trace propagation was incomplete.</span> <span class="lucid-idea" data-idea="5">The fix coalesced concurrent fills, added jittered TTLs, and instrumented the cache layer with a dedicated span emitter.</span>

Ce que `lucid-lint check --profile dev-doc` rapporte :

```text
warning input.md:1:1 5 consecutive sentences exceed 20 words (max 3). Vary sentence length or split the streak. [rhythm.consecutive-long-sentences]
```

**Après** (votre réécriture) :

> <span class="lucid-idea" data-idea="1">The migration introduced a caching layer in front of the primary database.</span> <span class="lucid-idea" data-idea="2">Latency spiked whenever the cache invalidated under heavy writes.</span> <span class="lucid-idea" data-idea="3">The cause was a thundering-herd pattern on cold keys.</span> <span class="lucid-idea" data-idea="4">Metrics misreported it as a generic timeout — trace propagation was broken.</span> <span class="lucid-idea" data-idea="5">The fix coalesced concurrent fills, added jittered TTLs, and emitted a dedicated span.</span>

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant) pour les formes en ligne et par bloc.

## Voir aussi

- [`structure.sentence-too-long`](./sentence-too-long.md) — capte les phrases longues isolées ; cette règle capte la série même quand chaque phrase reste sous ce plafond.
- [Modèle de score](../../guide/scoring.md) — `rhythm.consecutive-long-sentences` porte le poids par défaut `1` ; le coût cognitif est cumulatif, pas par phrase.

## Références

- [Sweller (1988)](../references.md#sweller-1988)
- [Sweller, Ayres & Kalyuga (2011)](../references.md#sweller-2011)

Voir [Références](../references.md) pour la bibliographie complète.
