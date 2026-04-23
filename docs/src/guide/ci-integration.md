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

> Note: RDJSON adapter is not shipped. For native code-review surfacing, prefer the [GitHub Code Scanning](#github-code-scanning-sarif) workflow below.

## GitHub Code Scanning (SARIF)

`--format=sarif` emits a SARIF v2.1.0 log that GitHub's Code Scanning ingests directly: each diagnostic becomes a code-scanning alert annotated on the PR diff.

```yaml
name: Lucid lint (code scanning)

on:
  pull_request:
    paths: ['**/*.md']
  push:
    branches: [main]
    paths: ['**/*.md']

permissions:
  security-events: write
  contents: read

jobs:
  lucid-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install lucid-lint
      - name: Run lucid-lint and emit SARIF
        run: |
          lucid-lint check \
            --profile=public \
            --format=sarif \
            --fail-on-warning=false \
            docs/ README.md > lucid-lint.sarif
      - name: Upload SARIF to Code Scanning
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: lucid-lint.sarif
          category: lucid-lint
```

Notes:

- `--fail-on-warning=false` lets the upload step always run; rely on Code Scanning's own gating in the PR UI rather than the lint exit code.
- Each rule appears once in `runs[0].tool.driver.rules` with its category, default severity, default scoring weight, and a `helpUri` pointing at the per-rule mdBook page.
- Per-result `properties.weight` and `properties.section` carry the scoring weight and the heading the diagnostic was found under.

## Exit code control

To avoid failing CI on warnings (e.g., during a gradual adoption phase), you can invert the default:

```bash
lucid-lint check --fail-on-warning=false docs/
```

This always returns 0 except on runtime error.

## Gating on score

You can also gate the build on the aggregate
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
| Catch newly introduced warnings (default behaviour) | default |
| Tolerate individual warnings but fail on drift | `--fail-on-warning=false --min-score=85` |
| Fail on both spikes and drift | default + `--min-score=85` |

A gated run that fails — lucid-lint prints its usual summary, then
the shell surfaces the non-zero exit code:

![Terminal capture: a lucid-lint run on examples/sample.md with --min-score=85, which produces three warnings, two info diagnostics, a score of 45/100, and an "exit: 1" line written by the trailing echo command](../assets/tty/score-fail.gif)

```text
$ lucid-lint check --min-score=85 examples/sample.md
…
score: 45/100
       structure    █▎░░░  5/20
       rhythm       █████  20/20
       lexicon      █▎░░░  5/20
       syntax       ██▌░░  10/20
       readability  █▎░░░  5/20
$ echo "exit: $?"
exit: 1
```
