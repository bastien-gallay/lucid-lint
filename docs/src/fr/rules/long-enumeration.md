<!-- en-source-sha: 1eb7d48f3dc779a906e509fb196fe187a43d6ce9 -->
# `structure.long-enumeration`

*Énumération trop longue.*

## Ce que cette règle signale

Les énumérations en prose inline qui seraient plus claires sous forme
de liste à puces — 5 items ou plus séparés par des virgules et fermés
par un coordinateur (`et`, `ou`, `and`, `or`).

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (détection identique) |
| **Source** | [`src/rules/long_enumeration.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/long_enumeration.rs), helper partagé [`src/rules/enumeration.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/enumeration.rs) |

## Détection

Séquence de `min_items` segments courts ou plus, séparés par des
virgules, terminée par `, et` / `, ou` / `, and` / `, or` (virgule
Oxford facultative). Le détecteur partagé alimente également
[`structure.excessive-commas`](../../rules/excessive-commas.md).

<aside class="since-version" aria-label="Nouveauté en v0.3">

<span class="since-version__tag">Depuis v0.3</span> — Le détecteur
partagé reconnaît aussi les énumérations rythmiquement régulières
d'items de 1 à 4 mots (5 items minimum, écart de mots ≤ 2, sans
frontière de proposition à l'intérieur). Des phrases comme
`Chaque règle a une page dédiée avec catégorie, sévérité, poids par
défaut, seuils par profil, exemples, et neutralisation.` ressortent
maintenant comme candidates à une liste à puces
([F22](../../roadmap.md), seconde tranche).

</aside>

## Paramètres

| Clé | Type | Défaut |
|---|---|---|
| `min_items` | `int` | `5` |

## Message de diagnostic

Suggère de convertir l'énumération en liste à puces.

## Exemples

Six items, teintes assorties d'un bout à l'autre de la réécriture —
chaque terme inline s'aligne avec sa puce.

**Avant** (FR, signalée) :

<!-- lucid-lint-disable structure.long-enumeration -->
<!-- lucid-lint-disable syntax.dense-punctuation-burst -->

> Le plat contient <span class="lucid-idea" data-idea="1">tomate</span>, <span class="lucid-idea" data-idea="2">oignon</span>, <span class="lucid-idea" data-idea="3">ail</span>, <span class="lucid-idea" data-idea="4">basilic</span>, <span class="lucid-idea" data-idea="5">persil</span> et <span class="lucid-idea" data-idea="1">thym</span>.

<!-- lucid-lint-enable -->

**Après :**

> Le plat contient :
>
> - <span class="lucid-idea" data-idea="1">tomate</span>
> - <span class="lucid-idea" data-idea="2">oignon</span>
> - <span class="lucid-idea" data-idea="3">ail</span>
> - <span class="lucid-idea" data-idea="4">basilic</span>
> - <span class="lucid-idea" data-idea="5">persil</span>
> - <span class="lucid-idea" data-idea="1">thym</span>

**Avant** (EN, signalée) :

<!-- lucid-lint-disable structure.long-enumeration -->
<!-- lucid-lint-disable syntax.dense-punctuation-burst -->

> The dish contains <span class="lucid-idea" data-idea="1">tomato</span>, <span class="lucid-idea" data-idea="2">onion</span>, <span class="lucid-idea" data-idea="3">garlic</span>, <span class="lucid-idea" data-idea="4">basil</span>, <span class="lucid-idea" data-idea="5">parsley</span>, and <span class="lucid-idea" data-idea="1">thyme</span>.

<!-- lucid-lint-enable -->

**Après :**

> The dish contains:
>
> - <span class="lucid-idea" data-idea="1">tomato</span>
> - <span class="lucid-idea" data-idea="2">onion</span>
> - <span class="lucid-idea" data-idea="3">garlic</span>
> - <span class="lucid-idea" data-idea="4">basil</span>
> - <span class="lucid-idea" data-idea="5">parsley</span>
> - <span class="lucid-idea" data-idea="1">thyme</span>

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Références

- [Plain Language US (2011)](../references.md#plain-language-us-2011)

Voir [Références](../references.md) pour la bibliographie complète.
