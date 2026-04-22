# `syntax.conditional-stacking`

## What it flags

Sentences that chain multiple conditional clauses. Each `if` / `when` / `unless` / `quand` / `si` opens a branch the reader must keep on a mental stack until the outer clause resolves; two or three of them stacked in one sentence is a known load multiplier for readers with aphasia, ADHD, and anyone reading under cognitive pressure. Plain-language guidelines (FALC, plainlanguage.gov) recommend splitting conditional chains into separate sentences or a bullet list.

## At a glance

| | |
|---|---|
| **Category** | `syntax` |
| **Default severity** | `warning` |
| **Default weight** | `2` |
| **Condition tags** | `aphasia`, `adhd`, `general` |
| **Languages** | EN · FR (language-specific lists) |
| **Source** | [`src/rules/conditional_stacking.rs`](https://github.com/bastien-gallay/lucid-lint/blob/main/src/rules/conditional_stacking.rs) |

## Detection

Per sentence, count the conditional connectors and report counts above `max_conditionals`.

- **English** — sum of word-bounded matches against the language list (`if`, `unless`, `when`, `whenever`, `while`, `until`, `provided`, `assuming`, `in case`, `as long as`, `as soon as`, `even if`, `only if`).
- **French** — sum of word-bounded matches against the language list (`si`, `sauf si`, `à moins que`, `à moins de`, `quand`, `lorsque`, `lorsqu'`, `dès que`, `tant que`, `pourvu que`, `à condition que`, `à condition de`, `au cas où`, `même si`, `en cas de`) plus the elliptic clitics `s'il` / `s'ils`.

## Parameters

| Key | Type | `dev-doc` | `public` | `falc` |
|---|---|---|---|---|
| `max_conditionals` | `int` | 3 | 2 | 1 |

## Examples

**EN — triggers under `public`:**

> If we ship, when the build passes, unless the gate fails, we deploy.

Three conditional connectors. Rewrite as a sequence or a bullet list.

**FR — triggers under `public`:**

> Si nous expédions, quand le test passe, à moins que la barrière échoue, nous déployons.

Three conditional connectors (`si`, `quand`, `à moins que`).

## Known false positives

The English list mixes pure conditionals with temporal conjunctions (`when`, `while`) that often introduce conditional-like sub-clauses. Pure-temporal usages may produce a false positive on long sentences. Use [`disable-next-line`](../guide/suppression.md) when the temporal reading is unambiguous.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`syntax.nested-negation`](./nested-negation.md)
- [`structure.deep-subordination`](./deep-subordination.md)
- [Conditions](../guide/conditions.md)
