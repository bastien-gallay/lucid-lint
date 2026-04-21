# Rules reference

`lucid-lint` ships 17 rules in v0.1, all carried forward in v0.2. Each rule has a dedicated page below with category, severity, default weight, thresholds per profile, examples, and suppression guidance.

The compact reference at [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) remains the single-file overview kept in the repository root.

## Categories

Every rule belongs to exactly one of five fixed buckets. The taxonomy is authoritative — the [scoring model](../guide/scoring.md) composes per-category sub-scores into the global `X / max`.

| Category | Rules |
|---|---|
| **`structure`** | [`sentence-too-long`](./sentence-too-long.md) · [`paragraph-too-long`](./paragraph-too-long.md) · [`heading-jump`](./heading-jump.md) · [`deeply-nested-lists`](./deeply-nested-lists.md) · [`excessive-commas`](./excessive-commas.md) · [`long-enumeration`](./long-enumeration.md) · [`deep-subordination`](./deep-subordination.md) · [`line-length-wide`](./line-length-wide.md) · [`mixed-numeric-format`](./mixed-numeric-format.md) |
| **`rhythm`** | [`consecutive-long-sentences`](./consecutive-long-sentences.md) · [`repetitive-connectors`](./repetitive-connectors.md) |
| **`lexicon`** | [`low-lexical-diversity`](./low-lexical-diversity.md) · [`excessive-nominalization`](./excessive-nominalization.md) · [`unexplained-abbreviation`](./unexplained-abbreviation.md) · [`weasel-words`](./weasel-words.md) · [`jargon-undefined`](./jargon-undefined.md) · [`all-caps-shouting`](./all-caps-shouting.md) |
| **`syntax`** | [`passive-voice`](./passive-voice.md) · [`unclear-antecedent`](./unclear-antecedent.md) · [`nested-negation`](./nested-negation.md) · [`conditional-stacking`](./conditional-stacking.md) |
| **`readability`** | [`readability-score`](./readability-score.md) |

> **Authoritative source.** The category of each rule is determined by `Category::for_rule` in `src/types.rs`. The mapping above mirrors that function. A coverage test (`tests/rule_docs_coverage.rs`) keeps the per-rule pages, the category helper, and the scoring weights in lock-step.

## Severity levels

| Level | Meaning | Effect |
|---|---|---|
| `info` | Signal worth knowing, not a defect | Reported; does not fail CI |
| `warning` | Quality issue worth fixing | Reported; may fail CI depending on `--min-score` |
| `error` | Reserved for v0.3+ | Not emitted in v0.2 |

## Contributing a rule

See [Contributing](../contributing.md) for the rule-addition checklist — every new rule must land with a page in this section.
