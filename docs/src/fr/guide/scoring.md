<!-- en-source-sha: a5a9e52823cafc1e3a88470bcfc74ef580b083f7 -->
# Score

v0.2 ajoute un **modèle hybride de score** par-dessus les diagnostics existants. Chaque exécution répond désormais à deux questions à la fois :

1. *Qu'est-ce qui ne va pas, précisément ?* — la liste des diagnostics, inchangée depuis v0.1.
2. *À quel point ce document est mauvais dans l'ensemble ?* — un nouveau score global, plus cinq sous-scores par catégorie.

Les deux surfaces sont complémentaires. Les scores sont des résumés ; les diagnostics restent le signal sur lequel agir.

## Ce que le score signifie

Le score prend la forme `X / max` — un maximum arbitraire, pas un nombre normalisé sur 0–100. v0.2 livre `max = 100` (cinq catégories × vingt points), mais ce nombre est traité comme un calibrage à tester et apprendre : l'échelle peut bouger dans une future version mineure, à mesure que les poids des règles sont ajustés sur de vrais corpus.

Les règles d'usage pour le calibrage du jour :

| Plage | Lecture |
|---|---|
| 80 – 100 | Le score s'affiche en vert dans le terminal. Rien de bloquant. |
| 60 – 79 | Le score s'affiche en jaune. Quelques signalements à passer en revue. |
| 0 – 59 | Le score s'affiche en rouge. Problèmes denses ou règle qui s'emballe. |

Les bandes de couleur aident la lecture ; elles ne sont pas un contrat de réussite ou d'échec. Pour bloquer la CI, utilisez [`--min-score`](#bloquer-la-ci-avec---min-score) avec un nombre concret que vous avez choisi.

## Les cinq catégories

Chaque règle appartient à exactement une catégorie. v0.2 fige la taxonomie en cinq cases :

| Catégorie | Couvre |
|---|---|
| `structure` | Longueur, imbrication, ponctuation, squelette du document |
| `rhythm` | Cadence et répétition entre phrases voisines |
| `lexicon` | Vocabulaire, terminologie, sigles, diversité lexicale |
| `syntax` | Style et clarté au niveau de la phrase |
| `readability` | Métriques de lisibilité au niveau du document |

Voir la [référence des règles](../rules/index.md) pour la correspondance règle → catégorie.

## Comment un score est calculé

Pour un seul document :

```text
coût_par_règle      = Σ (poids × multiplicateur_de_sévérité)   sur les hits
coût_par_catégorie  = min(Σ coût_par_règle / (mots / 1000),    ← densité
                          category_cap)                         ← plafond
score_de_catégorie  = category_max − coût_par_catégorie         (borné ≥ 0)
score_global        = Σ score_de_catégorie
```

Trois mécaniques s'empilent :

- **Somme pondérée** — chaque hit coûte `poids × multiplicateur_de_sévérité`. La table de poids par défaut vit dans `scoring::default_weight_for` ; elle insiste sur les règles dont les hits portent la plus grosse charge cognitive (`readability-score = 5`, longueur / subordination / passive / unclear-antecedent = 2, le reste = 1).
- **Normalisation par densité** — les coûts sont divisés par `mots / 1000`, pour qu'un manuel de 10 000 mots ne soit pas puni d'avoir plus de hits qu'un README de 400 mots. Les documents de moins de 200 mots sont traités comme des documents de 200 mots ; les petites fixtures ne sont donc pas pénalisées artificiellement.
- **Plafond par catégorie** — aucune catégorie ne peut perdre plus de `category_cap` sur `category_max`. Une règle bruyante mange au plus 75 % de sa propre catégorie (15 / 20 par défaut), et ne déborde pas sur les autres.

Le multiplicateur de sévérité est `info = 1`, `warning = 3`, `error = 5`.

## Lire la sortie TTY

Le formateur de terminal imprime chaque diagnostic, une courte ligne de résumé, puis un bloc de score : le nombre global, suivi de chaque score de catégorie avec une barre sparkline en huit pas.

![lucid-lint exécuté sur examples/sample.md — cinq diagnostics, un résumé qui compte 3 warnings et 2 info, une invite à utiliser explain, et un bloc de score qui affiche 45/100 avec des barres par catégorie pour structure, rhythm, lexicon, syntax et readability](../assets/tty/hero.gif)

La même exécution rendue en texte brut, pour les lecteurs d'écran et le copier-coller :

```text
warning examples/sample.md:7:1 Sentence is 35 words long (maximum 30). Consider splitting it into shorter sentences. [section: A paragraph with a long sentence] [structure.sentence-too-long]
warning examples/sample.md:7:11 Weasel phrase "rather" weakens the statement. Replace with concrete language or remove it. [section: A paragraph with a long sentence] [lexicon.weasel-words]
info    examples/sample.md:1:1 Flesch-Kincaid grade 6.8 (target ≤ 14.0). [readability.score]
info    examples/sample.md:7:1 Sentence starts with a bare demonstrative "this". Name the referent to avoid forcing the reader to guess. [section: A paragraph with a long sentence] [syntax.unclear-antecedent]
warning examples/sample.md:7:1 Line is 210 characters wide (maximum 120). [section: A paragraph with a long sentence] [structure.line-length-wide]

summary: 3 warnings, 2 info.
→ run 'lucid-lint explain <rule-id>' — seen here: structure.sentence-too-long, lexicon.weasel-words, readability.score + 2 more
────────────────────────────────────────────────────────────
score: 45/100
       structure    █▎░░░  5/20
       rhythm       █████  20/20
       lexicon      █▎░░░  5/20
       syntax       ██▌░░  10/20
       readability  █▎░░░  5/20
```

Les cinq catégories sont toujours affichées, pour que le découpage reste structurellement stable d'une exécution à l'autre. Un document parfait affiche `score: 100/100` avec toutes les barres pleines (`█████`). Quand la même règle se déclenche deux fois ou plus dans un fichier, les hits se groupent sous un en-tête compact, et le message ou la section partagés sont remontés pour n'apparaître qu'une fois.

## Lire la sortie JSON

Le schéma JSON est en `version = 2` dans v0.2. Nouveaux champs :

```json
{
  "version": 2,
  "diagnostics": [
    {
      "rule_id": "structure.sentence-too-long",
      "severity": "warning",
      "location": { "file": { "kind": "path", "path": "draft.md" }, "line": 12, "column": 1, "length": 42 },
      "section": "Introduction",
      "message": "Sentence is 27 words long (maximum 22).",
      "weight": 2
    }
  ],
  "summary": { "info": 0, "warning": 1, "error": 0, "total": 1 },
  "score": { "value": 88, "max": 100 },
  "category_scores": [
    { "category": "structure",   "value": 8,  "max": 20 },
    { "category": "rhythm",      "value": 20, "max": 20 },
    { "category": "lexicon",     "value": 20, "max": 20 },
    { "category": "syntax",      "value": 20, "max": 20 },
    { "category": "readability", "value": 20, "max": 20 }
  ]
}
```

Les valeurs de catégorie sont des chaînes minuscules, dans l'ordre fixe listé plus haut. Les outils qui lisaient le schéma v0.1 doivent :

- passer leur `version` attendue de `1` à `2` ;
- remplacer les anciens noms de catégorie (`length` → `structure`, `lexical` → `lexicon`, `style` → `syntax`, `global` → `readability`) ;
- ignorer les champs inconnus, pour qu'un futur ajout au schéma ne les casse pas.

## Bloquer la CI avec `--min-score`

La sous-commande `check` accepte une option facultative `--min-score=N`. L'exécution sort `1` si le score global agrégé est sous `N`, indépendamment du blocage par sévérité.

```bash
# Échoue le build si la qualité globale tombe sous 85/100
lucid-lint check --min-score=85 docs/
```

Les deux gardes s'empilent : l'exécution échoue si *l'une ou l'autre* se déclenche. Choisissez l'une, l'autre ou les deux selon votre flux :

- **Garde par sévérité seule** (comportement v0.1) : attrape les warnings nouvellement introduits, ne réagit pas à une dérive lente.
- **Garde par score seule** (`--fail-on-warning=false --min-score=85`) : tolère des warnings isolés, mais échoue quand la densité dépasse votre seuil.
- **Les deux** (défaut + `--min-score=85`) : pics et dérives échouent tous les deux le build.

## Ajuster les poids dans `lucid-lint.toml`

Les projets peuvent surcharger le calibrage dans leur `lucid-lint.toml` :

```toml
[scoring]
category_max = 20
category_cap = 15

[scoring.weights]
sentence-too-long = 3
weasel-words      = 2
```

Les champs absents retombent sur les défauts livrés. La sous-table `[scoring.weights]` est indexée par identifiant de règle ; les identifiants inconnus sont ignorés, donc retirer une règle plus tard ne casse pas les anciens fichiers.

## Ce qui est différé

Le brainstorm qui a façonné [F14](../roadmap.md) (voir [`brainstorm/20260420-score-semantics.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/brainstorm/20260420-score-semantics.md)) a gardé le modèle minimal. Les décorations ne seront promues que si les retours utilisateurs l'exigent :

- **Notes en lettres (A–F)** — suivi par [F-score-letter-grade](../roadmap.md#f-score-letter-grade). Promu si les nombres semblent bruyants ou difficiles à comparer entre documents.
- **Affichage feu tricolore + marge réussite/échec** — suivi par [F-score-traffic-light](../roadmap.md#f-score-traffic-light). Promu si les utilisateurs CI demandent un signal d'un coup d'œil plus fort.
- **Secondes de lecture comme unité alternative** — suivi par [F-reading-time-score](../roadmap.md#f-reading-time-score). Demande une heuristique validée et des métriques compagnes (confort, fatigue), pour ne pas monopoliser la lecture.
- **Sous-scores par section** — suivi par [F-section-scoring](../roadmap.md#f-section-scoring). Une fois les agrégats document + projet éprouvés sur le terrain.
- **Agrégat multi-fichiers au niveau projet** — suivi par [F-project-scoring-rollup](../roadmap.md#f-project-scoring-rollup). En v0.2, la CLI traite tous les chemins passés comme un seul document pour le score.
