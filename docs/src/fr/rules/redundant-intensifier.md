<!-- en-source-sha: 9e6be0c432915d364d877b302e70186928303088 -->
# `lexicon.redundant-intensifier`

*Intensificateurs redondants.*

## Ce que cette règle signale

Les intensificateurs — adverbes qui tentent de *renforcer* la
confiance d'une affirmation sans rien y ajouter en information.
`très important` se réduit à `important`, ou mieux, à une assertion
chiffrée. plainlanguage.gov (chapitre 4) et le *CDC Clear
Communication Index* signalent les intensificateurs comme un
anti-motif de langue claire.

La règle est le pendant délibéré de
[`lexicon.weasel-words`](./weasel-words.md) : les mots évasifs
*affaiblissent* la confiance (atténuations, qualifications) ; les
intensificateurs redondants la *renforcent*. Les deux listes sont
disjointes par construction.

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Étiquettes de condition** | `general` |
| **Langues** | EN · FR |
| **Source** | [`src/rules/redundant_intensifier.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/redundant_intensifier.rs) |

## Détection

Par paragraphe, le texte est mis en minuscules puis chaque
intensificateur de la liste par langue
([`en::INTENSIFIERS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/en.rs),
[`fr::INTENSIFIERS`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/language/fr.rs))
est cherché via la recherche partagée à frontières de mot. Les hits
à l'intérieur d'un span de code (clôturé ou inline) sont ignorés.
Les documents dont la langue est `Unknown` sont ignorés plutôt que
devinés, par parallèle avec `lexicon.weasel-words`.

## Paramètres

| Clé | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `custom_intensifiers_en` | `list<string>` | `[]` | `[]` | `[]` |
| `custom_intensifiers_fr` | `list<string>` | `[]` | `[]` | `[]` |
| `disable` | `list<string>` | `[]` | `[]` | `[]` |

`custom_intensifiers_en` / `_fr` ajoutent des locutions aux défauts.
`disable` retire des locutions de ces défauts (correspondance exacte
en minuscules).

## Cas connus

- `très` dans la formule figée `très bien` (comme acquiescement)
  déclenche tout de même — les guides de langue claire le signalent
  quand même, et la règle ne taille pas d'exception. Neutraliser via
  une directive inline si le contexte l'impose vraiment.
- Les références métalinguistiques (« le mot 'très' est un
  intensificateur ») déclenchent sauf si le mot cible est entre
  backticks. Utiliser un span de code inline pour ce genre de
  référence.

## Neutralisation

Voir [Neutraliser des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`lexicon.weasel-words`](./weasel-words.md)
- [`lexicon.jargon-undefined`](./jargon-undefined.md)
- [Conditions](../../guide/conditions.md) (page EN pour l'instant)

## Références

- [Strunk & White (1999)](../references.md#strunk-white-1999)
- [Quirk et al. (1985)](../references.md#quirk-1985)
- [Zinsser (2006)](../references.md#zinsser-2006)

Voir [Références](../references.md) pour la bibliographie complète.
