# `lexicon.all-caps-shouting`

*Majuscules criardes.*

## Ce que cette règle signale

Les suites de mots consécutifs en MAJUSCULES.

Le texte tout en majuscules supprime les indices de forme sur
lesquels les lecteurs dyslexiques s'appuient pour distinguer les
mots :

- **Ascendantes** — les hampes qui montent au-dessus du corps des
  lettres comme `b, d, h, k, l`.
- **Descendantes** — les hampes qui descendent sous la ligne de
  base dans `g, p, q, y`.
- **Contraste de hauteur d'x** — l'écart entre les lettres courtes
  comme `a, e, o` et les hautes comme `h, l`.

En tout-majuscules, chaque lettre repose sur la même ligne de base
à la même hauteur. Le lecteur perd la silhouette du mot et doit
décoder lettre à lettre. Le tout-majuscules déclenche aussi de
nombreux lecteurs d'écran à épeler la suite lettre à lettre, sauf
indication contraire dans le balisage.

WCAG 3.1.5 et le *BDA Dyslexia Style Guide* recommandent la
minuscule ou la casse de phrase pour l'emphase.

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Étiquettes de condition** | `a11y-markup`, `dyslexia`, `general` |
| **Langues** | EN · FR (détection sur le script — agnostique de la langue) |
| **Source** | [`src/rules/all_caps_shouting.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/all_caps_shouting.rs) |

## Détection

Par paragraphe, on cherche les suites de mots consécutifs en
MAJUSCULES. Les connecteurs mineurs (`,`, `;`, `:`, `-`, espaces)
gardent la suite vivante ; un mot en minuscule, un point ou un saut
de paragraphe la termine.

Un mot est en MAJUSCULES quand il fait au moins 2 lettres et ne
contient aucune minuscule. Les jetons en MAJUSCULES isolés sont
traités comme des abréviations et relèvent de
[`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md).

Les blocs de code sont exclus par le parseur Markdown avant que la
règle ne s'exécute.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_run_length` | `int` | 3 | 2 | 2 |

`dev-doc` tolère une emphase à 2 mots (`DO NOT`) courante en
documentation technique.

## Exemple

`lucid-lint` signale ; la réécriture vous appartient toujours.

Une seule formule d'emphase, mise en couleur dans la version
réécrite — le cri devient une emphase typographique sans perdre
l'insistance.

**Avant** (signalé) :

> Please <span class="lucid-idea" data-idea="1">DO NOT</span> touch
> this.

`DO NOT` se lit comme un cri.

Ce que `lucid-lint check --profile public` rapporte :

```text
warning input.md:1:8 2 consecutive ALL-CAPS words read as shouting and degrade legibility for dyslexic readers. Use sentence case and rely on emphasis (italics, bold) or a callout instead. [lexicon.all-caps-shouting]
```

**Après** (votre réécriture) :

> Please <span class="lucid-idea" data-idea="1">*do not*</span> touch
> this.

## Faux positifs connus

Une chaîne de trois acronymes ou plus en prose (`API HTTP TLS`) est
structurellement indiscernable d'un cri et déclenchera la règle.
Neutraliser sur la ligne si la chaîne est intentionnelle, ou
restructurer la phrase.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)

## Références

- [Arditi & Cho (2007)](../references.md#arditi-cho-2007)
- [Nielsen Norman Group](../references.md#nielsen-norman-allcaps)
- [Bringhurst (2013)](../references.md#bringhurst-2013)

Voir [Références](../references.md) pour la bibliographie complète.
