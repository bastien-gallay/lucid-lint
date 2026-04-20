# Accessibility

`lucid-lint` is a cognitive-accessibility tool. The docs site you are
reading is its first proof of concept. If the site itself is not
comfortable to read for the audiences the project claims to serve, the
pitch does not hold.

This page lists the bar, the controls, and the credits.

## The bar — WCAG 2.2 Level AAA

WCAG (Web Content Accessibility Guidelines) is the international
standard for web accessibility. It defines three conformance levels;
**AAA (the strictest)** is the ceiling, not the floor.

<!-- lucid-lint disable-next-line unexplained-abbreviation -->

The stated bar for this site is WCAG 2.2 Level AAA. In practice:

- Normal body text clears a contrast ratio of **7:1** against its background.
- Large text and UI chrome clears **4.5:1**.
- Interactive targets are at least **44 × 44 px**.
- A **skip-to-content** link is the first focusable element on every page.
- The focus ring is visible and does not rely on colour alone.
- Motion respects `prefers-reduced-motion: reduce` absolutely — no
  decorative animation, no parallax, no auto-playing content.
- Keyboard navigation reaches every interactive surface in a logical order.

<!-- lucid-lint disable-next-line unexplained-abbreviation -->

Both themes (**Lucid light** and **Lucid dark**) clear AAA for body
text (14:1 and above) and inline links (7.4:1 and above).

<!-- lucid-lint disable-next-line unexplained-abbreviation -->

Where AAA is impractical — for example contrast on a third-party
embed — the exception is documented in
[`.impeccable.md`](https://github.com/bastien-gallay/lucid-lint/blob/main/.impeccable.md).

## Reading preferences

A small set of controls tunes the site to your own reading profile.
Selections persist across visits via `localStorage`.

### Font

Three choices, picked from the [Introduction page
demonstrator](./introduction.md) or from the reading-preferences popover
(on the way — see the [roadmap](./roadmap.md)).

| Option | When it helps |
|---|---|
| **Atkinson Hyperlegible Next** *(default)* | A humanist sans built by the Braille Institute for maximum character differentiation. Reads well for most readers and especially for readers with low vision or reading-speed fatigue. Every surface on the site uses it by default. |
| **Standard** | The same Atkinson for body prose, paired with Literata serif for headings — a traditional bookish pairing for readers who prefer serif display contrast. |
| **OpenDyslexic** | A typeface whose letters are weighted at the bottom to reduce swapping and rotating. Preferred by some dyslexic readers; not universally helpful. |

### Line spacing

Adjustable from **1.4 to 2.0** in 0.05 steps. The default is **1.7** — the
research range for low-fatigue reading sits between 1.6 and 1.8.

### Text size

Adjustable from **90 % to 130 %** in 5 % steps. Browser zoom is honoured in
addition.

## Keyboard shortcuts

The site inherits mdBook's keyboard map:

| Key | Action |
|---|---|
| `/` or `s` | Focus the search box |
| `←` | Previous chapter |
| `→` | Next chapter |
| `Escape` | Close the search or theme popover |
| `Tab` | Follow the focus order. The first focusable element is always the **Skip to main content** link. |

## Typography credits

Every font on the site is self-hosted under
[`docs/src/_fonts/`](https://github.com/bastien-gallay/lucid-lint/tree/main/docs/src/_fonts).
<!-- lucid-lint disable-next-line unexplained-abbreviation -->

All four ship under the **SIL Open Font License 1.1**, issued by the
Summer Institute of Linguistics.

- **Atkinson Hyperlegible Next** — Braille Institute of America. Commissioned
  for low-vision readers; designed to maximise the differentiation between
  characters that commonly get confused (`rn` vs `m`, `I` vs `l` vs `1`).
- **Literata** — TypeTogether, commissioned by Google for Google Play Books.
  A contemporary serif with generous x-height tuned for long-form reading.
- **Commit Mono** — Eigil Nikolajsen. A monospaced face designed for code
  reading, with distinctive digits and unambiguous punctuation.
- **OpenDyslexic** — Abelardo Gonzalez. A public-domain typeface for readers
  who find weighted-bottom letterforms easier to track.

## Dogfooding

The prose on this site is linted by `lucid-lint` itself at the `public`
profile, via `just dogfood`. A page cannot regress below the bar the
tool sets for its users without the build failing.

## Reporting an accessibility issue

If something on this site is harder to use than it should be, open an
issue on
[GitHub](https://github.com/bastien-gallay/lucid-lint/issues/new) with
the `accessibility` label. Reports are triaged against the v0.2
milestone unless they block a release.
