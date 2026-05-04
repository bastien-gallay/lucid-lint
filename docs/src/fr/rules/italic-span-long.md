<!-- en-source-sha: a5a9e52823cafc1e3a88470bcfc74ef580b083f7 -->
# `structure.italic-span-long`

*Phrase en italique trop longue.*

> *Expérimentale en v0.2.x.* Désactivée par défaut ; activez-la via
> `--experimental structure.italic-span-long` ou
> `[experimental] enabled = ["structure.italic-span-long"]` dans
> `lucid-lint.toml`. Passe à `Stable` au moment du tag v0.3 dans le
> cadre de la cohorte
> [F-experimental-rule-status](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status).
> Voir [Conditions](../guide/conditions.md) pour le tag `dyslexia` qui
> gouverne cette règle selon les conditions actives.

## Ce que la règle détecte

Les spans italiques (`*…*` / `_…_`) qui dépassent un seuil de mots configurable. Les glyphes inclinés gênent la reconnaissance des formes de lettres pour les personnes dyslexiques — un constat solide qui motive la recommandation de la British Dyslexia Association : garder l'italique pour de courtes phrases plutôt que pour des passages entiers. Les longs passages en italique nuisent aussi au repérage visuel pour tout lecteur dont l'attention est déjà sollicitée (fatigue, lecture en seconde langue, basse vision).

## En bref

| | |
|---|---|
| **Catégorie** | `structure` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Statut** | `experimental` (v0.2.x) → `stable` au tag v0.3 |
| **Tag de condition** | `dyslexia` (gouverné ; ne s'exécute qu'avec `--conditions` correspondant) |
| **Langues** | EN · FR (détection identique — le substrat est agnostique) |
| **Source** | [`src/rules/structure/italic_span_long.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/structure/italic_span_long.rs) |

## Détection

Parcourt l'arbre inline typé attaché à chaque `Paragraph` (substrat [F143](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f143)) et signale tout span `Inline::Emphasis` dont le nombre de mots visibles dépasse le seuil du profil. Les blocs de code et le code inline sont exclus par le parseur ; un italique dans un bloc de code ne déclenche jamais la règle. Le gras (`**bold**`) ne déclenche pas non plus cette règle — seul l'italique (`*italique*` / `_italique_`).

La position du diagnostic pointe sur le délimiteur *d'ouverture* : le surlignage dans votre éditeur se place sur le `*` ou `_` visible, pas sur une colonne arbitraire dans le paragraphe.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_words` | `int` | 12 | 8 | 5 |

Pour ajuster via `lucid-lint.toml` :

```toml
[rules."structure.italic-span-long"]
max_words = 6
```

## Exemples

### Anglais

**Avant** (signalé) :

> The team eventually concluded that *the proposed migration plan would require careful coordination across three regional offices and an extended freeze window* before any deployment could begin.

Ce que `lucid-lint check --profile public --experimental structure.italic-span-long --conditions dyslexia` rapporte :

```text
warning input.md:1:36 Italic span is 17 words long (maximum 8). Long italic runs strain dyslexic readers; consider shortening the emphasized phrase or removing the italics. [structure.italic-span-long]
```

**Après** (réécriture proposée) :

> The team eventually concluded that the proposed migration plan would require careful coordination. Three regional offices and an extended freeze window are *prerequisites* before any deployment.

L'italique marque maintenant un seul mot porteur — l'usage que le guide BDA recommande.

### Français

**Avant** (signalé) :

> L'équipe a fini par conclure que *le plan de migration proposé nécessiterait une coordination soignée entre trois bureaux régionaux et une fenêtre de gel prolongée* avant tout déploiement.

Ce que `lucid-lint check --profile public --experimental structure.italic-span-long --conditions dyslexia` rapporte :

```text
warning input.md:1:35 Italic span is 18 words long (maximum 8). Long italic runs strain dyslexic readers; consider shortening the emphasized phrase or removing the italics. [structure.italic-span-long]
```

**Après** (réécriture proposée) :

> L'équipe a fini par conclure que le plan de migration nécessiterait une coordination soignée. Trois bureaux régionaux et une fenêtre de gel prolongée sont *indispensables* avant tout déploiement.

## Suppression

Voir [Supprimer un diagnostic](../guide/suppression.md) pour les formes inline et bloc. La directive inline fonctionne sur cette règle :

```markdown
<!-- lucid-lint disable-next-line structure.italic-span-long -->
Une *phrase volontairement longue en italique que la règle signalerait normalement* est ici.
```

## Voir aussi

- [Conditions](../guide/conditions.md) — le tag `dyslexia` qui gouverne cette règle.
- [F-experimental-rule-status — statut expérimental des règles](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f-experimental-rule-status) — substrat qui permet à cette règle d'arriver en v0.2.x sans affecter les scores par défaut.
- [F143 — couche AST inline](https://github.com/bastien-gallay/lucid-lint/blob/main/ROADMAP.md#f143) — substrat qui expose les bornes des spans d'emphase à cette règle.

## Références

- [British Dyslexia Association — Dyslexia Style Guide (2018)](https://www.bdadyslexia.org.uk/advice/employers/creating-a-dyslexia-friendly-workplace). Recommande de garder l'italique pour de courtes phrases afin de préserver la reconnaissance des formes de lettres.
- [Rello & Baeza-Yates (2013)](../references.md#rello-baeza-yates-2013) — contexte académique plus large sur la typographie favorable à la dyslexie.

Voir [Références](../references.md) pour la bibliographie complète.
