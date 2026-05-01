# `lexicon.weasel-words`

*Mots évasifs.*

## Ce que cette règle signale

Les qualificatifs vagues qui affaiblissent une affirmation. Un mot
fuyant ajoute une charge cognitive invisible : le lecteur doit décider
si l'assertion compte, est vraie, ou mesurable. Références : guide de
style Wikipédia (*Avoid weasel words*), Strunk & White, FALC.

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (listes distinctes) |
| **Source** | [`src/rules/weasel_words.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/weasel_words.rs) |

## Détection

Correspondance sur frontière de mot contre une liste par langue.
Insensible à la casse. Un diagnostic par occurrence.

<aside class="since-version" aria-label="Nouveauté en v0.2">

<span class="since-version__tag">Depuis v0.2</span> — Deux exclusions
contextuelles arrivent comme première tranche de
**[F23](../../roadmap.md)**.

</aside>

- **Spans de code inline.** Une occurrence à l'intérieur de `` `…` ``
  est ignorée. Entourer un terme fuyant de backticks quand on parle du
  mot lui-même.
- **Appariements directionnels.** `plutôt que` (FR) et `rather than`
  (EN) sont des conjonctions qui signifient « au lieu de » — ce ne
  sont pas des formules d'atténuation — et sont ignorés.

## Paramètres

| Clé | Type | Défaut |
|---|---|---|
| `custom_weasels_fr` | `list` | `[]` |
| `custom_weasels_en` | `list` | `[]` |
| `disable_weasels` | `list` | `[]` |

## Listes par défaut (v0.1)

- 🇫🇷 *quelques, certains, parfois, plutôt, assez, globalement, généralement, souvent, en général, la plupart, il semble que, il semblerait que, on pourrait dire que, on dit souvent, beaucoup de, peu de, presque, quasiment, environ, à peu près*
- 🇬🇧 *some, many, often, just, simply, clearly, obviously, seemingly, arguably, basically, essentially, virtually, various, numerous, sort of, kind of, a bit, rather, quite, fairly, relatively, mostly, generally*

## Faux positifs connus

Deux motifs se déclenchent encore en v0.2 : les termes entre guillemets
droits (`"many X"` sans backticks) et `"many X"` où X est un nom
concret. Les deux sont suivis sous **[F23](../../roadmap.md)** dans la
[feuille de route](../../roadmap.md). Entourer le terme cité de
backticks, ou utiliser un commentaire de neutralisation inline, pour
opter hors de la règle.

## Neutralisation

Utiliser `<!-- lucid-lint disable-next-line lexicon.weasel-words -->`
quand le mot fuyant est intentionnel (citation, référence légitime à
un sous-ensemble, méta-discussion). Voir
[Neutraliser des diagnostics](../../guide/suppression.md) (page EN
pour l'instant).

## Références

- [Strunk & White (1999)](../references.md#strunk-white-1999)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

Voir [Références](../references.md) pour la bibliographie complète.
