---
source_url: https://accessibilite.numerique.gouv.fr/methode/criteres-et-tests/
title: RGAA 4.1 — critères et tests
upstream_type: standard
polarity: good_example
languages:
- fr
redistribution: public_ok
license: Licence Ouverte / Open Licence 2.0 (Etalab) — reuse with attribution.
rules_relevant:
- structure.heading-jump
- lexicon.unexplained-abbreviation
- lexicon.jargon-undefined
conditions:
- a11y-markup
- dyslexia
- general
fetched_at: '2026-04-22T21:39:10+00:00'
markdownable: 4
---

La version 5 du RGAA est en cours de rédaction, avec une publication prévue fin 2026. Cette échéance ne remet pas en cause la pertinence des travaux de mise en accessibilité en cours ou à venir : ils ne doivent en aucun cas être suspendus ou reportés. Plus de détails sur le RGAA5.

# Critères et tests

## 1. Images Thématique Images

- ### 1.1 Chaque image porteuse d’information a-t-elle une alternative textuelle ? Critère 1.1

  #### 1.1.1

  Chaque image (balise

      <img>

  ou balise possédant l’attribut WAI-ARIA

      role="img"

  ) porteuse d’information a-t-elle une alternative textuelle ? Test 1.1.1

  - Retrouver dans le document les images structurées au moyen d’un élément

        <img>

    ou d’un élément possédant l’attribut WAI-ARIA

        role="img"

    ;

  - Pour chaque image, déterminer si l’image est porteuse d’information ;

  - Dans le cas où il s’agit d’un élément

        <img>

    , vérifier que l’image est pourvue au moins d’une alternative textuelle parmi les suivantes :

    - Passage de texte associé via l’attribut WAI-ARIA

          aria-labelledby

      ;

    - Contenu de l’attribut WAI-ARIA

          aria-label

      ;

    - Contenu de l’attribut

          alt

      ;

    - Contenu de l’attribut

          title

      .

  - Passage de texte associé via l’attribut WAI-ARIA

  - Dans le cas où il s’agit d’un élément possédant l’attribut WAI-ARIA

        role="img"

    , vérifier que l’image est pourvue au moins d’une alternative textuelle parmi les suivantes :

    - Passage de texte associé via l’attribut WAI-ARIA

          aria-labelledby

      ;

    - Contenu de l’attribut WAI-ARIA

          aria-label

      .

  - Passage de texte associé via l’attribut WAI-ARIA

  - Si au moins une alternative textuelle est trouvée, le test est validé.

  #### 1.1.2

  Chaque zone d’une image réactive (balise

      <area>

  ) porteuse d’information a-t-elle une alternative textuelle ? Test 1.1.2

  - Retrouver dans le document les éléments

        <area>

    ;

  - Pour chaque élément

        <area>

    , déterminer si la zone réactive est porteuse d’information ;

  - Vérifier que la zone réactive est pourvue au moins d’une alternative textuelle parmi les suivantes :
    - Contenu de l’attribut WAI-ARIA

          aria-label

      ;

    - Contenu de l’attribut

          alt

      ;

  - Contenu de l’attribut WAI-ARIA

  - Si au moins une alternative textuelle est trouvée, le test est validé.

  #### 1.1.3

  Chaque bouton de type

      image

  (balise

      <input>

  avec l’attribut

      type="image"

  ) a-t-il une alternative textuelle ? Test 1.1.3

  - Retrouver dans le document les éléments

        <input>

    pourvus de l’attribut

        type="image"

    ;

  - Pour chaque élément

        <input>

    pourvu de l’attribut type="image", déterminer si l’image utilisée est porteuse d’information ;

  - Vérifier que l’élément

        <input>

    est pourvu au moins d’une alternative textuelle parmi les suivantes :

    - Passage de texte associé via l’attribut WAI-ARIA

          aria-labelledby

      ;

    - Contenu de l’attribut WAI-ARIA

          aria-label

      ;

    - Contenu de l’attribut

          alt

      ;

    - Contenu de l’attribut

          title

      .

  - Passage de texte associé via l’attribut WAI-ARIA

  - Si au moins une alternative textuelle est trouvée, le test est validé.

  #### 1.1.4

  Chaque zone cliquable d’une image réactive côté serveur est-elle doublée d’un mécanisme utilisable quel que soit le dispositif de pointage utilisé et permettant d’accéder à la même destination ? Test 1.1.4

  - Retrouver dans le document les éléments

        <img>

    pourvus de l’attribut

        ismap

    ;

  - Pour chaque élément

        <img>

    pourvu de l’attribut

        ismap

    , vérifier la présence d’un lien ou d’un ensemble de liens (ou bien d’un autre type de composant d’interface qui jouerait un rôle similaire comme une liste de sélection, par exemple) permettant d’accéder aux mêmes ressources que lorsque l’image fait l’objet d’un clic.

  - Si c’est le cas, le test est validé.

  #### 1.1.5

  Chaque image vectorielle (balise

      <svg>

  ) porteuse d’information, vérifie-t-elle ces conditions ? Test 1.1.5

  - La balise

        <svg>

    possède un attribut WAI-ARIA

        role="img"

    ;

  - La balise

        <svg>

    a une alternative textuelle.

  <!-- -->

  - Retrouver dans le document les éléments

        <svg>

    ;

  - Pour chaque élément

        <svg>

    , déterminer si l’image est porteuse d’information ;

  - S’assurer que l’élément

        <svg>

    est pourvu d’un attribut WAI-ARIA

        role="img"

    ;

  - Si ce n’est pas le cas, le test est invalidé.

  - Le cas échéant, vérifier que l’élément

        <svg>

    est pourvu au moins d’une alternative textuelle parmi les suivantes :

    - Contenu de l’élément

          <title>

      ;

    - Passage de texte associé via l’attribut WAI-ARIA

          aria-labelledby

      ;

    - Contenu de l’attribut WAI-ARIA

          aria-label

      ;

  - Contenu de l’élément

  - Si au moins une alternative textuelle est trouvée, le test est validé.

  #### 1.1.6

  Chaque image objet (balise

      <object>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, vérifie-t-elle une de ces conditions ? Test 1.1.6

  - La balise

        <object>

    possède une alternative textuelle et un attribut

        role="img"

    ;

  - L’élément

        <object>

    est immédiatement suivi d’un lien ou bouton adjacent permettant d’accéder à un contenu alternatif ;

  - Un mécanisme permet à l’utilisateur de remplacer l’élément

        <object>

    par un contenu alternatif.

  <!-- -->

  - Retrouver dans le document les balises ouvrantes

        <object>

    pourvues de l’attribut

        type=“image/…”

    ;

  - Pour chaque balise ouvrante

        <object>

    pourvue de l’attribut

        type=“image/…”

    , déterminer si l’image utilisée est porteuse d’information ;

  - Vérifier que l’élément

        <object>

    est pourvu d’un attribut WAI-ARIA

        role=“img”

    ;

  - Vérifier que l’élément

        <object>

    est pourvu au moins d’une alternative textuelle parmi les suivantes :

    - Passage de texte associé via l’attribut WAI-ARIA

          aria-labelledby

      ;

    - Contenu de l’attribut WAI-ARIA

          aria-label

      ;

    - Contenu de l’attribut

          title

      .

  - Passage de texte associé via l’attribut WAI-ARIA

  - Si au moins une alternative textuelle est trouvée, le test est validé ;

  - Sinon, vérifier que l’élément

        <object>

    est :

    - Soit immédiatement suivi d’un lien ou bouton adjacent permettant d’accéder à un contenu alternatif ;

    - Soit un mécanisme permet à l’utilisateur de remplacer l’élément

          <object>

      par un contenu alternatif.

  - Si c’est le cas, le test est validé.

  #### 1.1.7

  Chaque image embarquée (balise

      <embed>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, vérifie-t-elle une de ces conditions ? Test 1.1.7

  - La balise

        <embed>

    possède une alternative textuelle et un attribut

        role="img"

    ;

  - L’élément

        <embed>

    est immédiatement suivi d’un lien ou bouton adjacent permettant d’accéder à un contenu alternatif ;

  - Un mécanisme permet à l’utilisateur de remplacer l’élément

        <embed>

    par un contenu alternatif.

  <!-- -->

  - Pour chaque élément

        <embed>

    pourvu de l’attribut

        type="image/…"

    , déterminer si l’image utilisée est porteuse d’information ;

  - Vérifier que l’élément

        <embed>

    est pourvu d’un attribut WAI-ARIA

        role="img"

    ;

  - Vérifier que l’élément

        <embed>

    est pourvu au moins d’une alternative textuelle parmi les suivantes :

    - Passage de texte associé via l’attribut WAI-ARIA

          aria-labelledby

      ;

    - Contenu de l’attribut WAI-ARIA

          aria-label

      ;

    - Contenu de l’attribut

          title

      .

  - Passage de texte associé via l’attribut WAI-ARIA

  - Si au moins une alternative textuelle est trouvée, le test est validé ;

  - Sinon, vérifier que l’élément

        <embed>

    est :

    - Soit immédiatement suivi d’un lien ou bouton adjacent permettant d’accéder à un contenu alternatif ;

    - Soit un mécanisme permet à l’utilisateur de remplacer l’élément

          <embed>

      par un contenu alternatif.

  - Si c’est le cas, le test est validé.

  #### 1.1.8

  Chaque image bitmap (balise

      <canvas>

  ) porteuse d’information, vérifie-t-elle une de ces conditions ? Test 1.1.8

  - La balise

        <canvas>

    possède une alternative textuelle et un attribut

        role="img"

    ;

  - Un contenu alternatif est présent entre les balises

        <canvas>

    et

        </canvas>

    ;

  - L’élément

        <canvas>

    est immédiatement suivi d’un lien ou bouton adjacent permettant d’accéder à un contenu alternatif ;

  - Un mécanisme permet à l’utilisateur de remplacer l’élément

        <canvas>

    par un contenu alternatif.

  <!-- -->

  - Retrouver dans le document les éléments

        <canvas>

    ;

  - Pour chaque élément

        <canvas>

    , déterminer si l’image utilisée est porteuse d’information ;

  - Vérifier que l’élément

        <canvas>

    est pourvu d’un attribut WAI-ARIA

        role=“img”

    ;

  - Vérifier que la balise ouvrante

        <canvas>

    est pourvue au moins d’une alternative textuelle parmi les suivantes :

    - Passage de texte associé via l’attribut WAI-ARIA

          aria-labelledby

      ;

    - Contenu de l’attribut WAI-ARIA

          aria-label

      .

  - Passage de texte associé via l’attribut WAI-ARIA

  - Si au moins une alternative textuelle est trouvée, le test est validé.

  - Si les étapes 3 et 4 ne sont pas satisfaites, vérifier que l’élément

        <canvas>

    est :

    - Soit pourvu d’un contenu alternatif présent entre les balises

          <canvas>

      et

          </canvas>

      ;

    - Soit immédiatement suivi d’un lien ou bouton adjacent permettant d’accéder à un contenu alternatif ;

    - Soit un mécanisme permet à l’utilisateur de remplacer l’élément

          <canvas>

      par un contenu alternatif.

  - Soit pourvu d’un contenu alternatif présent entre les balises

  - Si c’est le cas, le test est validé.

  Note : si l’élément

      <canvas>

  dispose d’un rôle

      img

  , son alternative ne peut être fournie que par les techniques listées à l’étape 4.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- Retrouver dans le document les images structurées au moyen d’un élément

- ### 1.2 Chaque image de décoration est-elle correctement ignorée par les technologies d’assistance ? Critère 1.2

  #### 1.2.1

  Chaque image (balise

      <img>

  ) de décoration, sans légende, vérifie-t-elle une de ces conditions ? Test 1.2.1

  - La balise

        <img>

    possède un attribut

        alt

    vide (

        alt=""

    ) et est dépourvue de tout autre attribut permettant de fournir une alternative textuelle ;

  - La balise

        <img>

    possède un attribut WAI-ARIA

        aria-hidden="true"

    ou

        role="presentation"

    .

  <!-- -->

  - Retrouver dans le document les images décoratives dépourvues de légende structurées au moyen d’un élément

        <img>

    ;

  - Pour chaque image, vérifier que l’image ne possède pas d’attributs

        aria-labelledby

    ,

        aria-label

    ou

        title

    et qu’elle possède :

    - Soit un attribut

          alt

      vide (

          alt=""

      ) ;

    - Soit un attribut WAI-ARIA

          aria-hidden="true"

      ou

          role="presentation"

      .

  - Soit un attribut

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.2.2

  Chaque zone non cliquable (balise

      <area>

  sans attribut

      href

  ) de décoration, vérifie-t-elle une de ces conditions ? Test 1.2.2

  - La balise

        <area>

    possède un attribut

        alt

    vide (

        alt=""

    ) et est dépourvue de tout autre attribut permettant de fournir une alternative textuelle ;

  - La balise

        <area>

    possède un attribut WAI-ARIA

        aria-hidden="true"

    ou

        role="presentation"

    .

  <!-- -->

  - Retrouver dans le document les images décoratives structurées au moyen d’un élément

        <area>

    (sans attribut

        href

    ) ;

  - Pour chaque image, vérifier que l’élément

        <area>

    ne possède pas d’attributs

        aria-labelledby

    ,

        aria-label

    ou

        title

    et qu’il possède :

    - Soit un attribut

          alt

      vide (

          alt=""

      ) ;

    - Soit un attribut WAI-ARIA

          aria-hidden="true"

      ou

          role="presentation"

      .

  - Soit un attribut

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.2.3

  Chaque image objet (balise

      <object>

  avec l’attribut

      type="image/…"

  ) de décoration, sans légende, vérifie-t-elle ces conditions ? Test 1.2.3

  - La balise

        <object>

    possède un attribut WAI-ARIA

        aria-hidden="true"

    ;

  - La balise

        <object>

    est dépourvue d’alternative textuelle ;

  - Il n’y a aucun texte faisant office d’alternative textuelle entre

        <object>

    et

        </object>

    .

  <!-- -->

  - Retrouver dans le document les images décoratives structurées dépourvues de légende au moyen d’un élément

        <object>

    (avec un attribut

        type="image/…"

    ) ;

  - Pour chaque image, vérifier que la balise ouvrante

        <object>

    ne possède pas d’attributs

        aria-labelledby

    ,

        aria-label

    ou

        title

    et qu’elle :

    - Possède un attribut WAI-ARIA

          aria-hidden="true"

      ;

    - Et est dépourvue d’alternative textuelle ;

    - Et est dépourvue d’un contenu alternatif présent entre les balises

          <object>

      et

          </object>

      .

  - Possède un attribut WAI-ARIA

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.2.4

  Chaque image vectorielle (balise

      <svg>

  ) de décoration, sans légende, vérifie-t-elle ces conditions ? Test 1.2.4

  - La balise

        <svg>

    possède un attribut WAI-ARIA

        aria-hidden="true"

    ;

  - La balise

        <svg>

    et ses enfants sont dépourvus d’alternative textuelle ;

  - Les balises

        <title>

    et

        <desc>

    sont absentes ou vides ;

  - La balise

        <svg>

    et ses enfants sont dépourvus d’attribut

        title

    .

  <!-- -->

  - Retrouver dans le document les images décoratives dépourvues de légende structurées au moyen d’un élément

        <svg>

    ;

  - Pour chaque image, vérifier que l’élément

        <svg>

    ne possède pas d’attributs

        aria-labelledby

    ou

        aria-label

    et qu’il :

    - Possède un attribut WAI-ARIA

          aria-hidden="true"

      ;

    - Et est dépourvu d’alternative textuelle (ainsi que ses éléments enfants) ;

    - Et ne contient pas d’éléments

          <title>

      et

          <desc>

      à moins que vides de contenu ;

    - Et est dépourvu d’attribut

          title

      (ainsi que ses éléments enfants).

  - Possède un attribut WAI-ARIA

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.2.5

  Chaque image bitmap (balise

      <canvas>

  ) de décoration, sans légende, vérifie-t-elle ces conditions ? Test 1.2.5

  - La balise

        <canvas>

    possède un attribut WAI-ARIA

        aria-hidden="true"

    ;

  - La balise

        <canvas>

    et ses enfants sont dépourvus d’alternative textuelle ;

  - Il n’y a aucun texte faisant office d’alternative textuelle entre

        <canvas>

    et

        </canvas>

    .

  <!-- -->

  - Retrouver dans le document les images décoratives dépourvues de légende structurées au moyen d’un élément

        <canvas>

    ;

  - Pour chaque image, vérifier que l’élément

        <canvas>

    ne possède pas d’attributs

        aria-labelledby

    ,

        aria-label

    ou

        title

    et qu’il :

    - Possède un attribut WAI-ARIA

          aria-hidden="true"

      ;

    - Et est dépourvu d’alternative textuelle ;

    - Et est dépourvu d’un contenu alternatif présent entre les balises

          <canvas>

      et

          </canvas>

      .

  - Possède un attribut WAI-ARIA

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.2.6

  Chaque image embarquée (balise

      <embed>

  avec l’attribut

      type="image/…"

  ) de décoration, sans légende, vérifie-t-elle ces conditions ? Test 1.2.6

  - La balise

        <embed>

    possède un attribut WAI-ARIA

        aria-hidden="true"

    ;

  - La balise

        <embed>

    et ses enfants sont dépourvus d’alternative textuelle.

  <!-- -->

  - Retrouver dans le document les images décoratives dépourvues de légende structurées au moyen d’un élément

        <embed>

    (avec un attribut

        type="image/…"

    ) ;

  - Pour chaque image, vérifier que l’élément

        <embed>

    ne possède pas d’attributs

        aria-labelledby

    ,

        aria-label

    ou

        title

    et qu’il :

    - Possède un attribut WAI-ARIA

          aria-hidden="true"

      ;

    - Et est dépourvu d’alternative textuelle ;

  - Possède un attribut WAI-ARIA

  - Si c’est le cas pour chaque image, le test est validé.

  #### Note technique

  Lorsqu'une image est associée à une légende, la note technique WCAG recommande de prévoir systématiquement une alternative textuelle (cf. critère 1.9). Dans ce cas le critère 1.2 est non applicable.

  Dans le cas d'une image vectorielle (balise

      <svg>

  ) de décoration qui serait affichée au travers d'un élément

      <use href="…">

  enfant de l'élément

      <svg>

  , le test 1.2.4 s'appliquera également à l'élément

      <svg>

  associée par le biais de l'élément

      <use>

  .

  Un attribut WAI-ARIA

      role="presentation"

  peut être utilisé sur les images de décoration et les zones non cliquables de décoration. Le rôle

      "none"

  introduit en ARIA 1.1 et synonyme du rôle

      "presentation"

  peut être aussi utilisé. Il reste préférable cependant d'utiliser le rôle

      "presentation"

  en attendant un support satisfaisant du rôle

      "none"

  .

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.4.1.2 Name, Role, Value (A)

- La balise

- ### 1.3 Pour chaque image porteuse d’information ayant une alternative textuelle, cette alternative est-elle pertinente (hors cas particuliers) ? Critère 1.3

  #### 1.3.1

  Chaque image (balise

      <img>

  ou balise possédant l’attribut WAI-ARIA

      role="img"

  ) porteuse d’information, ayant une alternative textuelle, cette alternative est-elle pertinente (hors cas particuliers) ? Test 1.3.1

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les images structurées au moyen d’un élément

        <img>

    (ou d’un élément possédant l’attribut WAI-ARIA

        role="img"

    ) pourvues d’une alternative textuelle ;

  - Pour chaque image, vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.2

  Pour chaque zone (balise

      <area>

  ) d’une image réactive porteuse d’information, ayant une alternative textuelle, cette alternative est-elle pertinente (hors cas particuliers) ? Test 1.3.2

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <area>

    pourvus d’une alternative textuelle ;

  - Pour chaque élément

        <area>

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.3

  Pour chaque bouton de type

      image

  (balise

      <input>

  avec l’attribut

      type="image"

  ), ayant une alternative textuelle, cette alternative est-elle pertinente (hors cas particuliers) ? Test 1.3.3

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <input>

    pourvus de l’attribut

        type="image"

    et d’une alternative textuelle ;

  - Pour chaque élément

        <input>

    pourvu de l’attribut

        type="image"

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.4

  Pour chaque image objet (balise

      <object>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, ayant une alternative textuelle ou un contenu alternatif, cette alternative est-elle pertinente (hors cas particuliers) ? Test 1.3.4

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent ;

  - S’il est présent le contenu alternatif est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <object>

    pourvus de l’attribut

        type="image/…"

    et d’une alternative textuelle ;

  - Pour chaque élément

        <object>

    pourvu de l’attribut

        type="image/…"

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.5

  Pour chaque image embarquée (balise

      <embed>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, ayant une alternative textuelle ou un contenu alternatif, cette alternative est-elle pertinente (hors cas particuliers) ? Test 1.3.5

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent ;

  - S’il est présent le contenu alternatif est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <embed>

    pourvus de l’attribut

        type="image/…"

    et d’une alternative textuelle ;

  - Pour chaque élément

        <embed>

    pourvu de l’attribut

        type="image/…"

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.6

  Pour chaque image vectorielle (balise

      <svg>

  ) porteuse d’information, ayant une alternative textuelle, cette alternative est-elle pertinente (hors cas particuliers) ? Test 1.3.6

  - S’il est présent, le contenu de l’élément

        <title>

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <svg>

    pourvus d’une alternative textuelle ;

  - Pour chaque élément

        <svg>

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.7

  Pour chaque image bitmap (balise

      <canvas>

  ) porteuse d’information, ayant une alternative textuelle ou un contenu alternatif, cette alternative est-elle pertinente (hors cas particuliers) ? Test 1.3.7

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent ;

  - S’il est présent le contenu alternatif est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <canvas>

    pourvus d’une alternative textuelle ;

  - Pour chaque élément

        <canvas>

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.8

  Pour chaque image bitmap (balise

      <canvas>

  ) porteuse d’information et ayant un contenu alternatif entre

      <canvas>

  et

      </canvas>

  , ce contenu alternatif est-il correctement restitué par les technologies d’assistance ? Test 1.3.8

  - Retrouver dans le document les éléments

        <canvas>

    pourvus d’un contenu alternatif entre les balises

        <canvas>

    et

        </canvas>

    ;

  - Pour chaque élément

        <canvas>

    , vérifier que le contenu alternatif est correctement restitué par les technologies d’assistance ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.3.9

  Pour chaque image porteuse d’information et ayant une alternative textuelle, l’alternative textuelle est-elle courte et concise (hors cas particuliers) ? Test 1.3.9

  - Retrouver dans le document les images pourvues d’une alternative textuelle ;
  - Pour chaque image, vérifier l’alternative textuelle est courte et concise ;
  - Si c’est le cas pour chaque image, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particuliers lorsque l’image est utilisée comme CAPTCHA ou comme image-test. Dans cette situation, où il n’est pas possible de donner une alternative pertinente sans détruire l’objet du CAPTCHA ou du test, le critère est non applicable.

  Note : le cas des CAPTCHA et des images-test est traité de manière spécifique par le critère 1.4.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.4.1.2 Name, Role, Value (A)

- S’il est présent, le contenu de l’attribut

- ### 1.4 Pour chaque image utilisée comme CAPTCHA ou comme image-test, ayant une alternative textuelle, cette alternative permet-elle d’identifier la nature et la fonction de l’image ? Critère 1.4

  #### 1.4.1

  Pour chaque image (balise

      <img>

  ) utilisée comme CAPTCHA ou comme image-test, ayant une alternative textuelle, cette alternative est-elle pertinente ? Test 1.4.1

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les images structurées au moyen d’un élément

        <img>

    pourvues d’une alternative textuelle et utilisées comme CAPTCHA ou comme image-test ;

  - Pour chaque image, vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.4.2

  Pour chaque zone (balise

      <area>

  ) d’une image réactive utilisée comme CAPTCHA ou comme image-test, ayant une alternative textuelle, cette alternative est-elle pertinente ? Test 1.4.2

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <area>

    pourvus d’une alternative textuelle et utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque élément

        <area>

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.4.3

  Pour chaque bouton de type image (balise

      <input>

  avec l’attribut

      type="image"

  ) utilisé comme CAPTCHA ou comme image-test, ayant une alternative textuelle, cette alternative est-elle pertinente ? Test 1.4.3

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <input>

    pourvus de l’attribut

        type="image"

    et d’une alternative textuelle, et utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque élément

        <input>

    pourvu de l’attribut

        type="image"

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.4.4

  Pour chaque image objet (balise

      <object>

  avec l’attribut

      type="image/…"

  ) utilisée comme CAPTCHA ou comme image-test, ayant une alternative textuelle ou un contenu alternatif, cette alternative est-elle pertinente ? Test 1.4.4

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent ;

  - S’il est présent le contenu alternatif est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <object>

    pourvus de l’attribut

        type="image/…"

    et d’une alternative textuelle, et utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque élément

        <object>

    pourvu de l’attribut

        type="image/…"

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.4.5

  Pour chaque image embarquée (balise

      <embed>

  avec l’attribut

      type="image/…"

  ) utilisée comme CAPTCHA ou comme image-test, ayant une alternative textuelle ou un contenu alternatif, cette alternative est-elle pertinente ? Test 1.4.5

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent ;

  - S’il est présent le contenu alternatif est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <embed>

    pourvus de l’attribut

        type="image/…"

    et d’une alternative textuelle, et utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque élément

        <embed>

    pourvu de l’attribut

        type="image/…"

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.4.6

  Pour chaque image vectorielle (balise

      <svg>

  ) utilisée comme CAPTCHA ou comme image-test, ayant une alternative textuelle, cette alternative est-elle pertinente ? Test 1.4.6

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <svg>

    pourvus d’une alternative textuelle et utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque élément

        <svg>

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.4.7

  Pour chaque image bitmap (balise

      <canvas>

  ) utilisée comme CAPTCHA ou comme image-test, ayant une alternative textuelle ou un contenu alternatif, cette alternative est-elle pertinente ? Test 1.4.7

  - S’il est présent, le contenu de l’attribut

        alt

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent ;

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte associé via l’attribut WAI-ARIA

        aria-labelledby

    est pertinent ;

  - S’il est présent le contenu alternatif est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <canvas>

    pourvus d’une alternative textuelle et utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque élément

        <canvas>

    , vérifier que l’alternative textuelle est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- S’il est présent, le contenu de l’attribut

- ### 1.5 Pour chaque image utilisée comme CAPTCHA, une solution d’accès alternatif au contenu ou à la fonction du CAPTCHA est-elle présente ? Critère 1.5

  #### 1.5.1

  Chaque image (balises

      <img>

  ,

      <area>

  ,

      <object>

  ,

      <embed>

  ,

      <svg>

  ,

      <canvas>

  ou possédant un attribut WAI-ARIA

      role="img"

  ) utilisée comme CAPTCHA vérifie-t-elle une de ces conditions ? Test 1.5.1

  - Il existe une autre forme de CAPTCHA non graphique, au moins ;
  - Il existe une autre solution d’accès à la fonctionnalité qui est sécurisée par le CAPTCHA.

  <!-- -->

  - Retrouver dans le document les images (éléments

        <img>

    ,

        <area>

    ,

        <object>

    ,

        <embed>

    ,

        <svg>

    ,

        <canvas>

    ou possédant un attribut WAI-ARIA

        role="img"

    ) utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque image, vérifier qu’il existe :
    - Soit une autre forme de CAPTCHA non graphique, au moins ;
    - Soit une autre solution d’accès à la fonctionnalité qui est sécurisée par le CAPTCHA.

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.5.2

  Chaque bouton associé à une image (balise

      input

  avec l’attribut

      type="image"

  ) utilisée comme CAPTCHA vérifie-t-il une de ces conditions ? Test 1.5.2

  - Il existe une autre forme de CAPTCHA non graphique, au moins ;
  - Il existe une autre solution d’accès à la fonctionnalité sécurisée par le CAPTCHA.

  <!-- -->

  - Retrouver dans le document les boutons associés à une image (éléments

        <input>

    avec l’attribut

        type="image"

    ) utilisés comme CAPTCHA ou comme image-test ;

  - Pour chaque bouton associé à une image, vérifier qu’il existe :
    - Soit une autre forme de CAPTCHA non graphique, au moins ;
    - Soit une autre solution d’accès à la fonctionnalité qui est sécurisée par le CAPTCHA.

  - Si c’est le cas pour chaque image, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- ### 1.6 Chaque image porteuse d’information a-t-elle, si nécessaire, une description détaillée ? Critère 1.6

  #### 1.6.1

  Chaque image (balise

      <img>

  ) porteuse d’information, qui nécessite une description détaillée, vérifie-t-elle une de ces conditions ? Test 1.6.1

  - Il existe un attribut

        longdesc

    qui donne l’adresse (URL) d’une page ou d’un emplacement dans la page contenant la description détaillée ;

  - Il existe une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;

  - Il existe un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  <!-- -->

  - Retrouver dans le document les images structurées au moyen d’un élément

        <img>

    (ou d’un élément possédant l’attribut WAI-ARIA

        role="img"

    ) porteuses d’information qui nécessitent une description détaillée ;

  - Pour chaque image, vérifier qu’il existe :
    - Soit un attribut longdesc qui donne l’adresse (url) d’une page ou d’un emplacement dans la page contenant la description détaillée ;
    - Soit une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;
    - Soit un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.6.2

  Chaque image objet (balise

      <object>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, qui nécessite une description détaillée, vérifie-t-elle une de ces conditions ? Test 1.6.2

  - Il existe un attribut

        longdesc

    qui donne l’adresse (URL) d’une page ou d’un emplacement dans la page contenant la description détaillée ;

  - Il existe une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;

  - Il existe un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  <!-- -->

  - Retrouver dans le document les éléments

        <object>

    pourvus de l’attribut

        type="image/…"

    , porteurs d’information qui nécessitent une description détaillée ;

  - Pour chaque élément

        <object>

    pourvu de l’attribut

        type="image/…"

    , vérifier qu’il existe :

    - Soit une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;
    - Soit un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  - Si c’est le cas pour chaque élément

        <object>

    pourvu de l’attribut

        type="image/…"

    , le test est validé.

  #### 1.6.3

  Chaque image embarquée (balise

      <embed>

  ) porteuse d’information, qui nécessite une description détaillée, vérifie-t-elle une de ces conditions ? Test 1.6.3

  - Il existe un attribut

        longdesc

    qui donne l’adresse (URL) d’une page ou d’un emplacement dans la page contenant la description détaillée ;

  - Il existe une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;

  - Il existe un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  <!-- -->

  - Retrouver dans le document les éléments

        <embed>

    pourvus de l’attribut

        type="image/…"

    , porteurs d’information qui nécessitent une description détaillée ;

  - Pour chaque élément

        <embed>

    pourvu de l’attribut

        type="image/…"

    , vérifier qu’il existe :

    - Soit une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;
    - Soit un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  - Si c’est le cas pour chaque élément

        <embed>

    pourvu de l’attribut

        type="image/…"

    , le test est validé.

  #### 1.6.4

  Chaque bouton de type image (balise

      <input>

  avec l’attribut

      type="image"

  ) porteur d’information, qui nécessite une description détaillée, vérifie-t-il une de ces conditions ? Test 1.6.4

  - Il existe un attribut

        longdesc

    qui donne l’adresse (URL) d’une page ou d’un emplacement dans la page contenant la description détaillée ;

  - Il existe une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;

  - Il existe un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  <!-- -->

  - Retrouver dans le document les éléments

        <input>

    pourvus de l’attribut

        type="image"

    , porteurs d’information qui nécessitent une description détaillée ;

  - Pour chaque élément

        <input>

    pourvu de l’attribut

        type="image"

    , vérifier qu’il existe :

    - Soit une alternative textuelle contenant la référence à une description détaillée adjacente à l’image ;
    - Soit un lien ou un bouton adjacent permettant d’accéder à la description détaillée ;
    - Soit un attribut WAI-ARIA aria-describedby associant un passage de texte faisant office de description détaillée.

  - Si c’est le cas pour chaque élément

        <input>

    pourvu de l’attribut

        type="image"

    , le test est validé.

  #### 1.6.5

  Chaque image vectorielle (balise

      <svg>

  ) porteuse d’information, qui nécessite une description détaillée, vérifie-t-elle une de ces conditions ? Test 1.6.5

  - Il existe un attribut WAI-ARIA

        aria-label

    contenant l’alternative textuelle et une référence à une description détaillée adjacente ;

  - Il existe un attribut WAI-ARIA

        aria-labelledby

    associant un passage de texte faisant office d’alternative textuelle et un autre faisant office de description détaillée ;

  - Il existe un attribut WAI-ARIA

        aria-describedby

    associant un passage de texte faisant office de description détaillée ;

  - Il existe un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  <!-- -->

  - Retrouver dans le document les éléments

        <svg>

    porteurs d’information qui nécessitent une description détaillée ;

  - Pour chaque élément

        <svg>

    , vérifier qu’il existe :

    - Soit un attribut WAI-ARIA

          aria-label

      contenant l’alternative textuelle et une référence à une description détaillée adjacente ;

    - Soit un attribut WAI-ARIA

          aria-labelledby

      associant un passage de texte faisant office d’alternative textuelle et un autre faisant office de description détaillée ;

    - Soit un attribut WAI-ARIA

          aria-describedby

      associant un passage de texte faisant office de description détaillée ;

    - Soit un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  - Soit un attribut WAI-ARIA

  - Si c’est le cas pour chaque élément

        <svg>

    , le test est validé.

  #### 1.6.6

  Pour chaque image vectorielle (balise

      <svg>

  ) porteuse d’information, ayant une description détaillée, la référence éventuelle à la description détaillée dans l’attribut WAI-ARIA

      aria-label

  et la description détaillée associée par l’attribut WAI-ARIA

      aria-labelledby

  ou

      aria-describedby

  sont-elles correctement restituées par les technologies d’assistance ? Test 1.6.6

  - Retrouver dans le document les éléments

        <svg>

    porteurs d’information dont la description détaillée est fournie au moyen d’un attribut

        aria-label

    ,

        aria-labelledby

    ou

        aria-describedby

    ;

  - Pour chaque élément

        <svg>

    , vérifier que le contenu de la description détaillée est correctement restitué par les technologies d’assistance ;

  - Si c’est le cas pour chaque élément

        <svg>

    , le test est validé.

  #### 1.6.7

  Chaque image bitmap (balise

      <canvas>

  ), porteuse d’information, qui nécessite une description détaillée, vérifie-t-elle une de ces conditions ? Test 1.6.7

  - Il existe un attribut WAI-ARIA

        aria-label

    contenant l’alternative textuelle et une référence à une description détaillée adjacente ;

  - Il existe un attribut WAI-ARIA

        aria-labelledby

    associant un passage de texte faisant office d’alternative textuelle et un autre faisant office de description détaillée ;

  - Il existe un contenu textuel entre

        <canvas>

    et

        </canvas>

    faisant référence à une description détaillée adjacente à l’image bitmap ;

  - Il existe un contenu textuel entre

        <canvas>

    et

        </canvas>

    faisant office de description détaillée ;

  - Il existe un lien ou bouton adjacent permettant d’accéder à la description détaillée.

  <!-- -->

  - Retrouver dans le document les éléments

        <canvas>

    porteurs d’information qui nécessitent une description détaillée ;

  - Pour chaque élément

        <canvas>

    , vérifier qu’il existe :

    - Soit un attribut WAI-ARIA aria-label contenant l’alternative textuelle et une référence à une description détaillée adjacente ;

    - Soit un attribut WAI-ARIA aria-labelledby associant un passage de texte faisant office d’alternative textuelle et un autre faisant office de description détaillée ;

    - Soit un contenu textuel entre

          <canvas>

      et

          </canvas>

      faisant référence à une description détaillée adjacente à l’image bitmap ;

    - Soit un contenu textuel entre

          <canvas>

      et

          </canvas>

      faisant office de description détaillée ;

    - Soit un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  - Si c’est le cas pour chaque élément

        <canvas>

    , le test est validé.

  #### 1.6.8

  Pour chaque image bitmap (balise

      <canvas>

  ) porteuse d’information, qui implémente une référence à une description détaillée adjacente, cette référence est-elle correctement restituée par les technologies d’assistance ? Test 1.6.8

  - Retrouver dans le document les éléments

        <canvas>

    porteurs d’information dont la description détaillée est fournie au moyen d’un attribut

        aria-label

    ,

        aria-labelledby

    ou

        aria-describedby

    ;

  - Pour chaque élément

        <canvas>

    , vérifier que le contenu de la description détaillée est correctement restitué par les technologies d’assistance ;

  - Si c’est le cas pour chaque élément

        <canvas>

    , le test est validé.

  #### 1.6.9

  Pour chaque image (balise

      <img>

  ,

      <input>

  avec l’attribut

      type="image"

  ,

      <area>

  ,

      <object>

  ,

      <embed>

  ,

      <svg>

  ,

      <canvas>

  , ou possédant un attribut WAI-ARIA

      role="img"

  ) porteuse d’information, qui est accompagnée d’une description détaillée et qui utilise un attribut WAI-ARIA

      aria-describedby

  , l’attribut WAI-ARIA

      aria-describedby

  associe-t-il la description détaillée ? Test 1.6.9

  - Retrouver dans le document les images (éléments

        <img>

    ,

        <input>

    avec l’attribut

        type="image"

    ,

        <area>

    ,

        <object>

    ,

        <embed>

    ,

        <svg>

    ,

        <canvas>

    ou possédant un attribut WAI-ARIA

        role="img"

    ) porteuses d’information dont la description détaillée utilise un attribut WAI-ARIA

        aria-describedby

    ;

  - Pour chaque image, vérifier que le contenu de la description détaillée est correctement restitué par les technologies d’assistance ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.6.10

  Chaque balise possédant un attribut WAI-ARIA

      role="img"

  porteuse d’information, qui nécessite une description détaillée, vérifie-t-elle une de ces conditions ? Test 1.6.10

  - Il existe un attribut WAI-ARIA

        aria-label

    contenant l’alternative textuelle et une référence à une description détaillée adjacente ;

  - Il existe un attribut WAI-ARIA

        aria-labelledby

    associant un passage de texte faisant office d’alternative textuelle et un autre faisant office de description détaillée ;

  - Il existe un attribut WAI-ARIA

        aria-describedby

    associant un passage de texte faisant office de description détaillée ;

  - Il existe un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  <!-- -->

  - Retrouver dans le document les éléments pourvus d’un attribut WAI-ARIA

        role="img"

    porteurs d’information qui nécessitent une description détaillée ;

  - Pour chaque élément

        role="img"

    , vérifier qu’il existe :

    - Soit un attribut WAI-ARIA

          aria-label

      contenant l’alternative textuelle et une référence à une description détaillée adjacente ;

    - Soit un attribut WAI-ARIA

          aria-labelledby

      associant un passage de texte faisant office d’alternative textuelle et un autre faisant office de description détaillée ;

    - Soit un attribut WAI-ARIA

          aria-describedby

      associant un passage de texte faisant office de description détaillée ;

    - Soit un lien ou un bouton adjacent permettant d’accéder à la description détaillée.

  - Soit un attribut WAI-ARIA

  - Si c’est le cas pour chaque élément

        role="img"

    , le test est validé.

  #### Notes techniques

  Dans le cas du SVG, le manque de support de l’élément

      <title>

  et

      <desc>

  par les technologies d’assistance crée une difficulté dans le cas de l’implémentation de l’alternative textuelle de l’image et de sa description détaillée. Dans ce cas, il est recommandé d’utiliser l’attribut WAI-ARIA

      aria-label

  pour implémenter à la fois l’alternative textuelle courte et la référence à la description détaillée adjacente ou l’attribut WAI-ARIA

      aria-labelledby

  pour associer les passages de texte faisant office d’alternative courte et de description détaillée.

  L’utilisation de l’attribut WAI-ARIA aria-describedby n’est pas recommandée pour lier une image (

      <img>

  ,

      <object>

  ,

      <embed>

  ,

      <canvas>

  ) à sa description détaillée, par manque de support des technologies d’assistance. Néanmoins, lorsqu’il est utilisé, l’attribut devra nécessairement faire référence à l’

      id

  de la zone contenant la description détaillée.

  La description détaillée adjacente peut être implémentée via une balise

      <figcaption>

  , dans ce cas le critère 1.9 doit être vérifié (utilisation de

      <figure>

  et des attributs WAI-ARIA

      role="figure"

  et

      aria-label

  , notamment).

  L'attribut

      longdesc

  qui constitue une des conditions du test 1.6.1 (et dont la pertinence est vérifiée avec le test 1.7.1) est désormais considéré comme obsolète par la spécification HTML en cours. La vérification de cet attribut ne sera donc requise que pour les versions de la spécification HTML antérieure à HTML 5.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- Il existe un attribut

- ### 1.7 Pour chaque image porteuse d’information ayant une description détaillée, cette description est-elle pertinente ? Critère 1.7

  #### 1.7.1

  Chaque image (balise

      <img>

  ) porteuse d’information, ayant une description détaillée, vérifie-t-elle ces conditions ? Test 1.7.1

  - La description détaillée via l’adresse référencée dans l’attribut

        longdesc

    est pertinente ;

  - La description détaillée dans la page et signalée par l’alternative textuelle est pertinente ;

  - La description détaillée via un lien ou un bouton adjacent est pertinente ;

  - Le passage de texte associé via l’attribut WAI-ARIA

        aria-describedby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les images structurées au moyen d’un élément

        <img>

    qui possèdent une description détaillée ;

  - Pour chaque image, vérifier que la description détaillée est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.7.2

  Chaque bouton de type image (balise

      <input>

  avec l’attribut

      type="image"

  ) porteur d’information, ayant une description détaillée, vérifie-t-il ces conditions ? Test 1.7.2

  - La description détaillée dans la page et signalée par l’alternative textuelle est pertinente ;

  - La description détaillée via un lien ou un bouton adjacent est pertinente ;

  - Le passage de texte associé via l’attribut WAI-ARIA

        aria-describedby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <input>

    pourvus de l’attribut

        type="image"

    qui possèdent une description détaillée ;

  - Pour chaque élément

        <input>

    pourvu de l’attribut

        type="image"

    , vérifier que la description détaillée est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.7.3

  Chaque image objet (balise

      <object>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, ayant une description détaillée, vérifie-t-elle ces conditions ? Test 1.7.3

  - La description détaillée dans la page et signalée par l’alternative textuelle est pertinente ;

  - La description détaillée adjacente à l’image objet est pertinente ;

  - La description détaillée via un lien ou un bouton adjacent est pertinente ;

  - Le passage de texte associé via l’attribut WAI-ARIA

        aria-describedby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <object>

    pourvus de l’attribut

        type="image/…"

    qui possèdent une description détaillée ;

  - Pour chaque élément

        <object>

    pourvu de l’attribut

        type="image/…"

    , vérifier que la description détaillée est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.7.4

  Chaque image embarquée (balise

      <embed>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, ayant une description détaillée, vérifie-t-elle ces conditions ? Test 1.7.4

  - La description détaillée dans la page et signalée par l’alternative textuelle est pertinente ;

  - La description détaillée adjacente à l’image embarquée est pertinente ;

  - La description détaillée via un lien ou un bouton adjacent est pertinente ;

  - Le passage de texte associé via l’attribut WAI-ARIA

        aria-describedby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <embed>

    pourvus de l’attribut

        type="image/…"

    qui possèdent une description détaillée ;

  - Pour chaque élément

        <embed>

    pourvu de l’attribut

        type="image/…"

    , vérifier que la description détaillée est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.7.5

  Chaque image vectorielle (balise

      <svg>

  ) porteuse d’information, ayant une description détaillée, vérifie-t-elle ces conditions ? Test 1.7.5

  - La description détaillée dans la page et signalée par l’alternative textuelle est pertinente ;

  - La description détaillée dans la page et signalée par le texte contenu dans la balise

        <desc>

    ou

        <title>

    est pertinente ;

  - La description détaillée adjacente contenue dans la balise

        <desc>

    est pertinente ;

  - La description détaillée via un lien ou un bouton adjacent est pertinente ;

  - Le passage de texte associé via l’attribut WAI-ARIA

        aria-describedby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <svg>

    qui possèdent une description détaillée ;

  - Pour chaque élément

        <svg>

    , vérifier que la description détaillée est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.7.6

  Chaque image bitmap (balise

      <canvas>

  ) porteuse d’information, ayant une description détaillée, vérifie-t-elle ces conditions ? Test 1.7.6

  - La description détaillée dans la page et signalée par l’alternative textuelle est pertinente ;

  - La description détaillée dans la page et signalée par le texte contenu entre

        <canvas>

    et

        </canvas>

    est pertinente ;

  - La description détaillée contenue entre

        <canvas>

    et

        </canvas>

    est pertinente ;

  - La description détaillée adjacente à l’image bitmap est pertinente ;

  - La description détaillée via un lien ou un bouton adjacent est pertinente ;

  - Le passage de texte associé via l’attribut WAI-ARIA

        aria-describedby

    est pertinent.

  <!-- -->

  - Retrouver dans le document les éléments

        <canvas>

    qui possèdent une description détaillée ;

  - Pour chaque élément

        <canvas>

    , vérifier que la description détaillée est pertinente ;

  - Si c’est le cas pour chaque image, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- La description détaillée via l’adresse référencée dans l’attribut

- ### 1.8 Chaque image texte porteuse d’information, en l’absence d’un mécanisme de remplacement, doit si possible être remplacée par du texte stylé. Cette règle est-elle respectée (hors cas particuliers) ? Critère 1.8

  #### 1.8.1

  Chaque image texte (balise

      <img>

  ou possédant un attribut WAI-ARIA

      role="img"

  ) porteuse d’information, en l’absence d’un mécanisme de remplacement, doit si possible être remplacée par du texte stylé. Cette règle est-elle respectée (hors cas particuliers) ? Test 1.8.1

  - Retrouver dans le document les images texte structurées au moyen d’un élément

        <img>

    (ou d’un élément possédant l’attribut WAI-ARIA

        role="img"

    ) ;

  - Pour chaque image, vérifier que :
    - Soit il existe un mécanisme de remplacement ;
    - Soit l’image contient un texte qui fait appel à un effet graphique qui ne peut pas être reproduit en CSS.

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.8.2

  Chaque bouton « image texte » (balise

      <input>

  avec l’attribut

      type="image"

  ) porteur d’information, en l’absence d’un mécanisme de remplacement, doit si possible être remplacé par du texte stylé. Cette règle est-elle respectée (hors cas particuliers) ? Test 1.8.2

  - Retrouver dans le document les boutons “images texte” (élément

        <input>

    avec l’attribut

        type="image"

    ) ;

  - Pour chaque image, vérifier que :
    - Soit il existe un mécanisme de remplacement ;
    - Soit l’image contient un texte qui fait appel à un effet graphique qui ne peut pas être reproduit en CSS.

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.8.3

  Chaque image texte objet (balise

      <object>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, en l’absence d’un mécanisme de remplacement, doit si possible être remplacée par du texte stylé. Cette règle est-elle respectée (hors cas particuliers) ? Test 1.8.3

  - Retrouver dans le document les images texte objet (élément

        <object>

    avec l’attribut

        type="image/…"

    ) ;

  - Pour chaque image, vérifier que :
    - Soit il existe un mécanisme de remplacement ;
    - Soit l’image contient un texte qui fait appel à un effet graphique qui ne peut pas être reproduit en CSS.

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.8.4

  Chaque image texte embarquée (balise

      <embed>

  avec l’attribut

      type="image/…"

  ) porteuse d’information, en l’absence d’un mécanisme de remplacement, doit si possible être remplacée par du texte stylé. Cette règle est-elle respectée (hors cas particuliers) ? Test 1.8.4

  - Retrouver dans le document les images texte embarquées (élément

        <embed>

    avec l’attribut

        type="image/…"

    ) ;

  - Pour chaque image, vérifier que :
    - Soit il existe un mécanisme de remplacement ;
    - Soit l’image contient un texte qui fait appel à un effet graphique qui ne peut pas être reproduit en CSS.

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.8.5

  Chaque image texte bitmap (balise

      <canvas>

  ) porteuse d’information, en l’absence d’un mécanisme de remplacement, doit si possible être remplacée par du texte stylé. Cette règle est-elle respectée (hors cas particuliers) ? Test 1.8.5

  - Retrouver dans le document les images texte bitmap (élément

        <canvas>

    ) ;

  - Pour chaque image, vérifier que :
    - Soit il existe un mécanisme de remplacement ;
    - Soit l’image contient un texte qui fait appel à un effet graphique qui ne peut pas être reproduit en CSS.

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.8.6

  Chaque image texte SVG (balise

      <svg>

  ) porteuse d’information et dont le texte n’est pas complètement structuré au moyen d’éléments

      <text>

  , en l’absence d’un mécanisme de remplacement, doit si possible être remplacée par du texte stylé. Cette règle est-elle respectée (hors cas particuliers) ? Test 1.8.6

  - Retrouver dans le document les images texte vectorielle (élément

        <svg>

    ) porteuse d’information et dont le texte n’est pas complètement structuré au moyen d’éléments

        <text>

    ;

  - Pour chaque image, vérifier que :
    - Soit il existe un mécanisme de remplacement ;
    - Soit l’image contient un texte qui fait appel à un effet graphique qui ne peut pas être reproduit en CSS.

  - Si c’est le cas pour chaque image, le test est validé.

  #### Cas particuliers

  Pour ce critère, il existe une gestion de cas particulier lorsque le texte fait partie du logo, d’une dénomination commerciale, d’un CAPTCHA, d’une image-test ou d’une image dont l’exactitude graphique serait considérée comme essentielle à la bonne transmission de l’information véhiculée par l’image. Dans ces situations, le critère est non applicable pour ces éléments.

  #### Notes techniques

  Le texte dans les images vectorielles étant du texte réel, il n’est pas concerné par ce critère.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.5 Images of Text (AA)

- Retrouver dans le document les images texte structurées au moyen d’un élément

- ### 1.9 Chaque légende d’image est-elle, si nécessaire, correctement reliée à l’image correspondante ? Critère 1.9

  #### 1.9.1

  Chaque image pourvue d’une légende (balise

      <img>

  ,

      <input>

  avec l’attribut

      type="image"

  ou possédant un attribut WAI-ARIA

      role="img"

  associée à une légende adjacente), vérifie-t-elle, si nécessaire, ces conditions ? Test 1.9.1

  - L’image (balise

        <img>

    ,

        <input>

    avec l’attribut

        type="image"

    ou possédant un attribut WAI-ARIA

        role="img"

    ) et sa légende adjacente sont contenues dans une balise

        <figure>

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        role="figure"

    ou

        role="group"

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        aria-label

    dont le contenu est identique au contenu de la légende ;

  - La légende est contenue dans une balise

        <figcaption>

    .

  <!-- -->

  - Retrouver dans le document les images pourvues d’une légende structurées au moyen d’élément

        <img>

    , d’un élément

        <input>

    avec l’attribut

        type="image"

    ou d’un élément possédant l’attribut WAI-ARIA

        role="img"

    ;

  - Pour chaque image, vérifier que :
    - L’image et sa légende sont contenues dans une balise

          <figure>

      ;

    - La balise

          <figure>

      possède une propriété WAI-ARIA

          role="figure"

      ou

          role="group"

      ;

    - La balise

          <figure>

      possède un attribut WAI-ARIA

          aria-label

      dont le contenu est identique au contenu de la légende ;

    - La légende est contenue dans une balise

          <figcaption>

      .

  - L’image et sa légende sont contenues dans une balise

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.9.2

  Chaque image objet pourvue d’une légende (balise

      <object>

  avec l’attribut

      type="image/…"

  associée à une légende adjacente), vérifie-t-elle, si nécessaire, ces conditions ? Test 1.9.2

  - L’image objet et sa légende adjacente sont contenues dans une balise

        <figure>

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        role="figure"

    ou

        role="group"

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        aria-label

    dont le contenu est identique au contenu de la légende ;

  - La légende est contenue dans une balise

        <figcaption>

    .

  <!-- -->

  - Retrouver dans le document les images objet pourvues d’une légende (élément

        <object>

    avec l’attribut

        type="image/…"

    ) ;

  - Pour chaque image, vérifier que :
    - L’image et sa légende sont contenues dans une balise

          <figure>

      ;

    - La balise

          <figure>

      possède une propriété WAI-ARIA

          role="figure

      " ou

          role="group"

      ;

    - La balise

          <figure>

      possède un attribut WAI-ARIA

          aria-label

      dont le contenu est identique au contenu de la légende ;

    - La légende est contenue dans une balise

          <figcaption>

      .

  - L’image et sa légende sont contenues dans une balise

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.9.3

  Chaque image embarquée pourvue d’une légende (balise

      <embed>

  associée à une légende adjacente), vérifie-t-elle, si nécessaire, ces conditions ? Test 1.9.3

  - L’image embarquée (balise

        <embed>

    ) et sa légende adjacente sont contenues dans une balise

        <figure>

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        role="figure"

    ou

        role="group"

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        aria-label

    dont le contenu est identique au contenu de la légende ;

  - La légende est contenue dans une balise

        <figcaption>

    .

  <!-- -->

  - Retrouver dans le document les images embarquées pourvues d’une légende (élément

        <embed>

    avec l’attribut

        type="image/…"

    ) ;

  - Pour chaque image, vérifier que :
    - L’image et sa légende sont contenues dans une balise

          <figure>

      ;

    - La balise

          <figure>

      possède une propriété WAI-ARIA

          role="figure"

      ou

          role="group"

      ;

    - La balise

          <figure>

      possède un attribut WAI-ARIA

          aria-label

      dont le contenu est identique au contenu de la légende ;

    - La légende est contenue dans une balise

          <figcaption>

      .

  - L’image et sa légende sont contenues dans une balise

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.9.4

  Chaque image vectorielle pourvue d’une légende (balise

      <svg>

  associée à une légende adjacente), vérifie-t-elle, si nécessaire, ces conditions ? Test 1.9.4

  - L’image vectorielle (balise

        <svg>

    ) et sa légende adjacente sont contenues dans une balise

        <figure>

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        role="figure"

    ou

        role="group"

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        aria-label

    dont le contenu est identique au contenu de la légende ;

  - La légende est contenue dans une balise

        <figcaption>

    .

  <!-- -->

  - Retrouver dans le document les images vectorielles pourvues d’une légende (élément

        <svg>

    ) ;

  - Pour chaque image, vérifier que :
    - L’image et sa légende sont contenues dans une balise

          <figure>

      ;

    - La balise

          <figure>

      possède une propriété WAI-ARIA

          role="figure"

      ou

          role="group"

      ;

    - La balise

          <figure>

      possède un attribut WAI-ARIA

          aria-label

      dont le contenu est identique au contenu de la légende ;

    - La légende est contenue dans une balise

          <figcaption>

      .

  - L’image et sa légende sont contenues dans une balise

  - Si c’est le cas pour chaque image, le test est validé.

  #### 1.9.5

  Chaque image bitmap pourvue d’une légende (balise

      <canvas>

  associée à une légende adjacente), vérifie-t-elle, si nécessaire, ces conditions ? Test 1.9.5

  - L’image bitmap (balise

        <canvas>

    ) et sa légende adjacente sont contenues dans une balise

        <figure>

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        role="figure"

    ou

        role="group"

    ;

  - La balise

        <figure>

    possède un attribut WAI-ARIA

        aria-label

    dont le contenu est identique au contenu de la légende ;

  - La légende est contenue dans une balise

        <figcaption>

    .

  <!-- -->

  - Retrouver dans le document les images bitmap (élément

        <canvas>

    ) ;

  - Pour chaque image, vérifier que :
    - L’image et sa légende sont contenues dans une balise

          <figure>

      ;

    - La balise

          <figure>

      possède une propriété WAI-ARIA

          role="figure"

      ou

          role="group"

      ;

    - La balise

          <figure>

      possède un attribut WAI-ARIA

          aria-label

      dont le contenu est identique au contenu de la légende ;

    - La légende est contenue dans une balise

          <figcaption>

      .

  - L’image et sa légende sont contenues dans une balise

  - Si c’est le cas pour chaque image, le test est validé.

  #### Note technique

  L’implémentation d’un attribut WAI-ARIA

      role="group"

  ou

      role="figure"

  sur l’élément parent

      <figure>

  est destiné à pallier le manque de support actuel des éléments

      <figure>

  par les technologies d’assistance. L’utilisation d’un élément

      <figcaption>

  pour associer une légende à une image impose au minimum l’utilisation d’un attribut WAI-ARIA

      aria-label

  sur l’élément parent

      <figure>

  dont le contenu sera identique au contenu de l’élément

      <figcaption>

  . Pour s’assurer d’un support optimal, il peut également être fait une association explicite entre le contenu de l’alternative textuelle de l’image et le contenu de l’élément

      <figcaption>

  , par exemple :

      <img src="image.png" alt="Photo : soleil couchant" /><figcaption>Photo : crédit xxx</figcaption>

  Les attributs WAI-ARIA

      aria-labelledby

  et

      aria-describedby

  ne peuvent pas être utilisés actuellement par manque de support par les technologies d’assistance.

  Note : les images légendées doivent par ailleurs respecter le critère 1.1 et le critère 1.3 relatifs aux images porteuses d’information.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.4.1.2 Name, Role, Value (A)

- L’image (balise

## 2. Cadres Thématique Cadres

- ### 2.1 Chaque cadre a-t-il un titre de cadre ? Critère 2.1

  #### 2.1.1

  Chaque cadre (balise

      <iframe>

  ou

      <frame>

  ) a-t-il un attribut

      title

  ? Test 2.1.1

  - Retrouver dans le document les cadres (élément

        <iframe>

    ou

        <frame>

    ) ;

  - Pour chaque cadre, vérifier qu’il possède un attribut

        title

    ;

  - Si c’est le cas pour chaque cadre, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.4.1.2 Name, Role, Value (A)

- Retrouver dans le document les cadres (élément

- ### 2.2 Pour chaque cadre ayant un titre de cadre, ce titre de cadre est-il pertinent ? Critère 2.2

  #### 2.2.1

  Pour chaque cadre (balise

      <iframe>

  ou

      <frame>

  ) ayant un attribut

      title

  , le contenu de cet attribut est-il pertinent ? Test 2.2.1

  - Retrouver dans le document les cadres (élément

        <iframe>

    ou

        <frame>

    ) ;

  - Pour chaque cadre pourvu d’un attribut

        title

    , vérifier que son contenu est pertinent ;

  - Si c’est le cas pour chaque cadre, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.4.1.2 Name, Role, Value (A)

- Retrouver dans le document les cadres (élément

## 3. Couleurs Thématique Couleurs

- ### 3.1 Dans chaque page web, l’information ne doit pas être donnée uniquement par la couleur. Cette règle est-elle respectée ? Critère 3.1

  #### 3.1.1

  Pour chaque mot ou ensemble de mots dont la mise en couleur est porteuse d’information, l’information ne doit pas être donnée uniquement par la couleur. Cette règle est-elle respectée ? Test 3.1.1

  - Retrouver dans le document les informations données par la couleur dans un mot ou un ensemble de mots ;
  - Pour chacune de ces informations, vérifier qu’il existe un autre moyen de récupérer cette information (présence d’un attribut title, d’une icône ou d’un effet graphique de forme ou de position, un effet typographique…) ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 3.1.2

  Pour chaque indication de couleur donnée par un texte, l’information ne doit pas être donnée uniquement par la couleur. Cette règle est-elle respectée ? Test 3.1.2

  - Retrouver dans le document les informations données par la couleur dans un texte ;
  - Pour chacune de ces informations, vérifier qu’il existe un autre moyen de récupérer cette information (présence d’un attribut title, d’une icône ou d’un effet graphique de forme ou de position, un effet typographique…) ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 3.1.3

  Pour chaque image véhiculant une information, l’information ne doit pas être donnée uniquement par la couleur. Cette règle est-elle respectée ? Test 3.1.3

  - Retrouver dans le document les informations données par la couleur dans une image ;
  - Pour chacune de ces informations, vérifier qu’il existe un autre moyen de récupérer cette information (présence d’un attribut title, d’une icône ou d’un effet graphique de forme ou de position, un effet typographique…) ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 3.1.4

  Pour chaque propriété CSS déterminant une couleur et véhiculant une information, l’information ne doit pas être donnée uniquement par la couleur. Cette règle est-elle respectée ? Test 3.1.4

  - Retrouver dans le document les informations données par la couleur dans une propriété CSS ;
  - Pour chacune de ces informations, vérifier qu’il existe un autre moyen de récupérer cette information (présence d’un attribut title, d’une icône ou d’un effet graphique de forme ou de position, un effet typographique…) ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 3.1.5

  Pour chaque média temporel véhiculant une information, l’information ne doit pas être donnée uniquement par la couleur. Cette règle est-elle respectée ? Test 3.1.5

  - Retrouver dans le document les informations données par la couleur dans un média temporel ;
  - Pour chacune de ces informations, vérifier qu’il existe un autre moyen de récupérer cette information (présence d’un attribut title, d’une icône ou d’un effet graphique de forme ou de position, un effet typographique…) ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 3.1.6

  Pour chaque média non temporel véhiculant une information, l’information ne doit pas être donnée uniquement par la couleur. Cette règle est-elle respectée ? Test 3.1.6

  - Retrouver dans le document les informations données par la couleur dans un média non temporel ;
  - Pour chacune de ces informations, vérifier qu’il existe un autre moyen de récupérer cette information (présence d’un attribut title, d’une icône ou d’un effet graphique de forme ou de position, un effet typographique…) ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.1.4.1 Use of color (A)

- ### 3.2 Dans chaque page web, le contraste entre la couleur du texte et la couleur de son arrière-plan est-il suffisamment élevé (hors cas particuliers) ? Critère 3.2

  #### 3.2.1

  Dans chaque page web, le texte et le texte en image sans effet de graisse d’une taille restituée inférieure à 24px vérifient-ils une de ces conditions (hors cas particuliers) ? Test 3.2.1

  - Le rapport de contraste entre le texte et son arrière-plan est de 4.5:1, au moins ;
  - Un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 4.5:1, au moins.

  <!-- -->

  - Retrouver dans le document les textes et les textes en image sans effet de graisse d’une taille restituée inférieure à 24px qui pourraient poser des problèmes de contraste ;
  - Pour chacun de ces textes, vérifier que :
    - Soit le rapport de contraste entre le texte et son arrière-plan est de 4.5:1, au moins ;
    - Soit un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 4.5:1, au moins.
  - Si c’est le cas pour chaque texte, le test est validé.

  #### 3.2.2

  Dans chaque page web, le texte et le texte en image en gras d’une taille restituée inférieure à 18,5px vérifient-ils une de ces conditions (hors cas particuliers) ? Test 3.2.2

  - Le rapport de contraste entre le texte et son arrière-plan est de 4.5:1, au moins ;
  - Un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 4.5:1, au moins.

  <!-- -->

  - Retrouver dans le document les textes et les textes en image en gras d’une taille restituée inférieure à 18,5px qui pourraient poser des problèmes de contraste ;
  - Pour chacun de ces textes, vérifier que :
    - Soit le rapport de contraste entre le texte et son arrière-plan est de 4.5:1, au moins ;
    - Soit un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 4.5:1, au moins.
  - Si c’est le cas pour chaque texte, le test est validé.

  #### 3.2.3

  Dans chaque page web, le texte et le texte en image sans effet de graisse d’une taille restituée supérieure ou égale à 24px vérifient-ils une de ces conditions (hors cas particuliers) ? Test 3.2.3

  - Le rapport de contraste entre le texte et son arrière-plan est de 3:1, au moins ;
  - Un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 3:1, au moins.

  <!-- -->

  - Retrouver dans le document les textes et les textes en image sans effet de graisse d’une taille restituée supérieure ou égale à 24px qui pourraient poser des problèmes de contraste ;
  - Pour chacun de ces textes, vérifier que :
    - Soit le rapport de contraste entre le texte et son arrière-plan est de 3:1, au moins ;
    - Soit un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 3:1, au moins.
  - Si c’est le cas pour chaque texte, le test est validé.

  #### 3.2.4

  Dans chaque page web, le texte et le texte en image en gras d’une taille restituée supérieure ou égale à 18,5px vérifient-ils une de ces conditions (hors cas particuliers) ? Test 3.2.4

  - Le rapport de contraste entre le texte et son arrière-plan est de 3:1, au moins ;
  - Un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 3:1, au moins.

  <!-- -->

  - Retrouver dans le document les textes et les textes en image en gras d’une taille restituée supérieure ou égale à 18,5px qui pourraient poser des problèmes de contraste ;
  - Pour chacun de ces textes, vérifier que :
    - Soit le rapport de contraste entre le texte et son arrière-plan est de 3:1, au moins ;
    - Soit un mécanisme permet à l’utilisateur d’afficher le texte avec un rapport de contraste de 3:1, au moins.
  - Si c’est le cas pour chaque texte, le test est validé.

  #### 3.2.5

  Dans le mécanisme qui permet d’afficher un rapport de contraste conforme, le rapport de contraste entre le texte et la couleur d’arrière-plan est-il suffisamment élevé ? Test 3.2.5

  - Retrouver dans le document les mécanismes qui permettent d’afficher un rapport de contraste conforme ;
  - Pour chacun de ces mécanismes, vérifier que le rapport de contraste entre le texte et la couleur d’arrière-plan est suffisamment élevé ;
  - Si c’est le cas pour chaque mécanisme, le test est validé.

  #### Cas particuliers

  Dans ces situations, les critères sont non applicables pour ces éléments :

  - Le texte fait partie d’un logo ou d’un nom de marque d’un organisme ou d’une société ;

  - Le texte ou l’image de texte est purement décoratif ;

  - Le texte fait partie d’une image véhiculant une information mais le texte lui-même n’apporte aucune information essentielle ;

  - Le texte ou l’image de texte fait partie d’un élément d’interface sur lequel aucune action n’est possible (par exemple un bouton avec l’attribut

        disabled

    ).

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.3 Contrast (Minimum) (AA)

- ### 3.3 Dans chaque page web, les couleurs utilisées dans les composants d’interface ou les éléments graphiques porteurs d’informations sont-elles suffisamment contrastées (hors cas particuliers) ? Critère 3.3

  #### 3.3.1

  Dans chaque page web, le rapport de contraste entre les couleurs d’un composant d’interface dans ses différents états et la couleur d’arrière-plan contiguë vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 3.3.1

  - Le rapport de contraste est de 3:1, au moins ;
  - Un mécanisme permet un rapport de contraste de 3:1, au moins.

  <!-- -->

  - Retrouver dans le document les composants d’interface qui pourraient poser des problèmes de contraste ;
  - Pour chacun de ces composants, vérifier que :
    - Soit le rapport de contraste entre les couleurs du composant dans ses différents états et la couleur d’arrière-plan contiguë est de 3:1, au moins ;
    - Soit un mécanisme permet à l’utilisateur d’afficher le composant avec un rapport de contraste de 3:1, au moins.
  - Si c’est le cas pour chaque composant, le test est validé.

  #### 3.3.2

  Dans chaque page web, le rapport de contraste des différentes couleurs composant un élément graphique, lorsqu’elles sont nécessaires à sa compréhension, et la couleur d’arrière-plan contiguë, vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 3.3.2

  - Le rapport de contraste est de 3:1, au moins ;
  - Un mécanisme permet un rapport de contraste de 3:1, au moins.

  <!-- -->

  - Retrouver dans le document les éléments graphiques qui pourraient poser des problèmes de contraste ;
  - Pour chacun de ces éléments, vérifier que :
    - Soit le rapport de contraste entre les couleurs de l’élément graphique nécessaires à sa compréhension et la couleur d’arrière-plan contiguë est de 3:1, au moins ;
    - Soit un mécanisme permet à l’utilisateur d’afficher l’élément graphique avec un rapport de contraste de 3:1, au moins.
  - Si c’est le cas pour chaque composant, le test est validé.

  #### 3.3.3

  Dans chaque page web, le rapport de contraste des différentes couleurs contiguës entre elles d’un élément graphique, lorsqu’elles sont nécessaires à sa compréhension, vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 3.3.3

  - Le rapport de contraste est de 3:1, au moins ;
  - Un mécanisme permet un rapport de contraste de 3:1, au moins.

  <!-- -->

  - Retrouver dans le document les éléments graphiques qui pourraient poser des problèmes de contraste ;
  - Pour chacun de ces éléments, vérifier que :
    - Soit le rapport de contraste des différentes couleurs contiguës de l’élément graphique entre elles, lorsqu’elles sont nécessaires à sa compréhension, est de 3:1, au moins ;
    - Soit un mécanisme permet à l’utilisateur d’afficher l’élément graphique avec un rapport de contraste de 3:1, au moins.
  - Si c’est le cas pour chaque élément graphique, le test est validé.

  #### 3.3.4

  Dans le mécanisme qui permet d’afficher un rapport de contraste conforme, les couleurs du composant ou des éléments graphiques porteurs d’informations qui le composent, sont-elles suffisamment contrastées ? Test 3.3.4

  - Retrouver dans le document les mécanismes qui permettent d’afficher un rapport de contraste conforme ;
  - Pour chacun de ces mécanismes, vérifier que le rapport de contraste entre les couleurs du composant ou des éléments graphiques porteurs d’informations qui le composent est suffisamment élevé ;
  - Si c’est le cas pour chaque mécanisme, le test est validé.

  Note : le critère est non applicable dans ces situations :

  - Composant d’interface inactif (par exemple, un bouton avec un attribut

        disabled

    ) sur lequel aucune action n’est possible ;

  - Composant d’interface pour lequel l’apparence est gérée par les styles natifs du navigateur sans aucune modification par l’auteur (par exemple, le style au focus natif dans Chrome ou Firefox) ;

  - Composant d’interface pour lequel la couleur n’est pas nécessaire pour identifier le composant ou son état (par exemple, un groupe de liens faisant office de navigation dont la position dans la page, la taille et la couleur du texte permettent de comprendre qu’il s’agit de liens même si la couleur du soulignement des liens avec le fond blanc n’a pas un ratio de 3:1 et que le texte lui a un ratio de 4.5:1) ;

  - Élément graphique ou parties d’élément graphique non porteur d’information ou ayant une alternative (description longue, informations identiques visibles dans la page) ;

  - Élément graphique ou parties d’élément graphique faisant partie d’un logo ou du nom de marque d’un organisme ou d’une société ;

  - Élément graphique ou parties d’élément graphique dont la présentation est essentielle à l’information véhiculée (exemple drapeaux, logotypes, photos de personnes ou de scènes, captures d’écran, diagrammes médicaux, carte de chaleurs) ;

  - Élément graphique ou parties d’élément graphique dynamiques dont le contraste au survol / focus est suffisant.

  #### Cas particuliers

  Les cas suivants sont non applicables pour ce critère :

  - Composant d’interface inactif (par exemple, un bouton avec un attribut

        disabled

    ) sur lequel aucune action n’est possible ;

  - Composant d’interface pour lequel l’apparence est gérée par les styles natifs du navigateur sans aucune modification par l’auteur (par exemple, le style au focus natif dans Chrome ou Firefox) ;

  - Composant d’interface pour lequel la couleur n’est pas nécessaire pour identifier le composant ou son état (par exemple, un groupe de liens faisant office de navigation dont la position dans la page, la taille et la couleur du texte permettent de comprendre qu’il s’agit de liens même si la couleur du soulignement des liens avec le fond blanc n’a pas un ratio de 3:1 et que le texte lui a un ratio de 4.5:1) ;

  - Élément graphique ou parties d’élément graphique non porteur d’information ou ayant une alternative (description longue, informations identiques visibles dans la page) ;

  - Élément graphique ou parties d’élément graphique faisant partie d’un logo ou du nom de marque d’un organisme ou d’une société ;

  - Élément graphique ou parties d’élément graphique dont la présentation est essentielle à l’information véhiculée (par exemple, drapeaux, logotypes, photos de personnes ou de scènes, captures d’écran, diagrammes médicaux, carte de chaleurs) ;

  - Élément graphique ou parties d’élément graphique dynamiques dont le contraste au survol / focus est suffisant.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.11 Non-text Contrast (AA)

## 4. Multimédia Thématique Multimédia

- ### 4.1 Chaque média temporel pré-enregistré a-t-il, si nécessaire, une transcription textuelle ou une audiodescription (hors cas particuliers) ? Critère 4.1

  #### 4.1.1

  Chaque média temporel pré-enregistré seulement audio, vérifie-t-il, si nécessaire, l’une de ces conditions (hors cas particuliers) ? Test 4.1.1

  - Il existe une transcription textuelle accessible via un lien ou bouton adjacent ;
  - Il existe une transcription textuelle adjacente clairement identifiable.

  <!-- -->

  - Retrouver dans le document les médias temporels (éléments

        <audio>

    ,

        <video>

    ou

        <object>

    ) seulement audio qui nécessitent une transcription textuelle ;

  - Pour chaque média temporel seulement audio, vérifier la présence d’une transcription textuelle :
    - Soit accessible au moyen d’un bouton ou d’un lien adjacent (une URL ou une ancre) ;
    - Soit adjacente clairement identifiable.

  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### 4.1.2

  Chaque média temporel pré-enregistré seulement vidéo vérifie-t-il, si nécessaire, l’une de ces conditions (hors cas particuliers) ? Test 4.1.2

  - Il existe une version alternative « audio seulement » accessible via un lien ou bouton adjacent ;
  - Il existe une version alternative « audio seulement » adjacente clairement identifiable ;
  - Il existe une transcription textuelle accessible via un lien ou bouton adjacent ;
  - Il existe une transcription textuelle adjacente clairement identifiable ;
  - Il existe une audiodescription synchronisée ;
  - Il existe une version alternative avec une audiodescription synchronisée accessible via un lien ou bouton adjacent.

  <!-- -->

  - Retrouver dans le document les médias temporels (éléments

        <video>

    ou

        <object>

    ) seulement vidéo qui nécessitent une transcription textuelle ;

  - Pour chaque média temporel seulement vidéo, vérifier la présence :
    - Soit d’une version alternative audio seulement accessible au moyen d’un lien ou bouton adjacent (une URL ou une ancre) ;
    - Soit d’une version alternative audio seulement adjacente ;
    - Soit d’une transcription textuelle accessible au moyen d’un bouton ou d’un lien adjacent (une URL ou une ancre) ;
    - Soit d’une transcription textuelle adjacente clairement identifiable ;
    - Soit d’une audiodescription synchronisée ;
    - Soit d’une version alternative avec une audiodescription synchronisée accessible au moyen d’un bouton ou d’un lien adjacent (une URL ou une ancre).

  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### 4.1.3

  Chaque média temporel synchronisé pré-enregistré vérifie-t-il, si nécessaire, une de ces conditions (hors cas particuliers) ? Test 4.1.3

  - Il existe une transcription textuelle accessible via un lien ou bouton adjacent ;
  - Il existe une transcription textuelle adjacente clairement identifiable ;
  - Il existe une audiodescription synchronisée ;
  - Il existe une version alternative avec une audiodescription synchronisée accessible via un lien ou bouton adjacent.

  <!-- -->

  - Retrouver dans le document les médias temporels (éléments

        <video>

    ou

        <object>

    ) synchronisés qui nécessitent une transcription textuelle ;

  - Pour chaque média temporel synchronisé, vérifier la présence :
    - Soit d’une transcription textuelle accessible au moyen d’un lien ou bouton adjacent (une URL ou une ancre) ;
    - Soit d’une transcription textuelle adjacente clairement identifiable ;
    - Soit d’une audiodescription synchronisée ;
    - Soit d’une version alternative avec une audiodescription synchronisée accessible au moyen d’un bouton ou d’un lien adjacent (une URL ou une ancre).

  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particulier lorsque :

  - Le média temporel est utilisé à des fins décoratives (c’est-à-dire qu’il n’apporte aucune information) ;
  - Le média temporel est lui-même une alternative à un contenu de la page (une vidéo en langue des signes ou la vocalisation d’un texte, par exemple) ;
  - Le média temporel est utilisé pour accéder à une version agrandie ;
  - Le média temporel est utilisé comme un CAPTCHA ;
  - Le média temporel fait partie d’un test qui deviendrait inutile si la transcription textuelle, les sous-titres synchronisés ou l’audiodescription étaient communiqués ;
  - Pour les services de l’État, les collectivités territoriales et leurs établissements : si le média temporel a été publié entre le 23 septembre 2019 et le 23 septembre 2020 sur un site internet, intranet ou extranet créé depuis le 23 septembre 2018, il est exempté de l’obligation d’accessibilité ;
  - Pour les personnes de droit privé mentionnées aux 2° à 4° du I de l’article 47 de la loi du 11 février 2005 : si le média temporel a été publié avant le 23 septembre 2020, il est exempté de l’obligation d’accessibilité.

  Dans ces situations, le critère est non applicable.

  Ce cas particulier s’applique également aux critères 4.2, 4.3, 4.5.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.2.1 Audio-only and Video-only (Prerecorded) (A)
  - 9.1.2.3 Audio Description or Media Alternative (Prerecorded) (A)

- ### 4.2 Pour chaque média temporel pré-enregistré ayant une transcription textuelle ou une audiodescription synchronisée, celles-ci sont-elles pertinentes (hors cas particuliers) ? Critère 4.2

  #### 4.2.1

  Pour chaque média temporel pré-enregistré seulement audio, ayant une transcription textuelle, celle-ci est-elle pertinente (hors cas particuliers) ? Test 4.2.1

  - Retrouver dans le document les médias temporels pré-enregistrés seulement audio qui possèdent une transcription textuelle ;
  - Pour chaque média temporel seulement audio, vérifier que transcription textuelle est pertinente ;
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### 4.2.2

  Chaque média temporel pré-enregistré seulement vidéo vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 4.2.2

  - La transcription textuelle est pertinente ;
  - L’audiodescription synchronisée est pertinente ;
  - L’audiodescription synchronisée de la version alternative est pertinente ;
  - La version alternative audio seulement est pertinente.

  <!-- -->

  - Retrouver dans le document les médias temporels pré-enregistrés seulement vidéo qui possèdent une transcription textuelle ;
  - Pour chaque média temporel seulement vidéo, vérifier la pertinence :
    - Soit de la transcription textuelle ;
    - Soit de l’audiodescription synchronisée ;
    - Soit de l’audiodescription synchronisée de la version alternative ;
    - Soit de la version alternative audio seulement.
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### 4.2.3

  Chaque média temporel synchronisé pré-enregistré vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 4.2.3

  - La transcription textuelle est pertinente ;
  - L’audiodescription synchronisée est pertinente ;
  - L’audiodescription synchronisée de la version alternative est pertinente.

  <!-- -->

  - Retrouver dans le document les médias temporels pré-enregistrés synchronisés ;
  - Pour chaque média temporel synchronisé, vérifier la pertinence :
    - Soit de la transcription textuelle ;
    - Soit de l’audiodescription synchronisée ;
    - Soit de l’audiodescription synchronisée de la version alternative.
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### Cas particuliers

  Voir cas particuliers critère 4.1.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.2.1 Audio-only and Video-only (Prerecorded) (A)
  - 9.1.2.3 Audio Description or Media Alternative (Prerecorded) (A)

- ### 4.3 Chaque média temporel synchronisé pré-enregistré a-t-il, si nécessaire, des sous-titres synchronisés (hors cas particuliers) ? Critère 4.3

  #### 4.3.1

  Chaque média temporel synchronisé pré-enregistré vérifie-t-il, si nécessaire, l’une de ces conditions (hors cas particuliers) ? Test 4.3.1

  - Le média temporel synchronisé possède des sous-titres synchronisés ;
  - Il existe une version alternative possédant des sous-titres synchronisés accessible via un lien ou bouton adjacent.

  <!-- -->

  - Retrouver dans le document les médias temporels pré-enregistrés synchronisés ;
  - Pour chaque média temporel synchronisé, vérifier la présence :
    - Soit de sous-titres synchronisés ;
    - Soit d’une version alternative possédant des sous-titres synchronisés accessible au moyen d’un lien ou d’un bouton adjacent.
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### 4.3.2

  Pour chaque média temporel synchronisé pré-enregistré possédant des sous-titres synchronisés diffusés via une balise

      <track>

  , la balise

      <track>

  possède-t-elle un attribut

      kind="captions"

  ? Test 4.3.2

  - Retrouver dans le document les médias temporels synchronisés possédant des sous-titres synchronisés au moyen d’un élément

        <track>

    ;

  - Pour chaque média temporel synchronisé, vérifier que la balise

        <track>

    possède un attribut

        kind="caption"

    ;

  - Si c’est le cas pour chaque média temporel synchronisé, le test est validé.

  #### Cas particuliers

  Voir cas particuliers critère 4.1.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.2.2 Captions (Prerecorded) (A)

- ### 4.4 Pour chaque média temporel synchronisé pré-enregistré ayant des sous-titres synchronisés, ces sous-titres sont-ils pertinents ? Critère 4.4

  #### 4.4.1

  Pour chaque média temporel synchronisé pré-enregistré ayant des sous-titres synchronisés, ces sous-titres sont-ils pertinents ? Test 4.4.1

  - Retrouver dans le document les médias temporels synchronisés possédant des sous-titres synchronisés ;
  - Pour chaque média temporel synchronisé, vérifier que les sous-titres sont :
    - Pertinents (toutes les informations sonores importantes sont présentes, les dialogues notamment) ;
    - Et correctement synchronisés.
  - Si c’est le cas pour chaque média temporel synchronisé, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.2.2 Captions (Prerecorded) (A)

- ### 4.5 Chaque média temporel pré-enregistré a-t-il, si nécessaire, une audiodescription synchronisée (hors cas particuliers) ? Critère 4.5

  #### 4.5.1

  Chaque média temporel pré-enregistré seulement vidéo vérifie-t-il, si nécessaire, une de ces conditions (hors cas particuliers) ? Test 4.5.1

  - Il existe une audiodescription synchronisée ;
  - Il existe une version alternative avec une audiodescription synchronisée.

  <!-- -->

  - Retrouver dans le document les médias temporels pré-enregistrés seulement vidéo qui nécessitent une audiodescription ;
  - Pour chaque média temporel seulement vidéo, vérifier la présence :
    - Soit d’une audiodescription synchronisée ;
    - Soit d’une version alternative avec une audiodescription synchronisée accessible au moyen d’un bouton ou d’un lien adjacent (une URL ou une ancre).
  - Si c’est le cas pour chaque média temporel seulement vidéo, le test est validé.

  #### 4.5.2

  Chaque média temporel synchronisé pré-enregistré vérifie-t-il, si nécessaire, une de ces conditions (hors cas particuliers) ? Test 4.5.2

  - Il existe une audiodescription synchronisée ;
  - Il existe une version alternative avec une audiodescription synchronisée.

  <!-- -->

  - Retrouver dans le document les médias temporels pré-enregistrés synchronisés qui nécessitent une audiodescription ;
  - Pour chaque média temporel synchronisé, vérifier la présence :
    - Soit d’une audiodescription synchronisée ;
    - Soit d’une version alternative avec une audiodescription synchronisée accessible au moyen d’un bouton ou d’un lien adjacent (une URL ou une ancre).
  - Si c’est le cas pour chaque média temporel synchronisé, le test est validé.

  #### Cas particuliers

  Voir cas particuliers critère 4.1.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.2.5 Audio Description (Prerecorded) (AA)

- ### 4.6 Pour chaque média temporel pré-enregistré ayant une audiodescription synchronisée, celle-ci est-elle pertinente ? Critère 4.6

  #### 4.6.1

  Pour chaque média temporel pré-enregistré seulement vidéo ayant une audiodescription synchronisée, celle-ci est-elle pertinente ? Test 4.6.1

  - Retrouver dans le document les médias temporels seulement vidéo qui possèdent une audiodescription ;
  - Pour chaque média temporel, vérifier que l’audiodescription synchronisée est pertinente (toutes les informations visuelles qu’il est possible de vocaliser dans les blancs de la bande son principale sont présentes, les textes incrustés notamment) ;
  - Si c’est le cas pour chaque média temporel seulement vidéo, le test est validé.

  #### 4.6.2

  Pour chaque média temporel synchronisé ayant une audiodescription synchronisée, celle-ci est-elle pertinente ? Test 4.6.2

  - Retrouver dans le document les médias temporels synchronisés qui possèdent une audiodescription ;
  - Pour chaque média temporel, vérifier que l’audiodescription synchronisée est pertinente (toutes les informations visuelles qu’il est possible de vocaliser dans les blancs de la bande son principale sont présentes, les textes incrustés notamment) ;
  - Si c’est le cas pour chaque média temporel synchronisé, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.2.5 Audio Description (Prerecorded) (AA)

- ### 4.7 Chaque média temporel est-il clairement identifiable (hors cas particuliers) ? Critère 4.7

  #### 4.7.1

  Pour chaque média temporel seulement son, seulement vidéo ou synchronisé, le contenu textuel adjacent permet-il d’identifier clairement le média temporel (hors cas particuliers) ? Test 4.7.1

  - Retrouver dans le document les médias temporels pré-enregistrés seulement vidéo, audio ou synchronisés ;
  - Pour chaque média temporel, vérifier que :
    - Un passage de texte (un titre ou un paragraphe, par exemple) qui précède ou suit immédiatement le média temporel, permet de l’identifier ;
    - Et le passage de texte est situé à l’extérieur du lecteur de contenu multimédia si ce dernier fait appel à la technologie Flash.
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particulier lorsque le média temporel est utilisé à des fins décoratives (c’est-à-dire qu’il n’apporte aucune information). Dans cette situation, le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- ### 4.8 Chaque média non temporel a-t-il, si nécessaire, une alternative (hors cas particuliers) ? Critère 4.8

  #### 4.8.1

  Chaque média non temporel vérifie-t-il, si nécessaire, une de ces conditions (hors cas particuliers) ? Test 4.8.1

  - Un lien ou un bouton adjacent, clairement identifiable, permet d’accéder à une page contenant une alternative ;
  - Un lien ou un bouton adjacent, clairement identifiable, permet d’accéder à une alternative dans la page.

  <!-- -->

  - Retrouver dans le document les médias non temporels ;
  - Pour chaque média non temporel, vérifier qu’un lien ou un bouton adjacent, clairement identifiable :
    - Soit contient l’adresse (url) d’une page contenant une alternative ;
    - Soit permet d’accéder à une alternative dans la page.
  - Si c’est le cas pour chaque média non temporel, le test est validé.

  #### 4.8.2

  Chaque média non temporel associé à une alternative vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 4.8.2

  - La page référencée par le lien ou bouton adjacent est accessible ;
  - L’alternative dans la page, référencée par le lien ou bouton adjacent, est accessible.

  <!-- -->

  - Retrouver dans le document les médias non temporels associés à une alternative ;
  - Pour chaque média non temporel, vérifier que :
    - La page référencée par le lien ou le bouton adjacent est accessible ;
    - L’alternative dans la page, référencée par le lien ou le bouton adjacent, est accessible.
  - Si c’est le cas pour chaque média non temporel, le test est validé.

  Note : le critère est non applicable dans les situations où :

  - Le média non temporel est utilisé à des fins décoratives (c’est-à-dire qu’il n’apporte aucune information) ;
  - Le média non temporel est diffusé dans un environnement maîtrisé ;
  - Le média non temporel est inséré via JavaScript en vérifiant la présence et la version du plug-in, en remplacement d’un contenu alternatif déjà présent.

  #### Cas particuliers

  Il existe une gestion de cas particulier lorsque :

  - Le média non temporel est utilisé à des fins décoratives (c’est-à-dire qu’il n’apporte aucune information) ;
  - Le média non temporel est diffusé dans un environnement maîtrisé ;
  - Le média non temporel est inséré via JavaScript en vérifiant la présence et la version du plug-in, en remplacement d’un contenu alternatif déjà présent.

  Dans ces situations, le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- ### 4.9 Pour chaque média non temporel ayant une alternative, cette alternative est-elle pertinente ? Critère 4.9

  #### 4.9.1

  Pour chaque média non temporel ayant une alternative, cette alternative permet-elle d’accéder au même contenu et à des fonctionnalités similaires ? Test 4.9.1

  - Retrouver dans le document les médias non temporels associés à une alternative ;
  - Pour chaque média non temporel, vérifier que l’alternative est pertinente (elle permet d’accéder au même contenu et à des fonctionnalités similaires) ;
  - Si c’est le cas pour chaque média non temporel, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- ### 4.10 Chaque son déclenché automatiquement est-il contrôlable par l’utilisateur ? Critère 4.10

  #### 4.10.1

  Chaque séquence sonore déclenchée automatiquement via une balise

      <object>

  ,

      <video>

  ,

      <audio>

  ,

      <embed>

  ,

      <bgsound>

  ou un code JavaScript vérifie-t-elle une de ces conditions ? Test 4.10.1

  - La séquence sonore a une durée inférieure ou égale à 3 secondes ;
  - La séquence sonore peut être stoppée sur action de l’utilisateur ;
  - Le volume de la séquence sonore peut être contrôlé par l’utilisateur indépendamment du contrôle de volume du système.

  <!-- -->

  - Au chargement du document, si un son se déclenche automatiquement, vérifier que :
    - Soit la séquence sonore a une durée inférieure ou égale à 3 secondes ;
    - Soit un dispositif (un bouton par exemple), sur l’élément ayant déclenché le son (voir note), ou dans la page, permet de le stopper ;
    - Soit le volume de la séquence peut être contrôlé par l’utilisateur, indépendamment du contrôle de volume du système.
  - Si c’est le cas, le test est validé.

  Note : les éléments suivants sont susceptibles de déclencher des sons au chargement de la page : éléments

      <audio>

  ,

      <video>

  ,

      <object>

  ,

      <embed>

  ,

      <bgsound>

  ou un code JavaScript (utilisation de la Web Audio API, par exemple).

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.2 Audio Control (A)

- ### 4.11 La consultation de chaque média temporel est-elle, si nécessaire, contrôlable par le clavier et tout dispositif de pointage ? Critère 4.11

  #### 4.11.1

  Chaque média temporel a-t-il, si nécessaire, les fonctionnalités de contrôle de sa consultation ? Test 4.11.1

  - Retrouver dans le document les médias temporels ;
  - Pour chaque média temporel, vérifier la présence des fonctionnalités obligatoires de contrôle de la consultation :
    - Au minimum : lecture, pause ou stop ;
    - Si le média a du son, il doit avoir une fonctionnalité d’activation / désactivation du son ;
    - Si le média a des sous-titres, il doit avoir une fonctionnalité de contrôle de l’apparition/disparition des sous-titres ;
    - Si le média a une audiodescription, il doit avoir une fonctionnalité de contrôle de l’apparition/disparition de l’audiodescription.
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### 4.11.2

  Pour chaque média temporel, chaque fonctionnalité vérifie-t-elle une de ces conditions ? Test 4.11.2

  - La fonctionnalité est accessible par le clavier et tout dispositif de pointage ;
  - Une fonctionnalité accessible par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.

  <!-- -->

  - Retrouver dans le document les médias temporels pourvus de fonctionnalités de contrôle ;
  - Pour chaque média temporel, vérifier que :
    - Soit la fonctionnalité est accessible par le clavier et tout dispositif de pointage ;
    - Soit une fonctionnalité accessible par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### 4.11.3

  Pour chaque média temporel, chaque fonctionnalité vérifie-t-elle une de ces conditions ? Test 4.11.3

  - La fonctionnalité est activable par le clavier et tout dispositif de pointage ;
  - Une fonctionnalité activable par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.

  <!-- -->

  - Retrouver dans le document les médias temporels pourvus de fonctionnalités de contrôle ;
  - Pour chaque média temporel, vérifier que :
    - Soit la fonctionnalité est activable par le clavier et tout dispositif de pointage ;
    - Soit une fonctionnalité activable par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.
  - Si c’est le cas pour chaque média temporel, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.1.1 Keyboard (A)
  - 9.2.1.2 No Keyboard Trap (A)

- ### 4.12 La consultation de chaque média non temporel est-elle contrôlable par le clavier et tout dispositif de pointage ? Critère 4.12

  #### 4.12.1

  Pour chaque média non temporel, chaque fonctionnalité vérifie-t-elle une de ces conditions ? Test 4.12.1

  - La fonctionnalité est accessible par le clavier et tout dispositif de pointage ;
  - Une fonctionnalité accessible par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.

  <!-- -->

  - Retrouver dans le document les médias non temporels pourvus de fonctionnalités de contrôle ;
  - Pour chaque média non temporel, vérifier que :
    - Soit la fonctionnalité est accessible par le clavier et tout dispositif de pointage ;
    - Soit une fonctionnalité accessible par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.
  - Si c’est le cas pour chaque média non temporel, le test est validé.

  #### 4.12.2

  Pour chaque média non temporel, chaque fonctionnalité vérifie-t-elle une de ces conditions ? Test 4.12.2

  - La fonctionnalité est activable par le clavier et tout dispositif de pointage ;
  - Une fonctionnalité activable par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.

  <!-- -->

  - Retrouver dans le document les médias non temporels pourvus de fonctionnalités de contrôle ;
  - Pour chaque média non temporel, vérifier que :
    - Soit la fonctionnalité est activable par le clavier et tout dispositif de pointage ;
    - Soit une fonctionnalité activable par le clavier et tout dispositif de pointage permettant de réaliser la même action est présente dans la page.
  - Si c’est le cas pour chaque média non temporel, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.1.1 Keyboard (A)
  - 9.2.1.2 No Keyboard Trap (A)

- ### 4.13 Chaque média temporel et non temporel est-il compatible avec les technologies d’assistance (hors cas particuliers) ? Critère 4.13

  #### 4.13.1

  Chaque média temporel et non temporel vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 4.13.1

  - Le nom, le rôle, la valeur, le paramétrage et les changements d’états des composants d’interfaces sont accessibles aux technologies d’assistance via une API d’accessibilité ;
  - Une alternative compatible avec une API d’accessibilité permet d’accéder aux mêmes fonctionnalités.

  <!-- -->

  - Retrouver dans le document les médias temporels et non temporels ;
  - Pour chaque média, vérifier que :
    - Soit le nom, le rôle, la valeur, le paramétrage et les changements d’états des composants d’interfaces sont accessibles aux technologies d’assistance via une API d’accessibilité (par exemple, les zones mises à jour dynamiquement dans un lecteur vidéo sont correctement restituées) ;
    - Soit une alternative compatible avec une API d’accessibilité permet d’accéder aux mêmes fonctionnalités.
  - Si c’est le cas pour chaque média temporel ou non temporel, le test est validé.

  #### 4.13.2

  Chaque média temporel et non temporel qui possède une alternative compatible avec les technologies d’assistance, vérifie-t-il une de ces conditions ? Test 4.13.2

  - L’alternative est adjacente au média temporel ou non temporel ;
  - L’alternative est accessible via un lien ou bouton adjacent ;
  - Un mécanisme permet de remplacer le média temporel ou non temporel par son alternative.

  <!-- -->

  - Retrouver dans le document les médias temporels et non temporels qui possèdent une alternative compatible avec les technologies d’assistance ;
  - Pour chaque média, vérifier que :
    - Soit l’alternative est adjacente au média temporel ou non temporel ;
    - Soit l’alternative est accessible au moyen d’un lien ou d’un bouton adjacent ;
    - Soit un mécanisme permet de remplacer le média temporel ou non temporel par son alternative.
  - Si c’est le cas pour chaque média temporel ou non temporel, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particulier lorsque le média temporel ou non temporel est utilisé à des fins décoratives (c’est-à-dire qu’il n’apporte aucune information).

  Dans ces situations, le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.4.1.2 Name, role, Value (A)

## 5. Tableaux Thématique Tableaux

- ### 5.1 Chaque tableau de données complexe a-t-il un résumé ? Critère 5.1

  #### 5.1.1

  Pour chaque tableau de données complexe, un résumé est-il disponible ? Test 5.1.1

  - Retrouver dans le document les tableaux de données complexes (tableau de données - élément

        <table>

    ou élément pourvu d’un attribut WAI-ARIA

        role="table"

    \- contenant des en-têtes qui ne sont pas répartis uniquement sur la première ligne et/ou la première colonne de la grille ou dont la portée n’est pas valable pour l’ensemble de la colonne ou de la ligne) ;

  - Pour chaque tableau de données complexe, vérifier qu’un passage de texte permettant de comprendre la nature et la structure du tableau, est présent :
    - Soit dans l’élément

          <caption>

      ;

    - Soit dans l’attribut

          summary

      de l’élément

          <table>

      (dans les versions de HTML et de XHTML antérieures à HTML 5) ;

    - Soit dans un passage de texte lié au tableau avec l’attribut 

          aria-describedby

      .

  - Soit dans l’élément

  - Si c’est le cas pour chaque tableau de données complexe, le test est validé.

  #### Notes techniques

  La spécification HTML propose plusieurs méthodes pour lier un résumé à un tableau (tableau lié à un passage de texte avec l’attribut

      aria-describedby

  , tableau groupé dans un élément

      figure

  avec un résumé présent dans un élément

      figcaption

  ou un élément

      p

  , résumé présent dans un élément

      details

  contenu dans l’élément

      caption

  ). Ces méthodes n’ont pas un support suffisant pour être utilisées actuellement.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- Retrouver dans le document les tableaux de données complexes (tableau de données - élément

- ### 5.2 Pour chaque tableau de données complexe ayant un résumé, celui-ci est-il pertinent ? Critère 5.2

  #### 5.2.1

  Pour chaque tableau de données complexe ayant un résumé, celui-ci est-il pertinent ? Test 5.2.1

  - Retrouver dans le document les résumés de tableaux de données complexes (tels que déterminés par le test 5.1.1) ;
  - Pour chaque résumé, vérifier que son contenu est pertinent ;
  - Si c’est le cas pour chaque résumé de tableaux de données complexes, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- ### 5.3 Pour chaque tableau de mise en forme, le contenu linéarisé reste-t-il compréhensible ? Critère 5.3

  #### 5.3.1

  Chaque tableau de mise en forme vérifie-t-il ces conditions ? Test 5.3.1

  - Le contenu linéarisé reste compréhensible ;

  - La balise

        <table>

    possède un attribut

        role="presentation"

    .

  <!-- -->

  - Retrouver dans le document les tableaux de mise en forme ;
  - Pour chaque tableau de mise en forme, vérifier que :
    - L’ordre d’accès aux cellules est cohérent avec le contenu ;

    - L’élément

          <table>

      est pourvu d’un attribut WAI-ARIA

          role="presentation"

      .
  - Si c’est le cas pour chaque tableau de mise en forme, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.2 Meaningful Sequence (A)
  - 9.4.1.2 Name, Role, Value (A)

- ### 5.4 Pour chaque tableau de données ayant un titre, le titre est-il correctement associé au tableau de données ? Critère 5.4

  #### 5.4.1

  Pour chaque tableau de données ayant un titre, le titre est-il correctement associé au tableau de données ? Test 5.4.1

  - Retrouver dans le document les tableaux de données pourvus d’un titre ;
  - Pour chaque titre, vérifier qu’il est fourni au moyen :
    - Soit d’un élément

          <caption>

      ;

    - Soit d’un attribut

          title

      ;

    - Soit d’un attribut WAI-ARIA

          aria-label

      ;

    - Soit d’un attribut WAI-ARIA

          aria-labelledby

      référençant un passage de texte.
  - Soit d’un élément
  - Si c’est le cas pour chaque titre de tableau de données, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- ### 5.5 Pour chaque tableau de données ayant un titre, celui-ci est-il pertinent ? Critère 5.5

  #### 5.5.1

  Pour chaque tableau de données ayant un titre, ce titre permet-il d’identifier le contenu du tableau de données de manière claire et concise ? Test 5.5.1

  - Retrouver dans le document les tableaux de données pourvus d’un titre ;
  - Pour chaque titre, vérifier qu’il est pertinent ;
  - Si c’est le cas pour chaque titre de tableau de données, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- ### 5.6 Pour chaque tableau de données, chaque en-tête de colonne et chaque en-tête de ligne sont-ils correctement déclarés ? Critère 5.6

  #### 5.6.1

  Pour chaque tableau de données, chaque en-tête de colonne s’appliquant à la totalité de la colonne vérifie-t-il une de ces conditions ? Test 5.6.1

  - L’en-tête de colonnes est structuré au moyen d’une balise

        <th>

    ;

  - L’en-tête de colonnes est structuré au moyen d’une balise pourvue d’un attribut WAI-ARIA

        role="columnheader"

    .

  <!-- -->

  - Retrouver dans le document les tableaux de données ;
  - Pour chaque en-tête de colonnes s’appliquant à la totalité de la colonne, vérifier que l’en-tête de colonne est structuré au moyen :
    - Soit d’un élément

          <th>

      ;

    - Soit d’un élément pourvu d’un attribut WAI-ARIA

          role="columnheader"

      .
  - Soit d’un élément
  - Si c’est le cas pour chaque en-tête de colonne s’appliquant à la totalité de la colonne, le test est validé.

  #### 5.6.2

  Pour chaque tableau de données, chaque en-tête de ligne s’appliquant à la totalité de la ligne vérifie-t-il une de ces conditions ? Test 5.6.2

  - L’en-tête de lignes est structuré au moyen d’une balise

        <th>

    ;

  - L’en-tête de lignes est structuré au moyen d’une balise pourvue d’un attribut WAI-ARIA

        role="rowheader"

    .

  <!-- -->

  - Retrouver dans le document les tableaux de données ;
  - Pour chaque en-tête de ligne s’appliquant à la totalité de la ligne, vérifier que l’en-tête de ligne est structuré au moyen :
    - Soit d’un élément

          <th>

      ;

    - Soit d’un élément pourvu d’un attribut WAI-ARIA

          role="rowheader"

      .
  - Soit d’un élément
  - Si c’est le cas pour chaque en-tête de ligne s’appliquant à la totalité de la ligne, le test est validé.

  #### 5.6.3

  Pour chaque tableau de données, chaque en-tête ne s’appliquant pas à la totalité de la ligne ou de la colonne est-il structuré au moyen d’une balise

      <th>

  ? Test 5.6.3

  - Retrouver dans le document les tableaux de données ;

  - Pour chaque en-tête ne s’appliquant pas à la totalité de la ligne ou de la colonne, vérifier que l’en-tête de ligne est structuré au moyen d’un élément

        <th>

    ;

  - Si c’est le cas pour chaque en-tête ne s’appliquant pas à la totalité de la ligne ou de la colonne, le test est validé.

  #### 5.6.4

  Pour chaque tableau de données, chaque cellule associée à plusieurs en-têtes est-elle structurée au moyen d’une balise

      <td>

  ou

      <th>

  ? Test 5.6.4

  - Retrouver dans le document les tableaux de données ;

  - Pour chaque cellule associée à plusieurs en-têtes est-elle structurée au moyen d’une balise

        <th>

    ou

        <td>

    ;

  - Si c’est le cas pour chaque en-tête ne s’appliquant pas à la totalité de la ligne ou de la colonne, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- L’en-tête de colonnes est structuré au moyen d’une balise

- ### 5.7 Pour chaque tableau de données, la technique appropriée permettant d’associer chaque cellule avec ses en-têtes est-elle utilisée (hors cas particuliers) ? Critère 5.7

  #### 5.7.1

  Pour chaque contenu de balise

      <th>

  s’appliquant à la totalité de la ligne ou de la colonne, la balise

      <th>

  respecte-t-elle une de ces conditions (hors cas particuliers) ? Test 5.7.1

  - La balise

        <th>

    possède un attribut

        id

    unique ;

  - La balise

        <th>

    possède un attribut

        scope

    ;

  - La balise

        <th>

    possède un attribut WAI-ARIA

        role="rowheader"

    ou

        role="columnheader"

    .

  <!-- -->

  - Retrouver dans le document les tableaux de données ;

  - Pour chaque en-tête (élément

        <th>

    ) s’appliquant à la totalité de la ligne ou de la colonne, vérifier que l’élément

        <th>

    possède :

    - Soit un attribut

          id

      unique ;

    - Soit un attribut scope ;

    - Soit un attribut WAI-ARIA

          role="rowheader"

      ou

          "columnheader"

      .

  - Soit un attribut

  - Si c’est le cas pour chaque en-tête s’appliquant à la totalité de la ligne ou de la colonne, le test est validé.

  #### 5.7.2

  Pour chaque contenu de balise

      <th>

  s’appliquant à la totalité de la ligne ou de la colonne et possédant un attribut

      scope

  , la balise

      <th>

  vérifie-t-elle une de ces conditions ? Test 5.7.2

  - La balise

        <th>

    possède un attribut

        scope

    avec la valeur

        "row"

    pour les en-têtes de ligne ;

  - La balise

        <th>

    possède un attribut

        scope

    avec la valeur

        "col"

    pour les en-têtes de colonne.

  <!-- -->

  - Retrouver dans le document les tableaux de données ;

  - Pour chaque en-tête (élément

        <th>

    ) s’appliquant à la totalité de la ligne ou de la colonne et pourvu d’un attribut

        scope

    , vérifier que l’attribut

        scope

    possède :

    - Soit une valeur

          "row"

      pour les en-têtes de ligne ;

    - Soit une valeur

          "col"

      pour les en-têtes de colonne.

  - Soit une valeur

  - Si c’est le cas pour chaque en-tête s’appliquant à la totalité de la ligne ou de la colonne et pourvu d’un attribut

        scope

    , le test est validé.

  #### 5.7.3

  Pour chaque contenu de balise

      <th>

  ne s’appliquant pas à la totalité de la ligne ou de la colonne, la balise

      <th>

  vérifie-t-elle ces conditions ? Test 5.7.3

  - La balise

        <th>

    ne possède pas d’attribut

        scope

    ;

  - La balise

        <th>

    ne possède pas d’attribut WAI-ARIA

        role="rowheader"

    ou

        role="columnheader"

    ;

  - La balise

        <th>

    possède un attribut

        id

    unique.

  <!-- -->

  - Retrouver dans le document les tableaux de données ;

  - Pour chaque en-tête (élément

        <th>

    ) ne s’appliquant pas à la totalité de la ligne ou de la colonne, vérifier que l’élément

        <th>

    :

    - Possède un attribut

          id

      unique ;

    - Et ne possède pas d’attribut

          scope

      ;

    - Et ne possède pas d’attribut WAI-ARIA

          role="rowheader"

      ou

          "columnheader"

      .

  - Possède un attribut

  - Si c’est le cas pour chaque en-tête ne s’appliquant pas à la totalité de la ligne ou de la colonne, le test est validé.

  #### 5.7.4

  Pour chaque contenu de balise

      <td>

  ou

      <th>

  associée à un ou plusieurs en-têtes possédant un attribut

      id

  , la balise vérifie-t-elle ces conditions ? Test 5.7.4

  - La balise possède un attribut

        headers

    ;

  - L’attribut

        headers

    possède la liste des valeurs d’attribut

        id

    des en-têtes associés.

  <!-- -->

  - Retrouver dans le document les tableaux de données ;

  - Pour chaque élément

        <td>

    ou

        <th>

    associé à un ou plusieurs en-têtes possédant un attribut

        id

    , vérifier que :

    - L’élément

          <td>

      ou

          <th>

      possède un attribut

          headers

      ;

    - Et l’attribut

          headers

      possède la liste des valeurs d’attribut

          id

      des en-têtes associés.

  - L’élément

  - Si c’est le cas pour chaque élément

        <td>

    ou

        <th>

    associé à un ou plusieurs en-têtes possédant un attribut

        id

    , le test est validé.

  #### 5.7.5

  Pour chaque balise pourvue d’un attribut WAI-ARIA

      role="rowheader"

  ou

      role="columnheader"

  dont le contenu s’applique à la totalité de la ligne ou de la colonne, la balise vérifie-t-elle une de ces conditions ? Test 5.7.5

  - La balise possède un attribut WAI-ARIA

        role="rowheader"

    pour les en-têtes de ligne ;

  - La balise possède un attribut WAI-ARIA

        role="columnheader"

    pour les en-têtes de colonne.

  <!-- -->

  - Retrouver dans le document les tableaux de données ;

  - Pour chaque en-tête s’appliquant à la totalité de la ligne ou de la colonne et pourvu d’un attribut WAI-ARIA

        role="rowheader"

    ou

        "columnheader"

    , vérifier que l’élément possède :

    - Soit un attribut WAI-ARIA

          role="rowheader"

      pour les en-têtes de ligne ;

    - Soit un attribut WAI-ARIA

          role="columnheader"

      pour les en-têtes de colonne.

  - Soit un attribut WAI-ARIA

  - Si c’est le cas pour chaque en-tête s’appliquant à la totalité de la ligne ou de la colonne et pourvu d’un attribut WAI-ARIA

        role="rowheader"

    ou

        "columnheader"

    , le test est validé.

  #### Cas particuliers

  Dans le cas de tableaux de données ayant des en-têtes sur une seule ligne ou une seule colonne, les en-têtes peuvent être structurés à l’aide de balise

      <th>

  sans attribut

      scope

  .

  #### Notes techniques

  Si l’attribut

      headers

  est implémenté sur une cellule déjà reliée à un en-tête (de ligne ou de colonne) avec l’attribut

      scope

  (avec la valeur

      col

  ou

      row

  ), c’est l’en-tête ou les en-têtes référencés par l’attribut

      headers

  qui seront restitués aux technologies d’assistance. Les en-têtes reliés avec l’attribut

      scope

  seront ignorés.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- La balise

- ### 5.8 Chaque tableau de mise en forme ne doit pas utiliser d’éléments propres aux tableaux de données. Cette règle est-elle respectée ? Critère 5.8

  #### 5.8.1

  Chaque tableau de mise en forme (balise

      <table>

  ) vérifie-t-il ces conditions ? Test 5.8.1

  - Le tableau de mise en forme (balise

        <table>

    ) n’a pas d’attribut

        summary

    (sinon vide) et ne contient pas de balises

        <caption>

    ,

        <th>

    ,

        <thead>

    ,

        <tfoot>

    ou de balises ayant un attribut WAI-ARIA

        role="rowheader"

    ,

        role="columnheader"

    ;

  - Les cellules du tableau de mise en forme (balises

        <td>

    ) ne possèdent pas d’attributs

        scope

    ,

        headers

    et

        axis

    .

  <!-- -->

  - Retrouver dans le document les tableaux de mise en forme ;
  - Pour chaque tableau de mise en forme, vérifier que :
    - L’élément

          <table>

      ne possède pas d’attribut

          summary

      , d’éléments enfant

          <caption>

      ,

          <thead>

      ,

          <th>

      ,

          <tfoot>

      ou d’éléments pourvus d’un attribut WAI-ARIA

          role=“rowheader”

      ou

          role=“columnheader”

      ;

    - Les éléments

          <td>

      ne possèdent pas d’attributs

          scope

      ,

          headers

      et

          axis

      .
  - L’élément
  - Si c’est le cas pour chaque tableau de mise en forme, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- Le tableau de mise en forme (balise

## 6. Liens Thématique Liens

- ### 6.1 Chaque lien est-il explicite (hors cas particuliers) ? Critère 6.1

  #### 6.1.1

  Chaque lien texte vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 6.1.1

  - L’intitulé de lien seul permet d’en comprendre la fonction et la destination ;
  - L’intitulé de lien additionné au contexte du lien permet d’en comprendre la fonction et la destination.

  <!-- -->

  - Retrouver dans le document les liens texte ;
  - Pour chaque lien texte, vérifier que ce qui permet d’en comprendre la fonction et la destination est :
    - Soit l’intitulé du lien seul ;
    - Soit le contexte du lien.
  - Si c’est le cas pour chaque lien texte, le test est validé.

  #### 6.1.2

  Chaque lien image vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 6.1.2

  - L’intitulé de lien seul permet d’en comprendre la fonction et la destination ;
  - L’intitulé de lien additionné au contexte du lien permet d’en comprendre la fonction et la destination.

  <!-- -->

  - Retrouver dans le document les liens image (lien avec pour contenu un élément

        <img>

    ou un élément ayant l’attribut WAI-ARIA

        role="img"

    , un élément

        <area>

    possédant un attribut

        href

    , un élément

        <object>

    , un élément

        <canvas>

    ou un élément

        <svg>

    ) ;

  - Pour chaque lien image, vérifier que ce qui permet d’en comprendre la fonction et la destination est :
    - Soit l’intitulé du lien seul, fourni par l’alternative textuelle de l’image ;
    - Soit le contexte du lien.

  - Si c’est le cas pour chaque lien image, le test est validé.

  #### 6.1.3

  Chaque lien composite vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 6.1.3

  - L’intitulé de lien seul permet d’en comprendre la fonction et la destination ;
  - L’intitulé de lien additionné au contexte du lien permet d’en comprendre la fonction et la destination.

  <!-- -->

  - Retrouver dans le document les liens composites (lien composé à la fois de contenu texte et d’éléments de type image) ;
  - Pour chaque lien composite, vérifier que ce qui permet d’en comprendre la fonction et la destination est :
    - Soit l’intitulé du lien seul, fourni par la combinaison du contenu texte et de l’alternative textuelle de l’image ;
    - Soit le contexte du lien.
  - Si c’est le cas pour chaque lien composite, le test est validé.

  #### 6.1.4

  Chaque lien SVG vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 6.1.4

  - L’intitulé de lien seul permet d’en comprendre la fonction et la destination ;
  - L’intitulé de lien additionné au contexte du lien permet d’en comprendre la fonction et la destination.

  <!-- -->

  - Retrouver dans le document les liens SVG (élément

        <svg>

    qui possède un élément

        <a>

    pourvu d’un attribut

        xlink-href

    (SVG 1.1) ou

        href

    (SVG 2)) ;

  - Pour chaque lien SVG, vérifier que ce qui permet d’en comprendre la fonction et la destination est :
    - Soit l’intitulé du lien seul, fourni par le nom accessible de l’élément

          <svg>

      (résolu généralement à partir du contenu d’un élément

          <text>

      ) ;

    - Soit le contexte du lien.

  - Soit l’intitulé du lien seul, fourni par le nom accessible de l’élément

  - Si c’est le cas pour chaque lien SVG, le test est validé.

  #### 6.1.5

  Pour chaque lien ayant un intitulé visible, le nom accessible du lien contient-il au moins l’intitulé visible (hors cas particuliers) ? Test 6.1.5

  - Retrouver dans le document les liens autres que SVG dont le contenu est fourni à la fois par un intitulé visible et par le contenu soit d’un attribut title ou d’un attribut

        aria-label

    ou d’un attribut

        aria-labelledby

    ;

  - Pour chaque lien, vérifier que le contenu de l’attribut

        title

    ou de l’attribut

        aria-label

    ou de l’attribut

        aria-labelledby

    contient l’intitulé visible ;

  - Si c’est le cas pour chaque lien, le test est validé pour les liens autres que SVG.

  - Retrouver dans le document les liens SVG dont le contenu est fourni à la fois par un intitulé visible et par le contenu soit d’un attribut

        aria-labelledby

    , ou d’un attribut

        aria-label

    ou d’un élément title (enfant direct de l’élément

        <svg>

    ) ou d’un attribut x-link:title (SVG 1.1) ou d’un ou plusieurs éléments

        <text>

    ;

  - Pour chaque lien SVG, vérifier que le contenu de l’attribut

        aria-labelledby

    ou de l’attribut

        aria-label

    ou de l’élément

        <title>

    ou de l’attribut

        x-link:title

    ou d’un ou plusieurs éléments

        <text>

    contient l’intitulé visible ;

  - Si c’est le cas pour chaque lien SVG, le test est validé pour les liens SVG.

  - Si le test est validé à la fois pour les liens non SVG et pour les liens SVG, le test est globalement validé.

  Note : considérant la détermination du nom accessible, il existe deux cas particuliers et une particularité liée aux expressions mathématiques :

  - La ponctuation et les lettres majuscules présentes dans le texte de l’intitulé visible peuvent être ignorées dans le nom accessible sans porter à conséquence.
  - Si le texte de l’intitulé visible sert de symbole, il ne doit pas être interprété littéralement au niveau du nom accessible. Le nom doit exprimer la fonction véhiculée par le symbole (par exemple, “B” au niveau d’un éditeur de texte aura pour nom accessible “Mettre en gras”, le signe “\>” en fonction du contexte signifiera “Suivant” ou “Lancer la vidéo”). Le cas des symboles mathématiques fait cependant exception (voir le point ci-dessous).
  - Si l’étiquette visible représente une expression mathématique, les symboles mathématiques peuvent être repris littéralement pour servir d’étiquette au nom accessible (par exemple, “A\>B”). Il est laissé à l’utilisateur le soin d’opérer la correspondance entre l’expression et ce qu’il doit épeler compte tenu de la connaissance qu’il a du fonctionnement de son logiciel de saisie vocale (“A plus grand que B” ou “A supérieur à B”).

  #### Cas particuliers

  Il existe une gestion de cas particuliers pour les tests 6.1.1, 6.1.2, 6.1.3 et 6.1.4 lorsque le lien est ambigu pour tout le monde. Dans cette situation, où il n’est pas possible de rendre le lien explicite dans son contexte, le critère est non applicable.

  Il existe une gestion de cas particuliers pour le test 6.1.5 lorsque :

  - La ponctuation et les lettres majuscules sont présentes dans le texte de l’intitulé visible : elles peuvent être ignorées dans le nom accessible sans porter à conséquence ;
  - Le texte de l’intitulé visible sert de symbole : le texte ne doit pas être interprété littéralement au niveau du nom accessible. Le nom doit exprimer la fonction véhiculée par le symbole (par exemple, “B” au niveau d’un éditeur de texte aura pour nom accessible “Mettre en gras”, le signe “\>” en fonction du contexte signifiera “Suivant” ou “Lancer la vidéo”). Le cas des symboles mathématiques fait cependant exception (voir la note ci-dessous).

  Note : si l’étiquette visible représente une expression mathématique, les symboles mathématiques peuvent être repris littéralement pour servir d’étiquette au nom accessible (ex. : “A\>B”). Il est laissé à l’utilisateur le soin d’opérer la correspondance entre l’expression et ce qu’il doit épeler compte tenu de la connaissance qu’il a du fonctionnement de son logiciel de saisie vocale (“A plus grand que B” ou “A supérieur à B”).

  #### Notes techniques

  Lorsque l’intitulé visible est complété par une autre expression dans le nom accessible :

  - WCAG insiste sur le placement de l’intitulé visible au début du nom accessible sans toutefois réserver l’exclusivité de cet emplacement ;
  - WCAG considère comme un cas d’échec une correspondance non exacte de la chaîne de caractères de l’intitulé visible au sein du nom accessible.

  Par exemple, si l’on considère l’intitulé visible « Commander maintenant » complété dans le nom accessible par l’expression « produit X », on peut avoir les différents cas suivants :

  - « Commander maintenant produit X » est valide (bonne pratique) ;
  - « Produit X : commander maintenant » est valide ;
  - « Commander produit X maintenant » est non valide.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.2.4.4 Link Purpose (In Context) (A)
  - 9.2.5.3 Label in Name (A)

- ### 6.2 Dans chaque page web, chaque lien a-t-il un intitulé ? Critère 6.2

  #### 6.2.1

  Dans chaque page web, chaque lien a-t-il un intitulé entre

      <a>

  et

      </a>

  ? Test 6.2.1

  - Retrouver dans le document les liens quels qu’ils soient ;

  - Pour chaque lien, vérifier que le contenu de l’élément

        <a>

    (ou d’un élément pourvu d’un attribut WAI-ARIA

        role=link

    ) contient un intitulé (texte ou alternative) ;

  - Si c’est le cas pour chaque lien, le test est validé.

  #### Notes techniques

  Une ancre n’est pas un lien même si pendant longtemps l’élément

      <a>

  a servi de support à cette technique. Elle n’est donc pas concernée par le présent critère.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.2.4.4 Link Purpose (In Context) (A)

## 7. Scripts Thématique Scripts

- ### 7.1 Chaque script est-il, si nécessaire, compatible avec les technologies d’assistance ? Critère 7.1

  #### 7.1.1

  Chaque script qui génère ou contrôle un composant d’interface vérifie-t-il, si nécessaire, une de ces conditions ? Test 7.1.1

  - Le nom, le rôle, la valeur, le paramétrage et les changements d’états sont accessibles aux technologies d’assistance via une API d’accessibilité ;
  - Un composant d’interface accessible permettant d’accéder aux mêmes fonctionnalités est présent dans la page ;
  - Une alternative accessible permet d’accéder aux mêmes fonctionnalités.

  <!-- -->

  - Retrouver dans le document tous les composants d’interface générés ou contrôlés au moyen de JavaScript ;
  - Vérifier que :
    - Le composant possède un rôle cohérent avec son usage (généralement un bouton ou un lien) ;
    - Le composant possède un nom explicite ;
    - Le nom du composant est cohérent avec l’état de la fonctionnalité ou des contenus contrôlés (par exemple pour une fonctionnalité permettant d’afficher ou de masquer une zone de contenu).
  - Sinon, vérifier la présence d’un composant d’interface accessible permettant d’accéder aux mêmes fonctionnalités ;
  - Sinon, vérifier la présence d’une alternative accessible permettant d’accéder aux mêmes fonctionnalités.
  - Si c’est le cas, le test est validé.

  #### 7.1.2

  Chaque script qui génère ou contrôle un composant d’interface respecte-t-il une de ces conditions ? Test 7.1.2

  - Le composant d’interface est correctement restitué par les technologies d’assistance ;
  - Une alternative accessible permet d’accéder aux mêmes fonctionnalités.

  <!-- -->

  - Pour chacun des composants d’interface ayant validé le test 7.1.1, vérifier que le composant d’interface est correctement restitué par les technologies d’assistance ;
  - Sinon, vérifier qu’une alternative accessible au composant d’interface permet d’accéder aux mêmes fonctionnalités ;
  - Si c’est le cas, le test est validé.

  #### 7.1.3

  Chaque script qui génère ou contrôle un composant d’interface vérifie-t-il ces conditions (hors cas particuliers) ? Test 7.1.3

  - Le composant possède un nom pertinent ;
  - Le nom accessible du composant contient au moins l’intitulé visible ;
  - Le composant possède un rôle pertinent.

  <!-- -->

  - Pour chacun des composants d’interface ayant validé le test 7.1.1, vérifier que le composant d’interface possède :
    - Un nom pertinent (intitulé visible) ;
    - Un rôle pertinent.
  - Si le composant d’interface possède un nom accessible, vérifier que ce nom est pertinent et contient au moins l’intitulé visible.
  - Si c’est le cas, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particuliers pour le test 7.1.3 lorsque :

  - La ponctuation et les lettres majuscules sont présentes dans le texte de l’intitulé visible : elles peuvent être ignorées dans le nom accessible sans porter à conséquence ;
  - Le texte de l’intitulé visible sert de symbole : le texte ne doit pas être interprété littéralement au niveau du nom accessible. Le nom doit exprimer la fonction véhiculée par le symbole (par exemple, “B” au niveau d’un éditeur de texte aura pour nom accessible “Mettre en gras”, le signe “\>” en fonction du contexte signifiera “Suivant” ou “Lancer la vidéo”). Le cas des symboles mathématiques fait cependant exception (voir la note ci-dessous).

  Note : si l’étiquette visible représente une expression mathématique, les symboles mathématiques peuvent être repris littéralement pour servir d’étiquette au nom accessible (ex. : “A\>B”). Il est laissé à l’utilisateur le soin d’opérer la correspondance entre l’expression et ce qu’il doit épeler compte tenu de la connaissance qu’il a du fonctionnement de son logiciel de saisie vocale (“A plus grand que B” ou “A supérieur à B”).

  #### Notes techniques

  Le critère 7.1 implémente la notion de « compatible avec les technologies d’assistance » telle que définie par les WCAG, ainsi que le recours à WAI-ARIA pour rendre un composant ou une fonctionnalité accessible. Le bon usage de WAI-ARIA est vérifié via les tests 7.1.1, 7.1.2, 7.1.3.

  Note importante : dans un environnement HTML5, beaucoup de composants peuvent nécessiter JavaScript pour fonctionner ; en conséquence la fourniture d’une alternative à un composant JavaScript qui ne pourrait pas être rendu accessible devra bénéficier d’une méthode spécifique au composant en cause, permettant de le remplacer par une alternative accessible (et de le réactiver). Cela signifie que la désactivation de JavaScript pour l’ensemble de la page ne sera pas acceptée comme une méthode valable, à moins qu’elle ne remette pas en cause l’utilisation des autres composants.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.5.3 Label in Name (A)
  - 9.4.1.2 Name, Role, Value (A)

- ### 7.2 Pour chaque script ayant une alternative, cette alternative est-elle pertinente ? Critère 7.2

  #### 7.2.1

  Chaque script débutant par la balise

      <script>

  et ayant une alternative vérifie-t-il une de ces conditions ? Test 7.2.1

  - L’alternative entre

        <noscript>

    et

        </noscript>

    permet d’accéder à des contenus et des fonctionnalités similaires ;

  - La page affichée, lorsque JavaScript est désactivé, permet d’accéder à des contenus et des fonctionnalités similaires ;

  - La page alternative permet d’accéder à des contenus et des fonctionnalités similaires ;

  - Le langage de script côté serveur permet d’accéder à des contenus et des fonctionnalités similaires ;

  - L’alternative présente dans la page permet d’accéder à des contenus et des fonctionnalités similaires.

  <!-- -->

  - Retrouver les alternatives aux fonctionnalités JavaScript :
  - Chercher dans la page, les alternatives à un composant ou une fonctionnalité JavaScript mises à disposition.
  - Désactiver JavaScript dans le document et retrouver les alternatives proposées.
  - Pour chacune des alternatives proposées, vérifier qu’elle permet d’accéder aux mêmes contenus et à des fonctionnalités similaires.
  - Si c’est le cas, le test est validé.

  #### 7.2.2

  Chaque élément non textuel mis à jour par un script (dans la page, ou dans un cadre) et ayant une alternative vérifie-t-il ces conditions ? Test 7.2.2

  - L’alternative de l’élément non textuel est mise à jour ;
  - L’alternative mise à jour est pertinente.

  <!-- -->

  - Retrouver dans le document tous les éléments non textuels mis à jour par une fonctionnalité JavaScript.
  - Si l’élément non textuel a une alternative, vérifier que :
    - L’alternative est mise à jour lorsque le contenu non textuel est mis à jour ;
    - L’alternative mise à jour est pertinente.
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.4.1.2 Name, Role, Value (A)

- L’alternative entre

- ### 7.3 Chaque script est-il contrôlable par le clavier et par tout dispositif de pointage (hors cas particuliers) ? Critère 7.3

  #### 7.3.1

  Chaque élément possédant un gestionnaire d’événement contrôlé par un script vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 7.3.1

  - L’élément est accessible par le clavier et tout dispositif de pointage ;
  - Un élément accessible par le clavier et tout dispositif de pointage permettant de réaliser la même action est présent dans la page.

  <!-- -->

  - Retrouver dans le document, tous les éléments sur lesquels est implémenté un gestionnaire d’événements JavaScript (par exemple click, focus, mouseover, blur, keydown, touch…).
  - Vérifier que l’élément est accessible au moyen du clavier :
    - Il est atteignable avec la touche de tabulation (tab) ;
    - Si l’élément gère une action simple, il est activable au clavier avec la touche entrée (Entrée) ;
    - Si l’élément gère une action complexe, il est utilisable avec le clavier (généralement avec les touches de direction).
  - Sinon, vérifier qu’un élément accessible par le clavier permettant de réaliser la même action est présent dans la page.
  - Vérifier que l’élément est accessible par tout dispositif de pointage (souris, toucher, stylet…).
  - Sinon, vérifier qu’un élément accessible au moyen d’un dispositif de pointage et permettant de réaliser la même action est présent dans la page.
  - Si c’est le cas, le test est validé.

  #### 7.3.2

  Un script ne doit pas supprimer le focus d’un élément qui le reçoit. Cette règle est-elle respectée (hors cas particuliers) ? Test 7.3.2

  - Activer, l’un après l’autre, tous les éléments capables de recevoir le focus.
  - Vérifier que le focus n’est pas supprimé via une fonctionnalité JavaScript.
  - Si c’est le cas, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particuliers lorsque la fonctionnalité dépend de l’utilisation d’un gestionnaire d’événement sans équivalent universel ; par exemple, une application de dessin à main levée ne pourra pas être rendue contrôlable au clavier. Dans ces situations, le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.2.1.1 Keyboard (A)
  - 9.2.4.7 Focus Visible (AA)

- ### 7.4 Pour chaque script qui initie un changement de contexte, l’utilisateur est-il averti ou en a-t-il le contrôle ? Critère 7.4

  #### 7.4.1

  Chaque script qui initie un changement de contexte vérifie-t-il une de ces conditions ? Test 7.4.1

  - L’utilisateur est averti par un texte de l’action du script et du type de changement avant son déclenchement ;

  - Le changement de contexte est initié par un bouton (input de type

        submit

    ,

        button

    ou

        image

    ou balise

        <button>

    ) explicite ;

  - Le changement de contexte est initié par un lien explicite.

  <!-- -->

  - Retrouver dans le document tous les événements JavaScript qui initient un changement de contexte, par exemple :
    - Une mise à jour dynamique de champs de formulaire ;

    - L’ouverture d’une nouvelle page à l’activation d’une option d’une liste de sélection (élément

          <select>

      ) ;

    - La mise à jour, via un procédé AJAX d’une partie essentielle de la page ;

    - Le lancement automatique d’un lecteur vidéo suite à la sélection d’une playlist ;

    - La manipulation du focus ayant pour résultat de modifier la position courante de l’utilisateur dans la page.
  - Vérifier que :
    - L’utilisateur est averti par un message de l’action du script et du type de changement avant son déclenchement ;
    - Ou bien le changement de contexte est initié par un bouton (input de type submit, button ou image ou la balise button) explicite ;
    - Ou bien le changement de contexte est initié par un lien explicite.
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.2.1 On Focus (A)
  - 9.3.2.2 On Input (A)

- ### 7.5 Dans chaque page web, les messages de statut sont-ils correctement restitués par les technologies d’assistance ? Critère 7.5

  #### 7.5.1

  Chaque message de statut qui informe de la réussite, du résultat d’une action ou bien de l’état d’une application utilise-t-il l’attribut WAI-ARIA

      role="status"

  ? Test 7.5.1

  - Retrouver dans le document les messages qui valent pour message de statut.
  - Pour chacun de ces messages, déterminer la nature de l’information dont est porteur le message :
  - Si le message informe de la réussite, du résultat d’une action ou bien de l’état d’une application, vérifier que l’élément qui contient le message :
    - Soit utilise l’attribut WAI-ARIA

          role=”status”

      ;

    - Soit utilise les attributs WAI-ARIA

          aria-live=”polite”

      et

          aria-atomic=”true”

      .
  - Soit utilise l’attribut WAI-ARIA
  - Si le message présente une suggestion, ou avertit de l’existence d’une erreur, vérifier que l’élément qui contient le message :
    - Soit utilise l’attribut WAI-ARIA

          role=”alert”

      ;

    - Soit utilise les attributs

          aria-live=”assertive”

      et

          aria-atomic=”true”

      .
  - Soit utilise l’attribut WAI-ARIA
  - Si le message indique la progression d’un processus, vérifier que l’élément qui contient le message :
    - Soit utilise l’un des attributs WAI-ARIA

          role=”log”

      ,

          role=”progressbar”

      ou

          role=”status”

      ;

    - Soit utilise l’attribut WAI-ARIA

          aria-live=”polite”

      si l’intention est de signaler l’équivalent d’un

          rôle “log”

      ;

    - Soit utilise les attributs WAI-ARIA

          aria-live=”polite”

      et aria-atomic=”true si l’intention est de signaler l’équivalent d’un rôle “status”.
  - Soit utilise l’un des attributs WAI-ARIA
  - Si c’est le cas, le test est validé.

  #### 7.5.2

  Chaque message de statut qui présente une suggestion, ou avertit de l’existence d’une erreur utilise-t-il l’attribut WAI-ARIA

      role="alert"

  ? Test 7.5.2

  Tests identiques à 7.5.1

  #### 7.5.3

  Chaque message de statut qui indique la progression d’un processus utilise-t-il l’un des attributs WAI-ARIA

      role="log"

  ,

      role="progressbar"

  ou

      role="status"

  ? Test 7.5.3

  Tests identiques à 7.5.1

  #### Notes techniques

  Les rôles WAI-ARIA

      log

  ,

      status

  et

      alert

  ont implicitement une valeur d’attribut WAI-ARIA

      aria-live

  et

      aria-atomic

  . On pourra donc considérer (conformément à la spécification WAI-ARIA 1.1) que :

  - Un attribut WAI-ARIA

        aria-live="polite"

    associé à un message de statut peut valoir pour un rôle WAI-ARIA

        log

    ;

  - Un attribut WAI-ARIA

        aria-live="polite"

    et un attribut WAI-ARIA

        aria-atomic="true"

    associés à un message de statut peuvent valoir pour un rôle WAI-ARIA

        status

    ;

  - Un attribut WAI-ARIA

        aria-live="assertive"

    et un attribut WAI-ARIA

        aria-atomic="true"

    associés à un message de statut peuvent valoir pour un rôle WAI-ARIA

        alert

    .

  C’est sous réserve que la nature du message de statut satisfasse bien à la correspondance implicitement établie. Dans le cas d’un message de statut indiquant la progression d’un processus et matérialisé graphiquement par une barre de progression, un rôle WAI-ARIA

      progressbar

  explicite est nécessaire.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.4.1.3 Status Messages (AA)

## 8. Éléments obligatoires Thématique Éléments obligatoires

- ### 8.1 Chaque page web est-elle définie par un type de document ? Critère 8.1

  #### 8.1.1

  Pour chaque page web, le type de document (balise

      doctype

  ) est-il présent ? Test 8.1.1

  - Retrouver dans le document la balise DOCTYPE (par exemple

        <!DOCTYPE html>

    ) ;

  - Vérifier que :
    - La balise DOCTYPE est placée avant la balise

          <html>

      ;

    - Le type de document est valide.

  - La balise DOCTYPE est placée avant la balise

  - Si c’est le cas, le test est validé.

  #### 8.1.2

  Pour chaque page web, le type de document (balise

      doctype

  ) est-il valide ? Test 8.1.2

  Tests identiques à 8.1.1

  #### 8.1.3

  Pour chaque page web possédant une déclaration de type de document, celle-ci est-elle située avant la balise

      <html>

  dans le code source ? Test 8.1.3

  Tests identiques à 8.1.1

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.4.1.1 Parsing (A)

- Retrouver dans le document la balise DOCTYPE (par exemple

- ### 8.2 Pour chaque page web, le code source généré est-il valide selon le type de document spécifié ? Critère 8.2

  #### 8.2.1

  Pour chaque déclaration de type de document, le code source généré de la page vérifie-t-il ces conditions ? Test 8.2.1

  - Les balises, attributs et valeurs d’attributs respectent les règles d’écriture ;
  - L’imbrication des balises est conforme ;
  - L’ouverture et la fermeture des balises sont conformes ;
  - Les valeurs d’attribut id sont uniques dans la page ;
  - Les attributs ne sont pas doublés sur un même élément.

  <!-- -->

  - Dans le menu « Check », activer l’option « W3C Nu markup checker (all frames) ».
  - Dans la page de résultats, vérifier que :
    - Les balises, attributs et valeurs d’attributs respectent les règles d’écriture ;
    - L’imbrication des balises est conforme ;
    - L’ouverture et la fermeture des balises sont conformes ;
    - Les valeurs d’attribut id sont uniques dans la page ;
    - Les attributs ne sont pas doublés sur un même élément.
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.4.1.1 Parsing (A)
  - 9.4.1.2 Name, Role, Value (A)

- ### 8.3 Dans chaque page web, la langue par défaut est-elle présente ? Critère 8.3

  #### 8.3.1

  Pour chaque page web, l’indication de langue par défaut vérifie-t-elle une de ces conditions ? Test 8.3.1

  - L’indication de la langue de la page (attribut

        lang

    et/ou

        xml:lang

    ) est donnée pour l’élément

        html

    ;

  - L’indication de la langue de la page (attribut

        lang

    et/ou

        xml:lang

    ) est donnée sur chaque élément de texte ou sur l’un des éléments parents.

  <!-- -->

  - Retrouver dans le document l’indication de langue par défaut ;
  - Vérifier la présence d’une indication de langue :
    - Soit au moyen de l’attribut lang sur la balise html si le code est du HTML5 ou du HTML4 ;
    - Soit au moyen des attributs lang et xml:lang sur la balise html si le code est du XHTML 1.0 ;
    - Soit au moyen de l’attribut xml:lang sur la balise html si le code est du XHTML 1.1 ;
    - Sinon, vérifier la présence d’une indication de langue sur chaque élément de texte ou l’un de ses parents.
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.1.1 Language of Page (A)

- L’indication de la langue de la page (attribut

- ### 8.4 Pour chaque page web ayant une langue par défaut, le code de langue est-il pertinent ? Critère 8.4

  #### 8.4.1

  Pour chaque page web ayant une langue par défaut, le code de langue vérifie-t-il ces conditions ? Test 8.4.1

  - Le code de langue est valide ;
  - Le code de langue est pertinent.

  <!-- -->

  - Retrouver dans le document l’indication de langue par défaut ;
  - Vérifier la présence d’un code de langue :
    - Valide (conforme à la norme ISO 639-1 ou ISO 639-2 et suivantes) ;
    - Et pertinent (qui indique la langue principale du document).
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.1.1 Language of Page (A)

- ### 8.5 Chaque page web a-t-elle un titre de page ? Critère 8.5

  #### 8.5.1

  Chaque page web a-t-elle un titre de page (balise

      <title>

  ) ? Test 8.5.1

  Test 8.5.1

  - Retrouver dans le document le titre structuré au moyen d’un élément

        <title>

    ;

  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.2 Page Titled (A)

- Retrouver dans le document le titre structuré au moyen d’un élément

- ### 8.6 Pour chaque page web ayant un titre de page, ce titre est-il pertinent ? Critère 8.6

  #### 8.6.1

  Pour chaque page web ayant un titre de page (balise

      <title>

  ), le contenu de cette balise est-il pertinent ? Test 8.6.1

  - Retrouver dans le document le titre structuré au moyen d’un élément

        <title>

    ;

  - Vérifier si le contenu de l’élément

        <title>

    est suffisamment pertinent (il permet de retrouver la page dans l’historique de navigation ou la liste des onglets).

  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.2 Page Titled (A)

- Retrouver dans le document le titre structuré au moyen d’un élément

- ### 8.7 Dans chaque page web, chaque changement de langue est-il indiqué dans le code source (hors cas particuliers) ? Critère 8.7

  #### 8.7.1

  Dans chaque page web, chaque texte écrit dans une langue différente de la langue par défaut vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 8.7.1

  - L’indication de langue est donnée sur l’élément contenant le texte (attribut

        lang

    et/ou

        xml:lang

    ) ;

  - L’indication de langue est donnée sur un des éléments parents (attribut

        lang

    et/ou

        xml:lang

    )

  <!-- -->

  - Retrouver les passages de texte en langue étrangère, à l’exception :
    - Des noms propres ;

    - Des mots d’origine étrangère, présents dans le dictionnaire de la langue du document ;

    - Des mots d’origine étrangère et d’usage courant dont la prononciation ne provoque pas d’incompréhension.

    - Vérifier que chaque passage de texte retenu possède une indication de langue (attribut

          lang

      et/ou

          xml:lang

      sur l’élément lui-même ou l’un de ses parents).
  - Si c’est le cas, le test est validé.

  #### Cas particuliers

  Il y a une gestion de cas particuliers sur le changement de langue pour les cas suivants :

  - Nom propre, le critère est non applicable ;
  - Nom commun de langue étrangère présent dans le dictionnaire officiel de la langue (voir note 1 ci-dessous) par défaut de la page web, le critère est non applicable ;
  - Le terme de langue étrangère soumis, via un champ de formulaire et rappelé dans la page (par exemple comme indication du terme recherché dans le cas d’un moteur de recherche), le critère est non applicable ;
  - Passage de texte dont la langue ne peut pas être déterminée : le critère est non applicable ;
  - Terme ou passage de texte issus d’une langue morte ou imaginaire pour laquelle il n’existe pas d’interprétation vocale : le critère est non applicable.

  Note 1 : le dictionnaire officiel est celui recommandé par l’académie en charge de la langue en question. Pour la France, par exemple, le lien vers le dictionnaire officiel se trouve sur le site de l’Académie française à l’adresse suivante : http://www.academie-francaise.fr/le-dictionnaire/la-9e-edition. Pour toute demande auprès du service du dictionnaire de l’Académie française, utiliser le formulaire de contact du service du dictionnaire.

  Note 2 : pour les noms communs de langue étrangère, absents dans le dictionnaire officiel de la langue par défaut de la page web, et qui sont passés dans le langage commun (exemple : newsletter) : le critère est applicable, uniquement lorsque l’absence d’indication de langue peut provoquer une incompréhension pour la restitution.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.1.2 Language of Parts (AA)

- L’indication de langue est donnée sur l’élément contenant le texte (attribut

- ### 8.8 Dans chaque page web, le code de langue de chaque changement de langue est-il valide et pertinent ? Critère 8.8

  #### 8.8.1

  Pour chaque page web, le code de langue de chaque changement de langue vérifie-t-il ces conditions ? Test 8.8.1

  - Le code de langue est valide ;
  - Le code de langue est pertinent.

  <!-- -->

  - Pour chaque passage de texte validé au test 8.7.1, vérifier que :
    - L’indication de langue est valide ;
    - L’indication de langue est pertinente.
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.1.2 Language of Parts (AA)

- ### 8.9 Dans chaque page web, les balises ne doivent pas être utilisées uniquement à des fins de présentation. Cette règle est-elle respectée ? Critère 8.9

  #### 8.9.1

  Dans chaque page web les balises (à l’exception de

      <div>

  ,

      <span>

  et

      <table>

  ) ne doivent pas être utilisées uniquement à des fins de présentation. Cette règle est-elle respectée ? Test 8.9.1

  - Retrouver dans le document l’ensemble des éléments sémantiques utilisés à des fins de présentation ;
  - Pour chacun de ces éléments, vérifier que :
    - L’élément est pourvu d’un attribut

          role=“presentation”

      ;

    - L’utilisation de cet élément à des fins de présentation reste justifée.
  - L’élément est pourvu d’un attribut
  - Si c’est le cas, le test est validé.

  Note : Quelques exemples, non exhaustifs de détournement de balisage : un élément

      <div>

  utilisé comme paragraphe, un titre utilisé comme légende, un élément

      <blockquote>

  ou des paragraphes vides ou encore des espaces utilisés pour créer des effets de marges. L’utilisation d’un

      role=“presentation”

  est formellement déconseillée, mais peut toutefois se justifier dans de rares cas. Cela peut être acceptable sur un élément

      <blockquote>

  ou un paragraphe vide, mais sera considéré comme non-conforme sur un titre.

  Le cas des tableaux : à noter que ce test aborde les tableaux de présentation qui ne devraient finalement pas apparaître au sein de la thématique Tableaux.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- ### 8.10 Dans chaque page web, les changements du sens de lecture sont-ils signalés ? Critère 8.10

  #### 8.10.1

  Dans chaque page web, chaque texte dont le sens de lecture est différent du sens de lecture par défaut est contenu dans une balise possédant un attribut

      dir

  ? Test 8.10.1

  - Retrouver dans le document les passages de textes qui utilisent une langue qui se lit dans le sens inverse de la langue du document (comme l’arabe ou l’hébreu pour le français par exemple).

  - Pour chaque passage de texte, vérifier que le passage de texte est contenu dans une balise qui possède un attribut

        dir

    .

  - Si c’est le cas pour chaque passage de texte, le test est validé.

  #### 8.10.2

  Dans chaque page web, chaque changement du sens de lecture (attribut

      dir

  ) vérifie-t-il ces conditions ? Test 8.10.2

  - La valeur de l’attribut

        dir

    est conforme (

        rtl

    ou

        ltr

    ) ;

  - La valeur de l’attribut

        dir

    est pertinente.

  <!-- -->

  - Pour chaque passage de texte validé au test 8.10.1, vérifier que :
    - L’indication de sens de lecture est conforme (ltr, pour le sens « de gauche à droite » et rtl pour le sens « de droite à gauche ») ;
    - L’indication de sens de lecture est pertinente.
  - Si c’est le cas pour chaque passage de texte, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.2 Meaningful Sequence (A)

## 9. Structuration de l’information Thématique Structuration de l’information

- ### 9.1 Dans chaque page web, l’information est-elle structurée par l’utilisation appropriée de titres ? Critère 9.1

  #### 9.1.1

  Dans chaque page web, la hiérarchie entre les titres (balise

      <hx>

  ou balise possédant un attribut WAI-ARIA

      role="heading"

  associé à un attribut WAI-ARIA

      aria-level

  ) est-elle pertinente ? Test 9.1.1

  - Retrouver dans le document les titres (balise

        <hx>

    ou balise possédant un attribut WAI-ARIA

        role="heading"

    associé à un attribut WAI-ARIA

        aria-level

    ) ;

  - Vérifier que la hiérarchie entre les titres est pertinente ;

  - Si c’est le cas, le test est validé.

  #### 9.1.2

  Dans chaque page web, le contenu de chaque titre (balise

      <hx>

  ou balise possédant un attribut WAI-ARIA

      role="heading"

  associé à un attribut WAI-ARIA

      aria-level

  ) est-il pertinent ? Test 9.1.2

  - Pour chaque titre identifié au test 9.1.1, vérifier que son contenu est pertinent ;
  - Si c’est le cas pour chaque titre, le test est validé.

  #### 9.1.3

  Dans chaque page web, chaque passage de texte constituant un titre est-il structuré à l’aide d’une balise

      <hx>

  ou d’une balise possédant un attribut WAI-ARIA

      role="heading"

  associé à un attribut WAI-ARIA

      aria-level

  ? Test 9.1.3

  - Pour chaque titre identifié au test 9.1.1, vérifier que :
    - Soit il est structuré au moyen d’une balise

          <hx>

      (“x” désignant une valeur numérique comprise entre 1 et 6);

    - Soit il est structuré au moyen d’une balise possédant un attribut WAI-ARIA

          role="heading"

      et un attribut WAI-ARIA

          aria-level=x

      (“x” désignant une valeur numérique).
  - Soit il est structuré au moyen d’une balise
  - Si c’est le cas pour chaque titre, le test est validé.

  #### Notes techniques

  WAI-ARIA permet de définir des titres via le rôle

      heading

  et l’attribut

      aria-level

  (indication du niveau de titre). Bien qu’il soit préférable d’utiliser l’élément de titre natif en HTML

      <hx>

  , l’utilisation du rôle WAI-ARIA

      heading

  est compatible avec l’accessibilité.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.2.4.1 Bypass Blocks (A)
  - 9.2.4.6 Headings and Labels (AA)
  - 9.4.1.2 Name, Role, Value (A)

- Retrouver dans le document les titres (balise

- ### 9.2 Dans chaque page web, la structure du document est-elle cohérente (hors cas particuliers) ? Critère 9.2

  #### 9.2.1

  Dans chaque page web, la structure du document vérifie-t-elle ces conditions (hors cas particuliers) ? Test 9.2.1

  - La zone d’en-tête de la page est structurée via une balise

        <header>

    ;

  - Les zones de navigation principales et secondaires sont structurées via une balise

        <nav>

    ;

  - La balise

        <nav>

    est réservée à la structuration des zones de navigation principales et secondaires ;

  - La zone de contenu principal est structurée via une balise

        <main>

    ;

  - La structure du document utilise une balise

        <main>

    visible unique ;

  - La zone de pied de page est structurée via une balise

        <footer>

    .

  <!-- -->

  - Vérifier que la zone d’en-tête est structurée au moyen d’un élément

        <header>

    ;

  - Vérifier que les zones de navigation principales et secondaires sont structurées au moyen d’un élément

        <nav>

    ;

  - Vérifier que l’élément

        <nav>

    n’est pas utilisé en dehors de la structuration des zones de navigation principales et secondaires ;

  - Vérifier que la zone de contenu principal est structurée au moyen d’un élément

        <main>

    ;

  - Si le document possède plusieurs éléments

        <main>

    , vérifier qu’un seul de ces éléments est visible (les autres occurrences de l’élément sont pourvues d’un attribut

        hidden

    ) ;

  - Vérifier que la zone de pied de page est structurée au moyen d’un élément

        <footer>

    .

  - Si c’est le cas pour chaque zone de contenu, le test est validé.

  #### Cas particuliers

  Lorsque le doctype déclaré dans la page n’est pas le doctype HTML5, ce critère est non applicable.

  #### Notes techniques

  La balise

      <main>

  peut être utilisée plusieurs fois dans le même document HTML. Néanmoins, il ne peut y avoir en permanence qu’une seule balise visible et lisible par les technologies d’assistances, les autres devant disposer d’un attribut

      hidden

  ou d’un style permettant de les masquer aux technologies d’assistances. À noter cependant que l’utilisation d’un style seul restera insuffisante pour assurer l’unicité d’une balise

      <main>

  visible en cas de désactivation des feuilles de styles.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- La zone d’en-tête de la page est structurée via une balise

- ### 9.3 Dans chaque page web, chaque liste est-elle correctement structurée ? Critère 9.3

  #### 9.3.1

  Dans chaque page web, les informations regroupées visuellement sous forme de liste non ordonnée vérifient-elles une de ces conditions ? Test 9.3.1

  - La liste utilise les balises HTML

        <ul>

    et

        <li>

    ;

  - La liste utilise les attributs WAI-ARIA

        role="list"

    et

        role="listitem"

    .

  <!-- -->

  - Retrouver dans le document les éléments regroupés visuellement sous la forme d’une liste non ordonnée ;
  - Pour chaque liste, vérifier que la liste est structurée :
    - Soit au moyen des éléments

          <ul>

      et

          <li>

      ;

    - Soit au moyen d’éléments pourvus d’attributs WAI-ARIA

          role="list"

      et

          role="listitem"

      .
  - Soit au moyen des éléments
  - Si c’est le cas pour chaque liste non ordonnée, le test est validé.

  #### 9.3.2

  Dans chaque page web, les informations regroupées visuellement sous forme de liste ordonnée vérifient-elles une de ces conditions ? Test 9.3.2

  - La liste utilise les balises HTML

        <ol>

    et

        <li>

    ;

  - La liste utilise les attributs WAI-ARIA

        role="list"

    et

        role="listitem"

    .

  <!-- -->

  - Retrouver dans le document les éléments regroupés visuellement sous la forme d’une liste ordonnée ;
  - Pour chaque liste, vérifier que la liste est structurée :
    - Soit au moyen des éléments

          <ol>

      et

          <li>

      ;

    - Soit au moyen d’éléments pourvus d’attributs WAI-ARIA

          role="list"

      et

          role="listitem"

      .
  - Soit au moyen des éléments
  - Si c’est le cas pour chaque liste ordonnée, le test est validé.

  #### 9.3.3

  Dans chaque page web, les informations regroupées sous forme de liste de description utilisent-elles les balises

      <dl>

  et

      <dt>/<dd>

  ? Test 9.3.3

  - Retrouver dans le document les éléments regroupés visuellement sous la forme d’une liste de description ;

  - Pour chaque liste, vérifier que la liste est structurée au moyen des éléments

        <dl>

    ,

        <dt>

    et

        <dd>

    ;

  - Si c’est le cas pour chaque liste de description, le test est validé.

  #### Notes techniques

  Les attributs WAI-ARIA

      role="list"

  et

      role="listitem"

  peuvent nécessiter l’utilisation des attributs WAI-ARIA

      aria-setsize

  et

      aria-posinset

  dans le cas où l’ensemble de la liste n’est pas disponible via le DOM généré au moment de la consultation.

  Les attributs WAI-ARIA

      role="tree"

  ,

      role="tablist"

  ,

      role="menu"

  ,

      role="combobox"

  et

      role="listbox"

  ne sont pas équivalents à une liste HTML

      <ul>

  ou

      <ol>

  .

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- La liste utilise les balises HTML

- ### 9.4 Dans chaque page web, chaque citation est-elle correctement indiquée ? Critère 9.4

  #### 9.4.1

  Dans chaque page web, chaque citation courte utilise-t-elle une balise

      <q>

  ? Test 9.4.1

  - Retrouver dans le document les citations courtes (ou en ligne) ;

  - Pour chaque citation, vérifier que la citation est structurée au moyen d’un élément

        <q>

    ;

  - Si c’est le cas pour chaque citation courte, le test est validé.

  #### 9.4.2

  Dans chaque page web, chaque bloc de citation utilise-t-il une balise

      <blockquote>

  ? Test 9.4.2

  - Retrouver dans le document les blocs de citation ;

  - Pour chaque bloc de citation, vérifier que le bloc de citation est structuré au moyen d’un élément

        <blockquote>

    ;

  - Si c’est le cas pour chaque bloc de citation, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

## 10. Présentation de l’information Thématique Présentation de l’information

- ### 10.1 Dans le site web, des feuilles de styles sont-elles utilisées pour contrôler la présentation de l’information ? Critère 10.1

  #### 10.1.1

  Dans chaque page web, les balises servant à la présentation de l’information ne doivent pas être présentes dans le code source généré des pages. Cette règle est-elle respectée ? Test 10.1.1

  - Vérifier l’absence des éléments de présentation

        <basefont>

    ,

        <big>

    ,

        <blink>

    ,

        <center>

    ,

        <font>

    ,

        <marquee>

    ,

        <s>

    ,

        <strike>

    ,

        <tt>

    ;

  - Vérifier l’absence de l’élément

        <u>

    uniquement si le DOCTYPE du document ne correspond pas à HTML 5 ;

  - Si c’est le cas, le test est validé.

  #### 10.1.2

  Dans chaque page web, les attributs servant à la présentation de l’information ne doivent pas être présents dans le code source généré des pages. Cette règle est-elle respectée ? Test 10.1.2

  - Vérifier l’absence des attributs de présentation :

        align

    ,

        alink

    ,

        background

    ,

        bgcolor

    ,

        border

    ,

        cellpadding

    ,

        cellspacing

    ,

        char

    ,

        charoff

    ,

        clear

    ,

        color

    ,

        compact

    ,

        frameborder

    ,

        hspace

    ,

        link

    ,

        marginheight

    ,

        marginwidth

    ,

        text

    ,

        valign

    ,

        vlink

    ,

        vspace

    ,

        size

    (exception faite de l’élément

        <select>

    ),

        width

    (exception faite des éléments

        <img>

    ,

        <object>

    ,

        <embed>

    ,

        <canvas>

    et

        <svg>

    ),

        height

    (exception faite des éléments

        <img>

    ,

        <object>

    ,

        <embed>

    ,

        <canvas>

    et

        <svg>

    ) ;

  - Si c’est le cas, le test est validé.

  #### 10.1.3

  Dans chaque page web, l’utilisation des espaces vérifie-t-elle ces conditions ? Test 10.1.3

  - Les espaces ne sont pas utilisées pour séparer les lettres d’un mot ;
  - Les espaces ne sont pas utilisées pour simuler des tableaux ;
  - Les espaces ne sont pas utilisées pour simuler des colonnes de texte.

  <!-- -->

  - Désactiver les styles (CSS) du document ;
  - Vérifier l’absence d’espaces utilisées :
    - Entre les lettres d’un mot ;
    - Pour créer des effets de marges ou d’alignement ;
    - Pour simuler des tableaux ou des colonnes.
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.1.3.2 Meaningful Sequence (A)

- Vérifier l’absence des éléments de présentation

- ### 10.2 Dans chaque page web, le contenu visible porteur d’information reste-t-il présent lorsque les feuilles de styles sont désactivées ? Critère 10.2

  #### 10.2.1

  Dans chaque page web, l’information reste-t-elle présente lorsque les feuilles de styles sont désactivées ? Test 10.2.1

  - Désactiver les styles (CSS) du document ;
  - Comparer le document dépourvu de styles avec le document mis en forme ;
  - Vérifier si dans le document dépourvu de styles, les contenus visibles porteurs d’information restent présents ;
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.1.3.1 Info and Relationships (A)

- ### 10.3 Dans chaque page web, l’information reste-t-elle compréhensible lorsque les feuilles de styles sont désactivées ? Critère 10.3

  #### 10.3.1

  Dans chaque page web, l’information reste-t-elle compréhensible lorsque les feuilles de styles sont désactivées ? Test 10.3.1

  - Désactiver les styles (CSS) du document ;
  - Vérifier que l’ordre dans lequel les contenus sont implémentés ne pose pas de problème de compréhension ;
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.2 Meaningful Sequence (A)
  - 9.2.4.3 Focus Order (A)

- ### 10.4 Dans chaque page web, le texte reste-t-il lisible lorsque la taille des caractères est augmentée jusqu’à 200 %, au moins (hors cas particuliers) ? Critère 10.4

  #### 10.4.1

  Dans chaque page web, l’augmentation de la taille des caractères jusqu’à 200 %, au moins, ne doit pas provoquer de perte d’information. Cette règle est-elle respectée selon une de ces conditions (hors cas particuliers) ? Test 10.4.1

  - Lors de l’utilisation de la fonction d’agrandissement du texte du navigateur ;
  - Lors de l’utilisation des fonctions de zoom graphique du navigateur ;
  - Lors de l’utilisation d’un composant d’interface propre au site permettant d’agrandir le texte ou de zoomer.

  <!-- -->

  - Vérifier dans le document si les textes restent présents et lisibles lorsque :
    - Le zoom texte du navigateur est réglé à 200 % ;
    - Le zoom graphique du navigateur est réglé à 200 % ;
    - Les fonctionnalités de zoom personnalisées proposé par le document sont utilisés.
  - Si c’est le cas, le test est validé.

  #### 10.4.2

  Dans chaque page web, l’augmentation de la taille des caractères jusqu’à 200 %, au moins, doit être possible pour l’ensemble du texte dans la page. Cette règle est-elle respectée selon une de ces conditions (hors cas particuliers) ? Test 10.4.2

  - Lors de l’utilisation de la fonction d’agrandissement du texte du navigateur ;
  - Lors de l’utilisation des fonctions de zoom graphique du navigateur ;
  - Lors de l’utilisation d’un composant d’interface propre au site permettant d’agrandir le texte ou de zoomer.

  <!-- -->

  - Vérifier dans le document si les textes sont effectivement agrandis lorsque :
    - Le zoom texte du navigateur est réglé à 200 % ;
    - Le zoom graphique du navigateur est réglé à 200 % ;
    - Les fonctionnalités de zoom personnalisées proposé par le document sont utilisés.
  - Si c’est le cas, le test est validé.

  #### Cas particuliers

  Font exception à ce critère, les contenus pour lesquels l’utilisateur n’a pas de possibilité de personnalisation :

  - Les sous-titres incrustés dans une vidéo ;

  - Les textes en image ;

  - Le texte au sein d’une balise

        <canvas>

    .

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.4 Resize Text (AA)

- ### 10.5 Dans chaque page web, les déclarations CSS de couleurs de fond d’élément et de police sont-elles correctement utilisées ? Critère 10.5

  #### 10.5.1

  Dans chaque page web, chaque déclaration CSS de couleurs de police (

      color

  ), d’un élément susceptible de contenir du texte, est-elle accompagnée d’une déclaration de couleur de fond (

      background

  ,

      background-color

  ), au moins, héritée d’un parent ? Test 10.5.1

  - Retrouver dans le document les textes mis en couleur, à l’exception des couleurs par défaut (par exemple les liens, etc.) ;

  - Déterminer l’élément qui contient le texte et vérifier la présence d’une valeur calculée pour la propriété

        background-color

    de l’élément ;

  - Si c’est le cas, le test est validé.

  #### 10.5.2

  Dans chaque page web, chaque déclaration de couleur de fond (

      background

  ,

      background-color

  ), d’un élément susceptible de contenir du texte, est-elle accompagnée d’une déclaration de couleur de police (

      color

  ) au moins, héritée d’un parent ? Test 10.5.2

  - Retrouver dans le document les textes mis en couleur, à l’exception des couleurs par défaut (par exemple les liens, etc.) ;

  - Déterminer l’élément qui contient le texte et vérifier la présence d’une valeur calculée pour la propriété

        color

    de l’élément ;

  - Si c’est le cas, le test est validé.

  #### 10.5.3

  Dans chaque page web, chaque utilisation d’une image pour créer une couleur de fond d’un élément susceptible de contenir du texte, via CSS (

      background

  ,

      background-image

  ), est-elle accompagnée d’une déclaration de couleur de fond (

      background

  ,

      background-color

  ), au moins, héritée d’un parent ? Test 10.5.3

  - Retrouver dans le document les textes dont l’arrière-plan est constitué d’une image (propriété background-image) ;
  - Déterminer l’élément qui contient le texte et vérifier que si l’image d’arrière-plan est absente, le texte reste lisible ;
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.3 Contrast (Minimum) (AA)

- ### 10.6 Dans chaque page web, chaque lien dont la nature n’est pas évidente est-il visible par rapport au texte environnant ? Critère 10.6

  #### 10.6.1

  Dans chaque page web, chaque lien texte signalé uniquement par la couleur, et dont la nature n’est pas évidente, vérifie-t-il ces conditions ? Test 10.6.1

  - La couleur du lien a un rapport de contraste supérieur ou égal à 3:1 par rapport au texte environnant ;
  - Le lien dispose d’une indication visuelle au survol autre qu’un changement de couleur ;
  - Le lien dispose d’une indication visuelle au focus autre qu’un changement de couleur.

  <!-- -->

  - Retrouver dans le document les éléments de type lien (élément

        <a>

    ou élément pourvu d’un attribut WAI-ARIA

        role="link"

    ) ;

  - Pour chaque élément de type lien, s’il peut être confondu avec un texte normal lorsqu’il est signalé uniquement par la couleur, vérifier que le contraste entre la couleur de police du lien et la couleur de police du texte environnant est de 3:1, au moins ;

  - Cette vérification doit être faite pour les différents états du lien s’ils sont présentés au moyen d’une couleur différente : l’état non visité, l’état visité, l’état activé, l’état au survol et l’état à la prise de focus ;

  - Si c’est le cas pour chaque élément de type lien, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.1 Use of Color (A)

- ### 10.7 Dans chaque page web, pour chaque élément recevant le focus, la prise de focus est-elle visible ? Critère 10.7

  #### 10.7.1

  Pour chaque élément recevant le focus, la prise de focus vérifie-t-elle une de ces conditions ? Test 10.7.1

  - Le style du focus natif du navigateur n’est pas supprimé ou dégradé ;
  - Un style du focus défini par l’auteur est visible.

  <!-- -->

  - Retrouver dans le document les éléments susceptibles de recevoir le focus (les éléments d’interface tels que les liens ou les contrôles de formulaire, ainsi que tout élément pourvu d’un attribut

        tabindex

    d’une valeur égale ou supérieure à 1) ;

  - Pour chaque élément susceptible de recevoir le focus, vérifier que l’indication visuelle de la prise de focus est présente (en agissant sur le contour ou le fond ou les deux) et est suffisamment contrastée (ratio de contraste égal ou supérieur à 3:1) ;

  - Si c’est le cas pour chaque élément susceptible de recevoir le focus, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.1 Use of Color (A)
  - 9.2.4.7 Focus Visible (AA)

- ### 10.8 Pour chaque page web, les contenus cachés ont-ils vocation à être ignorés par les technologies d’assistance ? Critère 10.8

  #### 10.8.1

  Dans chaque page web, chaque contenu caché vérifie-t-il une de ces conditions ? Test 10.8.1

  - Le contenu caché a vocation à être ignoré par les technologies d’assistance ;
  - Le contenu caché n’a pas vocation à être ignoré par les technologies d’assistance et est rendu restituable par les technologies d’assistance suite à une action de l’utilisateur réalisable au clavier ou par tout dispositif de pointage sur un élément précédent le contenu caché ou suite à un repositionnement du focus dessus.

  <!-- -->

  - Retrouver les contenus cachés (éléments pourvus de l’attribut hidden ou de l’attribut WAI-ARIA aria-hidden, ou bien d’une classe ou d’un ensemble de styles CSS susceptibles de masquer le contenu).
  - Pour chaque contenu caché, vérifier que :
    - Soit le contenu caché a vocation à être ignoré par les technologies d’assistance (un élément statistique de visites par exemple) ;
    - Soit le contenu caché n’a pas vocation à être ignoré par les technologies d’assistance, et dans ce cas il est rendu restituable par les technologies d’assistance au moyen :
      - Soit d’une action de l’utilisateur réalisable au clavier ou par tout dispositif de pointage sur un élément précédent le contenu caché ;
      - Soit d’une fonction de programmation qui repositionne le focus sur le contenu.
  - Si c’est le cas pour chaque contenu caché, le test est validé.

  #### Notes techniques

  WAI-ARIA propose un attribut

      aria-hidden

  (

      true

  ou

      false

  ) qui permet d’inhiber la restitution d’un contenu en direction des technologies d’assistance, sans action sur sa visibilité en direction des agents utilisateurs : un contenu avec

      aria-hidden="true"

  ne sera donc plus vocalisable, mais restera visible.

  Sauf si le contenu contrôlé par

      aria-hidden

  n’a pas vocation à être restitué par les technologies d’assistance, la valeur de l’attribut

      aria-hidden

  doit être cohérente avec l’état affiché ou masqué du contenu à l’écran.

  La spécification HTML5 propose un attribut

      hidden

  qui permet de rendre indisponible (quand l’attribut

      hidden

  est présent) un contenu dans le DOM généré (de manière similaire au

      type="hidden"

  sur un contrôle de formulaire).

  Il est possible d’avoir des situations où un contenu contrôlé par

      hidden

  ou

      aria-hidden

  se trouve momentanément dans un état incohérent avec le statut affiché ou masqué du contenu, par exemple si l’on désire rendre disponible un élément, mais que son affichage à l’écran reste dépendant d’une action ultérieure. Dans ce cas, c’est l’état final du contenu qui doit être considéré.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.2 Meaningful Sequence (A)
  - 9.4.1.2 Name, Role, Value (A)

- ### 10.9 Dans chaque page web, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle respectée ? Critère 10.9

  #### 10.9.1

  Dans chaque page web, pour chaque texte ou ensemble de textes, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle respectée ? Test 10.9.1

  - Retrouver dans le document les informations d’un texte données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier qu’il existe un autre moyen de récupérer cette information ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 10.9.2

  Dans chaque page web, pour chaque image ou ensemble d’images, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle respectée ? Test 10.9.2

  - Retrouver dans le document les informations d’une image données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier qu’il existe un autre moyen de récupérer cette information ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 10.9.3

  Dans chaque page web, pour chaque média temporel, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle respectée ? Test 10.9.3

  - Retrouver dans le document les informations d’un média temporel données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier qu’il existe un autre moyen de récupérer cette information ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### 10.9.4

  Dans chaque page web, pour chaque média non temporel, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle respectée ? Test 10.9.4

  - Retrouver dans le document les informations d’un média non temporel données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier qu’il existe un autre moyen de récupérer cette information ;
  - Si c’est le cas pour chaque information, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.3 Sensory Characteristics (A)
  - 9.1.4.1 Use of Color (A)

- ### 10.10 Dans chaque page web, l’information ne doit pas être donnée par la forme, taille ou position uniquement. Cette règle est-elle implémentée de façon pertinente ? Critère 10.10

  #### 10.10.1

  Dans chaque page web, pour chaque texte ou ensemble de textes, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle implémentée de façon pertinente ? Test 10.10.1

  - Retrouver dans le document les informations d’un texte données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier que le moyen alternatif de récupérer cette information est pertinent, c’est-à-dire qu’il permet de transmettre l’information dans tous les contextes de consultation et pour tous les utilisateurs.
  - Si c’est le cas pour chaque information, le test est validé.

  #### 10.10.2

  Dans chaque page web, pour chaque image ou ensemble d’images, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle implémentée de façon pertinente ? Test 10.10.2

  - Retrouver dans le document les informations d’une image données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier que le moyen alternatif de récupérer cette information est pertinent, c’est-à-dire qu’il permet de transmettre l’information dans tous les contextes de consultation et pour tous les utilisateurs.
  - Si c’est le cas pour chaque information, le test est validé.

  #### 10.10.3

  Dans chaque page web, pour chaque média temporel, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle implémentée de façon pertinente ? Test 10.10.3

  - Retrouver dans le document les informations d’un média temporel données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier que le moyen alternatif de récupérer cette information est pertinent, c’est-à-dire qu’il permet de transmettre l’information dans tous les contextes de consultation et pour tous les utilisateurs.
  - Si c’est le cas pour chaque information, le test est validé.

  #### 10.10.4

  Dans chaque page web, pour chaque média non temporel, l’information ne doit pas être donnée uniquement par la forme, taille ou position. Cette règle est-elle implémentée de façon pertinente ? Test 10.10.4

  - Retrouver dans le document les informations d’un média non temporel données par la forme, la taille ou la position ;
  - Pour chaque information donnée par la forme, la taille ou la position, vérifier que le moyen alternatif de récupérer cette information est pertinent, c’est-à-dire qu’il permet de transmettre l’information dans tous les contextes de consultation et pour tous les utilisateurs.
  - Si c’est le cas pour chaque information, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.3 Sensory Characteristics (A)
  - 9.1.4.1 Use of Color (A)

- ### 10.11 Pour chaque page web, les contenus peuvent-ils être présentés sans perte d’information ou de fonctionnalité et sans avoir recours soit à un défilement vertical pour une fenêtre ayant une hauteur de 256 px, soit à un défilement horizontal pour une fenêtre ayant une largeur de 320 px (hors cas particuliers) ? Critère 10.11

  #### 10.11.1

  Pour chaque page web, lorsque le contenu dont le sens de lecture est horizontal est affiché dans une fenêtre réduite à une largeur de 320 px, l’ensemble des informations et des fonctionnalités sont-elles disponibles sans aucun défilement horizontal (hors cas particuliers) ? Test 10.11.1

  - Retrouver dans le document si son contenu est conçu pour défiler verticalement (le sens de lecture du texte est horizontal), les informations et fonctionnalités ;
  - Réduire la fenêtre d’affichage à une largeur de 320 px et vérifier que les informations et les fonctionnalités restent disponibles sans aucun défilement horizontal ;
  - Si c’est le cas, le test est validé.

  #### 10.11.2

  Pour chaque page web, lorsque le contenu dont le sens de lecture est vertical est affiché dans une fenêtre réduite à une hauteur de 256 px, l’ensemble des informations et des fonctionnalités sont-elles disponibles sans aucun défilement vertical (hors cas particuliers) ? Test 10.11.2

  - Retrouver dans le document si son contenu est conçu pour défiler horizontalement (le sens de lecture du texte est vertical), les informations et fonctionnalités ;
  - Réduire la fenêtre d’affichage à une hauteur de 256 px et vérifier que les informations et les fonctionnalités restent disponibles sans aucun défilement vertical ;
  - Si c’est le cas, le test est validé.

  #### Cas particuliers

  L'objectif de ce critère est de garantir un défilement dans une unique direction pour une lecture facilitée selon le sens de l'écriture.

  Font exception à ce critère, les contenus dont l'agencement requiert deux dimensions pour être compris ou utilisés comme :

  - Les images, les graphiques ou les vidéos ;
  - Les jeux (jeux de plateforme, par exemple) ;
  - Les présentations (type diaporama, par exemple) ;
  - Les tableaux de données ;
  - Les interfaces où il est nécessaire d'avoir un ascenseur horizontal lors de la manipulation de l'interface.

  Note : la majorité des navigateurs sur les systèmes d'exploitation sur mobile (Android, iOS) ne gère pas correctement la redistribution en cas de zoom. Dans ce contexte, le critère sera considéré comme non applicable sur ces environnements.

  #### Note technique

  Lorsqu'il est ici question de pixel, il s'agit du pixel CSS tel que défini par le W3C https://www.w3.org/TR/css3-values/

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.10 Reflow (AA)

- ### 10.12 Dans chaque page web, les propriétés d’espacement du texte peuvent-elles être redéfinies par l’utilisateur sans perte de contenu ou de fonctionnalité (hors cas particuliers) ? Critère 10.12

  #### 10.12.1

  Dans chaque page web, le texte reste-t-il lisible lorsque l’affichage est modifié selon ces conditions (hors cas particuliers) ? Test 10.12.1

  - L’espacement entre les lignes (

        line-height

    ) est augmenté jusqu’à 1,5 fois la taille de la police ;

  - L’espacement suivant les paragraphes (balise

        <p>

    ) est augmenté jusqu’à 2 fois la taille de la police ;

  - L’espacement des lettres (

        letter-spacing

    ) est augmenté jusqu’à 0,12 fois la taille de la police ;

  - L’espacement des mots (

        word-spacing

    ) est augmenté jusqu’à 0,16 fois la taille de la police.

  <!-- -->

  - Modifier les styles du document en donnant :
    - Une valeur de 1.5 à la propriété

          line-height

      de tous les éléments du document ;

    - Une valeur de 2em à la propriété

          margin-bottom

      des éléments

          <p>

      ;

    - Une valeur de 0.12em à la propriété

          letter-spacing

      de tous les éléments du document ;

    - Une valeur de 0.16em à la propriété

          word-spacing

      de tous les éléments du document ;
  - Une valeur de 1.5 à la propriété
  - Pour chaque passage de texte, vérifier qu’il reste lisible, à l’exception :
    - Des sous-titres directement intégrés à une vidéo ;

    - Des images texte ;

    - Des textes au sein d’une balise

          <canvas>

      .
  - Si c’est le cas pour chaque passage de texte, le test est validé.

  Note : une implémentation de ces règles de modification est disponible dans les ressources du critère de succès WCAG 1.4.12 (https://github.com/alastc/adaptation-scripts/blob/master/scripts/text-adaptation.js).

  #### Cas particuliers

  Font exception à ce critère, les contenus pour lesquels l’utilisateur n’a pas de possibilité de personnalisation :

  - Les sous-titres directement intégrés à une vidéo ;

  - Les images texte ;

  - Le texte au sein d’une balise

        <canvas>

    .

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.12 Text Spacing (AA)

- L’espacement entre les lignes (

- ### 10.13 Dans chaque page web, les contenus additionnels apparaissant à la prise de focus ou au survol d’un composant d’interface sont-ils contrôlables par l’utilisateur (hors cas particuliers) ? Critère 10.13

  #### 10.13.1

  Chaque contenu additionnel devenant visible à la prise de focus ou au survol d’un composant d’interface peut-il être masqué par une action de l’utilisateur sans déplacer le focus ou le pointeur de la souris (hors cas particuliers) ? Test 10.13.1

  - Retrouver dans le document les contenus additionnels devenant visible à la prise de focus ou au survol d’un composant d’interface, à l’exception :
    - Des contenus additionnels contrôlés par l’agent utilisateur (par exemple, les infobulles associées à l’attribut

          title

      ou à la validation native d’un formulaire ;

    - Des contenus additionnels devenant visibles par une activation de l’utilisateur (par exemple, une fenêtre de dialogue).
  - Des contenus additionnels contrôlés par l’agent utilisateur (par exemple, les infobulles associées à l’attribut
  - Pour chaque contenu additionnel, vérifier que :
    - Soit le contenu additionnel est positionné de façon à ce qu’il ne gêne pas la consultation des autres contenus informatifs sur lesquels il viendrait se superposer (y compris le composant d’interface qui a déclenché son apparition), quelles que soient les conditions de consultation (y compris lors de l’utilisation d’un mécanisme de zoom) ;
    - Soit un mécanisme (au clavier) permet de faire disparaître le contenu additionnel (par exemple, la touche Echap).
  - Si c’est le cas pour chaque contenu additionnel, le test est validé.

  #### 10.13.2

  Chaque contenu additionnel qui apparait au survol d’un composant d’interface peut-il être survolé par le pointeur de la souris sans disparaître (hors cas particuliers) ? Test 10.13.2

  - Retrouver dans le document les contenus additionnels devenant visible au survol d’un composant d’interface, à l’exception :
    - Des contenus additionnels contrôlés par l’agent utilisateur (par exemple, les infobulles associées à l’attribut title ou à la validation native d’un formulaire) ;
    - Des contenus additionnels devenant visibles par une activation de l’utilisateur (par exemple, une fenêtre de dialogue).
  - Pour chaque contenu additionnel, vérifier qu’il peut être survolé par le pointeur de la souris sans disparaître ;
  - Si c’est le cas pour chaque contenu additionnel, le test est validé.

  #### 10.13.3

  Chaque contenu additionnel qui apparaît à la prise de focus ou au survol d’un composant d’interface vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 10.13.3

  - Le contenu additionnel reste visible jusqu’à ce que l’utilisateur retire le pointeur souris ou le focus du contenu additionnel et du composant d’interface ayant déclenché son apparition ;
  - Le contenu additionnel reste visible jusqu’à ce que l’utilisateur déclenche une action masquant ce contenu sans déplacer le focus ou le pointeur de la souris du composant d’interface ayant déclenché son apparition ;
  - Le contenu additionnel reste visible jusqu’à ce qu’il ne soit plus valide.

  <!-- -->

  - Retrouver dans le document les contenus additionnels devenant visible à la prise de focus ou au survol d’un composant d’interface, à l’exception :
    - Des contenus additionnels contrôlés par l’agent utilisateur (par exemple, les infobulles associées à l’attribut

          title

      ou à la validation native d’un formulaire) ;

    - Des contenus additionnels devenant visibles par une activation de l’utilisateur (par exemple, une fenêtre de dialogue).
  - Des contenus additionnels contrôlés par l’agent utilisateur (par exemple, les infobulles associées à l’attribut
  - Pour chaque contenu additionnel, vérifier qu’il reste visible :
    - Jusqu’à ce que l’utilisateur retire le pointeur souris ou le focus du contenu additionnel ou du composant d’interface ayant déclenché son apparition ;
    - Jusqu’à ce l’utilisateur déclenche le mécanisme prévu pour faire disparaître le contenu additionnel ;
    - Jusqu’à ce que l’information proposée par le contenu additionnel ne soit plus valide (par exemple un contenu additionnel signalant l’état “occupé” du composant d’interface que l’utilisateur souhaite activer ou encore un message d’erreur signalé sous la forme d’un contenu additionnel tant que l’utilisateur n’a pas rectifié sa saisie).
  - Si c’est le cas pour chaque contenu additionnel, le test est validé.

  #### Cas particuliers

  Lorsque le contenu additionnel est contrôlé par l’agent utilisateur (par exemple, attribut

      title

  ou validation native de formulaire) ou correspond à une fenêtre modale conforme au motif de conception WAI-ARIA

      dialog

  , le critère 10.13 est non applicable.

  Lorsque le contenu additionnel ne masque ou ne remplace aucun contenu porteur d’information, le test 10.13.1 est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.4.13 Content on Hover or Focus (AA)

- Retrouver dans le document les contenus additionnels devenant visible à la prise de focus ou au survol d’un composant d’interface, à l’exception :

- ### 10.14 Dans chaque page web, les contenus additionnels apparaissant via les styles CSS uniquement peuvent-ils être rendus visibles au clavier et par tout dispositif de pointage ? Critère 10.14

  #### 10.14.1

  Dans chaque page web, les contenus additionnels apparaissant au survol d’un composant d’interface via les styles CSS respectent-ils si nécessaire une de ces conditions ? Test 10.14.1

  - Les contenus additionnels apparaissent également à l’activation du composant via le clavier et tout dispositif de pointage ;
  - Les contenus additionnels apparaissent également à la prise de focus du composant ;
  - Les contenus additionnels apparaissent également par le biais de l’activation ou de la prise de focus d’un autre composant.

  <!-- -->

  - Retrouver dans le document les contenus additionnels devenant visible au survol d’un composant d’interface au moyen d’un mécanisme CSS (

        pseudo-classe :hover

    ) ;

  - Pour chaque contenu additionnel, vérifier que les contenus additionnels apparaissent également :
    - À l’activation du composant au moyen du clavier ou de tout autre dispositif de pointage ;
    - À la prise de focus du composant ;
    - À l’activation ou à la prise de focus d’un autre composant.

  - Si c’est le cas pour chaque contenu additionnel, le test est validé.

  #### 10.14.2

  Dans chaque page web, les contenus additionnels apparaissant au focus d’un composant d’interface via les styles CSS respectent-ils si nécessaire une de ces conditions ? Test 10.14.2

  - Les contenus additionnels apparaissent également à l’activation du composant via le clavier et tout dispositif de pointage ;
  - Les contenus additionnels apparaissent également au survol du composant ;
  - Les contenus additionnels apparaissent également par le biais de l’activation ou du survol d’un autre composant.

  <!-- -->

  - Retrouver dans le document les contenus additionnels devenant visible à la prise de focus d’un composant d’interface au moyen d’un mécanisme CSS (

        pseudo-classe :focus

    ) ;

  - Pour chaque contenu additionnel, vérifier que les contenus additionnels apparaissent également :

  <!-- -->

  - À l’activation du composant au moyen du clavier ou de tout autre dispositif de pointage ;
  - Au survol du composant ;
  - À l’activation ou du survol d’un autre composant.

  <!-- -->

  - Si c’est le cas pour chaque contenu additionnel, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.1.1 Keyboard (A)

## 11. Formulaires Thématique Formulaires

- ### 11.1 Chaque champ de formulaire a-t-il une étiquette ? Critère 11.1

  #### 11.1.1

  Chaque champ de formulaire vérifie-t-il une de ces conditions ? Test 11.1.1

  - Le champ de formulaire possède un attribut WAI-ARIA

        aria-labelledby

    référençant un passage de texte identifié ;

  - Le champ de formulaire possède un attribut WAI-ARIA

        aria-label

    ;

  - Une balise

        <label>

    ayant un attribut

        for

    est associée au champ de formulaire ;

  - Le champ de formulaire possède un attribut

        title

    ;

  - Un bouton adjacent au champ de formulaire lui fournit une étiquette visible et un élément

        <label>

    visuellement caché ou un attribut WAI-ARIA

        aria-label

    ,

        aria-labelledby

    ou

        title

    lui fournit un nom accessible.

  <!-- -->

  - Retrouver dans le document les champs de formulaire ;
  - Pour chaque champ de formulaire, vérifier que le champ de formulaire :
    - Possède un attribut WAI-ARIA

          aria-labelledby

      référençant un passage de texte identifié ;

    - Possède un attribut WAI-ARIA

          aria-label

      ;

    - Est associé à un élément

          <label>

      ayant un attribut

          for

      ;

    - Possède un attribut

          title

      ;

    - Un bouton adjacent au champ de formulaire lui fournit une étiquette visible et un élément

          <label>

      visuellement caché ou un attribut WAI-ARIA

          aria-label

      ,

          aria-labelledby

      ou

          title

      lui fournit un nom accessible.
  - Possède un attribut WAI-ARIA
  - Si c’est le cas pour champ de formulaire, le test est validé.

  #### 11.1.2

  Chaque champ de formulaire associé à une balise

      <label>

  ayant un attribut

      for

  , vérifie-t-il ces conditions ? Test 11.1.2

  - Le champ de formulaire possède un attribut

        id

    ;

  - La valeur de l’attribut

        for

    est égale à la valeur de l’attribut

        id

    du champ de formulaire associé.

  <!-- -->

  - Retrouver dans le document les champs de formulaire associé à un élément

        <label>

    ;

  - Pour chaque champ de formulaire, vérifier que :
    - Le champ de formulaire possède un attribut

          id

      ;

    - La valeur de l’attribut

          for

      de l’élément

          <label>

      est égale à la valeur de l’attribut

          id

      .

  - Le champ de formulaire possède un attribut

  - Si c’est le cas pour champ de formulaire, le test est validé.

  #### 11.1.3

  Chaque champ de formulaire ayant une étiquette dont le contenu n’est pas visible ou à proximité (masqué,

      aria-label

  ) ou qui n’est pas accolé au champ (

      aria-labelledby

  ), vérifie-t-il une de ses conditions ? Test 11.1.3

  - Le champ de formulaire possède un attribut

        title

    dont le contenu permet de comprendre la nature de la saisie attendue ;

  - Le champ de formulaire est accompagné d’un passage de texte accolé au champ qui devient visible à la prise de focus permettant de comprendre la nature de la saisie attendue ;

  - Le champ de formulaire est accompagné d’un passage de texte visible accolé au champ permettant de comprendre la nature de la saisie attendue.

  <!-- -->

  - Retrouver dans le document les champs de formulaire dont l’étiquette n’est pas visible ou à proximité (masquée, utilisation de l’attribut aria-label) ou n’est pas accolée au champ (utilisation de l’attribut

        aria-labelledby

    ) ;

  - Pour chaque champ de formulaire, vérifier que le champ de formulaire :
    - soit possède un attribut

          title

      dont le contenu permet de comprendre la nature de la saisie attendue ;

    - est accompagné d’un passage de texte accolé au champ qui devient visible à la prise de focus permettant de comprendre la nature de la saisie attendue ;

    - est accompagné d’un passage de texte visible accolé au champ permettant de comprendre la nature de la saisie attendue.

  - soit possède un attribut

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.2.4.6 Headings and Labels (AA)
  - 9.3.3.2 Labels or Instructions (A)
  - 9.4.1.2 Name, Role, Value (A)

- Le champ de formulaire possède un attribut WAI-ARIA

- ### 11.2 Chaque étiquette associée à un champ de formulaire est-elle pertinente (hors cas particuliers) ? Critère 11.2

  #### 11.2.1

  Chaque balise

      <label>

  permet-elle de connaître la fonction exacte du champ de formulaire auquel elle est associée ? Test 11.2.1

  - Retrouver dans le document les champs de formulaire dont l’étiquette est fournie par un élément

        <label>

    ;

  - Pour chaque champ de formulaire, vérifier que le contenu de l’élément est pertinent ;

  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.2.2

  Chaque attribut

      title

  permet-il de connaître la fonction exacte du champ de formulaire auquel il est associé ? Test 11.2.2

  - Retrouver dans le document les champs de formulaire dont l’étiquette est fournie par un attribut

        title

    ;

  - Pour chaque champ de formulaire, vérifier que le contenu de l’attribut est pertinent ;

  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.2.3

  Chaque étiquette implémentée via l’attribut WAI-ARIA

      aria-label

  permet-elle de connaître la fonction exacte du champ de formulaire auquel elle est associée ? Test 11.2.3

  - Retrouver dans le document les champs de formulaire dont l’étiquette est fournie par un attribut WAI-ARIA

        aria-label

    ;

  - Pour chaque champ de formulaire, vérifier que le contenu de l’attribut est pertinent ;

  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.2.4

  Chaque passage de texte associé via l’attribut WAI-ARIA

      aria-labelledby

  permet-il de connaître la fonction exacte du champ de formulaire auquel il est associé ? Test 11.2.4

  - Retrouver dans le document les champs de formulaire dont l’étiquette est fournie par un attribut WAI-ARIA

        aria-labelledby

    ;

  - Pour chaque champ de formulaire, vérifier que le contenu du passage de texte référencé est pertinent ;

  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.2.5

  Chaque champ de formulaire ayant un intitulé visible vérifie-t-il ces conditions (hors cas particuliers) ? Test 11.2.5

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    du champ de formulaire contient au moins l’intitulé visible ;

  - S’il est présent, le passage de texte lié au champ de formulaire via un attribut WAI-ARIA

        aria-labelledby

    contient au moins l’intitulé visible ;

  - S’il est présent, le contenu de l’attribut

        title

    du champ de formulaire contient au moins l’intitulé visible ;

  - S’il est présent le contenu de la balise

        <label>

    associé au champ de formulaire contient au moins l’intitulé visible.

  <!-- -->

  - Retrouver dans le document les champs de formulaire dont l’étiquette est fournie à la fois par un intitulé visible et par le contenu soit d’un élément

        <label>

    , soit d’un attribut

        title

    ou d’un attribut

        aria-label

    ou d’un attribut

        aria-labelledby

    ;

  - Pour chaque champ de formulaire, vérifier que le contenu de l’élément

        <label>

    ou de l’attribut

        title

    ou de l’attribut

        aria-label

    ou de l’attribut

        aria-labelledby

    contient l’intitulé visible ;

  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.2.6

  Chaque bouton adjacent au champ de formulaire qui fournit une étiquette visible permet-il de connaître la fonction exacte du champ de formulaire auquel il est associé ? Test 11.2.6

  - Retrouver dans le document les champs de formulaire dont l’étiquette visible est fournie par un bouton adjacent ;
  - Pour chaque champ de formulaire, vérifier que le contenu visible du bouton est pertinent ;
  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particuliers pour le test 11.2.5 lorsque :

  - La ponctuation et les lettres majuscules sont présentes dans le texte de l’intitulé visible : elles peuvent être ignorées dans le nom accessible sans porter à conséquence ;
  - Le texte de l’intitulé visible sert de symbole : le texte ne doit pas être interprété littéralement au niveau du nom accessible. Le nom doit exprimer la fonction véhiculée par le symbole (par exemple, “B” au niveau d’un éditeur de texte aura pour nom accessible “Mettre en gras”, le signe “\>” en fonction du contexte signifiera “Suivant” ou “Lancer la vidéo”). Le cas des symboles mathématiques fait cependant exception (voir la note ci-dessous).

  Note : si l’étiquette visible représente une expression mathématique, les symboles mathématiques peuvent être repris littéralement pour servir d’étiquette au nom accessible (ex. : “A\>B”). Il est laissé à l’utilisateur le soin d’opérer la correspondance entre l’expression et ce qu’il doit épeler compte tenu de la connaissance qu’il a du fonctionnement de son logiciel de saisie vocale (“A plus grand que B” ou “A supérieur à B”).

  Ce cas particulier s’applique également au test 11.9.2.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.6 Headings and Labels (AA)
  - 9.2.5.3 Label in Name (A)
  - 9.3.3.2 Labels or Instructions (A)

- Retrouver dans le document les champs de formulaire dont l’étiquette est fournie par un élément

- ### 11.3 Dans chaque formulaire, chaque étiquette associée à un champ de formulaire ayant la même fonction et répétée plusieurs fois dans une même page ou dans un ensemble de pages est-elle cohérente ? Critère 11.3

  #### 11.3.1

  Chaque étiquette associée à un champ de formulaire ayant la même fonction et répétée plusieurs fois dans une même page est-elle cohérente ? Test 11.3.1

  - Retrouver dans le document les champs de formulaire ayant une même fonction (par exemple plusieurs champs d’adresse) ;
  - Pour chaque champ de formulaire, vérifier que les étiquettes sont cohérentes (elles permettent de comprendre qu’il s’agit de saisies de natures identiques) ;
  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.3.2

  Chaque étiquette associée à un champ de formulaire ayant la même fonction et répétée dans un ensemble de pages est-elle cohérente ? Test 11.3.2

  - Retrouver dans l’ensemble des pages considérées les champs de formulaire ayant une même fonction (par exemple le champ de saisie d’un moteur de recherche ou le champ de saisie d’inscription à une newsletter) ;
  - Pour chaque champ de formulaire, vérifier que les étiquettes sont cohérentes (elles permettent de comprendre qu’il s’agit de saisies de natures identiques) ;
  - Si c’est le cas pour chaque champ de formulaire de l’ensemble des pages considérées, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.2.4 Consistent Identification (AA)

- ### 11.4 Dans chaque formulaire, chaque étiquette de champ et son champ associé sont-ils accolés (hors cas particuliers) ? Critère 11.4

  #### 11.4.1

  Chaque étiquette de champ et son champ associé sont-ils accolés ? Test 11.4.1

  - Retrouver dans le document les champs de formulaire ;
  - Pour chaque champ de formulaire, vérifier qu’il est accolé à son étiquette ;
  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.4.2

  Chaque étiquette accolée à un champ (à l’exception des cases à cocher, bouton radio ou balises ayant un attribut WAI-ARIA

      role="checkbox"

  ,

      role="radio"

  ou

      role="switch"

  ), vérifie-t-elle ces conditions (hors cas particuliers) ? Test 11.4.2

  - L’étiquette est visuellement accolée immédiatement au-dessus ou à gauche du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de gauche à droite ;
  - L’étiquette est visuellement accolée immédiatement au-dessus ou à droite du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de droite à gauche.

  <!-- -->

  - Retrouver dans le document les champs de formulaire qui ne sont pas des éléments

        <input>

    de type

        checkbox

    ou de type

        radio

    ou des éléments ayant un attribut WAI-ARIA

        role="checkbox"

    ,

        role="radio"

    ou

        role="switch

    ";

  - Pour chaque champ de formulaire, vérifier que l’étiquette est visuellement accolée :
    - Immédiatement au-dessus ou à gauche du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de gauche à droite ;
    - Immédiatement au-dessus ou à droite du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de droite à gauche.

  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### 11.4.3

  Chaque étiquette accolée à un champ de type

      checkbox

  ou

      radio

  ou à une balise ayant un attribut WAI-ARIA

      role="checkbox"

  ,

      role="radio"

  ou

      role="switch"

  , vérifie-t-elle ces conditions (hors cas particuliers) ? Test 11.4.3

  - L’étiquette est visuellement accolée immédiatement au-dessous ou à droite du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de gauche à droite ;
  - L’étiquette est visuellement accolée immédiatement au-dessous ou à gauche du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de droite à gauche.

  <!-- -->

  - Retrouver dans le document les champs de formulaire qui sont

        <input>

    de type

        checkbox

    ou de type

        radio

    ou des éléments ayant un attribut WAI-ARIA

        role="checkbox"

    ,

        role="radio"

    ou

        role="switch

    ";

  - Pour chaque champ de formulaire, vérifier que l’étiquette est visuellement accolée :
    - Immédiatement au-dessous ou à droite du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de gauche à droite ;
    - Immédiatement au-dessous ou à gauche du champ de formulaire lorsque le sens de lecture de la langue de l’étiquette est de droite à gauche.

  - Si c’est le cas pour chaque champ de formulaire, le test est validé.

  #### Cas particuliers

  Les tests 11.4.2 et 11.4.3 seront considérés comme non applicables :

  - Dans le cas où l’étiquette mélange une portion de texte qui se lit de droite à gauche avec une portion de texte qui se lit de gauche à droite ;

  - Dans le cas où un formulaire contient des labels de plusieurs langues qui se liraient de droite à gauche et inversement. Par exemple, un formulaire de commande en arabe qui propose une liste de cases à cocher de produit en langue française ou mixant des produits en langue arabe ou en langue française ;

  - Dans le cas où les champs de type

        radio

    ou

        checkbox

    et les balises ayant un attribut WAI-ARIA

        role="checkbox"

    ,

        role="radio"

    ou

        role="switch"

    ne sont pas visuellement présentés sous forme de bouton radio ou de case à cocher ;

  - Dans le cas où les champs seraient utilisés dans un contexte où il pourrait être légitime, du point de vue de l’expérience utilisateur, de placer les étiquettes de manière différente à celle requise dans les tests 11.4.2 et 11.4.3.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.3.2 Labels or Instructions (A)

- ### 11.5 Dans chaque formulaire, les champs de même nature sont-ils regroupés, si nécessaire ? Critère 11.5

  #### 11.5.1

  Les champs de même nature vérifient-ils l’une de ces conditions, si nécessaire ? Test 11.5.1

  - Les champs de même nature sont regroupés dans une balise

        <fieldset>

    ;

  - Les champs de même nature sont regroupés dans une balise possédant un attribut WAI-ARIA

        role="group"

    ;

  - Les champs de même nature de type radio (

        <input type="radio">

    ) ou balises possédant un attribut WAI-ARIA

        role="radio"

    ) sont regroupés dans une balise possédant un attribut WAI-ARIA

        role="radiogroup"

    ou

        role="group"

    .

  <!-- -->

  - Retrouver dans le document les champs de formulaire de même nature (par exemple un groupe de saisie d’informations d’identité, une série de cases à cocher, une saisie de date sur plusieurs champs successifs…) ;
  - Pour chaque groupe de champs de formulaire de même nature, vérifier que ces champs de même nature sont regroupés :
    - Soit dans un élément

          <fieldset>

      ;

    - Soit dans un élément possédant un attribut WAI-ARIA

          role="group"

      ;

    - Soit dans un élément possédant un attribut WAI-ARIA

          role="radiogroup"

      ou

          "group"

      , s’il s’agit d’éléments

          <input>

      de type

          radio

      ( ou d’éléments possédant un attribut WAI-ARIA

          role="radio"

      ).
  - Soit dans un élément
  - Si c’est le cas pour chaque groupe de champs de formulaire de même nature, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.3.3.2 Labels or Instructions (A)

- Les champs de même nature sont regroupés dans une balise

- ### 11.6 Dans chaque formulaire, chaque regroupement de champs de même nature a-t-il une légende ? Critère 11.6

  #### 11.6.1

  Chaque regroupement de champs de même nature possède-t-il une légende ? Test 11.6.1

  - Retrouver dans le document les groupes de champs de formulaire de même nature ;
  - Pour chaque groupe de champs de formulaire de même nature, vérifier que :
    - Si le regroupement utilise un élément

          <fieldset>

      , l’élément

          <fieldset>

      possède un élément

          <legend>

      ;

    - Si l’élément de regroupement utilise un attribut WAI-ARIA

          role="group"

      ou

          "radiogroup"

      , il possède un attribut WAI-ARIA

          aria-label

      ou

          aria-labelledby

      .
  - Si le regroupement utilise un élément
  - Sinon, pour chacun des champs de même nature, vérifier la présence :
    - Soit d’un attribut title permettant de déterminer l’appartenance du champ au groupement de champ ;

    - Soit d’un attribut

          aria-label

      permettant de déterminer l’appartenance du champ au groupement de champ ;

    - Soit d’un attribut

          aria-labelledby

      qui référence un passage de texte permettant de déterminer l’appartenance du champ au groupement de champ ;

    - Soit d’un attribut

          aria-describedby

      qui référence un passage de texte permettant de déterminer l’appartenance du champ au groupement de champ.
  - Si c’est le cas pour chaque groupe de champs de formulaire ou pour chacun des champs de même nature, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.3.3.2 Labels or Instructions (A)

- ### 11.7 Dans chaque formulaire, chaque légende associée à un regroupement de champs de même nature est-elle pertinente ? Critère 11.7

  #### 11.7.1

  Chaque légende associée à un regroupement de champs de même nature est-elle pertinente ? Test 11.7.1

  - Retrouver dans le document les groupes de champs de formulaire de même nature ;
  - Pour chaque groupe de champs de formulaire de même nature ou pour chacun des champs de même nature qui dispose d’une légende, vérifier que le texte de cette légende est pertinent ;
  - Si c’est le cas pour chaque groupe de champs de formulaire ou pour chacun des champs de même nature, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.3.3.2 Labels or Instructions (A)

- ### 11.8 Dans chaque formulaire, les items de même nature d’une liste de choix sont-ils regroupés de manière pertinente ? Critère 11.8

  #### 11.8.1

  Pour chaque balise

      <select>

  , les items de même nature d’une liste de choix sont-ils regroupés avec une balise

      <optgroup>

  , si nécessaire ? Test 11.8.1

  - Retrouver dans le document les listes de sélection (élément

        <select>

    ) ;

  - Pour chaque liste de sélection proposant des groupes d’items de même nature, vérifier que ces items sont regroupés au moyen d’éléments

        <optgroup>

    ;

  - Si c’est le cas pour chaque liste de sélection proposant des groupes d’items de même nature, le test est validé.

  #### 11.8.2

  Dans chaque balise

      <select>

  , chaque balise

      <optgroup>

  possède-t-elle un attribut

      label

  ? Test 11.8.2

  - Retrouver dans le document les listes de sélection (élément

        <select>

    ) qui possèdent des éléments

        <optgroup>

    ;

  - Pour chaque élément

        <optgroup>

    , vérifier qu’il possède un attribut

        label

    ;

  - Si c’est le cas pour chaque élément

        <optgroup>

    , le test est validé.

  #### 11.8.3

  Pour chaque balise

      <optgroup>

  ayant un attribut

      label

  , le contenu de l’attribut

      label

  est-il pertinent ? Test 11.8.3

  - Retrouver dans le document les listes de sélection (élément

        <select>

    ) qui possèdent des éléments

        <optgroup>

    pourvus d’un attribut

        label

    ;

  - Pour chaque attribut

        label

    , vérifier que son contenu est pertinent ;

  - Si c’est le cas pour chaque attribut

        label

    , le test est validé.

  #### Notes techniques

  Il est possible d’utiliser une balise ayant un attribut WAI-ARIA

      role="listbox"

  en remplacement d’une balise

      <select>

  . En revanche, il est impossible de créer des groupes d’options via l’utilisation de WAI-ARIA. De ce fait, une liste nécessitant un regroupement d’options structurée à l’aide d’une balise ayant un attribut WAI-ARIA

      role="listbox"

  sera considérée comme non conforme au critère 11.8.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)

- Retrouver dans le document les listes de sélection (élément

- ### 11.9 Dans chaque formulaire, l’intitulé de chaque bouton est-il pertinent (hors cas particuliers) ? Critère 11.9

  #### 11.9.1

  L’intitulé de chaque bouton vérifie-t-il ces conditions (hors cas particuliers) ? Test 11.9.1

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    est pertinent ;

  - S’il est présent, le passage de texte lié au bouton via un attribut WAI-ARIA

        aria-labelledby

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        value

    d’une balise

        <input>

    de type

        submit

    ,

        reset

    ou

        button

    est pertinent ;

  - S’il est présent, le contenu de la balise

        <button>

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        alt

    d’une balise

        <input>

    de type

        image

    est pertinent ;

  - S’il est présent, le contenu de l’attribut

        title

    est pertinent.

  <!-- -->

  - Retrouver dans le document les boutons présents au sein d’un formulaire ;
  - Pour chaque bouton, vérifier que son intitulé visible et son nom accessible sont pertinents ;
  - Si c’est le cas pour chaque bouton, le test est validé.

  #### 11.9.2

  Chaque bouton affichant un intitulé visible vérifie-t-il ces conditions (hors cas particuliers) ? Test 11.9.2

  - S’il est présent, le contenu de l’attribut WAI-ARIA

        aria-label

    contient au moins l’intitulé visible ;

  - S’il est présent, le passage de texte lié au bouton via un attribut WAI-ARIA

        aria-labelledby

    contient au moins l’intitulé visible ;

  - S’il est présent, le contenu de l’attribut value d’une balise

        <input>

    de type

        submit

    ,

        reset

    ou

        button

    contient au moins l’intitulé visible ;

  - S’il est présent, le contenu de la balise

        <button>

    contient au moins l’intitulé visible ;

  - S’il est présent, le contenu de l’attribut

        alt

    d’une balise

        <input>

    de type

        image

    contient au moins l’intitulé visible ;

  - S’il est présent, le contenu de l’attribut

        title

    contient au moins l’intitulé visible.

  <!-- -->

  - Retrouver dans le document les boutons présents au sein d’un formulaire ;
  - Pour chaque bouton, vérifier que son nom accessible contient au moins son intitulé visible ;
  - Si c’est le cas pour chaque bouton, le test est validé.

  #### Cas particuliers

  Pour le test 11.9.2, voir cas particuliers critère 11.2.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.5.3 Label in Name (A)
  - 9.4.1.2 Name, Role, Value (A)

- S’il est présent, le contenu de l’attribut WAI-ARIA

- ### 11.10 Dans chaque formulaire, le contrôle de saisie est-il utilisé de manière pertinente (hors cas particuliers) ? Critère 11.10

  #### 11.10.1

  Les indications du caractère obligatoire de la saisie des champs vérifient-elles une de ces conditions (hors cas particuliers) ? Test 11.10.1

  - Une indication de champ obligatoire est visible et permet d’identifier nommément le champ concerné préalablement à la validation du formulaire ;

  - Le champ obligatoire dispose de l’attribut

        aria-required="true"

    ou

        required

    préalablement à la validation du formulaire.

  <!-- -->

  - Retrouver dans le document les champs de formulaire obligatoires ;
  - Pour chaque champ de formulaire, vérifier que préalablement à la validation du formulaire :
    - Soit une indication de champ obligatoire est visible et permet d’identifier nommément le champ concerné ;

    - Soit le champ possède un attribut

          aria-required="true"

      ou

          required

      .
  - Si c’est le cas pour chaque champ de formulaire obligatoire, le test est validé.

  #### 11.10.2

  Les champs obligatoires ayant l’attribut

      aria-required="true"

  ou

      required

  vérifient-ils une de ces conditions ? Test 11.10.2

  - Une indication de champ obligatoire est visible et située dans l’étiquette associée au champ préalablement à la validation du formulaire ;
  - Une indication de champ obligatoire est visible et située dans le passage de texte associé au champ préalablement à la validation du formulaire.

  <!-- -->

  - Retrouver dans le document les champs de formulaire obligatoires qui possèdent un attribut

        aria-required="true"

    ou

        required

    ;

  - Pour chaque champ de formulaire, vérifier que préalablement à la validation du formulaire :
    - Soit une indication de champ obligatoire est visible et située dans l’étiquette associée au champ ;
    - Soit une indication de champ obligatoire est visible et située dans le passage de texte associé au champ.

  - Si c’est le cas pour chaque champ de formulaire obligatoire qui possèdent un attribut

        aria-required="true"

    ou

        required

    , le test est validé.

  #### 11.10.3

  Les messages d’erreur indiquant l’absence de saisie d’un champ obligatoire vérifient-ils une de ces conditions ? Test 11.10.3

  - Le message d’erreur indiquant l’absence de saisie d’un champ obligatoire est visible et permet d’identifier nommément le champ concerné ;

  - Le champ obligatoire dispose de l’attribut

        aria-invalid="true"

    .

  <!-- -->

  - Retrouver dans le document les messages d’erreur indiquant l’absence de saisie d’un champ obligatoire ;
  - Pour chaque message d’erreur, vérifier que :
    - Soit le message d’erreur est visible et permet d’identifier nommément le champ concerné ;

    - Soit le champ obligatoire associé au message d’erreur possède un attribut

          aria-invalid="true"

      .
  - Si c’est le cas pour chaque message d’erreur indiquant l’absence de saisie d’un champ obligatoire, le test est validé.

  #### 11.10.4

  Les champs obligatoires ayant l’attribut

      aria-invalid="true"

  vérifient-ils une de ces conditions ? Test 11.10.4

  - Le message d’erreur indiquant le caractère invalide de la saisie est visible et situé dans l’étiquette associée au champ ;
  - Le message d’erreur indiquant le caractère invalide de la saisie est visible et situé dans le passage de texte associé au champ.

  <!-- -->

  - Retrouver dans le document les champs de formulaire obligatoires qui possèdent un attribut

        aria-invalid="true"

    ;

  - Pour chaque champ de formulaire, vérifier que :
    - Soit le message d’erreur indiquant le caractère invalide de la saisie est visible et situé dans l’étiquette associée au champ ;
    - Soit le message d’erreur indiquant le caractère invalide de la saisie est visible et situé dans le passage de texte associé au champ.

  - Si c’est le cas pour chaque champ de formulaire obligatoire qui possède un attribut

        aria-invalid="true"

    , le test est validé.

  #### 11.10.5

  Les instructions et indications du type de données et/ou de format obligatoires vérifient-elles une de ces conditions ? Test 11.10.5

  - Une instruction ou une indication du type de données et/ou de format obligatoire est visible et permet d’identifier nommément le champ concerné préalablement à la validation du formulaire ;
  - Une instruction ou une indication du type de données et/ou de format obligatoire est visible dans l’étiquette ou le passage de texte associé au champ préalablement à la validation du formulaire.

  <!-- -->

  - Retrouver dans le document les champs de formulaire obligatoires auxquels est associée une instruction ou une indication du type de données et/ou de format obligatoire ;
  - Pour chaque champ de formulaire, vérifier que l’instruction ou l’indication du type de données et/ou de format obligatoire est préalablement à la validation du formulaire :
    - Soit visible et permet d’identifier nommément le champ concerné ;
    - Soit visible dans l’étiquette ou le passage de texte associé au champ.
  - Si c’est le cas pour chaque champ de formulaire obligatoire auquel est associée une instruction ou une indication du type de données et/ou de format obligatoire, le test est validé.

  #### 11.10.6

  Les messages d’erreurs fournissant une instruction ou une indication du type de données et/ou de format obligatoire des champs vérifient-ils une de ces conditions ? Test 11.10.6

  - Le message d’erreur fournissant une instruction ou une indication du type de données et/ou de format obligatoires est visible et identifie le champ concerné ;

  - Le champ dispose de l’attribut

        aria-invalid="true"

    .

  <!-- -->

  - Retrouver dans le document les messages d’erreur fournissant une instruction ou une indication du type de données et/ou de format obligatoire d’un champ ;
  - Pour chaque message d’erreur, vérifier que :
    - Soit le message d’erreur est visible et permet d’identifier nommément le champ concerné ;

    - Soit le champ associé au message d’erreur possède un attribut

          aria-invalid="true"

      .
  - Si c’est le cas pour chaque message d’erreur indiquant l’absence de saisie d’un champ obligatoire, le test est validé.

  #### 11.10.7

  Les champs ayant l’attribut

      aria-invalid="true"

  dont la saisie requiert un type de données et/ou de format obligatoires vérifient-ils une de ces conditions ? Test 11.10.7

  - Une instruction ou une indication du type de données et/ou de format obligatoire est visible et située dans la balise

        <label>

    associée au champ ;

  - Une instruction ou une indication du type de données et/ou de format obligatoire est visible et située dans le passage de texte associé au champ.

  <!-- -->

  - Retrouver dans le document les champs de formulaire qui possèdent un attribut

        aria-invalid="true"

    ;

  - Pour chaque champ de formulaire, vérifier que :
    - Soit une instruction ou une indication du type de données et/ou de format obligatoire est visible et située dans l’élément

          <label>

      associé au champ ;

    - Soit une instruction ou une indication du type de données et/ou de format obligatoire est visible et située dans le passage de texte associé au champ.

  - Soit une instruction ou une indication du type de données et/ou de format obligatoire est visible et située dans l’élément

  - Si c’est le cas pour chaque champ de formulaire qui possède un attribut

        aria-invalid="true"

    , le test est validé.

  #### Cas particuliers

  Le test 11.10.1 et le test 11.10.2 seront considérés comme non applicables lorsque le formulaire comporte un seul champ de formulaire ou qu’il indique les champs optionnels de manière :

  - Visible ;

  - Dans la balise

        <label>

    ou dans la légende associée au champ.

  Dans le cas où l’ensemble des champs d’un formulaire sont obligatoires, les tests 11.10.1 et 11.10.2 restent applicables.

  #### Notes techniques

  Dans un long formulaire dont la majorité des champs sont obligatoires, on pourrait constater que ce sont les quelques champs restés facultatifs qui sont explicitement signalés comme tels. Dans ce cas, il faudrait s’assurer que :

  - Un message précise visuellement en haut de formulaire que “tous les champs sont obligatoires sauf ceux indiqués comme étant facultatifs” ;

  - Une mention “facultatif” est présente visuellement dans le libellé des champs facultatifs ou dans la légende d’un groupe de champs facultatifs ;

  - Un attribut

        required

    ou

        aria-required="true"

    reste associé à chaque champ qui n’est pas concerné par ce caractère facultatif.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.3.1 Error Identification (A)
  - 9.3.3.2 Labels or Instructions (A)

- ### 11.11 Dans chaque formulaire, le contrôle de saisie est-il accompagné, si nécessaire, de suggestions facilitant la correction des erreurs de saisie ? Critère 11.11

  #### 11.11.1

  Pour chaque erreur de saisie, les types et les formats de données sont-ils suggérés, si nécessaire ? Test 11.11.1

  - Retrouver dans le document les messages d’erreur ;
  - Pour chaque message d’erreur, vérifier que les types et les formats de données attendus sont suggérés ;
  - Si c’est le cas pour chaque message d’erreur , le test est validé.

  #### 11.11.2

  Pour chaque erreur de saisie, des exemples de valeurs attendues sont-ils suggérés, si nécessaire ? Test 11.11.2

  - Retrouver dans le document les messages d’erreur ;
  - Pour chaque message d’erreur, vérifier que des exemples de valeurs attendues sont suggérés ;
  - Si c’est le cas pour chaque message d’erreur , le test est validé.

  #### Notes techniques

  Certains types de contrôles en HTML5 proposent des messages d’aide à la saisie automatique : par exemple le type

      email

  affiche un message du type « veuillez saisir une adresse e-mail valide » dans le cas où l’adresse e-mail saisie ne correspond pas au format attendu. Ces messages sont personnalisables via l’API Constraint Validation, ce qui permet de personnaliser les messages d’erreur et de valider le critère. L’attribut

      pattern

  permet d’effectuer automatiquement des contrôles de format (via des expressions régulières) et affiche un message d’aide personnalisable via l’attribut

      title

  : ce dispositif valide également le critère.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.3.3 Error Suggestion (AA)

- ### 11.12 Pour chaque formulaire qui modifie ou supprime des données, ou qui transmet des réponses à un test ou à un examen, ou dont la validation a des conséquences financières ou juridiques, les données saisies peuvent-elles être modifiées, mises à jour ou récupérées par l’utilisateur ? Critère 11.12

  #### 11.12.1

  Pour chaque formulaire qui modifie ou supprime des données, ou qui transmet des réponses à un test ou un examen, ou dont la validation a des conséquences financières ou juridiques, la saisie des données vérifie-t-elle une de ces conditions ? Test 11.12.1

  - L’utilisateur peut modifier ou annuler les données et les actions effectuées sur ces données après la validation du formulaire ;

  - L’utilisateur peut vérifier et corriger les données avant la validation d’un formulaire en plusieurs étapes ;

  - Un mécanisme de confirmation explicite, via une case à cocher (balise

        <input>

    de type

        checkbox

    ou balise ayant un attribut WAI-ARIA

        role="checkbox"

    ) ou une étape supplémentaire, est présent.

  <!-- -->

  - Retrouver dans le document les formulaires qui modifient ou suppriment des données, ou qui transmettent des réponses à un test ou un examen, ou dont la validation a des conséquences financières ou juridiques ;
  - Pour chaque formulaire, vérifier que l’utilisateur peut :
    - Soit modifier ou annuler les données et les actions effectuées sur ces données après la validation du formulaire ;
    - Soit vérifier et corriger les données avant la validation d’un formulaire en plusieurs étapes ;
    - Soit disposer d’un mécanisme de confirmation explicite (par exemple, une case à cocher ou une étape supplémentaire).
  - Si c’est le cas pour chaque formulaire retrouvé, le test est validé.

  #### 11.12.2

  Chaque formulaire dont la validation modifie ou supprime des données à caractère financier, juridique ou personnel vérifie-t-il une de ces conditions ? Test 11.12.2

  - Un mécanisme permet de récupérer les données supprimées ou modifiées par l’utilisateur ;
  - Un mécanisme de demande de confirmation explicite de la suppression ou de la modification, via un champ de formulaire ou une étape supplémentaire, est proposé.

  <!-- -->

  - Retrouver dans le document les formulaires qui modifient ou suppriment des données à caractère financier, juridique ou personnel ;
  - Pour chaque formulaire, vérifier que l’utilisateur dispose :
    - Soit d’un mécanisme qui permet de récupérer les données supprimées ou modifiées ;
    - Soit d’un mécanisme de demande de confirmation explicite de la suppression ou de la modification (par exemple, une case à cocher ou une étape supplémentaire).
  - Si c’est le cas pour chaque formulaire retrouvé, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.3.4 Error Prevention (Legal, Financial, Data) (AA)

- ### 11.13 La finalité d’un champ de saisie peut-elle être déduite pour faciliter le remplissage automatique des champs avec les données de l’utilisateur ? Critère 11.13

  #### 11.13.1

  Chaque champ de formulaire dont l’objet se rapporte à une information concernant l’utilisateur vérifie-t-il ces conditions ? Test 11.13.1

  - Le champ de formulaire possède un attribut

        autocomplete

    ;

  - L’attribut

        autocomplete

    est pourvu d’une valeur présente dans la liste des valeurs possibles pour l’attribut

        autocomplete

    associés à un champ de formulaire ;

  - La valeur indiquée pour l’attribut

        autocomplete

    est pertinente au regard du type d’information attendu.

  <!-- -->

  - Retrouver dans le document les champs de formulaire qui se rapportent à une information concernant l’utilisateur (nom, prénom, numéro de téléphone, etc.) ;
  - Pour chaque champ de formulaire, vérifier que :
    - Le champ de formulaire possède un attribut

          autocomplete

      ;

    - L’attribut

          autocomplete

      est pourvu d’une valeur présente dans la liste des valeurs possibles ;

    - La valeur indiquée pour l’attribut

          autocomplete

      est pertinente au regard du type d’information attendu.
  - Le champ de formulaire possède un attribut
  - Si c’est le cas pour chaque champ de formulaire retrouvé, le test est validé.

  #### Notes techniques

  La liste des valeurs possibles pour l’attribut

      autocomplete

  repose sur la liste des valeurs présentes dans la spécification WCAG2.1 qui reprend elle-même la liste des valeurs de type “field name” de la spécification HTML5.2. Le critère WCAG demande à ce que l’une de ces valeurs soit présente pour qualifier un champ de saisie concernant l’utilisateur.

  Ce que le critère WCAG laisse implicite, ce sont les différentes règles de construction possibles pour obtenir une valeur (simple ou composée) pour l’attribut

      autocomplete

  . C’est cependant l’affaire du développeur de fournir à l’attribut

      autocomplete

  une valeur ou un ensemble de valeurs valides au regard des exigences de l’algorithme fourni par la spécification HTML5.2. Ainsi, un attribut

      autocomplete

  ne peut contenir qu’une seule valeur de type

      “field name”

  , comme

      "name"

  ou

      "street-address"

  . On peut avoir également un ensemble composé de différentes valeurs comme, par exemple,

      autocomplete="shipping name"

  ou

      autocomplete="section-software shipping street-address"

  :

      "section-software"

  renvoie à une valeur de type “scope” et

      "shipping"

  à une valeur de type “hint set”, mais toujours une seule valeur de type “field name”.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.5 Identify Input Purpose (AA)

- Le champ de formulaire possède un attribut

## 12. Navigation Thématique Navigation

- ### 12.1 Chaque ensemble de pages dispose-t-il de deux systèmes de navigation différents, au moins (hors cas particuliers) ? Critère 12.1

  #### 12.1.1

  Chaque ensemble de pages vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 12.1.1

  - Un menu de navigation et un plan du site sont présents ;
  - Un menu de navigation et un moteur de recherche sont présents ;
  - Un moteur de recherche et un plan du site sont présents.

  <!-- -->

  - Pour chaque ensemble de pages du site, vérifier la présence :
    - Soit d’un menu de navigation et d’un plan du site ;
    - Soit d’un menu de navigation et d’un moteur de recherche ;
    - Soit d’un moteur de recherche et d’un plan du site.
  - Si c’est le cas pour chaque ensemble de pages du site, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particulier lorsque le site web est constitué d’une seule page ou d’un nombre très limité de pages (cf. note). Dans ce cas-là, le critère est non applicable.

  Le critère est également non applicable pour les pages d’un ensemble de pages qui sont le résultat ou une partie d’un processus (un processus de paiement ou de prise de commande, par exemple).

  Note : l’appréciation d’un nombre très limité de pages devrait être réservé à un site dont l’ensemble des pages sont atteignables depuis la page d’accueil.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.5 Multiple Ways (AA)

- ### 12.2 Dans chaque ensemble de pages, le menu et les barres de navigation sont-ils toujours à la même place (hors cas particuliers) ? Critère 12.2

  #### 12.2.1

  Dans chaque ensemble de pages, chaque page disposant d’un menu et les barres de navigation vérifie-t-elle ces conditions (hors cas particuliers) ? Test 12.2.1

  - Le menu et les barres de navigation sont toujours à la même place dans la présentation ;
  - Le menu et les barres de navigation se présentent toujours dans le même ordre relatif dans le code source.

  <!-- -->

  - Choisir une page de l’échantillon appartenant au même ensemble que la page en cours d’audit ;
  - Comparer visuellement les deux pages et vérifier que le menu ou les barres de navigation sont toujours à la même place dans la présentation ;
  - Comparer le code source (généré côté client) des deux pages et vérifier que le menu ou les barres de navigation se présentent toujours dans le même ordre relatif dans la structure ;
  - Si c’est le cas, le test est validé.

  Note : le critère est non applicable dans les situations où :

  - Les pages d’un ensemble de pages sont le résultat ou une partie d’un processus (un processus de paiement ou de prise de commande, par exemple) ;
  - La page est la page d’accueil ;
  - Le site web est constitué d’une seule page.

  #### Cas particuliers

  Il existe une gestion de cas particuliers lorsque :

  - La page est la page d’accueil ;
  - Le site web est constitué d’une seule page ;
  - Le changement fait suite à une modification initiée par l’utilisateur.

  Dans ces situations, le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.2.3 Consistent Navigation (AA)

- ### 12.3 La page « plan du site » est-elle pertinente ? Critère 12.3

  #### 12.3.1

  La page « plan du site » est-elle représentative de l’architecture générale du site ? Test 12.3.1

  - Vérifier que le plan du site est représentatif de l’architecture générale du site (cf. note) ;
  - Si c’est le cas, le test est validé.

  Note : Un plan du site trop complexe ou trop profond n’est pas recommandé pour aider à la navigation. Il n’est pas obligatoire que toutes les pages soient présentes dans le plan du site si elles peuvent être atteintes, par exemple, à partir de la page d’accueil d’une rubrique ou d’un catalogue.

  #### 12.3.2

  Les liens du plan du site sont-ils fonctionnels ? Test 12.3.2

  - Pour tous les liens du plan du site, vérifier qu’ils sont fonctionnels ;
  - Si c’est le cas, le test est validé.

  #### 12.3.3

  Les liens du plan du site renvoient-ils bien vers les pages indiquées par l’intitulé ? Test 12.3.3

  - Pour tous les liens du plan du site, vérifier qu’ils sont à jour (ni obsolètes ni en erreur) et conduisent à la page indiquée par leur intitulé ;
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.5 Multiple Ways (AA)

- ### 12.4 Dans chaque ensemble de pages, la page « plan du site » est-elle accessible à partir d’une fonctionnalité identique ? Critère 12.4

  #### 12.4.1

  Dans chaque ensemble de pages, la page « plan du site » est-elle accessible à partir d’une fonctionnalité identique ? Test 12.4.1

  - Choisir une page de l’échantillon appartenant au même ensemble que la page en cours d’audit ;
  - Comparer le code source (généré côté client) des deux pages et vérifier que le moyen d’accès au plan du site est toujours le même (un lien ou un bouton, par exemple) ;
  - Si c’est le cas, le test est validé.

  #### 12.4.2

  Dans chaque ensemble de pages, la fonctionnalité vers la page « plan du site » est-elle située à la même place dans la présentation ? Test 12.4.2

  - Choisir une page de l’échantillon appartenant au même ensemble que la page en cours d’audit ;
  - Comparer le code source (généré côté client) des deux pages et vérifier que le moyen d’accès au plan du site est toujours à la même place dans la structure (par rapport à l’ordre relatif des éléments de la page, par exemple il est toujours en haut de page) ;
  - Si c’est le cas, le test est validé.

  #### 12.4.3

  Dans chaque ensemble de pages, la fonctionnalité vers la page « plan du site » se présente-t-elle toujours dans le même ordre relatif dans le code source ? Test 12.4.3

  - Choisir une page de l’échantillon appartenant au même ensemble que la page en cours d’audit ;
  - Comparer visuellement les deux pages et vérifier que le moyen d’accès au plan du site est toujours à la même place dans la présentation ;
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.5 Multiple Ways (AA)
  - 9.3.2.3 Consistent Navigation (AA)

- ### 12.5 Dans chaque ensemble de pages, le moteur de recherche est-il atteignable de manière identique ? Critère 12.5

  #### 12.5.1

  Dans chaque ensemble de pages, le moteur de recherche est-il accessible à partir d’une fonctionnalité identique ? Test 12.5.1

  - Choisir une page de l’échantillon appartenant au même ensemble que la page en cours d’audit ;
  - Comparer le code source (généré côté client) des deux pages et vérifier que le moyen d’accès au moteur de recherche est toujours le même (un champ de formulaire, par exemple) ;
  - Si c’est le cas, le test est validé.

  #### 12.5.2

  Dans chaque ensemble de pages, la fonctionnalité vers le moteur de recherche est-elle située à la même place dans la présentation ? Test 12.5.2

  - Choisir une page de l’échantillon appartenant au même ensemble que la page en cours d’audit ;
  - Comparer visuellement les deux pages et vérifier que le moyen d’accès au moteur de recherche est toujours à la même place dans la présentation ;
  - Si c’est le cas, le test est validé.

  #### 12.5.3

  Dans chaque ensemble de pages, la fonctionnalité vers le moteur de recherche se présente-t-elle toujours dans le même ordre relatif dans le code source ? Test 12.5.3

  - Choisir une page de l’échantillon appartenant au même ensemble que la page en cours d’audit ;
  - Comparer le code source (généré côté client) des deux pages et vérifier que le moyen d’accès au moteur de recherche est toujours à la même place dans la structure (par rapport à l’ordre relatif des éléments de la page, par exemple il est toujours en haut de page) ;
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.2.3 Consistent Navigation (AA)

- ### 12.6 Les zones de regroupement de contenus présentes dans plusieurs pages web (zones d’en-tête, de navigation principale, de contenu principal, de pied de page et de moteur de recherche) peuvent-elles être atteintes ou évitées ? Critère 12.6

  #### 12.6.1

  Dans chaque page web où elles sont présentes, la zone d’en-tête, de navigation principale, de contenu principal, de pied de page et de moteur de recherche respectent-elles au moins une de ces conditions ? Test 12.6.1

  - La zone possède un rôle WAI-ARIA de type landmark correspondant à sa nature ;
  - La zone possède un titre dont le contenu permet de comprendre la nature du contenu de la zone ;
  - La zone peut être masquée par le biais d’un bouton précédent directement la zone dans l’ordre du code source ;
  - La zone peut être évitée par le biais d’un lien d’évitement précédent directement la zone dans l’ordre du code source ;
  - La zone peut être atteinte par le biais d’un lien d’accès rapide visible ou, à défaut, visible à la prise de focus.

  <!-- -->

  - Retrouver dans le document les zones de regroupement de contenus (zones d’en-tête, de navigation principale, de contenu principal, de pied de page et de moteur de recherche) ;
  - Pour chaque zone, vérifier que la zone :
    - Soit possède un rôle WAI-ARIA de type landmark correspondant à sa nature ;
    - Soit possède un titre de hiérarchie dont le contenu permet de comprendre la nature du contenu de la zone ;
    - Soit peut être masquée au moyen d’un bouton précédant directement la zone dans l’ordre du code source ;
    - Soit peut être évitée au moyen d’un lien d’évitement précédant directement la zone dans l’ordre du code source ;
    - Soit peut être atteinte au moyen d’un lien d’accès rapide soit visible par défaut, soit visible à la prise de focus lors d’une tabulation.
  - Si c’est le cas pour chaque zone de regroupement de contenus, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.1 Info and Relationships (A)
  - 9.2.4.1 Bypass Blocks (A)
  - 9.4.1.2 Name, Role, Value (A)

- ### 12.7 Dans chaque page web, un lien d’évitement ou d’accès rapide à la zone de contenu principal est-il présent (hors cas particuliers) ? Critère 12.7

  #### 12.7.1

  Dans chaque page web, un lien permet-il d’éviter la zone de contenu principal ou d’y accéder (hors cas particuliers) ? Test 12.7.1

  - Retrouver dans le document la zone de contenu principal (indiquée par l’élément main visible) ;
  - Vérifier que la zone :
    - Soit peut être évitée au moyen d’un lien d’évitement précédant directement la zone dans l’ordre du code source ;
    - Soit peut être atteinte au moyen d’un lien d’accès rapide visible à la prise de focus lors d’une tabulation.
  - Si c’est le cas, le test est validé.

  #### 12.7.2

  Dans chaque ensemble de pages, le lien d’évitement ou d’accès rapide à la zone de contenu principal vérifie-t-il ces conditions (hors cas particuliers) ? Test 12.7.2

  - Le lien est situé à la même place dans la présentation ;
  - Le lien se présente toujours dans le même ordre relatif dans le code source ;
  - Le lien est visible ou, à défaut, visible à la prise de focus ;
  - Le lien est fonctionnel.

  <!-- -->

  - Retrouver dans le document la zone de contenu principal (indiquée par l’élément main visible) ;
  - Vérifier que le lien d’évitement ou d’accès rapide à la zone est :
    - Situé à la même place dans la présentation ;
    - Présent toujours dans le même ordre relatif dans le code source (généré côté client) ;
    - Visible à la prise de focus lors d’une tabulation ;
    - Fonctionnel.
  - Si c’est le cas, le test est validé.

  Note : lorsque le site web est constitué d’une seule page, l’obligation de la présence d’un lien d’accès rapide est liée au contexte de la page (présence ou absence de navigation ou de contenus additionnels, par exemple). Le critère peut être considéré comme non applicable lorsqu’il est avéré qu’un lien d’accès rapide est inutile.

  #### Cas particuliers

  Il existe une gestion de cas particuliers lorsque le site web est constitué d’une seule page.

  Dans ce cas de figure, l’obligation de la présence d’un lien d’accès rapide est liée au contexte de la page : présence ou absence de navigation ou de contenus additionnels, par exemple. Le critère peut être considéré comme non applicable lorsqu’il est avéré qu’un lien d’accès rapide est inutile.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.1 Bypass Blocks (A)
  - 9.2.4.3 Focus Order (A)
  - 9.3.2.3 Consistent Navigation (AA)

- ### 12.8 Dans chaque page web, l’ordre de tabulation est-il cohérent ? Critère 12.8

  #### 12.8.1

  Dans chaque page web, l’ordre de tabulation dans le contenu est-il cohérent ? Test 12.8.1

  - Parcourir dans le document l’ensemble des contenus au moyen de la touche de tabulation vers l’avant (touche Tab) et vers l’arrière (touches Maj+Tab) ;
  - Vérifier que l’ordre de déplacement du focus reste cohérent relativement au contenu considéré (par exemple, l’ordre de tabulation dans une fenêtre modale ne doit considérer que les éléments d’interface présents au sein de cette fenêtre) ;
  - Si c’est le cas, le test est validé.

  Note : il n’est pas obligatoire que la tabulation suive l’ordre de lecture naturel (de gauche à droite et de haut en bas par exemple) tant que les éléments sont accessibles dans un ordre cohérent.

  #### 12.8.2

  Pour chaque script qui met à jour ou insère un contenu, l’ordre de tabulation reste-t-il cohérent ? Test 12.8.2

  - Retrouver dans le document l’ensemble des contenus insérés au moyen d’un script (affichage d’éléments masqués, mise jour de contenu via AJAX par exemple) ;
  - Positionner la tabulation sur l’élément déclencheur et l’activer ;
  - Après l’affichage du contenu mis à jour, vérifier que la tabulation reste cohérente (repositionnement correct du focus) ;
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.4.3 Focus Order (A)

- ### 12.9 Dans chaque page web, la navigation ne doit pas contenir de piège au clavier. Cette règle est-elle respectée ? Critère 12.9

  #### 12.9.1

  Dans chaque page web, chaque élément recevant le focus vérifie-t-il une de ces conditions ? Test 12.9.1

  - Il est possible d’atteindre l’élément suivant ou précédent pouvant recevoir le focus avec la touche de tabulation ;
  - L’utilisateur est informé d’un mécanisme fonctionnel permettant d’atteindre au clavier l’élément suivant ou précédent pouvant recevoir le focus.

  <!-- -->

  - Retrouver dans le document l’ensemble des éléments d’interface susceptibles de recevoir le focus (au moyen de la tabulation ou au moyen d’un script) ;
  - Pour chaque élément d’interface, vérifier que l’utilisateur peut atteindre l’élément suivant ou précédent pouvant recevoir le focus :
    - Soit au moyen de la touche de tabulation (Tab ou Maj+Tab) ;
    - Soit au moyen d’une autre interaction clavier dont l’utilisateur est informé (par exemple, les flèches de direction).
  - Si c’est le cas pour chaque élément d’interface, le test est validé.

  Note : certains éléments d’interface complexes, comme un groupe de boutons radio, une liste de sélection et tous les composants développés avec WAI-ARIA font appel à des navigations optimisées qui utilisent généralement les flèches de direction pour passer d’une partie du composant à l’autre. Par exemple, dans un groupe de boutons radio les options sont navigables avec les flèches de direction. De même dans un système d’onglets l’utilisateur active les onglets avec les flèches de direction. Le test sur le piège au clavier se limite alors à vérifier que le composant est atteint avec la tabulation et qu’il est possible de passer au composant suivant ou revenir au composant précédent.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.1.1 Keyboard (A)
  - 9.2.1.2 No Keyboard Trap (A)

- ### 12.10 Dans chaque page web, les raccourcis clavier n’utilisant qu’une seule touche (lettre minuscule ou majuscule, ponctuation, chiffre ou symbole) sont-ils contrôlables par l’utilisateur ? Critère 12.10

  #### 12.10.1

  Dans chaque page web, chaque raccourci clavier n’utilisant qu’une seule touche (lettre minuscule ou majuscule, ponctuation, chiffre ou symbole) vérifie-t-il l’une de ces conditions ? Test 12.10.1

  - Un mécanisme est disponible pour désactiver le raccourci clavier ;
  - Un mécanisme est disponible pour configurer la touche de raccourci clavier au moyen des touches de modification (Ctrl, Alt, Maj, etc.) ;
  - Dans le cas d’un composant d’interface utilisateur, le raccourci clavier qui lui est associé ne peut être activé que si le focus clavier est sur ce composant.

  <!-- -->

  - Retrouver dans le document l’ensemble des raccourcis clavier proposés à l’utilisateur ;
  - Pour chaque raccourci clavier, vérifier que :
    - Soit un mécanisme est disponible pour désactiver le raccourci clavier ;
    - Soit un mécanisme est disponible pour configurer la touche de raccourci clavier au moyen des touches de modification (Ctrl, Alt, Maj, etc.) ;
    - Soit, dans le cas d’un composant d’interface utilisateur, le raccourci clavier qui lui est associé ne peut être activé que si le focus clavier est sur ce composant.
  - Si c’est le cas pour chaque raccourci clavier, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.1.4 Character Key Shortcuts (A)

- ### 12.11 Dans chaque page web, les contenus additionnels apparaissant au survol, à la prise de focus ou à l’activation d’un composant d’interface sont-ils si nécessaire atteignables au clavier ? Critère 12.11

  #### 12.11.1

  Dans chaque page web, les contenus additionnels apparaissant au survol, à la prise de focus ou à l’activation d’un composant d’interface sont-ils si nécessaire atteignables au clavier ? Test 12.11.1

  - Retrouver dans le document l’ensemble des contenus additionnels apparaissant au survol, à la prise de focus ou à l’activation d’un composant d’interface ;
  - Pour chaque contenu additionnel, s’il contient des composants d’interface avec lesquels l’utilisateur peut interagir au clavier (par exemple, une infobulle personnalisée qui propose un lien dans son contenu), vérifier que ces composants d’interface sont atteignables au clavier ;
  - Si c’est le cas pour chaque contenu additionnel, le test est validé.

  #### Notes techniques

  Ce critère adresse les situations où un contenu additionnel contient des composants d’interface avec lesquels il doit être possible d’interagir au clavier. Par exemple, une infobulle personnalisée qui propose un lien dans son contenu.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.1.1 Keyboard (A)

## 13. Consultation Thématique Consultation

- ### 13.1 Pour chaque page web, l’utilisateur a-t-il le contrôle de chaque limite de temps modifiant le contenu (hors cas particuliers) ? Critère 13.1

  #### 13.1.1

  Pour chaque page web, chaque procédé de rafraîchissement (balise

      <object>

  , balise

      <embed>

  , balise

      <svg>

  , balise

      <canvas>

  , balise

      <meta>

  ) vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 13.1.1

  - L’utilisateur peut arrêter ou relancer le rafraîchissement ;
  - L’utilisateur peut augmenter la limite de temps entre deux rafraîchissements de dix fois, au moins ;
  - L’utilisateur est averti de l’imminence du rafraîchissement et dispose de vingt secondes, au moins, pour augmenter la limite de temps avant le prochain rafraîchissement ;
  - La limite de temps entre deux rafraîchissements est de vingt heures, au moins.

  <!-- -->

  - Retrouver dans le document les rafraîchissements initiés dans le contenu par un élément

        <object>

    ,

        <embed>

    ,

        <svg>

    ,

        <canvas>

    ou par un élément

        <meta http-equiv="refresh" content="[compteur]">

    (dans l’élément

        <head>

    de la page) ;

  - Pour chaque rafraîchissement, vérifier que :
    - Soit la présence d’un mécanisme permet à l’utilisateur de stopper et de relancer le rafraîchissement ;
    - Soit la présence d’un mécanisme permet à l’utilisateur d’augmenter la limite de temps entre deux rafraîchissements de dix fois, au moins ;
    - Soit la présence d’un mécanisme qui avertit l’utilisateur de l’imminence du rafraîchissement, laisse 20 secondes, au moins, à l’utilisateur, pour augmenter la limite de temps avant le prochain rafraîchissement ;
    - Soit la limite de temps entre deux rafraîchissements est de vingt heures, au moins.

  - Si c’est le cas, le test est validé.

  #### 13.1.2

  Pour chaque page web, chaque procédé de redirection effectué via une balise

      <meta>

  est-il immédiat (hors cas particuliers) ? Test 13.1.2

  - Retrouver dans le document une redirection automatique initiée par un élément

        <meta http-equiv=“refresh” content=“0;URL=‘[URL ciblée]’” />

    ;

  - Vérifier que la redirection est immédiate ;

  - Si c’est le cas, le test est validé.

  #### 13.1.3

  Pour chaque page web, chaque procédé de redirection effectué via un script vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 13.1.3

  - L’utilisateur peut arrêter ou relancer la redirection ;
  - L’utilisateur peut augmenter la limite de temps avant la redirection de dix fois, au moins ;
  - L’utilisateur est averti de l’imminence de la redirection et dispose de vingt secondes, au moins, pour augmenter la limite de temps avant la prochaine redirection ;
  - La limite de temps avant la redirection est de vingt heures, au moins.

  <!-- -->

  - Retrouver dans le document les redirections automatiques initiées par un script (sous la forme d’un décompte par exemple) ;
  - Pour chaque redirection automatique, vérifier que :
    - Soit la présence d’un mécanisme permet à l’utilisateur de stopper et relancer la redirection ;
    - Soit la présence d’un mécanisme permet à l’utilisateur d’augmenter la limite de temps avant le rafraîchissement de dix fois, au moins ;
    - Soit la présence d’un mécanisme qui avertit l’utilisateur de l’imminence du rafraîchissement, laisse 20 secondes, au moins, à l’utilisateur, pour augmenter la limite de temps avant le prochain rafraîchissement ;
    - Soit la limite de temps avant la redirection est de vingt heures, au moins.
  - Si c’est le cas, le test est validé.

  #### 13.1.4

  Pour chaque page web, chaque procédé limitant le temps d’une session vérifie-t-il une de ces conditions (hors cas particuliers) ? Test 13.1.4

  - L’utilisateur peut supprimer la limite de temps ;
  - L’utilisateur peut augmenter la limite de temps ;
  - La limite de temps avant la fin de la session est de vingt heures au moins.

  <!-- -->

  - Retrouver dans le document les procédés limitant le temps d’une session (par exemple, après une authentification) ;
  - Pour chaque procédé, vérifier que :
    - Soit la présence d’un mécanisme permet à l’utilisateur de supprimer la limite de temps ;
    - Soit la présence d’un mécanisme permet à l’utilisateur d’augmenter la limite de temps ;
    - Soit la limite de temps est de vingt heures, au moins.
  - Si c’est le cas, le test est validé.

  Note : lorsque la limite de temps est essentielle, notamment lorsqu’elle ne pourrait pas être supprimée sans changer fondamentalement le contenu ou les fonctionnalités liées au contenu, le critère est non applicable. Par exemple, le rafraîchissement d’un flux RSS dans une page n’est pas une limite de temps essentielle ; le critère est applicable. En revanche, une redirection automatique qui amène vers la nouvelle version d’une page à partir d’une url obsolète est essentielle ; le critère est non applicable.

  #### Cas particuliers

  Il existe une gestion de cas particuliers lorsque la limite de temps est essentielle, notamment lorsqu’elle ne pourrait pas être supprimée sans changer fondamentalement le contenu ou les fonctionnalités liées au contenu.

  Dans ces situations, le critère est non applicable. Par exemple, le rafraîchissement d’un flux RSS dans une page n’est pas une limite de temps essentielle ; le critère est applicable. En revanche, une redirection automatique qui amène vers la nouvelle version d’une page à partir d’une URL obsolète est essentielle ; le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.2.1 Timing Adjustable (A)
  - 9.2.2.2 Pause, Stop, Hide (A)

- ### 13.2 Dans chaque page web, l’ouverture d’une nouvelle fenêtre ne doit pas être déclenchée sans action de l’utilisateur. Cette règle est-elle respectée ? Critère 13.2

  #### 13.2.1

  Dans chaque page web, l’ouverture d’une nouvelle fenêtre ne doit pas être déclenchée sans action de l’utilisateur. Cette règle est-elle respectée ? Test 13.2.1

  - Vérifier qu’à l’ouverture du document, aucune nouvelle fenêtre (pop-up ou pop-under, par exemple) n’est ouverte.
  - Si c’est le cas, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.3.2.1 On focus (A)

- ### 13.3 Dans chaque page web, chaque document bureautique en téléchargement possède-t-il, si nécessaire, une version accessible (hors cas particuliers) ? Critère 13.3

  #### 13.3.1

  Dans chaque page web, chaque fonctionnalité de téléchargement d’un document bureautique vérifie-t-elle une de ces conditions ? Test 13.3.1

  - Le document en téléchargement est compatible avec l’accessibilité ;
  - Il en existe une version alternative en téléchargement compatible avec l’accessibilité ;
  - Il en existe une version alternative au format HTML compatible avec l’accessibilité.

  <!-- -->

  - Retrouver dans le document les liens et les contrôles de formulaire (un bouton de formulaire ou un formulaire de téléchargement par exemple) permettant de télécharger un fichier au format bureautique ;
  - Pour chaque fichier au format bureautique, vérifier la présence d’une version alternative présentée comme accessible :
    - Pour les documents au format .pdf, analyser le fichier avec l’outil PAC (PDF Accessibility Checker) et vérifier l’absence d’erreur d’accessibilité dans le document (cf. note) ;
    - Pour les documents au format .doc ou .docx, analyser le fichier avec l’outil de vérification d’accessibilité de Microsoft Office (à partir de la version 2010) et vérifier l’absence d’erreur d’accessibilité (cf. note) ;
    - Pour les documents au format .odt, analyser le document avec l’éditeur OpenOffice et vérifier que l’ensemble des contenus est conforme avec la liste des critères « Liste document bureautique en téléchargement » (cf. note pour une méthode alternative) ;
    - Pour les documents au format EPUB/DAISY, analyser le document avec un éditeur EPUB/DAISY et vérifier que l’ensemble des contenus est conforme avec la liste des critères « Liste document bureautique en téléchargement ».
    - Pour les documents eux-mêmes au format .html, analyser l’accessibilité du document.
  - Si c’est le cas pour chaque fichier au format bureautique, le test est validé.

  Note au sujet de l’outil PAC : l’outil analyse le document PDF du point de vue de l’accessibilité mais également de critères de qualité (par exemple la norme PDF/UA). Seules les erreurs relatives à des critères présents dans la liste des critères « Liste document bureautique en téléchargement » rendent le critère « Non conforme ». Par ailleurs, cet outil ne fonctionne que sur la plateforme Windows. Sur Mac, le contrôle doit se faire manuellement.

  Note au sujet Microsoft Office : le logiciel offre un vérificateur d’accessibilité en standard, (accessible via le menu « Fichier \> Informations \> Vérifier la présence de problèmes \> Vérifier l’accessibilité »). Ce vérificateur peut être considérablement amélioré via le plugin Word Accessibility Plug-in (voir dans la section Outils). Ce plugin ne fonctionne que sur Windows. Sur Mac, le contrôle doit se faire manuellement.

  Note au sujet des documents au format .odt : OpenOffice et LibreOffice ne possèdent pas de vérificateur d’accessibilité. Une méthode plus rapide qu’une analyse manuelle peut consister à enregistrer le document au format .docx et le vérifier via le vérificateur d’accessibilité de Microsoft Office 2010. Attention cependant : cette méthode rapide est à réserver aux documents très simples car certaines informations liées à l’accessibilité ne sont pas correctement transcodées. C’est le cas des indications de langue, de certaines alternatives d’images ou d’en-têtes fusionnées sur les tableaux par exemple.

  Note au sujet du format EPUB : l’utilitaire Ace by DAISY App permet d’effectuer le travail de validation d’un fichier EPUB 3 de manière efficace.

  Note au sujet des documents dérogés : le référentiel propose un statut de dérogation dans certains cas (cf. guide d’accompagnement). Dans ce cas, les tests ne sont pas à réaliser, la version accessible étant fournie sur demande de l’utilisateur.

  Note à l’attention des personnes de droit privé mentionnées aux 2° à 4° du I de l’article 47 de la loi du 11 février 2005 : si les fichiers bureautiques (ex : PDF, documents Microsoft ou LibreOffice, etc.) ont été publiés avant le 23 septembre 2018 (sauf si ce sont des documents nécessaires pour accomplir une démarche administrative relevant des tâches effectuées par l’organisme concerné), ils sont exemptés de l’obligation d’accessibilité.

  #### Cas particuliers

  Il existe une gestion de cas particuliers :

  - Pour les personnes de droit privé mentionnées aux 2° à 4° du I de l’article 47 de la loi du 11 février 2005 : si les fichiers bureautiques (ex : PDF, documents Microsoft ou LibreOffice, etc.) ont été publiés avant le 23 septembre 2018 (sauf si ce sont des documents nécessaires pour accomplir une démarche administrative relevant des tâches effectuées par l’organisme concerné), ils sont exemptés de l’obligation d’accessibilité.

  Dans cette situation, le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.1.3.1 Info and Relationships (A)
  - 9.1.3.2 Meaningful Sequence (A)
  - 9.2.4.1 Bypass Blocks (A)
  - 9.2.4.3 Focus Order (A)
  - 9.3.1.1 Language of Page (A)
  - 9.4.1.2 Name, Role, Value (A)

- ### 13.4 Pour chaque document bureautique ayant une version accessible, cette version offre-t-elle la même information ? Critère 13.4

  #### 13.4.1

  Chaque document bureautique ayant une version accessible vérifie-t-il une de ces conditions ? Test 13.4.1

  - La version compatible avec l’accessibilité offre la même information ;
  - La version alternative au format HTML est pertinente et offre la même information.

  <!-- -->

  - Retrouver dans le document les fichiers en téléchargement au format bureautique accompagné de leur version alternative accessible ;
  - Pour chaque couple de fichiers, ouvrir les deux documents (le document d’origine et le document accessible) et vérifier que les deux documents apportent la même information ;
  - Si c’est le cas pour chaque couple de fichiers, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)
  - 9.1.3.1 Info and Relationships (A)
  - 9.1.3.2 Meaningful Sequence (A)
  - 9.2.4.1 Bypass Blocks (A)
  - 9.2.4.3 Focus Order (A)
  - 9.3.1.1 Language of Page (A)
  - 9.4.1.2 Name, Role, Value (A)

- ### 13.5 Dans chaque page web, chaque contenu cryptique (art ASCII, émoticône, syntaxe cryptique) a-t-il une alternative ? Critère 13.5

  #### 13.5.1

  Dans chaque page web, chaque contenu cryptique (art ASCII, émoticône, syntaxe cryptique) vérifie-t-il une de ces conditions ? Test 13.5.1

  - Un attribut title est disponible ;
  - Une définition est donnée par le contexte adjacent.

  <!-- -->

  - Retrouver dans le document les contenus cryptiques (art ASCII, émoticône, syntaxe cryptique) ;
  - Pour chaque contenu cryptique, vérifier que :
    - Soit une définition est disponible au moyen d’un attribut

          title

      , sur un lien, un contrôle de formulaire, une abréviation (élément

          <abbr>

      ) par exemple ;

    - Soit une définition est donnée dans le contexte adjacent (immédiatement avant ou après).
  - Soit une définition est disponible au moyen d’un attribut
  - Si c’est le cas pour chaque contenu cryptique, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- ### 13.6 Dans chaque page web, pour chaque contenu cryptique (art ASCII, émoticône, syntaxe cryptique) ayant une alternative, cette alternative est-elle pertinente ? Critère 13.6

  #### 13.6.1

  Dans chaque page web, chaque contenu cryptique (art ASCII, émoticône, syntaxe cryptique) vérifie-t-il une de ces conditions ? Test 13.6.1

  - Le contenu de l’attribut

        title

    est pertinent ;

  - La définition donnée par le contexte adjacent est pertinente.

  <!-- -->

  - Retrouver dans le document les contenus cryptiques (art ASCII, émoticône, syntaxe cryptique) ;
  - Pour chaque contenu cryptique, vérifier que la définition donnée est pertinente.
  - Si c’est le cas pour chaque contenu cryptique, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.1.1 Non-text Content (A)

- Le contenu de l’attribut

- ### 13.7 Dans chaque page web, les changements brusques de luminosité ou les effets de flash sont-ils correctement utilisés ? Critère 13.7

  #### 13.7.1

  Dans chaque page web, chaque image ou élément multimédia (balise

      <video>

  , balise

      <img>

  , balise

      <svg>

  , balise

      <canvas>

  , balise

      <embed>

  ou balise

      <object>

  ) qui provoque un changement brusque de luminosité ou un effet de flash vérifie-t-il une de ces conditions ? Test 13.7.1

  - La fréquence de l’effet est inférieure à 3 par seconde ;
  - La surface totale cumulée des effets est inférieure ou égale à 21824 pixels.

  <!-- -->

  - Retrouver dans le document les contenus clignotants ou qui provoquent des effets de flash de type image animée, vidéo (cf. note) ou animation (éléments

        <img>

    ,

        <svg>

    ,

        <canvas>

    ,

        <embed>

    ,

        <object>

    ou

        <video>

    ) ;

  - Pour chaque contenu clignotant ou provoquant des effets de flash, vérifier que :
    - Soit la fréquence de l’effet est inférieur à 3 par seconde ;
    - Soit la surface cumulée est inférieure à 21824 pixels.

  - Si c’est le cas pour chaque contenu clignotant ou provoquant des effets de flash, le test est validé.

  Note : l’évaluation de ce critère peut être complexe. Lorsque l’effet est géré par un script ou au moyen de CSS, l’analyse du code est suffisante. L’outil PEAT permet d’analyser les vidéos au format .avi, par exemple. Un exemple de vidéo ayant provoqué des crises d’épilepsie peut être consulté ici : London 2012 Olympics Seizure (https://www.youtube.com/watch?v=vs0hfhSje9M).

  #### 13.7.2

  Dans chaque page web, chaque script qui provoque un changement brusque de luminosité ou un effet de flash vérifie-t-il une de ces conditions ? Test 13.7.2

  - La fréquence de l’effet est inférieure à 3 par seconde ;
  - La surface totale cumulée des effets est inférieure ou égale à 21824 pixels.

  <!-- -->

  - Retrouver dans le document les contenus clignotants ou qui provoquent des effets de flash obtenus au moyen d’un script ;
  - Pour chaque contenu clignotant ou provoquant des effets de flash, vérifier que :
    - Soit la fréquence de l’effet est inférieur à 3 par seconde ;
    - Soit la surface cumulée est inférieure à 21824 pixels.
  - Si c’est le cas pour chaque contenu clignotant ou provoquant des effets de flash, le test est validé.

  #### 13.7.3

  Dans chaque page web, chaque mise en forme CSS qui provoque un changement brusque de luminosité ou un effet de flash vérifie-t-il une de ces conditions ? Test 13.7.3

  - La fréquence de l’effet est inférieure à 3 par seconde ;
  - La surface totale cumulée des effets est inférieure ou égale à 21824 pixels.

  <!-- -->

  - Retrouver dans le document les contenus clignotants ou qui provoquent des effets de flash obtenus au moyen d’une animation CSS ;
  - Pour chaque contenu clignotant ou provoquant des effets de flash, vérifier que :
    - Soit la fréquence de l’effet est inférieur à 3 par seconde ;
    - Soit la surface cumulée est inférieure à 21824 pixels.
  - Si c’est le cas pour chaque contenu clignotant ou provoquant des effets de flash, le test est validé.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.3.1 Three Flashes or Below Threshold (A)

- ### 13.8 Dans chaque page web, chaque contenu en mouvement ou clignotant est-il contrôlable par l’utilisateur ? Critère 13.8

  #### 13.8.1

  Dans chaque page web, chaque contenu en mouvement déclenché automatiquement, vérifie-t-il une de ces conditions ? Test 13.8.1

  - La durée du mouvement est inférieure ou égale à 5 secondes ;
  - L’utilisateur peut arrêter et relancer le mouvement ;
  - L’utilisateur peut afficher et masquer le contenu en mouvement ;
  - L’utilisateur peut afficher la totalité de l’information sans le mouvement.

  <!-- -->

  - Retrouver dans le document les contenus en mouvement (obtenus au moyen d’une image, d’un script ou d’un effet CSS) déclenchés automatiquement au chargement de la page ou lors de l’affichage d’un contenu (cf. note) ;
  - Pour chaque contenu, vérifier que :
    - Soit la durée du mouvement est inférieure à 5 secondes ;
    - Soit la présence d’un mécanisme (un bouton, par exemple) permet d’arrêter et de relancer le mouvement ;
    - Soit la présence d’un mécanisme (un bouton, par exemple) permet de cacher et d’afficher à nouveau le contenu en mouvement ;
    - Soit la présence d’un mécanisme (un bouton, par exemple) permet d’afficher la totalité du contenu sans mouvement.
  - Si c’est le cas pour chaque contenu en mouvement, le test est validé.

  #### 13.8.2

  Dans chaque page web, chaque contenu clignotant déclenché automatiquement, vérifie-t-il une de ces conditions ? Test 13.8.2

  - La durée du clignotement est inférieure ou égale à 5 secondes ;
  - L’utilisateur peut arrêter et relancer le clignotement ;
  - L’utilisateur peut afficher et masquer le contenu clignotant ;
  - L’utilisateur peut afficher la totalité de l’information sans le clignotement.

  <!-- -->

  - Retrouver dans le document les contenus clignotants (obtenus au moyen d’une image, d’un script ou d’un effet CSS) déclenchés automatiquement au chargement de la page ou lors de l’affichage d’un contenu (cf. note).
  - Pour chaque contenu, vérifier que :
    - Soit la durée du clignotement est inférieure à 5 secondes ;
    - Soit la présence d’un mécanisme (un bouton, par exemple) permet d’arrêter et de relancer le clignotement ;
    - Soit la présence d’un mécanisme (un bouton, par exemple) permet de cacher et d’afficher à nouveau le contenu clignotant ;
    - Soit la présence d’un mécanisme (un bouton, par exemple) permet d’afficher la totalité du contenu clignotant.
  - Si c’est le cas pour chaque contenu clignotant, le test est validé.

  Note : l’arrêt ou la mise en pause d’un contenu en mouvement ou clignotant au moyen de la prise de focus (par exemple, l’effet est suspendu uniquement pendant la prise de focus) n’est pas considéré comme un procédé conforme. Dans certains cas, le mouvement ne peut pas être arrêté, par exemple dans le cas d’une barre de progression, dans ce cas, le critère est non applicable.

  #### Références

  ##### WCAG 2.1

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.2.1 Timing Adjustable (A)
  - 9.2.2.2 Pause, Stop, Hide (A)

- ### 13.9 Dans chaque page web, le contenu proposé est-il consultable quelle que soit l’orientation de l’écran (portrait ou paysage) (hors cas particuliers) ? Critère 13.9

  #### 13.9.1

  Dans chaque page web, chaque contenu vérifie-t-il ces conditions (hors cas particuliers) ? Test 13.9.1

  - La consultation est possible quel que soit le mode d’orientation de l’écran ;
  - Le contenu proposé reste le même quel que soit le mode d’orientation de l’écran utilisé même si sa présentation et le moyen d’y accéder peut différer.

  <!-- -->

  - Consulter le document dans un mode d’orientation portrait puis dans un mode d’orientation paysage ;
  - Vérifier que :
    - La consultation est possible quel que soit le mode d’orientation de l’écran.
    - Le contenu proposé reste le même quel que soit le mode d’orientation de l’écran utilisé même si sa présentation et le moyen d’y accéder peut différer.
  - Si c’est le cas, le test est validé.

  Note : il existe des interfaces pour lesquelles l’orientation du périphérique est essentielle à leur utilisation. Dans ces situations, le critère est non applicable. Il peut s’agir d’interfaces de jeu, de piano, de dépôt de chèques bancaires, etc. Si l’interface est le seul moyen d’accéder au service proposé, une alternative devrait être mise en place pour pallier cette carence.

  #### Cas particuliers

  Il existe des interfaces pour lesquelles l’orientation du périphérique est essentielle à leur utilisation.

  Dans ces situations, le critère est non applicable. Il peut s’agir d’interfaces de jeu, de piano, de dépôt de chèques bancaires, etc.

  Si l’interface est le seul moyen d’accéder au service proposé, une alternative devrait être mise en place pour pallier cette carence.

  #### Références documentaires

  - API JS : https://www.w3.org/TR/screen-orientation/
  - API Viewport : https://www.w3.org/TR/css-device-adapt-1/

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.1.3.4 Orientation (AA)

- ### 13.10 Dans chaque page web, les fonctionnalités utilisables ou disponibles au moyen d’un geste complexe peuvent-elles être également disponibles au moyen d’un geste simple (hors cas particuliers) ? Critère 13.10

  #### 13.10.1

  Dans chaque page web, chaque fonctionnalité utilisable ou disponible suite à un contact multipoint est-elle également utilisable ou disponible suite à un contact en un point unique de l’écran (hors cas particuliers). Test 13.10.1

  - Retrouver dans le document les fonctionnalités utilisables ou disponibles au moyen d’une interaction au toucher de type contact multipoint ;
  - Pour chaque fonctionnalité, vérifier qu’elle reste disponible au moyen d’une interaction au toucher de type contact en un point unique de l’écran (par exemple, la possibilité de consulter les éléments d’une liste par un mouvement de balayage horizontal droit ou gauche doit aussi être disponible au moyen de boutons “précédent” et “suivant” ou encore un geste de pincer et zoomer qui peut être alternativement réalisé au moyen de boutons “plus” et “moins”) ;
  - Si c’est le cas pour chaque fonctionnalité utilisable ou disponible au moyen d’une interaction au toucher de type contact multipoint, le test est validé.

  #### 13.10.2

  Dans chaque page web, chaque fonctionnalité utilisable ou disponible suite à un geste basé sur le suivi d’une trajectoire sur l’écran est-elle également utilisable ou disponible suite à un contact en un point unique de l’écran (hors cas particuliers). Test 13.10.2

  - Retrouver dans le document les fonctionnalités utilisables ou disponibles au moyen d’une interaction au toucher qui implique le suivi d’une trajectoire sur l’écran ;
  - Pour chaque fonctionnalité, vérifier qu’elle reste disponible au moyen d’une interaction au toucher de type contact en un point unique de l’écran (par exemple, la possibilité de composer son mot de passe en suivant une trajectoire sur un clavier virtuel doit aussi être disponible au moyen de pressions successives sur les touches du clavier) ;
  - Si c’est le cas pour chaque fonctionnalité utilisable ou disponible au moyen d’une interaction au toucher qui implique le suivi d’une trajectoire sur l’écran, le test est validé.

  Il existe une gestion de cas particuliers dans deux types de situation :

  - Le critère ne s’applique qu’à des fonctionnalités mises en place par l’auteur du site. Il ne concerne donc pas les gestes requis par l’agent utilisateur ou le système d’exploitation.
  - Le critère ne s’applique pas aux fonctionnalités dont la réalisation d’un geste complexe est essentielle (exécuter le tracé d’une signature, par exemple).

  #### Cas particuliers

  Il existe une gestion de cas particuliers dans deux types de situation :

  - Le critère ne s’applique qu’à des fonctionnalités mises en place par l’auteur du site. Il ne concerne donc pas les gestes requis par l’agent utilisateur ou le système d’exploitation ;
  - Le critère ne s’applique pas aux fonctionnalités dont la réalisation d’un geste complexe est essentielle (exécuter le tracé d’une signature, par exemple).

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ###### Technique(s) suffisante(s) et/ou échec(s) (en anglais) :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.5.1 Pointer Gestures (A)

- ### 13.11 Dans chaque page web, les actions déclenchées au moyen d’un dispositif de pointage sur un point unique de l’écran peuvent-elles faire l’objet d’une annulation (hors cas particuliers) ? Critère 13.11

  #### 13.11.1

  Dans chaque page web, les actions déclenchées au moyen d’un dispositif de pointage sur un point unique de l’écran vérifient-elles l’une de ces conditions (hors cas particuliers) ? Test 13.11.1

  - L’action est déclenchée au moment où le dispositif de pointage est relâché ou relevé ;
  - L’action est déclenchée au moment où le dispositif de pointage est pressé ou posé puis annulée lorsque le dispositif de pointage est relâché ou relevé ;
  - Un mécanisme est disponible pour abandonner (avant achèvement de l’action) ou annuler (après achèvement) l’exécution de l’action.

  <!-- -->

  - Retrouver dans le document les actions déclenchées au moyen d’un dispositif de pointage sur un point unique de l’écran ;
  - Pour chaque action, vérifier que :
    - Soit l’action est déclenchée au moment où le dispositif de pointage est relâché ou relevé ;
    - Soit l’action est déclenchée au moment où le dispositif de pointage est pressé ou posé puis annulée lorsque le dispositif de pointage est relâché ou relevé ;
    - Soit il existe un mécanisme pour abandonner (avant achèvement de l’action) ou annuler (après achèvement) l’exécution de l’action ; par exemple, lors d’une interaction de type glisser-déposer un relâchement du dispositif de pointage doit permettre d’abandonner l’interaction en cours et une fois la zone de dépôt atteinte, l’utilisateur doit rester en mesure d’annuler son opération de dépôt au moyen d’un dialogue de confirmation (choix de confirmer ou d’annuler le dépôt) ou par le fait de pouvoir replacer l’élément déposé à sa position initiale.
  - Si c’est le cas pour chaque action déclenchée au moyen d’un dispositif de pointage sur un point unique de l’écran, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particulier lorsque la fonctionnalité nécessite que le comportement attendu soit réalisé lors d’un événement descendant, par exemple, un émulateur de clavier dont les touches doivent s’activer à la pression comme sur un clavier physique. Dans ces situations, le critère est non applicable.

  #### Notes techniques

  Deux exemples de mécanisme mis en place pour annuler ou abandonner une action déclenchée au moyen d’un dispositif de pointage sur un point unique de l’écran :

  - Une fenêtre modale permettant d’annuler l’action après son achèvement ;
  - Pour une fonction de glisser/déposer, le fait d’abandonner l’action si l’utilisateur relâche l’élément en dehors de la zone cible.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.5.2 Pointer Cancellation (A)

- ### 13.12 Dans chaque page web, les fonctionnalités qui impliquent un mouvement de l’appareil ou vers l’appareil peuvent-elles être satisfaites de manière alternative (hors cas particuliers) ? Critère 13.12

  #### 13.12.1

  Dans chaque page web, les fonctionnalités disponibles en bougeant l’appareil peuvent-elles être accomplies avec des composants d’interface utilisateur (hors cas particuliers) ? Test 13.12.1

  - Retrouver dans le document les fonctionnalités disponibles en bougeant l’appareil ;
  - Pour chaque fonctionnalité, vérifier qu’elle peut être accomplie au moyen de composants d’interface utilisateur ;
  - Si c’est le cas pour chaque fonctionnalité disponible en bougeant l’appareil, le test est validé.

  #### 13.12.2

  Dans chaque page web, les fonctionnalités disponibles en faisant un geste en direction de l’appareil peuvent-elles être accomplies avec des composants d’interface utilisateur (hors cas particuliers) ? Test 13.12.2

  - Retrouver dans le document les fonctionnalités disponibles en faisant un geste en direction de l’appareil ;
  - Pour chaque fonctionnalité, vérifier qu’elle peut être accomplie au moyen de composants d’interface utilisateur ;
  - Si c’est le cas pour chaque fonctionnalité disponible en faisant un geste en direction de l’appareil, le test est validé.

  #### 13.12.3

  L’utilisateur a-t-il la possibilité de désactiver la détection du mouvement pour éviter un déclenchement accidentel de la fonctionnalité (hors cas particuliers) ? Test 13.12.3

  - Retrouver dans le document les fonctionnalités disponibles en mettant en mouvement l’appareil ;
  - Vérifier si l’utilisateur à la possibilité de désactiver la détection du mouvement ;
  - Si c’est le cas, pour chaque fonctionnalité, vérifier qu’elle ne peut pas être déclenchée ;
  - Si c’est le cas pour chaque fonctionnalité disponible en mettant en mouvement l’appareil, le test est validé.

  #### Cas particuliers

  Il existe une gestion de cas particulier lorsque :

  - Le mouvement est essentiel à l’accomplissement de la fonctionnalité (ex. podomètre) ;
  - La détection du mouvement est utilisée pour contrôler une fonctionnalité au travers d’une interface compatible avec l’accessibilité.

  #### Références

  ##### WCAG 2.1

  ###### Critère(s) de succès :

  ##### EN 301 549 V2.1.2 (2018-08) (en anglais)

  - 9.2.5.4 Motion Actuation (A)
