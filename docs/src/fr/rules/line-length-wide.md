<!-- en-source-sha: e3d59a57e55ff6172e68e297a7057b2c8d54dbe4 -->
# `structure.line-length-wide`

*Lignes trop larges.*

## Ce que cette règle signale

Les lignes choisies par l'auteur plus larges que le plafond du profil. WCAG 1.4.8 (AAA) plafonne le texte rendu à environ 80 caractères par ligne, car des lignes plus longues forcent l'œil à parcourir plus de distance entre saccades et augmentent la relecture au retour à la ligne — une difficulté connue chez les lecteurs dyslexiques (BDA Dyslexia Style Guide).

« Choisies par l'auteur » est important. En Markdown, les sauts mous sont remplacés par des espaces lors de l'analyse, parce que le rendu réorganise le texte selon la largeur de l'écran. La largeur de la ligne source ne dit donc rien de ce que voit le lecteur. Cette règle ne mesure que les sauts gardés volontairement : sauts durs Markdown (`<br>` ou deux espaces en fin de ligne) et retours à la ligne explicites en texte brut. Un paragraphe Markdown soft-wrappé est exempté, peu importe la longueur de son texte joint. Pour borner la densité d'un paragraphe, voir [`structure.paragraph-too-long`](./paragraph-too-long.md).

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

Pour chaque paragraphe qui contient un saut de ligne voulu par l'auteur, mesure de la largeur de chaque ligne en clusters de graphèmes ; signalement des lignes au-delà de `max_line_length`.

Un paragraphe Markdown sans saut dur (le cas courant en prose) est exempté. Les sauts mous sont remplacés par des espaces lors de l'analyse : ce qui reste est une ligne logique dont la longueur source suit la largeur de l'éditeur, pas le rendu visé par WCAG 1.4.8. Le texte brut suit la même logique : un paragraphe sans `\n` interne est exempté ; un paragraphe avec retours à la ligne internes est mesuré ligne par ligne.

Les blocs de code (clôturés ou indentés) sont exclus en amont par le parseur Markdown. Les titres, items de liste et cellules de tableau sont hors scope par construction — `paragraph-too-long`, `sentence-too-long` et les règles de titres couvrent les charges cognitives qui s'appliquent à ces blocs.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_line_length` | `int` | 120 | 100 | 80 |

Le profil FALC s'aligne sur la recommandation AAA WCAG 1.4.8 de 80
caractères.

## Mises en garde connues

Les paragraphes de prose en une seule ligne source sont exemptés volontairement. La règle se déclenchait dessus auparavant et générait beaucoup de bruit sur de la prose réelle ; v0.2.x la restreint aux sauts choisis par l'auteur. À combiner avec [`structure.paragraph-too-long`](./paragraph-too-long.md) si tu veux aussi un plafond sur la longueur jointe du paragraphe.

Les titres et items de liste ne sont pas mesurés par cette règle. Leur largeur de retour dépend du rendu (corps des titres, indentation des listes), et les charges cognitives sous-jacentes sont déjà couvertes par d'autres règles.

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
