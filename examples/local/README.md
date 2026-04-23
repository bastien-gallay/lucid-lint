# `examples/local/` — local scratch space

This folder is a local-only working directory. Its contents are **not**
tracked by git (see [`.gitignore`](../../.gitignore)); only this README
stays in version control so the folder itself exists after a fresh
clone.

Use it for whatever doesn't belong in the committed fixture set under
[`../public/`](../public/): drafts, one-off experiments, benchmark
runs, ad-hoc notes. Anything here stays on your machine.

## Layout suggestion

Mirror the structure of [`../public/`](../public/) so tools and
benchmarks can target either folder the same way:

```
examples/local/
├── README.md
├── en/
│   ├── good/
│   ├── bad/
│   └── neutral/
└── fr/
    ├── good/
    ├── bad/
    └── neutral/
```

Nothing here is authoritative. If a file becomes broadly useful and
its provenance is clear, consider moving it into `../public/` with
proper attribution in [`../public/SOURCES.md`](../public/SOURCES.md).
