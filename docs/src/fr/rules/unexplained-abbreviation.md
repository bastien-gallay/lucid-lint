# `lexicon.unexplained-abbreviation`

## Ce que cette règle signale

Les acronymes employés sans définition proche. Chaque interruption
forcée pour deviner ou chercher un acronyme casse le fil et augmente
le risque de perdre l'attention.

**Références.** WCAG 2.1 SC 3.1.4 (Abréviations) ; RGAA 9.4.

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (listes blanches distinctes) |
| **Source** | [`src/rules/unexplained_abbreviation.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/unexplained_abbreviation.rs) |

## Détection (v0.2, deux passes — F9)

1. **Pré-scan** du document entier pour repérer les acronymes définis
   sous l'une ou l'autre forme canonique :
   - `Expansion complète (ACRONYME)` — exemple : `World Wide Web (WWW)`
   - `ACRONYME (Expansion complète)` — exemple : `WWW (World Wide Web)`

   Le côté « expansion » doit contenir au moins deux mots alphabétiques,
   pour que des notes courtes entre parenthèses comme `(TBD)` ou
   `(à vérifier)` ne soient pas comptées comme définitions.

2. **Appariement** des séquences de 2 lettres capitales consécutives
   ou plus (optionnellement avec des chiffres) dans le texte principal.
3. **Filtrage** de chaque candidat par trois couches, dans l'ordre :
   1. Défini dans le document (issu du pré-scan) — le plus fort.
   2. Liste blanche utilisateur `[rules.unexplained-abbreviation].whitelist`.
   3. Liste blanche de base (pilotée par le profil).
4. **Signalement** de chaque occurrence restante.

Une seule définition n'importe où dans le document fait taire chaque
occurrence du même acronyme — ce qui correspond à la manière dont les
lecteurs utilisent réellement la documentation (remonter une fois pour
trouver l'expansion, la retenir ensuite).

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `min_length` | `int` | 3 | 2 | 2 |
| `whitelist` | `list` | étendue | minimale | vide |

**Liste blanche par défaut (v0.2, resserrée par F31)** : la pile
d'infrastructure — `URL, HTML, CSS, JSON, XML, HTTP, HTTPS, UTF, IO,
API, CLI, GUI, OS, CPU, RAM, SSD, USB, IDE, SDK, CI, CD` — plus les
acronymes FR/EN courants et les mots-clés d'emphase RFC 2119
(`PDF, SMS, GPS, ID, OK, FAQ`, `MUST, SHALL, SHOULD, …`).

**Ce qui a changé en v0.2 :** les standards d'accessibilité
(`WCAG`, `ARIA`, `RGAA`, …), les initiales du champ IA / traitement
des langues (`LLM`, `NLP`) et les acronymes de pratiques d'ingénierie
(`YAGNI`, `DRY`, `TDD`, …) ne sont plus dans la liste blanche de base
livrée. Les projets qui les utilisent doivent les ajouter à
`[rules.unexplained-abbreviation].whitelist` dans `lucid-lint.toml` —
voir le [guide de configuration](../../guide/configuration.md#per-rule-overrides).

```toml
[rules.unexplained-abbreviation]
whitelist = ["WCAG", "ARIA", "ADHD", "LLM"]
```

Les entrées de la liste blanche utilisateur sont **additives** par
rapport à la liste de base — elles l'étendent, jamais ne la
remplacent.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`lexicon.jargon-undefined`](../../rules/jargon-undefined.md) —
  l'équivalent pour les mots de contenu.
