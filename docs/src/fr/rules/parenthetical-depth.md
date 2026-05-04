<!-- en-source-sha: 0000000000000000000000000000000000000000 -->
# `syntax.parenthetical-depth`

> *Expérimentale en v0.2.x.* Désactivée par défaut ; activée via
> `--experimental syntax.parenthetical-depth` ou
> `[experimental] enabled = ["syntax.parenthetical-depth"]` dans
> `lucid-lint.toml`. Passe à `Stable` à la coupe v0.3 dans le cadre
> de la cohorte
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status).
> Voir [Conditions](../guide/conditions.md) pour les étiquettes
> `adhd` et `general`.

## Ce que la règle signale

Une phrase dont la profondeur d'imbrication maximale entre crochets équilibrés `()` et `[]` atteint le seuil du profil. Les parenthèses empilées obligent la lectrice à garder en mémoire plusieurs idées suspendues à la fois — un signal reconnu de « phrase difficile » dans la tradition plainlanguage.gov et Hemingway, et un coût particulier pour les lectrices avec TDAH, qui portent en premier la charge en mémoire de travail.

La règle complète `structure.excessive-commas`, qui ignore déjà les énumérations plates `(A, B, C)` à profondeur 1. Cette règle-ci ne se déclenche qu'à partir de la profondeur 2 ; les deux règles sont mécaniquement orthogonales.

## En un coup d'œil

| | |
|---|---|
| **Catégorie** | `syntax` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Statut** | `experimental` (v0.2.x) → `stable` à la coupe v0.3 |
| **Étiquettes de condition** | `adhd`, `general` (filtrées : exécutée seulement si `--conditions` correspond) |
| **Langues** | EN · FR (indépendant de la langue — les familles de crochets sont identiques) |
| **Source** | [`src/rules/syntax/parenthetical_depth.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/syntax/parenthetical_depth.rs) |

## Détection

Pour chaque phrase, la règle parcourt le texte du paragraphe une fois aplati par le parseur (les blocs de code sont donc déjà exclus en amont) et tient un seul compteur de profondeur courante.

### Algorithm

1. Parcourir la phrase un caractère à la fois.
2. Incrémenter la profondeur sur `(` ou `[` ; décrémenter sur `)` ou `]`.
3. Une fermeture qui ferait passer la profondeur sous zéro la remet à zéro — la règle reste tolérante face à un balisage déséquilibré, comme le fait l'aide `parenthesised_list_comma_count` utilisée par `structure.excessive-commas`.
4. Suivre la profondeur maximale atteinte et la position du crochet ouvrant qui l'a atteinte.
5. Émettre un diagnostic par phrase quand `max_depth ≥` le seuil du profil, ancré sur le crochet ouvrant le plus profond.

### Exclusions (garde-fous contre les faux positifs)

- **Spans / blocs de code** : déjà exclus en amont par le parseur Markdown.
- **Crochets déséquilibrés** : la remise à zéro empêche les fermetures isolées de gonfler une profondeur ultérieure.

### Reporté (hors MVP)

Les paires de tirets longs (`— … —`), les accolades (`{}`) et les appositions encadrées par des virgules sont volontairement hors scope en v0.2.x. Détecter une paire de tirets longs est fragile (confusion entre tirets demi-cadratin / cadratin, ambiguïté avec le trait d'union) et ramènerait du périmètre par la fenêtre.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_depth` | `int` | 4 | 3 | 2 |

`max_depth` est la profondeur d'imbrication inclusive à laquelle la règle se déclenche. Une phrase dont le crochet le plus profond reste un cran en-dessous reste silencieuse.

Réglage via `lucid-lint.toml` :

```toml
[rules."syntax.parenthetical-depth"]
max_depth = 3
```

## Exemples

### Anglais

**Avant** (signalé) :

> The migration tool (which now supports rollbacks (see `--reverse`, added in 0.4.2 [tracked in #312])) is opt-in.

Ce que `lucid-lint check --profile public --experimental syntax.parenthetical-depth --conditions adhd` rapporte :

```text
warning input.md:1:21 Nested parentheticals reach depth 3; readers must hold 3 suspended thoughts to reach the close. Split the sentence or unnest the inner bracket (plainlanguage.gov, Hemingway). [syntax.parenthetical-depth]
```

**Après** (réécriture proposée) :

> The migration tool is opt-in. It now supports rollbacks via `--reverse`, added in 0.4.2 (tracked in #312).

Les deux parenthétiques de premier niveau ont disparu ; il ne reste qu'une parenthèse plate à profondeur 1. La lectrice n'a plus à empiler trois pensées suspendues pour arriver au point.

### Français

**Avant** (signalé) :

> Le module (qui dépend du noyau (chargé au démarrage [voir le manuel])) est facultatif.

Ce que `lucid-lint check --profile public --experimental syntax.parenthetical-depth --conditions adhd` rapporte :

```text
warning input.md:1:23 Nested parentheticals reach depth 3; readers must hold 3 suspended thoughts to reach the close. Split the sentence or unnest the inner bracket (plainlanguage.gov, Hemingway). [syntax.parenthetical-depth]
```

**Après** (réécriture proposée) :

> Le module est facultatif. Il dépend du noyau, chargé au démarrage. Voir le manuel pour les détails.

Trois phrases, aucun crochet imbriqué. La chaîne de dépendances est désormais linéaire et la lectrice récupère chaque fait dans l'ordre où il apparaît.

## Neutralisation

Voir [Neutralisation des diagnostics](../guide/suppression.md) pour les formes inline et bloc. La désactivation inline fonctionne aussi sur cette règle :

```markdown
<!-- lucid-lint disable-next-line syntax.parenthetical-depth -->
The migration tool (which now supports rollbacks (see `--reverse`, added in 0.4.2 [tracked in #312])) is opt-in.
```

## Voir aussi

- [Conditions](../guide/conditions.md) — les étiquettes `adhd` et `general` qui filtrent cette règle.
- [`structure.excessive-commas`](./excessive-commas.md) — règle sœur sur les énumérations plates entre parenthèses. Découpage atomique : `excessive-commas` ignore les listes `(A, B, C)` à profondeur 1 ; cette règle se déclenche seulement à partir de la profondeur 2.
- [`syntax.dense-punctuation-burst`](./dense-punctuation-burst.md) — règle sœur sur la densité locale de ponctuation. Les deux règles signalent des phrases difficiles à analyser, sous deux angles différents.
- [F-experimental-rule-status — statut expérimental d'une règle](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrat qui permet à cette règle de paraître en v0.2.x sans affecter les scores par défaut.

## Références

- [plainlanguage.gov — Write short sentences](https://www.plainlanguage.gov/guidelines/concise/write-short-sentences/). Les recommandations plain-language traitent les qualificatifs empilés et les parenthèses imbriquées comme le symptôme canonique de la « phrase trop longue ».
- Tradition d'édition Hemingway — fait remonter les phrases « difficiles à lire » quand elles superposent plusieurs idées suspendues ; les parenthèses imbriquées en sont la lecture mécanique la plus propre.

Voir [Références](../references.md) pour la bibliographie complète.
