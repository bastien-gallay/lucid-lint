# Rules reference

`lucid-lint` ships 17 rules in v0.1, all carried forward in v0.2. Each rule has a dedicated page below with category, severity, default weight, thresholds per profile, examples, and suppression guidance.

The compact reference at [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) remains the single-file overview kept in the repository root.

## Categories

Every rule belongs to exactly one of five fixed buckets. The taxonomy is authoritative — the [scoring model](../guide/scoring.md) composes per-category sub-scores into the global `X / max`.

| Category | Rules |
|---|---|
| **`structure`** | [`structure.sentence-too-long`](./sentence-too-long.md) · [`structure.paragraph-too-long`](./paragraph-too-long.md) · [`structure.heading-jump`](./heading-jump.md) · [`structure.deeply-nested-lists`](./deeply-nested-lists.md) · [`structure.excessive-commas`](./excessive-commas.md) · [`structure.long-enumeration`](./long-enumeration.md) · [`structure.deep-subordination`](./deep-subordination.md) · [`structure.line-length-wide`](./line-length-wide.md) · [`structure.mixed-numeric-format`](./mixed-numeric-format.md) |
| **`rhythm`** | [`rhythm.consecutive-long-sentences`](./consecutive-long-sentences.md) · [`rhythm.repetitive-connectors`](./repetitive-connectors.md) |
| **`lexicon`** | [`lexicon.low-lexical-diversity`](./low-lexical-diversity.md) · [`lexicon.excessive-nominalization`](./excessive-nominalization.md) · [`lexicon.unexplained-abbreviation`](./unexplained-abbreviation.md) · [`lexicon.weasel-words`](./weasel-words.md) · [`lexicon.jargon-undefined`](./jargon-undefined.md) · [`lexicon.all-caps-shouting`](./all-caps-shouting.md) · [`lexicon.redundant-intensifier`](./redundant-intensifier.md) · [`lexicon.consonant-cluster`](./consonant-cluster.md) |
| **`syntax`** | [`syntax.passive-voice`](./passive-voice.md) · [`syntax.unclear-antecedent`](./unclear-antecedent.md) · [`syntax.nested-negation`](./nested-negation.md) · [`syntax.conditional-stacking`](./conditional-stacking.md) · [`syntax.dense-punctuation-burst`](./dense-punctuation-burst.md) |
| **`readability`** | [`readability.score`](./readability-score.md) |

> **Authoritative source.** The category of each rule is determined by `Category::for_rule` in `src/types.rs`. The mapping above mirrors that function. A coverage test (`tests/rule_docs_coverage.rs`) keeps the per-rule pages, the category helper, and the scoring weights in lock-step.

## Severity levels

| Level | Meaning | Effect |
|---|---|---|
| `info` | Signal worth knowing, not a defect | Reported; does not fail CI |
| `warning` | Quality issue worth fixing | Reported; may fail CI depending on `--min-score` |
| `error` | Reserved for v0.3+ | Not emitted in v0.2 |

## Contributing a rule

See [Contributing](../contributing.md) for the rule-addition checklist — every new rule must land with a page in this section.
