# `structure.line-length-wide`

*Lignes trop larges.*

## Ce que cette règle signale

Les lignes source plus larges que le plafond du profil. WCAG 1.4.8
(AAA) plafonne le texte rendu à environ 80 caractères par ligne, car
des lignes plus longues forcent l'œil à parcourir plus de distance
entre saccades et augmentent la relecture au retour à la ligne — une
difficulté connue chez les lecteurs dyslexiques (BDA Dyslexia Style
Guide).

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Mots-clés de condition** | `dyslexia`, `general` |
| **Langues** | EN · FR (indépendant de l'écriture) |
| **Source** | [`src/rules/line_length_wide.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/line_length_wide.rs) |

## Détection

Pour chaque paragraphe, mesure de la largeur de chaque ligne en
clusters de graphèmes ; signalement des lignes au-delà de
`max_line_length`.

Les blocs de code (clôturés ou indentés) sont exclus en amont par le
parseur Markdown.

`paragraph.text` conserve les sauts durs depuis la source. En
Markdown, les sauts mous sont reformés en espaces, ce qui signifie
qu'un paragraphe-source enveloppé selon l'intention de l'auteur
compte comme une seule ligne reformée pour cette règle. Les entrées
texte brut et stdin mesurent directement les lignes-source.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_line_length` | `int` | 120 | 100 | 80 |

Le profil FALC s'aligne sur la recommandation AAA WCAG 1.4.8 de 80
caractères.

## Mises en garde connues

Les sauts mous Markdown dans la source sont reformés en espaces
durant l'analyse ; la règle voit donc le texte du paragraphe une fois
*reformé*. Un paragraphe Markdown dont chaque ligne-source fait moins
de 80 caractères mais dont le texte reformé fait 400 caractères se
déclenchera quand même. Pour un contrôle strict de la largeur de
retour, lintez la sortie rendue ou utilisez une entrée texte brut.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`structure.paragraph-too-long`](./paragraph-too-long.md)
- [`structure.sentence-too-long`](./sentence-too-long.md)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)

## Références

- [WCAG 2.1 — 1.4.8 (AAA)](../references.md#wcag-2-1)
- [Legge & Bigelow (2011)](../references.md#legge-bigelow-2011)

Voir [Références](../references.md) pour la bibliographie complète.
