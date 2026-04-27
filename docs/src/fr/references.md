# Références

> Sources académiques, normatives et pratiques qui fondent la conception de `lucid-lint`.

Cette page liste les références qui ont façonné les règles, profils et décisions de conception de `lucid-lint`. Chaque entrée précise où la référence intervient dans le projet. Le miroir anglais est à [`references.md`](../references.md).

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

**✅ Sweller, J. (1988).** *Cognitive load during problem solving: Effects on learning.* Cognitive Science, 12(2), 257–285.

Papier fondateur. Distingue la charge intrinsèque, extrinsèque et germane.

→ Concerne : la plupart des règles, notamment `structure.*`, `rhythm.*`, `syntax.nested-negation`, `syntax.conditional-stacking`.

**📖 Sweller, J., Ayres, P., & Kalyuga, S. (2011).** *Cognitive Load Theory.* Springer.

## Cohésion textuelle et traitement du discours

**✅ Graesser, A. C., McNamara, D. S., Louwerse, M. M., & Cai, Z. (2004).** *Coh-Metrix: Analysis of text on cohesion and language.* Behavior Research Methods, Instruments, & Computers, 36(2), 193–202.

Papier de référence pour l'analyse automatisée de la cohésion.

→ Concerne : `rhythm.repetitive-connectors`, `syntax.unclear-antecedent`, `lexicon.low-lexical-diversity`.

**📖 McNamara, D. S., Graesser, A. C., McCarthy, P. M., & Cai, Z. (2014).** *Automated evaluation of text and discourse with Coh-Metrix.* Cambridge University Press.

## Complexité syntaxique

**✅ Gibson, E. (1998).** *Linguistic complexity: Locality of syntactic dependencies.* Cognition, 68(1), 1–76.

Papier fondateur de la *Dependency Locality Theory*.

→ Concerne : `structure.deep-subordination`, `syntax.unclear-antecedent`, `syntax.conditional-stacking`.

## Connecteurs du discours

**✅ Sanders, T. J. M., & Noordman, L. G. M. (2000).** *The role of coherence relations and their linguistic markers in text processing.* Discourse Processes, 29(1), 37–60.

→ Concerne : `rhythm.repetitive-connectors`.

## Formules de lisibilité

**✅ Flesch, R. (1948).** *A new readability yardstick.* Journal of Applied Psychology, 32(3), 221–233.

**✅ Kincaid, J. P., Fishburne, R. P., Rogers, R. L., & Chissom, B. S. (1975).** *Derivation of new readability formulas for Navy enlisted personnel.* Technical Report, Naval Technical Training Command.

→ Concerne : `readability.score`.

## Formules francophones

**⚠️ Kandel, L., & Moles, A. (1958).** *Application de l'indice de Flesch à la langue française.* Cahiers Études de Radio-Télévision, 19, 253–274.

> ⚠️ **À vérifier** : pagination et intitulé exact du périodique. À contrôler sur Cairn ou en bibliothèque universitaire.

**✅ Henry, G. (1975).** *Comment mesurer la lisibilité.* Labor, Bruxelles.

Ouvrage de référence francophone proposant la formule de Henry.

→ Concerne : candidat pour v0.2 de `readability.score`.

**✅ François, T., & Fairon, C. (2012).** *An "AI readability" formula for French as a foreign language.* EMNLP-CoNLL 2012.

> ⚠️ **Correction honnête** : « Scolarius », évoqué en session de conception, est un outil commercial québécois et non une formule académique publiée. À ne pas citer comme référence scientifique.

## Diversité lexicale

**📖 Herdan, G. (1960).** *Type-Token Mathematics: A Textbook of Mathematical Linguistics.*

→ Concerne : `lexicon.low-lexical-diversity`.

**✅ McCarthy, P. M., & Jarvis, S. (2010).** *MTLD, vocd-D, and HD-D: A validation study of sophisticated approaches to lexical diversity assessment.* Behavior Research Methods, 42(2), 381–392.

## Traitement de la négation

**✅ Clark, H. H., & Chase, W. G. (1972).** *On the process of comparing sentences against pictures.* Cognitive Psychology, 3(3), 472–517.

Travaux expérimentaux classiques démontrant que les phrases négatives prennent plus de temps à traiter que les affirmatives. Preuve fondamentale que la négation porte un coût de compréhension.

→ Concerne : `syntax.nested-negation`.

**✅ Carpenter, P. A., & Just, M. A. (1975).** *Sentence comprehension: A psycholinguistic processing model of verification.* Psychological Review, 82(1), 45–73.

Prolonge Clark & Chase avec un modèle formel du traitement des phrases. Les négations empilées composent le coût de vérification.

**🔍 Kaup, B., Lüdtke, J., & Zwaan, R. A. (2006).** *Processing negated sentences with contradictory predicates: Is a door that is not open mentally closed?* Journal of Pragmatics, 38(7), 1033–1050.

## Raisonnement conditionnel

**🔍 Johnson-Laird, P. N., & Byrne, R. M. J. (1991).** *Deduction.* Psychology Press.

Théorie des modèles mentaux du raisonnement conditionnel. Les conditionnelles empilées multiplient le nombre de modèles que le lecteur doit maintenir.

→ Concerne : `syntax.conditional-stacking`.

**🔍 Evans, J. St. B. T., & Over, D. E. (2004).** *If.* Oxford University Press.

> 🔍 **Note honnête** : le lien entre conditionnelles enchaînées et charge cognitive du lecteur est intuitif et bien étayé par la littérature globale sur le raisonnement, mais la règle *spécifique* « plus de N conditionnelles par phrase est néfaste » relève d'une heuristique de praticien, non d'un seuil directement testé. Traiter le seuil comme configurable et calibré empiriquement.

## Typographie et traitement visuel

**🔍 Arditi, A., & Cho, J. (2007).** *Letter case and text legibility in normal and low vision.* Vision Research, 47(19), 2499–2505.

Preuves empiriques du coût de lecture du texte en majuscules : le lecteur perd les indices de forme des mots que fournissent les jambages et hampes du mixed-case.

→ Concerne : `lexicon.all-caps-shouting`.

**🧪 Nielsen, J. (Nielsen Norman Group).** Articles multiples sur la lisibilité du texte en majuscules dans les interfaces.

Référence de l'industrie sur la réduction de vitesse de lecture avec ALL-CAPS.

→ Concerne : `lexicon.all-caps-shouting`.

**📖 Bringhurst, R. (2013).** *The Elements of Typographic Style* (4ᵉ éd.). Hartley & Marks.

Référence canonique en typographie.

**✅ Legge, G. E., & Bigelow, C. A. (2011).** *Does print size matter for reading? A review of findings from vision science and typography.* Journal of Vision, 11(5).

Revue des preuves issues des sciences de la vision sur la lecture. Couvre les effets de longueur de ligne.

→ Concerne : `structure.line-length-wide`.

## Complexité phonologique et lecture

**🔍 Seidenberg, M. S., Waters, G. S., Barnes, M. A., & Tanenhaus, M. K. (1984).** *When does irregular spelling or pronunciation influence word recognition?* Journal of Verbal Learning and Verbal Behavior, 23(3), 383–404.

Travail classique montrant que les patterns de lettres inhabituels ralentissent la reconnaissance des mots.

**🔍 Treiman, R., Kessler, B., Zevin, J. D., Bick, S., & Davis, M. (2006).** *Influence of consonantal context on the reading of vowels: Evidence from children.* Journal of Experimental Child Psychology, 93(1), 1–24.

Travaux montrant que les clusters consonantiques et leur contexte affectent précision et vitesse de lecture.

> 🔍 **Précaution honnête** : la règle `lexicon.consonant-cluster` est fondée sur la littérature globale sur la complexité des formes de mots, mais un seuil spécifique validé du type « 4+ consonnes d'affilée est néfaste » ne provient pas d'un papier canonique unique. C'est une heuristique de praticien informée par la littérature, non la transposition directe d'une métrique publiée.

## Intensificateurs et atténuateurs

**🔍 Quirk, R., Greenbaum, S., Leech, G., & Svartvik, J. (1985).** *A Comprehensive Grammar of the English Language.* Longman.

Grammaire classique classant les intensificateurs comme « amplificateurs » dont la contribution sémantique est souvent marginale.

→ Concerne : `lexicon.redundant-intensifier`.

**🧪 Zinsser, W. (2006).** *On Writing Well* (30ᵉ éd. anniversaire). HarperCollins.

Guide pratique qui plaide contre les adverbes intensificateurs comme encombrement.

## Guides de style et langage clair

**📖🧪 Strunk, W., & White, E. B. (1999).** *The Elements of Style* (4ᵉ éd.). Longman.

→ Concerne : `syntax.passive-voice`, `lexicon.weasel-words`, `lexicon.redundant-intensifier`, `syntax.unclear-antecedent`.

**🧪 US Plain Language Action and Information Network (2011).** *Federal Plain Language Guidelines.* <https://www.plainlanguage.gov/>

→ Concerne : `structure.sentence-too-long`, `structure.paragraph-too-long`, `lexicon.excessive-nominalization`, `lexicon.jargon-undefined`, `syntax.passive-voice`.

**🧪 European Commission (2011).** *Rédiger clairement.* Office des publications de l'Union européenne.

## Conventions de formatage numérique

**🌐 International Organization for Standardization (2022).** *ISO 80000-1:2022 — Quantities and units — Part 1: General.*

Standard international sur le formatage des nombres, y compris groupement des chiffres et séparateurs décimaux.

→ Concerne : `structure.mixed-numeric-format`.

**🧪 The Chicago Manual of Style (17ᵉ éd., 2017).** University of Chicago Press.

Guide de style canonique couvrant quand écrire les nombres en lettres ou en chiffres, et pourquoi la cohérence importe.

## Mémoire de travail et attention

**⚠️ Martinussen, R., Hayden, J., Hogg-Johnson, S., & Tannock, R. (2005).** *A meta-analysis of working memory impairments in children with attention-deficit/hyperactivity disorder.* Journal of the American Academy of Child & Adolescent Psychiatry, 44(4), 377–384.

> ⚠️ **Précaution** : la recherche spécifique sur « lisibilité textuelle pour lecteurs TDAH » est dispersée et de qualité variable. L'angle « accessibilité cognitive » est sain, mais traiter les affirmations spécifiques au TDAH avec prudence.

**📖 Barkley, R. A. (2012).** *Executive Functions: What They Are, How They Work, and Why They Evolved.* The Guilford Press.

## Dyslexie et accessibilité visuelle

**✅ Rello, L., & Baeza-Yates, R. (2013).** *Good fonts for dyslexia.* Proceedings of ASSETS '13.

## Standards normatifs internationaux

**🌐 W3C (2018).** *Web Content Accessibility Guidelines (WCAG) 2.1.* <https://www.w3.org/TR/WCAG21/>

Critères clés invoqués :

- **1.3.1** (Information et relations) → `structure.heading-jump`
- **1.4.8** (Présentation visuelle) — largeur de ligne ≤ 80 caractères → `structure.line-length-wide`
- **2.4.6** (En-têtes et étiquettes) → `structure.heading-jump`
- **3.1.3** (Mots inhabituels) → `lexicon.jargon-undefined`
- **3.1.4** (Abréviations) → `lexicon.unexplained-abbreviation`
- **3.1.5** (Niveau de lecture) → `readability.score`

> ⚠️ Vérifie les numéros de critères sur la version WCAG que tu veux citer (2.1 ou 2.2).

## Standards normatifs francophones

**🌐 DINUM (2023).** *Référentiel Général d'Amélioration de l'Accessibilité (RGAA) version 4.1.* <https://accessibilite.numerique.gouv.fr/>

- **Critère 9.1** — structure de l'information → `structure.heading-jump`
- **Critère 9.4** — expansion des abréviations → `lexicon.unexplained-abbreviation`

**🌐 Inclusion Europe (2009, mise à jour 2014).** *Information pour tous : Règles européennes pour une information facile à lire et à comprendre.*

Référentiel FALC (Facile À Lire et à Comprendre).

→ Concerne : le profil `falc` est directement inspiré de ces règles.

**🌐 Normes d'accessibilité Canada (2025).** *CAN-ASC-3.1:2025 — Langage clair* (première édition). <https://accessibilite.canada.ca/elaboration-normes-accessibilite/resume-de-la-norme-can-asc-312025-langage-clair>

Première norme nationale canadienne sur le langage clair, publiée en version bilingue par Normes d'accessibilité Canada dans le cadre de la *Loi canadienne sur l'accessibilité*. Exigences prescriptives (`doit` / `devrait` / `peut`) sur cinq axes : identification du public, méthodes d'évaluation, structure, formulation, conception. Fonde indépendamment plusieurs de nos seuils par défaut côté `lexicon.*`, `structure.*` et `readability.score`.

→ Concerne : `lexicon.jargon-undefined`, `lexicon.unexplained-abbreviation`, `lexicon.weasel-words`, `structure.sentence-too-long`, `structure.paragraph-too-long`, `syntax.passive-voice`, `readability.score`.

## Contexte légal européen

**🌐 Directive (UE) 2019/882** du Parlement européen et du Conseil du 17 avril 2019 — *European Accessibility Act (EAA)*.

Cadre légal étendant les exigences d'accessibilité aux services du secteur privé à partir du 28 juin 2025.

## Outils pratiques qui ont façonné notre design

- **🧪 Coh-Metrix** (Graesser & McNamara) — <https://soletlab.asu.edu/coh-metrix/>
- **🧪 Vale** (Chris Ward) — <https://vale.sh/>
- **🧪 textlint** — <https://textlint.github.io/>
- **🧪 Hemingway Editor** — <https://hemingwayapp.com/>
- **🧪 Proselint** — <https://github.com/amperser/proselint>

## Tableau récapitulatif règle → référence

### Lexique

| Règle                              | Références principales                                                     |
| ---------------------------------- | -------------------------------------------------------------------------- |
| `lexicon.all-caps-shouting`        | Arditi & Cho (2007); Nielsen Norman Group; Bringhurst (2013)               |
| `lexicon.consonant-cluster`        | Seidenberg et al. (1984); Treiman et al. (2006) — 🔍 heuristique praticien |
| `lexicon.excessive-nominalization` | Plain Language US; FALC; CAN-ASC-3.1:2025                                  |
| `lexicon.jargon-undefined`         | WCAG 3.1.3; Plain Language US; FALC; CAN-ASC-3.1:2025                      |
| `lexicon.low-lexical-diversity`    | Herdan (1960); McCarthy & Jarvis (2010); Graesser et al. (2004)            |
| `lexicon.redundant-intensifier`    | Strunk & White; Quirk et al. (1985); Zinsser (2006)                        |
| `lexicon.unexplained-abbreviation` | WCAG 3.1.4; RGAA 9.4; CAN-ASC-3.1:2025                                     |
| `lexicon.weasel-words`             | Strunk & White; Wikipedia style guide; CAN-ASC-3.1:2025                    |

### Lisibilité

| Règle               | Références principales                                                                      |
| ------------------- | ------------------------------------------------------------------------------------------- |
| `readability.score` | Flesch (1948); Kincaid et al. (1975); Henry (1975); Kandel & Moles (1958); CAN-ASC-3.1:2025 |

### Rythme

| Règle                               | Références principales                            |
| ----------------------------------- | ------------------------------------------------- |
| `rhythm.consecutive-long-sentences` | Sweller (1988); Sweller et al. (2011)             |
| `rhythm.repetitive-connectors`      | Sanders & Noordman (2000); Graesser et al. (2004) |

### Structure

| Règle                            | Références principales                                    |
| -------------------------------- | --------------------------------------------------------- |
| `structure.deep-subordination`   | Gibson (1998); FALC                                       |
| `structure.deeply-nested-lists`  | WCAG 2.1; heuristiques de charge cognitive                |
| `structure.excessive-commas`     | Gibson (1998) — 🔍 heuristique praticien                  |
| `structure.heading-jump`         | WCAG 1.3.1 & 2.4.6; RGAA 9.1                              |
| `structure.line-length-wide`     | WCAG 1.4.8 (AAA); Legge & Bigelow (2011)                  |
| `structure.long-enumeration`     | FALC; Plain Language US                                   |
| `structure.mixed-numeric-format` | ISO 80000-1; Chicago Manual of Style                      |
| `structure.paragraph-too-long`   | Sweller (1988); Graesser et al. (2004); CAN-ASC-3.1:2025  |
| `structure.sentence-too-long`    | Sweller (1988); Plain Language US; FALC; CAN-ASC-3.1:2025 |

### Syntaxe

| Règle                            | Références principales                                                                                |
| -------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `syntax.conditional-stacking`    | Johnson-Laird & Byrne (1991); Evans & Over (2004); Gibson (1998) — 🔍 seuil heuristique de praticien |
| `syntax.dense-punctuation-burst` | Sweller (1988); Gibson (1998) — 🔍 purement heuristique                                              |
| `syntax.nested-negation`         | Clark & Chase (1972); Carpenter & Just (1975); Kaup et al. (2006)                                     |
| `syntax.passive-voice`           | Strunk & White; Plain Language US; FALC; CAN-ASC-3.1:2025                                             |
| `syntax.unclear-antecedent`      | Strunk & White; Gibson (1998); Graesser et al. (2004)                                                 |

## Sur l'honnêteté académique

`lucid-lint` est un projet d'ingénierie informé par la recherche, pas un projet de recherche en soi. Les références ci-dessus fondent nos choix de conception mais nous ne prétendons pas valider de nouveaux résultats. Plusieurs règles (`lexicon.consonant-cluster`, `syntax.conditional-stacking`, `syntax.dense-punctuation-burst`, `structure.excessive-commas`) sont des heuristiques de praticien informées par la littérature, et non des transpositions directes de métriques publiées — nous les marquons 🔍 dans le tableau récapitulatif.

Lorsque nous simplifions une métrique académique (par exemple `syntax.unclear-antecedent` comme heuristique de pattern au lieu d'une résolution complète des anaphores), nous documentons la simplification dans [`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md) et planifions des versions plus riches dans la [feuille de route](./roadmap.md).

Si vous êtes chercheur et repérez une erreur, une citation obsolète ou une mauvaise attribution, [ouvrez une issue](https://github.com/bastien-gallay/lucid-lint/issues) — nous corrigerons rapidement et vous créditerons.
