# Configuration

`lucid-lint` is configured via a `lucid-lint.toml` file at the project root (optional) and CLI flags (overrides the file).

## File layout

```toml
# lucid-lint.toml

[default]
profile = "public"

[rules.sentence-too-long]
max_words = 22

[rules.passive-voice]
max_per_paragraph = 2
```

## Sections

### `[default]`

Top-level defaults applied to the whole run.

| Field | Type | Default | Description |
|---|---|---|---|
| `profile` | string | `"public"` | One of `dev-doc`, `public`, `falc` |
| `conditions` | array of strings | `[]` | Active condition tags (v0.2+). See [Conditions](./conditions.md). |

### `[rules.<rule-id>]`

Per-rule configuration. The fields available depend on the rule. See the rule pages in [Rules reference](../rules/index.md).

### `[scoring]` (v0.2+)

Tunables for the [hybrid scoring model](./scoring.md). All fields are
optional; missing fields fall back to the shipped defaults
(`category_max = 20`, `category_cap = 15`).

```toml
[scoring]
category_max = 20
category_cap = 15

[scoring.weights]
sentence-too-long = 3
weasel-words      = 2
```

The `[scoring.weights]` sub-table is keyed by rule id. Unknown ids are
ignored, so removing a rule in a future version does not break older
configs.

## Precedence

From lowest to highest:

1. Profile preset (e.g., `public`)
2. `lucid-lint.toml` overrides
3. CLI flags

An unset CLI flag defers to the TOML value; an unset TOML field defers to the profile preset.

## Discovery

`lucid-lint` walks up from the current working directory to the first `lucid-lint.toml` it finds, stopping at the nearest `.git` repo boundary. Passing `--config <path>` skips auto-discovery and loads the given file directly; a missing explicit path is an error, but a missing auto-discovered file is not.

## Per-rule overrides (v0.2+)

At time of writing, only `readability-score` honours its TOML sub-table (the `formula` field, shipped with F11 / F77). Extending TOML-driven config to the other rules lands rule-by-rule as each `Config` gains a `Deserialize` impl. Until then, `[rules.<id>]` tables for other rules parse without error but have no runtime effect.

```toml
[rules.readability-score]
formula = "kandel-moles"  # or "flesch-kincaid", "auto"
```
