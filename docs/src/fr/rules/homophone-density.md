<!-- en-source-sha: 9d0836e3c64ed3730215acef4ec4ec1e1a0bcfea -->
# `lexicon.homophone-density`

*Densité d'homophones trop élevée.*

> *Expérimentale en v0.2.x.* Désactivée par défaut ; activez-la via
> `--experimental lexicon.homophone-density` ou
> `[experimental] enabled = ["lexicon.homophone-density"]` dans
> `lucid-lint.toml`. Passe à `Stable` au moment du tag v0.3 dans le
> cadre de la cohorte
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status).
> Voir [Conditions](../guide/conditions.md) pour les tags `dyslexia` et
> `aphasia` qui gouvernent cette règle selon les conditions actives.

## Ce que la règle détecte

Les paragraphes dont la part d'homophones — des mots qui se prononcent pareil mais s'écrivent différemment (`their` / `there` / `they're`, `to` / `too` / `two`, `cours` / `court`, `amande` / `amende`) — dépasse un pourcentage configurable. Les homophones imposent une double passe : l'oreille reconnaît le mot, l'œil doit ensuite choisir la bonne orthographe via le contexte. Ce détour est anodin isolément, coûteux en grappe. Le guide de la British Dyslexia Association cite les homophones comme un point de friction connu pour la lecture dyslexique, et les recommandations FALC d'orthographe claire conseillent de reformuler les passages denses pour les lecteurs aphasiques et les publics « facile à lire ».

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Statut** | `experimental` (v0.2.x) → `stable` au tag v0.3 |
| **Tags de condition** | `dyslexia`, `aphasia` (gouvernés ; ne s'exécute qu'avec `--conditions` correspondant) |
| **Langues** | EN · FR (listes d'homophones spécifiques à chaque langue) |
| **Source** | [`src/rules/lexicon/homophone_density.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/lexicon/homophone_density.rs) |

## Détection

Pour chaque paragraphe, parcourt le flux de mots une fois, compte les mots alphabétiques au dénominateur, et compte comme « occurrences » les mots qui apparaissent dans la table d'homophones de la langue. Si `occurrences / total` dépasse strictement le seuil du profil, émet un diagnostic ancré sur la première ligne du paragraphe. Les paragraphes de moins de 20 mots de contenu sont ignorés — sous ce plancher, un seul homophone produit un pourcentage à deux chiffres trompeur. Le message du diagnostic cite jusqu'à deux exemples d'homophones effectivement rencontrés, pour que la localisation reste le paragraphe mais que les pistes de réécriture soient concrètes.

Les tables d'homophones (`HOMOPHONE_GROUPS_EN`, `HOMOPHONE_GROUPS_FR` dans `src/language/`) privilégient des paires de mots-contenu dont la confusion orthographique altère vraiment le sens. Les homophones-outils français très fréquents (`et` / `est`, `a` / `à`, `ou` / `où`) sont volontairement exclus : ils apparaissent dans presque toutes les phrases et feraient grimper la densité de référence au-dessus de tous les seuils, noyant le signal que la règle veut capter.

Quand la langue détectée est `Unknown`, la règle n'a pas de table à appliquer et s'abstient silencieusement plutôt que de deviner.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_density_percent` | `float` | 8.0 | 5.0 | 3.0 |

Pour ajuster via `lucid-lint.toml` :

```toml
[rules."lexicon.homophone-density"]
max_density_percent = 4.0
```

## Exemples

### Anglais

**Avant** (signalé) :

> Their report shows there were too many decisions to make and two teams could not affect the launch nor lose the schedule despite careful planning across each region and product line every quarter.

Ce que `lucid-lint check --profile public --experimental lexicon.homophone-density --conditions dyslexia` rapporte :

```text
warning input.md:1:1 Paragraph density of homophones is 21.2% (7 of 33 content words (e.g. their, there)); maximum 5.0%. Dense homophone runs raise the phonological-decoding load for dyslexic and aphasic readers; rephrase to disambiguate. [lexicon.homophone-density]
```

**Après** (réécriture proposée) :

> The report shows that the team made many decisions and that the two squads kept the launch on schedule despite careful planning across each region and product line every quarter.

La réécriture remplace `their` / `there` / `to` / `too` / `two` par des tournures ancrées dans le contexte (`the report`, `that`, `the team`, `kept`, `the two squads`), faisant tomber la densité bien sous le seuil.

### Français

**Avant** (signalé) :

> Pendant le cours du matin la cuisinière prépare le foie de veau avant la pause de midi puis revient à sa tâche après avoir rangé les ustensiles sur la grande table en bois clair.

Ce que `lucid-lint check --profile public --experimental lexicon.homophone-density --conditions dyslexia` rapporte :

```text
warning input.md:1:1 Paragraph density of homophones is 11.8% (4 of 34 content words (e.g. cours, foie)); maximum 5.0%. Dense homophone runs raise the phonological-decoding load for dyslexic and aphasic readers; rephrase to disambiguate. [lexicon.homophone-density]
```

**Après** (réécriture proposée) :

> Pendant la séance du matin la cuisinière prépare le foie de veau avant la coupure de midi puis reprend son travail après avoir rangé les ustensiles sur la grande table en bois clair.

`cours` devient `séance`, `pause` devient `coupure`, `tâche` devient `travail` — trois des quatre occurrences disparaissent sans perte de sens.

## Suppression

Voir [Supprimer un diagnostic](../guide/suppression.md) pour les formes inline et bloc. La directive inline fonctionne sur cette règle :

```markdown
<!-- lucid-lint disable-next-line lexicon.homophone-density -->
Their report shows there were too many decisions to make and two teams could not lose the launch.
```

## Voir aussi

- [Conditions](../guide/conditions.md) — les tags `dyslexia` et `aphasia` qui gouvernent cette règle.
- [F-experimental-rule-status — statut expérimental des règles](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrat qui permet à cette règle d'arriver en v0.2.x sans affecter les scores par défaut.

## Références

- [British Dyslexia Association — Dyslexia Style Guide (2018)](https://www.bdadyslexia.org.uk/advice/employers/creating-a-dyslexia-friendly-workplace). Cite les homophones comme point de friction pour la lecture dyslexique.
- [FALC — Information pour tous (2009)](../references.md). Recommandations d'orthographe claire pour les publics aphasiques et « facile à lire ».

Voir [Références](../references.md) pour la bibliographie complète.
