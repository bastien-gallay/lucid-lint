# `syntax.passive-voice`

*Voix passive.*

## Ce que cette règle signale

Les constructions à la voix passive. La passive masque l'agent et
allonge la phrase sans ajouter d'information. Des exceptions
légitimes existent (agent inconnu, style scientifique, mise en
relief volontaire de l'action) — la règle signale, l'auteur décide.

**Références.** US Plain Language ; Strunk & White ; FALC.

## En bref

| | |
|---|---|
| **Catégorie** | `syntax` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `2` |
| **Langues** | EN · FR (heuristiques distinctes) |
| **Source** | [`src/rules/passive_voice.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/passive_voice.rs) |

## Détection (heuristique v0.1)

- 🇬🇧 `be` (conjugué) + participe passé `[+ by …]`. Gère le `-ed`
  régulier et la table des participes irréguliers.
- 🇫🇷 `être` (conjugué) + participe passé `[+ par …]`, plus
  `se faire + infinitif`. Plus difficile qu'en anglais à cause de
  l'accord du participe (genre/nombre) et de la confusion avec
  (a) l'attribut du sujet (`il est content` vs `il est vu`) et
  (b) l'auxiliaire `être` des temps composés (`elle est partie` —
  passé composé, actif).

Précision attendue ~70–80 %. Un remplaçant à base d'analyseur
morphosyntaxique est prévu pour un futur greffon `lucid-lint-nlp`.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_per_paragraph` | `int` | 3 | 1 | 0 |
| `ignore_scientific_style` | `bool` | `false` | `false` | `false` |

## Neutralisation

Pour les passives volontaires, utiliser une directive inline. Voir
[Neutraliser des diagnostics](../../guide/suppression.md) (page EN
pour l'instant).

## Références

- [Strunk & White (1999)](../references.md#strunk-white-1999)
- [Plain Language US (2011)](../references.md#plain-language-us-2011)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

Voir [Références](../references.md) pour la bibliographie complète.
