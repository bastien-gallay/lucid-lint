# `syntax.dense-punctuation-burst`

*Rafale de ponctuation.*

## Ce que cette règle signale

<!-- lucid-lint disable-next-line syntax.dense-punctuation-burst -->

Des rafales *locales* de ponctuation : une fenêtre glissante de
graphèmes qui contient trop de signes qualifiants (`,`, `;`, `:`,
`—`, `–`). Les amas serrés de signes indiquent une subordination
empilée, des incises parenthétiques ou des listes dans des listes.
Ce sont des constructions difficiles à analyser pour les lecteurs
souffrant de troubles cognitifs ou attentionnels (lignes directrices
IFLA pour les textes faciles à lire).

À distinguer de [`structure.excessive-commas`](./excessive-commas.md),
qui compte les virgules sur une phrase entière. Une phrase avec
8 virgules réparties sur 200 caractères ne déclenche pas ici, alors
qu'une phrase avec 3 virgules dans 30 caractères déclenche.

## En bref

| | |
|---|---|
| **Catégorie** | `syntax` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Étiquettes de condition** | `general` |
| **Langues** | EN · FR (agnostique au script) |
| **Source** | [`src/rules/dense_punctuation_burst.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/dense_punctuation_burst.rs) |

## Détection

Par ligne source, on parcourt le flux de graphèmes une fois et on
recense la colonne de chaque signe qualifiant. Quand une fenêtre de
`window_graphemes` graphèmes contient `min_marks` signes ou plus, on
émet une rafale qui couvre du premier au dernier signe de la fenêtre.
Puis on avance au-delà de ce dernier signe pour éviter que les
fenêtres recouvrantes ne tirent deux fois sur le même amas.

Les blocs de code (fencés et indentés) sont exclus en amont par
l'analyseur Markdown. Les terminateurs de phrase (`.`, `!`, `?`) et
les parenthèses ne comptent pas dans la rafale.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_marks` | `int` | 4 | 3 | 3 |
| `window_graphemes` | `int` | 30 | 30 | 40 |

`dev-doc` tolère un amas de 3 signes — typique des listes techniques
au contact de la prose. FALC garde le même seuil de densité que
`public` mais élargit la fenêtre pour attraper des rafales un peu
plus lâches.

## Cas connus

- La règle opère par ligne source. Une rafale qui chevauche un saut
  de ligne dur en source n'est pas détectée ; en pratique c'est rare,
  car la ponctuation dense est aussi dense en octets source.
- Le tiret cadratin (`—`, U+2014) et le tiret demi-cadratin (`–`,
  U+2013) qualifient ; le succédané ASCII à double trait (`--`) non,
  sous l'hypothèse que les auteurs soucieux de lisibilité utilisent
  les bonnes formes Unicode.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`structure.excessive-commas`](./excessive-commas.md)
- [`structure.sentence-too-long`](./sentence-too-long.md)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)

## Références

- [Sweller (1988)](../references.md#sweller-1988)
- [Gibson (1998)](../references.md#gibson-1998)

Voir [Références](../references.md) pour la bibliographie complète.
