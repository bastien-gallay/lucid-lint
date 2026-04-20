# Conditions

A **condition tag** describes the cognitive condition a rule primarily targets. Conditions are *orthogonal* to profiles: a profile (`dev-doc`, `public`, `falc`) sets the strictness of the always-on rules; conditions enable additional rules tuned for a specific audience.

## The fixed ontology

| Tag | Targets |
|---|---|
| `general` | Always-on rules. The v0.2 baseline. |
| `a11y-markup` | Prose-adjacent markup signals (e.g. all-caps shouting). |
| `dyslexia` | Dyslexia-targeted signals. Source: BDA Dyslexia Style Guide. |
| `dyscalculia` | Numeric format and anchoring. Source: CDC Clear Communication Index. |
| `aphasia` | Aphasia-targeted signals. Source: FALC, plain-language guides. |
| `adhd` | Attention-fragility signals. |
| `non-native` | Non-native reader signals (vocabulary rarity, idioms). |

The set is fixed. New tags are a deliberate, versioned change.

## How filtering works

For every rule the engine evaluates:

- A rule tagged `general` is **always enabled**.
- A rule **without** `general` runs only when at least one of its tags appears in the user's active condition list.

All 17 v0.2 rules carry `general`, so the default behavior is unchanged. Future tagged rules (e.g. `lexicon.all-caps-shouting` for `a11y-markup`, `syntax.nested-negation` for `aphasia` + `adhd`) opt in via this list.

## Configuring conditions

In `lucid-lint.toml`:

```toml
[default]
profile = "falc"
conditions = ["dyslexia", "aphasia"]
```

On the command line (comma-separated, repeatable):

```bash
lucid-lint check --profile falc --conditions dyslexia,aphasia docs/
```

FALC retains its regulatory meaning. Adding `dyslexia` does not relax or rename it — it layers dyslexia-specific signals on top.

## Why tags, not parallel profiles

Three strictness levels × N conditions explodes combinatorially. Keeping the two axes orthogonal preserves the regulatory meaning of `falc` while letting users compose audience-specific overlays. See [ROADMAP](../roadmap.md) entries F71 and F72.
