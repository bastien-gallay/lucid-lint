# Example text sources / Textes d'exemple

> Curated index of publicly-accessible text sources usable as **examples** in
> docs, as **fixtures** for tests, or as **corpora** for benchmarks.
>
> Répertoire de sources de textes publiquement accessibles utilisables comme
> **exemples** dans la documentation, **fixtures** pour les tests, ou
> **corpus** pour les benchmarks.

**Not** academic references — those live in [`REFERENCES.md`](./REFERENCES.md).
The machine-readable source of truth for this page is [`texts.yaml`](./texts.yaml).

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

### Redistribution tiers

| Tier | Meaning | Action in this repo |
|---|---|---|
| `public_ok` | Open or compatible licence (public domain, CC-BY, CC-BY-SA, MIT, BSD, OGL, Etalab, W3C Document Licence, EU Decision 2011/833/EU). | **OK to commit** extracts under `examples/` with attribution. Respect share-alike if applicable. |
| `check_license` | Open in principle but constrained: NC (non-commercial), SA (copyleft), or per-item variance within a dataset/repo. | **Verify per use** before committing. Safer to link + extract only the items whose licence you've confirmed. |
| `link_only` | Copyrighted, publicly reachable, no open licence. | **Link from docs**; do not commit extracts beyond short fair-use quotations (a sentence or two with clear attribution). |
| `restricted` | Access gated (paywall, research-request, click-through agreement) or explicit no-redistribution in the licence. | **Internal use only** after accepting the terms. Must not be committed to a public repo. |

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

## 🚦 Redistribution summary — can it go in the public repo?

> Short answer: about half the list is safe to commit with attribution,
> a quarter needs per-item verification, a quarter is link-only, and a
> handful (Newsela, WeeBit, BMJ, Tandfonline) must stay internal.

| Tier | Count | Representative sources |
|---|---|---|
| ✅ `public_ok` | ~23 | plainlanguage.gov, GOV.UK (OGL), EC *How to write clearly* (EU reuse decision), RGAA (Etalab), EUR-Lex, Légifrance, CDC, WCAG pages (W3C Doc Licence), Simple English Wikipedia / Vikidia / OneStopEnglish / ASSET / WikiAuto (CC-BY-SA), Project Gutenberg (PD), CDL Readability (CC-BY-SA), proselint / write-good / alex / retext (MIT/BSD), 18F (CC0) |
| ⚠️ `check_license` | ~11 | Canada.ca (Crown copyright, non-commercial OK), HAS, Santé publique France, ToSDR (CC-BY-SA annotations but ToS excerpts © companies), CLEAR corpus, Alector, CLEAR-FR, EASSE (mixed per-dataset), Vale styles (per-pack), textlint-rule (per-repo), LanguageTool (LGPL — copyleft) |
| 🔗 `link_only` | ~10 | Center for Plain Language, Inclusion Europe, Unapei, IFLA, BDA Dyslexia, Stroke Association, ADHD Foundation, Hemingway, NN/g, Orthodidacte, Readable.com |
| 🔒 `restricted` | ~4 | Newsela (explicit no-redistribution in DUA), WeeBit (BBC/Weekly Reader © + access-request), BMJ article (paywalled), Coh-Metrix / Tandfonline paper (paywalled) |

**Practical recipe for the `examples/` directory**:

1. Prefer `public_ok` sources for anything committed as a fixture or doc example.
2. For `check_license` sources, read the licence of the *specific* file you copy; note the licence in a neighbour `LICENSE.txt` or header comment when bundling.
3. For `link_only` / `restricted`, quote at most a sentence or two with clear attribution, or just link. Keep large extracts of these sources in an untracked path (e.g. `examples/private/` in `.gitignore`) if needed for local experimentation.
4. If a share-alike (SA) source is bundled, make sure the combined work's licence is compatible with SA downstream.

## 🗂️ Sources by category / Sources par catégorie

Grouped for scanning. Rules/conditions/languages/redistribution columns are
abbreviated or omitted; see [`texts.yaml`](./texts.yaml) for the full
attribute set per entry (including `redistribution` and `license_details`).
The summary just above this section gives the redistribution tier for
every source at a glance.

### 🏛️ Plain-language & government style guides

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [plainlanguage.gov before/after](https://www.plainlanguage.gov/examples/before-and-after/) | before_after | EN | high | high |
| [Federal Plain Language Guidelines](https://www.plainlanguage.gov/guidelines/) | before_after | EN | high | high |
| [GOV.UK style guide A–Z](https://www.gov.uk/guidance/style-guide/a-to-z-of-gov-uk-style) | mixed | EN | high | high |
| [GOV.UK — writing for GOV.UK](https://www.gov.uk/guidance/content-design/writing-for-gov-uk) | good | EN | high | high |
| [EC — How to write clearly](https://op.europa.eu/en/publication-detail/-/publication/725b7eb0-d92e-11e5-8fea-01aa75ed71a1) | before_after | **EN+FR** | high | high |
| [Canada.ca content style guide](https://design.canada.ca/writing-style/) | before_after | **EN+FR** | high | high |
| [Canada.ca plain-language](https://www.canada.ca/en/revenue-agency/services/about-canada-revenue-agency-cra/plain-language.html) | before_after | **EN+FR** | high | high |
| [CDL Readability Guidelines](https://readabilityguidelines.co.uk/) | before_after | EN | high | high |
| [Center for Plain Language](https://centerforplainlanguage.org/learning-training/before-after-comparisons/) | before_after | EN | high | high |

### ♿ Accessibility standards

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [WCAG 2.1 — Reading Level (3.1.5)](https://www.w3.org/WAI/WCAG21/Understanding/reading-level.html) | before_after | EN | high | high |
| [WCAG 2.1 — Unusual Words (3.1.3)](https://www.w3.org/WAI/WCAG21/Understanding/unusual-words.html) | mixed | EN | med | high |
| [WCAG 2.1 — Abbreviations (3.1.4)](https://www.w3.org/WAI/WCAG21/Understanding/abbreviations.html) | mixed | EN | med | high |
| [RGAA 4.1 — critères et tests](https://accessibilite.numerique.gouv.fr/methode/criteres-et-tests/) | good | FR | high | high |

### 📖 Easy-read / FALC / dyslexia / aphasia / ADHD

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [Inclusion Europe — Information for All](https://www.inclusion-europe.eu/easy-to-read-standards-guidelines/) | before_after | **EN+FR** | high | high |
| [Unapei — Guide FALC](https://www.unapei.org/publication/guide-pratique-ecrire-facile-a-lire-et-a-comprendre/) | before_after | FR | high | high |
| [IFLA Easy-to-Read](https://www.ifla.org/publications/guidelines-for-easy-to-read-materials/) | mixed | EN+FR | high | high |
| [BDA Dyslexia Style Guide](https://www.bdadyslexia.org.uk/advice/employers/creating-a-dyslexia-friendly-workplace/dyslexia-friendly-style-guide) | good | EN | high | high |
| [Stroke Association — Accessible Information](https://www.stroke.org.uk/sites/default/files/accessible_information_guidelines.pdf) | before_after | EN | high | high |
| [ADHD Foundation — Neurodiversity](https://adhdfoundation.org.uk/information/neurodiversity/) | good | EN | med | med |

### 🩺 Health literacy

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [CDC Clear Communication Index](https://www.cdc.gov/ccindex/pdf/clear-communication-user-guide.pdf) | before_after | EN | high | high |
| [HAS — Littératie et santé](https://www.has-sante.fr/jcms/c_1724101/fr/litteratie-et-sante) | before_after | FR | med | high |
| [SPF — Communiquer pour tous](https://www.santepubliquefrance.fr/docs/communiquer-pour-tous-guide-pour-une-information-accessible) | before_after | FR | high | high |
| [BMJ — Gobbledygook in medicine](https://www.bmj.com/content/360/bmj.k1383) | before_after | EN | med | high |

### ⚖️ Bad-example corpora (legal / admin)

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [EUR-Lex legal corpus](https://eur-lex.europa.eu/) | bad | **EN+FR** | high | high |
| [Légifrance](https://www.legifrance.gouv.fr/) | bad | FR | high | high |
| [Terms of Service; Didn't Read](https://www.tosdr.org/) | before_after | EN+FR | high | high |

### 📊 Simplification & readability corpora

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [Simple English Wikipedia](https://simple.wikipedia.org/wiki/Main_Page) | good | EN | high | high |
| [Vikidia (FR)](https://fr.vikidia.org/wiki/Accueil) | good | FR | high | high |
| [OneStopEnglish corpus](https://github.com/nishkalavallabhi/OneStopEnglishCorpus) | before_after | EN | high | high |
| [Newsela corpus](https://newsela.com/data/) | before_after | EN | high | high (restricted) |
| [WeeBit corpus](https://zenodo.org/record/1219041) | mixed | EN | med | high (restricted) |
| [CLEAR corpus (CommonLit)](https://github.com/scrosseye/CLEAR-Corpus) | mixed | EN | high | high |
| [Alector corpus](https://corpusalector.huma-num.fr/) | before_after | FR | high | high |
| [CLEAR — FR (Grabar & Cardon)](https://aclanthology.org/2020.lrec-1.530/) | before_after | FR | high | high |
| [EASSE](https://github.com/feralvam/easse) | before_after | EN | high | high |
| [ASSET](https://github.com/facebookresearch/asset) | before_after | EN | high | high |
| [WikiAuto](https://huggingface.co/datasets/wiki_auto) | before_after | EN | high | high |
| [Project Gutenberg](https://www.gutenberg.org/) | neutral | EN+FR | med | high |

### 🛠️ Tool fixtures (labeled bad-example banks)

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [proselint](https://github.com/amperser/proselint) | bad | EN | high | high |
| [Vale styles](https://github.com/errata-ai/vale) | bad | EN | high | high |
| [write-good](https://github.com/btford/write-good) | bad | EN | med | high |
| [alex.js](https://github.com/get-alex/alex) | bad | EN | low | high |
| [textlint-rule](https://github.com/textlint-rule) | bad | EN | med | high |
| [retext plugins](https://github.com/retextjs/retext) | bad | EN | med | high |
| [LanguageTool](https://languagetool.org/) | bad | **EN+FR** | high | high |
| [Hemingway Editor](https://www.hemingwayapp.com/) | bad | EN | med | med |

### 🔬 Research & documentation

| Source | Polarity | Lang | Interest | Confidence |
|---|---|---|---|---|
| [Coh-Metrix / McNamara](https://www.tandfonline.com/doi/full/10.1080/01638539809545028) | neutral | EN | med | high (restricted) |
| [NN/g — Plain Language](https://www.nngroup.com/articles/plain-language-experts/) | before_after | EN | med | med |
| [18F plain-writing](https://18f.gsa.gov/2016/05/24/an-open-source-tool-for-plainer-writing/) | mixed | EN | low | med |
| [Orthodidacte blog](https://www.orthodidacte.com/blog/) | before_after | FR | low | med |
| [Readable.com blog](https://readable.com/blog/) | mixed | EN | low | med |

## 🔍 Known gaps / Lacunes connues

- **Dyscalculia** — no solid corpus found; only `structure.mixed-numeric-format`-adjacent WCAG notes.
- **Aphasia-FR** — French aphasia-specific samples beyond SPF/HAS are thin.
- **ADHD** — no before/after prose pairs; evidence base is mostly layout-focused.
- **Legal/EULA with expert plain-language annotations at scale** — ToSDR is the best we have but coverage is uneven.

## 📏 Coverage snapshot / Couverture

- **Bilingual-parallel resources** (aligned EN/FR, valuable for our stance):
  EC *How to write clearly*, Canada.ca (×2), Inclusion Europe, EUR-Lex.
- **FR-only** entries: RGAA, Unapei, Vikidia, Alector, CLEAR-FR, HAS, SPF,
  Légifrance, Orthodidacte.
- **Before/after with explanations**: plainlanguage.gov, EC, Canada.ca,
  CDL Readability, Center for Plain Language, Inclusion Europe, Unapei,
  Stroke Association, CDC, HAS, SPF, ToSDR, NN/g.

## 🔧 Maintenance

To add a source, append an entry to [`texts.yaml`](./texts.yaml) and add one
row to the relevant table above. Evaluate `confidence` against the heuristic
in the schema section — prefer research / normative / government sources
over blogs and forums, and favour sources that *explain* their examples.
