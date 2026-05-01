<!-- en-source-sha: 9594202969efbf001af153da9d37f18bb0786e18 -->
# `structure.heading-jump`

*Saut de niveau de titre.*

## Ce que cette règle signale

Les sauts de niveau de titre qui cassent la carte mentale du document
(par exemple H2 → H4). Chaque niveau doit suivre le précédent d'au
plus +1. Les lecteurs avec des difficultés attentionnelles s'appuient
fortement sur la hiérarchie des titres pour se repositionner après
une interruption ; une hiérarchie cassée détruit cet indice. Signale
aussi le tout premier titre s'il est plus profond que H2 quand
`allow_first_heading_any_level` vaut `false`, ainsi que l'absence de
H1 quand `require_h1` vaut `true`.

**Références.** WCAG 2.1 SC 1.3.1 (Information et relations) et 2.4.6
(En-têtes et étiquettes) ; RGAA 9.1.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | indépendant de la langue |
| **Source** | [`src/rules/heading_jump.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/heading_jump.rs) |

## Détection

Analyse des titres Markdown (`#`, `##`, …). Parcours dans l'ordre
source ; signalement de chaque titre dont le niveau dépasse le
précédent de plus d'un. Déterministe, pas de faux positifs.

## Paramètres

| Clé | Type | Défaut |
|---|---|---|
| `allow_first_heading_any_level` | `bool` | `true` |
| `require_h1` | `bool` | `false` |

Règle binaire — pas de seuils par profil.

## Exemples

Signalé :

```markdown
# Vue d'ensemble
#### Détails    ← saut de H1 à H4
```

Propre :

```markdown
# Vue d'ensemble
## Section
### Sous-section
```

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`structure.deeply-nested-lists`](./deeply-nested-lists.md) — le
  signal équivalent au niveau des listes.

## Références

- [WCAG 2.1 — 1.3.1 & 2.4.6](../references.md#wcag-2-1)

Voir [Références](../references.md) pour la bibliographie complète.
