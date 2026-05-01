<!-- en-source-sha: 5dbf67551b6fa17b85f329c937213827fa2e4639 -->
# Conditions

Une **étiquette de condition** décrit la condition cognitive qu'une règle vise en priorité. Les conditions sont *orthogonales* aux profils : un profil (`dev-doc`, `public`, `falc`) règle la sévérité des règles toujours actives ; les conditions ajoutent des règles ciblées pour un public précis.

## L'ontologie figée

| Étiquette | Cible |
|---|---|
| `general` | Règles toujours actives. La base de v0.2. |
| `a11y-markup` | Signaux de balisage proches de la prose (par exemple le cri en majuscules). |
| `dyslexia` | Signaux ciblant la dyslexie. Source : BDA Dyslexia Style Guide. |
| `dyscalculia` | Format des nombres et points d'ancrage. Source : CDC Clear Communication Index. |
| `aphasia` | Signaux ciblant l'aphasie. Source : FALC, guides en langage clair. |
| `adhd` | Signaux liés à la fragilité de l'attention. |
| `non-native` | Signaux pour lectrices et lecteurs non natifs (mots rares, expressions imagées). |

L'ensemble est figé. Ajouter une étiquette est un choix réfléchi et versionné.

## Comment le filtrage fonctionne

Pour chaque règle, le moteur évalue :

- Une règle marquée `general` est **toujours active**.
- Une règle **sans** `general` ne tourne que si une de ses étiquettes apparaît dans la liste de conditions actives de la personne qui lance l'outil.

Les 17 règles de v0.2 portent toutes `general`, donc le comportement par défaut ne change pas. Les futures règles étiquetées (par exemple `lexicon.all-caps-shouting` pour `a11y-markup`, `syntax.nested-negation` pour `aphasia` + `adhd`) s'activent par cette liste.

## Configurer les conditions

Dans `lucid-lint.toml` :

```toml
[default]
profile = "falc"
conditions = ["dyslexia", "aphasia"]
```

En ligne de commande (séparées par des virgules, répétables) :

```bash
lucid-lint check --profile falc --conditions dyslexia,aphasia docs/
```

FALC garde son sens réglementaire. Ajouter `dyslexia` ne le relâche pas et ne le renomme pas — la condition pose des signaux dyslexie par-dessus.

## Pourquoi des étiquettes, pas des profils parallèles

Trois niveaux de sévérité × N conditions explose en combinaisons. Garder les deux axes orthogonaux préserve le sens réglementaire de `falc` tout en laissant composer des couches dédiées à un public. Voir les entrées F71 et F72 sur la [feuille de route](../roadmap.md).
