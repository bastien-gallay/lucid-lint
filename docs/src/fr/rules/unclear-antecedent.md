# `syntax.unclear-antecedent`

*Antécédent flou.*

## Ce que cette règle signale

Les pronoms dont l'antécédent n'est pas évident dans le contexte
immédiat. La référence pronominale ambiguë est l'une des ruptures de
compréhension les plus coûteuses pour les lecteurs souffrant de
troubles attentionnels : chaque ambiguïté force un retour conscient
pour chercher l'antécédent.

**Références.** Strunk & White ; FALC (« préférer la répétition du
nom au pronom ») ; Graesser et al. *Coh-Metrix* (cohésion
référentielle).

## En bref

| | |
|---|---|
| **Catégorie** | `syntax` |
| **Sévérité par défaut** | `info` |
| **Poids par défaut** | `2` |
| **Langues** | EN · FR (listes de pronoms distinctes) |
| **Source** | [`src/rules/unclear_antecedent.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/unclear_antecedent.rs) |

## Détection (heuristique v0.1)

<!-- lucid-lint disable-next-line structure.mixed-numeric-format -->

La détection exacte demande une résolution d'anaphore (problème
avancé de traitement automatique du langage). v0.1 attrape les deux
motifs les plus fréquents :

1. Pronoms démonstratifs en début de phrase (`This`/`That`/`These`/
   `Those`, `Ceci`/`Cela`/`Ce`) **non** suivis d'un nom.
2. Pronoms personnels en début de paragraphe (aucun antécédent dans
   le contexte précédent).

La sévérité est `info` parce que l'heuristique est approximative —
le niveau de bruit justifie une sévérité douce.

## Paramètres

| Clé | Type | Défaut |
|---|---|---|
| `check_demonstratives` | `bool` | `true` |
| `check_paragraph_start_pronouns` | `bool` | `true` |

## Listes de pronoms

- 🇫🇷 *ce, cela, ceci, ça, celui-ci, celle-ci, il, elle, ils, elles*
- 🇬🇧 *this, that, these, those, it, they, them*

## Exemple

> Les performances étaient médiocres avec le cache LRU. **Cela** a
> motivé le changement.

À quoi renvoie *cela* ? Aux performances ? Au cache ? Ambigu.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Références

- [Strunk & White (1999)](../references.md#strunk-white-1999)
- [Gibson (1998)](../references.md#gibson-1998)
- [Graesser et al. (2004)](../references.md#graesser-2004)

Voir [Références](../references.md) pour la bibliographie complète.
