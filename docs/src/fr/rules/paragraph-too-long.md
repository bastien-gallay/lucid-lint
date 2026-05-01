<!-- en-source-sha: 9594202969efbf001af153da9d37f18bb0786e18 -->
# `structure.paragraph-too-long`

*Paragraphe trop long.*

## Ce que cette règle signale

Les paragraphes qui dépassent un seuil en nombre de phrases ou en
nombre de mots. Le paragraphe est l'unité visuelle de reprise : un
paragraphe trop long dilue ce point de reprise pour les lecteurs qui
s'interrompent souvent. Les deux mesures sont vérifiées afin qu'un
paragraphe court mais dense (une seule phrase de 80 mots) soit aussi
attrapé — [`structure.sentence-too-long`](./sentence-too-long.md)
couvre le cas complémentaire.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `2` |
| **Langues** | EN · FR (détection identique) |
| **Source** | [`src/rules/paragraph_too_long.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/paragraph_too_long.rs) |

## Détection

Découpage sur les lignes vides (convention Markdown du paragraphe).
Comptage des phrases et des mots par paragraphe. Signalement des
paragraphes dépassant *l'un ou l'autre* des seuils.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_sentences` | `int` | 7 | 5 | 3 |
| `max_words` | `int` | 150 | 100 | 60 |

## Exemples

Un paragraphe de huit phrases moyennes sous le profil `public` se
déclenchera sur `max_sentences`. Un paragraphe contenant une seule
phrase de 120 mots se déclenchera sur `max_words` (et également sur
[`structure.sentence-too-long`](./sentence-too-long.md)).

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`structure.sentence-too-long`](./sentence-too-long.md)
- [`rhythm.consecutive-long-sentences`](../../rules/consecutive-long-sentences.md)

## Références

- [Sweller (1988)](../references.md#sweller-1988)
- [Graesser et al. (2004)](../references.md#graesser-2004)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

Voir [Références](../references.md) pour la bibliographie complète.
