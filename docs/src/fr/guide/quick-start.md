<!-- en-source-sha: 3afe9b59984a266abbd99dc06f8925aa474bca95 -->
# Démarrage rapide

Cette page suit l'analyse de votre premier document.

## Analyser un seul fichier

```bash
lucid-lint check README.md
```

Sortie :

```text
warning <path>/README.md:14:1 Sentence is 27 words long (maximum 22). Consider splitting it into shorter sentences. [structure.sentence-too-long]

summary: 1 warnings.
→ run 'lucid-lint explain <rule-id>' — seen here: structure.sentence-too-long
────────────────────────────────────────────────────────────
score: 88/100
       structure    ██▏░░  8/20
       rhythm       █████  20/20
       lexicon      █████  20/20
       syntax       █████  20/20
       readability  █████  20/20
```

Le bloc final est le résumé du [score](./scoring.md). Il affiche un score global `X / 100` puis le détail par catégorie.

## Analyser plusieurs fichiers

```bash
lucid-lint check docs/*.md CHANGELOG.md
```

## Analyser un dossier

```bash
lucid-lint check docs/
```

Tous les fichiers avec une extension `.md`, `.markdown` ou `.txt` seront traités.

## Utiliser l'entrée standard

```bash
echo "This is a test sentence." | lucid-lint check -
```

## Recevoir depuis Pandoc

Pour les formats que `lucid-lint` ne sait pas encore lire nativement :

```bash
pandoc report.docx -t markdown | lucid-lint check -
```

## Choisir un profil

```bash
# Le plus strict : Facile À Lire et à Comprendre
lucid-lint check --profile=falc docs/

# Le plus permissif : documentation pour développeurs
lucid-lint check --profile=dev-doc docs/
```

Voir [Profils](./profiles.md) pour le détail.

## Changer le format de sortie

```bash
# JSON pour l'intégration continue
lucid-lint check --format=json docs/
```

Voir [Intégration continue](./ci-integration.md) pour les recettes CI.

## Codes de sortie

| Code | Signification |
|---|---|
| 0 | Aucun problème (ou seulement des `info`) et score au-dessus de `--min-score` (si défini) |
| 1 | Avertissements détectés **ou** score sous `--min-score` |
| 2 | Erreur d'exécution (arguments invalides, fichier illisible) |

Les deux portes se combinent. Voir [Intégration continue](./ci-integration.md#gating-on-score) pour les recettes combinées.
