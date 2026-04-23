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
| `exclude` | array of glob strings | `[]` | Paths to skip during directory recursion (v0.2+). See [Excluding paths](#excluding-paths-v02). |

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

## Excluding paths (v0.2+)

Large documentation repositories routinely contain generated output,
vendored text, and snapshots that would drown the linter in noise. Use
the `exclude` field in `[default]` — or the `--exclude <GLOB>` CLI flag
— to skip them at discovery time, before parsing.

```toml
[default]
exclude = [
    "vendor/**",
    "**/fixtures/**",
    "CHANGELOG.md",
]
```

Equivalently on the command line:

```bash
lucid-lint check --exclude 'vendor/**,**/fixtures/**,CHANGELOG.md' docs
```

Notes:

- **Matching.** Globs are matched against the path **relative to the
  walked root**. Passing `lucid-lint check docs` with
  `exclude = ["drafts/**"]` skips `docs/drafts/...`.
- **Prune, don't visit.** A matching directory is not descended into —
  large excluded trees cost nothing to walk.
- **Explicit files bypass.** If you pass `docs/CHANGELOG.md` directly
  on the command line, it is linted even when `CHANGELOG.md` is in the
  exclude list. If you named the path, you meant it.
- **Additive.** CLI `--exclude` and TOML `exclude` are unioned, not
  overridden. Comma-separate multiple patterns in a single flag, or
  repeat `--exclude`.

## Silencing rules globally (v0.2+)

Markdown documents support
[inline-disable directives](./suppression.md) for local silencing, but
plain text and stdin have no such escape hatch. `[[ignore]]` fills
that gap — and works uniformly across all input formats.

```toml
[[ignore]]
rule_id = "unexplained-abbreviation"

[[ignore]]
rule_id = "weasel-words"
```

Each `[[ignore]]` entry removes every diagnostic whose `rule_id`
matches, across Markdown files, plain text, and stdin. The filter is
applied after all rules have run but before scoring, so the score
reflects the post-filter view too.

Notes:

- **Global scope.** The filter is not per-file. Inline directives
  remain the recommended escape hatch for spot silencing in Markdown
  — reach for `[[ignore]]` only when a rule is genuinely noisy
  project-wide.
- **Unknown ids tolerated.** Entries referencing rules that no longer
  exist are dropped silently, so removing a rule in a future release
  does not break older configs.
- **Future fields.** A `reason = "..."` field on each entry is
  tracked as F20 — when it lands it will be surfaced in reports and
  optionally required via config.

## Per-rule overrides (v0.2+)

TOML-driven config is wired rule-by-rule as each `Config` gains a dedicated accessor. Two rules honour it today:

### `[rules.readability-score]`

```toml
[rules.readability-score]
formula = "kandel-moles"  # or "flesch-kincaid", "auto"
```

Pins the readability formula regardless of detected language. `auto` (default) preserves the F10 per-language selection.

### `[rules.unexplained-abbreviation]`

```toml
[rules.unexplained-abbreviation]
whitelist = ["WCAG", "ARIA", "ADHD", "LLM"]
```

Entries are **additive** over the profile baseline (F31). Use this to restore project-specific acronyms — accessibility standards, domain initialisms, engineering-practice terms — that the v0.2 baseline no longer ships. Each entry is silenced globally across the document, same as if it had been defined inline via `Expansion (ACRONYM)`.

### `[rules."structure.excessive-commas"]`

```toml
[rules."structure.excessive-commas"]
max_commas = 2
```

Overrides the per-sentence comma ceiling (default: 4 / 3 / 2 for `dev-doc` / `public` / `falc`). Must be a positive integer — `0` or negative values are rejected at load time. The override replaces the profile preset; it is not additive.

Tables for other rules parse without error but have no runtime effect. Extending this list is a mechanical per-rule change and will continue through the v0.2.x cycle.
