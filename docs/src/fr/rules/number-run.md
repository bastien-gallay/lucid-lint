<!-- en-source-sha: ed0f2b60fdc3b6bdadb49f0ecfeae94e8455db1f -->
# `structure.number-run`

*Trop de nombres dans une seule phrase.*

> *Expérimentale en v0.2.x.* Désactivée par défaut ; activez-la via
> `--experimental structure.number-run` ou
> `[experimental] enabled = ["structure.number-run"]` dans
> `lucid-lint.toml`. Passe à `Stable` au moment du tag v0.3 dans le
> cadre de la cohorte
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status).
> Voir [Conditions](../guide/conditions.md) pour le tag `dyscalculia`
> qui gouverne cette règle selon les conditions actives.

## Ce que la règle détecte

Les phrases qui empilent plus d'un seuil configurable de jetons numériques. plainlanguage.gov est explicite sur le cadrage — *« Don't put a lot of numbers together in one sentence »* et *« Avoid placing too many statistics close together »* — et les personnes dyscalculiques en paient le coût en premier : chaque jeton numérique force un nouvel ancrage quantité-vers-symbole qui ne profite pas du contexte de la prose comme un mot ordinaire. Les enfilades de citations (`(Smith 2020, Jones 2021, Wei 2022, Park 2023)`), les tableaux de mesures aplatis dans la prose et les paragraphes saturés de statistiques sont les cas typiques.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Statut** | `experimental` (v0.2.x) → `stable` au tag v0.3 |
| **Tag de condition** | `dyscalculia` (gouverné ; ne s'exécute qu'avec `--conditions` correspondant) |
| **Langues** | EN · FR (détection identique — les chiffres sont agnostiques) |
| **Source** | [`src/rules/structure/number_run.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/structure/number_run.rs) |

## Détection

Parcourt le flux de phrases de chaque paragraphe (après aplatissement, les blocs de code clos sont déjà exclus par le parseur) et compte les jetons numériques par phrase. Un jeton numérique est une suite contiguë de chiffres ASCII, contenant éventuellement *un* séparateur décimal (`.` ou `,`) suivi de chiffres. Le tiret, le deux-points, la barre oblique et les espaces séparent les jetons.

| Entrée | Jetons comptés | Remarque |
|---|---|---|
| `42` | 1 | Entier nu |
| `3.14` | 1 | Séparateur décimal conservé |
| `1,000` | 1 | Virgule conservée |
| `2026-05-04` | 3 | Les tirets séparent — une date *vaut* trois nombres en charge cognitive |
| `$3.50` | 1 | Préfixe monétaire non-chiffre, ignoré |
| `1st` | 1 | Lettres finales séparées ; les chiffres comptent |

La position du diagnostic pointe sur le *premier* jeton numérique de la phrase fautive : le surlignage de l'éditeur tombe sur le bloc visible plutôt qu'au début de la phrase.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_numbers` | `int` | 6 | 4 | 3 |

Pour ajuster via `lucid-lint.toml` :

```toml
[rules."structure.number-run"]
max_numbers = 5
```

## Exemples

### Anglais

**Avant** (signalé) :

> The 2024 cohort sat 1,200 students across 4 campuses, posted a 92.5% pass rate on the 3 reviewed papers, and improved 18 points over the prior year.

Ce que `lucid-lint check --profile public --experimental structure.number-run --conditions dyscalculia` rapporte :

```text
warning input.md:1:5 Sentence packs 8 numeric tokens (maximum 4). plain-language guidance recommends not placing many numbers or statistics together in one sentence; split the sentence or move some figures to a list or table. [structure.number-run]
```

**Après** (votre réécriture) :

> The 2024 cohort sat 1,200 students across 4 campuses. They posted a 92.5% pass rate on the reviewed papers and improved 18 points over the prior year.

Les chiffres voyagent toujours ensemble, mais chaque phrase porte une charge qu'une lectrice dyscalculique peut ré-ancrer sans perdre le référent.

### Français

**Avant** (signalé) :

> La promotion 2024 a réuni 1 200 étudiants sur 4 campus, affiché un taux de réussite de 92,5 % sur les 3 copies revues, et progressé de 18 points par rapport à l'année précédente.

**Après** (votre réécriture) :

> La promotion 2024 a réuni 1 200 étudiants sur 4 campus. Le taux de réussite atteint 92,5 % sur les copies revues et progresse de 18 points par rapport à l'année précédente.

## Suppression

Voir [Supprimer les diagnostics](../guide/suppression.md) pour les formes inline et bloc. La désactivation inline fonctionne aussi sur cette règle :

```markdown
<!-- lucid-lint disable-next-line structure.number-run -->
The 2024 cohort sat 1,200 students across 4 campuses, posted a 92.5% pass rate on the 3 reviewed papers, and improved 18 points.
```

## Voir aussi

- [Conditions](../guide/conditions.md) — le tag `dyscalculia` qui gouverne cette règle.
- [`structure.mixed-numeric-format`](./mixed-numeric-format.md) — règle sœur sur la cohérence de la *forme* numérique. Découpe atomique : `mixed-numeric-format` regarde si chiffres et numéraux écrits cohabitent ; `number-run` regarde combien de jetons numériques s'agglutinent, peu importe la forme.
- [F-experimental-rule-status — statut expérimental](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrat qui permet à cette règle d'arriver en v0.2.x sans affecter les scores par défaut.

## Références

- [plainlanguage.gov — Use short, simple sentences](https://www.plainlanguage.gov/guidelines/concise/use-short-simple-sentences/). *« Don't put a lot of numbers together in one sentence. »*
- [plainlanguage.gov — Use numerals](https://www.plainlanguage.gov/guidelines/words/use-numerals/). Conseil compagnon sur la cohérence de la forme numérique (qui motive `mixed-numeric-format`).

Voir [Références](../references.md) pour la bibliographie complète.
