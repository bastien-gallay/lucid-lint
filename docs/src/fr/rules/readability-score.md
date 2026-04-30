# `readability.score`

*Score de lisibilité.*

## Ce que cette règle signale

Un indice de lisibilité au niveau du document. Les formules de
lisibilité sont le signal synthétique historique de la complexité
textuelle — simples, reproductibles, reconnues par les guides
gouvernementaux US/UK et par WCAG. À traiter comme la complexité
cyclomatique : d'abord une métrique, ensuite un avertissement.

## En bref

| | |
|---|---|
| **Catégorie** | `readability` |
| **Sévérité par défaut** | `info` (toujours signalée) · `warning` quand au-dessus de `max_grade_level` |
| **Poids par défaut** | `5` |
| **Langues** | EN — Flesch-Kincaid · FR — Kandel-Moles (auto-sélection selon la langue détectée ; v0.2+) |
| **Source** | [`src/rules/readability_score.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/readability_score.rs) |

## Détection (v0.2 — formule par langue)

La formule est sélectionnée selon la langue détectée du document :

**Anglais — Flesch-Kincaid Grade Level :**

```
0.39 × (mots / phrases) + 11.8 × (syllabes / mots) − 15.59
```

Le résultat est un niveau scolaire américain. Comparé directement à
`max_grade_level`.

**Français — Kandel & Moles (1958) :**

```
207 − 1.015 × (mots / phrases) − 73.6 × (syllabes / mots)
```

Le résultat est un score d'aisance, typiquement dans `0..100` (plus
haut = plus facile), à la Flesch. Pour rester comparable d'une
langue à l'autre, la règle le convertit en équivalent niveau scolaire
avec l'approximation linéaire standard `(100 − score) / 10`, et
compare ce niveau à `max_grade_level`. Le message de diagnostic
remonte à la fois le score d'aisance natif et l'équivalent niveau
scolaire.

**Langue inconnue** : repli sur Flesch-Kincaid.

| Niveau | Équivalent scolaire (FR) |
|---|---|
| < 6 | Primaire |
| 6–9 | Collège |
| 9–12 | Lycée |
| 12–16 | Études supérieures |
| > 16 | Expert |

L'option `--readability-formula` (livrée avec F11 en v0.2) fige une
formule indépendamment de la langue détectée :
`--readability-formula flesch-kincaid` ou
`--readability-formula kandel-moles`. La valeur par défaut `auto`
garde le comportement par langue. D'autres formules (`Gunning Fog`,
`SMOG`, `Dale-Chall`, `Scolarius`) et un rapport multi-formules
`--readability-verbose` restent sur la
[feuille de route](../roadmap.md).

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_grade_level` | `float` | 14 | 9 | 6 |
| `always_report` | `bool` | `true` | `true` | `true` |
| `formula` | `auto` \| `flesch-kincaid` \| `kandel-moles` | `auto` | `auto` | `auto` |

`formula` peut être surchargée via `--readability-formula` en CLI ;
`auto` suit la langue détectée, les autres valeurs figent la formule.

## Modes de sortie

- Toujours signalé en `info` (pour l'observabilité, même sous le
  seuil).
- Signalé en `warning` quand le niveau dépasse `max_grade_level`.

## Neutralisation

Neutraliser une métrique au niveau du document est rarement la bonne
réponse ; ajuster `max_grade_level` dans `lucid-lint.toml` à la
place. Voir [Configuration](../../guide/configuration.md) (page EN
pour l'instant).

## Références

- [Flesch (1948)](../references.md#flesch-1948)
- [Kincaid et al. (1975)](../references.md#kincaid-1975)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

Voir [Références](../references.md) pour la bibliographie complète.
