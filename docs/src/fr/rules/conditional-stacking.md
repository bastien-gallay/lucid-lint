<!-- en-source-sha: 9e6be0c432915d364d877b302e70186928303088 -->
# `syntax.conditional-stacking`

*Empilement de conditions.*

## Ce que cette règle signale

Les phrases qui enchaînent plusieurs propositions conditionnelles.
Chaque `if` / `when` / `unless` / `quand` / `si` ouvre une branche
que le lecteur doit garder en pile mentale jusqu'à la résolution de
la proposition englobante. Deux ou trois empilées dans une même
phrase forment un multiplicateur de charge connu. L'effet touche les
lecteurs avec aphasie, trouble du déficit de l'attention (TDAH) et
toute personne sous pression cognitive. Les guides de langage clair
(FALC, plainlanguage.gov) recommandent de scinder les chaînes
conditionnelles en phrases distinctes ou en liste à puces.

## En bref

| | |
|---|---|
| **Catégorie** | `syntax` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `2` |
| **Étiquettes de condition** | `aphasia`, `adhd`, `general` |
| **Langues** | EN · FR (listes spécifiques par langue) |
| **Source** | [`src/rules/conditional_stacking.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/conditional_stacking.rs) |

## Détection

Par phrase, on compte les connecteurs conditionnels et on signale les
comptes au-dessus de `max_conditionals`.

- **Anglais** — somme des correspondances délimitées par mot contre
  la liste de langue (`if`, `unless`, `when`, `whenever`, `while`,
  `until`, `provided`, `assuming`, `in case`, `as long as`,
  `as soon as`, `even if`, `only if`).
- **Français** — somme des correspondances délimitées par mot contre
  la liste de langue (`si`, `sauf si`, `à moins que`, `à moins de`,
  `quand`, `lorsque`, `lorsqu'`, `dès que`, `tant que`, `pourvu que`,
  `à condition que`, `à condition de`, `au cas où`, `même si`,
  `en cas de`) plus les clitiques élidés `s'il` / `s'ils`.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_conditionals` | `int` | 3 | 2 | 1 |

## Exemples

Trois conditions, teintes assorties d'un bout à l'autre de la
réécriture — la position les appariait déjà, la couleur confirme
que la réécriture conserve chaque branche. `lucid-lint` signale ;
la réécriture reste à l'auteur.

### Français

**Avant** (signalée) :

> <span class="lucid-idea" data-idea="1">Si nous expédions,</span> <span class="lucid-idea" data-idea="2">quand le test passe,</span> <span class="lucid-idea" data-idea="3">à moins que la barrière échoue,</span> nous déployons.

Trois connecteurs conditionnels (`si`, `quand`, `à moins que`).

Ce que rapporte `lucid-lint check --profile public` :

```text
warning input.md:1:1 Sentence stacks 3 conditional clauses (maximum 2). Split the conditions across separate sentences or convert them to a bullet list. [syntax.conditional-stacking]
```

**Après** (votre réécriture) :

> Nous déployons quand les trois conditions tiennent :
>
> - <span class="lucid-idea" data-idea="1">la commande d'expédition a tourné,</span>
> - <span class="lucid-idea" data-idea="2">le test passe,</span>
> - <span class="lucid-idea" data-idea="3">la barrière n'échoue pas.</span>

### Anglais

**Avant** (signalée) :

> <span class="lucid-idea" data-idea="1">If we ship,</span> <span class="lucid-idea" data-idea="2">when the build passes,</span> <span class="lucid-idea" data-idea="3">unless the gate fails,</span> we deploy.

**Après :**

> We deploy when all three checks hold:
>
> - <span class="lucid-idea" data-idea="1">the ship command ran,</span>
> - <span class="lucid-idea" data-idea="2">the build passes,</span>
> - <span class="lucid-idea" data-idea="3">the gate does not fail.</span>

## Faux positifs connus

La liste anglaise mêle des conditionnels purs avec des conjonctions
temporelles (`when`, `while`) qui peuvent introduire des
sous-propositions à valeur conditionnelle. Un usage purement temporel
peut produire un faux positif sur des phrases longues. Utiliser
[`disable-next-line`](../../guide/suppression.md) (page EN pour
l'instant) quand la lecture temporelle est sans ambiguïté.

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`syntax.nested-negation`](./nested-negation.md)
- [`structure.deep-subordination`](./deep-subordination.md)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)

## Références

- [Johnson-Laird & Byrne (1991)](../references.md#johnson-laird-byrne-1991)
- [Evans & Over (2004)](../references.md#evans-over-2004)
- [Gibson (1998)](../references.md#gibson-1998)

Voir [Références](../references.md) pour la bibliographie complète.
