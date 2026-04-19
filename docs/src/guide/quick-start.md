# Quick start

This page walks through linting your first document.

## Lint a single file

```bash
lucid-lint check README.md
```

Output:

```
warning <path>/README.md:14:1 Sentence is 27 words long (maximum 22). Consider splitting it into shorter sentences.
  rule: sentence-too-long

Summary: 1 warnings.
```

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
| 0 | No issues, or only `info` |
| 1 | Warnings found |
| 2 | Runtime error (invalid args, unreadable file) |
