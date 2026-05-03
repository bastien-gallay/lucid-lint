<!-- en-source-sha: fd56c3b0fde4f323d70feff084fd1c62a9d15f41 -->
# Vue d'ensemble de l'architecture

`lucid-lint` est une petite caisse Rust avec un pipeline volontairement simple.

## Pipeline

```
 texte d'entrée
     │
     ▼
┌──────────────────────────┐
│ Détection de la langue   │   heuristique du ratio de mots vides
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Parseur                  │   pulldown-cmark ou texte brut
│ (Markdown | brut)        │
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Modèle de document       │   Section > Paragraphe > Phrase
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Règles                   │   Chaque règle reçoit le document + la langue
│ (sentence-too-long, ...) │
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Diagnostics              │   rule_id, severity, location, section,
│                          │   message, weight
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐     v0.2+
│ Score                    │   normalisé par densité, plafonné par catégorie
│ (Scorecard)              │   5 catégories figées
└─────────────┬────────────┘
              │
              ▼
┌──────────────────────────┐
│ Formateur de sortie      │   TTY (défaut) ou JSON
│                          │   — porte les diagnostics + le scorecard
└──────────────────────────┘
```

## Types clés

- **`Diagnostic`** — l'unité de sortie. Porte `weight` (initialisé depuis `scoring::default_weight_for`) depuis v0.2.
- **`Rule`** (trait) — `fn check(document, language) -> Vec<Diagnostic>`.
- **`Document`** — la sortie du parseur. Consciente des sections.
- **`Scorecard`** — `global: Score`, plus `[CategoryScore; 5]` dans l'ordre figé `Structure · Rhythm · Lexicon · Syntax · Readability`.
- **`Report`** — `diagnostics + scorecard + word_count`, renvoyé par `Engine::lint_*` depuis v0.2.
- **`Engine`** — regroupe un profil, un jeu de règles et une `ScoringConfig` facultative ; expose `lint_str`, `lint_file`, `lint_stdin`.

## Principes de conception

Ces principes sont appliqués en revue de code. Voir [Décisions de conception](./design-decisions.md) pour le contexte.

1. **Rendre les états impossibles impossibles** — types neufs, énumérations avec données, `NonZeroU32`.
2. **Style fonctionnel** où il aide — chaînes d'itérateurs, fonctions de règle pures.
3. **Règles atomiques** — une règle, un signal.
4. **Cœur déterministe** — ni réseau, ni LLM, ni comportement dépendant de l'environnement.
5. **YAGNI** — pas d'abstractions spéculatives.

## Disposition des modules

```
src/
├── lib.rs             — racine de la bibliothèque
├── main.rs            — point d'entrée du binaire
├── cli.rs             — CLI clap
├── config.rs          — préréglages de profil, lecture du fichier de configuration
├── engine.rs          — orchestration
├── language/          — détection + données par langue
├── parser/            — Markdown + texte brut + tokeniseur + modèle de document
├── rules/             — un fichier par règle
├── scoring.rs         — modèle hybride de score (v0.2+)
├── output/            — formateurs TTY + JSON
└── types.rs           — types métier (Diagnostic, Severity, Location, ...)
```
