# CI integration

`lucid-lint` is designed for CI. It returns:

- `0` when no issues (or only `info`) are found
- `1` when warnings are found
- `2` on runtime error (invalid args, unreadable file)

## GitHub Actions

```yaml
name: Docs lint

on:
  pull_request:
    paths:
      - '**/*.md'
  push:
    branches: [main]
    paths:
      - '**/*.md'

jobs:
  lucid-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install lucid-lint
        run: cargo install lucid-lint
      - name: Lint docs
        run: lucid-lint check --profile=public docs/ README.md
```

## Pre-commit

Add to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: lucid-lint
        name: lucid-lint
        entry: lucid-lint check --profile=public
        language: system
        types: [markdown]
```

## Reviewdog

To surface diagnostics as pull request review comments:

```bash
lucid-lint check --format=json docs/ | reviewdog -f=rdjson -reporter=github-pr-review
```

> Note: RDJSON adapter is not shipped in v0.1. For now, parse the JSON output yourself or wait for the [SARIF output](../../ROADMAP.md) planned in v0.2.

## Exit code control

To avoid failing CI on warnings (e.g., during a gradual adoption phase), you can invert the default:

```bash
lucid-lint check --fail-on-warning=false docs/
```

This always returns 0 except on runtime error.

## Gating on score

As of v0.2 you can also gate the build on the aggregate
[scoring model](./scoring.md). The run exits `1` if the global score is
below the threshold, independently of the severity gate.

```yaml
jobs:
  lucid-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install lucid-lint
      - name: Lint and gate on score
        run: lucid-lint check --min-score=85 docs/ README.md
```

Both gates stack — the run fails if *either* trips. Pick the combination
that fits your adoption curve:

| Goal | Flags |
|---|---|
| Catch newly introduced warnings (v0.1 behaviour) | default |
| Tolerate individual warnings but fail on drift | `--fail-on-warning=false --min-score=85` |
| Fail on both spikes and drift | default + `--min-score=85` |
