# `structure.excessive-commas`

*Virgules en excès.*

## Ce que cette règle signale

Les phrases dont le nombre de virgules dépasse un plafond par profil.
La virgule est le marqueur le plus fréquent de complexité syntaxique ;
plutôt que de démêler la cause (subordination, apposition, énumération,
incise), la règle se sert de la densité comme indicateur avancé de
surcharge.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (détection identique) |
| **Source** | [`src/rules/excessive_commas.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/excessive_commas.rs) |

## Détection

Compter les virgules par phrase, signaler celles qui dépassent
`max_commas`.

**Interaction.** Quand [`structure.long-enumeration`](../../rules/long-enumeration.md)
se déclenche sur la même phrase, cette règle est neutralisée pour cette
phrase afin d'éviter un double signalement. Le détecteur d'énumération
partagé décompte également les virgules Oxford (3 items courts ou plus)
et les virgules à l'intérieur des listes de tokens parenthésées
`(A, B, C, …)` (3 segments courts ou plus séparés par des virgules
entre parenthèses équilibrées) — les deux décomptes sont agnostiques
à la langue.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_commas` | `int` | 4 | 3 | 2 |

## Faux positifs connus

Les listes de tokens parenthésées `(A, B, C, …)` sont désormais
décomptées (première tranche de F22). Les faux positifs restants
viennent surtout des listes séparées par virgules avec des items de
3 mots et plus (`as long as`, `as soon as`) et des listes non-Oxford
ou fermées par « plus » ; ils sont suivis sous **[F22](../../roadmap.md)**
dans la [feuille de route](../../roadmap.md) pour une tranche v0.3.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`structure.long-enumeration`](../../rules/long-enumeration.md)
- [`structure.deep-subordination`](../../rules/deep-subordination.md)
