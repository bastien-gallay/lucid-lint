# `structure.deep-subordination`

## Ce que cette règle signale

Les cascades de subordonnées : plusieurs pronoms relatifs ou
conjonctions de subordination enchaînés sans rupture forte de
ponctuation. Chaque référent ouvert doit rester en mémoire de travail
jusqu'à sa clôture — la *Dependency Locality Theory* de Gibson (1998)
relie le coût de traitement directement à cette distance.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `2` |
| **Langues** | EN · FR (listes distinctes) |
| **Source** | [`src/rules/deep_subordination.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/deep_subordination.rs) |

## Détection

Parcours de la phrase entre ruptures fortes de ponctuation ;
décompte des subordonnants consécutifs. Signalement quand le décompte
dépasse `max_consecutive_subordinators`. Les *énumérations* de
pronoms (`qui, que, dont, où`) sont ignorées — le détecteur reconnaît
la forme listée et ne la traite pas comme une cascade.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_consecutive_subordinators` | `int` | 3 | 2 | 2 |

## Listes par langue

- 🇫🇷 Pronoms relatifs : *qui, que, dont, où, lequel, laquelle, lesquels, lesquelles*
- 🇫🇷 Subordonnants : *parce que, afin que, bien que, quoique, puisque, pour que, tandis que*
- 🇬🇧 Pronoms relatifs : *which, that, who, whom, whose*
- 🇬🇧 Subordonnants : *because, although, while, since, whereas, unless, until*

## Exemples

Chaque token surligné est un subordonnant compté par la règle. Quatre consécutifs déclenchent le seuil `dev-doc` (3) ; deux consécutifs déclenchent `public` et `falc`.

Signalé (FR) :

<!-- lucid-lint disable-next-line structure.deep-subordination -->
<!-- lucid-lint disable-next-line syntax.passive-voice -->

> Le document <span class="lucid-idea" data-idea="1">qui</span> a été rédigé par l'équipe <span class="lucid-idea" data-idea="2">que</span> nous avons constituée et <span class="lucid-idea" data-idea="3">qui</span> couvre les points <span class="lucid-idea" data-idea="4">que</span> nous avions discutés…

Signalé (EN) :

<!-- lucid-lint disable-next-line structure.deep-subordination -->
<!-- lucid-lint disable-next-line syntax.passive-voice -->

> The report <span class="lucid-idea" data-idea="1">that</span> was drafted by the team <span class="lucid-idea" data-idea="2">which</span> we formed last month and <span class="lucid-idea" data-idea="3">which</span> covers the topics <span class="lucid-idea" data-idea="4">that</span> we had discussed…

Non signalé (forme énumération, reconnue par le détecteur) :

> Les pronoms relatifs en français sont : qui, que, dont, où.

Et la forme équivalente en anglais :

> The English relative pronouns are: which, that, who, whom, whose.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).
