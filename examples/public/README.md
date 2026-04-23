# `examples/public/` — shareable text samples

This folder holds text samples that ship with the repo: fixtures for docs,
snapshots, and benchmarks that anyone cloning `lucid-lint` should get.

## What belongs here

Only material whose licence allows redistribution in this repository.
Concretely: sources tagged `redistribution: public_ok` in
[`../texts.yaml`](../texts.yaml), and individually-verified items from
`check_license` sources.

Every file added here must:

1. Come from a source listed in [`../texts.yaml`](../texts.yaml) with
   `redistribution: public_ok`, **or** be an item explicitly verified as
   reusable from a `check_license` source.
2. Carry attribution — either as a header comment at the top of the file
   or via a neighbour `SOURCES.md` (one line per file: filename, source
   URL, licence).
3. Respect share-alike terms if the upstream licence is SA (CC-BY-SA,
   LGPL, etc.).

## Suggested layout

```
examples/public/
├── README.md         (this file)
├── SOURCES.md        (per-file attribution)
├── en/
│   ├── good/         (good_example / before_after "after" side)
│   ├── bad/          (bad_example / before_after "before" side)
│   └── neutral/
└── fr/
    ├── good/
    ├── bad/
    └── neutral/
```

Bilingual-parallel pairs (EC *How to write clearly*, Inclusion Europe,
Canada.ca) can live under `en/` and `fr/` with matching filenames so the
parallel alignment stays obvious.

## What does *not* belong here

Anything without a clear reuse grant. When in doubt, don't commit it.
