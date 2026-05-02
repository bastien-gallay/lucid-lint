# lucid-lint — Roadmap

> Future rules, refinements, and platform extensions tracked from v0.1 onwards.

**Status as of 2026-05-02:** v0.1 shipped 2026-04-20 (17 rules). v0.2.0
shipped 2026-04-22 (25 rules + hybrid scoring + SARIF + condition tags),
v0.2.1 + v0.2.2 shipped 2026-04-23, v0.2.3 shipped 2026-04-29
(`structure.line-length-wide` author-break-aware + encoding hygiene
[F110](#f110)/[F111](#f111)/[F112](#f112) + correctness wins). The **v0.2.x patch cycle is active**:
[F25](#f25) closed 2026-05-01 (FR pair-completeness 41/41); the FR
content-staleness gate is `--strict` on `main` since 2026-05-01 and
on PRs since 2026-05-02 ([F92](#f92) sub-task fully closed);
[F35b](#f35b)/[F35c](#f35c), [F104](#f104), [F105](#f105), [F107](#f107), [F123](#f123) all shipped.
**v0.3 strategy locked 2026-05-02:** the breaking change is the 5-rule
cohort ([F46](#f46) / [F49](#f49) / [F51](#f51) / [F53](#f53) / [F57](#f57)) flipping from default-off to
default-on. Each rule ships in v0.2.x as `Experimental` via the [F139](#f139)
substrate — visible, opt-in for dogfooding, no score regression — then
flips to `Stable` at the v0.3 cut. v0.4 is a horizon bet list.

## Legend

| Status | Meaning |
|---|---|
| ✅ | Done (merged on `main`) |
| 🚧 | In progress |
| ☐ | Not started |

| Priority | Meaning |
|---|---|
| 🔴 Next | Actively queued for the next cut |
| 🟡 Later | Likely someday, not scheduled |
| 🟢 Speculative | Nice-to-have, exploratory |
| — | Shipped; priority meaningless once the item has landed |

---

## At a glance

Version-centric and topic-centric summary views. The sections below
this one are the authoritative topic-centric tables; use them when you
need origin, rationale, or full history. Use this section when you
need to answer "what's next?" or "what's the 0.3 shape?" in a glance.

### Version snapshot

| Version | State | Breaking? | Headline content |
|---|---|---|---|
| v0.1 | ✅ Released 2026-04-20 | — | 17 rules across 5 phases, minimal inline-disable, mdBook site with FR stub |
| v0.2.0 | ✅ Released 2026-04-22 | Yes (rule-id harmonisation) | Hybrid scoring ([F14](#f14)), SARIF ([F32](#f32)), condition tags ([F71](#f71)/[F72](#f72)), 8 new rules (25 total), [F10](#f10) EN/FR auto-formula |
| v0.2.1 | ✅ Released 2026-04-23 | No | Localhost 404.html fix, 3rd per-rule TOML override, fixtures pipeline, TTY GIFs, v0.1/v0.2 prose sweep |
| v0.2.2 | ✅ Released 2026-04-23 | No | FR `syntax.nested-negation` pair-based counting |
| v0.2.3 | ✅ Released 2026-04-29 | No | `structure.line-length-wide` author-break-aware (60+ FR FPs killed), encoding hygiene at the engine boundary ([F110](#f110)/[F111](#f111)/[F112](#f112) — UTF-8 BOM strip + NFC normalisation), strict whitelist validation, library `expect()` removal, scoring clamp invariant |
| **v0.2.x** | 🚧 **In progress** | No | FR translations ([F25](#f25) ✅ closed 2026-05-01), responsive ([F34](#f34)), [F30](#f30) rule-mention linking, [F84](#f84) part 2, [F15](#f15) project roll-up |
| **v0.3** | ☐ Scoped | **Yes** | [F22](#f22) v0.3 slice, [F10](#f10) remainder, 5 condition-tag rules ([F46](#f46)/[F49](#f49)/[F51](#f51)/[F53](#f53)/[F57](#f57)) |
| v0.4 | ☐ Horizon | Varies | LLM plugin ([F16](#f16)), alternative formats ([F5](#f5)–[F8](#f8)), feedback-driven items |

### Feature catalog (active work)

> Filtered to 🔴 Next + 🚧 In-progress. The narrative sections later
> in this file are the source of truth; this catalog is a derived
> index, hand-maintained alongside the narrative. If you spot drift,
> the narrative wins.
>
> Sort: target version (current cycle first) → status (🚧 in-progress
> before ☐ next) → F-ID.

| ID | Topic | Status | Target | Summary |
|---|---|---|---|---|
| <a id="f22"></a>[F22](#f22) | Rules refinement | 🚧 | v0.2.x → v0.3 | Parenthesised-list (Oxford ✅; non-Oxford + interleaved deferred to v0.3 slice) |
| <a id="f84"></a>[F84](#f84) | Example fixtures | 🚧 | v0.2.x | Part 2 — redistributable replacements (3/N closed 2026-05-01) |
| [F139](#f139) | Architecture | ☐ | v0.2.x | Experimental rule status substrate — gates the v0.3 cohort, opens dogfood window |
| <a id="f143"></a>[F143](#f143) | Architecture | ☐ | v0.2.x | Inline AST layer over pulldown-cmark — substrate for [F49](#f49) (gates the cohort lead) |
| [F144](#f144) | Rules refinement | ☐ | v0.2.x | Severity tiering for `lexicon.weasel-words` (quantifier `info`, hedge `warning`) — unblocks the audit-and-PR play |
| [F145](#f145) | Rules refinement | ☐ | v0.2.x | `lexicon.redundant-intensifier` parser miss in bullet / `**strong**` spans (verification slice for [F129](#f129)) |
| [F146](#f146) | Suppression / config | ☐ | v0.2.x | `--severity-floor=warning` CLI flag — narrow-audit shape for external PRs |
| [F-roadmap-slug-ids](#f-roadmap-slug-ids) | Architecture | ☐ | v0.2.x | New ROADMAP entries adopt `F-<slug>` form (legacy F1–F146 stay numeric); slug-uniqueness Rust test enforces, runs offline |
| <a id="f15"></a>[F15](#f15) | Architecture | ☐ | v0.2.x | Project-level scoring roll-up (per-file + summary) |
| <a id="f30"></a>[F30](#f30) | Docs — content | ☐ | v0.2.x | Rule-mention linking pass across guide-prose pages |
| <a id="f34"></a>[F34](#f34) | Docs — reading prefs | ☐ | v0.2.x | Responsive / mobile adaptation |
| <a id="f114"></a>[F114](#f114) | Adoption channels | 🚧 | v0.3 | GitHub Action — composite scaffold internal; v0.3 first cut emits `::warning::` |
| <a id="f10"></a>[F10](#f10) | Rules refinement | ☐ | v0.3 | SMOG / Dale-Chall / Scolarius / `--readability-verbose` |
| <a id="f129"></a>[F129](#f129) | Architecture | ☐ | v0.3 | Markdown parser emits paragraphs for tight list items (correctness) |
| <a id="f46"></a>[F46](#f46) | New rules (v0.3) | ☐ | v0.3 | `lexicon.homophone-density` (slip-flag: FR corpus > 2 d → 0.3.x) |
| <a id="f49"></a>[F49](#f49) | New rules (v0.3) | ☐ | v0.3 | `structure.italic-span-long` |
| <a id="f51"></a>[F51](#f51) | New rules (v0.3) | ☐ | v0.3 | `structure.number-run` |
| <a id="f53"></a>[F53](#f53) | New rules (v0.3) | ☐ | v0.3 | `readability.large-number-unanchored` |
| <a id="f57"></a>[F57](#f57) | New rules (v0.3) | ☐ | v0.3 | `syntax.parenthetical-depth` |
| <a id="f124"></a>[F124](#f124) | Adoption channels | ☐ | v0.3 | npm wrapper (`@lucid-lint/cli-{platform}` `optionalDependencies` pattern) |
| [F133](#f133)–[F136](#f136) | Docs.rs polish | ☐ | v0.3 | `[package.metadata.docs.rs]`, logo + favicon, doctests, `cargo public-api` audit |

### Topic heatmap

Where the active energy is. Counts include 🔴 Next only; shipped items
excluded.

| Topic | v0.2.x 🔴 | v0.3 🔴 | v0.4 bets | Later 🟡 | Speculative 🟢 |
|---|---|---|---|---|---|
| Rules (refinement) | 3 ([F22](#f22) follow-up, [F144](#f144), [F145](#f145)) | 2 ([F10](#f10), [F22](#f22)) | — | [F1](#f1), [F13](#f13), [F24](#f24) | [F2](#f2), [F3](#f3) |
| New rules | — | 5 ([F46](#f46), [F49](#f49), [F51](#f51), [F53](#f53), [F57](#f57)) | [F65](#f65)–[F69](#f69), [F63](#f63) | [F58](#f58), [F59](#f59), [F60](#f60), [F61](#f61) | [F64](#f64), [F70](#f70) |
| Architecture / scoring | 4 ([F15](#f15), [F139](#f139), [F143](#f143), [F-roadmap-slug-ids](#f-roadmap-slug-ids)) | 1 ([F129](#f129)) | [F38](#f38), [F41](#f41) | [F17](#f17), [F38](#f38), [F39](#f39), [F40](#f40) | [F41](#f41) |
| Docs site (bilingual / content / theming / reading) | 2 ([F30](#f30), [F34](#f34)) | — | — | [F36](#f36), [F43](#f43), [F44](#f44), [F73](#f73), [F89](#f89), [F90](#f90)/[F91](#f91) | — |
| Docs.rs polish | — | 4 ([F133](#f133)–[F136](#f136)) | — | — | — |
| Example-text fixtures | 1 ([F84](#f84) part 2) | — | [F85](#f85), [F86](#f86) | [F81](#f81), [F82](#f82), [F83](#f83) | [F86](#f86) |
| Performance / hygiene | — | — | — | [F97](#f97) | — |
| Adoption channels | 1 ([F137](#f137)) | 2 ([F114](#f114), [F124](#f124)) | — | [F115](#f115), [F116](#f116), [F120](#f120), [F125](#f125) | [F122](#f122) (WASM playground) |
| Suppression / config | 1 ([F146](#f146)) | — | — | [F20](#f20), [F21](#f21), [F97](#f97) | — |
| Formats | — | — | [F5](#f5)–[F8](#f8) (single pick) | [F5](#f5)–[F8](#f8) | — |
| Ecosystem interop | — | [F76](#f76) | — | [F76](#f76) | — |
| Plugins / NLP / LLM | — | [F75](#f75) (Should) | [F16](#f16), [F75](#f75) | [F75](#f75) | [F16](#f16) |
| Developer experience | — | [F88](#f88) (narrow `--fix`) | LSP server | [F138](#f138) (`--compare`), [F79](#f79) | [F12](#f12) |
| Research track | — | — | [F74](#f74), [F101](#f101) (user feedback) | — | [F64](#f64), [F70](#f70), [F74](#f74) |

### Cadence and gating

- **v0.2.x** is a **rolling patch cycle**, not a single release target.
  Each Must or Should ships as soon as it's green on `just check` + CI;
  any 🔴-tagged row is eligible to ride the next patch cut.
- **v0.3** opens only when the v0.2.x Must queue is empty and at least
  one breaking change is justified. Until then, breaking changes are
  held — non-breaking items that would otherwise fit 0.3 (e.g. [F39](#f39)
  letter grade) can slide into 0.2.x if they mature first.
- **v0.4** items do not progress by tenure. Each carries an **unlock
  signal** — a concrete event that promotes it from horizon to
  scheduled. See "v0.4 — horizon" at the bottom of this document.

---

## v0.4 — horizon (bets, not commitments)

Routed 2026-04-24 in `.personal/brainstorm/20260424-next-cycles.md`.
Each bet lists the **signal that unlocks it**, so horizon items don't
drift into Must by tenure alone. No commitments; this is "what could
be true in ~6 months if 0.2 and 0.3 land cleanly".

| Bet | Unlock signal |
|---|---|
| [F16](#f16) — `lucid-lint-llm` plugin | ≥ 2 concrete LLM-as-Judge rules designed on paper; deterministic-core base stable enough that non-determinism is a clear opt-in |
| [F5](#f5) / [F6](#f6) / [F7](#f7) / [F8](#f8) — alternative formats (AsciiDoc / HTML / .docx / pandoc bridge) | External user requests; pick the single format with most pull and ship it alone, not the set |
| [F85](#f85) + [F86](#f86) — fixture coverage maps + auto-discovery | Referential has stabilised ([F84](#f84) part 2 done) and rule set stops churning |
| [F63](#f63) — vocabulary-rarity | Lexique.org + COCA frequency lexicons built and licence-cleared |
| [F65](#f65) – [F69](#f69) — remaining condition-tag rules | [F46](#f46) / [F49](#f49) / [F51](#f51) / [F53](#f53) / [F57](#f57) validated in the wild at 0.3 |
| [F38](#f38) — section-level scoring | Document + project level proven; users ask "which H2 is the problem?" |
| [F41](#f41) — reading-time unit | Validated heuristic exists; companion metrics (comfort, fatigue, understandability) defined |
| [F12](#f12) — score-evolution dashboard | CI users explicitly ask for trend view (not delta — delta is [F138](#f138) / `--compare`) |
| [F76](#f76) — interop suppression (if not shipped in 0.3) | A second rule joins `deeply-nested-lists` as a markdownlint overlap |
| [F74](#f74) — rule-discovery corpus mining | Student / intern resource available; separate research track |
| LSP server | Editor demand visible (Cursor / VSCode issues); would change the deployment story |
| [F70](#f70) / [F64](#f64) — research-track rules | Only if someone codes them for fun |
| <a id="f101"></a>**[F101](#f101) — top 3 items from first-10-external-users feedback (TBD)** | 0.2.0 ships and ≥ 10 non-maintainer users exist — placeholder reserved so the horizon isn't 100 % maintainer bets (renumbered from [F98](#f98) post-collision with stream-2 cargo-mutants) |
| <a id="f140"></a>**[F140](#f140) — metaphor / analogy / comparison detection (NLP or LLM plugin).** Cognitive-load grounded: figurative language costs extra inference for tired readers, aphasia, L2 readers, and is a known axis for ASD (currently out of v0.2/v0.3 scope). Belongs in `lucid-lint-nlp` ([F75](#f75)) or `lucid-lint-llm` ([F16](#f16)) — non-deterministic, so plugin-only per prime directive #4. Bilingual-viable concern: idiomatic FR vs EN metaphors don't map; FR + EN paths need separate corpora at proposal time. | Either NLP or LLM plugin scaffolding lands AND a dogfood / external case surfaces a missed metaphor that confused a reader |

**Deliberately off the 0.4 list:**

- [F39](#f39) / [F40](#f40) letter grade + traffic light — routed to 0.3 Should; if
  they slip they go to 0.3.x, not 0.4.
- Full [F29](#f29) numeric codes — parked until a rename actually happens.
- [F2](#f2), [F3](#f3) speculative rule refinements — stay speculative until a
  concrete dogfood case surfaces.
- [F17](#f17) per-family sub-scores — category sub-scores ([F14](#f14)) already ship;
  unclear what "family" adds beyond that.

---

## v0.3+ — Advanced plugins

### LLM-enhanced detection

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f16"></a>[F16](#f16) | `lucid-lint-llm` plugin (LLM-as-Judge rules) | 🟢 Speculative | Research on existing tools |

The plugin would add rules like `unclear-antecedent-semantic` that use an LLM to detect semantic ambiguities the pattern-based heuristics miss.

Disabled by default due to non-determinism, API cost, and latency incompatible with pre-commit hooks.

### Advanced NLP

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f75"></a>[F75](#f75) | `lucid-lint-nlp` plugin specification and scaffolding (Python subprocess or WASM-based). Replaces heuristic rules with POS- / dependency-tree- / anaphora-backed precise versions. **Ship only when the first plugin rule is concretely scheduled** — scaffolding-without-consumer is the red flag from AGENTS.md directive #1 (2026-04-24 brainstorm-next-cycles). | 🟡 Later | Rule-system-growth brainstorm (2026-04-20) |

Candidate rules for the plugin:

- POS-based `syntax.passive-voice` detection (replaces v0.1 heuristic)
- Full anaphora resolution for `syntax.unclear-antecedent`
- Dependency-tree-based `structure.deep-subordination`
- Semantic similarity between adjacent sentences (discourse cohesion signal inspired by Coh-Metrix)

### New rules (v0.3 candidates)

Deferred from v0.2 because they require corpus work, lexicon builds, or
depend on earlier features ([F9](#f9), [F14](#f14)). Naming uses the provisional
`category.rule-name` prefix pending [F29](#f29).

| ID | Rule | Category | Tags | Grounding | Depends on |
|---|---|---|---|---|---|
| [F46](#f46) | `lexicon.homophone-density` | Lexicon | `dyslexia` | BDA (dyslexia) | FR corpus tuning; ships as `info`. Slip-flag (2026-04-24): if FR corpus tuning exceeds ~2 days, slides to 0.3.x. Ships as `Experimental` in v0.2.x via [F139](#f139); flips to `Stable` at v0.3 cut. |
| [F49](#f49) | `structure.italic-span-long` | Structure | `dyslexia` | BDA | Cohort lead (2026-05-02) — first rule on the [F139](#f139) substrate, depends on [F143](#f143) (inline AST layer). Ships as `Experimental` in v0.2.x; flips to `Stable` at v0.3 cut. |
| [F51](#f51) | `structure.number-run` | Structure | `dyscalculia` | plainlanguage.gov | Ships as `Experimental` in v0.2.x via [F139](#f139); flips to `Stable` at v0.3 cut. |
| [F53](#f53) | `readability.large-number-unanchored` | Readability | `dyscalculia`, `general` | CDC CCI | Ships as `Experimental` in v0.2.x via [F139](#f139); flips to `Stable` at v0.3 cut. |
| [F57](#f57) | `syntax.parenthetical-depth` | Syntax | `adhd`, `general` | plainlanguage.gov, Hemingway | Ships as `Experimental` in v0.2.x via [F139](#f139); flips to `Stable` at v0.3 cut. |
| <a id="f58"></a>[F58](#f58) | `syntax.front-loaded-subject-delay` | Syntax | `adhd`, `general` | plainlanguage.gov | FR corpus validation (dislocation FP risk) |
| <a id="f59"></a>[F59](#f59) | `rhythm.pronoun-density` | Rhythm | `aphasia`, `general` | FALC | — |
| <a id="f60"></a>[F60](#f60) | `rhythm.topic-shift-cluster` | Rhythm | `adhd`, `general` | Hemingway | May merge into [F13](#f13) after corpus review |
| <a id="f61"></a>[F61](#f61) | `lexicon.falc-idiom` | Lexicon | `aphasia`, `non-native` | IFLA, FALC | Curated bilingual idiom lexicon |
| <a id="f63"></a>[F63](#f63) | `lexicon.vocabulary-rarity` | Lexicon | `non-native`, `general` | — | Frequency lexicon per language (Lexique.org for FR, COCA / Google-Books for EN). Tiered weights: `common` / `context-dependent` / `expert`. LLM-built fallback only. |
| <a id="f65"></a>[F65](#f65) | `rhythm.forward-reference-heavy` | Rhythm | `adhd`, `general` | Working-memory load | — |
| <a id="f66"></a>[F66](#f66) | `lexicon.acronym-distance-from-definition` | Lexicon | `adhd`, `non-native` | Memory decay | [F9](#f9) (definition-aware abbreviation) |
| <a id="f67"></a>[F67](#f67) | `syntax.complex-tense` | Syntax | `non-native`, `aphasia` | FALC tense restrictions | FR morphology primary; EN lighter |
| <a id="f68"></a>[F68](#f68) | `syntax.impersonal-voice-heavy` | Syntax | `aphasia` | FALC direct-address rule | — |
| <a id="f69"></a>[F69](#f69) | `syntax.address-inconsistency` | Syntax | `non-native`, `general` | Register consistency | FR primary (tu / vous); EN weaker (you / one) |

### Developer experience (v0.3)

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f138"></a>[F138](#f138) | Differential diagnostics — `--compare=<ref>` CLI mode. Runs against two revisions of the same text(s) and reports score-delta + diagnostic-delta. Pitch: CI/PR comment framing ("this PR adds 2 warnings, removes 5, net −3"), inverting alarm fatigue the way coverage tools do. CLI + JSON + SARIF-run-comparison. No dashboard (that is [F12](#f12)). | 🟡 Later | Rule-system-growth brainstorm (2026-04-20). Depends on [F14](#f14) stabilising. |
| <a id="f79"></a>[F79](#f79) | Fancy terminal rendering for `lucid-lint explain` — pipe the bundled markdown through `termimad` (or a custom `pulldown-cmark` + `owo-colors` walker) so headings, tables, code fences, bullets, and inline `code` render with proper typography instead of raw markdown. Ship a toned `Skin` that matches the existing warning-yellow / info-cyan palette rather than termimad's magenta defaults — the brand direction is calm, typographic, not "rich CLI". Defer past v0.2 so the `check` output polish (F?) lands first. | 🟡 Later | TTY-output critique (2026-04-22) |

### Ecosystem interop

Motivation: lucid-lint and Markdown-syntax linters (markdownlint, Vale,
proselint, textlint) can flag the same line from different angles.
Cognitive-load rules that happen to share a substrate with a structural
check should stay shipped in core — users without markdownlint, users
who disabled the matching markdownlint rule, and users feeding
non-Markdown input (plain text, .docx via [F7](#f7), HTML via [F6](#f6)) all rely on
lucid-lint for that coverage. The pain point is editor LSP sessions
where two servers report the same span with different severities and
different wording, not CLI pipelines where tools run sequentially.

Scope audit at 2026-04-20: after the `structure.heading-jump` reframing (cognitive
"comprehension cliff" at skip ≥ 2 levels, distinct from MD001's strict
+1 rule), **`structure.deeply-nested-lists` is the only lucid-lint rule that
remains functionally equivalent to a markdownlint rule (MD007)**. The
mechanism below is designed to scale — Vale, proselint, textlint
overlaps are likely as the rule set grows — rather than to solve a
single-rule problem.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f77"></a>[F77](#f77) | ✅ Shipped in v0.2 — `main.rs` now auto-discovers `lucid-lint.toml` walking up from the CWD (stopping at the nearest `.git` boundary) and applies `[default].profile`, `[default].conditions`, `[scoring]` via `ScoringFileConfig::into_scoring_config`, and `[rules.readability-score].formula`. New `--config <path>` flag overrides discovery. Precedence: built-in profile defaults → TOML → CLI flags. Per-rule TOML overrides beyond `readability.score` extend rule-by-rule as each `Config` gains `Deserialize`. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md). | — | [F11](#f11) follow-up (2026-04-21) |
| <a id="f76"></a>[F76](#f76) | Interop suppression mechanism. Rules declare overlapping external linter rules in their metadata (e.g. `Rule::external_overlaps() -> &[(Linter, &'static str)]`, enum `Linter::Markdownlint \| Vale \| Proselint \| Textlint`). Users opt in via `[interop] suppress_when = ["markdownlint"]` in `lucid-lint.toml` (CLI equivalent: `--interop-suppress=markdownlint`); opt-out is default, so coverage never silently drops. When active, affected rules are skipped at emission time with an info-level trace in `--verbose`. Ships CLI + LSP (the LSP path is the real motivator: two servers squiggling the same span with different severities and wording erodes trust in both). Only `structure.deeply-nested-lists` qualifies at time of writing (MD007); framework is designed to scale to future overlaps. Non-goal: detecting whether the external linter is actually installed or configured — the config field is the signal. | 🟡 Later | Markdownlint-overlap scan (2026-04-20) |

### Adoption channels

Filed 2026-04-25 from the adoption-channels brainstorm
(`.personal/brainstorm/20260425-adoption-channels.md`). This section
tracks **distribution and integration channels** — work that lives
in this repo (release artifacts, plugins, docs pages, IDE / CI
integrations).

Pure promotion / outreach plays (DINUM submission, awesome-list PRs,
audit-and-PR on famous OSS docs, W3C COGA submission, conference
talks, social-media cadence, Hacker News, etc.) moved to
`.personal/promotion-channels.md` on 2026-05-01. The freed F-IDs
(F111, F112, F113, F117, F118, F119) are considered **lost** — not
reused. F110 (Vale style pack — code) was renumbered to F137 to
free F110 for the encoding-hygiene canonical entry already shipped
in v0.2.3.

The regulatory tailwind (EAA enforceable since 2025-06-28; RGAA 5
ships end-2026 with DGCCRF / Arcom sanctions up to 50k€ + renewable)
shapes the must-list — [F137](#f137) (Vale pack) leans directly on it. Bilingual
EN/FR is the differentiator that makes the FR-government channel
viable.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f137"></a>[F137](#f137) | **Vale style pack — subset of rules → `vale-cli/packages` topic.** Map only the rules that fit Vale's `existence` / `substitution` / `occurrence` checks (target list: `lexicon.weasel-words`, `lexicon.redundant-intensifier`, `lexicon.jargon-undefined`, `lexicon.unexplained-abbreviation`, `lexicon.all-caps-shouting` — plus a couple thresholded `structure` rules if Vale's `conditional` extends cleanly). The cognitive-load core (sentence-too-long thresholds, `structure.deep-subordination`, scoring engine, FALC profile) stays standalone-only. Pack is **generated** from the rule registry (~50 lines of Rust emitting Vale YAML) — zero hand-maintenance, regenerated per release. Each rule's Vale `link:` field points to `docs/src/rules/<id>.md` so curiosity about gaps surfaces the standalone tool. Pack README opens with: *"This is a subset of `lucid-lint` for Vale users. For sentence-shape, paragraph rhythm, scoring and the FALC profile, use `lucid-lint` standalone — see `[link]`."* The Vale pack is intentionally a "trailer." Risks (discovery dilution, identity blur, maintenance drag) all fall on the README + per-rule link surfaces; not cannibalisation — Vale users are a new audience, not poached existing users. | 🔴 Next | Adoption-channels brainstorm 2026-04-25 |
| [F114](#f114) | **GitHub Action in Marketplace** (promoted to 🔴 Next, targeted at v0.3 from 2026-04-27 Block E recon — early-adoption feedback channel). Verified peer shape: both `astral-sh/ruff-action` and `biomejs/setup-biome` are thin **composite actions** (yaml-only) that download the prebuilt binary from upstream Releases, add it to `PATH`, optionally run it. Composite > Docker container for sub-second cold start; pure JS action avoided (no Node runtime needed). Proposed contract: `uses: lucid-lint/lucid-lint-action@v1` with `with:` inputs `version` (default `latest`), `paths`, `profile` (`falc` / `dev-doc` / `public`), `format` (`tty` / `json` / `sarif`), `min-score`. v0.3 first cut emits `::warning file=…,line=…::` workflow commands for inline PR annotations; v0.4 swaps to SARIF upload via `github/codeql-action/upload-sarif` once the SARIF output stabilises, feeding GitHub Code Scanning natively. Risk: a composite action coupled to `cargo-dist` release-tarball naming — any rename breaks consumers, so pin the manifest contract. **Internal scaffold landed 2026-04-28** — `action.yml` at the repo root implements the locked input contract (`version`, `paths`, `profile`, `format`, `min-score`, plus `working-directory` and passthrough `args`); a smoke workflow (`.github/workflows/action-smoke.yml`) exercises it on Linux / macOS / Windows runners against this repo's own `docs/src/`. Not yet published, not yet `v1`-tagged, not yet listed in the Marketplace. Bake-in plan: dogfood the contract internally for 2–3 weeks, revise inputs that don't survive contact with reality, then split out to a dedicated `bastien-gallay/lucid-lint-action` repo (the canonical ruff / biome pattern) and tag `v1` alongside the v0.3 release. v0.3 first cut still emits `::warning::`; SARIF upload deferred to v0.4 behind [F32](#f32). | 🔴 Next | Adoption-channels brainstorm 2026-04-25 + Block E recon 2026-04-27 + scaffold 2026-04-28 |
| <a id="f115"></a>[F115](#f115) | **FALC-readiness guide page** — new docs page `docs/src/guide/falc-readiness.md` (FR mirror at `docs/src/fr/guide/falc-readiness.md`) explaining how `lucid-lint --profile=falc` maps to the Inclusion Europe European Easy-to-Read standards. Cite the European Easy-to-Read logo program (logo use is free if conditions met: document follows the standards + at least one person with intellectual disability validated readability). **Do not claim certification** — claim *readiness*. The guide drives qualified traffic from disability-federation networks (UNAPEI, Inclusion Europe, etc.). | 🟡 Later | Adoption-channels brainstorm 2026-04-25 |
| <a id="f116"></a>[F116](#f116) | **mdbook-lint coexistence guide.** Short page in our docs (and a one-liner cross-PR to mdbook-lint's README) explaining "use both": mdbook-lint = markdown structure, `lucid-lint` = prose / cognitive load. Different niches, complementary. Free, opportunistic. | 🟢 Could | Adoption-channels brainstorm 2026-04-25 |
| <a id="f120"></a>[F120](#f120) | **Pre-commit hook listing in `pre-commit/pre-commit` registry.** Fires once `--check` mode is stable across our CLI surface (currently most surfaces use `--format=json` and exit codes; hook-friendly summary + fast-fail mode is the prerequisite). | 🟢 Could | Adoption-channels brainstorm 2026-04-25 |
| <a id="f122"></a>[F122](#f122) | **WASM playground for in-browser linting.** Peer pattern (ruff `play.ruff.rs`, biome `biomejs.dev/playground`): single-page React/Preact + Vite app driving a Monaco editor, with a dedicated `*_wasm` Rust crate built via `wasm-pack` (ruff publishes `ruff_wasm`; biome publishes `@biomejs/wasm-web` from `biome_wasm`). Source layout: a `playground/` workspace at repo root with `wasm/` and `web/` sub-trees. Hosting: Cloudflare Pages or GitHub Pages on a subdomain (e.g. `play.lucid-lint.dev`). Proposed shape for `lucid-lint`: `crates/lucid-lint-wasm` exposing `lint(text, lang, profile) -> Diagnostic[]` via `wasm-bindgen`; tiny Vite+Preact UI; estimated 300–600 kB gzipped given our deterministic core (no network, no LLM). Phase: **v0.4+** — the surface needs its own brainstorm before scoping (UX shape, share-link encoding, persistence, mobile experience, contribution channel) and is best framed as a traction / acquisition lever once v0.3 distribution is in place. Risks: (1) bundle-size cliff if `regex` + `unicode-segmentation` push past 1 MB; (2) ongoing maintenance of a JS surface that can drift from CLI behaviour. | 🟢 Speculative | 2026-04-27 Block E recon |
| <a id="f123"></a>[F123](#f123) | ✅ Shipped 2026-04-28 — curl-pipe-sh + PowerShell one-liners are surfaced in `README.md` and `docs/src/guide/installation.md`. **The cargo-dist installer flip itself was a no-op** — `installers = ["shell", "powershell"]` has been in `Cargo.toml` `[workspace.metadata.dist]` since the initial scaffold (`d153ad8`), so v0.1.1 / v0.2.0 / v0.2.1 / v0.2.2 have all been attaching `lucid-lint-installer.sh` and `lucid-lint-installer.ps1` to their GitHub Releases. Yesterday's Block E recon mis-filed [F123](#f123) as a config flip; today's reconnaissance confirmed the actual gap was discoverability. Documentation now covers both one-liners (Linux / macOS / WSL via `curl … \| sh`; Windows via PowerShell `irm \| iex`), the `--check` / audit-before-running pattern (download to a file, `less`/`notepad`, then execute), version pinning (`releases/download/v<version>/…` instead of `releases/latest/…`), and how each installer drops the binary on `$PATH`. The `cargo install` and source-build routes stay alongside as fallbacks. README's stale "Once released to crates.io" lead-in dropped. Vanity `sh.lucid-lint.dev` redirect remains a v0.5 concern. | — | 2026-04-27 Block E recon |
| [F124](#f124) | **npm wrapper with platform `optionalDependencies`** — promoted to 🔴 Next, targeted at v0.3 (early-adoption feedback channel for the JS-toolchain audience: Prettier / ESLint / Husky / package.json scripts users). Canonical pattern verified on the npm registry: biome (`@biomejs/biome` 2.4.13) and dprint (0.54.0) both publish a thin root package whose `optionalDependencies` lists one sub-package per target; npm resolves only the matching platform; root `bin` shim execs the binary; dprint additionally runs a `postinstall` `install.cjs` as fallback. Proposed shape: root `lucid-lint` (~10 kB) + five platform-specific `@lucid-lint/cli-{aarch64-apple-darwin, x86_64-apple-darwin, x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl, x86_64-pc-windows-msvc}`. Version stays in lockstep with the Rust crate; release workflow gains an `npm publish --provenance` step using OIDC (biome already does this). Risks: (1) 5+ packages per release multiply publish-failure surface — release workflow needs all-or-nothing semantics; (2) npm registry outages would block JS users — document fallback to direct binary download ([F123](#f123)). | 🔴 Next | 2026-04-27 Block E recon |
| <a id="f125"></a>[F125](#f125) | **Homebrew distribution (own tap → core).** macOS-first audiences (writers, designers, docs teams) reach for `brew install` before `cargo`. Path is well-trodden: ship a tap on `<org>/homebrew-tap` immediately, graduate to `homebrew-core` once eligibility is met (current acceptable-formula policy needs a manual cross-check on `homebrew/brew docs/Acceptable-Formulae.md` — sandboxed during Block E recon; the old "75 stars" line was removed but maintainers still gate on adoption signal). Implementation: enable `cargo-dist`'s `homebrew` installer — it generates a Ruby formula referencing the same release tarballs we already build (`aarch64-apple-darwin`, `x86_64-apple-darwin`, plus Linux bottles) and opens a PR against our tap on each tag. Bottle building runs free on `macos-latest` runners. v0.4 launches the tap; `homebrew-core` submission deferred to v0.5+ behind real adoption signal. Risks: (1) tap fragmentation if we never graduate to core; (2) core review can take weeks. | 🟡 Later | 2026-04-27 Block E recon |

### Research track

Bets that don't commit to a ship date. Tracked to ensure they're not
forgotten.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f64"></a>[F64](#f64) | `structure.paragraph-landmark-density` — reprise-points for attention-fragile readers. Research needed to define "landmark" (bold / italic / headers / list-starts / code spans?). | 🟢 Speculative | Rule-system-growth brainstorm (2026-04-20) |
| <a id="f70"></a>[F70](#f70) | `structure.lede-buried` — journalistic inverted-pyramid check. Strong candidate for a future `lucid-lint-journalism` plugin rather than core. | 🟢 Speculative | Rule-system-growth brainstorm (2026-04-20) |
| <a id="f74"></a>[F74](#f74) | Rule-discovery corpus project — mine writer-heavy git histories for patterns that authors repeatedly rewrite. Source of evidence-grounded rule proposals. Intern / student project scale. | 🟢 Speculative | Rule-system-growth brainstorm (2026-04-20) |

Additional research directions captured for posterity but not yet ID'd:

- **Reader-model scoring** — tiny local model predicts processing time
  and accuracy per paragraph; output is a cognitive-load heatmap.
  Deterministic at inference, data-hungry at training.
- **TTS / screen-reader prosody** rules — detect prosody breakdown
  (mid-sentence acronyms, awkward punctuation cadence). Needs a TTS
  corpus.
- **Cross-document terminology drift** — same concept named three ways
  across a corpus ("user" / "customer" / "client"). Requires
  multi-file analysis infrastructure; performance implications.
- **Eye-tracking corpus collaboration** — partnership with a reading
  lab to ground thresholds in behavioural data.
- **LSP server** — live diagnostics in editors; same core, different
  frontend.
- **`--fix` / quickfix suggestions** — safe rules only (e.g.
  `structure.long-enumeration` → concrete list skeleton). Controversial for
  prose; needs guardrails.
- **`lucid-lint baseline`** — record per-project medians; rules flag
  regressions rather than absolutes (ESLint-style).
- **Profile composition** (`extends = "falc"`) — reduce duplication
  across projects.
- **Community rule-pack registry** — cargo-style publication of domain
  packs (medical, legal, edu, journalism).
- **`lucid-lint-style` plugin** — adverb overuse, show-don't-tell, and
  other aesthetic rules excluded from core by design.
- **`lucid-lint-a11y` plugin** — alternative home for `a11y-markup`-
  tagged rules if the tag proves insufficient to separate them from
  prose rules.

---

## v0.2 / v0.2.x — Must-ship shipped, patch cycle in progress

### Release cadence

The 2026-04-22 reprioritisation favoured a **tight 0.2.0 cut over a
fat one**: anything non-blocking slides to 0.2.x patch releases, which
exist precisely to absorb per-rule polish and per-surface slices.
v0.2.0, v0.2.1, and v0.2.2 are shipped; v0.2.x remains open as a
rolling patch cycle. 0.2.x routing was reviewed on 2026-04-24
in `.personal/brainstorm/20260424-next-cycles.md` (not tracked;
`.personal/` is gitignored).

### v0.2.0 — Blocking items (all ✅ shipped 2026-04-22)

| ID | Summary |
|---|---|
| [F29](#f29)-slim | Rule IDs moved to `category.rule-name` form (25 rules); `src/rules/<cat>/` subdirectories; `Category::for_rule` derives from prefix. Hard break — suppression directives, `[rules.<id>]` TOML keys, JSON/SARIF `ruleId` all use the new form. |
| <a id="f35a"></a>[F35a](#f35a) | `theme/index.hbs` forked from upstream mdBook; skip link + EN / FR switch server-rendered. WCAG 2.4.1 Bypass Blocks passes with JS disabled. |
| <a id="f35d"></a>[F35d](#f35d) | Accessibility statement page (`docs/src/accessibility.md` + FR counterpart). |
| <a id="f80"></a>[F80](#f80) | `--fail-on-warning` accepts optional boolean; hidden mirror `--no-fail-on-warning`. `--min-score` now testable in isolation on documents with warnings. |

### v0.2.1 — ✅ Released 2026-04-23

Localhost 404.html rendering fix ([F84](#f84) part 1), per-rule TOML override
for `structure.excessive-commas` (third rule wired after
`readability.score.formula` and `lexicon.unexplained-abbreviation.whitelist`),
scraped-prose fixtures pipeline (`examples/texts.yaml` + `just texts`),
TTY-capture GIFs via `vhs` tapes, v0.1 / v0.2 staleness sweep, idea-highlight
motif extended to the `structure.sentence-too-long` rule page. First
crates.io publish since v0.1.1 — packaging switched from `exclude` to
an explicit `include` list so `docs/src/rules/*.md` reach the tarball
(needed by `src/explain.rs`'s `include_str!`).

### v0.2.2 — ✅ Released 2026-04-23

[F87](#f87) — FR `syntax.nested-negation` pair-based counting over `ne` / `n'`
clitics and second-position particles (`pas`, `rien`, `jamais`, …).

### v0.2.x — MoSCoW routing (patch cycle, post-release)

Routed 2026-04-24 from the active-work view. Each row here has a
full entry under a topic section below; priority column reflects the
routing decision.

#### Must — 🔴 Next

| ID | Topic | Item |
|---|---|---|
| <a id="f25"></a>[F25](#f25) | Docs — bilingual | ✅ Closed 2026-05-01 — per-rule 25/25 + guides 8/8 + architecture 2/2 + contributing |
| [F34](#f34) | Docs — reading prefs | Responsive / mobile adaptation |
| <a id="f35b"></a>[F35b](#f35b) | Docs — reading prefs | Drop `role="radiogroup"` on reading chips (P2 a11y) |
| [F84](#f84) part 2 | Example-text fixtures | Redistributable replacements for load-bearing slots |
| [F137](#f137) | Adoption channels | Vale style pack (subset of rules → `vale-cli/packages` topic) |
| [F139](#f139) | Architecture | Experimental rule status substrate — gates v0.3 cohort, opens dogfood window |
| [F143](#f143) | Architecture | Inline AST layer over pulldown-cmark — substrate for [F49](#f49) (cohort lead) |
| [F144](#f144) | Rules refinement | Severity tiering for `lexicon.weasel-words` (quantifier `info`, hedge `warning`) — unblocks the audit-and-PR play |
| [F145](#f145) | Rules refinement | Fix `lexicon.redundant-intensifier` parser miss in bullet / `**strong**` spans — unblocks the audit-and-PR play |
| [F146](#f146) | Suppression / config | `--severity-floor` CLI flag — unblocks the audit-and-PR play narrow-audit shape |

#### Should — ships as the next patch absorbs it

| ID | Topic | Item |
|---|---|---|
| [F15](#f15) | Architecture | Project-level scoring roll-up (per-file + summary) |
| — | Suppression / config | Per-rule TOML plumbing, rule-by-rule as each `Config` gains `Deserialize` |
| <a id="f20"></a>[F20](#f20) | Suppression / config | `reason="..."` field on suppression directives |
| [F30](#f30) | Docs — content | Rule-mention linking audit + coverage test ([F44](#f44)) |
| [F114](#f114) | Adoption channels | GitHub Action published to Marketplace (depends on stable SARIF output) |
| [F115](#f115) | Adoption channels | FALC-readiness guide page citing Inclusion Europe standards |
| [F-roadmap-slug-ids](#f-roadmap-slug-ids) | Architecture | ROADMAP feature IDs adopt `F-<kebab-slug>` for new entries; legacy F1–F146 stay numeric; slug-uniqueness CI test (offline-runnable) |

#### Could — nice-to-have

[F24](#f24) (nominalization suffix refine), [F43](#f43) (RULES.md drift cleanup), [F73](#f73)
(font-leak CI gate), [F36](#f36) (final polish pass), [F79](#f79) (fancy `explain`
rendering), [F21](#f21) (`disable-file`), [F81](#f81) / [F82](#f82) / [F83](#f83) (fixture hygiene),
**[F116](#f116)** (mdbook-lint coexistence guide),
**[F120](#f120)** (pre-commit hook listing once `--check` mode stabilises).

#### Won't (pushed to 0.3)

[F39](#f39) letter grade, [F40](#f40) traffic light, [F17](#f17) per-family sub-scores, [F89](#f89)
`.lucid-stance` unify, [F88](#f88) `--fix` mode (narrow).

### v0.3 and later (already scoped)

Detail under "New rules (v0.3 candidates)" and the `## v0.4 — horizon`
section below.

- **[F22](#f22) v0.3 slice** — 3–4-word Oxford items, non-Oxford / "plus"-closed
  lists, interleaved parentheticals (first slice shipped in 0.2.x).
- **[F10](#f10) remainder** — SMOG, Dale-Chall, Scolarius,
  `--readability-verbose`.
- **Five condition-tag rules** — [F46](#f46), [F49](#f49), [F51](#f51), [F53](#f53), [F57](#f57). [F46](#f46) carries a
  slip-flag: if FR corpus tuning for homophone density exceeds ~2 days,
  it slides to 0.3.x.
- **Full [F29](#f29)** — demoted to 🟢 Speculative on 2026-04-24. [F29](#f29)-slim
  already fixed the category-drift problem by construction; numeric
  codes (`STR-001`) only earn their cost on a real rename, and there are
  zero scheduled renames. Revisit when one actually happens.

### Architecture

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f14"></a>[F14](#f14) | ✅ Hybrid scoring model shipped in v0.2 (global score + per-category sub-scores + diagnostics). `X/max` arbitrary-max at both levels, 5 fixed categories (Structure · Rhythm · Lexicon · Syntax · Readability), composition = weighted sum × density-normalization × per-category cap, `weight` field added to `Diagnostic`, `--min-score=N` CLI flag. See [`docs/src/guide/scoring.md`](docs/src/guide/scoring.md). Letter-grade / traffic-light / reading-time decorations deferred ([F39](#f39)–[F41](#f41)). | — | Architecture decision discussion |
| [F15](#f15) | 🚧 Document-level scoring shipped in v0.2 (multi-path runs are aggregated as one document). Project-level roll-up (per-file breakdown + project summary) still open. Section-level deferred → [F38](#f38). | 🔴 Next | Linked to [F14](#f14) |
| <a id="f17"></a>[F17](#f17) | Per-family sub-scores | 🟡 Later | Linked to [F14](#f14) |
| <a id="f32"></a>[F32](#f32) | ✅ Shipped in v0.2 — `lucid-lint check --format=sarif` emits a SARIF v2.1.0 log compatible with GitHub Code Scanning. One rule descriptor per observed rule id (category, default severity, default weight, `helpUri` to the per-rule mdBook page); per-result properties carry weight + section. Workflow snippet in [`docs/src/guide/ci-integration.md`](docs/src/guide/ci-integration.md#github-code-scanning-sarif). | — | v0.1 AGENTS.md audit |
| <a id="f37"></a>[F37](#f37) | ✅ Rule-message clarity audit completed: all 17 rules reviewed against "what do I change?" bar. 15 rules already actionable; `structure.heading-jump` updated (first-heading-not-H1 and missing-H1 variants now include repair guidance). `readability.score` info variant left observational by design (fires only when `always_report` is set). | — | [F14](#f14) `brainstorm/20260420-score-semantics.md` |
| <a id="f38"></a>[F38](#f38) | Section-level granularity for scoring (deferred from [F15](#f15)) — per-heading sub-scores once document + project are proven in the wild. | 🟡 Later | [F14](#f14) `brainstorm/20260420-score-semantics.md` |
| <a id="f39"></a>[F39](#f39) | Letter-grade decoration (A–F) on the `X/max` score — promote when user feedback shows the numbers feel noisy or hard to compare across docs. | 🟡 Later | [F14](#f14) `brainstorm/20260420-score-semantics.md` |
| <a id="f40"></a>[F40](#f40) | Traffic-light (🔴🟡🟢) + pass/fail margin in the TTY output — promote when CI users ask for a stronger glance signal than the number alone. | 🟡 Later | [F14](#f14) `brainstorm/20260420-score-semantics.md` |
| <a id="f41"></a>[F41](#f41) | Reading-time-seconds as an alternative score unit — ties score to concrete user outcome. Requires validated heuristic + companion metrics (comfort, fatigue, understandability) so the time unit doesn't monopolize the read. | 🟢 Speculative | [F14](#f14) `brainstorm/20260420-score-semantics.md` |
| <a id="f71"></a>[F71](#f71) | ✅ Shipped in v0.2 — `ConditionTag` enum (fixed 7-variant ontology: `a11y-markup`, `dyslexia`, `dyscalculia`, `aphasia`, `adhd`, `non-native`, `general`) plus `Rule::condition_tags()` trait method (default `&[General]`). All 17 v0.2 rules are `general`; future tagged rules ([F48](#f48), [F55](#f55), [F56](#f56)) opt in by overriding. See [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md). | — | Rule-system-growth brainstorm (2026-04-20) |
| [F129](#f129) | **Markdown parser — emit paragraphs for tight list items (correctness fix).** Discovered 2026-05-01 while verifying the [F22](#f22) third-tranche dogfood metric: the same bullet content that triggers `excessive-commas` / `dense-punctuation-burst` / `readability.score` in a *loose* list (multiple items separated by blank lines) is silent in a *tight* list (single item, or items without separating blank lines). Root cause: `pulldown-cmark` only emits `Tag::Paragraph` events for items in loose lists; tight-list text events fire directly inside `Tag::Item`. The parser at `src/parser/markdown.rs` only buffers text inside heading or paragraph contexts, so tight-list content goes into the void and every paragraph-level rule (all 17 in v0.1) inherits the blindspot. Same pre-existing limitation flagged in [F126](#f126) for `structure.line-length-wide` — [F129](#f129) resolves it once for every rule. Fix: synthesize a paragraph for each list-item span when no `Tag::Paragraph` event fires inside it. Expected dogfood impact: many CHANGELOG / release-note / README bullets become newly visible to rules — some genuine new diagnostics, some snapshot updates. | 🔴 Next | [F22](#f22) third-tranche verification (2026-05-01) |
| <a id="f72"></a>[F72](#f72) | ✅ Shipped in v0.2 — `[default] conditions = [...]` config field and `--conditions` CLI flag (comma-separated). Filter semantics: rules tagged `general` always run; tagged-only rules run iff their tags intersect the active list. Profiles unchanged; FALC retains its regulatory meaning. See [`docs/src/guide/conditions.md`](docs/src/guide/conditions.md). | — | Rule-system-growth brainstorm (2026-04-20) |
| <a id="f143"></a>[F143](#f143) | **Inline AST layer over pulldown-cmark — substrate for inline-positional rules.** Routed 2026-05-02 (`.personal/brainstorm/20260502-parser-substrate-choice.md`). The current Markdown parser at `src/parser/markdown.rs` flattens emphasis, strong, and link spans into the `Paragraph.text` string before rules see it — visible text preserved, structure lost. [F49](#f49) (`structure.italic-span-long`, cohort lead) needs italic-span boundaries; future inline-positional rules ([F64](#f64) speculative, [F66](#f66) conditional on [F9](#f9)) would hit the same wall. **Decision:** introduce a thin typed inline AST on top of pulldown-cmark, *not* swap the engine for comrak / markdown-rs. Reasons: pulldown stays the perf-leading parser; the AST is the *domain model* the rules walk (CUPID-aligned: composable, predictable, domain-based); the engine swap regresses bench by ≈ 2–3× and collides with the lightning-fast positioning pillar. **Minimal viable substrate (YAGNI applied inside the layer):** `enum Inline { Text(String), Emphasis(Vec<Inline>) }` plus a `Paragraph.inline: Vec<Inline>` field captured during the existing pulldown walk. **Not modeled yet:** `Strong`, `Link`, `Code`, footnotes, task-list markers, hard breaks inside emphasis. Each gets added when a second rule actually demands it; today only F49 does, and the steel-man check confirmed the cohort is non-uniform (F51 / F53 / F57 don't need inline spans). Plain-text parser path: empty `inline` vec (no Markdown semantics). **Estimated effort:** half a day for the substrate, then [F49](#f49) ships on top in a follow-up PR. **Bench gate:** PR-1 against the existing bench corpus; > 5 % regression triggers a profile pass before merge. **Reversibility:** the layer can be deleted and folded back into per-rule fields if a year passes with one consumer; the engine swap (comrak) was rejected partly because that reversal is much harder. | 🔴 Next | 2026-05-02 parser substrate brainstorm; F49 cohort lead unblocked |
| <a id="f139"></a>[F139](#f139) | **Experimental rule status — registry substrate for the v0.3 cohort.** Routed 2026-05-02 (`.personal/brainstorm/20260502-v03-breaking-change.md`). Soft-breaking changes (new default-active rules) are the SemVer-major signal for linters; lucid-lint has 5 such rules queued for v0.3 ([F46](#f46) / [F49](#f49) / [F51](#f51) / [F53](#f53) / [F57](#f57)). Rather than smear 5 score regressions across v0.2.x patches *or* hold all 5 until a single v0.3 cut, this entry adds a **rule lifecycle status** (`Stable` / `Experimental`) and ships the cohort in v0.2.x as `Experimental` (off by default). Users — including this repo's own dogfood loop on adjacent projects — opt in via a `[experimental]` config section (`enabled = ["structure.italic-span-long", …]` or `enabled = "*"`) or `--experimental <id>` CLI flag. v0.3's breaking change is then a single-line per rule (`Status::Experimental` → `Status::Stable`) plus a CHANGELOG cohort entry. **Why this shape, not per-rule `default = false` knobs:** `Status` is one concept that maps to a known industry pattern (clippy `nursery`, biome `nursery`, ESLint experimental rules, rust `#[unstable]`); per-rule booleans would add five toggles for the same concept and pre-figure no lifecycle. **Minimal viable substrate (resist gold-plating):** `Status` enum on the `Rule` trait (default `Stable`); `default_rules()` filters `Experimental` unless config opts in; `[experimental]` TOML section parsing; `--experimental` CLI flag (multi-occur + `*`); experimental tagging visible in `--list-rules` output; one snapshot test for the experimental-off vs experimental-on diff. **No** rule-group / preset / category-toggle machinery yet — the biome-style `recommendedRules` preset is filed as a v0.4 question. Estimated effort: half a day for the substrate, then one line per rule once F49 / F51 / F53 / F57 ship on top of it. F46 keeps its original FR-corpus slip-flag (independent of the experimental status). | 🔴 Next | 2026-05-02 v0.3 breaking-change brainstorm; user-proposed dogfood window |
| <a id="f-roadmap-slug-ids"></a>[F-roadmap-slug-ids](#f-roadmap-slug-ids) | **ROADMAP feature IDs adopt `F-<kebab-slug>` form for all new entries.** Routed 2026-05-02 (`.personal/brainstorm/20260502-roadmap-id-attribution.md`). Numeric `F<n>` IDs collided when two branches independently picked the same free number; reservation-on-`main` was ruled out because new features are routinely *discovered* mid-implementation inside an existing feature branch. **Decision:** new ROADMAP entries use a slug-as-ID form (`F-inline-ast-substrate`); legacy F1–F146 stay numeric (no migration — Devil's-Advocate verified no programmatic parser in `src/`, `tests/`, `scripts/`, `.github/`, `justfile` depends on the format; mixed taxonomy is cosmetic). Slugs are coined locally with no coordination. The cross-branch race that survives (two offline branches independently coining the same slug) is detected at PR time and resolved by a one-line slug rename in ROADMAP + CHANGELOG — no branch rename, no rebase, no commit-history rewrite, because the new convention drops the `F-` prefix from branch names and commit subjects. **Minimal viable substrate:** (a) `tests/roadmap_id_uniqueness.rs` parses `ROADMAP.md` + `CHANGELOG.md` and asserts every `F-<slug>` appears uniquely as a definition site, no slug shadows the legacy `F<number>` namespace, and every referenced `[F-foo](#f-foo)` resolves; runs offline via `cargo test`, re-runs in CI as a backstop. (b) Explicit `<a id="f-..."></a>` anchors on first definition (matches the existing convention for numeric IDs). (c) `F-` prefix becomes optional in branch names and commit subjects — branches use plain feature slugs (`feat/<slug>`), commits use scope syntax (`feat(parser): <subject>`). **Surfaces touched:** `tests/roadmap_id_uniqueness.rs` (new), `AGENTS.md` Conventions section, `CHANGELOG.md` `[Unreleased]`, this entry itself (first dogfood). **Reversibility:** if the mixed taxonomy ever bites (it shouldn't — no parser depends on it), a one-shot rename script could fold slug entries into a numeric scheme at any future v0.x cut. **Estimated effort:** ~1 h total — uniqueness test (30 min), `AGENTS.md` update (15 min), this ROADMAP wiring (15 min). | 🔴 Next | 2026-05-02 ID-attribution brainstorm |

### Encoding / input handling

The linter is a UTF-8 → diagnostics function. Encoding conversion is
the user's responsibility, exactly once, before lint-time (`iconv`
or "save as UTF-8"). Invalid UTF-8 fails at the read boundary
(`std::fs::read_to_string` returns an `io::Error`). Other encodings
(Windows-1252, Latin-1, Shift-JIS, …) are explicit non-goals: any
in-process transcoder would violate the deterministic-core prime
directive (charset detection is heuristic, "same input, same output"
no longer holds). The entries below cover the *valid-UTF-8* edge
cases the test surface should pin.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f110"></a>[F110](#f110) | ✅ Shipped 2026-04-28 — leading `\u{FEFF}` stripped once at the engine boundary (`Engine::lint_with_source`, via the `normalize_input` helper). Funnels every input path (string, stdin, file) through the same boundary so rules never see the BOM. Regression test in `src/engine.rs::tests::bom_prefix_does_not_shift_diagnostics` proves identical diagnostics + line/column locations with and without a leading BOM on a sentence-too-long fixture. | — | 2026-04-25 encoding survey |
| <a id="f111"></a>[F111](#f111) | ✅ Shipped 2026-04-28 — `unicode-normalization = "0.1"` added; `Engine::lint_with_source` NFC-normalizes input at the same boundary as [F110](#f110), fast-pathing already-NFC text via `is_nfc_quick`. NFC `café` and NFD `cafe + U+0301` now hash identically in every HashMap-using rule. Regression test in `src/engine.rs::tests::nfd_input_yields_same_diagnostics_as_nfc` exercises a 4-sentence FR fixture and asserts diagnostic count + per-diagnostic rule id and line match across NFC and NFD inputs. | — | 2026-04-25 encoding survey |
| <a id="f112"></a>[F112](#f112) | ✅ Shipped 2026-04-28 — `src/engine.rs::tests::lone_cr_line_endings_are_normalized` pins parity between LF and lone-CR three-paragraph fixtures (word count + diagnostic count). `src/engine.rs::tests::zero_width_chars_inside_words_pin_behaviour` pins observed behaviour for U+200B / 200C / 200D inside words: the engine round-trips without panicking and produces a valid `Report`; exact word count is intentionally not asserted because `nfc()` does not strip them and tokenisation is owned by `unicode-segmentation`. | — | 2026-04-25 encoding survey |
| <a id="f113"></a>[F113](#f113) | **Mixed-script test fixtures.** Pin behaviour on EN + CJK and LTR + RTL prose mixed within one paragraph. `unicode_words()` should handle the boundaries correctly (UAX-29), but no regression test exists. Filed as Speculative — no known bug, just a coverage gap. Open if a real-world bilingual corpus surfaces edge cases. | 🟢 Speculative | 2026-04-25 encoding survey |
| <a id="f126"></a>[F126](#f126) | ✅ Shipped — Markdown parser maps `<br>` to `\n` in `paragraph.text`. Pulldown-cmark emits `<br>` as `Event::InlineHtml`, not `Event::HardBreak`, so the v0.2.x author-break-aware fix for `structure.line-length-wide` silently dropped `<br>` despite advertising it as a measured hard break. Helper `html_is_br_tag` recognises `<br>`, `<br/>`, `<br />` (any case, optional whitespace); HTML comments (suppression directives) flow through unchanged. Five new tests pin the contract: `br_tag_inside_paragraph_is_a_hard_break` and `html_comment_directives_do_not_inject_newlines` (parser); `markdown_br_tag_is_checked`, `list_item_text_is_out_of_scope`, `table_cell_text_is_out_of_scope` (rule). The two out-of-scope tests pin the parser-construction contract that list-item content and GFM table cells are not emitted as paragraphs today, so the rule is silent on over-length content inside them — a future parser change that starts emitting either as paragraphs would need to revisit this rule. | — | 2026-04-30 audit follow-up to the `structure.line-length-wide` author-break-aware fix (`.personal/2026-04-30-today.md:125`) |

### Rules refinement

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f9"></a>[F9](#f9) | ✅ Shipped in v0.2 — definition-aware `lexicon.unexplained-abbreviation` is now two-pass. A pre-scan collects acronyms defined anywhere in the document in either canonical form (`Expansion (ACRONYM)` or `ACRONYM (Expansion)`; expansion side ≥ 2 alphabetic words to reject `(TBD)`-shaped noise), and a single definition silences every occurrence of that token. Silencing precedence: defined-in-doc → user whitelist → baseline. See [`docs/src/rules/unexplained-abbreviation.md`](docs/src/rules/unexplained-abbreviation.md). | — | Rule 10 simplified in v0.1 |
| [F10](#f10) | 🚧 Must-ship slice shipped in v0.2 — `readability.score` auto-selects the formula by detected language: Flesch-Kincaid for EN (kept), Kandel & Moles (1958) for FR. Kandel-Moles ease scores are converted to a grade-equivalent so per-profile `max_grade_level` stays comparable across languages. Unknown language → Flesch-Kincaid. See [`docs/src/rules/readability-score.md`](docs/src/rules/readability-score.md). Still open: Gunning Fog / SMOG / Dale-Chall (EN), Scolarius / Flesch-Kandel (FR), `--readability-verbose` multi-formula reports, per-file override (covered by [F11](#f11)). | 🟡 Later | Rule 11 simplified in v0.1; scope expanded in rule-system-growth brainstorm (2026-04-20) |
| <a id="f11"></a>[F11](#f11) | ✅ Shipped in v0.2 — `--readability-formula {auto,flesch-kincaid,kandel-moles}` CLI flag + `FormulaChoice` enum on `readability_score::Config` + `Engine::with_readability_formula(choice)`. `auto` (default) keeps [F10](#f10) per-language selection; `flesch-kincaid` / `kandel-moles` pin a formula for cross-document comparison. TOML config wiring is tracked separately as [F77](#f77). | 🟡 Later | Rule 11 |
| <a id="f13"></a>[F13](#f13) | `missing-connectors` rule (15b not shipped in v0.1) | 🟡 Later | Rule 15 decomposition |
| <a id="f1"></a>[F1](#f1) | Custom stoplist parameter for `lexicon.low-lexical-diversity` | 🟡 Later | Rule 5 |
| <a id="f2"></a>[F2](#f2) | Sentence-level low-lexical-diversity density | 🟢 Speculative | Rule 5 |
| <a id="f3"></a>[F3](#f3) | Comma density metric (relative) for `structure.excessive-commas` | 🟢 Speculative | Rule 3a |
| [F22](#f22) | 🚧 First slice shipped in v0.2.x — `structure.excessive-commas` now discounts commas inside `(A, B, C, …)` parenthesised token lists (3+ short comma-separated segments inside balanced parens, language-agnostic). Sibling helper `parenthesised_list_comma_count` in `src/rules/enumeration.rs`. Dogfood drops from 25 → 15 hits (10 FPs killed, ~40% reduction). Deferred to v0.3: relaxing `MAX_SEGMENT_WORDS = 2` for 3–4-word Oxford items, non-Oxford / "plus"-closed lists, interleaved parentheticals inside Oxford runs. See research note in `.personal/research/[F22](#f22).md`. | 🔴 Next | v0.1 dogfood: 5 false-ish positives on technical docs |
| <a id="f23"></a>[F23](#f23) | ✅ Shipped in v0.2 — false-positive cleanup complete for v0.2. Hits inside inline code spans, straight `"..."` quotes, paired curly `"..."` quotes, and directional `rather than` / `plutôt que` pairings are now skipped. Single quotes / apostrophes are deliberately not recognised (possessives, contractions, FR elisions). The "concrete noun" semantic check (`"many X"` where X is a concrete noun) stays unshipped — needs POS data and belongs in the `lucid-lint-nlp` plugin ([F75](#f75)) rather than the deterministic core. | — | v0.1 dogfood: 11 false-ish positives on this repo's own docs |
| <a id="f24"></a>[F24](#f24) | Refine `lexicon.excessive-nominalization` suffix list (drop or gate `-al`; many adjectives — `crucial`, `horizontal`, `positional`, `attentional` — are flagged despite not being abstract nouns) | 🟡 Later | v0.1 dogfood |
| <a id="f87"></a>[F87](#f87) | ✅ Shipped in 0.2.x — FR `syntax.nested-negation` now uses pair-based counting over `ne` / `n'` clitics and the second-position particles `pas`, `rien`, `jamais`, `plus`, `personne`, `aucun`, `aucune`, `guère`, `nulle part`. Each clitic contributes one negation and consumes its nearest particle within a 6-token window; unpaired particles in a `ne`-sentence contribute one more — so `Nous ne disons pas que rien n'est jamais possible` now counts as 3 (was 2). Guards: `pas` / `plus` never count when unpaired, `de rien` idiom is skipped, particles in ne-less sentences are skipped. Fixture at `tests/corpus/fr/nested-negation.md` anchors the behaviour. | — | 2026-04-23 docs clarity session — FR pedagogical example surfaced the detection gap |
| <a id="f31"></a>[F31](#f31) | ✅ Shipped in v0.2 — `dev-doc` baseline narrowed to the infrastructure stack (`URL`, `HTML`, `CSS`, `JSON`, `XML`, `HTTP`, `HTTPS`, `UTF`, `IO`, `API`, `CLI`, `GUI`, `OS`, `CPU`, `RAM`, `SSD`, `USB`, `IDE`, `SDK`, `CI`, `CD`). Accessibility standards, engineering-practice initialisms, and AI/language-tech terms moved to project config via new `[rules.unexplained-abbreviation].whitelist` in `lucid-lint.toml` (additive over baseline). Breaking change for downstream users, flagged in CHANGELOG with the recovery snippet. Dogfooded in this repo's own [`lucid-lint.toml`](lucid-lint.toml). | — | v0.1 review feedback |
| [F126](#f126) | TOML overrides for `lexicon.jargon-undefined`. In v0.2 the active jargon lists are baked into the profile preset and there is no `[rules."lexicon.jargon-undefined"]` deserializer in `src/config.rs` — users can't add custom domain terms, silence individual entries, or activate a non-default list combination from `lucid-lint.toml`. Wire the same shape `unexplained-abbreviation` already uses (validated `whitelist`, plus `custom_jargon` for additive terms and an explicit `active_lists` enum array). The rule's underlying `Config` struct already exposes the fields (`active_lists`, `custom`, `whitelist`) — this is a config-layer wiring task, not a rule rewrite. Definition of done: TOML round-trip test, docs page (`docs/src/rules/jargon-undefined.md` + FR mirror) describing the schema, drop the [F126](#f126) forward-link in those pages. | 🟡 Later | 2026-04-28 FR-translation review surfaced the gap |
| <a id="f144"></a>[F144](#f144) | **Severity tiering for `lexicon.weasel-words`.** Routed 2026-05-02 (`.personal/brainstorm/20260502-async-book-pr-timing.md`); blocks the async-book audit-and-PR play (tracked in `.personal/promotion-channels.md`). The current rule fires uniform `warning` on every entry in the EN/FR weasel-list, conflating two distinct linguistic functions: **quantifiers** (`some`, `many`, `often`, `most`, `several`) which are legitimate technical hedging in reference docs, and **hedges** (`a bit`, `just`, `quite`, `rather`, `pretty`, `kind of`) which signal under-confident prose. Stripping all of them in a Rust async-reference produces prose reviewers reject as artisan ("over-edited" — surfaced in `.personal/f113-async-book/READABILITY_REVIEW.md`). Fix: split `WEASEL_WORDS_EN` / `_FR` into two sub-lists, emit `Severity::Info` on quantifier hits and `Severity::Warning` on hedge hits. Per-rule TOML override stays available for users who want stricter / looser bands. Surfaces the pattern other lexical rules can adopt later (no architectural lift; a per-match severity decision inside the rule body). Definition of done: split lists in `src/language/{en,fr}/weasel.rs`, severity routing in `src/rules/lexicon/weasel_words.rs`, snapshot regen for both languages, docs page (`docs/src/rules/weasel-words.md` + FR mirror) describing the two bands and the rationale, CHANGELOG `## [Unreleased]` entry. Pairs with [F146](#f146): once `--severity-floor=warning` exists, an external auditor running on a Rust-reference repo gets the "no contested edits" view in one flag. | 🔴 Next | F113 audit-and-PR play (2026-05-02) |
| <a id="f145"></a>[F145](#f145) | **`lexicon.redundant-intensifier` parser miss inside bullet items / `**strong**` spans.** Routed 2026-05-02 (`.personal/brainstorm/20260502-async-book-pr-timing.md`); blocks the async-book audit-and-PR play (tracked in `.personal/promotion-channels.md`). Surfaced while linting `rust-lang/async-book/src/why_async.md`: `very` inside `- **OS threads** are very …` (bullet + strong span) does not fire, while `highly` in a flat paragraph does. Same family of misses as [F129](#f129) — paragraph-level rules go silent when the surrounding event is `Tag::Item` with no enclosing `Tag::Paragraph`. [F129](#f129) is the right substrate to fix once for every paragraph-level rule; F145 is the verification slice that pins the regression for `redundant-intensifier` so the case cannot regress when [F129](#f129) lands. Definition of done: corpus fixture `tests/corpus/en/redundant-intensifier-bullet.md` + FR mirror, snapshot covering `very` / `highly` / `really` inside `- **strong** ...` and `* **strong** ...` shapes, comment in the test linking to [F129](#f129) so the slot stays after F129 lands, CHANGELOG entry. | 🔴 Next | F113 audit-and-PR play (2026-05-02); same family as [F129](#f129) |

<!-- lucid-lint disable-next-line weasel-words -->

**[F22](#f22) context.** The v0.1 rule is a flat comma-per-sentence threshold.
In technical docs that routinely enumerate short items, this fires
often even when the sentence is perfectly scannable. Candidate
relaxations to evaluate (needs corpus research — don't pick blindly):

- **Discount commas inside parenthesis-like elements** (`(...)`,
  `[...]`, en/em-dash pairs). A parenthetical enumeration is already
  visually bracketed; its commas are not adding subordination load.
- **Discount commas after a colon `:`** when what follows is a list of
  short items. Colon + short items is idiomatic prose-enumeration and
  reads well.
- **Short-item enumeration exemption**: if all comma-separated
  segments are 1–2 words, treat the enumeration as a single
  "flattened list" token for counting purposes (a
  `max_short_enum_items` parameter, or implicit).
- **Interaction with `structure.long-enumeration`**: the shared
  `enumeration::detect_enumerations` helper already discounts Oxford-
  style enumeration commas from `structure.excessive-commas` (3+ short items).
  [F22](#f22) is specifically about the cases that helper still misses:
  parentheticals, post-colon lists, and non-Oxford enumerations
  ("A, B, C and D" without the final comma).

Research inputs to gather before deciding: FR/EN corpus samples of
technical docs, a handful of real false positives from dogfooding and
downstream projects, how `textlint` / Vale / `write-good` handle
parentheticals. Decide between relaxation parameters vs. a smarter
token-aware counter.

### Performance / hygiene (0.2.x)

Findings filed from the 2026-04-24 code-review stream-2 pass on
`src/`. Each has a concrete source reference so it survives past the
`.personal/<date>-today.md` scratchpad.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f93"></a>[F93](#f93) | **Parser hot-path allocations.** `src/parser/mod.rs:43` (`Paragraph::new(trimmed.to_string(), …)`) and `src/parser/tokenizer.rs:~88/109` (`current.trim().to_string()` per sentence) allocate in hot loops. ~~Confirm constructors accept `impl Into<String>`; pass the already-owned buffer where possible.~~ **Refuted by samply profile 2026-04-25**: `Paragraph::new` does not appear in the profile; `to_string()` in tokenizer = 3 samples / 0.03%. Real hot spots are [F102](#f102) (`detect_language` 7.5%) and [F103](#f103) (per-rule `split_sentences`). | ✅ Done (refuted) | 2026-04-24 code review (stream-2 #3); refuted 2026-04-25 |
| <a id="f94"></a>[F94](#f94) | **Tokenizer `Vec<char>` per sentence.** `src/parser/tokenizer.rs:~60` collects a full `Vec<char>` for lookahead. ~~Swap to `Peekable<CharIndices>`.~~ **Refuted by samply profile 2026-04-25**: `Vec<char>` drop = 3 samples / 0.03% on the engine path. Yesterday's "low ceiling" note (~5%) was generous; real ceiling is ~0.1%. Skip. | ✅ Done (refuted) | 2026-04-24 code review (stream-2 #5); refuted 2026-04-25 |
| <a id="f102"></a>[F102](#f102) | **`detect_language` cost.** Single function showed 7.5% inclusive in samply profile 2026-04-25. Rewrote as single-pass, alloc-light: scalar counters, `to_lowercase()` only for words containing an uppercase character, no intermediate vectors. Bench delta on `engine_lint_str/en_long_devdoc` vs `stream2-noisy`: **−0.56 % (p = 0.00, ~20 µs)** — smaller than profile suggested because most of the inclusive cost is `unicode_words()` itself, which the rewrite cannot touch. | ✅ Done | 2026-04-25 samply profile; landed 2026-04-25 |
| <a id="f103"></a>[F103](#f103) | **Per-rule `split_sentences` re-parse.** 8 rules called `split_sentences(&paragraph.text, …)` directly. Moved sentence splitting into `Paragraph::new`; rules now read `&paragraph.sentences`. Bench delta vs `stream2-noisy`: **`engine_lint_str/en_long_devdoc` −11.58 % (~394 µs)**; `parse_markdown/en_long` +17.67 % (~38 µs, intentional — split cost moved into the parser phase, where it pays for itself across the eight consumers). Net user-facing win ~360 µs. New baseline saved as `stream2-after-f103`. | ✅ Done | 2026-04-25 samply profile; landed 2026-04-25 |
| <a id="f95"></a>[F95](#f95) | ✅ Shipped 2026-04-24 in commit `925ffb5`. Two non-literal expects fixed: `consecutive_long_sentences.rs` (`streak_start` unwrap when `streak_len > max`) and `all_caps_shouting.rs::flush_run` (`first()`/`last()` on a `Vec` already verified `len >= min_run`). The originally flagged `parser/tokenizer.rs:177` candidate is now an `if let Some(...)` pattern. Remaining `expect("non-zero literal")` sites are all `NonZeroU32::new(LITERAL)` — idiomatic compile-time invariants, explicitly out of audit scope. | ✅ Done | 2026-04-24 code review (stream-2 #2) |
| <a id="f96"></a>[F96](#f96) | ✅ Shipped 2026-04-24 in commit `925ffb5`. `src/scoring.rs:199-209` now carries an explicit safety-contract comment naming the `[0, cap]` clamp dependency, plus a `debug_assert!(normalized.is_finite() && (0.0..=cap).contains(&normalized))` that trips in debug builds if a future edit loosens the clamp. The `#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]` stays — it masks a lint, not a real bug — but the invariant is now load-bearing in tests. | ✅ Done | 2026-04-24 code review (stream-2 #1) |
| <a id="f97"></a>[F97](#f97) | **Config whitelist normalization at load time.** `src/config.rs` — normalize (trim, case-fold per rule needs) on load instead of per invocation; catches user typos early. Small win; fits a v0.3 config-plumbing pass rather than a 0.2.x patch. | 🟡 Later | 2026-04-24 code review (stream-2 #6) |

### New rules (v0.2)

New rule candidates raised in the rule-system-growth brainstorm
(2026-04-20). Naming uses a provisional `category.rule-name` prefix
pending [F29](#f29) harmonisation. Grounding column points at the standard or
research that justifies the rule.

**Must-ship v0.2 (blocking release):**

| ID | Rule | Category | Tags | Grounding | Priority |
|---|---|---|---|---|---|
| <a id="f48"></a>[F48](#f48) | ✅ `lexicon.all-caps-shouting` shipped in v0.2 — see [`docs/src/rules/all-caps-shouting.md`](docs/src/rules/all-caps-shouting.md) | Lexicon | `a11y-markup`, `dyslexia`, `general` | WCAG 3.1.5, BDA Dyslexia Style Guide | — |
| <a id="f55"></a>[F55](#f55) | ✅ `syntax.nested-negation` shipped in v0.2 — see [`docs/src/rules/nested-negation.md`](docs/src/rules/nested-negation.md) | Syntax | `aphasia`, `adhd`, `general` | FALC, CDC Clear Communication Index | — |
| <a id="f56"></a>[F56](#f56) | ✅ `syntax.conditional-stacking` shipped in v0.2 — see [`docs/src/rules/conditional-stacking.md`](docs/src/rules/conditional-stacking.md) | Syntax | `aphasia`, `adhd`, `general` | FALC, plainlanguage.gov | — |

**Should-ship v0.2 (cuttable under time pressure, in suggested cut order):**

| ID | Rule | Category | Tags | Grounding | Priority |
|---|---|---|---|---|---|
| <a id="f62"></a>[F62](#f62) | ✅ `lexicon.redundant-intensifier` shipped in v0.2 — see [`docs/src/rules/redundant-intensifier.md`](docs/src/rules/redundant-intensifier.md) | Lexicon | `general` | Plain-language guides | 🟡 Later |
| <a id="f52"></a>[F52](#f52) | ✅ `structure.mixed-numeric-format` shipped in v0.2 — see [`docs/src/rules/mixed-numeric-format.md`](docs/src/rules/mixed-numeric-format.md) | Structure | `dyscalculia`, `general` | CDC Clear Communication Index | 🟡 Later |
| <a id="f50"></a>[F50](#f50) | ✅ `structure.line-length-wide` shipped in v0.2 — see [`docs/src/rules/line-length-wide.md`](docs/src/rules/line-length-wide.md) | Structure | `dyslexia`, `general` | WCAG 1.4.8 (AAA) | 🟡 Later |
| <a id="f47"></a>[F47](#f47) | ✅ `lexicon.consonant-cluster` shipped in v0.2 — see [`docs/src/rules/consonant-cluster.md`](docs/src/rules/consonant-cluster.md) | Lexicon | `dyslexia`, `general` | BDA Dyslexia Style Guide | 🟡 Later |
| <a id="f54"></a>[F54](#f54) | ✅ `syntax.dense-punctuation-burst` shipped in v0.2 — see [`docs/src/rules/dense-punctuation-burst.md`](docs/src/rules/dense-punctuation-burst.md) | Syntax | `general` | IFLA easy-to-read guidelines | 🟡 Later |

**Cut order if schedule slips:** [F47](#f47) → [F54](#f54) → [F62](#f62) → [F52](#f52) → [F50](#f50) → [F11](#f11). [F55](#f55)
and [F56](#f56) are non-negotiable (trivial implementation cost, strong
grounding).

### Format support

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f5"></a>[F5](#f5) | Native AsciiDoc support | 🟡 Later | Format scope v0.1 |
| <a id="f6"></a>[F6](#f6) | Native HTML support | 🟡 Later | Relevant for EAA compliance |
| <a id="f7"></a>[F7](#f7) | `.docx` support via Pandoc integration | 🟡 Later | FALC institutional target |
| <a id="f8"></a>[F8](#f8) | Companion script `pandoc → lucid-lint` | 🟡 Later | Documented in v0.1 README |

### Example-text fixtures

Scraper + cleaner + converter triplet under `scripts/texts_*.py`
populates `examples/public/` (committable `public_ok` sources) from
`examples/texts.yaml`. First batch landed 21 fixtures. The follow-ups
below close the remaining rough edges.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f81"></a>[F81](#f81) | Per-source adapters for git-cloned upstreams. The generic `clean` / `convert` path doesn't know how to extract text from shallow-cloned repos (proselint checks, Vale style packs, write-good / alex / retext / textlint-rule fixtures, ASSET / OneStopEnglish / EASSE / CLEAR-corpus datasets). Each needs a small extractor that walks the repo and emits one or more `.md` files per rule / excerpt. | 🟡 Later | First scraper batch, 2026-04-22 |
| <a id="f82"></a>[F82](#f82) | Refine `texts_convert._split_before_after`. The current heuristic looks for literal `## Before` / `## After` (EN/FR) headings; no upstream page in the current batch uses that shape, so every `before_after` source fell back to a single `content.md` with a warning. Replace with a per-source pair-extraction rule (plainlanguage.gov, EC *How to write clearly*, Canada.ca, OneStopEnglish, ASSET, Inclusion Europe) that emits `before.md` + `after.md`. | 🟡 Later | First scraper batch, 2026-04-22 |
| <a id="f83"></a>[F83](#f83) | Maintenance pass on `examples/texts.yaml` URLs. 12 sources failed on the first batch — 404s from moved landing pages (canada.ca × 2, BDA Dyslexia, Center for Plain Language, Newsela, HuggingFace wiki_auto), UA-/bot-blocking (Légifrance 403, Orthodidacte 403, ADHD Foundation 400), and a DNS error for the specific 18F post. Audit and update entries; for sources that genuinely require a browser-flavoured UA, add a per-source override in the fetcher. Fold in the opportunistic hygiene tasks from the 2026-04-23 brainstorm: (a) dedupe overlapping canada.ca / plainlanguage.gov entries, (b) add a licence-drift guard that flags when a source's `redistribution` changes between fetches. | 🟡 Later | First scraper batch, 2026-04-22 + referential brainstorm, 2026-04-23 |
| [F84](#f84) | Desired-fixture-shapes coverage table + replacements for high-value local-only entries. **Part 1 — coverage tables:** ✅ Shipped (2026-04-23) — `scripts/texts_coverage.py` splits output by audience: the committed `examples/texts.md` shows `public_ok` counts only (no totals, no names that would leak local-only existence), spliced between `<!-- coverage:begin/end -->` markers; the gitignored `examples/local/COVERAGE.md` carries the full matrices plus the load-bearing local-only list. Wired as `just texts-coverage` / `just texts-coverage-check`. **Part 2 — replacement hunting:** 🟡 In progress. First addition (2026-04-25): a French government FALC source under Etalab Open Licence 2.0 — knock-on lifted `aphasia × FR` and `gov_guide × FR` out of `0 / N ⚠`. Second addition (2026-04-27): three US-federal public-domain ADHD sources — NIMH ADHD topic page (mixed shape, ~780 words), CDC About ADHD (good, ~920 words), CDC Treatment of ADHD (good, ~1040 words). All three covered by the explicit reproduction notices in NIMH and CDC reuse policies (17 USC § 105 + agency policy pages). Knock-on: `adhd × EN` lifted from the load-bearing list; public-coverage `gov_guide × EN` and `condition adhd × EN` rise to non-zero counts. Remaining load-bearing slots: `dyscalculia × EN` (one BDA `link_only`) and `aphasia × EN+FR` (three plain-language standards as `link_only`). | 🟡 In progress | Referential brainstorm, 2026-04-23 |
| <a id="f85"></a>[F85](#f85) | Bidirectional rule ↔ fixture coverage map. Generate `examples/COVERAGE.md` from each `content.md`'s `rules_relevant` frontmatter, rendered as two views: rule → fixtures that exercise it (surfaces under-fixtured rules) and fixture → rules it covers (surfaces untagged or mis-tagged fixtures). Once stable, embed or link the canonical fixture per rule from `docs/src/rules/<rule-id>.md`. Optional follow-up: calibrated snapshot tests that lock expected lint output per canonical fixture. | 🟡 Later | Referential brainstorm, 2026-04-23 |
| <a id="f86"></a>[F86](#f86) | Auto-discovery of new references with triage queue. Crawler (sitemaps, RSS, GitHub search, ACL Anthology API) surfaces candidate sources against a relevance filter derived from `rules_relevant` keywords; a lightweight triage file lists candidates with accept / ignore / defer. Mini-product — revisit post-v0.3 once the referential has stabilised. | 🟢 Speculative | Referential brainstorm, 2026-04-23 |

### Documentation rules plugin

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f4"></a>[F4](#f4) | `code-block-without-lang` rule | 🟡 Later | Rule 8 dropped from v0.1, candidate for `lucid-lint-docs` plugin |

### Docs.rs / API reference polish

Polish items for the auto-generated rustdoc surface at <https://docs.rs/lucid-lint>. The crate-level banner pointing readers to the mdBook + repo + RULES.md was added 2026-05-01 (`src/lib.rs`); module-level `//!` headers are already in place and `#![warn(missing_docs)]` is satisfied. Items below are deferred extras.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f133"></a>[F133](#f133) | `[package.metadata.docs.rs]` block in `Cargo.toml`. Pin the toolchain and feature set docs.rs builds with; add `rustdoc-args = ["--cfg", "docsrs"]` so any future feature-gated items can carry `#[cfg_attr(docsrs, doc(cfg(feature = "x")))]` and render the "available with feature X" badge. Cheap, lands the day a real feature flag is introduced. Renumbered from [F129](#f129) (collision with the parser tight-list fix that landed in parallel). | 🟢 Speculative (0.2.x or 0.3) | 2026-05-01 docs.rs polish discussion |
| <a id="f130"></a><a id="f134"></a>[F134](#f134) | Logo + favicon on docs.rs via `#![doc(html_logo_url = "…")]` and `#![doc(html_favicon_url = "…")]` at crate root. Reuses an asset hosted under the repo's raw URL. Tiny visual identity win on the docs.rs landing page. Renumbered from [F130](#f130). | 🟢 Speculative (0.2.x) | 2026-05-01 docs.rs polish discussion |
| <a id="f131"></a><a id="f135"></a>[F135](#f135) | One runnable doctest per major entry point (`Engine::with_profile`, `Engine::lint_str`, `Report` field access, key `Profile` variants). `///` blocks render as code samples on docs.rs *and* run under `cargo test --doc`, so they cannot rot. ~5 lines each. Lifts the API page from "list of names" to self-explanatory reference. Renumbered from [F131](#f131). | 🟡 Later (0.3) | 2026-05-01 docs.rs polish discussion |
| <a id="f132"></a><a id="f136"></a>[F136](#f136) | Public-API audit with `cargo public-api`: surface candidates that should carry `#[doc(hidden)]` (re-exports for macros, internal helpers leaked via `pub`) so the rustdoc index reflects the *intended* surface, not the *current* surface. Pair with a CI gate later if the surface becomes load-bearing for SemVer. Renumbered from [F132](#f132). | 🟡 Later (0.3) | 2026-05-01 docs.rs polish discussion |

### Docs site — bilingual

| ID | Item | Priority | Origin |
|---|---|---|---|
| [F25](#f25) | French mirror of the mdBook docs (`/fr/` tree). First slice shipped 2026-04-22: translated `introduction` + `rules-index`, short FR `accessibility` and `roadmap` pages pointing at EN, SUMMARY sidebar entry. Second slice shipped post-0.2.1 (2026-04-23): `fr/rules-index.md` renamed to `fr/rules/index.md` for EN-parity, first FR per-rule page landed (`structure.sentence-too-long`), parallel-version sidebar and EN↔FR deep-link toggle ([F90](#f90) plan slot A, [F92](#f92)). Third slice shipped 2026-04-24: four more FR per-rule pages landed (`structure.excessive-commas`, `structure.long-enumeration`, `lexicon.weasel-words`, `lexicon.unexplained-abbreviation`), locked template honoured, `SUMMARY.md` + `fr/rules/index.md` rewired to point at the local FR versions. Fourth slice shipped 2026-04-25: six more FR per-rule pages landed (`structure.paragraph-too-long`, `structure.line-length-wide`, `structure.mixed-numeric-format`, `structure.deeply-nested-lists`, `structure.heading-jump`, `structure.deep-subordination`), closing out the `structure` category (9 / 9 rules FR-complete). Fifth slice shipped 2026-04-27: two more FR per-rule pages landed (`rhythm.consecutive-long-sentences`, `rhythm.repetitive-connectors`), closing out the `rhythm` category (2 / 2 rules FR-complete). Both EN pages were brought up to canonical template first (Examples + See also added). Sixth slice shipped 2026-04-28: six more FR per-rule pages landed (`lexicon.low-lexical-diversity`, `lexicon.excessive-nominalization`, `lexicon.jargon-undefined`, `lexicon.all-caps-shouting`, `lexicon.redundant-intensifier`, `lexicon.consonant-cluster`), closing out the `lexicon` category (8 / 8 rules FR-complete). Three of five categories now at 100 % (structure + rhythm + lexicon). Seventh slice shipped 2026-04-30: six more FR per-rule pages landed (`syntax.passive-voice`, `syntax.unclear-antecedent`, `syntax.dense-punctuation-burst`, `syntax.conditional-stacking`, `syntax.nested-negation`, `readability.score`), closing out the `syntax` (5 / 5) and `readability` (1 / 1) categories — all 5 categories now 100 % FR-complete (25 / 25 per-rule pages). `SUMMARY.md` was missing FR Syntaxe + Lisibilité subsections entirely; added in the same commit. Also fixed an EN/FR logic bug in `syntax.nested-negation` example (After clause now `something is possible` / `quelque chose est possible`, matching the predicate-logic-faithful inversion of the Before clause). Eighth slice shipped 2026-05-01 (Block C slice A): first two FR guide pages landed (`fr/guide/installation.md`, `fr/guide/quick-start.md`); new `Premiers pas` draft-chapter group in `SUMMARY.md`; both pages stamped with the [F92](#f92) sub-task `en-source-sha` HTML comment. Ninth slice shipped 2026-05-01 (Block C slice B): two more FR guide pages landed (`fr/guide/profiles.md`, `fr/guide/suppression.md`) — Block C now half-done (4 / 8). 4 EN-only guide pages remain (`conditions`, `configuration`, `scoring`, `ci-integration`); FR pair-completeness now 35 / 42 (untranslated EN: 7, down from 11 at start of day). Tenth slice shipped 2026-05-01 (Block C slice C — closing slice): four FR guide pages landed (`fr/guide/conditions.md`, `fr/guide/configuration.md`, `fr/guide/scoring.md`, `fr/guide/ci-integration.md`); `SUMMARY.md` `Premiers pas` group now lists all 8 children. **Block C complete (8 / 8).** All 8 EN guide pages now have FR mirrors; FR pair-completeness 39 / 42 — only the architecture overview, design-decisions, and contributing pages remain untranslated (these are next-tier surfaces, not part of the user-facing guide). Eleventh slice shipped 2026-05-01 (next-tier close): three FR pages landed (`fr/architecture/overview.md`, `fr/architecture/design-decisions.md`, `fr/contributing.md`); `SUMMARY.md` gains an `Architecture` draft-chapter group + `Contribuer` entry under `Version française`. **[F25](#f25) closes** — pair-completeness 41 / 41 (only `roadmap.md` remains intentionally asymmetric). | ✅ Closed 2026-05-01 | v0.1 docs `/shape` session, bilingual-equality prime directive |
| <a id="f90"></a>[F90](#f90) | Split `SUMMARY.md` per locale (EN + FR) via a small preprocessor. v0.2.1 ships the single-`SUMMARY.md` + CSS `:has()` locale-hiding approach (1.A); both language trees coexist in the built HTML and each viewer only sees theirs. A clean separation would maintain `SUMMARY.en.md` + `SUMMARY.fr.md` and stitch them at build. Benefit: smaller per-page sidebar payload; clearer authoring story; no `:has()` browser-support floor. Cost: build-time stitcher, tooling to keep the two files in pair-sync. File when the FR tree outgrows the hide-via-CSS approach. | 🟢 Speculative | 2026-04-23 FR per-rule pages session |
| <a id="f91"></a>[F91](#f91) | Multi-book mdBook layout (one book per locale). The truest "parallel version" — `/` redirects to `/en/`, `/fr/` is its own mdBook with its own theme inheritance. Benefit: each locale has its own table of contents, its own search index, its own navigation neighbour hints; no cross-locale bleed in any surface. Cost: biggest surgery — book.toml per locale, build orchestration, shared theme / asset de-duplication, sitemap updates, redirects. Revisit only if [F90](#f90) isn't enough. | 🟢 Speculative | 2026-04-23 FR per-rule pages session |
| <a id="f92"></a>[F92](#f92) | ✅ Shipped post-0.2.1 (2026-04-23) — `scripts/sync_lang_counterparts.py` walks `docs/book/**/*.html` after `mdbook build` and rewrites both `hreflang="en"` and `hreflang="fr"` anchors so the lang-switch deep-links to the matching page (e.g. `/fr/rules/sentence-too-long.html` ↔ `/rules/sentence-too-long.html`). Wired into `just docs-build`, the Deploy-docs workflow, and a new `just docs-lang-check` CI gate that runs with `--check` and fails on orphaned FR pages (FR without EN counterpart). The invariant is asymmetric by design: EN is canonical, FR is a translation layer — untranslated EN pages are informational and tracked as [F25](#f25), not gated. No front-matter flag yet; add a `counterpart: none` flag only when a truly asymmetric page appears. **Sub-task — FR content-staleness gate (shipped 2026-05-01):** filename parity is gated; *content* drift was not. Every FR page now carries an `en-source-sha` HTML-comment stamp on its first line (`<!-- en-source-sha: 5e24f614… -->`), recording the EN counterpart's last commit SHA at translation time. mdBook passes HTML comments through unchanged so the stamp is invisible in the rendered page; YAML front-matter was tried first but mdBook renders `---` as `<hr>` and the body as text. `scripts/check_lang_staleness.py` walks every FR page, compares the stored SHA to `git log -n1 --pretty=%H -- <EN counterpart>`, reports drift soft (PR `ci.yml` + main `docs-deploy.yml`) and fails on `main` with `STRICT=1` once the existing stale backlog clears. Wired as `just docs-lang-staleness`. `scripts/backfill_en_source_sha.py` (one-shot) stamped the 29 already-translated FR pages with the EN SHA at their introduction commit. **Reconcile shipped 2026-05-01** (commit `438fa48b`, "F92 — reconcile stale FR backlog (13 → 0) + flip gate to strict"): of the 13 pages reported stale, 12 were cosmetic stamp drift only (the [F105](#f105)/[F105b](#f105b) references-section sweep, the [F35b](#f35b)/[F35c](#f35c) a11y fix, and the `line-length-wide` author-break-aware fix all touched FR counterparts in the same commits — only the `en-source-sha` stamps lagged), 1 was substantive (`fr/index.md` had drifted on three sections — `État du projet` v0.2 numbers, `Aperçu` peak-end demo block, `Pour aller plus loin` guide-links update). Same commit flipped `docs-deploy.yml` from soft to `--strict`. **PR-side `ci.yml` flipped to `--strict` on 2026-05-02** — both surfaces now strict-gated, sub-task fully closed. Optional further layers: an mdBook preprocessor banner above stale FR pages; a `needs-fr-translation` PR label automation for EN edits without FR counterparts. | — (sub-task: ✅ Closed 2026-05-02) | 2026-04-23 FR per-rule pages session, option 2.B; 2026-05-01 Block C planning |
| <a id="f128"></a>[F128](#f128) | **Docs i18n substrate evaluation (Starlight vs Sphinx).** mdBook is a twin-tree at the file level with a post-build `hreflang` patcher ([F92](#f92)); it cannot deliver page-keyed translations or identical section numbering across languages by construction. A real i18n model needs either route-keyed translations (Astro Starlight: `defaultLocale` + `locales`, language dropdown built in, Markdown sources kept) or message-catalogue translations (Sphinx + `sphinx-intl` / gettext PO files: FR is the same file with strings substituted, headings and numbering identical by construction; weblate-style flow). **Don't migrate now** — [F92](#f92) + [F25](#f25) + the [F92](#f92) staleness sub-task carry through v0.2.x. Migration triggers (any one): (a) a third language is requested (Spanish or German via the EU disability-federation play, [F115](#f115)); (b) docs surface crosses ~50 pages; (c) contributors complain about FR/EN drift after the staleness gate is in place. Default pick on trigger: **Starlight** (lightest migration, keeps Markdown). Sphinx only if RGAA-mandated structural parity becomes a contractual requirement. Placeholder entry — no work scheduled. | 🟡 Later | 2026-05-01 Block C planning, [F25](#f25) follow-up |
| <a id="f107"></a>[F107](#f107) | ✅ Shipped 2026-04-27 — Two-part fix without aliasing the rule ID. (1) Page subtitle: every shipped FR rule page opens with a short italic gloss directly under the H1 (e.g. `*Phrase trop longue.*`); 13 pages received the subtitle, the remaining 12 land alongside their translation. (2) Index gloss: `fr/rules/index.md` "Catégories" block reshaped into 5 per-category sub-tables (Structure / Rythme / Lexique / Syntaxe / Lisibilité), each `Règle \| Libellé` two-column. All 25 rules carry a FR label even when the page still points to the EN version (marked `(en)` inline). One-line note clarifies the `kebab-case` ID is the stable contract; the FR label is a reading aid. Sidebar TOC labels stay in EN — translating them would force a per-locale `SUMMARY.md` ([F90](#f90), parked Speculative). | — | 2026-04-25 docs UX critique (Block E) |

### Docs site — content

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f27"></a>[F27](#f27) | ✅ Shipped in v0.2 — `docs/src/roadmap.md` is auto-generated from the root `ROADMAP.md` by [`scripts/sync-roadmap.py`](scripts/sync-roadmap.py). `just docs-build` / `just docs-serve` run the sync first, so the mdBook site always ships the current roadmap. Relative links are rewritten (targets under `docs/src/` become docs-relative; others become absolute GitHub URLs) so the `docs_links_stay_inside_docs` gate still passes. | — | v0.1 docs review |
| <a id="f28"></a>[F28](#f28) | ✅ Shipped in v0.2 — one page per rule under `docs/src/rules/`, wired into `docs/src/SUMMARY.md`, enforced by [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs). Each page carries category, severity, default weight, parameters per profile, EN/FR examples where applicable, and suppression guidance. | — | v0.1 docs review |
| <a id="f29"></a>[F29](#f29) | Rule ID harmonisation. **[F29](#f29)-slim** ✅ shipped 2026-04-22 in v0.2.0: the 25 rule IDs now use `category.rule-name` form (`structure.excessive-commas`, `lexicon.weasel-words`, `readability.score`, …) and rule source files moved into category subdirectories under `src/rules/<cat>/`. `Category::for_rule` derives the category from the id prefix rather than a hand-maintained match arm ([F43](#f43)-style drift now impossible by construction). Hard break — suppression directives, `[rules.<id>]` TOML keys, JSON/SARIF `ruleId` fields all use the new form; no alias layer. mdBook filenames and docs URLs still use the flat kebab slug; docs-tree rearchitecture into category subdirs is a separate slice. **[F29](#f29)-full** (parked 2026-04-24) would add a stable category-numbered code (`STR-001`, `LEX-002`, `SYN-003`) that survives renames — slim already makes drift impossible by construction, and numeric codes only earn their cost on a real rename. Revisit only when a rename actually happens. | — (slim) / 🟢 Speculative (full) | v0.1 docs review; 2026-04-22 reprioritisation; 2026-04-24 brainstorm-next-cycles |
| [F30](#f30) | Audit every rule mention across the docs and link it to its reference page ([F28](#f28)). Requires [F28](#f28) to land first. References-page surface (rule IDs in `→ Relevant to:` lines + rule → reference summary table) covered by [F105b](#f105b) 2026-04-27; remaining surface is rule mentions in `docs/src/guide/*` prose pages, `RULES.md`, and the introduction. | 🟡 Later | v0.1 docs review |
| <a id="f42"></a>[F42](#f42) | ✅ Shipped in v0.2 — rule documentation coverage gate. [`tests/rule_docs_coverage.rs`](tests/rule_docs_coverage.rs) cross-checks every shipped rule id against its mdBook page, `Category::for_rule`, `scoring::WEIGHTED_RULE_IDS`, and (on CI, gated by `RULE_DOCS_GATE_GIT=1`) the `## [Unreleased]` section of `CHANGELOG.md`. Contract documented in [`CONTRIBUTING.md`](CONTRIBUTING.md#adding-or-modifying-a-rule--documentation-contract). | — | v0.2 interlude |
| <a id="f43"></a>[F43](#f43) | ✅ Shipped in v0.2 — `RULES.md` category drift fixed. Per-rule `**Category**` lines and the Categories table now match `Category::for_rule`: `structure.excessive-commas` and `structure.deep-subordination` are `structure`, `rhythm.repetitive-connectors` is `rhythm`, `syntax.unclear-antecedent` is `syntax`. The drift banners on the four per-rule mdBook pages are removed. | 🟡 Later | Surfaced by [F42](#f42) interlude |
| <a id="f44"></a>[F44](#f44) | Coverage test for [F30](#f30) rule-mention linking — assert each rule id mentioned in `docs/src/**/*.md` is linked on first-per-section occurrence. Follow-up from [F30](#f30). | 🟡 Later | [F30](#f30) follow-up |
| <a id="f104"></a>[F104](#f104) | ✅ Shipped 2026-04-27 — `SUMMARY.md` reshaped into 5 collapsible sub-trees (Structure / Rhythm / Lexicon / Syntax / Readability) using mdBook draft chapters (`- [Title]()`) as non-clickable group headers; FR `Version française` block mirrors the same shape (Structure / Rythme / Lexique — Syntaxe and Lisibilité materialise as those FR translations land). `markdownlint` MD042 disabled globally to permit the empty-link draft-chapter syntax (matches the pre-existing MD025 carve-out for SUMMARY-required multiple H1s). Picked over (B) "one sub-page per category" — B doubles the page count without adding clarity the index table doesn't already provide. | — | 2026-04-25 docs UX critique (Block E) |
| <a id="f105"></a>[F105](#f105) | ✅ Shipped 2026-04-27 — `docs/src/references.md` (EN, under Project) and `docs/src/fr/references.md` (FR, under Version française) consolidate every cited source into one informative surface, preserving the full taxonomy of `examples/REFERENCES.md` (legend, per-domain sections, rule → reference summary table) and the scholarly-honesty note. `examples/REFERENCES.md` becomes a thin redirect to the docs sources — kept because external citations may already point there. Both rule indexes (EN + FR) cross-link to the new page next to the existing `RULES.md` pointer. Per-citation anchors deferred — readers scan the page or use browser search; if a need surfaces, file a follow-up. | — | 2026-04-25 docs UX critique (Block E) |
| <a id="f105b"></a>[F105b](#f105b) | ✅ Shipped 2026-04-27 — Per-citation anchors (`<a id="author-year">`) on every entry of `references.md` + `fr/references.md`, plus a `## References` / `## Références` section on every rule page (25 EN + 13 FR) listing the relevant citations as anchored links. The references page now links rule IDs in `→ Relevant to:` lines and the rule → reference summary table to their per-rule mdBook pages — bidirectional rules ↔ references. Verified canonical URLs (DOI, publisher landing page, official archive — researched in 2026-04-27 lap, 26 of 34 academic citations carry one) added inline as raw HTML anchors with `rel="nofollow noopener noreferrer" target="_blank"`: `nofollow` so the docs site does not vouch for outside content, `noopener noreferrer` for new-tab safety. Sources without a verifiable canonical URL stay text-only — no guessed links. Subsumes the [F30](#f30) rule-mention linking pass for the references-page surface; wider [F30](#f30) audit (rule mentions in `docs/src/guide/*` prose pages) stays open. | — | [F105](#f105) follow-up filed 2026-04-27 |
| <a id="f127"></a>[F127](#f127) | **Code → docs codegen for data-heavy surfaces.** Several docs surfaces are hand-maintained today but derivable from the rule registry and config types: per-rule pages (defaults, weight, condition tags, category, severity), `docs/src/rules/index.md` table, `docs/src/guide/profiles.md` threshold tables, `docs/src/guide/conditions.md` tag list, `docs/src/guide/suppression.md` directive list, JSON output schema page. Proposed shape: a `lucid-lint manifest --format=json` subcommand emits one document with everything pulled from `default_rules`, `Category::for_rule`, `scoring::default_weight_for`, the `Condition` enum, profile presets, and `schemars::JsonSchema` derives. A `just docs-gen` script renders **marked regions** in existing prose pages (`<!-- BEGIN: lucid-gen rule-defaults id=structure.sentence-too-long lang=en -->` … `<!-- END: lucid-gen -->`) so prose around the data stays hand-authored. CI runs `just docs-gen` and fails on non-empty `git diff` (same shape as [F27](#f27) for the roadmap sync). Translation surface shrinks to prose only — labels (`Default`, `Profile`, `Threshold`) come from a small `i18n.toml` keyed by `(lang, key)`, the *data* is identical across languages by construction. **Block on [F25](#f25) guide translations landing first** so we don't change the substrate mid-translation; open after Block C closes. | 🟡 Later | 2026-05-01 Block C planning, [F25](#f25) / [F28](#f28) / [F42](#f42) follow-up |
| <a id="f106"></a>[F106](#f106) | **Landing-page polish.** `docs/src/introduction.md` already plays both roles today: lens-motif hero, before/after figure, "what makes it different", quick-taste terminal capture, "where to next". A real landing-page push only earns its cost when there's a *first consumer outside the maintainer* (project gets adopted, traffic shows up). Until then, polishing is design work without a forcing function. Notes for when triggered: more positioning above the fold, demo grid for the rule families (one canonical example per category), CTA toward profiles + quick-start, lens-motif extension already validated for use across the page. | 🟢 Speculative | 2026-04-25 docs UX critique (Block E) |

### Docs site — theming

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f26"></a>[F26](#f26) | ✅ MVP shipped in v0.2 via DOM-level trim in `lucid-navigation.js` — the picker now shows three honest items (`Auto · Lucid light · Lucid dark`); the stock Rust / Navy / Ayu `<li>`s are marked `hidden` so they're inert for keyboard and screen-reader. CSS class mapping is unchanged (`.light` / `.rust` → lucid-light, `.coal` / `.navy` / `.ayu` → lucid-dark), so pre-existing localStorage selections still render correctly. Follow-up (optional): a full `index.hbs` override to drop the stock markup entirely rather than hide it; preferred once the mdBook upgrade cadence settles. | 🟡 Later | v0.1 docs `/colorize` session; mdBook stock limitation |
| <a id="f73"></a>[F73](#f73) | ✅ Pre-deploy font-leak gate shipped in v0.2 — `just docs-check-clean` rebuilds the book, runs `scripts/sanitize-stock-css.py`, and greps the output for active `font-family` / `--*-font` / `local()` references to `Open Sans` or `Source Code Pro`. Not wired into `just check` (mdbook build is too slow for the dev loop); wire it into the docs-publish CI workflow before any release-candidate goes live. | 🟡 Later | v0.2 `/critique` polish pass follow-up |
| [F84](#f84) | ✅ Shipped in v0.2.1 — fixed localhost 404.html rendering under `mdbook serve`. `book.toml` sets `site-url = "/lucid-lint/"` for GitHub Pages, and mdBook emits `<base href="/lucid-lint/">` into 404.html (only there). On localhost that prefix doesn't exist, so the browser's preload scanner fired 18 stylesheet/script requests with the wrong prefix before the page recovered via a second fetch. The previous JS workaround in `docs/theme/head.hbs` rewrote `<base>` at parse time, but ran after the preload scanner. Fix: `just docs-serve` now sets `MDBOOK_OUTPUT__HTML__SITE_URL=/` for the serve process, so 404.html carries `<base href="/">` on localhost and the correct `<base href="/lucid-lint/">` in production builds; the JS workaround is removed. | — | 2026-04-23 Block A |

### Docs site — reading preferences

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f33"></a>[F33](#f33) | Full reading-preferences popover UI — cog button in the header opens a popover with font radio (Atkinson / Standard / OpenDyslexic), line-spacing slider (1.4–2.0, 0.05 step) and text-size slider (90–130 %, 5 % step). v0.1 ships only the Introduction-page demonstrator; the CSS-variable plumbing (`--reading-scale`, `--reading-line-height`, `[data-font]`) is already in place, so this is UI work only. | 🟡 Later | v0.1 docs `/shape` + `/typeset` sessions |
| [F34](#f34) | Responsive / mobile adaptation — right-rail page TOC and header controls collapse gracefully below 700 px; touch targets verified ≥ 44 × 44 px; sidebar drawer behaviour polished. | 🔴 Next | v0.1 docs `/layout` session, deferred to `/adapt` |
| <a id="f35"></a>[F35](#f35) | Accessibility audit sweep — full AAA pass on both themes (contrast, focus order, `prefers-reduced-motion` coverage, keyboard-only walk-through, skip-link), plus a published accessibility statement page. First audit pass ran 2026-04-22 (17/20, 0 P0, 2 P1, 3 P2); findings filed as [F35a](#f35a)–[F35d](#f35d) below. [F35](#f35) stays open until the statement page ships and P1s are cleared. | 🟡 In progress | v0.1 docs `/audit` plan |
| [F35a](#f35a) | ✅ Shipped 2026-04-22 — `theme/index.hbs` is now forked from mdBook v0.5.2's upstream template (minimal-diff approach, documented so future mdBook upgrades stay a mechanical re-sync). The skip link and EN / FR language switch are emitted as server-rendered HTML inside `<body>` and inside `.right-buttons`; both language variants are rendered and CSS in `lucid-layout.css` hides the wrong-locale copy based on `html[lang]` (which `head.hbs` sets synchronously before first paint on `/fr/` pages). The previous `skipLink()` and `langSwitch()` IIFEs in `lucid-navigation.js` are gone; the only remaining JS on the skip-link path is a progressive-enhancement smooth-scroll handler. WCAG 2.4.1 Bypass Blocks now passes with JS disabled. Unblocks [F26](#f26) (stock theme labels can be collapsed at the markup level). | — | [F35](#f35) audit 2026-04-22 |
| [F35b](#f35b) | **Drop `role="radiogroup"`/`role="radio"` on reading-demo chips** (P2 from [F35](#f35) audit). Current markup declares radiogroup semantics but the JS only binds `click` — arrow-key traversal is missing, so the ARIA contract is broken. Simpler fix is to switch to plain buttons with `aria-pressed` (the chips are preset toggles, not radios) rather than add a keyboard handler. Promoted to 🔴 Next on 2026-04-24 (brainstorm-next-cycles). | 🔴 Next | [F35](#f35) audit 2026-04-22 |
| <a id="f35c"></a>[F35c](#f35c) | ✅ Closed 2026-05-01 as **audit false-positive**. The 2026-04-22 audit reported that `.lucid-stance__idea` lost its colour tint under `prefers-reduced-motion`. Re-audit on 2026-05-01 against `docs/theme/css/lucid-layout.css:567-622` and `docs/theme/css/lucid-typography.css:424-431`: no `@media (prefers-reduced-motion: reduce)` rule touches `.lucid-stance__idea`; the global reduced-motion reset zeroes `animation-duration` / `transition-duration` only and never overrides `background-color`. The only rule that strips the tint is `@media (forced-colors: active)` (line 620–622), which is intentional (Windows High Contrast users get the OS palette, position-based pairing carries the meaning). The original audit appears to have conflated `forced-colors: active` with `prefers-reduced-motion: reduce`. No code change needed; accessibility.md known-limitation bullet removed in the same commit. | — | [F35](#f35) audit 2026-04-22 |
| [F35d](#f35d) | **Publish an accessibility statement page** (`docs/src/accessibility.md`, FR counterpart at `docs/src/fr/accessibility.md`). EN page carries the stated bar (WCAG 2.2 AAA), first audit pass result (2026-04-22, 17/20), a "Known limitations" block listing [F35a](#f35a)/b/c pending, report route, and audit cadence. FR stub mirrors the limitations block. Shipped 2026-04-22. | 🟢 Shipped | [F35](#f35) audit 2026-04-22 |
| <a id="f36"></a>[F36](#f36) | Final polish pass — optical alignment, spacing rhythm, edge-state copy, favicon PNG fallback, social-card refinement, re-running `/critique` to verify the score moves above 30/40. | 🟡 Later | v0.1 docs `/polish` plan |
| <a id="f121"></a>[F121](#f121) | **Terminal-demo accessibility — keep VHS, add motion + transcript fallbacks.** Audited VHS (charmbracelet/vhs, active 2026-04-27, headless+CI-reproducible) vs. terminalizer (~16k stars, last commit 2024-08-29, effectively unmaintained). Verdict: keep VHS — `.tape` files are text-diffable, the build is reproducible, and the motion-handling problem is the same on both tools, so it is not a recorder choice but a wrapping problem. **AAA gap to close:** every embedded GIF on the docs site (today: `docs/src/assets/tty/explain.gif` plus future captures) must (1) honour `prefers-reduced-motion` — browsers do not pause animated GIFs automatically, so a static `<picture>` source-set with a still PNG fallback served when `(prefers-reduced-motion: reduce)` is the right shape; (2) carry the per-step transcript inside the page so non-sighted, screen-reader, and reduced-motion readers reach the same content as motion viewers — a stepwise prose block (e.g. `<details><summary>Transcript</summary>…</details>` with each tape command + its visible output as a list) sitting next to the GIF, plus an `alt=` summary on the image itself. The `.tape` source already encodes the steps deterministically — a small generator can emit the transcript from the same file the GIF is built from, keeping motion view and transcript view pair-locked. Phase: v0.3 marketing. | 🟡 Later | 2026-04-27 Block E recon |

### Quality features

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f12"></a>[F12](#f12) | Score evolution dashboard across runs | 🟢 Speculative | Rule 11, inspired by coverage reports |
| <a id="f98"></a>[F98](#f98) | **Mutation testing via `cargo-mutants`.** ✅ Baseline shipped 2026-04-25 — dev-tool installed, `just mutants <file>` recipe added (timeout 60 s, no-shuffle for reproducibility), four-file probe run: `sentence_too_long.rs` 6 caught / 0 missed / 4 unviable (100 %), `scoring.rs` 18 / 0 / 2 (100 %), `engine.rs` 5 / 0 / 12 (100 %), `low_lexical_diversity.rs` 29 / 47 / 5 (36 %). Canonical reference rule + cross-cutting layer score perfectly; the lexical-diversity rule has two clear test gaps surfaced as [F108](#f108) + [F109](#f109). Triage methodology: cluster missed mutants by site → one ROADMAP entry per root cause, not per mutant. | ✅ Done | Stream-2 testing brainstorm, 2026-04-24 |
| <a id="f108"></a>[F108](#f108) | **`low_lexical_diversity::ratio_at_anchor_min` — assert reported ratio in tests.** ✅ Shipped 2026-04-25. Added `reported_ratio()` helper (parses the documented message format) and three new test fixtures: `reported_ratio_is_minimum_observed_in_cluster` (50 W + 100 cache + 50 V → cluster-exit path with min ratio 0.01 deep mid-slide, not at anchor), `flush_path_reports_final_ratio` (cache-only doc → flush path), and `exactly_window_size_tokens_runs_the_check` (boundary on the early-return guard). Ratio assertion uses `(ratio - 0.01).abs() < 1e-9` so floating-point shifts from arithmetic mutations are caught. Bonus refactor (typed-ratio field on `Diagnostic`) deferred — string parsing is fine for the test-only consumer. | ✅ Done | [F98](#f98) baseline 2026-04-25 |
| <a id="f109"></a>[F109](#f109) | **`low_lexical_diversity::check` — borderline-cluster fixtures.** ✅ Shipped 2026-04-25 alongside [F108](#f108). Added `cluster_starts_at_strict_inequality` and `ratio_exactly_at_threshold_does_not_trigger` — the latter uses 49 W + 51 cache so the only full window has unique=50 → ratio exactly 0.50 = `min_ratio`. With strict `<` the rule must not trigger; a `< → <=` flip would emit a diagnostic and fail the test. Combined effect: the rule's mutation score moved from 36 % (29 / 47 / 5) at [F98](#f98) baseline to **89 %** (68 / 8 / 5). The remaining 8 missed mutants are equivalent under the current rule logic — defensive guards (`start_index + window > tokens.len()` is unreachable in normal flow because `anchor.index ≤ len − window`), or initial values the slide loop unconditionally overwrites (`let mut best = unique / window` is replaced as soon as a lower ratio appears, which it always does in a real cluster). Closing those would require rule refactoring (e.g. starting `best` at `f64::INFINITY` to prove the initial computation is dead) — diminishing returns; deferred. | ✅ Done | [F98](#f98) baseline 2026-04-25 |
| <a id="f99"></a>[F99](#f99) | **Property-based tests via `proptest`** (dep already in `[dev-dependencies]`, zero call sites today — paid for, unused). Four invariants in `tests/properties.rs`, deliberately small: (1) `split_sentences` never drops a non-whitespace character on round-trip, (2) re-linting an identical string yields identical diagnostics (engine idempotence), (3) for threshold-driven rules, `public`-profile diagnostics are a superset of `dev-doc`-profile diagnostics on the same input (profile monotonicity), (4) `Engine::lint_str` never panics on arbitrary valid UTF-8 ≤ 10KB. Goal: fortify tokenizer / engine seams, not rewrite the suite. | 🟡 Later | Stream-2 testing brainstorm, 2026-04-24 |
| <a id="f100"></a>[F100](#f100) | **LLM false-positive miner via Claude Code.** Dev-only audit script (not a test, not a CI gate) that runs lucid-lint across the CC corpus, asks Claude to flag diagnostics that look wrong, writes a triage report to `.personal/audits/`. Reframed from the original "LLM-as-Judge harness" after Devil's Advocate surfaced three blockers on the gating form: non-determinism across Claude model versions, ambiguity about whether a disagreement indicts the rule or the judge, cost / wall-clock at 600×N scale. The miner form sheds all three — human triages, Claude suggests. Respects prime directive #4 (deterministic core, no LLM) because it lives entirely outside the library crate and never blocks `just check`. Wait until v0.3 `lucid-lint-nlp` plugin work surfaces the need for correctness review at scale. | 🟢 Speculative | Stream-2 testing brainstorm, 2026-04-24 |
| [F93](#f93) | Tokenizer `split_sentences` `Vec\<char\>` allocation. The helper collects the full input into a `Vec\<char\>` per call to support lookbehind (`chars[idx-1]`) and arbitrary lookahead (`chars[idx+1..].find(!ws)` for ellipsis-continuation). Nominal waste on real corpus is ~5% of the `split_sentences` budget (bench shows 35µs total, `Vec\<char\>` alloc ~1–2µs). Refactor to a small ring-buffer + `Peekable\<CharIndices\>` is feasible but high-churn for low ceiling. Revisit only if profiling pins the tokenizer as a bottleneck. | 🟢 Speculative | Stream-2 code review 2026-04-24 (measured; deferred) |
| <a id="f89"></a>[F89](#f89) | Unify rule-page example figures on the `.lucid-stance` component. Today the intro page uses a custom `.lucid-stance` figure (Before / After side-by-side, colour-matched ideas, diagnostic in the figcaption), while rule pages use plain H3 + blockquote + fenced `text` for the diagnostic (see `docs/src/rules/sentence-too-long.md`). The H3 form works and is cheap to roll out, but wide screens could show stronger Before↔After pairing with the side-by-side figure. Scope: extract `.lucid-stance` into a reusable component (mdBook include or raw HTML pattern), tune the styling for in-content width (rule pages sit inside the narrower content column, not the landing-page hero), one figure per language, drop the H3 subsections in favour of a `data-lang` attribute surfaced as a chip on the figure. Ship only after the H3-based rollout has landed across all example-bearing rule pages and the unified pairing is confirmed as the dominant reader complaint. | 🟢 Speculative | 2026-04-23 docs clarity session — H3 subsections landed as the lightweight option; [F89](#f89) parks the heavier unify-the-components path |
| <a id="f88"></a>[F88](#f88) | `--fix` mode for the mechanical subset of rules — promoted to 🟡 Later on 2026-04-24 (brainstorm-next-cycles, 0.3 Should). Narrow scope locked: `lexicon.all-caps-shouting` (lowercase the run), `lexicon.redundant-intensifier` (drop the intensifier), `structure.mixed-numeric-format` (normalise to the detected majority style), `structure.line-length-wide` (rewrap to `max_chars`). All other rules stay report-only — cognitive-load judgments need the author to choose the rewrite. Borderline `structure.heading-jump` stays out of the initial cut. Design: per-rule `fixable: bool` metadata on the `Rule` trait, `--fix` flag walks diagnostics in document order applying only those with concrete replacements, writes files in place (or emits a unified diff with `--fix=print`), exits with count of fixes applied. Conservative default: `--fix` only touches the explicitly-fixable set, never guesses. | 🟡 Later | 2026-04-23 docs clarity session — framing "lucid-lint reports, you rewrite" surfaced the question |

### Scope control

File/directory discovery. Distinct from suppression (below): scope
control excludes inputs before they are scanned; suppression hides
diagnostics after scanning.

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f78"></a>[F78](#f78) | ✅ Shipped in v0.2 — `exclude = [...]` glob list in `[default]` of `lucid-lint.toml` and `--exclude <GLOB>` CLI flag (comma-delimited, repeatable). Patterns match against paths relative to the walked root; matching directories are pruned, not descended. Explicit file args bypass exclusion. Backed by `globset`. See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#excluding-paths). `.lucidignore` (gitignore-style file) deferred to [F78b](#f78b) if user demand surfaces. | — | Dogfood feedback 2026-04-21 |
| <a id="f78b"></a>[F78b](#f78b) | `.lucidignore` file (gitignore-style, with negations and nested files). Different crate (`ignore`) and a larger test matrix than the glob-list MVP. Ship only if users ask — the `exclude` list in `lucid-lint.toml` covers the dominant use case. | 🟢 Speculative | [F78](#f78) deferral, 2026-04-21 |

### Suppression mechanism

v0.1 ships the minimal inline-disable directive (see brainstorm
`brainstorm/20260419-inline-disable-feature.md`). Extensions deferred:

| ID | Item | Priority | Origin |
|---|---|---|---|
| <a id="f18"></a>[F18](#f18) | ✅ Block form shipped in v0.2: `<!-- lucid-lint-disable <rule-id> -->` … `<!-- lucid-lint-enable -->` silences one rule across every line in the scope. `enable` with no argument closes every open scope; with a rule id, closes only that rule's scope (so overlapping disables for different rules can nest). Unterminated `disable` extends to end-of-document. See [RULES.md → Suppressing diagnostics](RULES.md#suppressing-diagnostics). | — | v0.1 inline-disable brainstorm |
| <a id="f19"></a>[F19](#f19) | ✅ Shipped in v0.2 — top-level `[[ignore]]` array-of-tables in `lucid-lint.toml`, each entry with a required `rule_id` silences every diagnostic for that rule across Markdown, plain text, and stdin. Unknown ids tolerated. Applied post-engine, pre-scoring, so scoring / rendering / exit-code logic all see the filtered view. Scope broadened from the roadmap's original "`.txt` and stdin" wording because a global filter is simpler and more useful; Markdown users can still prefer inline directives for local silencing. `reason` field tracked as [F20](#f20). See [`docs/src/guide/configuration.md`](docs/src/guide/configuration.md#silencing-rules-globally). | — | v0.1 inline-disable brainstorm |
| [F20](#f20) | `reason="..."` field, optional in v0.1, surfaced in reports and optionally required via config | 🟡 Later | v0.1 inline-disable brainstorm |
| <a id="f21"></a>[F21](#f21) | File-level directive (`disable-file`) and multi-rule lists | 🟡 Later | v0.1 inline-disable brainstorm |
| <a id="f146"></a>[F146](#f146) | **`--severity-floor=warning` CLI flag.** Routed 2026-05-02 (`.personal/brainstorm/20260502-async-book-pr-timing.md`); supports the async-book audit-and-PR play (tracked in `.personal/promotion-channels.md`). Need: external audit PRs (async-book and adjacent) want a "narrow audit" mode that drops `info` diagnostics from output and from score impact, so the PR demonstrates value on the unambiguous wins (sentence-too-long, redundant-intensifier, unclear-antecedent, paragraph-too-long) without the contested ones (`info`-tier weasel words after [F144](#f144) lands). Shape: `--severity-floor={info,warning,error}` with default `info` (current behavior). Pairs with [F144](#f144): once weasel-words emits `info` on quantifiers, an auditor running `--severity-floor=warning` ships a PR where reviewers see only the prose changes the tool is most confident about. Implementation is a post-engine filter (mirrors [F19](#f19) `[[ignore]]` post-engine pre-scoring shape) so JSON / SARIF / TTY all see the same filtered view; scoring excludes filtered diagnostics so `--min-score` interacts correctly. Definition of done: CLI flag in `src/cli.rs`, filter in `src/engine.rs` post-rule pre-score, two snapshot tests (info-included default vs warning-floor), docs in `docs/src/guide/configuration.md` + FR mirror with a "running a narrow audit on someone else's repo" worked example, CHANGELOG entry. | 🔴 Next | F113 audit-and-PR play (2026-05-02) |

---

## v0.1 — Released 2026-04-20

Shipped in the tag: all 17 rules across 5 phases, the minimal inline-disable directive, and the mdBook documentation site (Lucid light / Lucid dark themes, Atkinson Hyperlegible Next / Literata / Commit Mono / OpenDyslexic typography layer, reading-preferences demonstrator, accessibility page, EN/FR header switch with v0.2 FR-stub). See [`CHANGELOG.md`](CHANGELOG.md) for the full release notes.

### Rules (17 / 17) ✅

#### Phase 1 — Deterministic structural rules

| Status | Rule | Notes |
|---|---|---|
| ✅ | `structure.paragraph-too-long` | Sentence-count + word-count thresholds per profile (`src/rules/paragraph_too_long.rs`) |
| ✅ | `structure.deeply-nested-lists` | Flags list items nested beyond profile depth (`src/rules/deeply_nested_lists.rs`) |
| ✅ | `structure.heading-jump` | Walks section depths, flags jumps > +1 level (`src/rules/heading_jump.rs`) |

#### Phase 2 — Simple text rules

| Status | Rule | Notes |
|---|---|---|
| ✅ | `structure.sentence-too-long` | Reference implementation — template for the 15 others (`src/rules/sentence_too_long.rs`) |
| ✅ | `structure.excessive-commas` | Per-profile comma-per-sentence threshold (`src/rules/excessive_commas.rs`) |
| ✅ | `rhythm.consecutive-long-sentences` | Intra-paragraph streak of long sentences (`src/rules/consecutive_long_sentences.rs`) |

#### Phase 3 — Lexical rules with word lists

| Status | Rule | Notes |
|---|---|---|
| ✅ | `lexicon.weasel-words` | Per-language phrase list, word-boundary match (`src/rules/weasel_words.rs`) |
| ✅ | `lexicon.unexplained-abbreviation` | Pattern-based (v0.1); definition-awareness tracked as [F9](#f9) (`src/rules/unexplained_abbreviation.rs`) |
| ✅ | `lexicon.jargon-undefined` | Pattern-based, profile-activated category lists (`src/rules/jargon_undefined.rs`) |
| ✅ | `lexicon.excessive-nominalization` | Per-sentence suffix-based density check (`src/rules/excessive_nominalization.rs`) |
| ✅ | `rhythm.repetitive-connectors` | Sliding-window connector frequency, one diagnostic per cluster (`src/rules/repetitive_connectors.rs`) |

#### Phase 4 — Global metric

| Status | Rule | Notes |
|---|---|---|
| ✅ | `readability.score` | Per-document Flesch-Kincaid grade; info under threshold, warning above (`src/rules/readability_score.rs`) |

#### Phase 5 — Heuristic rules (hardest)

| Status | Rule | Notes |
|---|---|---|
| ✅ | `structure.long-enumeration` | Shared enumeration detector with `structure.excessive-commas`; suggests list conversion (`src/rules/long_enumeration.rs`, `src/rules/enumeration.rs`) |
| ✅ | `structure.deep-subordination` | Counts subordinators between strong-punct breaks; skips pronoun enumerations (`src/rules/deep_subordination.rs`) |
| ✅ | `syntax.passive-voice` | Heuristic `be/être`+past-participle detector; POS-based detection remains a `lucid-lint-nlp` plugin candidate (`src/rules/passive_voice.rs`) |
| ✅ | `syntax.unclear-antecedent` | Info-level heuristic: bare demonstrative + verb, or paragraph-start personal pronoun (`src/rules/unclear_antecedent.rs`) |
| ✅ | `lexicon.low-lexical-diversity` | Sliding-window TTR over non-stopword content tokens (`src/rules/low_lexical_diversity.rs`) |

### Cross-cutting features

| Status | Feature | Notes |
|---|---|---|
| ✅ | Minimal inline-disable | `<!-- lucid-lint disable-next-line <rule-id> -->` for Markdown inputs, single rule id, optional reason. See [RULES.md → Suppressing diagnostics](RULES.md#suppressing-diagnostics). Block form, config ignores, file-level scope and required `reason=` are tracked as [F18](#f18)–[F21](#f21) below. |
| ✅ | Accessibility page in the docs | `docs/src/accessibility.md` covers the WCAG 2.2 AAA bar, the reading-preferences control, typography credits (Atkinson Hyperlegible Next — Braille Institute; OpenDyslexic — Abelardo Gonzalez; Literata — TypeTogether), keyboard shortcuts, and how the site dogfoods the project's mission. Linked from the sidebar and the footer. |

---

## Design decisions from v0.1 session

### Diagnostic structure

Decided: v0.1 diagnostics carry only what cannot be trivially recomputed.

```rust
pub struct Diagnostic {
    pub rule_id: String,
    pub severity: Severity,
    pub location: Location,
    pub section: Option<String>,  // H2 (or configured level) containing the diagnostic
    pub message: String,
}
```

**Kept** : `section` is stored at emission. Recomputing it a posteriori would require re-parsing the Markdown to walk headings and match locations. Expensive. Storing it is cheap.

**Omitted** : `category` is a pure function of `rule_id`. A `category_of(rule_id) -> Category` utility derives it in O(1). No duplication in diagnostics.

**Omitted** : `weight` and `suggestion` are not used in v0.1 and will be introduced when the hybrid scoring model ([F14](#f14)) lands.

This aligns with the "open to change, not abstracted for change" principle applied earlier to format handling: struct fields can be added later without breaking JSON serialization compatibility.

---

## Points deferred from v0.1 session

A number of configuration and ergonomics questions were raised but postponed. They will be addressed before or during v0.2:

### Configuration

- Config file format decision: TOML (recommended), YAML, or JSON
- Config filename convention
- Profile name finalization (`dev-doc`, `public`, `falc` confirmed)
- Naming convention for rules (kebab-case confirmed, flat vs. hierarchical namespace)
- Rule codes (short codes like `LL001` vs. name-only)
- Suppression mechanism (`# lucid-lint disable-next`, block disable/enable, ignore file)

### Output

- TTY format (colors, snippets, condensed report)
- Structured format: JSON schema, SARIF exactness, native format
- Exit code granularity (0/1 vs. graduated)

### Architecture

- Language detection: simple heuristic (stop-words) vs. dedicated crate (`whatlang`)
- Parallelism: `rayon` for multi-file processing
- Glob patterns and `.lucidignore` (now tracked as **[F78](#f78)**)
- Core library exposed as `lucid-lint-core` for third-party integration

### Project

- Repo structure: single crate vs. Cargo workspace
- Reference corpus for testing
- README v0.1 content and positioning
- Tagline and visual identity

---

## Contribution invitation

Future rules and plugins can be proposed by the community. The default jargon and stoplists (`lexicon.jargon-undefined`, `lexicon.weasel-words`, `lexicon.low-lexical-diversity`) are especially welcome targets for community pull requests to expand coverage across domains and languages.
