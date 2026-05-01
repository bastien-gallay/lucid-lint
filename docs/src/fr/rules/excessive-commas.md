<!-- en-source-sha: 1eb7d48f3dc779a906e509fb196fe187a43d6ce9 -->
# `structure.excessive-commas`

*Virgules en excès.*

## Ce que cette règle signale

Les phrases dont le nombre de virgules dépasse un plafond par profil.
La virgule est le marqueur le plus fréquent de complexité syntaxique ;
plutôt que de démêler la cause (subordination, apposition, énumération,
incise), la règle se sert de la densité comme indicateur avancé de
surcharge.

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (détection identique) |
| **Source** | [`src/rules/excessive_commas.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/excessive_commas.rs) |

## Détection

Compter les virgules par phrase, signaler celles qui dépassent
`max_commas`.

**Interaction.** Quand [`structure.long-enumeration`](../../rules/long-enumeration.md)
se déclenche sur la même phrase, cette règle est neutralisée pour cette
phrase afin d'éviter un double signalement. Le détecteur d'énumération
partagé décompte les virgules Oxford (3 items courts ou plus, plus une
passe rythmique relâchée pour les items de 1 à 4 mots, plus les listes
fermées par `plus` au même titre que `et` / `ou` — voir « Faux positifs
connus » ci-dessous) et les virgules à l'intérieur des listes de tokens
parenthésées `(A, B, C, …)` (3 segments courts ou plus séparés par des
virgules entre parenthèses équilibrées) — tous les décomptes sont
agnostiques à la langue.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_commas` | `int` | 4 | 3 | 2 |

## Faux positifs connus

<aside class="since-version" aria-label="Nouveauté en v0.2.x">

<span class="since-version__tag">Depuis v0.2.x</span> — Les listes de
tokens parenthésées `(A, B, C, …)` ne comptent plus dans le total de
virgules ([F22](../../roadmap.md), première tranche).

</aside>

<aside class="since-version" aria-label="Nouveauté en v0.3">

<span class="since-version__tag">Depuis v0.3</span> — Les énumérations
Oxford d'items de 1 à 4 mots qui partagent une cadence régulière sont
désormais décomptées — par exemple `categorie, severite, poids par
defaut, seuils par profil, exemples, et neutralisation`. Le détecteur
exige au moins 5 items, un écart de mots ≤ 2, et ne remonte pas plus
loin que la frontière de proposition la plus proche
([F22](../../roadmap.md), seconde tranche).

</aside>

<aside class="since-version" aria-label="Nouveauté en v0.3">

<span class="since-version__tag">Depuis v0.3</span> — `plus` est
désormais reconnu comme terminateur Oxford au même titre que `and` /
`or` / `et` / `ou`. Une liste comme `profile, format, min-score, plus
working-directory and args` est détectée et décomptée. Même mot
connecteur en EN et FR ([F22](../../roadmap.md), troisième tranche).

</aside>

Les faux positifs restants viennent surtout des listes sans
connecteur terminal (par exemple `Rules touched: A, B, C`) et des
énumérations Oxford interrompues par une parenthèse interleavée ;
ils sont suivis sous **[F22](../../roadmap.md)** dans la
[feuille de route](../../roadmap.md) pour les prochaines
sous-tranches v0.3.

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`structure.long-enumeration`](../../rules/long-enumeration.md)
- [`structure.deep-subordination`](../../rules/deep-subordination.md)

## Références

- [Gibson (1998)](../references.md#gibson-1998)

Voir [Références](../references.md) pour la bibliographie complète.
