# Quick start

This page walks through linting your first document.

## Lint a single file

```bash
lucid-lint check README.md
```

Output:

```text
warning <path>/README.md:14:1 Sentence is 27 words long (maximum 22). Consider splitting it into shorter sentences. [structure.sentence-too-long]

summary: 1 warnings.
→ run 'lucid-lint explain <rule-id>' — seen here: structure.sentence-too-long
────────────────────────────────────────────────────────────
score: 88/100
       structure    ██▏░░  8/20
       rhythm       █████  20/20
       lexicon      █████  20/20
       syntax       █████  20/20
       readability  █████  20/20
```

The trailing block is the [scoring](./scoring.md) summary — a global
`X / 100` score followed by the full per-category breakdown.

## Lint several files

```bash
lucid-lint check docs/*.md CHANGELOG.md
```

## Lint a directory

```bash
lucid-lint check docs/
```

All files with `.md`, `.markdown`, or `.txt` extensions will be processed.

## Use stdin

```bash
echo "This is a test sentence." | lucid-lint check -
```

## Pipe from Pandoc

For formats that `lucid-lint` does not parse natively yet:

```bash
pandoc report.docx -t markdown | lucid-lint check -
```

## Choose a profile

```bash
# Strictest: Easy-to-Read
lucid-lint check --profile=falc docs/

# Looser: developer documentation
lucid-lint check --profile=dev-doc docs/
```

See [Profiles](./profiles.md) for details.

## Change the output format

```bash
# JSON for CI
lucid-lint check --format=json docs/
```

See [CI integration](./ci-integration.md) for CI recipes.

## Exit codes

| Code | Meaning |
|---|---|
| 0 | No issues (or only `info`) and score above `--min-score` (if set) |
| 1 | Warnings found **or** score below `--min-score` |
| 2 | Runtime error (invalid args, unreadable file) |

The two gates stack. See [CI integration](./ci-integration.md#gating-on-score) for
combination recipes.
