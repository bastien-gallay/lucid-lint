<!-- en-source-sha: 3afe9b59984a266abbd99dc06f8925aa474bca95 -->
# Intégration CI

`lucid-lint` est conçu pour la CI. Il renvoie :

- `0` quand aucun problème (ou seulement `info`) n'est trouvé
- `1` quand des warnings sont trouvés
- `2` sur erreur d'exécution (arguments invalides, fichier illisible)

## GitHub Actions

```yaml
name: Docs lint

on:
  pull_request:
    paths:
      - '**/*.md'
  push:
    branches: [main]
    paths:
      - '**/*.md'

jobs:
  lucid-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install lucid-lint
        run: cargo install lucid-lint
      - name: Lint docs
        run: lucid-lint check --profile=public docs/ README.md
```

## Pre-commit

À ajouter dans votre `.pre-commit-config.yaml` :

```yaml
repos:
  - repo: local
    hooks:
      - id: lucid-lint
        name: lucid-lint
        entry: lucid-lint check --profile=public
        language: system
        types: [markdown]
```

## Reviewdog

Pour faire remonter les diagnostics en commentaires de revue de pull request :

```bash
lucid-lint check --format=json docs/ | reviewdog -f=rdjson -reporter=github-pr-review
```

> Note : l'adaptateur RDJSON n'est pas livré. Pour une remontée native dans la revue de code, préférez le flux [GitHub Code Scanning](#github-code-scanning-sarif) ci-dessous.

## GitHub Code Scanning (SARIF)

`--format=sarif` émet un journal SARIF v2.1.0 que GitHub Code Scanning lit directement : chaque diagnostic devient une alerte de code-scanning, annotée sur le diff de la pull request.

```yaml
name: Lucid lint (code scanning)

on:
  pull_request:
    paths: ['**/*.md']
  push:
    branches: [main]
    paths: ['**/*.md']

permissions:
  security-events: write
  contents: read

jobs:
  lucid-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install lucid-lint
      - name: Run lucid-lint and emit SARIF
        run: |
          lucid-lint check \
            --profile=public \
            --format=sarif \
            --fail-on-warning=false \
            docs/ README.md > lucid-lint.sarif
      - name: Upload SARIF to Code Scanning
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: lucid-lint.sarif
          category: lucid-lint
```

Notes :

- `--fail-on-warning=false` laisse l'étape d'upload toujours s'exécuter ; reposez-vous sur les gardes de Code Scanning dans l'UI de la pull request, plutôt que sur le code de sortie du linter.
- Chaque règle apparaît une fois dans `runs[0].tool.driver.rules`, avec sa catégorie, sa sévérité par défaut, son poids de score par défaut, et un `helpUri` qui pointe vers la page mdBook de la règle.
- Sur chaque résultat, `properties.weight` et `properties.section` portent le poids de score et le titre de section sous lequel le diagnostic a été trouvé.

## Contrôle du code de sortie

Pour ne pas faire échouer la CI sur des warnings (par exemple pendant une phase d'adoption progressive), vous pouvez inverser le défaut :

```bash
lucid-lint check --fail-on-warning=false docs/
```

L'exécution renvoie alors toujours 0, sauf en cas d'erreur d'exécution.

## Bloquer sur le score

Vous pouvez aussi bloquer le build sur le [modèle de score](./scoring.md) agrégé. L'exécution sort `1` si le score global est sous le seuil, indépendamment de la garde par sévérité.

```yaml
jobs:
  lucid-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install lucid-lint
      - name: Lint and gate on score
        run: lucid-lint check --min-score=85 docs/ README.md
```

Les deux gardes s'empilent — l'exécution échoue si *l'une ou l'autre* se déclenche. Choisissez la combinaison adaptée à votre courbe d'adoption :

| Objectif | Options |
|---|---|
| Attraper les warnings nouvellement introduits (comportement par défaut) | par défaut |
| Tolérer des warnings isolés mais échouer sur la dérive | `--fail-on-warning=false --min-score=85` |
| Échouer sur les pics et la dérive | par défaut + `--min-score=85` |

Une exécution bloquée qui échoue — lucid-lint imprime son résumé habituel, puis le shell expose le code de sortie non nul :

![Capture terminal : une exécution lucid-lint sur examples/sample.md avec --min-score=85, qui produit trois warnings, deux diagnostics info, un score de 45/100, et une ligne « exit: 1 » écrite par la commande echo qui suit](../assets/tty/score-fail.gif)

```text
$ lucid-lint check --min-score=85 examples/sample.md
…
score: 45/100
       structure    █▎░░░  5/20
       rhythm       █████  20/20
       lexicon      █▎░░░  5/20
       syntax       ██▌░░  10/20
       readability  █▎░░░  5/20
$ echo "exit: $?"
exit: 1
```
