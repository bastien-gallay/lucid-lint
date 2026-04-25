# `structure.deeply-nested-lists`

## Ce que cette règle signale

Les items de liste à puces imbriqués au-delà d'une profondeur
raisonnable. Une liste profondément imbriquée force le lecteur à
reconstruire une hiérarchie mentale complexe — l'indentation
horizontale cesse d'être un indice positionnel et devient du bruit.
Quatre niveaux d'indentation, c'est trop pour des lecteurs avec des
difficultés attentionnelles.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | indépendant de la langue |
| **Source** | [`src/rules/deeply_nested_lists.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/deeply_nested_lists.rs) |

## Détection

Analyse Markdown via `pulldown-cmark` ; extraction des items de liste
avec leur niveau d'indentation ; signalement des items au-delà de
`max_depth`. Déterministe, pas de faux positifs.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_depth` | `int` | 4 | 3 | 2 |

## Exemple

Sous le profil `public` (profondeur max 3) :

```markdown
- Niveau 1
  - Niveau 2
    - Niveau 3
      - Niveau 4    ← signalé
```

## Message de diagnostic

Inclut un guide de réparation : aplatir la structure, scinder en
plusieurs listes, ou promouvoir les sous-items en sous-sections avec
des titres.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).
