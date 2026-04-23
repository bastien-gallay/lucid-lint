# `scripts/`

Repo-local developer tooling. Rust lives in `src/`; anything else that
orchestrates builds, docs, or fixtures sits here.

## Existing scripts

- `sync-roadmap.py` — mirrors root `ROADMAP.md` into `docs/src/roadmap.md`
  (mdBook tree), rewriting relative links. Invoked by `just docs-build`
  and `just docs-serve`.
- `sanitize-stock-css.py` — strips banned stock fonts from the mdBook
  build output. Post-`mdbook build` hook.

Both are `#!/usr/bin/env python3` stdlib-only and run via `python3
scripts/<name>.py`.

## Text-fixture pipeline (`texts_*.py`)

Pulls text sources from `examples/texts.yaml` into `examples/public/`
and `examples/local/` for use as post-0.2.0 test fixtures and
benchmarks. Three composable stages plus shared helpers:

| Script | Purpose |
|---|---|
| `texts_common.py` | YAML loader, destination resolver, slug, frontmatter writer. Imported by the other three. |
| `texts_fetch.py` | Download raw source into `<slug>/raw.<ext>` (HTTP) or `<slug>/raw/` (shallow `git clone`). Writes `source.yaml` sidecar. |
| `texts_clean.py` | Produce `clean.<ext>` next to each `raw.*` — `trafilatura` for HTML, `pypdf` for PDF, copy for plain formats. |
| `texts_convert.py` | Emit `content.md` or `before.md`+`after.md`. Appends a row per file to `examples/{public,local}/SOURCES.md`. |

### Running

Per user convention (global CLAUDE.md: "always use `uv` or `uvx`"),
these scripts declare their dependencies via
[PEP 723](https://peps.python.org/pep-0723/) inline metadata and are
invoked through `uv run`:

```bash
just texts-plan       # dry-run: show routing, no network I/O
just texts-fetch      # download raw.<ext>
just texts-clean      # produce clean.<ext>
just texts-convert    # emit content.md / before.md + after.md
just texts            # all three in order
```

For a single source:

```bash
uv run scripts/texts_fetch.py --only=plainlanguage-gov-before-and-after
uv run scripts/texts_clean.py --only=plainlanguage-gov-before-and-after
uv run scripts/texts_convert.py --only=plainlanguage-gov-before-and-after
```

### Scope

Sources in `examples/texts.yaml` with `markdownable >= 3`.
Lower-score PDFs and proprietary tools stay link-only in `texts.yaml`.

Only `public_ok` sources are committed — the scraper writes those into
`examples/public/`. Everything else is either cited and linked, or left
out of the repo. The single mapping function is
`texts_common.resolve_destination(src)`; every write guards against
escape via `assert_under_known_root(path)`.

### Output layout (per source)

```
examples/<public|local>/<lang>/<polarity>/<slug>/
  ├── source.yaml     # upstream metadata + fetch/convert timestamps
  ├── raw.<ext>       # original download (kept for future format work)
  ├── clean.<ext>     # noise-stripped native format
  └── content.md      # final Markdown — or for before_after:
      ├── before.md
      └── after.md
```

`<polarity>` folder names: `good`, `bad`, `before-after`, `mixed`,
`neutral`. `<lang>` is the primary language (`en`, `fr`, or `bi` for
bilingual-parallel sources).

### Failure handling

Network errors, HTTP 4xx/5xx, and missing `pandoc` are logged to
stdout. The scraper continues with the next source.

### Idempotency

- `fetch` skips sources whose `raw.*` already exists (override with
  `--force`).
- `clean` compares recorded sha256 of the raw; re-cleans on change.
- `convert` skips folders whose target `.md` is present (override with
  `--force`).

### Licence drift guard

If the `redistribution` value of a source changes between runs, `fetch`
refuses to overwrite the old folder and prints instructions to move it
manually. This prevents accidental promotion of still-unverified
material into `examples/public/`.
