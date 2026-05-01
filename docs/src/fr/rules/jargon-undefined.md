<!-- en-source-sha: 8400741a215a89c5e49eeff68e7194d9c643b350 -->
# `lexicon.jargon-undefined`

*Jargon non défini.*

## Ce que cette règle signale

Les termes spécialisés employés sans définition. Le jargon est
contextuel : acceptable entre spécialistes, exclusif autrement. Comme
les acronymes, le jargon impose des interruptions de lecture au
non-spécialiste ; à la différence des acronymes, ce sont des mots de
contenu, pas des séquences en majuscules.

**Références.** *Plain Language* (US), FALC, WCAG 2.1 SC 3.1.3
(*Mots inhabituels*).

## En bref

| | |
|---|---|
| **Catégorie** | `lexicon` |
| **Sévérité par défaut** | `warning` |
| **Poids par défaut** | `1` |
| **Langues** | EN · FR (listes distinctes par langue et par domaine) |
| **Source** | [`src/rules/jargon_undefined.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/jargon_undefined.rs) |

## Détection

1. Plusieurs listes de jargon par domaine sont maintenues (`tech`,
   `legal`, `medical`, `admin`).
2. L'utilisateur active les listes pertinentes via le profil.
3. Chaque occurrence d'un terme listé est signalée.

## Activation par profil

| Profil | Listes actives |
|---|---|
| `dev-doc` | aucune (les développeurs maîtrisent leur propre jargon) |
| `public` | `tech`, `legal`, `medical`, `admin` |
| `falc` | `tech`, `legal`, `medical`, `admin`, mode strict |

## Configuration

En v0.2, les listes actives sont fixées par le profil et ne sont
pas encore surchargées depuis `lucid-lint.toml`. Les surcharges
TOML par règle — ajouter des termes de domaine, neutraliser des
entrées précises, ou activer une combinaison de listes différente
du profil — sont suivies sous **[F126](../roadmap.md)** dans la
[feuille de route](../roadmap.md).

## Listes de départ par défaut (contributions bienvenues)

- **Tech :** *idempotent, orthogonal, deterministic, polymorphic,
  serialization, deserialization, synchronous, asynchronous,
  concurrency, thread-safe, side-effect, referential transparency,
  memoization, currying, hoisting, closure, monad, immutable,
  stateless, refactoring*
- **Juridique (surtout FR) :** *apériteur, clause résolutoire, force
  majeure, cessation de paiement, préjudice subi, onéreux,
  nonobstant, préalablement, susmentionné, infra, supra, ad hoc,
  de facto, in fine, subséquemment*
- **Médical :** *anamnèse, étiologie, pathognomonique, iatrogène,
  nosocomial, décompensation, récidive, rémission, syndromique*
- **Administratif (surtout FR) :** *attributaire, solliciter,
  diligenter, instruction du dossier, pièces justificatives,
  circulaire, délibération, arrêté préfectoral, transmission des
  pièces, ayant droit*

## Neutralisation

Voir [Neutralisation des diagnostics](../../guide/suppression.md) (page
EN pour l'instant).

## Voir aussi

- [`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md)

## Références

- [WCAG 2.1 — 3.1.3](../references.md#wcag-2-1)
- [Plain Language US (2011)](../references.md#plain-language-us-2011)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

Voir [Références](../references.md) pour la bibliographie complète.
