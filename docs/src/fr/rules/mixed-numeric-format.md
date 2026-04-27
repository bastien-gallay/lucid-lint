# `structure.mixed-numeric-format`

*Formats numériques mixtes.*

## Ce que cette règle signale

Les phrases qui mêlent des numéraux en chiffres (`42`, `3.14`,
`1,000`, `1 000`) avec des numéraux écrits en toutes lettres (`two`,
`trois`, `twenty`, `cent`) au sein de la même phrase. Présenter les
nombres de manière incohérente force le lecteur à changer de forme
visuelle en cours de proposition et à ré-ancrer le référent — une
charge connue pour les lecteurs dyscalculiques et un anti-patron du
langage clair.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Mots-clés de condition** | `dyscalculia`, `general` |
| **Langues** | EN · FR |
| **Source** | [`src/rules/mixed_numeric_format.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/mixed_numeric_format.rs) |

## Détection

Pour chaque phrase produite par le tokenizer, balayage des tokens
chiffrés et des entrées de la liste des numéraux en lettres pour la
langue. Si au moins un de chaque type co-existe, un seul diagnostic
est émis pour la phrase, citant un token représentatif de chaque
type.

Les tokens chiffrés acceptent les chiffres ASCII plus un séparateur
décimal facultatif (`.`) ou de milliers (`,`, espace fine `U+0020`)
quand il est encadré de chiffres des deux côtés. Les correspondances
en toutes lettres sont des comparaisons ASCII insensibles à la casse
contre [`en::SPELLED_NUMERALS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/en.rs)
et [`fr::SPELLED_NUMERALS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/fr.rs).

Les formes ambiguës `one` (EN) et `un` / `une` (FR) sont **exclues**
de la liste des numéraux en lettres parce qu'elles servent aussi de
pronoms indéfinis et d'articles. Cela maintient un taux de faux
positifs gérable, au prix de manquer les vrais cas de format mixte
dont le seul numéral en lettres est `one` / `un` / `une`. Les formes
régionales (Suisse / Belgique : `septante`, `huitante`, `octante`,
`nonante`) ainsi que les formes métropolitaines sont incluses.

Les phrases sont produites par le tokenizer partagé
(voir [`src/parser/tokenizer.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/parser/tokenizer.rs)),
afin que les abréviations, décimales et points de suspension ne
fragmentent pas indûment les phrases. Les blocs de code (clôturés ou
indentés) sont exclus en amont par le parseur Markdown.

## Paramètres

Aucun. La règle n'a pas de seuil configurable — une seule
co-occurrence des deux formes suffit.

## Mises en garde connues

- Les phrases dont le seul numéral en lettres est `one` / `un` /
  `une` ne sont pas signalées, par construction (voir *Détection*).
- Les ordinaux (`first`, `premier`, `2nd`, `3e`) sont hors périmètre.
  `2nd` se lit actuellement comme un token chiffré (`2`) suivi d'un
  mot (`nd`), ce qui ne correspond pas à la liste des numéraux en
  lettres — pas de faux positif.
- Les chiffres romains (`IV`, `XIV`) ne sont ni des chiffres ni des
  numéraux en lettres pour cette règle.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`readability.score`](../../rules/readability-score.md)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)
