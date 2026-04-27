# Références

> Sources académiques, normatives et pratiques qui fondent la conception de `lucid-lint`.

Cette page liste les références qui ont façonné les règles, profils et décisions de conception de `lucid-lint`. Chaque entrée précise où la référence intervient dans le projet. Le miroir anglais est à [`references.md`](../references.md).

Les liens externes ouvrent un nouvel onglet ; ils portent `rel="nofollow noopener noreferrer"` pour que le nouvel onglet reste sûr et que le site documentaire ne cautionne pas les contenus tiers.

## Légende

| Statut | Signification                                            |
| ------ | -------------------------------------------------------- |
| ✅      | Vérifiée — référence canonique                           |
| ⚠️      | À vérifier — probablement correcte, détails à confirmer  |
| 🔍      | Opportuniste — raisonnement solide, citation plus lâche  |
| 📖      | Livre / source secondaire                                |
| 🌐      | Standard normatif                                        |
| 🧪      | Source pratique (guide de style, outil)                  |

## Théorie de la charge cognitive — la colonne vertébrale

Le socle théorique de `lucid-lint` : un texte impose un coût mental au lecteur, et ce coût peut être mesuré et réduit.

<a id="sweller-1988"></a>**✅ Sweller, J. (1988).** *Cognitive load during problem solving: Effects on learning.* Cognitive Science, 12(2), 257–285. <a href="https://doi.org/10.1207/s15516709cog1202_4" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Papier fondateur. Distingue la charge intrinsèque, extrinsèque et germane.

→ Concerne : la plupart des règles, notamment `structure.*`, `rhythm.*`, [`syntax.nested-negation`](../rules/nested-negation.md), [`syntax.conditional-stacking`](../rules/conditional-stacking.md).

<a id="sweller-2011"></a>**📖 Sweller, J., Ayres, P., & Kalyuga, S. (2011).** *Cognitive Load Theory.* Springer. <a href="https://doi.org/10.1007/978-1-4419-8126-4" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

## Cohésion textuelle et traitement du discours

<a id="graesser-2004"></a>**✅ Graesser, A. C., McNamara, D. S., Louwerse, M. M., & Cai, Z. (2004).** *Coh-Metrix: Analysis of text on cohesion and language.* Behavior Research Methods, Instruments, & Computers, 36(2), 193–202. <a href="https://doi.org/10.3758/BF03195564" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Papier de référence pour l'analyse automatisée de la cohésion.

→ Concerne : [`rhythm.repetitive-connectors`](./rules/repetitive-connectors.md), [`syntax.unclear-antecedent`](../rules/unclear-antecedent.md), [`lexicon.low-lexical-diversity`](../rules/low-lexical-diversity.md).

<a id="mcnamara-2014"></a>**📖 McNamara, D. S., Graesser, A. C., McCarthy, P. M., & Cai, Z. (2014).** *Automated evaluation of text and discourse with Coh-Metrix.* Cambridge University Press. <a href="https://www.cambridge.org/core/books/automated-evaluation-of-text-and-discourse-with-cohmetrix/AE4A1D5DCCBA1AE3A9632E9D4D380270" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.cambridge.org — opens in new tab">↗</a>

## Complexité syntaxique

<a id="gibson-1998"></a>**✅ Gibson, E. (1998).** *Linguistic complexity: Locality of syntactic dependencies.* Cognition, 68(1), 1–76. <a href="https://doi.org/10.1016/S0010-0277(98)00034-1" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Papier fondateur de la *Dependency Locality Theory*.

→ Concerne : [`structure.deep-subordination`](./rules/deep-subordination.md), [`syntax.unclear-antecedent`](../rules/unclear-antecedent.md), [`syntax.conditional-stacking`](../rules/conditional-stacking.md).

## Connecteurs du discours

<a id="sanders-noordman-2000"></a>**✅ Sanders, T. J. M., & Noordman, L. G. M. (2000).** *The role of coherence relations and their linguistic markers in text processing.* Discourse Processes, 29(1), 37–60. <a href="https://doi.org/10.1207/S15326950dp2901_3" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

→ Concerne : [`rhythm.repetitive-connectors`](./rules/repetitive-connectors.md).

## Formules de lisibilité

<a id="flesch-1948"></a>**✅ Flesch, R. (1948).** *A new readability yardstick.* Journal of Applied Psychology, 32(3), 221–233. <a href="https://doi.org/10.1037/h0057532" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

<a id="kincaid-1975"></a>**✅ Kincaid, J. P., Fishburne, R. P., Rogers, R. L., & Chissom, B. S. (1975).** *Derivation of new readability formulas for Navy enlisted personnel.* Technical Report, Naval Technical Training Command. <a href="https://apps.dtic.mil/sti/citations/ADA006655" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="apps.dtic.mil — opens in new tab">↗</a>

→ Concerne : [`readability.score`](../rules/readability-score.md).

## Formules francophones

<a id="kandel-moles-1958"></a>**⚠️ Kandel, L., & Moles, A. (1958).** *Application de l'indice de Flesch à la langue française.* Cahiers Études de Radio-Télévision, 19, 253–274.

> ⚠️ **À vérifier** : pagination et intitulé exact du périodique. À contrôler sur Cairn ou en bibliothèque universitaire.

<a id="henry-1975"></a>**✅ Henry, G. (1975).** *Comment mesurer la lisibilité.* Labor, Bruxelles. <a href="https://www.persee.fr/doc/rfp_0556-7807_1976_num_36_1_2097_t1_0071_0000_2" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.persee.fr — opens in new tab">↗ (compte rendu)</a>

Ouvrage de référence francophone proposant la formule de Henry. Le lien Persée pointe vers le compte rendu de De Landsheere (1976), faute de page éditeur en ligne pour l'ouvrage.

→ Concerne : candidat pour v0.2 de [`readability.score`](../rules/readability-score.md).

<a id="francois-fairon-2012"></a>**✅ François, T., & Fairon, C. (2012).** *An "AI readability" formula for French as a foreign language.* EMNLP-CoNLL 2012. <a href="https://aclanthology.org/D12-1043/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="aclanthology.org — opens in new tab">↗</a>

> ⚠️ **Rectification** : « Scolarius », évoqué en session de conception, est un outil commercial québécois et non une formule académique publiée. À ne pas citer comme référence scientifique.

## Diversité lexicale

<a id="herdan-1960"></a>**📖 Herdan, G. (1960).** *Type-Token Mathematics: A Textbook of Mathematical Linguistics.*

→ Concerne : [`lexicon.low-lexical-diversity`](../rules/low-lexical-diversity.md).

<a id="mccarthy-jarvis-2010"></a>**✅ McCarthy, P. M., & Jarvis, S. (2010).** *MTLD, vocd-D, and HD-D: A validation study of sophisticated approaches to lexical diversity assessment.* Behavior Research Methods, 42(2), 381–392. <a href="https://doi.org/10.3758/BRM.42.2.381" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

## Traitement de la négation

<a id="clark-chase-1972"></a>**✅ Clark, H. H., & Chase, W. G. (1972).** *On the process of comparing sentences against pictures.* Cognitive Psychology, 3(3), 472–517. <a href="https://doi.org/10.1016/0010-0285(72)90019-9" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Travaux expérimentaux classiques démontrant que les phrases négatives prennent plus de temps à traiter que les affirmatives. Preuve fondamentale que la négation porte un coût de compréhension.

→ Concerne : [`syntax.nested-negation`](../rules/nested-negation.md).

<a id="carpenter-just-1975"></a>**✅ Carpenter, P. A., & Just, M. A. (1975).** *Sentence comprehension: A psycholinguistic processing model of verification.* Psychological Review, 82(1), 45–73. <a href="https://doi.org/10.1037/h0076248" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Prolonge Clark & Chase avec un modèle formel du traitement des phrases. Les négations empilées composent le coût de vérification.

<a id="kaup-2006"></a>**🔍 Kaup, B., Lüdtke, J., & Zwaan, R. A. (2006).** *Processing negated sentences with contradictory predicates: Is a door that is not open mentally closed?* Journal of Pragmatics, 38(7), 1033–1050. <a href="https://doi.org/10.1016/j.pragma.2005.09.012" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

## Raisonnement conditionnel

<a id="johnson-laird-byrne-1991"></a>**🔍 Johnson-Laird, P. N., & Byrne, R. M. J. (1991).** *Deduction.* Psychology Press. <a href="https://psycnet.apa.org/record/1991-97828-000" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="psycnet.apa.org — opens in new tab">↗</a>

Théorie des modèles mentaux du raisonnement conditionnel. Les conditionnelles empilées multiplient le nombre de modèles que le lecteur doit maintenir.

→ Concerne : [`syntax.conditional-stacking`](../rules/conditional-stacking.md).

<a id="evans-over-2004"></a>**🔍 Evans, J. St. B. T., & Over, D. E. (2004).** *If.* Oxford University Press. <a href="https://global.oup.com/academic/product/if-9780198525134" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="global.oup.com — opens in new tab">↗</a>

> 🔍 **Précaution** : le lien entre conditionnelles enchaînées et charge cognitive du lecteur est intuitif et bien étayé par la littérature globale sur le raisonnement, mais la règle *spécifique* « plus de N conditionnelles par phrase est néfaste » relève d'une heuristique de praticien, non d'un seuil directement testé. Traiter le seuil comme configurable et calibré empiriquement.

## Typographie et traitement visuel

<a id="arditi-cho-2007"></a>**🔍 Arditi, A., & Cho, J. (2007).** *Letter case and text legibility in normal and low vision.* Vision Research, 47(19), 2499–2505. <a href="https://doi.org/10.1016/j.visres.2007.06.010" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Preuves empiriques du coût de lecture du texte en majuscules : le lecteur perd les indices de forme des mots que fournissent les jambages et hampes du mixed-case.

→ Concerne : [`lexicon.all-caps-shouting`](../rules/all-caps-shouting.md).

<a id="nielsen-norman-allcaps"></a>**🧪 Nielsen, J. (Nielsen Norman Group).** Articles multiples sur la lisibilité du texte en majuscules dans les interfaces.

→ Concerne : [`lexicon.all-caps-shouting`](../rules/all-caps-shouting.md).

<a id="bringhurst-2013"></a>**📖 Bringhurst, R. (2013).** *The Elements of Typographic Style* (4ᵉ éd.). Hartley & Marks.

Référence canonique en typographie.

<a id="legge-bigelow-2011"></a>**✅ Legge, G. E., & Bigelow, C. A. (2011).** *Does print size matter for reading? A review of findings from vision science and typography.* Journal of Vision, 11(5). <a href="https://doi.org/10.1167/11.5.8" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Revue des preuves issues des sciences de la vision sur la lecture. Couvre les effets de longueur de ligne.

→ Concerne : [`structure.line-length-wide`](./rules/line-length-wide.md).

## Complexité phonologique et lecture

<a id="seidenberg-1984"></a>**🔍 Seidenberg, M. S., Waters, G. S., Barnes, M. A., & Tanenhaus, M. K. (1984).** *When does irregular spelling or pronunciation influence word recognition?* Journal of Verbal Learning and Verbal Behavior, 23(3), 383–404. <a href="https://doi.org/10.1016/S0022-5371(84)90270-6" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Travail classique montrant que les patterns de lettres inhabituels ralentissent la reconnaissance des mots.

<a id="treiman-2006"></a>**🔍 Treiman, R., Kessler, B., Zevin, J. D., Bick, S., & Davis, M. (2006).** *Influence of consonantal context on the reading of vowels: Evidence from children.* Journal of Experimental Child Psychology, 93(1), 1–24. <a href="https://doi.org/10.1016/j.jecp.2005.06.008" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

Travaux montrant que les clusters consonantiques et leur contexte affectent précision et vitesse de lecture.

> 🔍 **Précaution** : la règle [`lexicon.consonant-cluster`](../rules/consonant-cluster.md) est fondée sur la littérature globale sur la complexité des formes de mots, mais un seuil spécifique validé du type « 4+ consonnes d'affilée est néfaste » ne provient pas d'un papier canonique unique. C'est une heuristique de praticien informée par la littérature, non la transposition directe d'une métrique publiée.

## Intensificateurs et atténuateurs

<a id="quirk-1985"></a>**🔍 Quirk, R., Greenbaum, S., Leech, G., & Svartvik, J. (1985).** *A Comprehensive Grammar of the English Language.* Longman.

Grammaire classique classant les intensificateurs comme « amplificateurs » dont la contribution sémantique est souvent marginale.

→ Concerne : [`lexicon.redundant-intensifier`](../rules/redundant-intensifier.md).

<a id="zinsser-2006"></a>**🧪 Zinsser, W. (2006).** *On Writing Well* (30ᵉ éd. anniversaire). HarperCollins.

Guide pratique qui plaide contre les adverbes intensificateurs comme encombrement.

## Guides de style et langage clair

<a id="strunk-white-1999"></a>**📖🧪 Strunk, W., & White, E. B. (1999).** *The Elements of Style* (4ᵉ éd.). Longman.

→ Concerne : [`syntax.passive-voice`](../rules/passive-voice.md), [`lexicon.weasel-words`](./rules/weasel-words.md), [`lexicon.redundant-intensifier`](../rules/redundant-intensifier.md), [`syntax.unclear-antecedent`](../rules/unclear-antecedent.md).

<a id="plain-language-us-2011"></a>**🧪 US Plain Language Action and Information Network (2011).** *Federal Plain Language Guidelines.* <a href="https://www.plainlanguage.gov/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.plainlanguage.gov — opens in new tab">↗</a>

→ Concerne : [`structure.sentence-too-long`](./rules/sentence-too-long.md), [`structure.paragraph-too-long`](./rules/paragraph-too-long.md), [`lexicon.excessive-nominalization`](../rules/excessive-nominalization.md), [`lexicon.jargon-undefined`](../rules/jargon-undefined.md), [`syntax.passive-voice`](../rules/passive-voice.md).

<a id="ec-write-clearly-2011"></a>**🧪 European Commission (2011).** *Rédiger clairement.* Office des publications de l'Union européenne. <a href="https://op.europa.eu/en/publication-detail/-/publication/725b7eb0-d92e-11e5-8fea-01aa75ed71a1" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="op.europa.eu — opens in new tab">↗</a>

## Conventions de formatage numérique

<a id="iso-80000-1-2022"></a>**🌐 International Organization for Standardization (2022).** *ISO 80000-1:2022 — Quantities and units — Part 1: General.* <a href="https://www.iso.org/standard/76921.html" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.iso.org — opens in new tab">↗</a>

Standard international sur le formatage des nombres, y compris groupement des chiffres et séparateurs décimaux.

→ Concerne : [`structure.mixed-numeric-format`](./rules/mixed-numeric-format.md).

<a id="chicago-manual-17e"></a>**🧪 The Chicago Manual of Style (17ᵉ éd., 2017).** University of Chicago Press. <a href="https://press.uchicago.edu/sites/cmos17/index.html" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="press.uchicago.edu — opens in new tab">↗</a>

Guide de style canonique couvrant quand écrire les nombres en lettres ou en chiffres, et pourquoi la cohérence importe.

## Mémoire de travail et attention

<a id="martinussen-2005"></a>**⚠️ Martinussen, R., Hayden, J., Hogg-Johnson, S., & Tannock, R. (2005).** *A meta-analysis of working memory impairments in children with attention-deficit/hyperactivity disorder.* Journal of the American Academy of Child & Adolescent Psychiatry, 44(4), 377–384. <a href="https://doi.org/10.1097/01.chi.0000153228.72591.73" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

> ⚠️ **Précaution** : la recherche spécifique sur « lisibilité textuelle pour lecteurs TDAH » est dispersée et de qualité variable. L'angle « accessibilité cognitive » est sain, mais traiter les affirmations spécifiques au TDAH avec prudence.

<a id="barkley-2012"></a>**📖 Barkley, R. A. (2012).** *Executive Functions: What They Are, How They Work, and Why They Evolved.* The Guilford Press. <a href="https://www.guilford.com/books/Executive-Functions/Russell-Barkley/9781462545933" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.guilford.com — opens in new tab">↗</a>

## Dyslexie et accessibilité visuelle

<a id="rello-baeza-yates-2013"></a>**✅ Rello, L., & Baeza-Yates, R. (2013).** *Good fonts for dyslexia.* Proceedings of ASSETS '13. <a href="https://doi.org/10.1145/2513383.2513447" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="doi.org — opens in new tab">↗</a>

## Standards normatifs internationaux

<a id="wcag-2-1"></a>**🌐 W3C (2018).** *Web Content Accessibility Guidelines (WCAG) 2.1.* <a href="https://www.w3.org/TR/WCAG21/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="www.w3.org — opens in new tab">↗</a>

Critères clés invoqués :

- **1.3.1** (Information et relations) → [`structure.heading-jump`](./rules/heading-jump.md)
- **1.4.8** (Présentation visuelle) — largeur de ligne ≤ 80 caractères → [`structure.line-length-wide`](./rules/line-length-wide.md)
- **2.4.6** (En-têtes et étiquettes) → [`structure.heading-jump`](./rules/heading-jump.md)
- **3.1.3** (Mots inhabituels) → [`lexicon.jargon-undefined`](../rules/jargon-undefined.md)
- **3.1.4** (Abréviations) → [`lexicon.unexplained-abbreviation`](./rules/unexplained-abbreviation.md)
- **3.1.5** (Niveau de lecture) → [`readability.score`](../rules/readability-score.md)

> ⚠️ Vérifie les numéros de critères sur la version WCAG que tu veux citer (2.1 ou 2.2).

## Standards normatifs francophones

<a id="rgaa-4-1"></a>**🌐 DINUM (2023).** *Référentiel Général d'Amélioration de l'Accessibilité (RGAA) version 4.1.* <a href="https://accessibilite.numerique.gouv.fr/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="accessibilite.numerique.gouv.fr — opens in new tab">↗</a>

- **Critère 9.1** — structure de l'information → [`structure.heading-jump`](./rules/heading-jump.md)
- **Critère 9.4** — expansion des abréviations → [`lexicon.unexplained-abbreviation`](./rules/unexplained-abbreviation.md)

<a id="inclusion-europe-falc"></a>**🌐 Inclusion Europe (2009, mise à jour 2014).** *Information pour tous : Règles européennes pour une information facile à lire et à comprendre.*

Référentiel FALC (Facile À Lire et à Comprendre).

→ Concerne : le profil `falc` est directement inspiré de ces règles.

<a id="can-asc-3-1-2025"></a>**🌐 Normes d'accessibilité Canada (2025).** *CAN-ASC-3.1:2025 — Langage clair* (première édition). <a href="https://accessibilite.canada.ca/elaboration-normes-accessibilite/resume-de-la-norme-can-asc-312025-langage-clair" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="accessibilite.canada.ca — opens in new tab">↗</a>

Première norme nationale canadienne sur le langage clair, publiée en version bilingue par Normes d'accessibilité Canada dans le cadre de la *Loi canadienne sur l'accessibilité*. Exigences prescriptives (`doit` / `devrait` / `peut`) sur cinq axes : identification du public, méthodes d'évaluation, structure, formulation, conception. Fonde indépendamment plusieurs de nos seuils par défaut côté `lexicon.*`, `structure.*` et `readability.score`.

→ Concerne : [`lexicon.jargon-undefined`](../rules/jargon-undefined.md), [`lexicon.unexplained-abbreviation`](./rules/unexplained-abbreviation.md), [`lexicon.weasel-words`](./rules/weasel-words.md), [`structure.sentence-too-long`](./rules/sentence-too-long.md), [`structure.paragraph-too-long`](./rules/paragraph-too-long.md), [`syntax.passive-voice`](../rules/passive-voice.md), [`readability.score`](../rules/readability-score.md).

## Contexte légal européen

<a id="eu-eaa-2019-882"></a>**🌐 Directive (UE) 2019/882** du Parlement européen et du Conseil du 17 avril 2019 — *European Accessibility Act (EAA)*. <a href="https://eur-lex.europa.eu/eli/dir/2019/882/oj" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="eur-lex.europa.eu — opens in new tab">↗</a>

Cadre légal étendant les exigences d'accessibilité aux services du secteur privé à partir du 28 juin 2025.

## Outils pratiques qui ont façonné notre design

- <a id="coh-metrix"></a>**🧪 Coh-Metrix** (Graesser & McNamara) — <a href="https://soletlab.asu.edu/coh-metrix/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="soletlab.asu.edu — opens in new tab">↗</a>
- <a id="vale"></a>**🧪 Vale** (Chris Ward) — <a href="https://vale.sh/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="vale.sh — opens in new tab">↗</a>
- <a id="textlint"></a>**🧪 textlint** — <a href="https://textlint.github.io/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="textlint.github.io — opens in new tab">↗</a>
- <a id="hemingway"></a>**🧪 Hemingway Editor** — <a href="https://hemingwayapp.com/" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="hemingwayapp.com — opens in new tab">↗</a>
- <a id="proselint"></a>**🧪 Proselint** — <a href="https://github.com/amperser/proselint" class="ref-link" rel="nofollow noopener noreferrer" target="_blank" aria-label="github.com — opens in new tab">↗</a>

## Tableau récapitulatif règle → référence

### Lexique

| Règle                                                                  | Références principales                                                                                                                                                                                                                              |
| ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`lexicon.all-caps-shouting`](../rules/all-caps-shouting.md)           | [Arditi & Cho (2007)](#arditi-cho-2007); [Nielsen Norman Group](#nielsen-norman-allcaps); [Bringhurst (2013)](#bringhurst-2013)                                                                                                                     |
| [`lexicon.consonant-cluster`](../rules/consonant-cluster.md)           | [Seidenberg et al. (1984)](#seidenberg-1984); [Treiman et al. (2006)](#treiman-2006) — 🔍 heuristique praticien                                                                                                                                     |
| [`lexicon.excessive-nominalization`](../rules/excessive-nominalization.md) | [Plain Language US](#plain-language-us-2011); [FALC](#inclusion-europe-falc); [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                                            |
| [`lexicon.jargon-undefined`](../rules/jargon-undefined.md)             | [WCAG 3.1.3](#wcag-2-1); [Plain Language US](#plain-language-us-2011); [FALC](#inclusion-europe-falc); [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                        |
| [`lexicon.low-lexical-diversity`](../rules/low-lexical-diversity.md)   | [Herdan (1960)](#herdan-1960); [McCarthy & Jarvis (2010)](#mccarthy-jarvis-2010); [Graesser et al. (2004)](#graesser-2004)                                                                                                                          |
| [`lexicon.redundant-intensifier`](../rules/redundant-intensifier.md)   | [Strunk & White](#strunk-white-1999); [Quirk et al. (1985)](#quirk-1985); [Zinsser (2006)](#zinsser-2006)                                                                                                                                           |
| [`lexicon.unexplained-abbreviation`](./rules/unexplained-abbreviation.md) | [WCAG 3.1.4](#wcag-2-1); [RGAA 9.4](#rgaa-4-1); [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                                                                          |
| [`lexicon.weasel-words`](./rules/weasel-words.md)                      | [Strunk & White](#strunk-white-1999); Wikipedia style guide; [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                                                                                                                                  |

### Lisibilité

| Règle                                              | Références principales                                                                                                                                            |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`readability.score`](../rules/readability-score.md) | [Flesch (1948)](#flesch-1948); [Kincaid et al. (1975)](#kincaid-1975); [Henry (1975)](#henry-1975); [Kandel & Moles (1958)](#kandel-moles-1958); [CAN-ASC-3.1:2025](#can-asc-3-1-2025) |

### Rythme

| Règle                                                                  | Références principales                                                                |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| [`rhythm.consecutive-long-sentences`](./rules/consecutive-long-sentences.md) | [Sweller (1988)](#sweller-1988); [Sweller et al. (2011)](#sweller-2011)         |
| [`rhythm.repetitive-connectors`](./rules/repetitive-connectors.md)     | [Sanders & Noordman (2000)](#sanders-noordman-2000); [Graesser et al. (2004)](#graesser-2004) |

### Structure

| Règle                                                          | Références principales                                                                                              |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| [`structure.deep-subordination`](./rules/deep-subordination.md) | [Gibson (1998)](#gibson-1998); [FALC](#inclusion-europe-falc)                                                       |
| [`structure.deeply-nested-lists`](./rules/deeply-nested-lists.md) | [WCAG 2.1](#wcag-2-1); heuristiques de charge cognitive                                                           |
| [`structure.excessive-commas`](./rules/excessive-commas.md)    | [Gibson (1998)](#gibson-1998) — 🔍 heuristique praticien                                                            |
| [`structure.heading-jump`](./rules/heading-jump.md)            | [WCAG 1.3.1 & 2.4.6](#wcag-2-1); [RGAA 9.1](#rgaa-4-1)                                                              |
| [`structure.line-length-wide`](./rules/line-length-wide.md)    | [WCAG 1.4.8 (AAA)](#wcag-2-1); [Legge & Bigelow (2011)](#legge-bigelow-2011)                                        |
| [`structure.long-enumeration`](./rules/long-enumeration.md)    | [FALC](#inclusion-europe-falc); [Plain Language US](#plain-language-us-2011)                                        |
| [`structure.mixed-numeric-format`](./rules/mixed-numeric-format.md) | [ISO 80000-1](#iso-80000-1-2022); [Chicago Manual of Style](#chicago-manual-17e)                                |
| [`structure.paragraph-too-long`](./rules/paragraph-too-long.md) | [Sweller (1988)](#sweller-1988); [Graesser et al. (2004)](#graesser-2004); [CAN-ASC-3.1:2025](#can-asc-3-1-2025)    |
| [`structure.sentence-too-long`](./rules/sentence-too-long.md)  | [Sweller (1988)](#sweller-1988); [Plain Language US](#plain-language-us-2011); [FALC](#inclusion-europe-falc); [CAN-ASC-3.1:2025](#can-asc-3-1-2025) |

### Syntaxe

| Règle                                                              | Références principales                                                                                                                                                                       |
| ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`syntax.conditional-stacking`](../rules/conditional-stacking.md)  | [Johnson-Laird & Byrne (1991)](#johnson-laird-byrne-1991); [Evans & Over (2004)](#evans-over-2004); [Gibson (1998)](#gibson-1998) — 🔍 seuil heuristique de praticien                        |
| [`syntax.dense-punctuation-burst`](../rules/dense-punctuation-burst.md) | [Sweller (1988)](#sweller-1988); [Gibson (1998)](#gibson-1998) — 🔍 purement heuristique                                                                                                 |
| [`syntax.nested-negation`](../rules/nested-negation.md)            | [Clark & Chase (1972)](#clark-chase-1972); [Carpenter & Just (1975)](#carpenter-just-1975); [Kaup et al. (2006)](#kaup-2006)                                                                  |
| [`syntax.passive-voice`](../rules/passive-voice.md)                | [Strunk & White](#strunk-white-1999); [Plain Language US](#plain-language-us-2011); [FALC](#inclusion-europe-falc); [CAN-ASC-3.1:2025](#can-asc-3-1-2025)                                    |
| [`syntax.unclear-antecedent`](../rules/unclear-antecedent.md)      | [Strunk & White](#strunk-white-1999); [Gibson (1998)](#gibson-1998); [Graesser et al. (2004)](#graesser-2004)                                                                                |

## Sur l'honnêteté académique

`lucid-lint` est un projet d'ingénierie informé par la recherche, pas un projet de recherche en soi. Les références ci-dessus fondent nos choix de conception mais nous ne prétendons pas valider de nouveaux résultats. Plusieurs règles ([`lexicon.consonant-cluster`](../rules/consonant-cluster.md), [`syntax.conditional-stacking`](../rules/conditional-stacking.md), [`syntax.dense-punctuation-burst`](../rules/dense-punctuation-burst.md), [`structure.excessive-commas`](./rules/excessive-commas.md)) sont des heuristiques de praticien informées par la littérature, et non des transpositions directes de métriques publiées — nous les marquons 🔍 dans le tableau récapitulatif.

Lorsque nous simplifions une métrique académique (par exemple [`syntax.unclear-antecedent`](../rules/unclear-antecedent.md) comme heuristique de pattern au lieu d'une résolution complète des anaphores), nous documentons la simplification dans [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) et planifions des versions plus riches dans la [feuille de route](./roadmap.md).

Si vous êtes chercheur et repérez une erreur, une citation obsolète ou une mauvaise attribution, [ouvrez une issue](https://github.com/bastien-gallay/lucid-lint/issues) — nous corrigerons rapidement et vous créditerons.
