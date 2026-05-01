<!-- en-source-sha: 9594202969efbf001af153da9d37f18bb0786e18 -->
# Supprimer des diagnostics

`lucid-lint` propose deux directives en ligne pour faire taire des diagnostics dans les entrées Markdown. Elles servent aux cas rares où une règle se déclenche sur de la prose intentionnelle (un terme vague cité, un exemple didactique de nominalisation lourde, une voix passive légitime). Préférez réécrire la prose d'abord. Sortez une directive quand la détection est un faux positif connu, ou quand l'autrice a vu l'avertissement et choisi de garder le texte.

## Forme « ligne »

```markdown
<!-- lucid-lint disable-next-line structure.sentence-too-long -->

Une phrase longue qui est intentionnelle et ne doit pas être signalée.
```

- **Syntaxe.** Commentaire HTML, un identifiant de règle par directive. Plusieurs directives ligne peuvent précéder la même ligne cible.
- **Portée.** La prochaine ligne non vide dans la source.
- **Raison facultative.** `<!-- lucid-lint disable-next-line lexicon.weasel-words reason="citation du guide de style" -->` — surfacée dans la sortie JSON ; sera *exigée* via configuration dans une version future (suivi par [F20](../roadmap.md) sur la [feuille de route](../roadmap.md)).

## Forme « bloc » (v0.2, F18)

```markdown
<!-- lucid-lint-disable structure.sentence-too-long -->

Une phrase longue.

Une autre phrase longue dans la même portée.

<!-- lucid-lint-enable -->
```

- **Ouverture.** `<!-- lucid-lint-disable <rule-id> -->` ouvre une portée pour une règle.
- **Fermeture.** `<!-- lucid-lint-enable -->` ferme **toutes** les portées en cours. Passer un identifiant de règle (`<!-- lucid-lint-enable <rule-id> -->`) ne ferme que la portée de cette règle, ce qui permet d'imbriquer proprement des désactivations chevauchantes pour des règles différentes.
- **Portée.** Toutes les lignes entre les deux commentaires (incluses).
- **Désactivation non fermée.** S'étend jusqu'à la fin du document — utile pour un opt-out sur un fichier entier, mais préférez la directive `disable-file` planifiée ([F21](../roadmap.md)) dès qu'elle arrive.
- **Une règle par commentaire.** Les listes multi-règles sont suivies par F21.

## Propriétés communes

- **S'applique au Markdown uniquement.** Le texte brut et l'entrée standard ne peuvent pas porter de commentaires HTML. Les ignorés par configuration (`[[ignore]]` dans `lucid-lint.toml`) couvrant `.txt` et l'entrée standard sont suivis par **[F19](../roadmap.md)**.
- **Les identifiants de règle inconnus sont silencieusement ignorés.** Cela rend les directives compatibles d'une version de lint à une autre.
- **Les diagnostics supprimés ne coûtent rien au score.** Les modèles de suppression et de [score](./scoring.md) sont cohérents — faire taire un diagnostic le retire de la somme pondérée. Aucune double pénalité cachée.

## Différé

Les extensions suivantes sont suivies sur la [feuille de route](../roadmap.md) :

| ID | Élément |
|---|---|
| [F19](../roadmap.md) | Ignorés par configuration (`[[ignore]]` dans `lucid-lint.toml`) pour les entrées `.txt` et l'entrée standard |
| [F20](../roadmap.md) | Champ `reason="..."` facultatif puis exigé, surfacé dans les rapports |
| [F21](../roadmap.md) | Directive niveau fichier (`disable-file`) et listes multi-règles séparées par virgule |

## Voir aussi

- [Configuration](./configuration.md) — seuils TOML et surcharges de profil.
- [Score](./scoring.md) — comment les diagnostics supprimés influent sur les scores global et par catégorie.
- Notes spécifiques par règle sur les cas où une suppression est idiomatique — voir la section `## Suppression` sur chaque page de règle dans la [référence des règles](../rules/index.md).
