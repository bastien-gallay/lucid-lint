# Profiles

A profile is a preset bundle of rule thresholds tuned for a specific audience.

## Available profiles

### `dev-doc`

For technical documentation, API references, ADRs, and developer-facing content.

Thresholds are loose: technical readers have higher tolerance for long sentences, nominalizations, and domain-specific jargon.

### `public` (default)

For general-audience content: marketing pages, product descriptions, blog posts.

Thresholds are moderate. Plain-language guidelines apply.

### `falc`

For content that follows the *Facile À Lire et à Comprendre* / Easy-to-Read European standard.

Thresholds are strict: short sentences, simple vocabulary, no passive voice, no undefined acronyms.

## Choosing a profile

Start with the profile that matches the intent of the content. Override specific rules if needed via `lucid-lint.toml`.

## Threshold comparison

See the [rule reference](../rules/index.md) for exact thresholds per rule and per profile.

The overall pattern is:

- `dev-doc`: 30 words per sentence, 4 commas, 7 sentences per paragraph
- `public`: 22 words per sentence, 3 commas, 5 sentences per paragraph
- `falc`: 15 words per sentence, 2 commas, 3 sentences per paragraph

The same file linted three times under `dev-doc`, `public`, and
`falc` in turn — the score drops as the profile tightens:

![Terminal capture: three sequential lucid-lint runs against examples/sample.md under the dev-doc, public, and falc profiles. The dev-doc pass surfaces a handful of diagnostics and a mid-range score; public tightens and more issues appear; falc flags the most and the score drops the furthest](../assets/tty/profiles.gif)

## Overriding a profile

Any per-rule threshold set in `lucid-lint.toml` takes precedence over the profile preset.

```toml
[default]
profile = "public"

[rules.sentence-too-long]
max_words = 18   # stricter than public's 22
```
