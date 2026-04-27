# References

> Academic, normative, and practical sources that inform the design of `lucid-lint`.

This page lists the references that shaped `lucid-lint`'s rules, profiles, and design decisions. Each entry states where the reference matters in the project. The French mirror lives at [`fr/references.md`](./fr/references.md).

External links open in a new tab; we mark them `rel="nofollow noopener noreferrer"` so the new-tab is safe and the docs site does not vouch for outside content.

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

<a id="sweller-1988"></a>**✅ Sweller, J. (1988).** *Cognitive load during problem solving: Effects on learning.* Cognitive Science, 12(2), 257–285. <a href="https://doi.org/10.1207/s15516709cog1202_4" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Foundational paper. Distinguishes intrinsic, extraneous, and germane load. Justifies the core premise that poor prose imposes extraneous load that can be reduced through better structure.

→ Relevant to: most rules, especially `structure.*`, `rhythm.*`, [`syntax.nested-negation`](./rules/nested-negation.md), [`syntax.conditional-stacking`](./rules/conditional-stacking.md).

<a id="sweller-2011"></a>**📖 Sweller, J., Ayres, P., & Kalyuga, S. (2011).** *Cognitive Load Theory.* Springer. <a href="https://doi.org/10.1007/978-1-4419-8126-4" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Modern synthesis of 30 years of research.

## Text cohesion and discourse processing

<a id="graesser-2004"></a>**✅ Graesser, A. C., McNamara, D. S., Louwerse, M. M., & Cai, Z. (2004).** *Coh-Metrix: Analysis of text on cohesion and language.* Behavior Research Methods, Instruments, & Computers, 36(2), 193–202. <a href="https://doi.org/10.3758/BF03195564" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

The reference paper for automated cohesion analysis. Over 200 linguistic indices measuring local and global cohesion. Our rules are simplified, deterministic versions of several Coh-Metrix metrics.

→ Relevant to: [`rhythm.repetitive-connectors`](./rules/repetitive-connectors.md), [`syntax.unclear-antecedent`](./rules/unclear-antecedent.md), [`lexicon.low-lexical-diversity`](./rules/low-lexical-diversity.md).

<a id="mcnamara-2014"></a>**📖 McNamara, D. S., Graesser, A. C., McCarthy, P. M., & Cai, Z. (2014).** *Automated evaluation of text and discourse with Coh-Metrix.* Cambridge University Press. <a href="https://www.cambridge.org/core/books/automated-evaluation-of-text-and-discourse-with-cohmetrix/AE4A1D5DCCBA1AE3A9632E9D4D380270" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.cambridge.org — opens in new tab">↗</a>

## Syntactic complexity

<a id="gibson-1998"></a>**✅ Gibson, E. (1998).** *Linguistic complexity: Locality of syntactic dependencies.* Cognition, 68(1), 1–76. <a href="https://doi.org/10.1016/S0010-0277(98)00034-1" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Foundational paper on Dependency Locality Theory. Formalizes the cost of holding distant grammatical referents in working memory.

→ Relevant to: [`structure.deep-subordination`](./rules/deep-subordination.md), [`syntax.unclear-antecedent`](./rules/unclear-antecedent.md), [`syntax.conditional-stacking`](./rules/conditional-stacking.md).

## Discourse connectors

<a id="sanders-noordman-2000"></a>**✅ Sanders, T. J. M., & Noordman, L. G. M. (2000).** *The role of coherence relations and their linguistic markers in text processing.* Discourse Processes, 29(1), 37–60. <a href="https://doi.org/10.1207/S15326950dp2901_3" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Central reference on how logical connectors guide or confuse readers.

→ Relevant to: [`rhythm.repetitive-connectors`](./rules/repetitive-connectors.md).

## Readability formulas

<a id="flesch-1948"></a>**✅ Flesch, R. (1948).** *A new readability yardstick.* Journal of Applied Psychology, 32(3), 221–233. <a href="https://doi.org/10.1037/h0057532" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Original paper for the Flesch Reading Ease formula.

<a id="kincaid-1975"></a>**✅ Kincaid, J. P., Fishburne, R. P., Rogers, R. L., & Chissom, B. S. (1975).** *Derivation of new readability formulas for Navy enlisted personnel.* Technical Report, Naval Technical Training Command. <a href="https://apps.dtic.mil/sti/citations/ADA006655" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="apps.dtic.mil — opens in new tab">↗</a>

Origin of the Flesch-Kincaid Grade Level formula used in v0.1.

→ Relevant to: [`readability.score`](./rules/readability-score.md).

<a id="mclaughlin-1969"></a>**📖 McLaughlin, G. H. (1969).** *SMOG grading: A new readability formula.* Journal of Reading, 12(8), 639–646. <a href="https://ogg.osu.edu/media/documents/health_lit/WRRSMOG_Readability_Formula_G._Harry_McLaughlin__1969_.pdf" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="ogg.osu.edu — opens in new tab">↗</a>

Alternative readability formula. Candidate for v0.2.

## Lexical diversity

<a id="herdan-1960"></a>**📖 Herdan, G. (1960).** *Type-Token Mathematics: A Textbook of Mathematical Linguistics.*

Origin of the Type-Token Ratio used in lexical diversity analysis.

→ Relevant to: [`lexicon.low-lexical-diversity`](./rules/low-lexical-diversity.md).

<a id="mccarthy-jarvis-2010"></a>**✅ McCarthy, P. M., & Jarvis, S. (2010).** *MTLD, vocd-D, and HD-D: A validation study of sophisticated approaches to lexical diversity assessment.* Behavior Research Methods, 42(2), 381–392. <a href="https://doi.org/10.3758/BRM.42.2.381" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

## Negation processing

<a id="clark-chase-1972"></a>**✅ Clark, H. H., & Chase, W. G. (1972).** *On the process of comparing sentences against pictures.* Cognitive Psychology, 3(3), 472–517. <a href="https://doi.org/10.1016/0010-0285(72)90019-9" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Classic experimental work showing that negative sentences take longer to verify than affirmative ones. Foundational evidence that negation carries a comprehension cost.

→ Relevant to: [`syntax.nested-negation`](./rules/nested-negation.md).

<a id="carpenter-just-1975"></a>**✅ Carpenter, P. A., & Just, M. A. (1975).** *Sentence comprehension: A psycholinguistic processing model of verification.* Psychological Review, 82(1), 45–73. <a href="https://doi.org/10.1037/h0076248" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Extends Clark & Chase with a formal model of sentence processing. Stacked negations compound the verification cost.

→ Relevant to: [`syntax.nested-negation`](./rules/nested-negation.md).

<a id="kaup-2006"></a>**🔍 Kaup, B., Lüdtke, J., & Zwaan, R. A. (2006).** *Processing negated sentences with contradictory predicates: Is a door that is not open mentally closed?* Journal of Pragmatics, 38(7), 1033–1050. <a href="https://doi.org/10.1016/j.pragma.2005.09.012" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Modern reference on negation processing. Useful if you want to go deeper.

## Conditional reasoning

<a id="johnson-laird-byrne-1991"></a>**🔍 Johnson-Laird, P. N., & Byrne, R. M. J. (1991).** *Deduction.* Psychology Press. <a href="https://psycnet.apa.org/record/1991-97828-000" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="psycnet.apa.org — opens in new tab">↗</a>

Mental models theory of conditional reasoning. Stacked conditionals multiply the number of mental models the reader must maintain.

→ Relevant to: [`syntax.conditional-stacking`](./rules/conditional-stacking.md).

<a id="evans-over-2004"></a>**🔍 Evans, J. St. B. T., & Over, D. E. (2004).** *If.* Oxford University Press. <a href="https://global.oup.com/academic/product/if-9780198525134" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="global.oup.com — opens in new tab">↗</a>

Comprehensive review of the psychology of conditionals. More accessible than Johnson-Laird for non-specialists.

> 🔍 **Caveat**: the link between chained conditionals and reader cognitive load is intuitive and well-supported by the broader reasoning literature, but the *specific* rule "more than N conditionals per sentence is harmful" is a practitioner heuristic, not a directly tested threshold. Treat the threshold as configurable and empirically calibrated.

## Typography and visual processing

<a id="arditi-cho-2007"></a>**🔍 Arditi, A., & Cho, J. (2007).** *Letter case and text legibility in normal and low vision.* Vision Research, 47(19), 2499–2505. <a href="https://doi.org/10.1016/j.visres.2007.06.010" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Empirical evidence on the reading-speed cost of all-caps text: readers lose the word-shape cues that mixed-case ascenders and descenders provide.

→ Relevant to: [`lexicon.all-caps-shouting`](./rules/all-caps-shouting.md).

<a id="nielsen-norman-allcaps"></a>**🧪 Nielsen, J. (Nielsen Norman Group).** Multiple articles on all-caps readability in user interfaces.

Industry-standard reference on why ALL-CAPS text reduces reading speed.

→ Relevant to: [`lexicon.all-caps-shouting`](./rules/all-caps-shouting.md).

<a id="bringhurst-2013"></a>**📖 Bringhurst, R. (2013).** *The Elements of Typographic Style* (4th ed.). Hartley & Marks.

Canonical reference on typography. Supports the principle that uniform-height text (all-caps) slows reading compared to mixed-case.

<a id="legge-bigelow-2011"></a>**✅ Legge, G. E., & Bigelow, C. A. (2011).** *Does print size matter for reading? A review of findings from vision science and typography.* Journal of Vision, 11(5). <a href="https://doi.org/10.1167/11.5.8" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Review of vision-science evidence on reading. Covers line-length effects among other factors.

→ Relevant to: [`structure.line-length-wide`](./rules/line-length-wide.md).

## Phonological complexity and reading

<a id="seidenberg-1984"></a>**🔍 Seidenberg, M. S., Waters, G. S., Barnes, M. A., & Tanenhaus, M. K. (1984).** *When does irregular spelling or pronunciation influence word recognition?* Journal of Verbal Learning and Verbal Behavior, 23(3), 383–404. <a href="https://doi.org/10.1016/S0022-5371(84)90270-6" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Classic work showing that unusual letter patterns slow word recognition.

<a id="treiman-2006"></a>**🔍 Treiman, R., Kessler, B., Zevin, J. D., Bick, S., & Davis, M. (2006).** *Influence of consonantal context on the reading of vowels: Evidence from children.* Journal of Experimental Child Psychology, 93(1), 1–24. <a href="https://doi.org/10.1016/j.jecp.2005.06.008" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Work showing that consonant clusters and their context affect reading accuracy and speed.

> 🔍 **Caveat**: the [`lexicon.consonant-cluster`](./rules/consonant-cluster.md) rule is grounded in the broader literature on word-form complexity, but a specific validated threshold like "4+ consonants in a row is harmful" does not come from a single canonical paper. The rule is a practitioner heuristic informed by the literature, not a direct transposition of a published metric.

## Intensifiers and hedges

<a id="quirk-1985"></a>**🔍 Quirk, R., Greenbaum, S., Leech, G., & Svartvik, J. (1985).** *A Comprehensive Grammar of the English Language.* Longman.

Classical grammar reference classifying intensifiers as "amplifiers" whose semantic contribution is often marginal. Justifies flagging them as low-value words.

→ Relevant to: [`lexicon.redundant-intensifier`](./rules/redundant-intensifier.md).

<a id="zinsser-2006"></a>**🧪 Zinsser, W. (2006).** *On Writing Well* (30th anniversary ed.). HarperCollins.

Practical guide that famously argues against adverb intensifiers ("very", "really", "quite") as clutter. Not academic, but widely cited in writing pedagogy.

## Style guides and plain language

<a id="strunk-white-1999"></a>**📖🧪 Strunk, W., & White, E. B. (1999).** *The Elements of Style* (4th ed.). Longman.

The canonical English writing guide. Codifies active voice, concision, clear pronouns, and warns against qualifiers like weasel words and intensifiers.

→ Relevant to: [`syntax.passive-voice`](./rules/passive-voice.md), [`lexicon.weasel-words`](./rules/weasel-words.md), [`lexicon.redundant-intensifier`](./rules/redundant-intensifier.md), [`syntax.unclear-antecedent`](./rules/unclear-antecedent.md).

<a id="plain-language-us-2011"></a>**🧪 US Plain Language Action and Information Network (2011).** *Federal Plain Language Guidelines.* <a href="https://www.plainlanguage.gov/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.plainlanguage.gov — opens in new tab">↗</a>

Grounds short sentences, active voice, no nominalization, no jargon.

→ Relevant to: [`structure.sentence-too-long`](./rules/sentence-too-long.md), [`structure.paragraph-too-long`](./rules/paragraph-too-long.md), [`lexicon.excessive-nominalization`](./rules/excessive-nominalization.md), [`lexicon.jargon-undefined`](./rules/jargon-undefined.md), [`syntax.passive-voice`](./rules/passive-voice.md).

<a id="ec-write-clearly-2011"></a>**🧪 European Commission (2011).** *How to write clearly.* Publications Office of the European Union. <a href="https://op.europa.eu/en/publication-detail/-/publication/725b7eb0-d92e-11e5-8fea-01aa75ed71a1" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="op.europa.eu — opens in new tab">↗</a>

European plain-language equivalent in all EU languages.

## Numeric formatting conventions

<a id="iso-80000-1-2022"></a>**🌐 International Organization for Standardization (2022).** *ISO 80000-1:2022 — Quantities and units — Part 1: General.* <a href="https://www.iso.org/standard/76921.html" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.iso.org — opens in new tab">↗</a>

International standard on numeric formatting, including digit grouping and decimal separators. Grounds the idea that mixing formats within a single text impairs scanning.

→ Relevant to: [`structure.mixed-numeric-format`](./rules/mixed-numeric-format.md).

<a id="chicago-manual-17e"></a>**🧪 The Chicago Manual of Style (17th ed., 2017).** University of Chicago Press. <a href="https://press.uchicago.edu/sites/cmos17/index.html" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="press.uchicago.edu — opens in new tab">↗</a>

Canonical style guide covering when to spell numbers out vs. use digits, and why consistency matters.

→ Relevant to: [`structure.mixed-numeric-format`](./rules/mixed-numeric-format.md).

## Working memory and attention

<a id="martinussen-2005"></a>**⚠️ Martinussen, R., Hayden, J., Hogg-Johnson, S., & Tannock, R. (2005).** *A meta-analysis of working memory impairments in children with attention-deficit/hyperactivity disorder.* Journal of the American Academy of Child & Adolescent Psychiatry, 44(4), 377–384. <a href="https://doi.org/10.1097/01.chi.0000153228.72591.73" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

> ⚠️ **Caveat**: direct research on "text readability for ADHD readers" is dispersed and of variable quality. The cognitive accessibility angle is sound, but treat specific ADHD claims carefully.

<a id="barkley-2012"></a>**📖 Barkley, R. A. (2012).** *Executive Functions: What They Are, How They Work, and Why They Evolved.* The Guilford Press. <a href="https://www.guilford.com/books/Executive-Functions/Russell-Barkley/9781462545933" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.guilford.com — opens in new tab">↗</a>

## Dyslexia and visual accessibility

<a id="rello-baeza-yates-2013"></a>**✅ Rello, L., & Baeza-Yates, R. (2013).** *Good fonts for dyslexia.* Proceedings of ASSETS '13. <a href="https://doi.org/10.1145/2513383.2513447" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Empirical research on font choice impact for dyslexic readers.

## Concreteness norms

<a id="brysbaert-2014"></a>**✅ Brysbaert, M., Warriner, A. B., & Kuperman, V. (2014).** *Concreteness ratings for 40 thousand generally known English word lemmas.* Behavior Research Methods, 46(3), 904–911. <a href="https://doi.org/10.3758/s13428-013-0403-5" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

→ Relevant to: possible future rule "abstractness density" (not in v0.1).

## Normative standards

<a id="wcag-2-1"></a>**🌐 W3C (2018).** *Web Content Accessibility Guidelines (WCAG) 2.1.* <a href="https://www.w3.org/TR/WCAG21/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.w3.org — opens in new tab">↗</a>

Key criteria invoked:

- **1.3.1** (Info and Relationships) → [`structure.heading-jump`](./rules/heading-jump.md)
- **1.4.8** (Visual Presentation) — line width ≤ 80 characters → [`structure.line-length-wide`](./rules/line-length-wide.md)
- **2.4.6** (Headings and Labels) → [`structure.heading-jump`](./rules/heading-jump.md)
- **3.1.3** (Unusual Words) → [`lexicon.jargon-undefined`](./rules/jargon-undefined.md)
- **3.1.4** (Abbreviations) → [`lexicon.unexplained-abbreviation`](./rules/unexplained-abbreviation.md)
- **3.1.5** (Reading Level) → [`readability.score`](./rules/readability-score.md)

> ⚠️ Verify exact criterion numbers against the WCAG version you want to cite (2.1 or 2.2).

<a id="can-asc-3-1-2025"></a>**🌐 Accessibility Standards Canada (2025).** *CAN-ASC-3.1:2025 — Plain Language* (first edition). <a href="https://accessible.canada.ca/creating-accessibility-standards/summary-standard-can-asc-312025-plain-language" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="accessible.canada.ca — opens in new tab">↗</a>

First-edition Canadian national standard on plain language, published bilingually by Accessibility Standards Canada under the *Accessible Canada Act*. Prescriptive (`shall` / `should` / `may`) requirements over five areas: audience identification, evaluation methods, structure, wording, design. Grounds many of our `lexicon.*`, `structure.*`, and `readability.score` defaults independently of the US / EU plain-language canons.

→ Relevant to: [`lexicon.jargon-undefined`](./rules/jargon-undefined.md), [`lexicon.unexplained-abbreviation`](./rules/unexplained-abbreviation.md), [`lexicon.weasel-words`](./rules/weasel-words.md), [`structure.sentence-too-long`](./rules/sentence-too-long.md), [`structure.paragraph-too-long`](./rules/paragraph-too-long.md), [`syntax.passive-voice`](./rules/passive-voice.md), [`readability.score`](./rules/readability-score.md).

## European legal context

<a id="eu-eaa-2019-882"></a>**🌐 Directive (EU) 2019/882** of the European Parliament and of the Council of 17 April 2019 — *European Accessibility Act (EAA)*. <a href="https://eur-lex.europa.eu/eli/dir/2019/882/oj" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="eur-lex.europa.eu — opens in new tab">↗</a>

Legal framework extending accessibility requirements to private-sector services from 28 June 2025.

## Practical tools that shaped our design

- <a id="coh-metrix"></a>**🧪 Coh-Metrix** (Graesser & McNamara) — <a href="https://soletlab.asu.edu/coh-metrix/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="soletlab.asu.edu — opens in new tab">↗</a>
- <a id="vale"></a>**🧪 Vale** (Chris Ward) — <a href="https://vale.sh/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="vale.sh — opens in new tab">↗</a>
- <a id="textlint"></a>**🧪 textlint** — <a href="https://textlint.github.io/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="textlint.github.io — opens in new tab">↗</a>
- <a id="hemingway"></a>**🧪 Hemingway Editor** — <a href="https://hemingwayapp.com/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="hemingwayapp.com — opens in new tab">↗</a>
- <a id="proselint"></a>**🧪 Proselint** — <a href="https://github.com/amperser/proselint" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="github.com — opens in new tab">↗</a>

## Rule → reference summary

### Lexicon

| Rule                                                                  | Primary references                                                                                                                                                                                                                                          |
| --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`lexicon.all-caps-shouting`](./rules/all-caps-shouting.md)           | [Arditi & Cho (2007)](#arditi-cho-2007); [Nielsen Norman Group](#nielsen-norman-allcaps); [Bringhurst (2013)](#bringhurst-2013)                                                                                                                             |
| [`lexicon.consonant-cluster`](./rules/consonant-cluster.md)           | [Seidenberg et al. (1984)](#seidenberg-1984); [Treiman et al. (2006)](#treiman-2006) — 🔍 practitioner heuristic                                                                                                                                            |
| [`lexicon.excessive-nominalization`](./rules/excessive-nominalization.md) | [Plain Language US](#plain-language-us-2011); FALC; [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                                                                              |
| [`lexicon.jargon-undefined`](./rules/jargon-undefined.md)             | [WCAG 3.1.3](#wcag-2-1); [Plain Language US](#plain-language-us-2011); FALC; [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                                                          |
| [`lexicon.low-lexical-diversity`](./rules/low-lexical-diversity.md)   | [Herdan (1960)](#herdan-1960); [McCarthy & Jarvis (2010)](#mccarthy-jarvis-2010); [Graesser et al. (2004)](#graesser-2004)                                                                                                                                  |
| [`lexicon.redundant-intensifier`](./rules/redundant-intensifier.md)   | [Strunk & White](#strunk-white-1999); [Quirk et al. (1985)](#quirk-1985); [Zinsser (2006)](#zinsser-2006)                                                                                                                                                   |
| [`lexicon.unexplained-abbreviation`](./rules/unexplained-abbreviation.md) | [WCAG 3.1.4](#wcag-2-1); RGAA 9.4; [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                                                                                                |
| [`lexicon.weasel-words`](./rules/weasel-words.md)                     | [Strunk & White](#strunk-white-1999); Wikipedia style guide; [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                                                                          |

### Readability

| Rule                                              | Primary references                                                                                                                                            |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`readability.score`](./rules/readability-score.md) | [Flesch (1948)](#flesch-1948); [Kincaid et al. (1975)](#kincaid-1975); Henry (1975); Kandel & Moles (1958); [CAN-ASC-3.1:2025](#can-asc-3-1-2025)             |

### Rhythm

| Rule                                                                  | Primary references                                                                |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| [`rhythm.consecutive-long-sentences`](./rules/consecutive-long-sentences.md) | [Sweller (1988)](#sweller-1988); [Sweller et al. (2011)](#sweller-2011)      |
| [`rhythm.repetitive-connectors`](./rules/repetitive-connectors.md)    | [Sanders & Noordman (2000)](#sanders-noordman-2000); [Graesser et al. (2004)](#graesser-2004) |

### Structure

| Rule                                                          | Primary references                                                                                                |
| ------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| [`structure.deep-subordination`](./rules/deep-subordination.md) | [Gibson (1998)](#gibson-1998); FALC                                                                             |
| [`structure.deeply-nested-lists`](./rules/deeply-nested-lists.md) | [WCAG 2.1](#wcag-2-1); cognitive load heuristics                                                              |
| [`structure.excessive-commas`](./rules/excessive-commas.md)   | [Gibson (1998)](#gibson-1998) — 🔍 practitioner heuristic                                                         |
| [`structure.heading-jump`](./rules/heading-jump.md)           | [WCAG 1.3.1 & 2.4.6](#wcag-2-1); RGAA 9.1                                                                         |
| [`structure.line-length-wide`](./rules/line-length-wide.md)   | [WCAG 1.4.8 (AAA)](#wcag-2-1); [Legge & Bigelow (2011)](#legge-bigelow-2011)                                      |
| [`structure.long-enumeration`](./rules/long-enumeration.md)   | FALC; [Plain Language US](#plain-language-us-2011)                                                                |
| [`structure.mixed-numeric-format`](./rules/mixed-numeric-format.md) | [ISO 80000-1](#iso-80000-1-2022); [Chicago Manual of Style](#chicago-manual-17e)                            |
| [`structure.paragraph-too-long`](./rules/paragraph-too-long.md) | [Sweller (1988)](#sweller-1988); [Graesser et al. (2004)](#graesser-2004); [CAN-ASC-3.1:2025](#can-asc-3-1-2025) |
| [`structure.sentence-too-long`](./rules/sentence-too-long.md) | [Sweller (1988)](#sweller-1988); [Plain Language US](#plain-language-us-2011); FALC; [CAN-ASC-3.1:2025](#can-asc-3-1-2025) |

### Syntax

| Rule                                                              | Primary references                                                                                                                                                                          |
| ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`syntax.conditional-stacking`](./rules/conditional-stacking.md)  | [Johnson-Laird & Byrne (1991)](#johnson-laird-byrne-1991); [Evans & Over (2004)](#evans-over-2004); [Gibson (1998)](#gibson-1998) — 🔍 threshold is practitioner heuristic                  |
| [`syntax.dense-punctuation-burst`](./rules/dense-punctuation-burst.md) | [Sweller (1988)](#sweller-1988); [Gibson (1998)](#gibson-1998) — 🔍 purely heuristic                                                                                                    |
| [`syntax.nested-negation`](./rules/nested-negation.md)            | [Clark & Chase (1972)](#clark-chase-1972); [Carpenter & Just (1975)](#carpenter-just-1975); [Kaup et al. (2006)](#kaup-2006)                                                                |
| [`syntax.passive-voice`](./rules/passive-voice.md)                | [Strunk & White](#strunk-white-1999); [Plain Language US](#plain-language-us-2011); FALC; [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                             |
| [`syntax.unclear-antecedent`](./rules/unclear-antecedent.md)      | [Strunk & White](#strunk-white-1999); [Gibson (1998)](#gibson-1998); [Graesser et al. (2004)](#graesser-2004)                                                                               |

## On scholarly honesty

`lucid-lint` is an engineering project informed by research, not a research project itself. The references above ground our design choices but we do not claim to validate new findings. Several rules ([`lexicon.consonant-cluster`](./rules/consonant-cluster.md), [`syntax.conditional-stacking`](./rules/conditional-stacking.md), [`syntax.dense-punctuation-burst`](./rules/dense-punctuation-burst.md), [`structure.excessive-commas`](./rules/excessive-commas.md)) are practitioner heuristics informed by the literature rather than direct transpositions of published metrics — we mark these with 🔍 in the summary table.

Where we simplify an academic metric (e.g., [`syntax.unclear-antecedent`](./rules/unclear-antecedent.md) as a pattern heuristic vs. full anaphora resolution), we document the simplification in [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) and plan richer versions in the [roadmap](./roadmap.md).

If you are a researcher and spot an error, an outdated citation, or a misattribution, please [open an issue](https://github.com/bastien-gallay/lucid-lint/issues) — we will correct it promptly and credit you.
