# `lexicon.jargon-undefined`

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

## Detection

1. Maintain multiple jargon lists per domain (`tech`, `legal`, `medical`, `admin`).
2. User activates the relevant lists via profile.
3. Flag each occurrence of a listed term.

## Profile activation

| Profile | Lists active |
|---|---|
| `dev-doc` | none (developers understand their own jargon) |
| `public` | `tech`, `legal`, `medical`, `admin` |
| `falc` | `tech`, `legal`, `medical`, `admin`, strict mode |

## Configuration

In v0.2, the active lists are set by the profile and are not yet user-overridable from `lucid-lint.toml`. Per-rule TOML overrides — adding custom domain terms, silencing specific entries, or activating a non-default list combination — are tracked as **[F126](../roadmap.md)** on the [roadmap](../roadmap.md).

## Default starter lists (community contributions welcome)

- **Tech:** *idempotent, orthogonal, deterministic, polymorphic, serialization, deserialization, synchronous, asynchronous, concurrency, thread-safe, side-effect, referential transparency, memoization, currying, hoisting, closure, monad, immutable, stateless, refactoring*
- **Legal (mostly FR):** *apériteur, clause résolutoire, force majeure, cessation de paiement, préjudice subi, onéreux, nonobstant, préalablement, susmentionné, infra, supra, ad hoc, de facto, in fine, subséquemment*
- **Medical:** *anamnèse, étiologie, pathognomonique, iatrogène, nosocomial, décompensation, récidive, rémission, syndromique*
- **Admin (mostly FR):** *attributaire, solliciter, diligenter, instruction du dossier, pièces justificatives, circulaire, délibération, arrêté préfectoral, transmission des pièces, ayant droit*

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md)

## References

- [WCAG 2.1 — 3.1.3](../references.md#wcag-2-1)
- [Plain Language US (2011)](../references.md#plain-language-us-2011)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

See [References](../references.md) for the full bibliography.
