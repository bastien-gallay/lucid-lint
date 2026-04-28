# Changelog

All notable changes to `lucid-lint` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Encoding hygiene at the engine boundary (F110 / F111 / F112).**
  `Engine::lint_with_source` now strips a leading UTF-8 BOM
  (`U+FEFF`) and NFC-normalizes input once at the funnel point, so
  every rule consumes the same shape of text. F110: a Windows-edited
  Markdown file with `EF BB BF` no longer shifts column counts by one
  on the first character. F111: NFC `café` and NFD `café`
  (`e + U+0301`) now hash to the same key in HashMap-using rules
  (e.g. `low-lexical-diversity`, the stop-words detector); same prose
  → same lint output. Adds `unicode-normalization = "0.1"` and uses
  `is_nfc_quick` for the zero-cost path on already-normalized input.
  F112: regression fixtures for bare `\r` line endings (classic Mac)
  and zero-width characters (`U+200B/200C/200D`) inside words pin the
  observed behaviour so it can't drift.
- **Three US-federal public-domain ADHD sources (F84 part 2 progress).**
  `examples/texts.yaml` gains the NIMH ADHD topic page (`mixed` shape,
  ~780 words), CDC About ADHD (`good`, ~920 words) and CDC Treatment of
  ADHD (`good`, ~1040 words). All three are public-domain US federal
  works under 17 USC § 105 with explicit reproduction notices on the
  agencies' policy pages — quoted verbatim in each entry's
  `license_details`. Material lands under `examples/public/en/` after
  `just texts`. Knock-on on the public coverage snapshot in
  `examples/texts.md`: `condition adhd × EN` lifts to `3` (was `—`),
  `gov_guide × EN` rises to `6`, and the gitignored load-bearing list
  drops the `adhd × EN` slot entirely. Remaining load-bearing slots are
  `dyscalculia × EN` and `aphasia × EN+FR` — both flagged as harder in
  the existing `gaps:` notes; tackled in a later lap.

- **Strict validation of the `unexplained-abbreviation` whitelist at
  config-load.** `[rules."lexicon.unexplained-abbreviation"].whitelist`
  entries must now be non-empty strings of ASCII uppercase letters and
  digits (e.g. `"WCAG"`, `"HTML5"`). The detector only ever emits
  uppercase+digit acronym tokens, so previously a typo like `"wcag"` or
  `"Wcag"` would silently never match. The error message names the
  offending entry + its index so the fix is one grep away. Digits
  inside entries remain supported (`"WCAG21"`).
- **Parser / engine micro-benchmarks (`benches/parser_hotpath.rs`).**
  New `criterion` dev-dep and `just bench` recipe cover
  `split_sentences`, `parse_markdown`, and `Engine::lint_str` over two
  real tracked corpus files. Gives us a defensible baseline before
  touching hot-path code — the first exploratory rewrite of the
  `split_sentences` buffer-reuse pattern came back 48% slower, so the
  harness already earned its keep by catching the regression before it
  shipped. Parked the counter-finding on `ROADMAP.md` F93 (tokenizer
  `Vec<char>` alloc, measured at ~5% ceiling — deferred).
- **`engine::replace_rule` helper (internal).** The three `with_*`
  config-override builders
  (`with_readability_formula`, `with_unexplained_whitelist`,
  `with_excessive_commas_max_commas`) collapsed from three duplicated
  find-and-replace loops into a single named helper. No behaviour
  change; three new regression tests lock in the tightening path,
  whitelist path, and filtered-out no-op contract.
- **Real-world corpus regression anchors.** Three short passages
  lifted verbatim from `examples/public/` land under
  `tests/corpus/public/`: a GOV.UK plain-language exemplar
  (`public`-profile clean), the plainlanguage.gov intro
  (`public` flags `structure.sentence-too-long`; `falc` also flags
  `syntax.passive-voice`), and a Vikidia `Accueil` passage
  (`falc`-profile clean). New `tests/cli.rs` tests shell out with
  `--format=json`, parse the diagnostics, and pin each expectation
  so a silent rule-tuning regression on curated real-world prose
  fails loudly. Machinery reuses the existing `corpus_path` helper
  — no new fixture format.
- **Four more FR per-rule pages (F25 progress, 1/25 → 5/25).**
  `structure.excessive-commas`, `structure.long-enumeration`,
  `lexicon.weasel-words`, `lexicon.unexplained-abbreviation` land
  under `docs/src/fr/rules/` using the locked template from
  `sentence-too-long` (sections `Ce que cette règle signale` /
  `En bref` / `Détection` / `Paramètres` / `Exemples` /
  `Faux positifs connus` / `Neutralisation` / `Voir aussi`,
  FR-first example ordering when bilingual examples exist).
  `SUMMARY.md` and `fr/rules/index.md` now link these pages
  locally instead of pointing back to the EN versions. Guides
  (`suppression`, `configuration`, `scoring`) still cross-link
  via `../../` to EN until FR guide translations land. Remaining:
  20 per-rule FR pages + FR guide translations.
- **Six more FR per-rule pages — `structure` category 100 %
  FR-complete (F25 progress, 5/25 → 11/25).**
  `structure.paragraph-too-long`, `structure.line-length-wide`,
  `structure.mixed-numeric-format`, `structure.deeply-nested-lists`,
  `structure.heading-jump`, `structure.deep-subordination` all land
  under `docs/src/fr/rules/` against the same locked template. The
  whole `structure` category is now FR-complete (9 / 9 rules
  translated). `SUMMARY.md` and `fr/rules/index.md` rewired to point
  at the local FR versions for the six new pages. Remaining: 14
  per-rule FR pages (rhythm 2, lexicon 6, syntax 5, readability 1)
  plus the FR guide translations.
- **F123 — install-route documentation surfaces the cargo-dist
  installers.** `README.md` and `docs/src/guide/installation.md`
  now lead with the `curl … | sh` (Linux / macOS / WSL) and
  PowerShell (`irm | iex`) one-liners that ship with every
  release, document the `--check` / audit-before-running pattern
  (download to a file, `less`/`notepad`, then execute), explain
  how to pin a specific version (`releases/download/v<version>/`
  vs `releases/latest/`), and keep `cargo install` and source-build
  routes alongside as fallbacks. Stale "Once released to crates.io"
  lead-in dropped. The cargo-dist `installers = ["shell",
  "powershell"]` flip itself was a no-op — that config has been
  in `Cargo.toml` `[workspace.metadata.dist]` since the initial
  scaffold; v0.1.1 / v0.2.0 / v0.2.1 / v0.2.2 have all been
  attaching the installer scripts to their GitHub Releases.
- **`lexicon.jargon-undefined` page polish (EN + FR).** Drops the
  obsolete `(v0.1 simplified)` section qualifier, removes the stale
  link to F9 (which shipped a different feature — definition-aware
  acronym detection — not a definition-aware variant of jargon),
  and replaces the inaccurate Parameters table (which suggested
  `jargon_lists`/`custom_jargon`/`whitelist` were TOML-configurable)
  with an honest Configuration section: in v0.2 the active lists
  are set by the profile and not yet user-overridable from
  `lucid-lint.toml`, with the gap tracked as **F126**.
- **`lexicon.all-caps-shouting` shape-cue explainer (EN + FR).**
  The "ascenders / descenders / x-height contrast" parenthetical
  expands into a teaching paragraph naming representative letters
  for each cue and explaining why ALL-CAPS flattens the silhouette
  — closes the implicit-jargon gap surfaced in the
  FR-translation review.
- **`lexicon.excessive-nominalization` example highlights
  (EN + FR).** Nominalizations in the "before" line and matching
  active verbs in the "after" line now wear `lucid-idea` colour
  spans (1–4) so the rewrite mapping is visible at a glance,
  matching the convention `lexicon.all-caps-shouting` already
  uses.
- **`lexicon.low-lexical-diversity` reference cleanup
  (EN + FR).** Drops the inline `**Reference.** Type-Token Ratio …
  (Herdan, 1960)` line from the body — the reference is already
  in the page's References section. (A full-rules pass to
  normalise this convention is pending.)
- **F126 filed — TOML overrides for `lexicon.jargon-undefined`.**
  v0.2 has no `[rules."lexicon.jargon-undefined"]` deserializer;
  the rule's `Config` struct exposes `active_lists` / `custom` /
  `whitelist` but they're only set through the profile preset.
  Filed under "Rules refinement" 🟡 Later — same shape as the
  existing `unexplained-abbreviation` whitelist wiring.
- **Six more FR per-rule pages — `lexicon` category 100 %
  FR-complete (F25 progress, 13/25 → 19/25).**
  `lexicon.low-lexical-diversity`, `lexicon.excessive-nominalization`,
  `lexicon.jargon-undefined`, `lexicon.all-caps-shouting`,
  `lexicon.redundant-intensifier`, `lexicon.consonant-cluster` land
  under `docs/src/fr/rules/` against the locked template. The whole
  `lexicon` category is now FR-complete (8 / 8 rules translated),
  joining `structure` and `rhythm` at 100 %. `SUMMARY.md` and
  `fr/rules/index.md` rewired to local FR versions. Remaining: 6
  per-rule FR pages (syntax 5, readability 1) plus FR guide
  translations.
- **Two more FR per-rule pages — `rhythm` category 100 %
  FR-complete (F25 progress, 11/25 → 13/25).**
  `rhythm.consecutive-long-sentences` and `rhythm.repetitive-connectors`
  land under `docs/src/fr/rules/` against the locked template. The
  whole `rhythm` category is now FR-complete (2 / 2 rules
  translated), joining `structure` at 100 %. Both EN pages were
  brought up to canonical shape first (Examples + See also added to
  `consecutive-long-sentences`; FR Examples block added to
  `repetitive-connectors`). `SUMMARY.md` and `fr/rules/index.md`
  rewired to local FR versions. Remaining: 12 per-rule FR pages
  (lexicon 6, syntax 5, readability 1) plus FR guide translations.
- **Per-category sidebar grouping in `SUMMARY.md` (F104).** The
  rules sidebar is reshaped into 5 collapsible sub-trees
  (Structure / Rhythm / Lexicon / Syntax / Readability) using mdBook
  draft-chapter syntax (`- [Title]()`) as non-clickable group
  headers. The flat 25-row list under "Overview" is replaced by
  category groups that mirror the per-category table on
  `rules/index.md` — readers scanning for a rhythm or readability
  rule no longer wade past 9 structure entries first. The
  "Version française" block mirrors the same shape (Structure /
  Rythme / Lexique; Syntaxe and Lisibilité materialise as the FR
  translations of those categories land). `markdownlint` MD042
  (no-empty-links) is disabled globally to permit the draft-chapter
  syntax — same pattern as the pre-existing MD025 carve-out for
  SUMMARY's required multiple H1s.
- **FR rule labels — page subtitle + index gloss (F107).** The rule
  ID stays the stable contract (CLI flags, JSON output, config keys,
  citations), but FR readers now see a human-readable label in two
  places. Each shipped FR rule page opens with a short italic gloss
  directly under the H1 (e.g. `*Phrase trop longue.*`); 13 pages
  updated, the remaining 12 receive theirs alongside translation.
  `fr/rules/index.md` "Catégories" block reshaped from a single
  roll-up into 5 per-category sub-tables (Structure / Rythme /
  Lexique / Syntaxe / Lisibilité), each with a `Règle | Libellé`
  two-column layout — all 25 rules now carry their FR label even
  when the page still points to the EN version (marked `(en)`).
  Sidebar TOC labels stay in EN to keep `SUMMARY.md` single-locale
  (per-locale split is F90, parked Speculative).
- **References page cross-links + external URLs (F105b).** Per-citation
  anchors (`<a id="author-year">`) on every entry of `docs/src/references.md`
  and `docs/src/fr/references.md`, so each rule page now links to the
  exact citation rather than scrolling. Rule pages gain a `## References`
  / `## Références` section listing the relevant citations as anchored
  links back into the references page (25 EN + 13 FR pages). The
  references page links rule IDs in `→ Relevant to:` lines and the
  rule → reference summary table to their per-rule mdBook pages,
  closing the loop in both directions. Verified canonical URLs (DOI,
  publisher landing page, official archive) are added inline as raw
  HTML anchors with `rel="nofollow noopener noreferrer" target="_blank"`
  — `nofollow` so the docs site does not vouch for outside content,
  `noopener noreferrer` so the new tab is safe from reverse-tabnabbing.
  Sources without a verifiable canonical URL (Herdan 1960, Quirk 1985,
  Strunk & White 1999, Bringhurst 2013, Zinsser 2006, Kandel & Moles
  1958, Inclusion Europe FALC, Nielsen Norman Group all-caps articles)
  are kept as text-only entries; we prefer no link to a guessed one.
  External-link visible text reduced to a single `↗` (the repeated
  "doi.org" / "doi.org" / "doi.org" cadence read as visual noise);
  the hostname moves to `aria-label` for screen readers and the link
  carries `class="ref-link"` so a small CSS rule in `lucid-colors.css`
  mutes it (smaller, fog-coloured, underline only on hover) — it sits
  next to its citation without competing with the rule-page links.
  Caveat phrasing unified: EN page now uses "Caveat" everywhere
  ("Note" / "Honest caveat" merged in); FR uses "Précaution" for the
  generic uncertainty marker, while keeping "Rectification"
  (factual correction, distinct) and "À vérifier" (pending
  verification, distinct).
  Subsumes the F30 rule-mention linking pass for the references-page
  surface; wider F30 audit (rule mentions in `docs/src/guide/*` prose
  pages) stays open.
- **Consolidated references page (F105).** Every academic, normative,
  and practical source that grounds a rule now lives in the docs
  site at `docs/src/references.md` (EN, under Project) and
  `docs/src/fr/references.md` (FR, under Version française). The
  page preserves the full taxonomy from `examples/REFERENCES.md`:
  status legend, per-domain sections (Cognitive Load Theory, text
  cohesion, syntactic complexity, discourse connectors, readability
  formulas, lexical diversity, negation processing, conditional
  reasoning, typography, phonological complexity, intensifiers,
  style guides, numeric formatting, working memory, dyslexia,
  WCAG/RGAA/FALC/CAN-ASC/EAA normative standards, practical tools)
  and the rule → reference summary table. `examples/REFERENCES.md`
  becomes a thin redirect to the docs sources — kept because external
  citations may already point there. Both rule indexes cross-link
  to the new page next to the existing `RULES.md` pointer.
- **`cargo-mutants` baseline + `just mutants` recipe (F98 ✅).**
  Mutation testing wired in as a dev-tool (no new runtime dep).
  `just mutants <file>` runs `cargo mutants --file <file> --timeout 60
  --no-shuffle`; default file is the canonical reference rule
  (`src/rules/structure/sentence_too_long.rs`). Four-file probe run
  on 2026-04-25 — `sentence_too_long.rs` 6 / 0 / 4 (caught / missed /
  unviable, 100 % score), `scoring.rs` 18 / 0 / 2, `engine.rs`
  5 / 0 / 12, `low_lexical_diversity.rs` 29 / 47 / 5 (36 % score).
  The canonical rule and the cross-cutting layer (engine + scoring)
  are well-tested at the mutation level; the lexical-diversity rule
  has two clear test gaps now filed as F108 (assert the reported
  ratio in tests — 36 of the 47 misses) and F109 (borderline-cluster
  fixtures — the remaining 11). Triage methodology: cluster missed
  mutants by site → one ROADMAP entry per root cause, not per
  mutant. `mutants.out/` is run-local output and stays gitignored.
- **Four ROADMAP entries from the 2026-04-25 docs UX critique
  (Block E).** No code today — design lap. F104 per-category
  sidebar grouping in `SUMMARY.md` (mirror the index-page table
  shape in the sidebar), F105 consolidated references page (single
  surface for the 10+ scattered citations: WCAG, RGAA, Sweller,
  Gibson, Graesser, Coh-Metrix, BDA, IFLA, FALC, plainlanguage.gov,
  Kandel-Moles), F106 landing-page polish (deferred — `introduction.md`
  already plays the landing role; revisit when there's a first
  external consumer), F107 FR rule labels via page subtitle + FR
  index gloss (rule IDs stay as the stable contract; FR labels
  earn a visible surface for FR-only readers). Picks documented
  on the entries.
- **Four ROADMAP entries from the 2026-04-25 encoding-coverage
  survey (F110 – F113).** No code today — design lap. The test
  surface covers grapheme clusters (`unicode-segmentation`
  everywhere), CRLF normalisation, and Latin-1 NFC accented
  characters. It does **not** cover four real corpus realities:
  F110 — UTF-8 BOM (Windows-edited Markdown carries a leading
  `\u{FEFF}` that `read_to_string` does not strip); F111 — NFC vs
  NFD normalisation (the biggest risk: HashMap-keying rules like
  `low_lexical_diversity`, `consecutive_long_sentences`,
  `weasel-words`, the stop-words check in `detect_language` would
  treat NFC `café` and NFD `café` as different words); F112 —
  lone-CR (the implemented but untested classic-Mac path) and
  zero-width characters mid-word; F113 — mixed-script paragraphs
  (filed Speculative). Out-of-scope and explicitly so: invalid
  UTF-8 (the type system rejects it at the read boundary) and
  non-UTF-8 encodings (would violate the deterministic-core prime
  directive — charset detection is heuristic, users transcode at
  the file boundary with `iconv` or "save as UTF-8" once). New
  ROADMAP sub-section "Encoding / input handling" carries the
  rationale.
- **F84 part 2 — first redistributable replacement (public_ok count
  +1).** `Mon Parcours Handicap — fiches FALC`
  (`monparcourshandicap.gouv.fr/aides`) added to
  `examples/texts.yaml` under the Health-literacy section. French
  government portal, Easy-to-Read fact sheets on disability
  benefits (PCH, AAH, complément de ressources, …). Licence is
  Etalab Open Licence 2.0 (site footer: "Sauf mention contraire,
  tous les contenus de ce site sont sous licence etalab-2.0"),
  CC-BY-equivalent — `redistribution: public_ok`. Type
  `gov_guide`, polarity `good_example`, conditions
  `[aphasia, non-native, general]`, language FR. Fact sheets are
  flowing-text PDFs (`markdownable: 2`); still inside the existing
  CDC-style precedent. The entry is also surfaced in the curated
  per-section table in `examples/texts.md` under "Easy-read / FALC
  / dyslexia / aphasia / ADHD". Auto-generated coverage matrix
  refreshed; the public-side `texts.md` snapshot reflects the new
  count without leaking local-only state. The "Aphasia-specific
  French samples beyond SPF/HAS are thin" note in `gaps:` is
  marked partially closed.
- **F108 + F109 closed — `low_lexical_diversity` mutation score 36 %
  → 89 %.** Five new tests target the gaps that the F98 baseline
  surfaced. `reported_ratio_is_minimum_observed_in_cluster` builds a
  50-W + 100-cache + 50-V fixture so the cluster exits via the
  non-flush path and the min ratio (0.01) appears mid-slide, not at
  the anchor — kills 36 of the 47 missed mutants in
  `ratio_at_anchor_min` (arithmetic shifts in slide-in / slide-out
  / ratio computation). `flush_path_reports_final_ratio` covers the
  end-of-document case. `exactly_window_size_tokens_runs_the_check`
  pins the early-return guard at exactly `tokens.len() == window`.
  `cluster_starts_at_strict_inequality` and
  `ratio_exactly_at_threshold_does_not_trigger` use a 49-W +
  51-cache construction so the single full window has unique = 50
  → ratio == `min_ratio` exactly: a `< → <=` flip on the trigger
  would emit, the test asserts none. Remaining 8 missed mutants
  are equivalent under the current rule logic (defensive guards
  unreachable in normal flow; initial values overwritten by the
  first slide step) — documented in the F108 / F109 ROADMAP rows.

### Changed

- **Two library-code `.expect()` calls dropped.**
  `consecutive-long-sentences` and `all-caps-shouting` now use
  idiomatic `if let` / `Option::filter` patterns instead of panic
  paths. The invariants were infallible by construction but the panic
  paths were cognitive overhead per AGENTS.md ("No `unwrap()` /
  `expect()` in library code"). The remaining
  `NonZeroU32::new(LITERAL).expect` sites stay — they collapse to
  compile-time-checkable invariants.
- **`scoring::compute` category-cost cast now asserts the clamp
  invariant in debug builds.** Replaces the bare `#[allow(
  clippy::cast_possible_truncation, …)]` with an explicit safety
  contract comment plus `debug_assert!(normalized.is_finite() &&
  (0.0..=cap).contains(&normalized))`, so any future edit that
  loosens the `.min(cap).max(0.0)` clamp trips tests before it can
  produce silently-wrong scores.
- **`detect_language` single-pass, alloc-light hot path (F102).**
  The stop-words ratio scan no longer materialises two intermediate
  vectors (`Vec<&str>` of words + `Vec<String>` of lowercased forms)
  and no longer iterates the lowercased vector twice (once per
  language). One pass over `text.unicode_words()` now feeds three
  scalar counters; `to_lowercase()` is only invoked when the word
  contains an uppercase character, so pure-ASCII lowercase prose —
  the common case — pays no allocation. Bench delta on
  `engine_lint_str/en_long_devdoc` against a fresh `stream2-noisy`
  baseline: −0.56 % (p = 0.00, ~20 µs). Smaller than the samply
  profile's 7.5 % inclusive figure suggested because most of the
  inclusive cost was `unicode_words()` itself, which the rewrite
  cannot touch. Same session refuted F93 + F94 — the previously
  suspected hot spots accounted for under 0.1 % combined; ROADMAP
  entries closed with the profile evidence.
- **Sentence splitting moved into the parser phase (F103).**
  `Paragraph::new` now calls `split_sentences` once at construction
  and the result lives on the paragraph as a public `sentences:
  Vec<Sentence>` field. The eight rules that previously re-ran the
  split — `consecutive_long_sentences`,
  `repetitive_connectors`, `excessive_nominalization`,
  `nested_negation`, `passive_voice`, `unclear_antecedent`,
  `conditional_stacking`, `paragraph_too_long` — read
  `&paragraph.sentences` instead. The previous lazy-per-rule pattern
  predates the rule explosion and silently violated the AGENTS.md
  "do not re-parse the document per rule" directive once eight rules
  needed sentences. Bench delta against `stream2-noisy`:
  `engine_lint_str/en_long_devdoc` −11.58 % (p = 0.00, ~394 µs);
  `parse_markdown/en_long` +17.67 % (p = 0.00, ~38 µs) — the split
  cost moved into the parser phase, where it was previously
  multiplied across rules. Net user-facing path: ~360 µs faster on
  the long EN devdoc input, which closes the stream-2 perf arc
  (F95–F96 hygiene shipped earlier today; F93/F94 refuted by
  profile; F102 + F103 measured wins).

## [0.2.2] — 2026-04-23

### Added

- **FR parallel sidebar locale filter (1.A of the FR per-rule pages
  plan).** The mdBook sidebar is a single flat SUMMARY, so until now
  both language trees showed side by side. CSS `:has()` rules in
  `lucid-layout.css` now hide the wrong-locale entries based on
  `html[lang]` (which `head.hbs` sets pre-paint on `/fr/` pages) —
  same pattern F35a used for the header lang-switch. EN viewers see
  only the EN tree; FR viewers see only the "Version française"
  subtree. F90 (split `SUMMARY.md` per locale) and F91 (multi-book
  mdBook layout) filed as future alternatives.
- **Server-rendered EN ↔ FR counterpart deep-linking (F92, 2.B of
  the same plan).** The stock lang-switch in `index.hbs` always
  points at `/` and `/fr/`; a new post-build step
  (`scripts/sync_lang_counterparts.py`) walks `docs/book/**/*.html`
  and rewrites both `hreflang="en"` and `hreflang="fr"` anchors so
  the toggle deep-links to the matching page. From
  `/fr/rules/sentence-too-long.html` the EN toggle now goes to
  `../../rules/sentence-too-long.html`, and vice versa. Wired into
  `just docs-build`, `.github/workflows/docs-deploy.yml`, and the
  new `just docs-lang-check` recipe (also run by CI) — which fails
  the build if any FR page lacks an EN counterpart (orphaned
  translation), while untranslated EN pages remain informational
  (tracked as F25, not a gate).
- **First FR per-rule page: `structure.sentence-too-long`.** Mirrors
  the EN page's section structure (`Ce que cette règle signale` /
  `En bref` / `Détection` / `Paramètres` / `Exemples` /
  `Neutralisation` / `Voir aussi`) with FR-first example ordering.
  Cross-links into the EN guide + scoring pages via `../../…`
  until the FR guides land.
- **`examples/texts.yaml` + `examples/texts.md` split by redistribution tier.**
  The tracked referential now lists only `public_ok` entries (25 of 55
  sources); the 30 `check_license` / `link_only` / `restricted` entries
  moved to the gitignored `examples/local/texts.yaml` +
  `examples/local/texts.md` companions. `scripts/texts_common.py`'s
  `load_sources()` merges both halves when the local one exists, so
  `just texts-{plan,fetch,clean,convert}` keep working transparently.
  Rationale: local-only sources get `.env`-style discretion — parsed
  by tooling, never named in published surfaces. Codified as AGENTS.md
  prime directive #10. Dropped the static `redistribution_summary` and
  `markdownable_summary` blocks from `texts.yaml` (redundant with the
  auto-generated coverage snapshot).
- **Auto-generated coverage tables for the texts referential (F84, part 1).**
  `scripts/texts_coverage.py` compiles `examples/texts.yaml` into two
  artefacts with two audiences: the committed `examples/texts.md`
  carries a public-facing matrix of **`public_ok` counts only** (shape
  × lang, condition × lang, type × lang) spliced between
  `<!-- coverage:begin -->` / `<!-- coverage:end -->` markers — no
  totals, no names, no signal that non-redistributable sources exist.
  The gitignored `examples/local/COVERAGE.md` carries the full map
  (`public / total` cells plus the load-bearing local-only list — the
  F84 part 2 hunting target). New `just texts-coverage` regenerates
  both; `just texts-coverage-check` fails on drift. A 26-test stdlib
  `unittest` suite (`scripts/test_texts_coverage.py`, run via
  `just texts-coverage-test`) pins the leak contract — the public
  render must not surface `link_only` / `check_license` titles or
  totals — alongside axis slotting, counting, splice idempotence, and
  a smoke test against the real YAML.

### Changed

- **`syntax.nested-negation` — FR counter extended to second-position
  negators (F87).** The French detector now recognises `pas`, `rien`,
  `jamais`, `plus`, `personne`, `aucun`, `aucune`, `guère`, and
  `nulle part` alongside the `ne` / `n'` clitics, using pair-based
  counting: each clitic contributes one negation and consumes its
  nearest particle within a 6-token window; unpaired particles in a
  `ne`-sentence count as one more — which catches the canonical
  triple-negation shape `Nous ne disons pas que rien n'est jamais
  possible` (now 3, previously 2). Guards: `pas` and `plus` never
  count when unpaired (too ambiguous outside `ne …`); `rien` after
  `de` is treated as the idiom `de rien` and skipped; particles in a
  sentence without a `ne` clitic are skipped too (`plus de courage`,
  `personne d'autre`). Standalones `sans` / `non` keep their previous
  semantics. Fixture at `tests/corpus/fr/nested-negation.md` exercises
  the F87 target, the RGAA 4.1 single-negation cases, the `de rien`
  guard, and the multi-standalone `sans … sans …` shape.
- **FR rules index moved from `fr/rules-index.md` to `fr/rules/index.md`**
  for parity with the EN tree (`rules/index.md`). SUMMARY.md entry
  updated; the intra-page relative links now use `../../rules/…` to
  reach EN rule pages one directory deeper.
- **`docs_links_stay_inside_docs` test now respects file depth.** The
  previous heuristic flagged any `](../../…)` target as an escape,
  assuming the docs tree was at most two levels deep. The FR tree
  added a third level (`docs/src/fr/<section>/<page>.md`); the test
  now compares the `../` count to the source file's depth relative
  to `docs/src/`, so `../../` from a depth-2 page (into `docs/src/`)
  is accepted while `../../../` (out of the tree) still fails.

## [0.2.1] — 2026-04-23

### Added

- **Per-rule TOML override for `structure.excessive-commas`.**
  `[rules."structure.excessive-commas"].max_commas` now reaches the
  rule builder and replaces the profile preset. Must be a positive
  integer; `0`, negatives, and non-integer types are rejected at
  load time with a targeted error. Third rule wired into the
  per-rule override path, after `readability.score.formula` and
  `lexicon.unexplained-abbreviation.whitelist`.
- **Scraped prose fixtures pipeline.** `examples/texts.yaml` +
  `just texts-plan` / `just texts` pull real-world prose sources
  (plainlanguage.gov, EC *How to write clearly*, Canada.ca,
  proselint / write-good test fixtures, ASSET / OneStopEnglish
  before-after datasets, …) into `examples/public/` (committed) or
  `examples/local/` (gitignored). Python cleaning + conversion
  scripts under `scripts/`. See [`scripts/README.md`](scripts/README.md).
- **TTY capture GIFs across the docs site.** New `docs/tapes/*.tape`
  set (hero, score-clean, score-fail, profiles, explain) rendered
  via `vhs` into `docs/src/assets/tty/*.gif`, embedded in the README,
  introduction, scoring guide, CI-integration guide, profiles guide,
  and rules index.
- **Idea-highlight motif extended to the `structure.sentence-too-long`
  rule page.** The before/after EN + FR examples now carry the same
  colour-matched `data-idea` spans used on the introduction page, so
  readers can trace each idea across the rewrite. Generic
  `.lucid-idea` selectors added to `lucid-layout.css` in parallel
  with the landing-page `.lucid-stance__idea` selectors.

### Fixed

- **`mdbook serve` no longer triggers 18 stylesheet / script 404s on
  the 404.html page (Block A).** `book.toml` sets
  `site-url = "/lucid-lint/"` for GitHub Pages, and mdBook emits
  `<base href="/lucid-lint/">` into 404.html (and only there). On
  localhost that prefix doesn't exist, so every asset would 404
  before the page recovered via a second fetch. The old JS
  workaround in `docs/theme/head.hbs` rewrote `<base>` at parse
  time, but the browser's preload scanner had already dispatched
  the wrong URLs. Fix: `just docs-serve` now sets
  `MDBOOK_OUTPUT__HTML__SITE_URL=/` for the serve process, so 404.html
  carries `<base href="/">` on localhost and the correct
  `<base href="/lucid-lint/">` in production builds. The JS
  workaround has been removed.

### Changed

- **Docs sweep: v0.1 / v0.2 claims refreshed post-0.2.0 release.**
  README, RULES.md, and the mdBook site no longer describe lucid-lint
  as "v0.1 under active development" or "17 rules" — 25 rules are
  now shipped (17 from v0.1, 8 added in the v0.2 cycle). Example
  TTY output in the docs uses the current format (new `[rule-id]`
  suffix, sparkline score block); JSON examples use the
  post-F29-slim `category.rule-name` rule-ID form. `(v0.2+)` section
  tags dropped from shipped-feature headings. Editor-integration
  section moved from "planned for v0.2" to "roadmap v0.3+". The
  README's config example uses working per-rule overrides instead
  of ones that parsed but had no runtime effect.

### Packaging

- **Crate now publishes to crates.io** (first publish since v0.1.1).
  `Cargo.toml`'s packaging switched from `exclude = ["docs/", …]`
  to an explicit `include = […]` list that keeps `docs/src/rules/`
  inside the tarball — `src/explain.rs` uses
  `include_str!("../docs/src/rules/<slug>.md")` to bundle reference
  pages into the binary, so these files must ship.

## [0.2.0] — 2026-04-22

### Added

- **`--fail-on-warning` is now toggleable, plus a `--no-fail-on-warning`
  mirror (F80).** The flag used to be a bare switch with
  `default_value_t = true`, which meant there was no CLI path to
  turn it off — `--fail-on-warning=false` raised a usage error and
  `--no-fail-on-warning` was unrecognised. It now accepts an
  optional boolean value (`--fail-on-warning`,
  `--fail-on-warning=true`, `--fail-on-warning=false`) and a hidden
  `--no-fail-on-warning` mirror shortcut. If both forms are passed
  on the same invocation, `--no-fail-on-warning` wins. This unblocks
  CI callers who want their gate to depend purely on `--min-score`
  — set the score floor, let the warning count inform the review,
  don't fail the build on warnings alone.
- **Server-rendered skip link and language switch (F35a)** — the
  mdBook theme now forks upstream `index.hbs` (minimal-diff against
  mdBook v0.5.2) and emits the WCAG 2.4.1 "skip to content" anchor
  and the EN / FR switch inside the server-rendered HTML, not via
  post-paint JS injection. Both language variants are emitted; CSS
  in `lucid-layout.css` hides the wrong-locale copy based on
  `html[lang]`, which `theme/head.hbs` sets synchronously before
  first paint on `/fr/` pages. The skip link and lang switch now
  work with JavaScript disabled, closing the P1 findings from the
  2026-04-22 accessibility audit. A progressive-enhancement
  smooth-scroll handler remains in `lucid-navigation.js`; the
  previous `skipLink()` and `langSwitch()` IIFEs are gone. Unblocks
  F26 (stock theme labels can now be collapsed at the markup
  level).
- **Accessibility statement page fleshed out (F35d)** — the EN
  `docs/src/accessibility.md` now carries the first AAA audit
  pass (2026-04-22, 17/20) and a "Known limitations" section
  that lists every open finding in the F35 family (F35a–F35c)
  with a roadmap link and a plain-language explanation. Report
  route and audit cadence are spelled out. The FR
  `docs/src/fr/accessibility.md` stub gains a short parallel
  "Écarts connus" section so bilingual readers see the same
  gaps. F35d moves to 🟢 Shipped; F35 stays 🟡 In progress
  until F35a clears.
- **French docs mirror — skeleton + home (F25 first slice)** — the
  `docs/src/fr/` tree now carries a translated `introduction` (the
  prior stub is gone) plus a translated `rules-index` that mirrors
  the EN rules overview. Short `accessibility.md` and `roadmap.md`
  FR pages point readers to the full EN versions for now. A new
  "Version française" section in `SUMMARY.md` surfaces the FR pages
  in the sidebar. The `fr/` tree stays depth-2 to satisfy the
  `docs_links_stay_inside_docs` coverage rule. Per-rule FR pages
  stay in F25 for later slices.
- **`lexicon.weasel-words` false-positive cleanup (F23 completion)** —
  hits inside straight `"..."` or paired curly `"..."` quotes are
  now skipped, joining the inline-code-span and directional-pair
  exceptions shipped earlier in the F23 cycle. The author is
  *mentioning* the word in those spans rather than using it as a
  hedge (`Use "many" sparingly`, `The word "rather" weakens the
  sentence`). Scan is per-line, so an unclosed quote cannot leak
  across a newline and silence later hits. Single quotes /
  apostrophes are deliberately NOT recognised — they collide with
  possessives, contractions, and French elisions. The "concrete
  noun" semantic slice (`"many X"` with a concrete noun) remains
  unshipped by design: it needs POS data and belongs in the future
  `lucid-lint-nlp` plugin (F75).
- **Definition-aware `lexicon.unexplained-abbreviation` + project whitelist
  (F9 + F31)** — the rule is now two-pass. A pre-scan collects
  acronyms defined in the document in either canonical form
  (`Full Expansion (ACRONYM)` or `ACRONYM (Full Expansion)`) and a
  single definition anywhere in the document silences every
  occurrence of the same acronym. The parenthesised half of a
  definition must contain at least two alphabetic words, so
  throwaway notes like `(TBD)` or `(check later)` do not count.
  Paired with F31: the shipped `dev-doc` baseline whitelist is
  narrowed to the ubiquitous infrastructure stack (`URL`, `HTML`,
  `API`, `CPU`, `IDE`, …). Accessibility standards (`WCAG`, `ARIA`,
  `RGAA`, …), engineering-practice initialisms (`YAGNI`, `DRY`,
  `TDD`, …), and AI/language-tech terms (`LLM`, `NLP`) are no longer
  part of the baseline. Projects that use them add them to
  `[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml`
  — user entries are additive over the baseline. Silencing
  precedence is now `defined-in-doc → user whitelist → baseline`.
  See [`docs/src/rules/unexplained-abbreviation.md`](docs/src/rules/unexplained-abbreviation.md).

- **Config-based diagnostic ignores (F19)** — new top-level
  `[[ignore]]` array-of-tables in `lucid-lint.toml` silences every
  diagnostic with the matching `rule_id`, across Markdown, plain
  text, and stdin. Fills the gap for formats that have no inline-
  disable escape hatch, and gives users a project-wide alternative
  to sprinkling inline directives when a rule is globally noisy.
  Each entry requires a `rule_id` string; unknown ids are tolerated
  silently so removing a rule in a future release never breaks an
  older config. The filter runs after rule emission and before
  scoring, so score, render, and exit-code logic all see the
  filtered view. A `reason = "..."` field is tracked separately as
  F20. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#silencing-rules-globally-v02).
- **Path exclusion (F78)** — new `--exclude <GLOB>` CLI flag
  (comma-delimited, repeatable) and `exclude = [...]` field in the
  `[default]` section of `lucid-lint.toml`. Both lists are unioned and
  applied during directory recursion: matching files are skipped,
  matching directories are pruned without being entered. Patterns are
  matched against the path relative to the walked root, so
  `vendor/**` behaves the same whether the user passes a relative or
  absolute root. Explicit file arguments bypass exclusion — naming a
  path on the command line is treated as intent. Backed by the
  `globset` crate. Unblocks adoption on large documentation
  repositories. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#excluding-paths-v02).
- **TOML config loader wiring (F77)** — `lucid-lint check` now
  auto-discovers a `lucid-lint.toml` at or above the current working
  directory (stopping at the nearest `.git` boundary) and applies its
  `[default].profile`, `[default].conditions`, `[scoring]` weights /
  caps, and `[rules.readability-score].formula` fields. A new
  `--config <path>` flag overrides discovery; a missing explicit path
  is an error, a missing auto-discovered file is not. Precedence:
  built-in profile defaults → TOML → CLI flags (unset CLI defers to
  TOML; unset TOML defers to the preset). Per-rule TOML overrides
  beyond `readability.score` will land as each rule's `Config` gains a
  `Deserialize` impl. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md).
- **User-configurable readability formula (F11)** — new
  `--readability-formula` CLI flag and `FormulaChoice` enum exposed on
  `readability_score::Config`. `auto` (default) preserves the F10
  per-language behaviour; `flesch-kincaid` and `kandel-moles` pin a
  concrete formula regardless of detected language, which is useful
  for cross-document comparison on mixed corpora. Wired into the
  engine via `Engine::with_readability_formula(choice)`. See
  [`docs/src/rules/readability-score.md`](docs/src/rules/readability-score.md).
- **`lexicon.consonant-cluster` rule (F47)** — flags words whose longest run of
  consecutive consonants meets or exceeds a per-profile threshold (BDA
  Dyslexia Style Guide). Language-aware vowel sets: French accented
  forms (`é`, `è`, `ê`, `à`, `œ`, …) count as vowels; English fallback
  still accepts common latin-1 accented vowels for borrowed words
  (`café`, `naïve`); `y` is a vowel everywhere (lenient). Hyphens and
  apostrophes close the word. Profile thresholds: `min_run_length` 6 /
  5 / 4 (`dev-doc` / `public` / `falc`). Condition tags: `dyslexia`,
  `general`. See [`docs/src/rules/consonant-cluster.md`](docs/src/rules/consonant-cluster.md).
- **`syntax.dense-punctuation-burst` rule (F54)** — flags *local* bursts of
  punctuation: windows where ≥ N qualifying marks (`,`, `;`, `:`, `—`,
  `–`) cluster within W grapheme clusters (IFLA easy-to-read
  guidelines). Distinct from `structure.excessive-commas` (per-sentence count):
  this rule fires on local density, not total count. Per-source-line
  sliding window with greedy-extend; emits one diagnostic per burst,
  never overlapping. Profile thresholds: `dev-doc` 4/30, `public` 3/30,
  `falc` 3/40 — `dev-doc` tolerates a 3-mark cluster, FALC widens the
  window. Tag: `general`. See [`docs/src/rules/dense-punctuation-burst.md`](docs/src/rules/dense-punctuation-burst.md).
- **`lexicon.redundant-intensifier` rule (F62)** — flags intensifiers
  (`very`, `really`, `extremely`, `absolutely`, … / FR `très`,
  `vraiment`, `extrêmement`, `absolument`, …) that try to upgrade the
  confidence of a statement without adding information
  (plainlanguage.gov Chapter 4, CDC Clear Communication Index).
  Deliberate sibling of `lexicon.weasel-words`: weasel words downgrade
  confidence, intensifiers upgrade it — the two lexical lists are
  disjoint by construction. Per-language config supports
  `custom_intensifiers_{en,fr}` and `disable` for per-phrase
  suppression. Fenced / inline code spans are ignored; `Unknown`
  language skips the rule. See [`docs/src/rules/redundant-intensifier.md`](docs/src/rules/redundant-intensifier.md).
- **`structure.mixed-numeric-format` rule (F52)** — flags sentences that mix
  digit numerals (`42`, `3.14`, `1,000`, `1 000`) with spelled-out
  numerals (`two`, `trois`, `twenty`, `cent`) in the same sentence
  (CDC Clear Communication Index 3.5, plainlanguage.gov Chapter 4).
  Per-sentence scan using the shared tokenizer; code blocks excluded
  upstream by the Markdown parser. No configurable threshold — a
  single co-occurrence suffices. EN `one` and FR `un` / `une` are
  excluded from the spelled-numeral lists because they double as
  indefinite pronouns / articles. First `dyscalculia`-tagged
  `structure` rule. See [`docs/src/rules/mixed-numeric-format.md`](docs/src/rules/mixed-numeric-format.md).
- **`structure.line-length-wide` rule (F50)** — flags source lines wider than
  the per-profile ceiling (WCAG 1.4.8 AAA / BDA Dyslexia Style Guide
  grounding). Per-paragraph grapheme-cluster scan; fenced code blocks
  excluded upstream by the Markdown parser. Profile thresholds:
  `max_line_length` 120 / 100 / 80 (`dev-doc` / `public` / `falc`);
  FALC matches the WCAG 1.4.8 AAA recommendation. Condition tags:
  `dyslexia`, `general`. See [`docs/src/rules/line-length-wide.md`](docs/src/rules/line-length-wide.md).
- **Per-language readability formula (F10 must-ship slice)** —
  `readability.score` now selects its formula from the detected document
  language: Flesch-Kincaid for English (kept), Kandel & Moles (1958) for
  French. The Kandel-Moles ease score is converted to a grade-equivalent
  via the standard `(100 − score) / 10` linear approximation so
  per-profile `max_grade_level` thresholds remain comparable across
  languages. Unknown language falls back to Flesch-Kincaid. Diagnostic
  messages now surface the formula name and, for FR, both the native
  ease score and the grade-equivalent. User-configurable formula choice
  (F11) and the `Gunning Fog` / `SMOG` / `Dale-Chall` / `Scolarius`
  alternatives (F10 should-ship) are still pending.
- **`lexicon.all-caps-shouting` rule (F48)** — flags runs of two or more
  consecutive ALL-CAPS words (WCAG 3.1.5 / BDA Dyslexia Style Guide
  grounding). Per-profile thresholds: `min_run_length` 3 / 2 / 2
  (`dev-doc` / `public` / `falc`); `dev-doc` tolerates a 2-word `DO NOT`
  emphasis run. Single ALL-CAPS tokens stay with `lexicon.unexplained-abbreviation`.
  First `lexicon` rule to declare the `a11y-markup` tag (also `dyslexia`,
  `general`). See [`docs/src/rules/all-caps-shouting.md`](docs/src/rules/all-caps-shouting.md).
- **`syntax.conditional-stacking` rule (F56)** — flags sentences chaining
  multiple conditional connectors (FALC / plainlanguage.gov grounding).
  Per-profile thresholds: 3 / 2 / 1 (`dev-doc` / `public` / `falc`).
  Bilingual lists in `language::{en,fr}::CONDITIONALS`; FR also counts
  the `s'il` / `s'ils` clitics. Condition tags: `aphasia`, `adhd`,
  `general`. See [`docs/src/rules/conditional-stacking.md`](docs/src/rules/conditional-stacking.md).
- **`syntax.nested-negation` rule (F55)** — flags sentences that stack multiple
  negations (FALC / CDC Clear Communication Index grounding). Per-profile
  thresholds: 3 / 2 / 1 (`dev-doc` / `public` / `falc`). Bilingual: EN
  counts the negation list plus contracted `n't`; FR counts `ne` / `n'`
  clitics plus standalone `sans` / `non` (avoids the `plus` / `personne`
  ambiguity outside `ne ... X`). Condition tags: `aphasia`, `adhd`,
  `general`. See [`docs/src/rules/nested-negation.md`](docs/src/rules/nested-negation.md).
- **Condition-tag ontology (F71 + F72)** — new `ConditionTag` enum
  (`a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`,
  `non-native`, `general`) plus `Rule::condition_tags()` trait method
  (default `&[General]`). User-facing surface: `[default] conditions =
  [...]` in `lucid-lint.toml` and the `--conditions` CLI flag
  (comma-separated). Filter semantics: rules tagged `general` always
  run; tagged-only rules opt in via the active list. All 17 v0.2 rules
  are `general`, so default behavior is unchanged. Tagged rules (F48,
  F55, F56) ride this infrastructure. See
  [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md).
- **Hybrid scoring model (F14)** — every run now emits a global `X / max`
  score plus five per-category sub-scores (Structure · Rhythm · Lexicon ·
  Syntax · Readability). Composition stacks a weighted sum, density
  normalization (per 1 000 words, floored at 200 words) and a per-category
  cap so no single rule dominates. See [`docs/src/guide/scoring.md`](docs/src/guide/scoring.md).
- **`--min-score=N` CLI flag** — optional gate that exits `1` when the
  aggregate document score falls below `N`. Stacks with the existing
  severity gate.
- **`[scoring]` / `[scoring.weights]` in `lucid-lint.toml`** — override
  `category_max`, `category_cap`, and per-rule weights without touching
  code.
- **`Diagnostic.weight` field** — populated at emission from
  `scoring::default_weight_for`; overridable per-diagnostic via
  `with_weight`.
- **Public `scoring` module** — `Score`, `CategoryScore`, `Scorecard`,
  `ScoringConfig`, `compute`, `default_weight_for`, `severity_multiplier`,
  plus the `Category::ALL` constant for fixed iteration order.
- **Per-rule mdBook pages (F28)** — every one of the 17 rules now has
  a dedicated page under `docs/src/rules/<rule-id>.md` covering
  category, severity, default weight, parameters per profile, EN/FR
  examples, and suppression. Wired into `docs/src/SUMMARY.md`;
  `docs/src/rules/index.md` turns into a categorised link index.
  Rules touched: `structure.sentence-too-long`, `structure.paragraph-too-long`,
  `structure.heading-jump`, `structure.deeply-nested-lists`, `structure.excessive-commas`,
  `structure.long-enumeration`, `structure.deep-subordination`,
  `rhythm.consecutive-long-sentences`, `rhythm.repetitive-connectors`,
  `lexicon.low-lexical-diversity`, `lexicon.excessive-nominalization`,
  `lexicon.unexplained-abbreviation`, `lexicon.weasel-words`, `lexicon.jargon-undefined`,
  `syntax.passive-voice`, `syntax.unclear-antecedent`, `readability.score`.
- **Rule documentation coverage gate (F42)** — new integration test
  `tests/rule_docs_coverage.rs` keeps five surfaces in lock-step: rule
  source, `rules::default_rules`, `Category::for_rule`,
  `scoring::WEIGHTED_RULE_IDS` (new public const), and the per-rule
  mdBook pages. When `RULE_DOCS_GATE_GIT=1` (set by the CI `test` job),
  any rule source modified versus `origin/main` must also be mentioned
  in the `## [Unreleased]` section of this file. Contract documented
  in `CONTRIBUTING.md` §"Adding or modifying a rule — documentation
  contract" and mirrored as directive 8 in `AGENTS.md`.
- **`docs/src/guide/suppression.md`** — dedicated page covering the
  inline and block disable directives, common properties, and the
  F19–F21 deferred extensions. The 17 per-rule pages now link to it
  via `../guide/suppression.md` (previously pointed at the
  repo-root `RULES.md`, which renders as a 404 inside mdBook).
- **SARIF v2.1.0 output (F32)** — `lucid-lint check --format=sarif` now
  emits a SARIF log compatible with GitHub Code Scanning (and any
  SARIF v2.1.0 consumer). Each rule appears once under
  `runs[0].tool.driver.rules` with its category, default severity,
  default scoring weight, and a `helpUri` to the per-rule mdBook page;
  per-result `properties` carry the diagnostic's scoring weight and
  the heading it was found under.
  [`docs/src/guide/ci-integration.md`](docs/src/guide/ci-integration.md)
  ships a full GitHub Actions workflow that uploads the SARIF to
  Code Scanning via `github/codeql-action/upload-sarif@v3`.
- **Context-aware `lexicon.weasel-words` — first slice of F23.** Hits inside
  inline code spans (`` `…` ``) are skipped so an author discussing a
  weasel term by name is no longer flagged for using it. Directional
  pairings `rather than` (EN) and `plutôt que` (FR) are recognised as
  conjunctions and skipped. Dogfood on this repo drops from 9 to 5
  weasel hits. Follow-up work on straight-quoted terms and `"many X"`
  with concrete X remains queued under F23. Rule touched:
  `lexicon.weasel-words`.
- **Roadmap mirror in the mdBook site (F27)** — `docs/src/roadmap.md`
  is auto-generated from the root `ROADMAP.md` by
  [`scripts/sync-roadmap.py`](scripts/sync-roadmap.py), invoked as a
  dependency of `just docs-build` / `just docs-serve`. Relative links
  are rewritten on the fly: targets under `docs/src/` become
  docs-relative, everything else becomes an absolute GitHub URL, so
  the `docs_links_stay_inside_docs` gate still passes.
- **Rule + feature cross-links across the mdBook site (F30)** — first
  mention of a rule id or feature id (in backticks / plain text) per
  H2/H3 section now links to its canonical page
  (`docs/src/rules/<id>.md` or `docs/src/roadmap.md`). Touches
  `accessibility.md`, `architecture/design-decisions.md`,
  `guide/scoring.md`, `guide/suppression.md`, `roadmap.md`, and eight
  per-rule pages.
- **`RULES.md` category drift fixed (F43)** — per-rule `**Category**`
  lines and the Categories table now match `Category::for_rule`:
  `structure.excessive-commas` and `structure.deep-subordination` are `structure`,
  `rhythm.repetitive-connectors` is `rhythm`, `syntax.unclear-antecedent` is
  `syntax`. The drift banners on the four affected per-rule mdBook
  pages are removed.
- **In-docs link convention + gate.** A new test
  `docs_links_stay_inside_docs` scans `docs/src/**/*.md` and fails on
  any `](../../…)` pattern that escapes the mdBook tree. Convention
  written up as `AGENTS.md` directive 9 and the "Docs links stay
  inside `docs/src/`" section of `CONTRIBUTING.md`. Also fixed a
  pre-existing broken `../../ROADMAP.md` link in
  `docs/src/guide/ci-integration.md`.

### Changed

- **Shorter diagnostic messages for `lexicon.unexplained-abbreviation`
  and `structure.line-length-wide`.** Both rules used to inline their
  full repair guidance in the diagnostic line (TOML snippet, WCAG
  citation, recommended line width). Now the diagnostic states the
  fact only — `Acronym "X" is not defined on first use.` /
  `Line is N characters wide (maximum M).` — and the `explain`
  subcommand (or the mdBook page linked from the summary hint) is the
  canonical place for the fix recipe. Keeps the terminal output
  scannable when a cluster fires on many lines.
- **Clustered TTY headers hoist shared section and shared message.**
  When every member of a `(file, rule_id)` cluster shares the same
  `section:` label, it moves to the header rather than repeating on
  every row. Same treatment for a shared message: it is dimmed under
  the header and each row below carries only its location. Reduces
  visual repetition on long clusters without losing per-row locations.
- **Breaking — rule IDs now use `category.rule-name` form (F29-slim).**
  Every rule ID has been renamed from flat kebab-case to a
  category-prefixed form: `sentence-too-long` →
  `structure.sentence-too-long`, `weasel-words` →
  `lexicon.weasel-words`, `readability-score` → `readability.score`,
  and so on for the other 22 rules. Source files moved into category
  subdirectories under `src/rules/<category>/`. `Category::for_rule`
  now derives the category from the id prefix instead of a
  hand-maintained match arm (F43-style drift is impossible by
  construction). Pre-1.0 is the last cheap moment to break IDs
  before they propagate through downstream tooling. **Hard break —
  no alias layer.** Downstream callers must update every occurrence:
  - Markdown suppression directives:
    `<!-- lucid-lint disable-next-line sentence-too-long -->` →
    `<!-- lucid-lint disable-next-line structure.sentence-too-long -->`
    (same for `lucid-lint-disable` / `lucid-lint-enable` block forms).
  - `lucid-lint.toml` per-rule tables:
    `[rules.sentence-too-long]` → `[rules."structure.sentence-too-long"]`
    (quotes are required because of the `.`).
  - `lucid-lint.toml` scoring weights:
    `[scoring.weights] sentence-too-long = 3` →
    `[scoring.weights] "structure.sentence-too-long" = 3`.
  - JSON output (`version = 2`) and SARIF `ruleId` fields carry the
    new ids — CI consumers parsing these streams need to update any
    hard-coded id matches.
  - `lucid-lint explain <rule-id>` and `--disable <rule-id>` CLI
    args expect the new form.

  mdBook filenames and published docs URLs still use the flat kebab
  slug (`docs/src/rules/sentence-too-long.md`,
  `/rules/sentence-too-long.html`) — the docs-tree rearchitecture
  into category subdirectories is tracked as a separate slice. The
  explain module carries a slug mapping so `canonical_url()`
  resolves correctly.

- **Breaking — `lexicon.unexplained-abbreviation` baseline whitelist (F31).**
  Removed from the shipped `dev-doc` defaults: `WCAG`, `WAI`, `ARIA`,
  `RGAA`, `EAA`, `FALC`, `AA`, `AAA`, `ADHD`, `LLM`, `NLP`, `YAGNI`,
  `DRY`, `KISS`, `SOLID`, `TDD`, `BDD`, `MVP`, `WASM`, `MIT`, `LRU`.
  Projects that relied on these being built-in must add them to
  `[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml`
  to restore the previous behaviour. The diagnostic message now
  names the TOML field so the fix is self-evident. See
  [`lucid-lint.toml`](lucid-lint.toml) at the repo root for the
  full dogfood list lucid-lint uses on its own docs.
- **Breaking — `Category` remap.** The six v0.1 variants collapse to
  five: `length` / `structure` → `Structure`, `rhythm` → `Rhythm`,
  `lexical` → `Lexicon`, `style` / `rhythm.repetitive-connectors` → `Syntax` /
  `Rhythm`, `global` → `Readability`. JSON `category` values and
  `Category::for_rule` are updated accordingly.
- **Breaking — `Engine::lint_str` / `lint_stdin` / `lint_file` return
  `Report`** (`diagnostics` + `scorecard` + `word_count`) instead of
  `Vec<Diagnostic>`. Call sites consume `report.diagnostics` and
  `report.scorecard`.
- **Breaking — JSON schema `version = 2`.** Adds `score`,
  `category_scores`, and per-diagnostic `weight`. Consumers on v0.1
  should bump the expected version and remap the category names listed
  above.
- **TTY output** appends one colorized `score:` line after the existing
  summary, followed by all five per-category scores in fixed order.

### Fixed

- **`structure.excessive-commas` parenthesised-list discount (F22 first
  slice)** — the rule now discounts commas inside `(A, B, C, …)`
  parenthesised token lists in addition to Oxford enumerations. A
  new sibling helper `parenthesised_list_comma_count` in
  `src/rules/enumeration.rs` recognises any balanced paren span
  holding three or more short comma-separated segments, accepting
  empty segments too so that markdown inline code (stripped by the
  parser) stays recognisable as `(, , , )`. Language-agnostic, no
  config change, no threshold change. Drops this repo's own dogfood
  `structure.excessive-commas` hit count from 25 to 15 (about 40% FP
  reduction). Non-parenthesised bare lists with 3+-word items
  (`as long as`, `as soon as`) and other surfaces stay flagged —
  they are explicitly deferred to v0.3 per
  `.personal/research/F22.md`.

### Still in flight (v0.2)

- **SARIF v2.1.0 output** for GitHub Code Scanning (F32).
- **Rule refinement** — definition-aware `lexicon.unexplained-abbreviation`,
  language-specific readability formulas (Kandel-Moles FR, SMOG,
  Coleman-Liau), context-aware relaxations for `structure.excessive-commas` and
  `lexicon.weasel-words` (F9, F10, F22, F23).
- **Rule-message clarity audit (F37)** — every diagnostic must answer
  "what do I change?"; gates the v0.2 release because score
  actionability depends on diagnostic actionability.
- **Docs site** — French mirror of the docs (`/fr/`), full reading-
  preferences popover UI, responsive / mobile adaptation, brand-owned
  theme-picker labels, accessibility-audit sweep (F25, F26, F33–F36).
- **Format support** — native AsciiDoc and HTML, a Pandoc companion
  script for docx (F5, F6, F7, F8).
- **Project-level scoring roll-up (F15)** — document-level lands in this
  cycle; multi-file roll-up still open.

See [`ROADMAP.md`](ROADMAP.md) for the full backlog with priorities.

## [0.1.1] — 2026-04-20

### Fixed

- **Windows release packaging** — the `Package (Windows)` job now runs under `bash` so the Unix-style `cp` invocation is parsed by Git Bash instead of PowerShell; v0.1.0 shipped without a Windows archive because PowerShell rejected `cp README.md LICENSE-MIT LICENSE-APACHE dist/` as too many positional arguments.

## [0.1.0] — 2026-04-20

First public release. Seventeen rules, bilingual EN/FR, deterministic core, Markdown + plain-text + stdin inputs.

### Added

#### Rules (17)

Length and structure:

- `structure.sentence-too-long` — reference implementation; per-profile word-count threshold
- `structure.paragraph-too-long` — sentence-count and word-count thresholds
- `structure.excessive-commas` — per-profile comma-per-sentence threshold, aware of enumerations
- `structure.long-enumeration` — flags 5+ comma-separated items; suggests list conversion
- `structure.deep-subordination` — counts subordinators between strong-punctuation breaks
- `structure.deeply-nested-lists` — flags list items nested beyond profile depth
- `structure.heading-jump` — flags section-level jumps greater than +1

Rhythm:

- `rhythm.consecutive-long-sentences` — intra-paragraph streaks of long sentences

Lexical:

- `lexicon.weasel-words` — per-language vague-qualifier phrase list
- `lexicon.unexplained-abbreviation` — acronym detection with baseline + per-profile whitelists
- `lexicon.jargon-undefined` — profile-activated jargon lists (tech, legal, medical, accessibility)
- `lexicon.excessive-nominalization` — per-sentence suffix-based density check
- `rhythm.repetitive-connectors` — sliding-window connector frequency
- `lexicon.low-lexical-diversity` — sliding-window type-token ratio over non-stopword tokens

Style and global:

- `syntax.passive-voice` — EN/FR heuristic `be`/`être` + past participle detector
- `syntax.unclear-antecedent` — bare demonstrative + verb, or paragraph-opening pronoun
- `readability.score` — Flesch–Kincaid grade per document

#### Engine and parser

- `Engine` with `with_profile` and `with_rules` constructors; `lint_str`, `lint_stdin`, `lint_file`.
- Pulldown-cmark-backed Markdown parser extracting paragraphs, sentences, sections, and list items.
- Plain-text parser with blank-line paragraph splitting and CRLF normalization.
- Shared `parser::phrase_search` helper: word-bounded, char-boundary-safe phrase search used by three lexical rules.
- Inline-disable directive: `<!-- lucid-lint disable-next-line <rule-id> -->` (Markdown only).

#### Profiles

- `dev-doc`, `public`, `falc` presets with per-rule threshold sets tuned to each audience.

#### Output

- TTY with color (auto-detected for terminals).
- `--format=json` with a stable schema.

#### CLI

- `lucid-lint check <path>…` accepting Markdown files, plain-text files, and stdin (`-`).
- `--profile` selects the threshold preset.
- Exit code 0 on clean input, 1 when warnings are present.

#### Documentation

- mdBook site (`docs/`) with installation, quick-start, profiles, configuration, CI integration, architecture, accessibility, and roadmap pages.
- WCAG 2.2 AAA typography layer (Atkinson Hyperlegible Next, Literata, OpenDyslexic, Commit Mono) with self-hosted fonts and a reading-preferences control.
- Design principles captured in `CODING_STANDARDS.md` around CUPID (Composable, Unix philosophy, Predictable, Idiomatic, Domain-based) plus YAGNI as the anti-speculation rule.
- Rule reference in `RULES.md`; living roadmap in `ROADMAP.md`.

#### Quality

- Pre-commit hooks: trailing whitespace, EOF, cargo fmt, cargo clippy (`-D warnings`), markdownlint.
- CI matrix: format, clippy, tests on Linux/macOS/Windows, MSRV build, rustdoc, security audit, dogfood.
- MSRV pinned to Rust 1.80.

[Unreleased]: https://github.com/bastien-gallay/lucid-lint/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/bastien-gallay/lucid-lint/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/bastien-gallay/lucid-lint/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/bastien-gallay/lucid-lint/releases/tag/v0.1.0
