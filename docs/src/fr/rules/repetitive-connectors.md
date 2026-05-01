<!-- en-source-sha: 3dbafc6a8549974088e2c425e4509e41fc7311ad -->
# `rhythm.repetitive-connectors`

*Répétition de connecteurs.*

## Ce que cette règle signale

Surutilisation d'un même connecteur logique dans une fenêtre courte
de phrases. Les connecteurs (opposition, cause, conséquence, séquence,
illustration, addition) sont des points d'attention ; répétés, ils
aplatissent le sentiment de progression. Sanders & Noordman (2000),
*Connectives as processing signals* ; Graesser et al. (2004), cohésion
locale.

## En bref

| | |
|---|---|
| **Catégorie** | `rhythm` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (listes séparées) |
| **Source** | [`src/rules/repetitive_connectors.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/repetitive_connectors.rs) |

## Détection

Fenêtre glissante de `window_size` phrases. Par connecteur, compter
les occurrences dans la fenêtre. Émettre un diagnostic par grappe
qui dépasse `max_per_window`.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_per_window` | `int` | 4 | 3 | 2 |
| `window_size` | `int` | 5 | 5 | 5 |
| `custom_connectors` | `list` | `[]` | `[]` | `[]` |

## Listes de connecteurs par défaut

- 🇫🇷 Opposition : *cependant, toutefois, en revanche, néanmoins, pourtant, mais*
- 🇫🇷 Cause : *parce que, car, puisque, en effet*
- 🇫🇷 Conséquence : *donc, ainsi, par conséquent, c'est pourquoi*
- 🇫🇷 Séquence : *d'abord, ensuite, puis, enfin, premièrement*
- 🇫🇷 Illustration : *par exemple, notamment, en particulier*
- 🇫🇷 Addition : *de plus, en outre, également, par ailleurs*
- 🇬🇧 Opposition : *however, nevertheless, yet, although, but*
- 🇬🇧 Cause : *because, since, as, for*
- 🇬🇧 Conséquence : *therefore, thus, consequently, hence, so*
- 🇬🇧 Séquence : *first, then, next, finally*
- 🇬🇧 Illustration : *for example, notably, in particular, such as*
- 🇬🇧 Addition : *moreover, furthermore, also, additionally*

## Exemples

`lucid-lint` signale ; la réécriture vous appartient.

### Français

Cinq actions, teintes assorties d'un bout à l'autre de la réécriture —
seuls les connecteurs changent.

<!-- lucid-lint-disable rhythm.repetitive-connectors -->

**Avant** (signalée) :

> <span class="lucid-idea" data-idea="1">Nous avons analysé les données.</span> Ensuite <span class="lucid-idea" data-idea="2">nous avons construit le modèle.</span> Ensuite <span class="lucid-idea" data-idea="3">nous avons validé les résultats.</span> Ensuite <span class="lucid-idea" data-idea="4">nous avons publié le rapport.</span> Ensuite <span class="lucid-idea" data-idea="5">nous avons archivé les données brutes.</span>

Quatre *ensuite* en cinq phrases — aucune progression ressentie.

Ce que `lucid-lint check --profile public` rapporte :

```text
warning input.md:1:1 Connector "ensuite" appears 4 times within 5 consecutive sentences (max 3). Vary the connector or restructure the passage. [rhythm.repetitive-connectors]
```

**Après** (votre réécriture) :

> <span class="lucid-idea" data-idea="1">Nous avons analysé les données.</span> À partir de là <span class="lucid-idea" data-idea="2">nous avons construit le modèle.</span> <span class="lucid-idea" data-idea="3">La validation a suivi,</span> et dès que les résultats ont tenu <span class="lucid-idea" data-idea="4">nous avons publié le rapport.</span> <span class="lucid-idea" data-idea="5">Les données brutes ont été archivées en dernier.</span>

<!-- lucid-lint-enable -->

### Anglais

Cinq actions, teintes assorties d'un bout à l'autre de la réécriture —
seuls les connecteurs changent.

<!-- lucid-lint-disable rhythm.repetitive-connectors -->

**Avant** (signalée) :

> <span class="lucid-idea" data-idea="1">We analysed the data.</span> Then <span class="lucid-idea" data-idea="2">we built the model.</span> Then <span class="lucid-idea" data-idea="3">we validated the results.</span> Then <span class="lucid-idea" data-idea="4">we published the report.</span> Then <span class="lucid-idea" data-idea="5">we archived the raw data.</span>

Quatre *then* en cinq phrases — aucune progression ressentie.

Ce que `lucid-lint check --profile public` rapporte :

```text
warning input.md:1:1 Connector "then" appears 4 times within 5 consecutive sentences (max 3). Vary the connector or restructure the passage. [rhythm.repetitive-connectors]
```

**Après** (votre réécriture) :

> <span class="lucid-idea" data-idea="1">We analysed the data.</span> From it <span class="lucid-idea" data-idea="2">we built the model.</span> <span class="lucid-idea" data-idea="3">Validation followed,</span> and once the results held up <span class="lucid-idea" data-idea="4">we published the report.</span> <span class="lucid-idea" data-idea="5">The raw data was archived last.</span>

<!-- lucid-lint-enable -->

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant) pour les formes en ligne et par bloc.

## Voir aussi

- [`structure.sentence-too-long`](./sentence-too-long.md) — phrases longues et abus de connecteurs co-occurrent souvent ; signaler les deux fait apparaître un signal de rythme plus riche.
- [Modèle de score](../../guide/scoring.md) — `rhythm.repetitive-connectors` porte le poids par défaut `1` ; le coût est local, pas cumulatif.

## Références

- [Sanders & Noordman (2000)](../references.md#sanders-noordman-2000)
- [Graesser et al. (2004)](../references.md#graesser-2004)

Voir [Références](../references.md) pour la bibliographie complète.
