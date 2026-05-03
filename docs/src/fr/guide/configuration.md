<!-- en-source-sha: 158ff69e11e3510a010b221eb8bf8da0597504d4 -->
# Configuration

`lucid-lint` se configure par un fichier `lucid-lint.toml` à la racine du projet (facultatif) et par des options en ligne de commande (qui priment sur le fichier).

## Forme du fichier

```toml
# lucid-lint.toml

[default]
profile = "public"

[rules.sentence-too-long]
max_words = 22

[rules.passive-voice]
max_per_paragraph = 2
```

## Sections

### `[default]`

Réglages par défaut appliqués à toute l'exécution.

| Champ | Type | Défaut | Description |
|---|---|---|---|
| `profile` | chaîne | `"public"` | Une valeur parmi `dev-doc`, `public`, `falc` |
| `conditions` | tableau de chaînes | `[]` | Étiquettes de condition actives. Voir [Conditions](./conditions.md). |
| `exclude` | tableau de motifs glob | `[]` | Chemins ignorés pendant la descente récursive. Voir [Exclure des chemins](#exclure-des-chemins). |

### `[rules.<rule-id>]`

Configuration par règle. Les champs disponibles dépendent de la règle. Voir les pages de règles dans la [référence des règles](../rules/index.md).

### `[scoring]`

Paramètres ajustables du [modèle hybride de score](./scoring.md). Tous les champs sont facultatifs ; un champ absent retombe sur le défaut livré (`category_max = 20`, `category_cap = 15`).

```toml
[scoring]
category_max = 20
category_cap = 15

[scoring.weights]
sentence-too-long = 3
weasel-words      = 2
```

La sous-table `[scoring.weights]` est indexée par identifiant de règle. Les identifiants inconnus sont ignorés ; retirer une règle dans une version future ne casse donc pas les anciens fichiers.

## Ordre de priorité

Du plus faible au plus fort :

1. Préréglage du profil (par exemple `public`)
2. Surcharges de `lucid-lint.toml`
3. Options en ligne de commande

Une option non passée en ligne de commande retombe sur la valeur TOML ; un champ TOML absent retombe sur le préréglage du profil.

## Découverte

`lucid-lint` remonte depuis le dossier courant jusqu'au premier `lucid-lint.toml` trouvé, et s'arrête à la frontière du dépôt `.git` le plus proche. L'option `--config <chemin>` saute la découverte et charge le fichier indiqué ; un chemin explicite manquant est une erreur, mais un fichier auto-découvert manquant ne l'est pas.

## Exclure des chemins

Les gros dépôts de documentation contiennent souvent des sorties générées, des textes vendus avec le projet et des instantanés qui noieraient le linter sous le bruit. Utilisez le champ `exclude` dans `[default]` — ou l'option `--exclude <GLOB>` en ligne de commande — pour les écarter à la découverte, avant l'analyse.

```toml
[default]
exclude = [
    "vendor/**",
    "**/fixtures/**",
    "CHANGELOG.md",
]
```

L'équivalent en ligne de commande :

```bash
lucid-lint check --exclude 'vendor/**,**/fixtures/**,CHANGELOG.md' docs
```

Notes :

- **Mise en correspondance.** Les motifs glob s'appliquent au chemin **relatif à la racine parcourue**. Lancer `lucid-lint check docs` avec `exclude = ["drafts/**"]` ignore `docs/drafts/...`.
- **Élaguer, ne pas visiter.** Un dossier qui correspond n'est pas parcouru — les gros arbres exclus ne coûtent rien à traverser.
- **Les fichiers nommés explicitement passent quand même.** Si vous passez `docs/CHANGELOG.md` directement en ligne de commande, il est analysé même quand `CHANGELOG.md` est dans la liste d'exclusion. Si vous le nommez, c'est que vous le voulez.
- **Additif.** L'option `--exclude` et le champ TOML `exclude` se cumulent ; ils ne se remplacent pas. Séparez plusieurs motifs par des virgules dans une option, ou répétez `--exclude`.

## Faire taire des règles globalement

Les documents Markdown acceptent des [directives de désactivation en ligne](./suppression.md) pour faire taire localement, mais le texte brut et l'entrée standard n'ont pas cette porte de sortie. `[[ignore]]` comble le manque — et fonctionne pareil sur tous les formats d'entrée.

```toml
[[ignore]]
rule_id = "unexplained-abbreviation"

[[ignore]]
rule_id = "weasel-words"
```

Chaque entrée `[[ignore]]` retire tous les diagnostics dont le `rule_id` correspond, dans les fichiers Markdown, le texte brut et l'entrée standard. Le filtre s'applique après l'exécution de toutes les règles, mais avant le score, donc le score reflète la vue post-filtre.

Notes :

- **Portée globale.** Le filtre n'est pas par fichier. Les directives en ligne restent la porte de sortie recommandée pour faire taire ponctuellement en Markdown — utilisez `[[ignore]]` seulement quand une règle est vraiment bruyante sur tout le projet.
- **Identifiants inconnus tolérés.** Les entrées qui visent des règles disparues sont retirées sans rien dire, donc retirer une règle dans une version future ne casse pas les anciens fichiers.
- **Champs futurs.** Un champ `reason = "..."` sur chaque entrée est suivi par [F-suppression-reason-field](../roadmap.md#f-suppression-reason-field) — quand il arrivera, il sera affiché dans les rapports et exigible par configuration.

## Surcharges par règle

La configuration TOML est branchée règle par règle, à mesure que chaque `Config` reçoit son accesseur dédié. Deux règles l'honorent aujourd'hui :

### `[rules.readability-score]`

```toml
[rules.readability-score]
formula = "kandel-moles"  # ou "flesch-kincaid", "auto"
```

Fixe la formule de lisibilité, quelle que soit la langue détectée. `auto` (défaut) garde la sélection par langue de [F-readability-formulas-extra](../roadmap.md#f-readability-formulas-extra).

### `[rules.unexplained-abbreviation]`

```toml
[rules.unexplained-abbreviation]
whitelist = ["WCAG", "ARIA", "ADHD", "LLM"]
```

Les entrées sont **additives** par rapport à la base du profil (F31). Utilisez ce champ pour réintroduire des sigles propres au projet — normes d'accessibilité, sigles métier, termes de pratique d'ingénierie — que la base de v0.2 ne livre plus. Chaque entrée fait taire le sigle dans tout le document, comme si vous l'aviez défini en ligne par `Expansion (ACRONYME)`.

### `[rules."structure.excessive-commas"]`

```toml
[rules."structure.excessive-commas"]
max_commas = 2
```

Surcharge le plafond de virgules par phrase (défaut : 4 / 3 / 2 pour `dev-doc` / `public` / `falc`). La valeur doit être un entier positif — `0` ou une valeur négative est refusée au chargement. La surcharge remplace le préréglage du profil ; elle n'est pas additive.

Les tables pour les autres règles se lisent sans erreur, mais n'ont pas d'effet à l'exécution. Étendre cette liste est un changement mécanique par règle, qui se poursuivra pendant le cycle v0.2.x.
