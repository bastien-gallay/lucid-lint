# `lexicon.low-lexical-diversity`

*Diversité lexicale faible.*

## Ce que cette règle signale

Les passages qui répètent excessivement leurs mots de contenu. Un
texte monotone perd l'attention du lecteur et trahit souvent une
pensée mal structurée. La règle n'est *pas* un anti-jargon : les
termes techniques (`API`, `requête`, `cache`) sont attendus comme
récurrents — le signal vise les mots de contenu non techniques.

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `info` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (listes de mots-outils distinctes) |
| **Source** | [`src/rules/low_lexical_diversity.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/low_lexical_diversity.rs) |

## Détection

Fenêtre glissante de `window_size` mots. Dans la fenêtre, on calcule
`mots_uniques / mots_totaux` sur les jetons hors mots-outils et hors
blocs de code. Le diagnostic se déclenche quand le ratio passe sous
`min_ratio`.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `window_size` | `int` | 100 | 100 | 80 |
| `min_ratio` | `float` | 0.40 | 0.50 | 0.55 |
| `use_stoplist` | `bool` | `true` | `true` | `true` |

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Références

- [Herdan (1960)](../references.md#herdan-1960)
- [McCarthy & Jarvis (2010)](../references.md#mccarthy-jarvis-2010)
- [Graesser et al. (2004)](../references.md#graesser-2004)

Voir [Références](../references.md) pour la bibliographie complète.
