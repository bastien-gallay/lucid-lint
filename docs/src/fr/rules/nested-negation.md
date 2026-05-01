<!-- en-source-sha: 69f4624777150fb219be85d99459d3e6ac107015 -->
# `syntax.nested-negation`

*Négations imbriquées.*

## Ce que cette règle signale

Les phrases qui empilent plusieurs négations. Deux négations ou plus
dans une même phrase forcent le lecteur à basculer mentalement les
valeurs de vérité. La charge est connue pour les lecteurs aphasiques
et ceux qui souffrent d'un trouble du déficit de l'attention (TDAH).
Le coût se multiplie sous pression cognitive. Les guides de langage
clair (FALC, CDC Clear Communication Index, plainlanguage.gov)
recommandent de réécrire les doubles négatives au positif.

## En bref

| | |
|---|---|
| **Catégorie** | `syntax` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `2` |
| **Étiquettes de condition** | `aphasia`, `adhd`, `general` |
| **Langues** | EN · FR (comptage spécifique par langue) |
| **Source** | [`src/rules/nested_negation.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/nested_negation.rs) |

## Détection

On compte les négations par phrase ; on signale les phrases dont le
compte dépasse `max_negations`.

- **Anglais** — somme des correspondances délimitées par mot contre
  la liste de négations de la langue (`not`, `no`, `never`, `none`,
  `nothing`, `nobody`, `nowhere`, `neither`, `nor`, `cannot`,
  `without`) plus les occurrences du suffixe contracté `n't`
  (`don't`, `won't`, `isn't`, `doesn't`, …).
- **Français** — comptage bipartite par paires. Chaque clitique
  `ne` / `n'` contribue pour une négation et s'apparie à la particule
  de seconde position la plus proche (`pas`, `rien`, `jamais`,
  `plus`, `personne`, `aucun`, `aucune`, `guère`, `nulle part`) dans
  une fenêtre courte ; l'appariement consomme simplement la particule
  pour éviter le double comptage. Les particules non appariées dans
  une phrase avec `ne` contribuent pour une de plus — ce qui attrape
  les formes comme `rien` employé en sujet nominal négatif. Garde-fous :
  `pas` / `plus` ne comptent jamais sans appariement (trop ambigus en
  dehors de `ne …`) ; `rien` précédé de `de` est traité comme
  l'idiome `de rien` et ignoré ; les particules d'une phrase sans
  clitique `ne` sont ignorées également (`plus de courage`,
  `personne d'autre`). Les autonomes `sans` / `non` comptent toujours.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_negations` | `int` | 3 | 2 | 1 |

## Exemples

`lucid-lint` signale ; la réécriture reste à l'auteur.

### Français

**Passe sous `public`** :

> Nous ne sommes pas prêts.

Le bipartite `ne … pas` compte pour une négation.

**Avant** (signalée) :

> Nous <span class="lucid-idea" data-idea="1">ne disons pas</span> que <span class="lucid-idea" data-idea="2">rien</span> <span class="lucid-idea" data-idea="3">n'est jamais possible</span>.

Trois négations : `ne…pas` (un bipartite), `rien` (non apparié),
`n'…jamais` (un bipartite).

Ce que rapporte `lucid-lint check --profile public` :

```text
warning input.md:1:1 Sentence stacks 3 negations (maximum 2). Rewrite as a positive statement or split the negations across separate sentences. [syntax.nested-negation]
```

**Après** (votre réécriture) :

> Nous <span class="lucid-idea" data-idea="1">disons</span> que <span class="lucid-idea" data-idea="2">quelque chose</span> <span class="lucid-idea" data-idea="3">est possible</span>.

### Anglais

Trois négations → trois affirmations, teintes assorties d'un bout à
l'autre de la réécriture. Le `not` disparaît simplement — la
simplification se voit.

**Avant** (signalée) :

> We do <span class="lucid-idea" data-idea="1">not say</span> <span class="lucid-idea" data-idea="2">nothing</span> is <span class="lucid-idea" data-idea="3">never possible</span>.

Trois négations (`not`, `nothing`, `never`).

**Après :**

> We <span class="lucid-idea" data-idea="1">say</span> <span class="lucid-idea" data-idea="2">something</span> <span class="lucid-idea" data-idea="3">is possible</span>.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`syntax.passive-voice`](./passive-voice.md)
- [`structure.deep-subordination`](./deep-subordination.md)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)

## Références

- [Clark & Chase (1972)](../references.md#clark-chase-1972)
- [Carpenter & Just (1975)](../references.md#carpenter-just-1975)
- [Kaup et al. (2006)](../references.md#kaup-2006)

Voir [Références](../references.md) pour la bibliographie complète.
