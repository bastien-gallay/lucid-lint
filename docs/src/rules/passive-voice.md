# `syntax.passive-voice`

## What it flags

Passive-voice constructions. Passive hides the agent and lengthens the sentence without adding information. Legitimate exceptions exist (unknown agent, scientific style, intentional focus on the action) — the rule flags, the author decides.

**References.** US Plain Language; Strunk & White; FALC.

## At a glance

| | |
|---|---|
| **Category** | `syntax` |
| **Default severity** | `warning` |
| **Default weight** | `2` |
| **Languages** | EN · FR (separate heuristics) |
| **Source** | [`src/rules/passive_voice.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/passive_voice.rs) |

## Detection (v0.1 heuristic)

- 🇬🇧 `be` (conjugated) + past participle `[+ by …]`. Handles regular `-ed` and the irregular-participle table.
- 🇫🇷 `être` (conjugated) + past participle `[+ par …]`, plus `se faire + infinitif`. Harder than EN because of participle agreement (gender/number) and confusion with (a) subject attribute (`il est content` vs `il est vu`) and (b) compound-tense `être` auxiliary (`elle est partie` — `passé composé`, active).

Expect ~70–80% precision. A POS-parser-based replacement is planned for a future `lucid-lint-nlp` plugin.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_per_paragraph` | `int` | 3 | 1 | 0 |
| `ignore_scientific_style` | `bool` | `false` | `false` | `false` |

## Suppression

Use inline disables on intentional passives. See [Suppressing diagnostics](../guide/suppression.md).

## References

- [Strunk & White (1999)](../references.md#strunk-white-1999)
- [Plain Language US (2011)](../references.md#plain-language-us-2011)
- [CAN-ASC-3.1:2025](../references.md#can-asc-3-1-2025)

See [References](../references.md) for the full bibliography.
