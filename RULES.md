# lucid-lint — Rules Reference

> A cognitive accessibility linter for prose. Built on cognitive load research. Bilingual EN/FR with equal care. Plugin-first, CI-native.

This document describes the 17 rules included in `lucid-lint`. The rule
set landed in v0.1; v0.2 adds the [scoring model](#scoring), renames
the category taxonomy to 5 fixed buckets, and introduces the `weight`
field on diagnostics.

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

Each rule belongs to exactly one of five fixed categories. The taxonomy
mirrors the F14 scoring model (see
[`brainstorm/20260420-score-semantics.md`](brainstorm/20260420-score-semantics.md)).

| Category | Purpose | Rules |
|---|---|---|
| `structure` | Length, nesting, punctuation, document skeleton | `sentence-too-long`, `paragraph-too-long`, `excessive-commas`, `long-enumeration`, `deep-subordination`, `deeply-nested-lists`, `heading-jump` |
| `rhythm` | Cadence and repetition across adjacent sentences | `consecutive-long-sentences`, `repetitive-connectors` |
| `lexicon` | Vocabulary, terminology, acronyms, lexical diversity | `low-lexical-diversity`, `excessive-nominalization`, `unexplained-abbreviation`, `weasel-words`, `jargon-undefined` |
| `syntax` | Sentence-level style and syntactic clarity | `passive-voice`, `unclear-antecedent` |
| `readability` | Document-level readability metrics | `readability-score` |

> v0.2 remapped the v0.1 taxonomy: `length` and the pre-v0.2 `structure`
> merged into `structure`; `lexical` became `lexicon`; `style` split
> between `syntax` and `rhythm`; `global` became `readability`.

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
| `5` | `readability-score` |
| `2` | `sentence-too-long`, `paragraph-too-long`, `deep-subordination`, `passive-voice`, `unclear-antecedent` |
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

#### `sentence-too-long`

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

#### `paragraph-too-long`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

**Intent** : detect paragraphs that are too long. A paragraph is a visual reprise unit. Long paragraphs make recovery after interruption harder.

**Rationale**

<!-- lucid-lint disable-next-line weasel-words -->

A paragraph is a mental chunk. A reader with attentional load interrupts often and must find their place again. Short paragraphs create clear reprise points; long ones dilute them.

The rule uses both a sentence count and a word count so that a short-but-dense paragraph (one 80-word sentence) is still flagged. Rule 1 `sentence-too-long` catches the complementary case.

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

#### `excessive-commas`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

<!-- lucid-lint disable-next-line weasel-words -->

**Intent** : detect sentences with too many commas. A high comma count is almost always a sign of overload, regardless of cause (subordination, apposition, enumeration, inline parenthetical).

**Rationale**

<!-- lucid-lint disable-next-line weasel-words -->

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

When `long-enumeration` is also active, it disables `excessive-commas` on the specific sentence it flags. This avoids double-reporting.

---

#### `long-enumeration`

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

#### `deep-subordination`

**Category** : `structure`
**Severity** : `warning`
**Bilingual** : yes, FR/EN lists differ

<!-- lucid-lint disable-next-line weasel-words -->

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

#### `deeply-nested-lists`

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

<!-- lucid-lint disable-next-line weasel-words -->
<!-- lucid-lint disable-next-line excessive-nominalization -->

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

#### `heading-jump`

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

#### `consecutive-long-sentences`

**Category** : `rhythm`
**Severity** : `warning`
**Bilingual** : yes, identical FR/EN

**Intent** : detect several long sentences in a row. Rhythm fatigues attention as much as individual sentence length.

**Rationale**

An isolated long sentence is manageable. Three long sentences in a row is almost guaranteed to lose an attention-fragile reader. This rule catches the *rhythm*, complementing `sentence-too-long` which catches individual cases.

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

**Important** : `word_threshold` must be lower than `sentence-too-long`'s `max_words` for the same profile. Otherwise both rules trigger on the same sentences.

---

### Lexicon rules

---

#### `low-lexical-diversity`

**Category** : `lexicon`
**Severity** : `info`
**Bilingual** : yes, FR/EN stoplists differ

<!-- lucid-lint disable-next-line weasel-words -->

**Intent** : detect passages with excessive repetition of content words. A monotonous text loses attention and often signals unstructured thinking.

**Rationale**

Lexical diversity is a measure of writing quality. The rule is NOT an anti-jargon detector: technical terms (`API`, `request`, `cache`) are expected to repeat. It targets non-technical content words that recur without reason.

<!-- lucid-lint disable-next-line unexplained-abbreviation -->

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

#### `excessive-nominalization`

**Category** : `lexicon`
**Severity** : `warning`
**Bilingual** : yes, FR/EN suffixes overlap significantly

<!-- lucid-lint disable-next-line weasel-words -->

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

<!-- lucid-lint disable-next-line weasel-words -->

Technical vocabulary (`function`, `implementation`, `configuration`) contains many technical nominalizations. The profile `dev-doc` accommodates this with a looser threshold. The rule targets *density*, not isolated occurrences.

---

#### `unexplained-abbreviation`

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

#### `weasel-words`

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

**All profiles** : active. Use `<!-- lucid-lint disable-next-line weasel-words -->` to opt out when usage is intentional (legitimate subset reference, quotation, etc.). See [Suppressing diagnostics](#suppressing-diagnostics).

---

#### `jargon-undefined`

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

### Syntax rules

---

#### `passive-voice`

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

#### `repetitive-connectors`

**Category** : `syntax`
**Severity** : `warning`
**Bilingual** : yes, FR/EN lists differ

**Intent** : detect overuse of the same logical connector. Repetition flattens the sense of progression.

**Rationale**

<!-- lucid-lint disable-next-line excessive-commas -->
<!-- lucid-lint disable-next-line excessive-nominalization -->

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

#### `unclear-antecedent`

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

### Readability rules

---

#### `readability-score`

**Category** : `readability`
**Severity** : `info` when below threshold, `warning` when above

**Intent** : calculate a readability score and flag documents exceeding a target grade level.

**Rationale**

Readability indices are the historical metric for text complexity. Simple, reproducible, recognized by US/UK government guidelines and WCAG. For readers with attentional difficulties, a poor score is a synthetic warning sign that deeper rules will refine.

**Detection (v0.1 unified formula)**

**Flesch-Kincaid Grade Level** applied regardless of language :

```
0.39 × (words / sentences) + 11.8 × (syllables / words) − 15.59
```

| Grade | US school level equivalent |
|---|---|
| < 6 | Elementary |
| 6–9 | Middle school |
| 9–12 | High school |
| 12–16 | College |
| > 16 | Expert |

**Note** : the formula is calibrated for English. Applied to French, it slightly overestimates (+1 to +2 grades). Language-specific calibration (Kandel-Moles, Scolarius) is planned for v0.2. See registry F10.

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

v0.1 supports a single inline-disable directive for Markdown inputs:

```markdown
<!-- lucid-lint disable-next-line sentence-too-long -->

A long sentence that is intentional and should not be flagged.
```

- **Syntax** : HTML comment, one rule id per directive.
- **Scope** : the next non-blank line in the source.
- **Applies to** : Markdown only. Plain text and stdin cannot carry comments; config-based ignores (`[[ignore]]` in `lucid-lint.toml`) are planned — see `ROADMAP.md` F19.
- **Unknown rule ids** : silently ignored.

Block disable/enable (`<!-- lucid-lint-disable -->` … `<!-- lucid-lint-enable -->`), file-level directives, multi-rule lists, and an optional `reason=` field are tracked as F18–F21 in `ROADMAP.md`.

---

## Rule interactions

- `long-enumeration` disables `excessive-commas` on sentences it flags.
- `consecutive-long-sentences` has a lower threshold than `sentence-too-long` within the same profile to avoid double-reporting.

---

## Known limitations of v0.1

1. **No anaphora resolution** : `unclear-antecedent` uses pattern heuristics only. A full NLP implementation is scheduled for v0.2.
2. **Single readability formula** : Flesch-Kincaid is used for all languages. Language-specific formulas (Kandel-Moles for French, etc.) are scheduled for v0.2.
3. **Heuristic passive voice detection** : expected precision ~70-80%. A POS-parser-based detector is planned for v0.2.
4. **Acronym and jargon detection without definition-awareness** : v0.1 flags all non-whitelisted occurrences. A two-pass definition-aware version is planned for v0.2.
5. **Supported formats** : Markdown, `.txt`, stdin only. Use Pandoc to convert other formats.

---

## Future work

A full backlog of future rules, refinements, and platform extensions is tracked separately in `ROADMAP.md`.
