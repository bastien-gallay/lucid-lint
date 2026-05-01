<!-- en-source-sha: 9e6be0c432915d364d877b302e70186928303088 -->
# `lexicon.consonant-cluster`

*Amas consonantiques.*

## Ce que cette règle signale

Les mots dont la plus longue suite de consonnes consécutives atteint
ou dépasse un seuil par profil. Les amas consonantiques denses sont
une barrière de décodage connue pour les lecteurs dyslexiques (*BDA
Dyslexia Style Guide*) : le lecteur doit retenir plus de phonèmes en
mémoire de travail avant que la voyelle suivante « libère » la
syllabe.

Exemples typiques en anglais au seuil `public` de 5 :
`strengths` (n-g-t-h-s), `twelfths` (l-f-t-h-s), `sixths` (x-t-h-s
sur 4 + contexte). Exemples typiques en français au seuil `falc`
de 4 : `constructions` (n-s-t-r).

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Étiquettes de condition** | `dyslexia`, `general` |
| **Langues** | EN · FR |
| **Source** | [`src/rules/consonant_cluster.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/consonant_cluster.rs) |

## Détection

Par ligne source, on parcourt le flux de graphèmes une seule fois.
Un mot est une suite maximale de caractères alphabétiques ; les
traits d'union, apostrophes et espaces ferment le mot (ainsi
`dys-lexique` compte pour deux mots, pas un amas de dix lettres).
À l'intérieur d'un mot, on suit la plus longue suite de consonnes
consécutives. Un diagnostic est émis par mot dont la plus longue
suite atteint `min_run_length`.

Les voyelles sont sensibles à la langue — les formes accentuées
françaises (`é`, `è`, `ê`, `à`, `â`, `î`, `ï`, `ô`, `ö`, `ù`, `û`,
`ü`, `ÿ`, `œ`, `æ`) comptent comme des voyelles. Le repli anglais
accepte les voyelles latin-1 accentuées courantes pour que les
emprunts (`café`, `naïve`) soient décodés correctement. Le `y` est
traité comme une voyelle dans toutes les langues (clémence), ce qui
évite des faux positifs gênants sur des mots comme `fly`, `rhythm`.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_run_length` | `int` | 6 | 5 | 4 |

`dev-doc` est tolérant — la prose technique nomme régulièrement des
choses comme `strengths` ou `benchmarks`. `falc` (audience grand
public) attrape toute suite de 4 consonnes.

## Cas connus

- La règle est aveugle à la structure syllabique : elle compte les
  graphèmes consonantiques bruts, pas les phonèmes. Un mot comme
  `hatching` (5 lettres : t-c-h-n-g — suite de 5) se lit fluidement
  pour la plupart des lecteurs parce que `tch` est un seul digramme
  anglais. Neutraliser via directive inline quand un hit est
  inévitable.
- Agnostique pour tout script alphabétique, mais les listes de
  voyelles ne sont calibrées que pour les scripts latins. Les mots
  en cyrillique, grec, arabe, etc., déclencheront probablement dès
  que le drapeau de langue est `en` ou `fr` — en pratique ce
  contenu sort du périmètre d'un linter bilingue EN/FR.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`lexicon.all-caps-shouting`](./all-caps-shouting.md)
- [`readability.score`](../../rules/readability-score.md) (page EN
  pour l'instant)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)

## Références

- [Seidenberg et al. (1984)](../references.md#seidenberg-1984)
- [Treiman et al. (2006)](../references.md#treiman-2006)

Voir [Références](../references.md) pour la bibliographie complète.
