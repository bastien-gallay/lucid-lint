# Example text sources / Textes d'exemple

> Curated index of publicly-accessible text sources usable as **examples** in
> docs, as **fixtures** for tests, or as **corpora** for benchmarks.
>
> Répertoire de sources de textes publiquement accessibles utilisables comme
> **exemples** dans la documentation, **fixtures** pour les tests, ou
> **corpus** pour les benchmarks.

**Not** academic references — those live in [`REFERENCES.md`](./REFERENCES.md).
The machine-readable source of truth for this page is [`texts.yaml`](./texts.yaml).

This page is a curated subset — only sources whose licence lets us
commit extracts into [`examples/public/`](./public/) are listed here.
Tooling reads additional local-only entries from an off-repo file it
never names in any published surface; see AGENTS.md prime directive
number 10 for the policy.

## 📋 Criteria schema / Schéma des critères

Each entry is evaluated against the following fields. The schema extends the
user-proposed starter set (type, interest, confidence) with attributes useful
for building balanced example sets.

| Field | Values | Purpose |
|---|---|---|
| `type` | `research_paper`, `standard`, `style_guide`, `corpus`, `tool_fixtures`, `gov_guide`, `blog`, `dataset`, `textbook`, `sample_collection` | Nature of the source |
| `polarity` | `good_example`, `bad_example`, `before_after`, `mixed_unaligned`, `neutral` | Does the source illustrate good prose, bad prose, aligned rewrites, an unaligned mix, or neither? |
| `rules_relevant` | lucid-lint rule IDs | Which rules the source exercises |
| `conditions` | `a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`, `non-native`, `general` | F71 condition tags |
| `languages` | `en`, `fr` | Available languages |
| `bilingual_parallel` | bool | *Aligned* EN/FR content (rare, high value for our bilingual stance) |
| `interest` | `high`, `medium`, `low` | Usefulness for our example set |
| `confidence` | `high`, `medium`, `low` | Weighted by institutional weight, claim quality, cross-source consistency |
| `use_case` | `example`, `benchmark`, `corpus`, `documentation` | Primary intended use |
| `has_explanations` | bool | Does the source explain *why* text is good/bad? |
| `license_or_access` | `open`, `restricted`, `unknown` | Reuse status |
| `redistribution` | `public_ok`, `link_only`, `restricted`, `check_license` | Can extracts be bundled into the public git repo? (see below) |
| `license_details` | free text | Actual licence name / constraints |
| `markdownable` | `0`–`5` | Ease of turning the source into Markdown fixtures (see scale below) |

### Markdownable scale

How easily a source can be turned into Markdown fixtures for
`lucid-lint`'s tests and benchmarks.

| Score | Meaning | Typical example |
|---|---|---|
| `5` | Already Markdown, wikitext, plain text, or deterministic conversion (CSV/JSON/XML datasets, MediaWiki dumps, GitHub repos). | Simple English Wikipedia, ASSET, proselint fixtures |
| `4` | Clean, well-structured HTML — single-shot pandoc/readability conversion. | GOV.UK pages, WCAG Understanding pages |
| `3` | HTML with significant noise (ads, navigation, rich layout); cleanup needed. | Some magazine/blog layouts |
| `2` | PDF that is mostly flowing text — extract + manual cleanup. | EC *How to write clearly*, CDC CCI |
| `1` | PDF / HTML with complex layout, tables, images interleaved — heavy manual work. | Stroke Association brochure |
| `0` | Impossible — image-only, proprietary binary, no accessible source. | — (none in this list) |

### Redistribution tiers

| Tier | Meaning | Action in this repo |
|---|---|---|
| `public_ok` | Open or compatible licence (public domain, CC-BY, CC-BY-SA, MIT, BSD, OGL, Etalab, W3C Document Licence, EU Decision 2011/833/EU). | **OK to commit** extracts under `examples/public/` with attribution. Respect share-alike if applicable. |
| `check_license` | Open in principle but constrained: NC (non-commercial), SA (copyleft), or per-item variance within a dataset/repo. | **Verify per use** before committing. Safer to link and only copy the items whose licence you've confirmed. |
| `link_only` | Copyrighted, publicly reachable, no open licence. | **Link from docs**; don't commit extracts beyond short fair-use quotations (a sentence or two with clear attribution). |
| `restricted` | Access gated (paywall, research-request, click-through agreement) or explicit no-redistribution in the licence. | **Cite only.** Don't include in this repo. |

**Confidence heuristic.** High = peer-reviewed research, normative standards
(WCAG, RGAA, ISO), or national government plain-language programs with
explained rewrites. Medium = reputable associations, well-established tools,
commercial but evidence-linked blogs. Low = individual blogs, forum posts,
or sources asserting claims without explanation.

**Changes from the starter criteria:**

- Split `polarity: both` into `before_after` (aligned pairs, highest teaching
  value) and `mixed_unaligned` (both polarities present but not paired).
- Added `bilingual_parallel` — a resource available *in* EN *and* FR is not
  the same as a resource with *aligned* EN/FR content.
- Dropped `use_case: generator` (no entry justified it; fold into `benchmark`).
- Added `has_explanations` as a separate axis from `polarity` — a
  `before_after` collection without commentary still has less teaching value
  than one with annotations.

## 🗂️ Sources by category / Sources par catégorie

Grouped for scanning. Rules/conditions/languages/redistribution columns
are abbreviated or omitted; see [`texts.yaml`](./texts.yaml) for the
full attribute set per entry. Every source listed below is
`redistribution: public_ok`.

### 🏛️ Plain-language & government style guides

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|
| [plainlanguage.gov before/after](https://www.plainlanguage.gov/examples/before-and-after/) | before_after | EN | high | high | 4 |
| [Federal Plain Language Guidelines](https://www.plainlanguage.gov/guidelines/) | before_after | EN | high | high | 4 |
| [GOV.UK style guide A–Z](https://www.gov.uk/guidance/style-guide/a-to-z-of-gov-uk-style) | mixed | EN | high | high | 4 |
| [GOV.UK — writing for GOV.UK](https://www.gov.uk/guidance/content-design/writing-for-gov-uk) | good | EN | high | high | 4 |
| [EC — How to write clearly](https://op.europa.eu/en/publication-detail/-/publication/725b7eb0-d92e-11e5-8fea-01aa75ed71a1) | before_after | **EN+FR** | high | high | 2 |
| [CDL Readability Guidelines](https://readabilityguidelines.co.uk/) | before_after | EN | high | high | 4 |

### ♿ Accessibility standards

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|
| [WCAG 2.1 — Reading Level (3.1.5)](https://www.w3.org/WAI/WCAG21/Understanding/reading-level.html) | before_after | EN | high | high | 4 |
| [WCAG 2.1 — Unusual Words (3.1.3)](https://www.w3.org/WAI/WCAG21/Understanding/unusual-words.html) | mixed | EN | med | high | 4 |
| [WCAG 2.1 — Abbreviations (3.1.4)](https://www.w3.org/WAI/WCAG21/Understanding/abbreviations.html) | mixed | EN | med | high | 4 |
| [RGAA 4.1 — critères et tests](https://accessibilite.numerique.gouv.fr/methode/criteres-et-tests/) | good | FR | high | high | 4 |

### 📖 Easy-read / FALC / dyslexia / aphasia / ADHD

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|

### 🩺 Health literacy

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|
| [CDC Clear Communication Index](https://www.cdc.gov/ccindex/pdf/clear-communication-user-guide.pdf) | before_after | EN | high | high | 2 |

### ⚖️ Bad-example corpora (legal / admin)

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|
| [EUR-Lex legal corpus](https://eur-lex.europa.eu/) | bad | **EN+FR** | high | high | 4 |
| [Légifrance](https://www.legifrance.gouv.fr/) | bad | FR | high | high | 4 |

### 📊 Simplification & readability corpora

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|
| [Simple English Wikipedia](https://simple.wikipedia.org/wiki/Main_Page) | good | EN | high | high | 5 |
| [Vikidia (FR)](https://fr.vikidia.org/wiki/Accueil) | good | FR | high | high | 5 |
| [OneStopEnglish corpus](https://github.com/nishkalavallabhi/OneStopEnglishCorpus) | before_after | EN | high | high | 5 |
| [ASSET](https://github.com/facebookresearch/asset) | before_after | EN | high | high | 5 |
| [WikiAuto](https://huggingface.co/datasets/wiki_auto) | before_after | EN | high | high | 5 |
| [Project Gutenberg](https://www.gutenberg.org/) | neutral | EN+FR | med | high | 5 |

### 🛠️ Tool fixtures (labeled bad-example banks)

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|
| [proselint](https://github.com/amperser/proselint) | bad | EN | high | high | 5 |
| [write-good](https://github.com/btford/write-good) | bad | EN | med | high | 5 |
| [alex.js](https://github.com/get-alex/alex) | bad | EN | low | high | 5 |
| [retext plugins](https://github.com/retextjs/retext) | bad | EN | med | high | 5 |

### 🔬 Research & documentation

| Source | Polarity | Lang | Interest | Confidence | MD |
|---|---|---|---|---|---|
| [Bakker & Kamps (2024) — Context-aware simplification](https://aclanthology.org/2024.determit-1.3.pdf) | neutral | EN | low | high | 2 |
| [18F plain-writing](https://18f.gsa.gov/2016/05/24/an-open-source-tool-for-plainer-writing/) | mixed | EN | low | med | 4 |

## 🔍 Known gaps / Lacunes connues

- **Dyscalculia** — no solid corpus found; only `structure.mixed-numeric-format`-adjacent WCAG notes.
- **Aphasia-FR** — French aphasia-specific samples beyond SPF/HAS are thin.
- **ADHD** — no before/after prose pairs; evidence base is mostly layout-focused.
- **Legal/EULA with expert plain-language annotations at scale** — ToSDR is the best we have but coverage is uneven.

<!-- coverage:begin -->
<!-- generated by scripts/texts_coverage.py — do not edit by hand -->

## 📏 Coverage snapshot / Couverture

Auto-generated from [`texts.yaml`](./texts.yaml). Cells count **`public_ok` sources only** — the 25 of 55 entries safe to commit under `examples/public/`. A `—` cell means zero redistributable sources for that slot. The full map (including non-redistributable slots and the load-bearing target list for sourcing work) lives at `examples/local/COVERAGE.md`, which is gitignored by design.

### Shape × language

| Shape | EN | FR | EN+FR |
|---|---|---|---|
| good | 2 | 2 | — |
| bad | 4 | 1 | 1 |
| before/after | 8 | — | 1 |
| mixed | 4 | — | — |
| neutral | 1 | — | 1 |

### Condition × language

| Condition | EN | FR | EN+FR |
|---|---|---|---|
| `general` | 19 | 3 | 3 |
| `dyslexia` | 2 | 1 | — |
| `dyscalculia` | — | — | — |
| `aphasia` | 5 | — | — |
| `adhd` | — | — | — |
| `non-native` | 14 | 2 | 2 |
| `a11y-markup` | — | 1 | — |

### Type × language

| Type | EN | FR | EN+FR |
|---|---|---|---|
| `gov_guide` | 3 | — | 1 |
| `style_guide` | 2 | — | — |
| `standard` | 4 | 1 | — |
| `research_paper` | 1 | — | — |
| `corpus` | 2 | 2 | 2 |
| `dataset` | 2 | — | — |
| `tool_fixtures` | 4 | — | — |
| `sample_collection` | — | — | — |
| `textbook` | — | — | — |
| `blog` | 1 | — | — |

<!-- coverage:end -->

## 🔧 Maintenance

To add a source, append an entry to [`texts.yaml`](./texts.yaml) and add one
row to the relevant table above. Evaluate `confidence` against the heuristic
in the schema section — prefer research / normative / government sources
over blogs and forums, and favour sources that *explain* their examples.
