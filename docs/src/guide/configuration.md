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

### `[rules.<rule-id>]`

Per-rule configuration. The fields available depend on the rule. See the rule pages in [Rules reference](../rules/index.md).

## Precedence

From lowest to highest:

1. Profile preset (e.g., `public`)
2. `lucid-lint.toml` overrides
3. CLI flags

## Discovery

`lucid-lint` looks for `lucid-lint.toml` in the current working directory. Explicit config file loading via `--config <path>` is on the roadmap for v0.2.
