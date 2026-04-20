# Introduction

<p class="lucid-audience" role="doc-subtitle">
Built for readers whose attention is stretched — ADHD, dyslexia, fatigue, a
second language, or an accessibility-sensitive context.
</p>

`lucid-lint` reads your Markdown or plain text and flags the moments that
make prose hard to process. It does not rewrite your voice. It hands you a
short list and gets out of the way.

<section class="reading-demo" aria-label="Reading preferences demonstrator">
  <p class="reading-demo__note">
    The whole site is built as a reading companion. Pick the font that reads
    best for you — it will stick across pages.
  </p>
  <div class="reading-demo__grid">
    <article class="reading-demo__card">
      <p class="reading-demo__label">Atkinson Hyperlegible Next</p>
      <p class="reading-demo__sample" data-demo="atkinson">
        A dense paragraph can ask a lot of a stretched mind. Every comma, every
        clause, every bracketed aside adds a little cost. Good prose keeps
        that cost low.
      </p>
      <button type="button" class="reading-demo__apply" data-apply="atkinson" aria-pressed="true">
        Use this <span class="reading-demo__default">— default</span>
      </button>
    </article>
    <article class="reading-demo__card">
      <p class="reading-demo__label">Standard (serif headings)</p>
      <p class="reading-demo__sample" data-demo="standard">
        A dense paragraph can ask a lot of a stretched mind. Every comma, every
        clause, every bracketed aside adds a little cost. Good prose keeps
        that cost low.
      </p>
      <button type="button" class="reading-demo__apply" data-apply="standard" aria-pressed="false">
        Use this
      </button>
    </article>
    <article class="reading-demo__card">
      <p class="reading-demo__label">OpenDyslexic</p>
      <p class="reading-demo__sample" data-demo="dyslexic">
        A dense paragraph can ask a lot of a stretched mind. Every comma, every
        clause, every bracketed aside adds a little cost. Good prose keeps
        that cost low.
      </p>
      <button type="button" class="reading-demo__apply" data-apply="dyslexic" aria-pressed="false">
        Use this
      </button>
    </article>
  </div>
  <p class="reading-demo__caption">
    Line spacing and text size are on the way as sliders. Until then, pick a
    font and your browser's zoom is honoured.
  </p>
</section>

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

`lucid-lint` is in v0.2. All 17 rules listed in
[`RULES.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/RULES.md)
shipped in v0.1. v0.2 adds the [hybrid scoring model](./guide/scoring.md) —
a global `X / max` score plus five per-category sub-scores, computed on
top of the existing diagnostics. See the [roadmap](./roadmap.md) for
what comes next.

## Quick taste

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

# Fail the build if the aggregate score drops below 85/100 (v0.2+)
lucid-lint check --min-score=85 docs/
```

## Where to next

- [Installation](./guide/installation.md) — how to install it.
- [Quick start](./guide/quick-start.md) — a five-minute walkthrough.
- [Profiles](./guide/profiles.md) — pick the one that fits.
- [Rules reference](./rules/index.md) — all sixteen rules explained.
- [Accessibility](./accessibility.md) — the WCAG AAA bar and how the site
  itself dogfoods the project.

## License

<!-- lucid-lint disable-next-line unexplained-abbreviation -->

Dual-licensed under MIT or Apache-2.0, at your option.
