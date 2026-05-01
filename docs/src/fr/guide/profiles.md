<!-- en-source-sha: c621ab2c0f7f8e0fa1d6e0ac1974450570cbd2ac -->
# Profils

Un profil est un ensemble pré-configuré de seuils de règles, ajusté pour un public précis.

## Profils disponibles

### `dev-doc`

Pour la documentation technique, les références d'API, les ADR et le contenu destiné aux développeurs.

Les seuils sont permissifs. Les lecteurs techniques tolèrent mieux les phrases longues, les nominalisations et le jargon de domaine.

### `public` (par défaut)

Pour le contenu grand public : pages marketing, descriptions produit, articles de blog.

Les seuils sont modérés. Les principes du langage clair s'appliquent.

### `falc`

Pour le contenu qui suit le standard *Facile À Lire et à Comprendre* / Easy-to-Read européen.

Les seuils sont stricts : phrases courtes, vocabulaire simple, pas de voix passive, pas d'acronyme non défini.

## Choisir un profil

Commencez par le profil qui correspond à l'intention du contenu. Surchargez les règles individuelles si besoin via `lucid-lint.toml`.

## Comparaison des seuils

Voir la [référence des règles](../rules/index.md) pour les seuils exacts par règle et par profil.

Le schéma général :

- `dev-doc` : 30 mots par phrase, 4 virgules, 7 phrases par paragraphe
- `public` : 22 mots par phrase, 3 virgules, 5 phrases par paragraphe
- `falc` : 15 mots par phrase, 2 virgules, 3 phrases par paragraphe

Le même fichier analysé trois fois sous `dev-doc`, `public` puis `falc` — le score baisse à mesure que le profil se resserre :

![Capture terminal : trois exécutions successives de lucid-lint sur examples/sample.md sous les profils dev-doc, public et falc. Le passage dev-doc remonte une poignée de diagnostics et un score moyen ; public se resserre et plus de problèmes apparaissent ; falc en signale le plus et le score chute davantage](../assets/tty/profiles.gif)

## Surcharger un profil

Tout seuil défini par règle dans `lucid-lint.toml` prend le pas sur le préréglage du profil.

```toml
[default]
profile = "public"

[rules.sentence-too-long]
max_words = 18   # plus strict que les 22 de public
```
