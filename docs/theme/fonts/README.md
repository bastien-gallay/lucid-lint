# Fonts

The lucid-lint docs self-host every font they use. No CDN, no Google Fonts runtime fetch, no network dependency at read time. This matters because:

1. The audience includes readers on flaky or censored networks.
2. WCAG 2.2 AAA is the declared bar; third-party font calls can block rendering.
3. Determinism: the rendered site must look identical whether online or offline.

## Files expected in this directory

Each entry is licensed under the SIL Open Font License (OFL 1.1) and is redistributable with attribution. The attribution is surfaced on the forthcoming `accessibility.md` page.

| File | Family | Role | Source |
|---|---|---|---|
| `atkinson-hyperlegible-next.woff2` | Atkinson Hyperlegible Next (variable, wght 200–800) | Body default, all-surface when `data-font="atkinson"` | Braille Institute — <https://www.brailleinstitute.org/freefont/> or Google Fonts <https://fonts.google.com/specimen/Atkinson+Hyperlegible+Next> |
| `atkinson-hyperlegible-next-italic.woff2` | Atkinson Hyperlegible Next Italic | Italic body text | same |
| `literata.woff2` | Literata (variable, wght 200–900) | Display / headings, body when `data-font="standard"` | TypeTogether — <https://github.com/googlefonts/literata> or Google Fonts <https://fonts.google.com/specimen/Literata> |
| `literata-italic.woff2` | Literata Italic | Italic headings | same |
| `commit-mono.woff2` | Commit Mono (variable, wght 400–700) | Code, inline monospace | Eigil Nikolajsen — <https://commitmono.com/> or <https://github.com/eigilnikolajsen/commit-mono> |
| `opendyslexic.woff2` | OpenDyslexic (wght 400 regular) | All surfaces when `data-font="dyslexic"` | Abelardo Gonzalez — <https://opendyslexic.org/> |
| `opendyslexic-bold.woff2` | OpenDyslexic Bold | Headings when `data-font="dyslexic"` | same |

## Fetching

A reproducible fetch script is intentionally *not* checked in — font distribution URLs change, and the OFL licence terms mean the binaries live with their upstream projects rather than being vendored here. Contributors fetch once locally:

```sh
# From the repo root
./scripts/fetch-fonts.sh        # ← to be written in the /adapt pass
```

Until the script lands, download the WOFF2 files from the sources above and drop them into this directory with the exact filenames listed. The build will fail visibly (missing font on first paint) if any file is absent — which is preferable to silently falling back to a system sans-serif.

## Licensing

All four families ship under **SIL Open Font License 1.1**. Their copyright notices must be preserved verbatim in the forthcoming `docs/src/accessibility.md` page. Do not strip licence metadata from the font files; do not claim authorship.
