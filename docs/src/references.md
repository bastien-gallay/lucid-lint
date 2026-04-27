# References

> Academic, normative, and practical sources that inform the design of `lucid-lint`.

This page lists the references that shaped `lucid-lint`'s rules, profiles, and design decisions. Each entry states where the reference matters in the project. The French mirror lives at [`fr/references.md`](./fr/references.md).

## Legend

| Status | Meaning                                                 |
| ------ | ------------------------------------------------------- |
| ✅      | Verified — canonical reference                          |
| ⚠️      | To verify — likely correct, confirm citation details    |
| 🔍      | Opportunistic — sound rationale, citation may be looser |
| 📖      | Book / secondary source                                 |
| 🌐      | Normative standard                                      |
| 🧪      | Practical source (style guide, tool)                    |

## Cognitive Load Theory — the backbone

The theoretical core of `lucid-lint`: prose imposes a mental cost on the reader, and this cost can be measured and reduced.

**✅ Sweller, J. (1988).** *Cognitive load during problem solving: Effects on learning.* Cognitive Science, 12(2), 257–285.

Foundational paper. Distinguishes intrinsic, extraneous, and germane load. Justifies the core premise that poor prose imposes extraneous load that can be reduced through better structure.

→ Relevant to: most rules, especially `structure.*`, `rhythm.*`, `syntax.nested-negation`, `syntax.conditional-stacking`.

**📖 Sweller, J., Ayres, P., & Kalyuga, S. (2011).** *Cognitive Load Theory.* Springer.

Modern synthesis of 30 years of research.

## Text cohesion and discourse processing

**✅ Graesser, A. C., McNamara, D. S., Louwerse, M. M., & Cai, Z. (2004).** *Coh-Metrix: Analysis of text on cohesion and language.* Behavior Research Methods, Instruments, & Computers, 36(2), 193–202.

The reference paper for automated cohesion analysis. Over 200 linguistic indices measuring local and global cohesion. Our rules are simplified, deterministic versions of several Coh-Metrix metrics.

→ Relevant to: `rhythm.repetitive-connectors`, `syntax.unclear-antecedent`, `lexicon.low-lexical-diversity`.

**📖 McNamara, D. S., Graesser, A. C., McCarthy, P. M., & Cai, Z. (2014).** *Automated evaluation of text and discourse with Coh-Metrix.* Cambridge University Press.

## Syntactic complexity

**✅ Gibson, E. (1998).** *Linguistic complexity: Locality of syntactic dependencies.* Cognition, 68(1), 1–76.

Foundational paper on Dependency Locality Theory. Formalizes the cost of holding distant grammatical referents in working memory.

→ Relevant to: `structure.deep-subordination`, `syntax.unclear-antecedent`, `syntax.conditional-stacking`.

## Discourse connectors

**✅ Sanders, T. J. M., & Noordman, L. G. M. (2000).** *The role of coherence relations and their linguistic markers in text processing.* Discourse Processes, 29(1), 37–60.

Central reference on how logical connectors guide or confuse readers.

→ Relevant to: `rhythm.repetitive-connectors`.

## Readability formulas

**✅ Flesch, R. (1948).** *A new readability yardstick.* Journal of Applied Psychology, 32(3), 221–233.

Original paper for the Flesch Reading Ease formula.

**✅ Kincaid, J. P., Fishburne, R. P., Rogers, R. L., & Chissom, B. S. (1975).** *Derivation of new readability formulas for Navy enlisted personnel.* Technical Report, Naval Technical Training Command.

Origin of the Flesch-Kincaid Grade Level formula used in v0.1.

→ Relevant to: `readability.score`.

**📖 McLaughlin, G. H. (1969).** *SMOG grading: A new readability formula.* Journal of Reading, 12(8), 639–646.

Alternative readability formula. Candidate for v0.2.

## Lexical diversity

**📖 Herdan, G. (1960).** *Type-Token Mathematics: A Textbook of Mathematical Linguistics.*

Origin of the Type-Token Ratio used in lexical diversity analysis.

→ Relevant to: `lexicon.low-lexical-diversity`.

**✅ McCarthy, P. M., & Jarvis, S. (2010).** *MTLD, vocd-D, and HD-D: A validation study of sophisticated approaches to lexical diversity assessment.* Behavior Research Methods, 42(2), 381–392.

## Negation processing

**✅ Clark, H. H., & Chase, W. G. (1972).** *On the process of comparing sentences against pictures.* Cognitive Psychology, 3(3), 472–517.

Classic experimental work showing that negative sentences take longer to verify than affirmative ones. Foundational evidence that negation carries a comprehension cost.

→ Relevant to: `syntax.nested-negation`.

**✅ Carpenter, P. A., & Just, M. A. (1975).** *Sentence comprehension: A psycholinguistic processing model of verification.* Psychological Review, 82(1), 45–73.

Extends Clark & Chase with a formal model of sentence processing. Stacked negations compound the verification cost.

→ Relevant to: `syntax.nested-negation`.

**🔍 Kaup, B., Lüdtke, J., & Zwaan, R. A. (2006).** *Processing negated sentences with contradictory predicates: Is a door that is not open mentally closed?* Journal of Pragmatics, 38(7), 1033–1050.

Modern reference on negation processing. Useful if you want to go deeper.

## Conditional reasoning

**🔍 Johnson-Laird, P. N., & Byrne, R. M. J. (1991).** *Deduction.* Psychology Press.

Mental models theory of conditional reasoning. Stacked conditionals multiply the number of mental models the reader must maintain.

→ Relevant to: `syntax.conditional-stacking`.

**🔍 Evans, J. St. B. T., & Over, D. E. (2004).** *If.* Oxford University Press.

Comprehensive review of the psychology of conditionals. More accessible than Johnson-Laird for non-specialists.

> 🔍 **Note**: the link between chained conditionals and reader cognitive load is intuitive and well-supported by the broader reasoning literature, but the *specific* rule "more than N conditionals per sentence is harmful" is a practitioner heuristic, not a directly tested threshold. Treat the threshold as configurable and empirically calibrated.

## Typography and visual processing

**🔍 Arditi, A., & Cho, J. (2007).** *Letter case and text legibility in normal and low vision.* Vision Research, 47(19), 2499–2505.

Empirical evidence on the reading-speed cost of all-caps text: readers lose the word-shape cues that mixed-case ascenders and descenders provide.

→ Relevant to: `lexicon.all-caps-shouting`.

**🧪 Nielsen, J. (Nielsen Norman Group).** Multiple articles on all-caps readability in user interfaces.

Industry-standard reference on why ALL-CAPS text reduces reading speed.

→ Relevant to: `lexicon.all-caps-shouting`.

**📖 Bringhurst, R. (2013).** *The Elements of Typographic Style* (4th ed.). Hartley & Marks.

Canonical reference on typography. Supports the principle that uniform-height text (all-caps) slows reading compared to mixed-case.

**✅ Legge, G. E., & Bigelow, C. A. (2011).** *Does print size matter for reading? A review of findings from vision science and typography.* Journal of Vision, 11(5).

Review of vision-science evidence on reading. Covers line-length effects among other factors.

→ Relevant to: `structure.line-length-wide`.

## Phonological complexity and reading

**🔍 Seidenberg, M. S., Waters, G. S., Barnes, M. A., & Tanenhaus, M. K. (1984).** *When does irregular spelling or pronunciation influence word recognition?* Journal of Verbal Learning and Verbal Behavior, 23(3), 383–404.

Classic work showing that unusual letter patterns slow word recognition.

**🔍 Treiman, R., Kessler, B., Zevin, J. D., Bick, S., & Davis, M. (2006).** *Influence of consonantal context on the reading of vowels: Evidence from children.* Journal of Experimental Child Psychology, 93(1), 1–24.

Work showing that consonant clusters and their context affect reading accuracy and speed.

> 🔍 **Honest caveat**: the `lexicon.consonant-cluster` rule is grounded in the broader literature on word-form complexity, but a specific validated threshold like "4+ consonants in a row is harmful" does not come from a single canonical paper. The rule is a practitioner heuristic informed by the literature, not a direct transposition of a published metric.

## Intensifiers and hedges

**🔍 Quirk, R., Greenbaum, S., Leech, G., & Svartvik, J. (1985).** *A Comprehensive Grammar of the English Language.* Longman.

Classical grammar reference classifying intensifiers as "amplifiers" whose semantic contribution is often marginal. Justifies flagging them as low-value words.

→ Relevant to: `lexicon.redundant-intensifier`.

**🧪 Zinsser, W. (2006).** *On Writing Well* (30th anniversary ed.). HarperCollins.

Practical guide that famously argues against adverb intensifiers ("very", "really", "quite") as clutter. Not academic, but widely cited in writing pedagogy.

## Style guides and plain language

**📖🧪 Strunk, W., & White, E. B. (1999).** *The Elements of Style* (4th ed.). Longman.

The canonical English writing guide. Codifies active voice, concision, clear pronouns, and warns against qualifiers like weasel words and intensifiers.

→ Relevant to: `syntax.passive-voice`, `lexicon.weasel-words`, `lexicon.redundant-intensifier`, `syntax.unclear-antecedent`.

**🧪 US Plain Language Action and Information Network (2011).** *Federal Plain Language Guidelines.* <https://www.plainlanguage.gov/>

Grounds short sentences, active voice, no nominalization, no jargon.

→ Relevant to: `structure.sentence-too-long`, `structure.paragraph-too-long`, `lexicon.excessive-nominalization`, `lexicon.jargon-undefined`, `syntax.passive-voice`.

**🧪 European Commission (2011).** *How to write clearly.* Publications Office of the European Union.

European plain-language equivalent in all EU languages.

## Numeric formatting conventions

**🌐 International Organization for Standardization (2022).** *ISO 80000-1:2022 — Quantities and units — Part 1: General.*

International standard on numeric formatting, including digit grouping and decimal separators. Grounds the idea that mixing formats within a single text impairs scanning.

→ Relevant to: `structure.mixed-numeric-format`.

**🧪 The Chicago Manual of Style (17th ed., 2017).** University of Chicago Press.

Canonical style guide covering when to spell numbers out vs. use digits, and why consistency matters.

→ Relevant to: `structure.mixed-numeric-format`.

## Working memory and attention

**⚠️ Martinussen, R., Hayden, J., Hogg-Johnson, S., & Tannock, R. (2005).** *A meta-analysis of working memory impairments in children with attention-deficit/hyperactivity disorder.* Journal of the American Academy of Child & Adolescent Psychiatry, 44(4), 377–384.

> ⚠️ **Caveat**: direct research on "text readability for ADHD readers" is dispersed and of variable quality. The cognitive accessibility angle is sound, but treat specific ADHD claims carefully.

**📖 Barkley, R. A. (2012).** *Executive Functions: What They Are, How They Work, and Why They Evolved.* The Guilford Press.

## Dyslexia and visual accessibility

**✅ Rello, L., & Baeza-Yates, R. (2013).** *Good fonts for dyslexia.* Proceedings of ASSETS '13.

Empirical research on font choice impact for dyslexic readers.

## Concreteness norms

**✅ Brysbaert, M., Warriner, A. B., & Kuperman, V. (2014).** *Concreteness ratings for 40 thousand generally known English word lemmas.* Behavior Research Methods, 46(3), 904–911.

→ Relevant to: possible future rule "abstractness density" (not in v0.1).

## Normative standards

**🌐 W3C (2018).** *Web Content Accessibility Guidelines (WCAG) 2.1.* <https://www.w3.org/TR/WCAG21/>

Key criteria invoked:

- **1.3.1** (Info and Relationships) → `structure.heading-jump`
- **1.4.8** (Visual Presentation) — line width ≤ 80 characters → `structure.line-length-wide`
- **2.4.6** (Headings and Labels) → `structure.heading-jump`
- **3.1.3** (Unusual Words) → `lexicon.jargon-undefined`
- **3.1.4** (Abbreviations) → `lexicon.unexplained-abbreviation`
- **3.1.5** (Reading Level) → `readability.score`

> ⚠️ Verify exact criterion numbers against the WCAG version you want to cite (2.1 or 2.2).

**🌐 Accessibility Standards Canada (2025).** *CAN-ASC-3.1:2025 — Plain Language* (first edition). <https://accessible.canada.ca/creating-accessibility-standards/summary-standard-can-asc-312025-plain-language>

First-edition Canadian national standard on plain language, published bilingually by Accessibility Standards Canada under the *Accessible Canada Act*. Prescriptive (`shall` / `should` / `may`) requirements over five areas: audience identification, evaluation methods, structure, wording, design. Grounds many of our `lexicon.*`, `structure.*`, and `readability.score` defaults independently of the US / EU plain-language canons.

→ Relevant to: `lexicon.jargon-undefined`, `lexicon.unexplained-abbreviation`, `lexicon.weasel-words`, `structure.sentence-too-long`, `structure.paragraph-too-long`, `syntax.passive-voice`, `readability.score`.

## European legal context

**🌐 Directive (EU) 2019/882** of the European Parliament and of the Council of 17 April 2019 — *European Accessibility Act (EAA)*.

Legal framework extending accessibility requirements to private-sector services from 28 June 2025.

## Practical tools that shaped our design

- **🧪 Coh-Metrix** (Graesser & McNamara) — <https://soletlab.asu.edu/coh-metrix/>
- **🧪 Vale** (Chris Ward) — <https://vale.sh/>
- **🧪 textlint** — <https://textlint.github.io/>
- **🧪 Hemingway Editor** — <https://hemingwayapp.com/>
- **🧪 Proselint** — <https://github.com/amperser/proselint>

## Rule → reference summary

### Lexicon

| Rule                               | Primary references                                                         |
| ---------------------------------- | -------------------------------------------------------------------------- |
| `lexicon.all-caps-shouting`        | Arditi & Cho (2007); Nielsen Norman Group; Bringhurst (2013)               |
| `lexicon.consonant-cluster`        | Seidenberg et al. (1984); Treiman et al. (2006) — 🔍 practitioner heuristic |
| `lexicon.excessive-nominalization` | Plain Language US; FALC; CAN-ASC-3.1:2025                                  |
| `lexicon.jargon-undefined`         | WCAG 3.1.3; Plain Language US; FALC; CAN-ASC-3.1:2025                      |
| `lexicon.low-lexical-diversity`    | Herdan (1960); McCarthy & Jarvis (2010); Graesser et al. (2004)            |
| `lexicon.redundant-intensifier`    | Strunk & White; Quirk et al. (1985); Zinsser (2006)                        |
| `lexicon.unexplained-abbreviation` | WCAG 3.1.4; RGAA 9.4; CAN-ASC-3.1:2025                                     |
| `lexicon.weasel-words`             | Strunk & White; Wikipedia style guide; CAN-ASC-3.1:2025                    |

### Readability

| Rule                | Primary references                                                                          |
| ------------------- | ------------------------------------------------------------------------------------------- |
| `readability.score` | Flesch (1948); Kincaid et al. (1975); Henry (1975); Kandel & Moles (1958); CAN-ASC-3.1:2025 |

### Rhythm

| Rule                                | Primary references                                |
| ----------------------------------- | ------------------------------------------------- |
| `rhythm.consecutive-long-sentences` | Sweller (1988); Sweller et al. (2011)             |
| `rhythm.repetitive-connectors`      | Sanders & Noordman (2000); Graesser et al. (2004) |

### Structure

| Rule                             | Primary references                                        |
| -------------------------------- | --------------------------------------------------------- |
| `structure.deep-subordination`   | Gibson (1998); FALC                                       |
| `structure.deeply-nested-lists`  | WCAG 2.1; cognitive load heuristics                       |
| `structure.excessive-commas`     | Gibson (1998) — 🔍 practitioner heuristic                  |
| `structure.heading-jump`         | WCAG 1.3.1 & 2.4.6; RGAA 9.1                              |
| `structure.line-length-wide`     | WCAG 1.4.8 (AAA); Legge & Bigelow (2011)                  |
| `structure.long-enumeration`     | FALC; Plain Language US                                   |
| `structure.mixed-numeric-format` | ISO 80000-1; Chicago Manual of Style                      |
| `structure.paragraph-too-long`   | Sweller (1988); Graesser et al. (2004); CAN-ASC-3.1:2025  |
| `structure.sentence-too-long`    | Sweller (1988); Plain Language US; FALC; CAN-ASC-3.1:2025 |

### Syntax

| Rule                             | Primary references                                                                                       |
| -------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `syntax.conditional-stacking`    | Johnson-Laird & Byrne (1991); Evans & Over (2004); Gibson (1998) — 🔍 threshold is practitioner heuristic |
| `syntax.dense-punctuation-burst` | Sweller (1988); Gibson (1998) — 🔍 purely heuristic                                                       |
| `syntax.nested-negation`         | Clark & Chase (1972); Carpenter & Just (1975); Kaup et al. (2006)                                        |
| `syntax.passive-voice`           | Strunk & White; Plain Language US; FALC; CAN-ASC-3.1:2025                                                |
| `syntax.unclear-antecedent`      | Strunk & White; Gibson (1998); Graesser et al. (2004)                                                    |

## On scholarly honesty

`lucid-lint` is an engineering project informed by research, not a research project itself. The references above ground our design choices but we do not claim to validate new findings. Several rules (`lexicon.consonant-cluster`, `syntax.conditional-stacking`, `syntax.dense-punctuation-burst`, `structure.excessive-commas`) are practitioner heuristics informed by the literature rather than direct transpositions of published metrics — we mark these with 🔍 in the summary table.

Where we simplify an academic metric (e.g., `syntax.unclear-antecedent` as a pattern heuristic vs. full anaphora resolution), we document the simplification in [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) and plan richer versions in the [roadmap](./roadmap.md).

If you are a researcher and spot an error, an outdated citation, or a misattribution, please [open an issue](https://github.com/bastien-gallay/lucid-lint/issues) — we will correct it promptly and credit you.
