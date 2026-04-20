# Brainstorm: score semantics for F14 (hybrid scoring model)

| Field | Value |
| --- | --- |
| **Date** | 2026-04-20 |
| **Duration** | ~25 min |
| **Participants** | Bastien + AI Facilitator |
| **Problem shape** | Few ideas → latent binary (synthesized vs. no-synthesis) → first-principles cut |
| **Output** | Design direction for F14; five new ROADMAP items (F37–F41) |

## Session plan

| # | Phase | Technique | Status |
| --- | --- | --- | --- |
| 0 | Intake | AI-seeded ideas (4 user + 4 AI) | Done |
| 1 | Premise cut | Filter against ROADMAP commitment | Done |
| 2 | SCAMPER — Combine | Per-granularity synthesis | Done |
| 3 | Impact / Effort | Category taxonomy finalist pick | Done |
| 4 | First principles (adapted) | Cut everything redundant from the "now" | Done — strongest convergence step |
| 5 | Remaining discovery | Composition, categories, granularity, exit code, residual risks | Done |
| 6 | Crystallize | Decision set + ROADMAP additions | Done |

## Ideas — starting point

Four user ideas, four AI-seeded additions:

1. 0–100 integer, higher = better (Vale, SonarQube, Lighthouse)
2. Letter grade A/B/C/D/F (Hemingway, Code Climate)
3. Flesch-style grade level (reading-age equivalent)
4. Pass/fail with numeric margin
5. `[AI]` Traffic light + number (🔴🟡🟢 decorating a 0–100)
6. `[AI]` Issue density only (reject synthesis; "N issues per 1000 words" per category)
7. `[AI]` "Reading time cost" — translate diagnostics to comprehension-overhead seconds
8. `[AI]` Dual score: readability + clarity (two numbers, reject single global)

## Step 1: premise cut

F14's ROADMAP commitment is **global + per-category + diagnostics**. Applying that filter:

| # | Idea | Verdict |
|---|---|---|
| 1 | 0–100 integer | ✅ keep |
| 2 | Letter grade | ✅ keep |
| 3 | Flesch grade level | ❌ cut — that's what `readability-score` already does |
| 4 | Pass/fail + margin | ✅ keep (wraps any numeric form) |
| 5 | Traffic light + number | ✅ keep (decoration) |
| 6 | Issue density only | ❌ cut — rejects premise |
| 7 | Reading-time cost | ⚠️ keep as unit option |
| 8 | Dual score | ❌ cut — rejects single-global premise |

Survivors: `{0–100, letter, pass/fail margin, traffic light, reading-time-seconds}`.

## Step 2: SCAMPER — Combine

Bastien's feedback on each survivor revealed that **no single form wins at every level**:

- **0–100**: unstable as rules grow (today's 80 becomes tomorrow's 60). Proposed fix: `X/max` arbitrary-max score during test-and-learn (`/impeccable` in this repo already uses this form — precedent works).
- **Letter grade**: hard to calibrate globally, but per-category letters are tractable — bounded scope makes thresholds definable.
- **Pass/fail margin**: fine as an association with another form.
- **Traffic light**: better glance signal than pass/fail for CI.
- **Reading-time-seconds**: hard to heuristicize accurately today; would overshadow other quality dimensions (comfort, fatigue, understandability). Park for a future version.

First Combine proposal (later pruned in Step 4):

| Level | Form |
|---|---|
| Global | `X / max` arbitrary-max |
| Per-category | Letter grade A–F |
| Decoration on global | Traffic light + pass/fail margin |
| Parked | Reading-time-seconds |

## Step 3: Impact / Effort — category taxonomy

| Option | Verdict |
|---|---|
| A. Fixed 5 categories now | ✅ Structure · Rhythm · Lexicon · Syntax · Readability |
| B. Derive categories from `rule_id` prefix via a rename pass | ❌ Couples F14 to a 17-rule rename; better as F29 (rule numbering) |
| C. Start with fixed + defer rename | ✅ **picked** — uses the `category_of(rule_id)` helper already decided in v0.1 |

## Step 4: first principles — cut redundancy

Bastien flagged "a lot of elements for a first attempt" and asked for a first-principles review. Four functions a score MUST serve:

1. At-a-glance quality signal
2. Actionability — point at what to fix
3. Stability as rules evolve
4. No gaming / no fake precision

| Element | Function served | Redundant? |
|---|---|---|
| `X/max` global | 1, 3 | — |
| Per-category sub-scores | 2 | — |
| Letter grades A–F | 1 + 2 (duplicates above) | ❌ adds a 2nd convention for zero new information |
| Traffic light | 1 (duplicates global) | ❌ |
| Pass/fail margin in display | CI gating — but exit code already handles this | ❌ clutter; `--min-score` flag is a separate concern |
| Reading-time-seconds | 1 (alt unit) | future |

**Key insight:** actionability (function 2) is *already delivered by the existing diagnostics output* (rule_id + message + location). Sub-scores merely summarize. So the sub-score level can afford to be minimal too.

**Second insight:** one convention across global and per-category (`X/max` at both) is easier to read than two conventions.

## Step 5: remaining discovery

| Q | Decision |
|---|---|
| Composition | **Mix all three:** weighted sum (base) + density-normalization (per 1000 words) + per-category cap (no single rule dominates) |
| Categories | Structure · Rhythm · Lexicon · Syntax · Readability (5, derived via helper) |
| Granularity (F15) | Document + project for v0.2. Section-level deferred → F38 |
| Exit code | **Both:** existing severity-driven exit + new `--min-score=N` flag |
| Biggest remaining risk | Unclear rule messages — users can't act on a score if they can't act on the diagnostics it aggregates. Docs mitigate; message audit needed → F37 |

## Decision — what F14 ships

| Element | Form | Surface |
|---|---|---|
| Global score | `X / max` arbitrary-max | TTY summary line + JSON |
| Per-category sub-scores | `X / max` (same form) | JSON always; TTY when `--verbose` or category has non-zero issues |
| Per-rule contribution cap | Enforced internally | Hidden mechanism, not displayed |
| `weight` field on `Diagnostic` | New field on the struct | JSON schema additive change |
| `--min-score=N` flag | CI gating | Exit code contract extension |

### Composition formula (reference)

```text
per_rule_cost       = Σ (rule.weight × severity_multiplier)    for each hit
per_category_cost   = min( Σ per_rule_cost / (words/1000),     ← density
                           category_cap )                       ← cap
category_score      = category_max − per_category_cost
global_score        = Σ category_scores
```

All three composition mechanics (weighted sum, density, cap) stack without conflict.

### Deferred to ROADMAP

| New ID | Item | Trigger for promotion |
|---|---|---|
| F37 | Rule-message clarity audit | Before v0.2 ships — F14 makes score actionability depend on diagnostic actionability |
| F38 | Section-level granularity | Once document + project are proven in the wild |
| F39 | Letter-grade decoration on `X/max` | User feedback: numbers feel noisy or hard to compare |
| F40 | Traffic-light + pass/fail margin in display | CI users ask for a stronger glance signal than the number |
| F41 | Reading-time-seconds as alternative unit | Validated heuristic + companion metrics (comfort, fatigue) so it doesn't monopolize the read |

---

## Session meta-analysis

- **Duration:** ~25 min (planned: "quick session", ~10 min) — first-principles step extended the session but sharpened the outcome
- **Techniques used:** Premise cut (1 min) · SCAMPER Combine (4 min) · Impact/Effort (3 min) · First principles (5 min, adapted from plan) · Discovery closure (5 min) · Crystallize (7 min)
- **Techniques skipped:** Devil's advocate was folded into Step 4 (first principles surfaced the same redundancy concerns)
- **Adaptations made:** inserted first-principles after Step 3 in response to Bastien's "too many elements" pushback — this was the convergence point
- **Problem shape:** detected "Few ideas" → actual "Few ideas with premise cut + over-synthesis correction"
- **Convergence point:** Step 4 (first principles) — cutting letters/traffic-light/margin from the "now" crystallized the design
- **What worked well:** Bastien's per-item critique in Step 2 did most of the synthesis; facilitator role was mostly stacking the decisions
- **What could improve:** I proposed the three-form synthesis in Step 2 without auditing for redundancy; the first-principles pass should have come earlier in the sequence
- **Session energy:** high, decisive — Bastien's answers were short and direct throughout
- **Recommendation for similar sessions:** for feature-design brainstorms with a pre-committed premise (here: ROADMAP entry shape), run first-principles *before* SCAMPER, not after — it prevents over-synthesis that has to be cut back
