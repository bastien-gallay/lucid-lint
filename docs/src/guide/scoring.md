# Scoring

v0.2 adds a **hybrid scoring model** on top of the existing diagnostics.
Every run now answers two questions at once:

1. *What specifically is wrong?* — the diagnostics list, unchanged from v0.1.
2. *How bad is this document overall?* — a new global score plus five
   per-category sub-scores.

The two surfaces are complementary. Scores are summaries; diagnostics remain
the actionable signal.

## What the score means

The score takes the form `X / max` — an arbitrary maximum rather than a
0–100 normalized number. v0.2 ships with `max = 100` (five categories ×
twenty points), but the number is treated as a test-and-learn calibration:
the scale may shift in a future minor release as rule weights are tuned
against real corpora.

The rules of thumb for today's calibration:

| Range | Reading |
|---|---|
| 80 – 100 | Score reads green in the terminal. Nothing blocking. |
| 60 – 79 | Score reads yellow. A handful of hits worth reviewing. |
| 0 – 59 | Score reads red. Dense issues or a runaway rule. |

The colour bands are a reader aid, not a pass / fail contract. For CI
gating, use [`--min-score`](#gating-ci-with---min-score) with a concrete
number you picked.

## The five categories

Every rule belongs to exactly one category. v0.2 fixes the taxonomy at five
buckets:

| Category | Covers |
|---|---|
| `structure` | Length, nesting, punctuation, document skeleton |
| `rhythm` | Cadence and repetition across adjacent sentences |
| `lexicon` | Vocabulary, terminology, acronyms, lexical diversity |
| `syntax` | Sentence-level style and syntactic clarity |
| `readability` | Document-level readability metrics |

See the [rules reference](../rules/index.md) for the rule-to-category
mapping.

## How a score is computed

For a single document:

```text
per_rule_cost     = Σ (weight × severity_multiplier)        over hits
per_category_cost = min(Σ per_rule_cost / (words / 1000),   ← density
                        category_cap)                        ← cap
category_score    = category_max − per_category_cost         (clamped ≥ 0)
global_score      = Σ category_score
```

Three mechanics stack:

- **Weighted sum** — each hit costs `weight × severity_multiplier`. The
  default weight table lives in `scoring::default_weight_for` and
  emphasises rules whose hits carry the most cognitive load
  (`readability-score = 5`, length / subordination / passive /
  unclear-antecedent = 2, everything else = 1).
- **Density normalization** — costs are divided by `words / 1000` so a
  10 000-word handbook is not punished for having more hits than a
  400-word README. Documents shorter than 200 words are treated as
  200-word documents, so tiny fixtures are not artificially penalized.
- **Per-category cap** — no single category can lose more than
  `category_cap` out of `category_max`. A single noisy rule eats at most
  75 % of its own category (15 / 20 by default) and cannot leak into the
  others.

The severity multiplier is `info = 1`, `warning = 3`, `error = 5`.

## Reading the TTY output

The terminal formatter prints each diagnostic, a short summary line,
then a score block: the global number followed by every category
score with an eight-step sparkline bar.

![lucid-lint run on examples/sample.md — five diagnostics, a summary counting 3 warnings and 2 info, an explain hint, and a score block that reads 45/100 with category bars for structure, rhythm, lexicon, syntax, and readability](../assets/tty/hero.gif)

The same run rendered as plain text, for screen readers and
copy-paste:

```text
warning examples/sample.md:7:1 Sentence is 35 words long (maximum 30). Consider splitting it into shorter sentences. [section: A paragraph with a long sentence] [structure.sentence-too-long]
warning examples/sample.md:7:11 Weasel phrase "rather" weakens the statement. Replace with concrete language or remove it. [section: A paragraph with a long sentence] [lexicon.weasel-words]
info    examples/sample.md:1:1 Flesch-Kincaid grade 6.8 (target ≤ 14.0). [readability.score]
info    examples/sample.md:7:1 Sentence starts with a bare demonstrative "this". Name the referent to avoid forcing the reader to guess. [section: A paragraph with a long sentence] [syntax.unclear-antecedent]
warning examples/sample.md:7:1 Line is 210 characters wide (maximum 120). [section: A paragraph with a long sentence] [structure.line-length-wide]

summary: 3 warnings, 2 info.
→ run 'lucid-lint explain <rule-id>' — seen here: structure.sentence-too-long, lexicon.weasel-words, readability.score + 2 more
────────────────────────────────────────────────────────────
score: 45/100
       structure    █▎░░░  5/20
       rhythm       █████  20/20
       lexicon      █▎░░░  5/20
       syntax       ██▌░░  10/20
       readability  █▎░░░  5/20
```

All five categories are always displayed so the breakdown stays
structurally stable run-to-run. A perfect document reads `score:
100/100` with every bar full (`█████`). When the same rule fires two
or more times on one file, the hits cluster under a compact header
and any shared message or section is hoisted up so it only appears
once.

## Reading the JSON output

The JSON schema is at `version = 2` in v0.2. New fields:

```json
{
  "version": 2,
  "diagnostics": [
    {
      "rule_id": "structure.sentence-too-long",
      "severity": "warning",
      "location": { "file": { "kind": "path", "path": "draft.md" }, "line": 12, "column": 1, "length": 42 },
      "section": "Introduction",
      "message": "Sentence is 27 words long (maximum 22).",
      "weight": 2
    }
  ],
  "summary": { "info": 0, "warning": 1, "error": 0, "total": 1 },
  "score": { "value": 88, "max": 100 },
  "category_scores": [
    { "category": "structure",   "value": 8,  "max": 20 },
    { "category": "rhythm",      "value": 20, "max": 20 },
    { "category": "lexicon",     "value": 20, "max": 20 },
    { "category": "syntax",      "value": 20, "max": 20 },
    { "category": "readability", "value": 20, "max": 20 }
  ]
}
```

Category values are lowercase strings in the fixed order listed above.
Consumers that parsed the v0.1 schema should:

- bump their expected `version` from `1` to `2`;
- replace the old category names (`length` → `structure`,
  `lexical` → `lexicon`, `style` → `syntax`, `global` → `readability`);
- ignore unknown fields so future additive schema changes don't break them.

## Gating CI with `--min-score`

The `check` subcommand takes an optional `--min-score=N` flag. The run
exits `1` if the aggregate global score is below `N`, independently of
the severity-based gate.

```bash
# Fail the build if overall quality drops below 85/100
lucid-lint check --min-score=85 docs/
```

Both gates stack: the run fails if *either* the severity gate trips or
the score gate trips. Pick one or both depending on your workflow:

- **Severity gate only** (v0.1 behaviour): catches newly introduced
  warnings, doesn't react to a slow drift.
- **Score gate only** (`--fail-on-warning=false --min-score=85`):
  tolerates individual warnings but fails when density drifts past your
  threshold.
- **Both** (default + `--min-score=85`): both spikes and drifts fail the
  build.

## Tuning weights in `lucid-lint.toml`

Projects can override the calibration in their `lucid-lint.toml`:

```toml
[scoring]
category_max = 20
category_cap = 15

[scoring.weights]
sentence-too-long = 3
weasel-words      = 2
```

Missing fields fall back to the shipped defaults. The `[scoring.weights]`
sub-table is keyed by rule id; unknown ids are ignored so removing a rule
later doesn't break older configs.

## What's deferred

The brainstorm that shaped [F14](../roadmap.md) (see
[`brainstorm/20260420-score-semantics.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/brainstorm/20260420-score-semantics.md))
kept the model minimal. Decorations promoted only when user feedback
requires them:

- **Letter grades (A–F)** — tracked as [F-score-letter-grade](../roadmap.md#f-score-letter-grade). Promoted if the numbers feel
  noisy or hard to compare across documents.
- **Traffic-light + pass/fail margin display** — tracked as [F-score-traffic-light](../roadmap.md#f-score-traffic-light).
  Promoted if CI users ask for a stronger glance signal.
- **Reading-time-seconds as alternative unit** — tracked as [F41](../roadmap.md). Needs a
  validated heuristic plus companion metrics (comfort, fatigue) so it
  doesn't monopolize the read.
- **Section-level sub-scores** — tracked as [F38](../roadmap.md). Once document + project
  roll-ups are proven in the wild.
- **Project-level multi-file roll-up** — tracked as [F-project-scoring-rollup](../roadmap.md#f-project-scoring-rollup). The CLI in v0.2
  treats all passed paths as a single document for scoring purposes.
