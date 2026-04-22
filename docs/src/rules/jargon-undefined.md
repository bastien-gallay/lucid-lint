# `jargon-undefined`

## What it flags

Domain-specific terms used without definition. Jargon is contextual: acceptable among specialists, exclusionary otherwise. Like acronyms, jargon creates reading interruptions for the non-specialist; unlike acronyms, these are content words, not uppercase sequences.

**References.** US Plain Language, FALC, WCAG 2.1 SC 3.1.3 (Unusual Words).

## At a glance

| | |
|---|---|
| **Category** | `lexicon` |
| **Default severity** | `warning` |
| **Default weight** | `1` |
| **Languages** | EN · FR (separate lists per language and domain) |
| **Source** | [`src/rules/jargon_undefined.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/jargon_undefined.rs) |

## Detection (v0.1 simplified)

1. Maintain multiple jargon lists per domain (`tech`, `legal`, `medical`, `admin`).
2. User activates the relevant lists via profile.
3. Flag each occurrence of a listed term.

A definition-aware variant is tracked as part of **[F9](../roadmap.md)** on the [roadmap](../roadmap.md).

## Parameters

| Key | Type | Default |
|---|---|---|
| `jargon_lists` | `list` | profile-dependent |
| `custom_jargon` | `list` | `[]` |
| `whitelist` | `list` | `[]` |

## Profile activation

| Profile | Lists active |
|---|---|
| `dev-doc` | none (developers understand their own jargon) |
| `public` | `tech`, `legal`, `medical`, `admin` |
| `falc` | `tech`, `legal`, `medical`, `admin`, strict mode |

## Default starter lists (v0.1 — community contributions welcome)

- **Tech:** *idempotent, orthogonal, deterministic, polymorphic, serialization, deserialization, synchronous, asynchronous, concurrency, thread-safe, side-effect, referential transparency, memoization, currying, hoisting, closure, monad, immutable, stateless, refactoring*
- **Legal (mostly FR):** *apériteur, clause résolutoire, force majeure, cessation de paiement, préjudice subi, onéreux, nonobstant, préalablement, susmentionné, infra, supra, ad hoc, de facto, in fine, subséquemment*
- **Medical:** *anamnèse, étiologie, pathognomonique, iatrogène, nosocomial, décompensation, récidive, rémission, syndromique*
- **Admin (mostly FR):** *attributaire, solliciter, diligenter, instruction du dossier, pièces justificatives, circulaire, délibération, arrêté préfectoral, transmission des pièces, ayant droit*

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`unexplained-abbreviation`](./unexplained-abbreviation.md)
