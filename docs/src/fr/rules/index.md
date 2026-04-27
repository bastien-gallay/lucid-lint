# Référence des règles

`lucid-lint` livre 25 règles en v0.2 (17 reprises de v0.1, 8 ajouts
v0.2). Chaque règle dispose d'une page dédiée avec sa catégorie, sa
sévérité, son poids par défaut, ses seuils par profil, des exemples,
et les consignes de neutralisation.

<!-- lucid-lint disable-next-line lexicon.unexplained-abbreviation -->

La référence compacte [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md)
reste la vue d'ensemble en un seul fichier, conservée à la racine du
dépôt.

> **Traduction FR en cours.** Seules quelques règles ont leur page
> dédiée en français pour l'instant ; les autres liens du tableau
> ci-dessous pointent vers la version anglaise. La progression est
> suivie dans **F25** sur la [feuille de route](../roadmap.md).

## Catégories

Chaque règle appartient à exactement une des cinq catégories fixes.
La taxonomie fait autorité — le [modèle de score](../../guide/scoring.md)
compose les sous-scores par catégorie dans le score global `X / max`.

L'identifiant en `kebab-case` (par ex. `structure.sentence-too-long`)
est le contrat stable utilisé partout : option CLI, sortie JSON, clé
de configuration, citation dans les docs. Le **libellé FR** ci-dessous
est un repère humain ; il n'aliase jamais l'identifiant.

### Structure

| Règle | Libellé |
|---|---|
| [`structure.sentence-too-long`](./sentence-too-long.md) | Phrase trop longue |
| [`structure.paragraph-too-long`](./paragraph-too-long.md) | Paragraphe trop long |
| [`structure.heading-jump`](./heading-jump.md) | Saut de niveau de titre |
| [`structure.deeply-nested-lists`](./deeply-nested-lists.md) | Listes trop imbriquées |
| [`structure.excessive-commas`](./excessive-commas.md) | Virgules en excès |
| [`structure.long-enumeration`](./long-enumeration.md) | Énumération trop longue |
| [`structure.deep-subordination`](./deep-subordination.md) | Subordination profonde |
| [`structure.line-length-wide`](./line-length-wide.md) | Lignes trop larges |
| [`structure.mixed-numeric-format`](./mixed-numeric-format.md) | Formats numériques mixtes |

### Rythme

| Règle | Libellé |
|---|---|
| [`rhythm.consecutive-long-sentences`](./consecutive-long-sentences.md) | Phrases longues consécutives |
| [`rhythm.repetitive-connectors`](./repetitive-connectors.md) | Répétition de connecteurs |

### Lexique

| Règle | Libellé |
|---|---|
| [`lexicon.low-lexical-diversity`](../../rules/low-lexical-diversity.md) (en) | Diversité lexicale faible |
| [`lexicon.excessive-nominalization`](../../rules/excessive-nominalization.md) (en) | Nominalisations en excès |
| [`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md) | Abréviations non explicitées |
| [`lexicon.weasel-words`](./weasel-words.md) | Mots évasifs |
| [`lexicon.jargon-undefined`](../../rules/jargon-undefined.md) (en) | Jargon non défini |
| [`lexicon.all-caps-shouting`](../../rules/all-caps-shouting.md) (en) | Majuscules criardes |
| [`lexicon.redundant-intensifier`](../../rules/redundant-intensifier.md) (en) | Intensificateurs redondants |
| [`lexicon.consonant-cluster`](../../rules/consonant-cluster.md) (en) | Amas consonantiques |

### Syntaxe

| Règle | Libellé |
|---|---|
| [`syntax.passive-voice`](../../rules/passive-voice.md) (en) | Voix passive |
| [`syntax.unclear-antecedent`](../../rules/unclear-antecedent.md) (en) | Antécédent flou |
| [`syntax.nested-negation`](../../rules/nested-negation.md) (en) | Négations imbriquées |
| [`syntax.conditional-stacking`](../../rules/conditional-stacking.md) (en) | Empilement de conditions |
| [`syntax.dense-punctuation-burst`](../../rules/dense-punctuation-burst.md) (en) | Rafale de ponctuation |

### Lisibilité

| Règle | Libellé |
|---|---|
| [`readability.score`](../../rules/readability-score.md) (en) | Score de lisibilité |

> **Source d'autorité.** La catégorie de chaque règle est déterminée
> par `Category::for_rule` dans `src/types.rs`. Les tableaux ci-dessus
> reflètent cette fonction. Un test de couverture
> (`tests/rule_docs_coverage.rs`) tient les pages par règle, le
> helper de catégorie et les poids du score synchronisés.

## Niveaux de sévérité

| Niveau | Sens | Effet |
|---|---|---|
| `info` | Signal à connaître, pas un défaut | Remonté ; ne fait pas échouer la CI |
| `warning` | Problème de qualité à corriger | Remonté ; peut faire échouer la CI selon `--min-score` |
| `error` | Réservé pour v0.3+ | Non émis en v0.2 |

## Proposer une règle

Voir [Contributing](../../contributing.md) pour la checklist
d'ajout de règle — toute nouvelle règle doit être livrée avec une
page dans cette section.
