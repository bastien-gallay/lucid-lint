<!-- en-source-sha: 0000000000000000000000000000000000000000 -->
# `readability.large-number-unanchored`

> *Expérimentale en v0.2.x.* Désactivée par défaut ; activée via
> `--experimental readability.large-number-unanchored` ou
> `[experimental] enabled = ["readability.large-number-unanchored"]`
> dans `lucid-lint.toml`. Passe à `Stable` à la coupe v0.3 dans le
> cadre de la cohorte
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status).
> Voir [Conditions](../guide/conditions.md) pour les étiquettes
> `dyscalculia` et `general`.

## Ce que la règle signale

Un grand nombre ou un mot d'ordre de grandeur qui apparaît dans une phrase sans aucun ancrage proche — pas d'unité, pas de pourcentage, pas de symbole monétaire, pas de ratio, pas de phrase de comparaison. Le CDC Clear Communication Index demande si les nombres sont *clairs et utiles pour le public visé* ; plainlanguage.gov est plus direct sur le mécanisme — *« Use Numbers Effectively »* recommande d'accompagner chaque grand nombre d'une comparaison ou d'un dénominateur que la lectrice peut situer. Les lectrices avec dyscalculie portent ce coût en premier : un *« 4,8 milliards »* hors contexte impose une estimation de l'ordre de grandeur à l'aveugle, là où la prose ordinaire fournit habituellement des appuis.

La règle complète `structure.number-run`, qui se déclenche sur des *grappes* numériques (≥ N tokens dans une même phrase). Cette règle-ci se déclenche sur un *seul* grand nombre ou mot d'ordre de grandeur sans ancrage.

## En un coup d'œil

| | |
|---|---|
| **Catégorie** | `readability` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Statut** | `experimental` (v0.2.x) → `stable` à la coupe v0.3 |
| **Étiquettes de condition** | `dyscalculia`, `general` (filtrée ; ne s'active qu'avec `--conditions` correspondants) |
| **Langues** | EN · FR (lexiques de comparateurs et de références figure/page par langue) |
| **Source** | [`src/rules/readability/large_number_unanchored.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/readability/large_number_unanchored.rs) |

## Détection

Pour chaque phrase, la règle parcourt le texte de paragraphe (post-aplatissement, donc les blocs de code clôturés sont déjà exclus par le parseur) et cherche les candidats sans ancrage.

### Définition d'un candidat

Un candidat de niveau phrase est l'un de :

1. Un **token numérique** dont le nombre de chiffres est ≥ 4 *et* dont la valeur entière est ≥ le seuil du profil. Le scanner replie les séparateurs de milliers courants (`,`, `.`, espace ASCII, NBSP, espace fine, NBSP étroite) entre les groupes de chiffres, donc `1 000` (FR) et `1,000` (EN) comptent tous les deux comme un seul token de 4 chiffres et de valeur 1000.
2. Un **mot d'ordre de grandeur** — `million`(s), `milliard`(s), `billion`(s), `trillion`(s) en FR ; `million`(s), `billion`(s), `trillion`(s) en EN. Mot entier, insensible à la casse.

### Filtres (garde-fous contre les faux positifs)

- **Forme année** : exactement 4 chiffres contigus, sans séparateur de milliers ni de décimale, et de valeur dans `1000..=2999`. `2024` et `1789` sont des années, pas des ordres de grandeur.
- **Ordinal** : suite de chiffres immédiatement suivie d'une lettre (`1st`, `12th`).
- **Référence figure / page / section** : candidat précédé (dans les 16 octets, même phrase) par `figure`, `page`, `section`, `tableau`, `chapitre`, `annexe`, `§`, `p.`, `pp.`, `n°`, `#`, ou les équivalents EN.

### Types d'ancrage (au niveau de la phrase)

L'un quelconque des éléments ci-dessous, n'importe où dans la phrase, ancre *tous* les candidats de la phrase :

- Signe pourcent (`%`).
- Symbole monétaire (`€`, `$`, `£`, `¥`).
- Token d'unité issu d'une petite liste curated (`km`, `kg`, `m²`, `°C`, `L`, `Hz`, `Mo`, …).
- Motif de ratio : `X sur Y`, `X out of Y`, ou `X / Y` entre chiffres.
- Phrase de comparateur du lexique par langue (FR : `soit environ`, `équivalent à`, `environ`, `plus de`, `par rapport à`, … ; EN : `roughly`, `approximately`, `more than`, `the size of`, …).

La position du diagnostic pointe sur le *premier* candidat survivant dans la phrase fautive, pour que le surlignage tombe sur le nombre visible plutôt que sur le début de la phrase.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_value` | `int` | 100000 | 10000 | 1000 |

`min_value` est la borne inférieure inclusive sur la valeur entière d'un candidat numérique. Les tokens qui passent le filtre du nombre de chiffres mais dont la valeur est en-dessous de `min_value` sont ignorés — les quantités de type numéro de page passent déjà par le filtre référence figure/page ; ce paramètre est un second filet.

À régler via `lucid-lint.toml` :

```toml
[rules."readability.large-number-unanchored"]
min_value = 50000
```

## Exemples

### Français

**Avant** (signalé) :

> Le budget atteint 4 800 000 000 selon le rapport final.

Ce que `lucid-lint check --profile public --experimental readability.large-number-unanchored --conditions dyscalculia` rapporte :

```text
warning input.md:1:19 Large numeral (10-digit, value ≈ 4800000000) appears with no anchor in this sentence (no unit, percentage, ratio, or comparison phrase). plain-language guidance recommends giving large numbers a comparison or denominator the reader can ground. [readability.large-number-unanchored]
```

**Après** (votre réécriture) :

> Le budget atteint 4,8 milliards d'euros, soit environ 6 % du PIB selon le rapport final.

Le nombre est désormais accompagné d'une unité (`euros`), d'un pourcentage (`6 %`) et d'une phrase de comparateur (`soit environ`). Une lectrice qui ne peut pas estimer « 4,8 milliards » à brut dispose maintenant de trois ancres indépendantes.

### Anglais

**Avant** (signalé) :

> The proposal mentions several billion in vague spending across regions.

**Après** (votre réécriture) :

> The proposal mentions several billion dollars in vague spending across regions, roughly the annual budget of a mid-sized state agency.

L'ordre de grandeur est désormais accompagné d'une unité (`dollars`) et d'une phrase de comparateur (`roughly the annual budget`).

## Suppression

Voir [Suppression des diagnostics](../guide/suppression.md) pour les formes en ligne et en bloc. La désactivation en ligne fonctionne aussi sur cette règle :

```markdown
<!-- lucid-lint disable-next-line readability.large-number-unanchored -->
Le budget atteint 4 800 000 000 selon le rapport final.
```

## Voir aussi

- [Conditions](../guide/conditions.md) — les étiquettes `dyscalculia` et `general` qui filtrent cette règle.
- [`structure.number-run`](./number-run.md) — règle sœur sur les grappes numériques. Découpe atomique : `number-run` se déclenche sur des grappes de tokens numériques ; cette règle-ci se déclenche sur un seul grand nombre sans ancrage.
- [`structure.mixed-numeric-format`](./mixed-numeric-format.md) — autre règle sœur, sur la cohérence de *forme* numérique (chiffres vs lettres).
- [F-experimental-rule-status — statut expérimental des règles](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrat qui permet à cette règle d'être livrée en v0.2.x sans affecter les scores par défaut.

## Références

- [plainlanguage.gov — Use numbers effectively](https://www.plainlanguage.gov/guidelines/words/use-numbers-effectively/). *« Help your reader visualize numbers… Compare numbers to something the reader is familiar with. »*
- [CDC Clear Communication Index — Numbers](https://www.cdc.gov/ccindex/). L'item 6 demande si les nombres sont clairs et utiles pour le public visé.

Voir [Références](../references.md) pour la bibliographie complète.
