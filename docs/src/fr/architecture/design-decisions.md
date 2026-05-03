<!-- en-source-sha: 9179cb7bc93b77150cabde79bca19b3ec0edce8f -->
# Décisions de conception

Cette page consigne les décisions de conception prises pendant v0.1 qui méritent d'être revues avant tout changement.

## Modèle linter contre modèle de score

**Décision** : v0.1 a livré la forme classique de linter, avec les sévérités `info` / `warning`. v0.2 a ajouté un modèle hybride de score (score global + sous-scores par catégorie + diagnostics) par-dessus, sans retirer la forme linter.

**Raison** : livrer la forme linter d'abord nous a permis de valider la qualité de détection sur de vrais corpus avant d'ajouter la couche d'agrégation. La couche de score est additive — les outils qui ne s'intéressent qu'aux diagnostics ignorent le scorecard.

## Modèle hybride de score (v0.2)

**Décision** : un score global + 5 sous-scores par catégorie, tous sous la forme `X / max`. La composition empile une somme pondérée, une normalisation par densité (par 1 000 mots, plancher à 200) et un plafond par catégorie. 5 catégories figées : `Structure · Rhythm · Lexicon · Syntax · Readability`. Nouveau champ `Diagnostic.weight`, nouvelle option `--min-score=N` en ligne de commande.

**Raison** (brainstorm complet dans [`brainstorm/20260420-score-semantics.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/brainstorm/20260420-score-semantics.md)) :

- `X / max` plutôt que 0–100 : un maximum arbitraire nous laisse réajuster sans prétendre que le 80 d'aujourd'hui est le 80 de la prochaine version. La compétence `/impeccable` utilise déjà cette convention.
- 5 catégories figées : ne couplent rien à un renommage de règle ; utilisent l'aide `category_of(rule_id)` déjà décidée en v0.1. Dériver depuis le préfixe (plan B) a été rejeté : il aurait fallu renommer 17 règles rien que pour [F14](../roadmap.md).
- Trois mécaniques de composition empilées : aucune seule ne couvre tous les modes de défaillance. La densité seule punit les courts documents ; les poids seuls perdent face à une règle qui s'emballe ; les plafonds seuls ne reflètent pas l'ampleur du coût.
- Notes en lettres, feux tricolores, marge réussite/échec et secondes de lecture ont été coupés du design v0.2 après une analyse à partir des principes de base ([F-score-letter-grade](../roadmap.md#f-score-letter-grade)–[F41](../roadmap.md) dans ROADMAP). Ils dupliquent la fonction-1 (vue d'un coup d'œil) que le nombre remplit déjà.
- L'actionnabilité (fonction-2) est portée par la liste des diagnostics, pas par le score. Les sous-scores peuvent donc se permettre d'être minimaux — [F37](../roadmap.md) veille à ce que les messages de diagnostic tiennent le côté actionnable du contrat.

## Structure `Diagnostic`

**Décision** : un `Diagnostic` porte `rule_id`, `severity`, `location`, `section`, `message` et (depuis v0.2) `weight`.

**Ce qui n'est PAS stocké, et pourquoi** :

- **`category`** — dérivable depuis `rule_id` via `Category::for_rule`. La stocker dupliquerait l'information et créerait un risque de dérive.
- **`suggestion`** — toujours différée ; les messages actuels sont actionnables par eux-mêmes.

**Ce qui EST stocké, et pourquoi** :

- **`section`** — la recalculer après coup demanderait de reparser le document pour parcourir les titres et faire correspondre les positions. Le coût de stockage est une `Option<String>` par diagnostic ; le coût de recalcul est un second parsing complet.
- **`weight`** (v0.2) — initialisé à l'émission depuis `scoring::default_weight_for`, pour que les surcharges utilisatrices (par configuration) et les surcharges au niveau règle (par `with_weight`) traversent l'agrégation sans seconde recherche.

## Cœur déterministe, extensions pour le reste

**Décision** : le cœur ne livre que des règles déterministes. Les règles à base de LLM, les règles qui s'appuient sur le réseau ou les règles à base de modèle d'apprentissage vivent dans des caisses d'extension facultatives (prévues pour v0.3).

**Raison** : un hook pre-commit qui prend 5 secondes et varie d'une exécution à l'autre est pire que pas de hook du tout. Le déterminisme n'est pas négociable sur le chemin nominal.

## Bilingue EN/FR dès le premier jour

**Décision** : chaque règle qui dépend de la langue gère l'anglais et le français depuis v0.1.

**Raison** : la plupart des développeurs francophones de l'open source écrivent leur documentation en anglais. Viser le français seul passerait à côté de la majorité. Gérer les deux dès le premier jour coûte peu et signale l'ambition.

## Une seule formule de lisibilité en v0.1

**Décision** : v0.1 utilise le grade Flesch-Kincaid pour toutes les langues. Les formules par langue (Kandel-Moles pour le français, SMOG, Coleman-Liau) sont différées à v0.2.

**Raison** : Flesch-Kincaid est connue, reproductible et bien comprise. Ajouter trois formules avant de valider les bases serait une optimisation prématurée.

## Markdown + texte brut + entrée standard, Pandoc pour le reste

**Décision** : prise en charge native de `.md`, `.markdown`, `.txt` et de l'entrée standard en v0.1. Les autres formats (AsciiDoc, HTML, docx, PDF) passent par Pandoc en pré-traitement.

**Raison** : Markdown couvre la grande majorité de l'écriture open-source et technique. Pandoc est libre, omniprésent, et lève la charge de maintenir plusieurs parseurs.

## Un fichier par règle

**Décision** : chaque règle vit dans son propre fichier sous `src/rules/`, avec une structure cohérente (struct, config, impl `Rule`, tests).

**Raison** : ajouter une règle devient une opération bien définie (un nouveau fichier depuis un gabarit), et la revue est facile (une règle, une PR, un fichier à lire).

## Heuristique des mots vides pour la détection de langue

**Décision** : v0.1 détecte la langue par le ratio de mots vides. Aucune dépendance externe.

**Raison** : court, déterministe, sans coût à l'exécution. Pour les cas où elle échoue (textes très courts, documents pleins de code), la valeur de repli `unknown` est sûre.

## Préréglages de profil comme variantes d'énumération

**Décision** : les profils sont `Profile::DevDoc | Public | Falc`. Ils ne peuvent pas être définis dans la configuration de l'utilisateur en v0.1.

**Raison** : ajouter des profils personnalisés est une abstraction spéculative tant que personne ne le demande. Les surcharges par règle suffisent à couvrir 95 % des cas « je veux un préréglage légèrement différent ».

## Références à consulter avant de changer

- [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) — la référence des règles qui fait foi
- [`ROADMAP.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md) — les travaux à venir
- [`CODING_STANDARDS.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/CODING_STANDARDS.md) — les conventions du quotidien
