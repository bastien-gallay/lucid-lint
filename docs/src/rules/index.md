# Rules reference

The canonical rules reference lives in [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) at the repository root.

It documents each of the 17 v0.1 rules in detail, including:

- The cognitive load problem addressed
- Research references (Sweller, Gibson, Graesser, WCAG, RGAA, FALC)
- Detection approach and known limitations
- Parameters and default thresholds per profile
- Bilingual specifications where applicable

## Categories

v0.2 fixes the taxonomy at 5 buckets (remapped from v0.1 — see the
[Categories](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md#categories)
section of RULES.md for the rename map). The buckets also drive the
[scoring model](../guide/scoring.md).

| Category | Rules |
|---|---|
| **Structure** | `sentence-too-long`, `paragraph-too-long`, `excessive-commas`, `long-enumeration`, `deep-subordination`, `deeply-nested-lists`, `heading-jump` |
| **Rhythm** | `consecutive-long-sentences`, `repetitive-connectors` |
| **Lexicon** | `low-lexical-diversity`, `excessive-nominalization`, `unexplained-abbreviation`, `weasel-words`, `jargon-undefined` |
| **Syntax** | `passive-voice`, `unclear-antecedent` |
| **Readability** | `readability-score` |

## Severity

- `info`: signal worth knowing, does not fail CI
- `warning`: quality issue worth fixing
- `error`: reserved for v0.2+ (not emitted in v0.1)

## Per-rule pages

Per-rule detail pages will be added in v0.2 (tracked as F28 in the roadmap). For now, refer to [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md).
