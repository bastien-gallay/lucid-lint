<!-- en-source-sha: 5e24f614b0378a6ff57d9f497d052361e2cfcec3 -->
# `structure.sentence-too-long`

*Phrase trop longue.*

## Ce que cette règle signale

Les phrases dont la longueur dépasse un plafond par profil. La charge
cognitive intrinsèque d'une phrase croît de façon non linéaire avec
son nombre de mots (Graesser et al. 2004, *Coh-Metrix*) ; le FALC
plafonne à 15 mots, le Plain English à 20. Les phrases longues
augmentent la probabilité qu'un lecteur à l'attention fragilisée
perde le fil en cours de lecture.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `2` |
| **Langues** | EN · FR (détection identique) |
| **Source** | [`src/rules/sentence_too_long.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/sentence_too_long.rs) |

## Détection

Le texte est découpé en phrases via la ponctuation forte (`.`, `!`,
`?`, `…`, sauts de paragraphe). Les tokens mots Unicode sont comptés
en excluant la ponctuation. Les contractions (`don't`) et élisions
(`l'accessibilité`) comptent pour un seul mot quand l'apostrophe est
entourée de deux lettres. Les blocs de code sont ignorés.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_words` | `int` | 30 | 22 | 15 |
| `exclude_code_blocks` | `bool` | `true` | `true` | `true` |

## Exemples

Trois idées, teintes assorties d'un bout à l'autre de la réécriture —
la position les appariait déjà, la couleur confirme que la réécriture
n'en perd aucune.

**Avant** (FR, signalée) :

> <span class="lucid-idea" data-idea="1">Le sous-système de cache introduit lors d'un jalon précédent</span> <span class="lucid-idea" data-idea="2">interagit mal avec le nouveau pipeline de requêtes sous charge soutenue,</span> et <span class="lucid-idea" data-idea="3">l'enquête a nécessité plusieurs rondes de profilage.</span>

**Après :**

> <span class="lucid-idea" data-idea="1">Le cache a été introduit lors d'un jalon précédent.</span> <span class="lucid-idea" data-idea="2">Il interagit mal avec le nouveau pipeline sous charge soutenue.</span> <span class="lucid-idea" data-idea="3">L'enquête a nécessité plusieurs rondes de profilage.</span>

**Avant** (EN, signalée) :

> <span class="lucid-idea" data-idea="1">The caching subsystem, which was introduced in an earlier milestone,</span> <span class="lucid-idea" data-idea="2">turned out to interact poorly with the new request pipeline under sustained load,</span> and <span class="lucid-idea" data-idea="3">the investigation that followed required multiple rounds of profiling.</span>

**Après :**

> <span class="lucid-idea" data-idea="1">The caching subsystem was introduced earlier.</span> <span class="lucid-idea" data-idea="2">It interacts poorly with the new request pipeline under sustained load.</span> <span class="lucid-idea" data-idea="3">The investigation required several rounds of profiling.</span>

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant) pour les formes en ligne et par bloc.

## Voir aussi

- [`rhythm.consecutive-long-sentences`](../../rules/consecutive-long-sentences.md) — capture le rythme ; son seuil doit rester inférieur au `max_words` d'ici.
- [Modèle de score](../../guide/scoring.md) — `structure.sentence-too-long` porte un poids de `2` parce que le coût cognitif se compose avec la longueur.

## Références

- [Sweller (1988)](../references.md#sweller-1988)
- [Plain Language US (2011)](../references.md#plain-language-us-2011)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

Voir [Références](../references.md) pour la bibliographie complète.
