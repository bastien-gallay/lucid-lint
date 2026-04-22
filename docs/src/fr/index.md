<svg class="lucid-landing__mark" viewBox="0 0 120 36" aria-hidden="true" focusable="false">
  <title>loupe lucid-lint</title>
  <circle cx="18" cy="18" r="12" fill="none" stroke="currentColor" stroke-width="2.25"/>
  <circle cx="18" cy="18" r="2" fill="currentColor"/>
  <line class="lucid-landing__mark-line" x1="32" y1="18" x2="116" y2="18" stroke-width="1.75" stroke-linecap="round"/>
</svg>

<h1 id="introduction" class="lucid-landing__title">Introduction</h1>

<p class="lucid-audience" role="doc-subtitle">
Conçu pour les lecteurs dont l'attention est sollicitée — TDAH,
dyslexie, fatigue, langue seconde, ou contexte d'accessibilité.
</p>

`lucid-lint` lit votre Markdown ou texte brut et repère les passages
qui alourdissent la lecture. Il ne réécrit pas votre voix. Il vous
tend une liste courte, puis s'efface.

<figure class="lucid-stance" aria-label="Avant et après : une phrase signalée par lucid-lint">
  <div class="lucid-stance__pair">
    <div class="lucid-stance__side" data-stance-side="before">
      <p class="lucid-stance__label">Avant</p>
      <p class="lucid-stance__prose">
        <span class="lucid-stance__idea" data-idea="1">Le sous-système de cache, introduit lors d'un jalon antérieur,</span>
        <span class="lucid-stance__idea" data-idea="2">s'est révélé mal interagir avec la nouvelle chaîne de traitement des requêtes sous charge soutenue,</span>
        et
        <span class="lucid-stance__idea" data-idea="3">l'enquête qui a suivi a exigé plusieurs rondes de profilage.</span>
      </p>
    </div>
    <div class="lucid-stance__side" data-stance-side="after">
      <p class="lucid-stance__label">Après</p>
      <p class="lucid-stance__prose">
        <span class="lucid-stance__idea" data-idea="1">Le sous-système de cache a été introduit plus tôt.</span>
        <span class="lucid-stance__idea" data-idea="2">Il interagit mal avec la nouvelle chaîne de traitement des requêtes sous charge soutenue.</span>
        <span class="lucid-stance__idea" data-idea="3">L'enquête a exigé plusieurs rondes de profilage.</span>
      </p>
    </div>
  </div>
  <figcaption class="lucid-stance__caption">
    Trois idées, teintées de la même couleur à gauche et à droite — la
    réécriture raccourcit les phrases sans en perdre une seule.
    <code>lucid-lint</code> a signalé <code>sentence-too-long</code>
    (43 mots) et <code>consecutive-long-sentences</code>. Il n'a pas
    proposé la réécriture — elle est de vous.
  </figcaption>
</figure>

## Ce qui le distingue

La plupart des outils mesurent le style (`write-good`), la grammaire
(`Antidote`) ou un score de lisibilité de surface (Flesch).
`lucid-lint` mesure la **charge cognitive** — l'effort mental qu'un
lecteur dépense pour comprendre une phrase. Il repère les motifs que
la recherche de Sweller, Gibson, Graesser et
[Coh-Metrix](http://cohmetrix.com) ont isolés.

- **Bilingue EN/FR** dès le premier jour, à qualité égale.
- **Déterministe** par défaut. Une même entrée produit une même
  sortie. Les règles fondées sur un LLM vivent dans des extensions
  optionnelles.
- **Pensé pour l'intégration continue.** Sorties texte et JSON ; codes
  de retour que pre-commit et GitHub Actions comprennent sans
  adaptateur.
- **Par profil.** Choisissez `dev-doc`, `public` ou `falc` (Facile À
  Lire et à Comprendre), puis ajustez chaque règle si besoin.

## État du projet

`lucid-lint` est en v0.2. Les 17 règles listées dans
[`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md)
ont été livrées en v0.1. La v0.2 ajoute le
[modèle de score hybride](../guide/scoring.md) — un score global
`X / max` accompagné de cinq sous-scores par catégorie, calculé
au-dessus des diagnostics existants. La [feuille de route](./roadmap.md)
indique la suite.

## Aperçu

```bash
cargo install lucid-lint

# Analyser un fichier
lucid-lint check README.md

# Profil le plus strict (FALC)
lucid-lint check --profile=falc docs/

# Entrée standard
echo "Ceci est une phrase de test." | lucid-lint check -

# JSON pour la CI
lucid-lint check --format=json docs/

# Échouer la build si le score global passe sous 85/100 (v0.2+)
lucid-lint check --min-score=85 docs/
```

## Pour aller plus loin

- [Référence des règles](./rules-index.md) — les 25 règles livrées.
- [Accessibilité](./accessibility.md) — l'exigence WCAG AAA et comment
  le site lui-même met en pratique ce qu'il prêche.
- [Feuille de route](./roadmap.md) — ce qui vient ensuite.
- Les pages de guide (installation, démarrage rapide, profils) sont
  pour l'instant en anglais. Elles seront traduites au fil des
  prochains jalons — suivi dans **F25** sur la feuille de route.

## Préférences de lecture

<section class="reading-demo reading-demo--chips" aria-label="Sélecteur de police de lecture">
  <p class="reading-demo__note">
    Tout le site est conçu comme un compagnon de lecture. Choisissez
    la police qui vous convient le mieux — elle sera mémorisée entre
    les pages.
  </p>
  <article class="reading-demo__preview">
    <p class="reading-demo__label" data-chip-label>Atkinson Hyperlegible Next</p>
    <p class="reading-demo__sample" data-demo="atkinson">
      Un paragraphe dense peut beaucoup demander à un esprit sollicité.
      Chaque virgule, chaque proposition, chaque parenthèse ajoute
      son coût. Une bonne prose maintient ce coût bas.
    </p>
  </article>
  <div class="reading-demo__chips" role="radiogroup" aria-label="Police de lecture">
    <button type="button" class="reading-demo__chip" data-apply="atkinson" role="radio" aria-checked="true">
      Atkinson <span class="reading-demo__default">défaut</span>
    </button>
    <button type="button" class="reading-demo__chip" data-apply="standard" role="radio" aria-checked="false">
      Standard
    </button>
    <button type="button" class="reading-demo__chip" data-apply="dyslexic" role="radio" aria-checked="false">
      OpenDyslexic
    </button>
  </div>
  <p class="reading-demo__caption">
    L'interligne et la taille du texte arriveront bientôt sous forme de
    curseurs. En attendant, choisissez une police et le zoom du
    navigateur est respecté.
  </p>
</section>

## Version anglaise

- [Return to the English version](../introduction.html)

## Licence

<!-- lucid-lint disable-next-line unexplained-abbreviation -->

Double licence MIT ou Apache-2.0, à votre choix.
