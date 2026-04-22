# `examples/local/` — your local-only working samples

This folder is **not** tracked by git (see [`.gitignore`](../../.gitignore)).
Use it for text samples that should stay on your own machine while you
experiment, benchmark, or debug.

## When to put something here instead of `../public/`

Put a sample here whenever any of the following is true for its source:

- The source is listed in [`../texts.yaml`](../texts.yaml) with
  `redistribution: link_only` or `redistribution: restricted`.
- The source is listed as `redistribution: check_license` and you
  haven't yet verified the licence of the specific item.
- You obtained the material under a data-use agreement, research-access
  form, or any terms that don't permit third-party redistribution.
- You aren't sure where the material came from or under what terms.

When in doubt, put it here.

## Suggested layout

Mirror the structure of [`../public/`](../public/) so your tooling and
benchmarks can target either folder the same way:

```
examples/local/
├── README.md         (this file)
├── en/
│   ├── good/
│   ├── bad/
│   └── neutral/
└── fr/
    ├── good/
    ├── bad/
    └── neutral/
```

## If you find a file here that could move to `../public/`

1. Verify the specific item's licence against
   [`../texts.yaml`](../texts.yaml).
2. Move the file to the matching path under `../public/`.
3. Add a row to [`../public/SOURCES.md`](../public/SOURCES.md) with
   the source URL and licence.
