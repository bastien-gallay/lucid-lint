# lucid-lint — Rules Reference

> A cognitive accessibility linter for prose. Built on cognitive load research. Bilingual EN/FR with equal care. Plugin-first, CI-native.

This document describes the 25 rules included in `lucid-lint` as of v0.2
(17 shipped in v0.1, 8 added during the v0.2 cycle). v0.2 also adds the
[scoring model](#scoring), renames the category taxonomy to 5 fixed
buckets, and introduces the `weight` field on diagnostics.

Each rule is atomic, documented, and configurable via a per-profile threshold.

> Section groupings below are pedagogical (by detection theme). The
> authoritative per-rule category is the `**Category**` line at the top
> of each rule; see [Categories](#categories) for the full mapping.

## Table of Contents

1. [Conventions](#conventions)
2. [Profiles](#profiles)
3. [Categories](#categories)
4. [Scoring](#scoring)
5. [Rules](#rules)
   - [Length rules](#length-rules) *(structure — length signals)*
   - [Structure rules](#structure-rules) *(structure — punctuation and skeleton)*
   - [Rhythm rules](#rhythm-rules)
   - [Lexicon rules](#lexicon-rules)
   - [Syntax rules](#syntax-rules)
   - [Readability rules](#readability-rules)
6. [Suppressing diagnostics](#suppressing-diagnostics)

---

## Conventions

### Rule identifiers

- Rule IDs use **kebab-case**.
- Each rule has one atomic detection goal.

### Severity levels

| Level | Meaning | Effect |
|---|---|---|
| `info` | Signal worth knowing, not a defect | Reported, does not fail CI |
| `warning` | Quality issue worth fixing | Reported, may fail CI depending on configuration |
| `error` | Reserved for future use | Not used in v0.1 |

### Profiles

A profile is a preset bundle of rule thresholds. Users pick a profile close to their intent, then override specific rules if needed.

- `dev-doc` — technical documentation, API docs, developer-facing content
- `public` — content for a general audience
- `falc` — *Facile À Lire et à Comprendre* (European Easy-to-Read standard)

---

## Categories

Each rule belongs to exactly one of five fixed categories. The split
separates phenomena that carry distinct cognitive costs:

- `structure` — document and prose shape at scale.
- `syntax` — sentence-internal mechanics: punctuation, clause
  construction, voice, connector usage.
- `rhythm` — cross-sentence cohesion and cadence.
- `lexicon` — vocabulary, terminology, acronyms, lexical diversity.
- `readability` — document-level metrics.

This taxonomy is the unit over which the F14 hybrid scoring model
composes: each category carries an independent sub-score and the global
score is their weighted sum.

| Category | Rules |
|---|---|
| `structure` | `structure.sentence-too-long`, `structure.paragraph-too-long`, `structure.deeply-nested-lists`, `structure.heading-jump`, `structure.excessive-commas`, `structure.long-enumeration`, `structure.deep-subordination`, `structure.line-length-wide`, `structure.mixed-numeric-format`, `structure.italic-span-long` *(experimental, [F-experimental-rule-status](ROADMAP.md#f-experimental-rule-status) cohort)* |
| `lexicon` | `lexicon.low-lexical-diversity`, `lexicon.excessive-nominalization`, `lexicon.unexplained-abbreviation`, `lexicon.weasel-words`, `lexicon.jargon-undefined`, `lexicon.all-caps-shouting`, `lexicon.redundant-intensifier`, `lexicon.consonant-cluster` |
| `syntax` | `syntax.passive-voice`, `syntax.unclear-antecedent`, `syntax.nested-negation`, `syntax.conditional-stacking`, `syntax.dense-punctuation-burst` |
| `rhythm` | `rhythm.consecutive-long-sentences`, `rhythm.repetitive-connectors` |
| `readability` | `readability.score` |

> v0.2 remapped the v0.1 taxonomy: `length` merged into `structure`;
> `lexical` became `lexicon`; `rhythm.repetitive-connectors` moved from
> `style` to `rhythm`; `global` became `readability`. Punctuation and
> clause-level rules (`structure.excessive-commas`, `structure.deep-subordination`) stay in
> `structure` as skeleton-level signals; `syntax.unclear-antecedent` stays in
> `syntax` alongside `syntax.passive-voice` as sentence-internal clarity.

---

## Scoring

As of v0.2 every run emits a global `X / max` score plus five
per-category sub-scores, in addition to the existing diagnostics. The
composition formula is:

```text
per_rule_cost     = Σ (weight × severity_multiplier)
per_category_cost = min(Σ per_rule_cost / (words / 1000), category_cap)
category_score    = category_max − per_category_cost   (clamped ≥ 0)
global_score      = Σ category_score
```

Default weights (from `scoring::default_weight_for`):

| Weight | Rules |
|---|---|
| `5` | `readability.score` |
| `2` | `structure.sentence-too-long`, `structure.paragraph-too-long`, `structure.deep-subordination`, `syntax.passive-voice`, `syntax.unclear-antecedent`, `syntax.nested-negation`, `syntax.conditional-stacking` |
| `1` | every other rule |

Severity multiplier: `info = 1`, `warning = 3`, `error = 5` (reserved).

Full details, CI gating, and TOML overrides in the user guide:
[Scoring](docs/src/guide/scoring.md). Suppressed diagnostics (via
`<!-- lucid-lint disable-next-line ... -->`) contribute **zero cost** —
suppression and scoring are consistent.

---

## Rules

### Length rules

---

#### `structure.sentence-too-long`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

**Intent** : detect sentences whose length exceeds a threshold. Long sentences are harder to process, especially under attentional load.

**Rationale**

The intrinsic cognitive load of a sentence grows non-linearly with its length (Graesser et al. 2004, *Coh-Metrix*). FALC recommends a 15-word ceiling. Plain English (US) recommends 20 words. For readers with attentional difficulties, longer sentences increase the probability of losing the thread mid-read.

**Detection**

Split text into sentences via strong punctuation (`.` `!` `?` `…` + paragraph breaks). Count Unicode word tokens (excluding punctuation). Report each sentence exceeding the threshold, with position and measured length.

Contractions (`don't`) and elisions (`l'accessibilité`) are counted as one word when the apostrophe is between two letters.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_words` | int | profile-dependent |
| `exclude_code_blocks` | bool | `true` |

**Thresholds by profile**

| Profile | `max_words` |
|---|---|
| `dev-doc` | 30 |
| `public` | 22 |
| `falc` | 15 |

---

#### `structure.paragraph-too-long`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

**Intent** : detect paragraphs that are too long. A paragraph is a visual reprise unit. Long paragraphs make recovery after interruption harder.

**Rationale**

<!-- lucid-lint disable-next-line lexicon.weasel-words -->

A paragraph is a mental chunk. A reader with attentional load interrupts often and must find their place again. Short paragraphs create clear reprise points; long ones dilute them.

The rule uses both a sentence count and a word count so that a short-but-dense paragraph (one 80-word sentence) is still flagged. Rule 1 `structure.sentence-too-long` catches the complementary case.

**Detection**

Split text on blank lines (Markdown paragraph convention). Count sentences and words per paragraph. Flag paragraphs exceeding either threshold.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_sentences` | int | profile-dependent |
| `max_words` | int | profile-dependent |

**Thresholds by profile**

| Profile | `max_sentences` | `max_words` |
|---|---|---|
| `dev-doc` | 7 | 150 |
| `public` | 5 | 100 |
| `falc` | 3 | 60 |

---

### Structure rules

---

#### `structure.excessive-commas`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

<!-- lucid-lint disable-next-line lexicon.weasel-words -->

**Intent** : detect sentences with too many commas. A high comma count is almost always a sign of overload, regardless of cause (subordination, apposition, enumeration, inline parenthetical).

**Rationale**

<!-- lucid-lint disable-next-line lexicon.weasel-words -->

The comma is the most frequent marker of syntactic complexity. Rather than trying to disentangle the exact cause, this rule flags density as a leading indicator.

**Detection**

Count commas per sentence. Report sentences exceeding the threshold.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_commas` | int | profile-dependent |

**Thresholds by profile**

| Profile | `max_commas` |
|---|---|
| `dev-doc` | 4 |
| `public` | 3 |
| `falc` | 2 |

**Interaction with other rules**

When `structure.long-enumeration` is also active, it disables `structure.excessive-commas` on the specific sentence it flags. This avoids double-reporting.

---

#### `structure.long-enumeration`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

**Intent** : detect inline enumerations in prose that would be clearer as bulleted lists.

**Rationale**

A prose enumeration of 5+ items is harder to scan than a bulleted list. The rule suggests converting to a list.

**Detection**

Sequence of 4+ short comma-separated segments ending with `, et` / `, or` / `, ou` / `, and`.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `min_items` | int | 5 |

**Diagnostic suggestion**

> *"Consider converting this enumeration to a bulleted list."*

---

#### `structure.deep-subordination`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, FR/EN lists differ

<!-- lucid-lint disable-next-line lexicon.weasel-words -->

**Intent** : detect cascading subordinate clauses. Multiple nested relative pronouns or subordinating conjunctions force the reader to hold many open referents in working memory.

**Rationale**

Gibson (1998), *Dependency Locality Theory*: the longer the distance between a referent and its linked element, the more costly the processing. FALC recommends avoiding subordinates.

**Detection**

Sequence of 2+ relative pronouns or subordinating conjunctions without strong punctuation between them.

**Example detected**

> *"Le document qui a été rédigé par l'équipe que nous avons constituée..."*

**Example NOT detected (false positive avoided)**

> *"Les pronoms relatifs en français sont : qui, que, dont, où."*

The comma-separated enumeration of pronouns does not trigger the rule, because the pronouns are not used in a cascading construction.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_consecutive_subordinators` | int | profile-dependent |

**Thresholds by profile**

| Profile | `max_consecutive_subordinators` |
|---|---|
| `dev-doc` | 3 |
| `public` | 2 |
| `falc` | 2 |

**Language lists**

- 🇫🇷 Relative pronouns: `qui, que, dont, où, lequel, laquelle, lesquels, lesquelles`
- 🇫🇷 Subordinators: `parce que, afin que, bien que, quoique, puisque, pour que, tandis que`
- 🇬🇧 Relative pronouns: `which, that, who, whom, whose`
- 🇬🇧 Subordinators: `because, although, while, since, whereas, unless, until`

---

#### `structure.deeply-nested-lists`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : language-agnostic

**Intent** : detect bulleted lists nested beyond a reasonable depth. Deep nesting becomes an unreadable tree.

**Rationale**

A list helps scanning. A deeply nested list forces the reader to reconstruct a complex mental hierarchy.

- Level 1: parallel items, easy.
- Level 2: sub-details, still readable.
- Level 3: acceptable, detailed outline.
- Level 4+: mental hierarchy lost.

<!-- lucid-lint disable-next-line lexicon.weasel-words -->
<!-- lucid-lint disable-next-line lexicon.excessive-nominalization -->

For readers with attentional difficulties, horizontal indentation becomes a crucial positional cue. Four levels of indent are too many to track.

**Detection**

Parse Markdown via `pulldown-cmark`. Extract list items with indentation level. Flag items exceeding the max depth.

Fully deterministic. No false positives.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_depth` | int | profile-dependent |

**Thresholds by profile**

| Profile | `max_depth` |
|---|---|
| `dev-doc` | 4 |
| `public` | 3 |
| `falc` | 2 |

**Diagnostic suggestion**

> *"List item at depth N exceeds maximum depth of M. Consider flattening structure, splitting into multiple lists, or using subsections with headings."*

---

#### `structure.line-length-wide`

**Category** : `structure`
**Severity** : `warning`
**Default weight** : `1`
**Condition tags** : `dyslexia`, `general`
**Bilingual** : yes, script-agnostic

**Intent** : flag author-chosen lines wider than the per-profile ceiling. WCAG 1.4.8 (AAA) caps rendered text at roughly 80 characters because longer lines force longer return sweeps and increase re-reading — a known difficulty for dyslexic readers. Markdown soft-wraps are NOT author choices; only hard breaks (`<br>` / two trailing spaces) and plain-text newlines are.

**References** : WCAG 1.4.8 (AAA), BDA Dyslexia Style Guide.

**Detection**

For every paragraph that carries an authorial line break, count grapheme clusters on each chunk between breaks and report any chunk above `max_line_length`. Soft-wrapped Markdown paragraphs (the common prose case) are exempt by construction: the parser collapses their soft breaks to spaces, so what remains is one logical line whose source length tracks the editor width, not the rendered width WCAG 1.4.8 targets. Pair with `structure.paragraph-too-long` if you also want a ceiling on joined paragraph length. Fenced and indented code blocks are excluded by the Markdown parser. Headings, list items, and table cells are out of scope.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_line_length` | int | profile-dependent |

**Thresholds by profile**

| Profile | `max_line_length` |
|---|---|
| `dev-doc` | 120 |
| `public` | 100 |
| `falc` | 80 |

---

#### `structure.mixed-numeric-format`

**Category** : `structure`
**Severity** : `warning`
**Default weight** : `1`
**Condition tags** : `dyscalculia`, `general`
**Bilingual** : yes, EN + FR

**Intent** : flag sentences that mix digit numerals (`42`, `3.14`, `1,000`) with spelled-out numerals (`two`, `trois`, `twenty`, `cent`) in the same sentence. Inconsistent numeric presentation forces readers — especially those with dyscalculia — to switch surface forms mid-clause and re-anchor the referent.

**References** : CDC Clear Communication Index (3.5 — "present numbers consistently throughout"), plainlanguage.gov (Chapter 4 — "use numerals").

**Detection**

Per sentence, scan for digit-numeric tokens (`\d+(?:[.,\s]\d+)*`) and for entries in the per-language spelled-numeral list. Emit one diagnostic per sentence where both surface forms co-occur. The ambiguous forms `one` (EN) and `un` / `une` (FR) are excluded from the spelled list because they double as indefinite pronouns and articles.

**Parameters**

None. The rule has no configurable threshold — a single co-occurrence of the two forms is sufficient.

---

#### `structure.heading-jump`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : language-agnostic

**Intent** : detect heading level jumps (e.g., H2 → H4). A broken heading hierarchy disrupts the reader's mental map of the document.

**Rationale**

Heading hierarchy creates a mental map. Each level must follow the previous one by at most +1. Readers with attentional difficulties rely heavily on headings to reposition themselves after interruption. A broken hierarchy destroys these cues.

**References**

- WCAG 2.1 criterion 1.3.1 (Info and Relationships)
- WCAG 2.1 criterion 2.4.6 (Headings and Labels)
- RGAA criterion 9.1 (structure of information)

**Detection**

Parse Markdown headings (`#`, `##`, `###`, etc.). Verify each heading only increases by at most one level from the previous one.

Fully deterministic. No false positives.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `allow_first_heading_any_level` | bool | `true` |
| `require_h1` | bool | `false` |

**All profiles** : active, no threshold (binary rule).

---

### Rhythm rules

---

#### `rhythm.consecutive-long-sentences`

**Category** : `rhythm`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

**Intent** : detect several long sentences in a row. Rhythm fatigues attention as much as individual sentence length.

**Rationale**

An isolated long sentence is manageable. Three long sentences in a row is almost guaranteed to lose an attention-fragile reader. This rule catches the *rhythm*, complementing `structure.sentence-too-long` which catches individual cases.

**Detection**

Walk sentences sequentially. Count consecutive sentences exceeding a length threshold. Flag when N consecutive long sentences are found.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `word_threshold` | int | profile-dependent |
| `max_consecutive` | int | profile-dependent |

**Thresholds by profile**

| Profile | `word_threshold` | `max_consecutive` |
|---|---|---|
| `dev-doc` | 20 | 3 |
| `public` | 15 | 2 |
| `falc` | 10 | 2 |

**Important** : `word_threshold` must be lower than `structure.sentence-too-long`'s `max_words` for the same profile. Otherwise both rules trigger on the same sentences.

---

### Lexicon rules

---

#### `lexicon.low-lexical-diversity`

**Category** : `lexicon`
**Severity** : `info`
**Bilingual** : yes, FR/EN stoplists differ

<!-- lucid-lint disable-next-line lexicon.weasel-words -->

**Intent** : detect passages with excessive repetition of content words. A monotonous text loses attention and often signals unstructured thinking.

**Rationale**

Lexical diversity is a measure of writing quality. The rule is NOT an anti-jargon detector: technical terms (`API`, `request`, `cache`) are expected to repeat. It targets non-technical content words that recur without reason.

<!-- lucid-lint disable-next-line lexicon.unexplained-abbreviation -->

Reference: *type-token ratio* (TTR), classical metric in corpus linguistics (Herdan, 1960).

**Detection**

Slide a window of N words over the text. Compute `unique_words / total_words` within the window, excluding stop-list words. Flag if ratio falls below threshold.

Words inside code blocks are excluded.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `window_size` | int | profile-dependent |
| `min_ratio` | float | profile-dependent |
| `use_stoplist` | bool | `true` |

**Thresholds by profile**

| Profile | `window_size` | `min_ratio` |
|---|---|---|
| `dev-doc` | 100 | 0.40 |
| `public` | 100 | 0.50 |
| `falc` | 80 | 0.55 |

---

#### `lexicon.excessive-nominalization`

**Category** : `lexicon`
**Severity** : `warning`
**Bilingual** : yes, FR/EN suffixes overlap significantly

<!-- lucid-lint disable-next-line lexicon.weasel-words -->

**Intent** : detect sentences with too many nominalizations (action verbs turned into abstract nouns). Nominalization makes text abstract and heavy.

**Rationale**

Two cognitive problems with nominalization:

1. **More abstract** = more costly to process.
2. **Hidden agent** : nominalized text often obscures who is doing what.

**Heavy example**

> *"La réalisation de l'analyse de la conformité permettra l'identification des axes d'amélioration."*

**Lighter rewrite**

> *"Nous analyserons la conformité. Cela permettra d'identifier les axes à améliorer."*

**References**

- FALC : prefer action verbs, avoid abstract nouns.
- US Plain Writing Act / HHS guidelines : *"Use strong verbs, not nominalizations."*

**Detection**

Walk words. Detect typical nominalization suffixes. Count density per sentence.

- 🇫🇷 Suffixes : `-tion`, `-sion`, `-ment`, `-ance`, `-ence`, `-age`, `-ité`, `-isme`, `-ure`
- 🇬🇧 Suffixes : `-tion`, `-sion`, `-ment`, `-ance`, `-ence`, `-ity`, `-ism`, `-ness`, `-al`

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_per_sentence` | int | profile-dependent |
| `suffixes` | list | language-dependent defaults |

**Thresholds by profile**

| Profile | `max_per_sentence` |
|---|---|
| `dev-doc` | 4 |
| `public` | 3 |
| `falc` | 2 |

**Known limitation**

<!-- lucid-lint disable-next-line lexicon.weasel-words -->

Technical vocabulary (`function`, `implementation`, `configuration`) contains many technical nominalizations. The profile `dev-doc` accommodates this with a looser threshold. The rule targets *density*, not isolated occurrences.

---

#### `lexicon.unexplained-abbreviation`

**Category** : `lexicon`
**Severity** : `warning`
**Bilingual** : yes, FR/EN whitelists differ

**Intent** : detect acronyms used without definition. An undefined acronym forces the reader to guess or look up, breaking the flow.

**Rationale**

Each forced interruption increases the risk of losing attention. Standard convention: define an acronym on first use.

**References**

- WCAG 2.1 criterion 3.1.4 (Abbreviations)
- RGAA criterion 9.4 (expansion of abbreviations)

**Detection (v0.1 simplified)**

1. Detect acronyms via pattern: sequence of 2+ consecutive uppercase letters (optionally with digits).
2. Apply whitelist.
3. Flag all non-whitelisted occurrences.

**Note on v0.2+** : a smarter two-pass version will check if each acronym is defined in the document before flagging. See registry F9.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `min_length` | int | profile-dependent |
| `whitelist` | list | profile-dependent |

**Default whitelist (v0.1)**

General IT : `URL, HTML, CSS, JSON, XML, HTTP, HTTPS, API, CLI, GUI, OS, CPU, RAM, SSD, USB, WiFi`

Common FR/EN : `PDF, SMS, GPS, ID, OK, FAQ`

**Thresholds by profile**

| Profile | `min_length` | Whitelist |
|---|---|---|
| `dev-doc` | 3 | Extended (tech acronyms added) |
| `public` | 2 | Minimal |
| `falc` | 2 | Empty (every acronym must be defined) |

---

#### `lexicon.weasel-words`

**Category** : `lexicon`
**Severity** : `warning`
**Bilingual** : yes, FR/EN lists differ

**Intent** : detect vague words that weaken a statement. A weasel word adds an invisible cognitive load: the reader must decide whether it matters, is true, or measurable.

**Rationale**

References: Wikipedia style guide (*"Avoid weasel words"*), Strunk & White, FALC.

**Detection**

Maintain a list of weasel words per language. Walk words. Flag each occurrence.

Trivial implementation. HashSet lookup.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `custom_weasels_fr` | list | `[]` |
| `custom_weasels_en` | list | `[]` |
| `disable_weasels` | list | `[]` |

**Default lists (v0.1)**

🇫🇷

- *quelques*, *certains*, *parfois*, *plutôt*, *assez*
- *globalement*, *généralement*, *souvent*, *en général*, *la plupart*
- *il semble que*, *il semblerait que*, *on pourrait dire que*, *on dit souvent*
- *beaucoup de*, *peu de*, *presque*, *quasiment*, *environ*, *à peu près*

🇬🇧

- *some*, *many*, *often*, *just*, *simply*
- *clearly*, *obviously*, *seemingly*, *arguably*
- *basically*, *essentially*, *virtually*, *various*, *numerous*
- *sort of*, *kind of*, *a bit*, *rather*, *quite*
- *fairly*, *relatively*, *mostly*, *generally*

**All profiles** : active. Use `<!-- lucid-lint disable-next-line lexicon.weasel-words -->` to opt out when usage is intentional (legitimate subset reference, quotation, etc.). See [Suppressing diagnostics](#suppressing-diagnostics).

---

#### `lexicon.jargon-undefined`

**Category** : `lexicon`
**Severity** : `warning`
**Bilingual** : yes, separate lists per language and per domain

**Intent** : detect domain-specific jargon terms used without definition. Jargon is contextual: acceptable among specialists, exclusionary otherwise.

**Rationale**

Like acronyms, jargon creates reading interruptions for the non-specialist reader. Unlike acronyms, jargon is content words, not uppercase sequences.

**References**

- US Plain Language : *"Avoid jargon."*
- FALC : use common vocabulary only.
- WCAG 3.1.3 : define unusual words.

**Detection (v0.1 simplified)**

1. Maintain multiple jargon lists (`tech`, `legal`, `medical`, `admin`).
2. User activates relevant lists.
3. Flag each occurrence of a listed term.

**Note on v0.2+** : a smarter version will check if the term is defined in the document, similar to F9 for acronyms.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `jargon_lists` | list | profile-dependent |
| `custom_jargon` | list | `[]` |
| `whitelist` | list | `[]` |

**Default lists (v0.1 — starter sets, to be enriched by community)**

*Tech (~20 terms)* : `idempotent, orthogonal, deterministic, polymorphic, serialization, deserialization, synchronous, asynchronous, concurrency, thread-safe, side-effect, referential transparency, memoization, currying, hoisting, closure, monad, immutable, stateless, refactoring`

*Legal (~15 terms, mostly FR)* : `apériteur, clause résolutoire, force majeure, cessation de paiement, préjudice subi, onéreux, nonobstant, préalablement, susmentionné, infra, supra, ad hoc, de facto, in fine, subséquemment`

*Medical (~10 terms)* : `anamnèse, étiologie, pathognomonique, iatrogène, nosocomial, pronostic vital engagé, décompensation, récidive, rémission, syndromique`

*Admin (~10 terms, mostly FR)* : `attributaire, solliciter, diligenter, instruction du dossier, pièces justificatives, circulaire, délibération, arrêté préfectoral, transmission des pièces, ayant droit`

**Profile activation**

| Profile | Lists active |
|---|---|
| `dev-doc` | None (developers understand their domain jargon) |
| `public` | `tech`, `legal`, `medical`, `admin` |
| `falc` | `tech`, `legal`, `medical`, `admin`, with strict mode |

---

#### `lexicon.all-caps-shouting`

**Category** : `lexicon`
**Severity** : `warning`
**Default weight** : `1`
**Condition tags** : `a11y-markup`, `dyslexia`, `general`
**Bilingual** : yes, language-agnostic (script-only detection)

**Intent** : flag runs of consecutive ALL-CAPS words. ALL-CAPS prose strips the shape cues dyslexic readers rely on, and triggers many screen readers to spell out the run letter by letter.

**Rationale**

WCAG 3.1.5 and the BDA Dyslexia Style Guide both recommend lowercase or sentence case for emphasis. Use bold or italics, or a callout, instead of `SHOUTING`.

**References** : WCAG 3.1.5 (Reading Level), BDA Dyslexia Style Guide.

**Detection**

Per paragraph, scan for runs of consecutive ALL-CAPS words. Minor connectors (`,`, `;`, `:`, `-`, whitespace) keep a run alive; a lowercase word, a period, or paragraph break ends it. A word is ALL-CAPS when it is at least 2 letters long and contains no lowercase letter. Single ALL-CAPS tokens are treated as abbreviations and are the responsibility of `lexicon.unexplained-abbreviation`. Code blocks are excluded by the Markdown parser.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `min_run_length` | int | profile-dependent |

**Thresholds by profile**

| Profile | `min_run_length` |
|---|---|
| `dev-doc` | 3 |
| `public` | 2 |
| `falc` | 2 |

`dev-doc` tolerates a 2-word emphasis run (`DO NOT`) common in technical docs.

---

#### `lexicon.redundant-intensifier`

**Category** : `lexicon`
**Severity** : `warning`
**Default weight** : `1`
**Condition tags** : `general`
**Bilingual** : yes, EN + FR

**Intent** : flag intensifiers — adverbs that try to upgrade the confidence of a statement without adding information (`very important` → `important` or a quantified claim). Deliberate sibling of [`lexicon.weasel-words`](#lexiconweasel-words): weasel words downgrade confidence, redundant intensifiers upgrade it. Lists are disjoint by construction.

**References** : plainlanguage.gov Chapter 4, CDC Clear Communication Index.

**Detection**

Per paragraph, lowercase the text and look for each intensifier phrase using the shared word-bounded search. Hits inside fenced or inline code spans are ignored. Documents whose language is `Unknown` are skipped.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `custom_intensifiers_en` | list<string> | `[]` |
| `custom_intensifiers_fr` | list<string> | `[]` |
| `disable` | list<string> | `[]` |

**All profiles** : same thresholds; suppress per-phrase via `disable` or per-instance via inline directives.

---

#### `lexicon.consonant-cluster`

**Category** : `lexicon`
**Severity** : `warning`
**Default weight** : `1`
**Condition tags** : `dyslexia`, `general`
**Bilingual** : yes, EN + FR

**Intent** : flag words whose longest run of consecutive consonants meets or exceeds a per-profile threshold. Dense consonant clusters force a dyslexic reader to hold more phonemes in working memory before the next vowel "releases" the syllable (BDA Dyslexia Style Guide).

**References** : BDA Dyslexia Style Guide.

**Detection**

Per source line, walk the grapheme stream once. A word is a maximal run of alphabetic characters; hyphens, apostrophes, and whitespace close the word. Within a word, track the longest run of consecutive consonants. Vowels are language-aware (French accented forms count as vowels; `y` is always a vowel). Emit one diagnostic per qualifying word.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `min_run_length` | int | profile-dependent |

**Thresholds by profile**

| Profile | `min_run_length` |
|---|---|
| `dev-doc` | 6 |
| `public` | 5 |
| `falc` | 4 |

`dev-doc` is tolerant of `strengths`-class words; `falc` catches any 4-consonant run.

---

### Syntax rules

---

#### `syntax.passive-voice`

**Category** : `syntax`
**Severity** : `warning`
**Bilingual** : yes, separate heuristics FR/EN

**Intent** : detect passive voice constructions. Passive voice hides the agent and lengthens the sentence without adding information.

**Rationale**

Two problems: the reader doesn't know who does the action, and the sentence is longer than necessary. For readers under attentional load, every unneeded word is a cost.

Legitimate exceptions exist (unknown agent, scientific style, focus on the action). The rule flags, the author decides.

**References** : Plain Language US, Strunk & White, FALC.

**Detection (v0.1 heuristic)**

🇬🇧 : `be-verb + past participle [+ by ...]`. List of conjugated `to be` forms + regular (`-ed`) and irregular past participles.

🇫🇷 : `être conjugated + past participle [+ par ...]`, also `se faire + infinitive`. Harder than EN due to:
- Agreement of past participle (gender, number).
- Confusion with subject attribute: *"il est content"* (not passive) vs *"il est vu"* (passive).
- Confusion with active compound tenses with `être` auxiliary: *"elle est partie"* (passé composé, active).

Expect ~70-80% precision in v0.1. False positives handled via inline disable comments.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_per_paragraph` | int | profile-dependent |
| `ignore_scientific_style` | bool | `false` |

**Thresholds by profile**

| Profile | `max_per_paragraph` |
|---|---|
| `dev-doc` | 3 |
| `public` | 1 |
| `falc` | 0 |

**Note on severity**

`warning` for all profiles. High levels of technical education do not equate to rhetorical mastery. Tech profiles are trained in technical reasoning, not careful prose. A `warning` level respects the reader.

---

#### `syntax.nested-negation`

**Category** : `syntax`
**Severity** : `warning`
**Default weight** : `2`
**Condition tags** : `aphasia`, `adhd`, `general`
**Bilingual** : yes, language-specific counting

**Intent** : flag sentences that stack multiple negations. Two or more negations in a single sentence force the reader to mentally toggle truth values, a known burden for readers with aphasia, ADHD, and anyone reading under load.

**Rationale**

Plain-language guidelines (FALC, CDC Clear Communication Index, plainlanguage.gov) recommend rewriting double negatives as positives. A negation is cheap to write and expensive to read: every additional toggle multiplies the inferential cost.

**References** : FALC, CDC Clear Communication Index, plainlanguage.gov.

**Detection**

Per sentence, count the negations and report counts above `max_negations`.

🇬🇧 : sum of word-boundary matches against the negation list (`not`, `no`, `never`, `none`, `nothing`, `nobody`, `nowhere`, `neither`, `nor`, `cannot`, `without`) plus occurrences of the contracted `n't` suffix (`don't`, `won't`, `isn't`, `doesn't`, …).

🇫🇷 : bipartite negation: each `ne` / `n'` clitic counts as one negation, plus standalone negators (`sans`, `non`). Counting the second-position particle (`pas`, `jamais`, `plus`, …) directly would trigger false positives because many of those forms are ambiguous outside the `ne ... X` construction.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_negations` | int | profile-dependent |

**Thresholds by profile**

| Profile | `max_negations` |
|---|---|
| `dev-doc` | 3 |
| `public` | 2 |
| `falc` | 1 |

---

#### `syntax.conditional-stacking`

**Category** : `syntax`
**Severity** : `warning`
**Default weight** : `2`
**Condition tags** : `aphasia`, `adhd`, `general`
**Bilingual** : yes, language-specific lists

**Intent** : flag sentences that chain multiple conditional clauses. Each `if` / `when` / `unless` (and FR equivalents) opens a branch the reader must hold open until the outer clause resolves.

**Rationale**

Conditionals are inferential branches. Two or three of them stacked in one sentence force the reader to maintain a mental call stack — costly for readers with aphasia, ADHD, and anyone reading under load. Plain-language guidelines (FALC, plainlanguage.gov) recommend splitting chains into separate sentences or a bullet list.

**References** : FALC, plainlanguage.gov.

**Detection**

Per sentence, sum word-bounded matches against the language's conditional connector list and report counts above `max_conditionals`.

🇬🇧 : `if`, `unless`, `when`, `whenever`, `while`, `until`, `provided`, `assuming`, `in case`, `as long as`, `as soon as`, `even if`, `only if`.

🇫🇷 : `si`, `sauf si`, `à moins que`, `à moins de`, `quand`, `lorsque`, `lorsqu'`, `dès que`, `tant que`, `pourvu que`, `à condition que`, `à condition de`, `au cas où`, `même si`, `en cas de`, plus the elliptic clitics `s'il` / `s'ils`.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_conditionals` | int | profile-dependent |

**Thresholds by profile**

| Profile | `max_conditionals` |
|---|---|
| `dev-doc` | 3 |
| `public` | 2 |
| `falc` | 1 |

---

#### `rhythm.repetitive-connectors`

**Category** : `rhythm`
**Severity** : `warning`
**Bilingual** : yes, FR/EN lists differ

**Intent** : detect overuse of the same logical connector. Repetition flattens the sense of progression.

**Rationale**

<!-- lucid-lint disable-next-line structure.excessive-commas -->
<!-- lucid-lint disable-next-line lexicon.excessive-nominalization -->

Logical connectors guide the reader: opposition, cause, consequence, sequence, illustration, addition. Used well, they are attentional anchors. Repeated, they become noise.

**Bad example**

> *"We analyzed the data. Then we built the model. Then we validated the results. Then we published the report."*

Four *"then"* = no progression felt.

**References**

- Sanders & Noordman (2000), *Connectives as processing signals*.
- Graesser et al. (2004), *Coh-Metrix*, local cohesion.

**Detection**

Sliding window of N sentences. Count occurrences per connector in the window. Flag if a connector exceeds the threshold.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_per_window` | int | profile-dependent |
| `window_size` | int | 5 |
| `custom_connectors` | list | `[]` |

**Thresholds by profile**

| Profile | `max_per_window` |
|---|---|
| `dev-doc` | 4 |
| `public` | 3 |
| `falc` | 2 |

**Default connector lists**

🇫🇷 :
- Opposition : *cependant, toutefois, en revanche, néanmoins, pourtant, mais*
- Cause : *parce que, car, puisque, en effet*
- Consequence : *donc, ainsi, par conséquent, c'est pourquoi*
- Sequence : *d'abord, ensuite, puis, enfin, premièrement*
- Illustration : *par exemple, notamment, en particulier, ainsi*
- Addition : *de plus, en outre, également, par ailleurs*

🇬🇧 :
- Opposition : *however, nevertheless, yet, although, but*
- Cause : *because, since, as, for*
- Consequence : *therefore, thus, consequently, hence, so*
- Sequence : *first, then, next, finally*
- Illustration : *for example, notably, in particular, such as*
- Addition : *moreover, furthermore, also, additionally*

---

#### `syntax.unclear-antecedent`

**Category** : `syntax`
**Severity** : `info`
**Bilingual** : yes, FR/EN pronoun lists differ

**Intent** : detect pronouns with ambiguous referents. An unclear *"it"*, *"this"*, *"cela"* forces the reader to backtrack and search for the antecedent.

**Rationale**

Ambiguous pronominal reference is one of the most costly comprehension breaks for readers with attentional difficulties. Each ambiguity forces a conscious return-and-search.

**References**

- Strunk & White : *"Pronouns must clearly refer to a specific antecedent."*
- FALC : prefer name repetition over pronouns.
- Graesser et al. *Coh-Metrix* : referential cohesion.

**Detection (v0.1 simplified)**

Exact detection requires anaphora resolution, which is advanced NLP. v0.1 catches only the most frequent ambiguity patterns:

1. Bare demonstrative pronouns at the start of a sentence (`This/That/These/Those`, `Ceci/Cela/Ce`) not immediately followed by a noun.
2. Personal pronouns at the start of a paragraph (no nearby antecedent).

**Example detected**

> *"Les performances étaient médiocres avec le cache LRU. **Cela** a motivé le changement."*

*"Cela"* refers to the poor performance? To the LRU cache? Ambiguous.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `check_demonstratives` | bool | `true` |
| `check_paragraph_start_pronouns` | bool | `true` |

**All profiles** : active. Severity `info` in v0.1 because the heuristic is approximate. Noise level warrants soft signaling.

**Pronoun lists**

- 🇫🇷 : *ce, cela, ceci, ça, celui-ci, celle-ci, il, elle, ils, elles*
- 🇬🇧 : *this, that, these, those, it, they, them*

---

#### `syntax.dense-punctuation-burst`

**Category** : `syntax`
**Severity** : `warning`
**Default weight** : `1`
**Condition tags** : `general`
**Bilingual** : yes, script-agnostic

**Intent** : flag *local* bursts of punctuation — windows where ≥ N qualifying marks (`,`, `;`, `:`, `—`, `–`) cluster within W grapheme clusters. Tight clusters signal layered subordination, parenthetical interjections, or list-within-list constructions that are hard to parse for readers with cognitive or attentional difficulties. Distinct from [`structure.excessive-commas`](#structureexcessive-commas), which counts commas across an entire sentence rather than over a sliding window.

**References** : IFLA easy-to-read guidelines.

**Detection**

Per source line, walk the grapheme stream once and collect the column of every qualifying mark. When a window of `window_graphemes` graphemes contains `min_marks` or more marks, emit a burst spanning the first to the last mark in the window, then advance past that last mark so overlapping windows do not double-fire. Code blocks are excluded upstream by the parser.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `min_marks` | int | profile-dependent |
| `window_graphemes` | int | profile-dependent |

**Thresholds by profile**

| Profile | `min_marks` | `window_graphemes` |
|---|---|---|
| `dev-doc` | 4 | 30 |
| `public` | 3 | 30 |
| `falc` | 3 | 40 |

`dev-doc` tolerates a 3-mark cluster (often unavoidable in technical lists adjacent to prose). FALC keeps the same density floor as `public` but widens the window to catch slightly looser bursts.

---

### Readability rules

---

#### `readability.score`

**Category** : `readability`
**Severity** : `info` when below threshold, `warning` when above

**Intent** : calculate a readability score and flag documents exceeding a target grade level.

**Rationale**

Readability indices are the historical metric for text complexity. Simple, reproducible, recognized by US/UK government guidelines and WCAG. For readers with attentional difficulties, a poor score is a synthetic warning sign that deeper rules will refine.

**Detection (v0.2 — per-language formula)**

The formula is selected by the document's detected language ([F-readability-formulas-extra](ROADMAP.md#f-readability-formulas-extra) must-ship slice).

🇬🇧 **Flesch-Kincaid Grade Level** :

```
0.39 × (words / sentences) + 11.8 × (syllables / words) − 15.59
```

The result is a US-school grade. Compared directly to `max_grade_level`.

🇫🇷 **Kandel & Moles (1958)** :

```
207 − 1.015 × (words / sentences) − 73.6 × (syllables / words)
```

The result is an ease score on roughly `0..100` (higher = easier), Flesch-style. To stay comparable across languages, the rule converts it to a grade-equivalent with the standard linear approximation `(100 − score) / 10` and compares that against `max_grade_level`. The diagnostic message surfaces both the native ease score and the grade-equivalent.

**Unknown language** falls back to Flesch-Kincaid.

| Grade | US school level equivalent |
|---|---|
| < 6 | Elementary |
| 6–9 | Middle school |
| 9–12 | High school |
| 12–16 | College |
| > 16 | Expert |

The `--readability-formula` CLI flag (F11, v0.2) pins a formula regardless of detected language — `auto` (default) keeps the per-language behaviour, `flesch-kincaid` / `kandel-moles` force the respective formula. Additional formulas (`Gunning Fog`, `SMOG`, `Dale-Chall`, `Scolarius`) and multi-formula `--readability-verbose` reports are tracked as [F-readability-formulas-extra](ROADMAP.md#f-readability-formulas-extra) should-ship.

**Granularity** : computed per document. One diagnostic per file.

**Output modes**

- Always reported as an `info` even when under threshold, for observability.
- Reported as a `warning` when above `max_grade_level`.

**Analogy** : this rule behaves like cyclomatic complexity. It's a metric first, a warning second.

**Parameters**

| Parameter | Type | Default |
|---|---|---|
| `max_grade_level` | float | profile-dependent |
| `always_report` | bool | `true` |

**Thresholds by profile**

| Profile | `max_grade_level` |
|---|---|
| `dev-doc` | 14 |
| `public` | 9 |
| `falc` | 6 |

---

## Suppressing diagnostics

Two inline-disable directives are supported for Markdown inputs.

### Line form

```markdown
<!-- lucid-lint disable-next-line structure.sentence-too-long -->

A long sentence that is intentional and should not be flagged.
```

- **Syntax** : HTML comment, one rule id per directive.
- **Scope** : the next non-blank line in the source.

### Block form (v0.2, F18)

```markdown
<!-- lucid-lint-disable structure.sentence-too-long -->

A long sentence.

Another long sentence in the same scope.

<!-- lucid-lint-enable -->
```

- **Syntax** : `<!-- lucid-lint-disable <rule-id> -->` opens a scope; `<!-- lucid-lint-enable -->` closes every currently-open scope. An optional rule id on `enable` closes only that rule's scope, which lets nested disables overlap for different rules.
- **Scope** : every line between the two comments (inclusive). An unterminated `disable` extends to the end of the document.
- **One rule per comment** : multi-rule lists are tracked as `F-suppression-disable-file`.

### Common properties

- **Applies to** : Markdown only. Plain text and stdin cannot carry comments; config-based ignores (`[[ignore]]` in `lucid-lint.toml`) are planned — see `ROADMAP.md` F19.
- **Unknown rule ids** : silently ignored.

File-level directives, multi-rule lists, and an optional `reason=` field are tracked as `F-suppression-reason-field` and `F-suppression-disable-file` in `ROADMAP.md`.

---

## Rule interactions

- `structure.long-enumeration` disables `structure.excessive-commas` on sentences it flags.
- `rhythm.consecutive-long-sentences` has a lower threshold than `structure.sentence-too-long` within the same profile to avoid double-reporting.

---

## Known limitations of v0.1

1. **No anaphora resolution** : `syntax.unclear-antecedent` uses pattern heuristics only. A full NLP implementation is scheduled for v0.2.
2. **Single readability formula** : Flesch-Kincaid is used for all languages. Language-specific formulas (Kandel-Moles for French, etc.) are scheduled for v0.2.
3. **Heuristic passive voice detection** : expected precision ~70-80%. A POS-parser-based detector is planned for v0.2.
4. **Acronym and jargon detection without definition-awareness** : v0.1 flags all non-whitelisted occurrences. A two-pass definition-aware version is planned for v0.2.
5. **Supported formats** : Markdown, `.txt`, stdin only. Use Pandoc to convert other formats.

---

## Future work

A full backlog of future rules, refinements, and platform extensions is tracked separately in `ROADMAP.md`.
