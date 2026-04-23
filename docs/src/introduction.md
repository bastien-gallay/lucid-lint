<svg class="lucid-landing__mark" viewBox="0 0 120 36" aria-hidden="true" focusable="false">
  <title>lucid-lint lens</title>
  <circle cx="18" cy="18" r="12" fill="none" stroke="currentColor" stroke-width="2.25"/>
  <circle cx="18" cy="18" r="2" fill="currentColor"/>
  <line class="lucid-landing__mark-line" x1="32" y1="18" x2="116" y2="18" stroke-width="1.75" stroke-linecap="round"/>
</svg>

<h1 id="introduction" class="lucid-landing__title">Introduction</h1>

<p class="lucid-audience" role="doc-subtitle">
Built for readers whose attention is stretched — ADHD, dyslexia, fatigue, a
second language, or an accessibility-sensitive context.
</p>

`lucid-lint` reads your Markdown or plain text and flags the moments that
make prose hard to process. It does not rewrite your voice. It hands you a
short list and gets out of the way.

<figure class="lucid-stance" aria-label="Before and after: a sentence flagged by lucid-lint">
  <div class="lucid-stance__pair">
    <div class="lucid-stance__side" data-stance-side="before">
      <p class="lucid-stance__label">Before</p>
      <p class="lucid-stance__prose">
        <span class="lucid-stance__idea" data-idea="1">The caching subsystem, which was introduced in an earlier milestone,</span>
        <span class="lucid-stance__idea" data-idea="2">turned out to interact poorly with the new request pipeline under sustained load,</span>
        and
        <span class="lucid-stance__idea" data-idea="3">the investigation that followed required multiple rounds of profiling.</span>
      </p>
    </div>
    <div class="lucid-stance__side" data-stance-side="after">
      <p class="lucid-stance__label">After</p>
      <p class="lucid-stance__prose">
        <span class="lucid-stance__idea" data-idea="1">The caching subsystem was introduced earlier.</span>
        <span class="lucid-stance__idea" data-idea="2">It interacts poorly with the new request pipeline under sustained load.</span>
        <span class="lucid-stance__idea" data-idea="3">The investigation required several rounds of profiling.</span>
      </p>
    </div>
  </div>
  <figcaption class="lucid-stance__caption">
    Three ideas, colour-matched left to right — the rewrite shortens the
    sentences without losing any of them. <code>lucid-lint</code> flagged
    <code>sentence-too-long</code> (43 words) and
    <code>consecutive-long-sentences</code>. It did not propose the
    rewrite — that's yours.
  </figcaption>
</figure>

## What makes it different

Most prose tools measure style (`write-good`), grammar (`Antidote`), or a
surface readability score (Flesch). `lucid-lint` measures **cognitive
load** — the mental effort a reader spends to understand a sentence. It
flags the patterns that the research behind Sweller, Gibson, Graesser,
and [Coh-Metrix](http://cohmetrix.com) single out.

- **Bilingual EN/FR** from day one, with equal quality.
- **Deterministic** by default. Identical input produces identical output.
  LLM-based rules live in optional plugins.
- **CI-native**. Plain-text and JSON outputs; exit codes that pre-commit and
  GitHub Actions understand without a wrapper.
- **Profile-based**. Pick `dev-doc`, `public`, or `falc` (Easy-to-Read),
  then override per rule if you want.

## Project status

`lucid-lint` is at v0.2 (released 2026-04-22). All 25 rules listed in
[`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md)
are shipped (17 from v0.1, 8 added during the v0.2 cycle), alongside
the [hybrid scoring model](./guide/scoring.md) —
a global `X / max` score plus five per-category sub-scores, computed on
top of the diagnostics. Pre-1.0: breaking changes remain possible
between minor versions. See the [roadmap](./roadmap.md) for what
comes next.

## Quick taste

A clean file earns the full 100/100 and a wordmark banner — the
peak-end moment of a passing lint run:

![Terminal capture: a clean lucid-lint run showing the three-part wordmark banner, the message "No issues found.", and a score block reading 100/100 with every category bar full](./assets/tty/score-clean.gif)

```text
~~~~~ ⟨ • ⟩ ─────  lucid-lint  v0.2.0
                   cognitive accessibility linter · prose · EN / FR
                   ────────────────────────────────────────────────

No issues found.

────────────────────────────────────────────────────────────
score: 100/100
       structure    █████  20/20
       rhythm       █████  20/20
       lexicon      █████  20/20
       syntax       █████  20/20
       readability  █████  20/20
```

```bash
cargo install lucid-lint

# Lint a file
lucid-lint check README.md

# Strictest profile (Easy-to-Read / FALC)
lucid-lint check --profile=falc docs/

# Stdin
echo "This is a test sentence." | lucid-lint check -

# JSON for CI
lucid-lint check --format=json docs/

# Fail the build if the aggregate score drops below 85/100
lucid-lint check --min-score=85 docs/
```

## Where to next

- [Installation](./guide/installation.md) — how to install it.
- [Quick start](./guide/quick-start.md) — a five-minute walkthrough.
- [Profiles](./guide/profiles.md) — pick the one that fits.
- [Rules reference](./rules/index.md) — all twenty-five rules explained.
- [Accessibility](./accessibility.md) — the WCAG AAA bar and how the site
  itself dogfoods the project.

## Reading preferences

<section class="reading-demo reading-demo--chips" aria-label="Reading preferences demonstrator">
  <p class="reading-demo__note">
    The whole site is built as a reading companion. Pick the font that reads
    best for you — it will stick across pages.
  </p>
  <article class="reading-demo__preview">
    <p class="reading-demo__label" data-chip-label>Atkinson Hyperlegible Next</p>
    <p class="reading-demo__sample" data-demo="atkinson">
      A dense paragraph can ask a lot of a stretched mind. Every comma, every
      clause, every bracketed aside adds a little cost. Good prose keeps
      that cost low.
    </p>
  </article>
  <div class="reading-demo__chips" role="radiogroup" aria-label="Reading font">
    <button type="button" class="reading-demo__chip" data-apply="atkinson" role="radio" aria-checked="true">
      Atkinson <span class="reading-demo__default">default</span>
    </button>
    <button type="button" class="reading-demo__chip" data-apply="standard" role="radio" aria-checked="false">
      Standard
    </button>
    <button type="button" class="reading-demo__chip" data-apply="dyslexic" role="radio" aria-checked="false">
      OpenDyslexic
    </button>
  </div>
  <p class="reading-demo__caption">
    Line spacing and text size are on the way as sliders. Until then, pick a
    font and your browser's zoom is honoured.
  </p>
</section>

## License

<!-- lucid-lint disable-next-line lexicon.unexplained-abbreviation -->

Dual-licensed under MIT or Apache-2.0, at your option.
