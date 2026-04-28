# `lexicon.excessive-nominalization`

*Nominalisations en excès.*

## Ce que cette règle signale

Les phrases densément peuplées de nominalisations — verbes
transformés en noms abstraits. Deux problèmes se cumulent : le
texte nominalisé est plus abstrait (plus coûteux à traiter) et il
masque l'agent (« qui fait quoi » disparaît). Le FALC et le *Plain
Writing Act* américain recommandent les verbes forts plutôt que
les nominalisations.

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (listes de suffixes qui se recoupent) |
| **Source** | [`src/rules/excessive_nominalization.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/excessive_nominalization.rs) |

## Détection

Parcours de la phrase. On signale les mots dont le suffixe figure
dans la liste de la langue. Le diagnostic se déclenche quand le
nombre par phrase franchit `max_per_sentence`.

- 🇫🇷 Suffixes : `-tion`, `-sion`, `-ment`, `-ance`, `-ence`,
  `-age`, `-ité`, `-isme`, `-ure`
- 🇬🇧 Suffixes : `-tion`, `-sion`, `-ment`, `-ance`, `-ence`,
  `-ity`, `-ism`, `-ness`, `-al`

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_per_sentence` | `int` | 4 | 3 | 2 |
| `suffixes` | `list` | défauts par langue | défauts par langue | défauts par langue |

## Faux positifs connus

Le vocabulaire technique (`function`, `implementation`,
`configuration`) contient beaucoup de nominalisations légitimes,
ce qui justifie le seuil relâché de `dev-doc`. Le suffixe `-al` en
anglais est trop large (il signale `crucial`, `horizontal`,
`positional` alors qu'il ne s'agit pas de noms abstraits) et reste
suivi sous **[F24](../roadmap.md)** dans la
[feuille de route](../roadmap.md).

## Exemple

Nominalisations mises en couleur, appariées aux verbes actifs
correspondants dans la version réécrite.

Avant (lourd) :

<!-- lucid-lint disable-next-line lexicon.excessive-nominalization -->

> La <span class="lucid-idea" data-idea="1">réalisation</span> de
> l'<span class="lucid-idea" data-idea="2">analyse</span> de la
> conformité permettra l'<span class="lucid-idea" data-idea="3">identification</span>
> des axes d'<span class="lucid-idea" data-idea="4">amélioration</span>.

Après (allégé) :

> Nous <span class="lucid-idea" data-idea="2">analyserons</span> la
> conformité. Cela permettra d'<span class="lucid-idea" data-idea="3">identifier</span>
> les axes à <span class="lucid-idea" data-idea="4">améliorer</span>.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Références

- [Plain Language US (2011)](../references.md#plain-language-us-2011)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

Voir [Références](../references.md) pour la bibliographie complète.
