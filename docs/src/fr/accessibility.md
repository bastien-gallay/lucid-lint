<!-- en-source-sha: 9179cb7bc93b77150cabde79bca19b3ec0edce8f -->
# Accessibilité

> **Traduction en cours.** La page d'accessibilité détaillée est
> pour l'instant [disponible en anglais](../accessibility.html). Sa
> traduction FR est suivie dans **F25** sur la
> [feuille de route](./roadmap.md).

En résumé : le site vise **WCAG 2.2 niveau AAA**. Il dogfoode
`lucid-lint` sur sa propre prose. Les contrastes, tailles,
raccourcis clavier et la compatibilité avec les lecteurs d'écran
sont testés à chaque livraison.

## Écarts connus

Premier audit complet le 2026-04-22 : **17 / 20**, 0 bloquant.

- Le lien « Aller au contenu principal » et le sélecteur EN / FR
  sont ajoutés par JavaScript en fin de page. Un rendu côté
  serveur via `theme/index.hbs` est prévu
  ([F35a](./roadmap.md)).

## Signaler un défaut d'accessibilité

Ouvrez une
[issue sur GitHub](https://github.com/bastien-gallay/lucid-lint/issues/new)
avec le label `accessibility`. Les signalements sont traités sur le
jalon v0.2, sauf s'ils bloquent une publication.
