# Rules reference

The canonical rules reference lives in [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) at the repository root.

It documents each of the 16 v0.1 rules in detail, including:

- The cognitive load problem addressed
- Research references (Sweller, Gibson, Graesser, WCAG, RGAA, FALC)
- Detection approach and known limitations
- Parameters and default thresholds per profile
- Bilingual specifications where applicable

## Categories

| Category | Rules |
|---|---|
| **Length** | `sentence-too-long`, `paragraph-too-long` |
| **Structure** | `excessive-commas`, `long-enumeration`, `deep-subordination`, `deeply-nested-lists`, `heading-jump` |
| **Rhythm** | `consecutive-long-sentences` |
| **Lexical** | `low-lexical-diversity`, `excessive-nominalization`, `unexplained-abbreviation`, `weasel-words`, `jargon-undefined` |
| **Style** | `passive-voice`, `repetitive-connectors`, `unclear-antecedent` |
| **Global** | `readability-score` |

## Severity

- `info`: signal worth knowing, does not fail CI
- `warning`: quality issue worth fixing
- `error`: reserved for v0.2+ (not emitted in v0.1)

## Per-rule pages

Per-rule detail pages will be added in v0.2 when the full set of rules is implemented. For now, refer to [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md).
