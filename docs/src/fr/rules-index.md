# Référence des règles

`lucid-lint` livre 25 règles en v0.2 (17 reprises de v0.1, 8 ajouts
v0.2). Chaque règle dispose d'une page dédiée avec sa catégorie, sa
sévérité, son poids par défaut, ses seuils par profil, des exemples,
et les consignes de neutralisation.

<!-- lucid-lint disable-next-line unexplained-abbreviation -->

La référence compacte [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md)
reste la vue d'ensemble en un seul fichier, conservée à la racine du
dépôt.

> **En anglais pour l'instant.** Les pages de règles individuelles
> sont encore en anglais. Les liens du tableau ci-dessous pointent
> vers la version anglaise ; la traduction FR de chaque règle est
> suivie dans **F25** sur la [feuille de route](../roadmap.md).

## Catégories

Chaque règle appartient à exactement une des cinq catégories fixes.
La taxonomie fait autorité — le [modèle de score](../guide/scoring.md)
compose les sous-scores par catégorie dans le score global `X / max`.

| Catégorie | Règles |
|---|---|
| **`structure`** | [`sentence-too-long`](../rules/sentence-too-long.md) · [`paragraph-too-long`](../rules/paragraph-too-long.md) · [`heading-jump`](../rules/heading-jump.md) · [`deeply-nested-lists`](../rules/deeply-nested-lists.md) · [`excessive-commas`](../rules/excessive-commas.md) · [`long-enumeration`](../rules/long-enumeration.md) · [`deep-subordination`](../rules/deep-subordination.md) · [`line-length-wide`](../rules/line-length-wide.md) · [`mixed-numeric-format`](../rules/mixed-numeric-format.md) |
| **`rhythm`** | [`consecutive-long-sentences`](../rules/consecutive-long-sentences.md) · [`repetitive-connectors`](../rules/repetitive-connectors.md) |
| **`lexicon`** | [`low-lexical-diversity`](../rules/low-lexical-diversity.md) · [`excessive-nominalization`](../rules/excessive-nominalization.md) · [`unexplained-abbreviation`](../rules/unexplained-abbreviation.md) · [`weasel-words`](../rules/weasel-words.md) · [`jargon-undefined`](../rules/jargon-undefined.md) · [`all-caps-shouting`](../rules/all-caps-shouting.md) · [`redundant-intensifier`](../rules/redundant-intensifier.md) · [`consonant-cluster`](../rules/consonant-cluster.md) |
| **`syntax`** | [`passive-voice`](../rules/passive-voice.md) · [`unclear-antecedent`](../rules/unclear-antecedent.md) · [`nested-negation`](../rules/nested-negation.md) · [`conditional-stacking`](../rules/conditional-stacking.md) · [`dense-punctuation-burst`](../rules/dense-punctuation-burst.md) |
| **`readability`** | [`readability-score`](../rules/readability-score.md) |

> **Source d'autorité.** La catégorie de chaque règle est déterminée
> par `Category::for_rule` dans `src/types.rs`. Le tableau ci-dessus
> reflète cette fonction. Un test de couverture
> (`tests/rule_docs_coverage.rs`) tient les pages par règle, le
> helper de catégorie et les poids du score synchronisés.

## Niveaux de sévérité

| Niveau | Sens | Effet |
|---|---|---|
| `info` | Signal à connaître, pas un défaut | Remonté ; ne fait pas échouer la CI |
| `warning` | Problème de qualité à corriger | Remonté ; peut faire échouer la CI selon `--min-score` |
| `error` | Réservé pour v0.3+ | Non émis en v0.2 |

## Proposer une règle

Voir [Contributing](../contributing.md) pour la checklist
d'ajout de règle — toute nouvelle règle doit être livrée avec une
page dans cette section.
