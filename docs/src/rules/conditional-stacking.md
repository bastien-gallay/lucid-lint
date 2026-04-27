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

Three conditionals, colour-matched across the rewrite — position already pairs them, the tint just confirms each branch carries through. `lucid-lint` reports; the rewrite is always yours.

### English

**Before** (flagged):

> <span class="lucid-idea" data-idea="1">If we ship,</span> <span class="lucid-idea" data-idea="2">when the build passes,</span> <span class="lucid-idea" data-idea="3">unless the gate fails,</span> we deploy.

What `lucid-lint check --profile public` reports:

```text
warning input.md:1:1 Sentence stacks 3 conditional clauses (maximum 2). Split the conditions across separate sentences or convert them to a bullet list. [syntax.conditional-stacking]
```

**After** (your rewrite):

> We deploy when all three checks hold:
>
> - <span class="lucid-idea" data-idea="1">the ship command ran,</span>
> - <span class="lucid-idea" data-idea="2">the build passes,</span>
> - <span class="lucid-idea" data-idea="3">the gate does not fail.</span>

### French

**Before** (flagged):

> Si nous expédions, quand le test passe, à moins que la barrière échoue, nous déployons.

Three conditional connectors (`si`, `quand`, `à moins que`). French rewrite to come with the FR translation pass.

## Known false positives

The English list mixes pure conditionals with temporal conjunctions (`when`, `while`) that often introduce conditional-like sub-clauses. Pure-temporal usages may produce a false positive on long sentences. Use [`disable-next-line`](../guide/suppression.md) when the temporal reading is unambiguous.

## Suppression

See [Suppressing diagnostics](../guide/suppression.md).

## See also

- [`syntax.nested-negation`](./nested-negation.md)
- [`structure.deep-subordination`](./deep-subordination.md)
- [Conditions](../guide/conditions.md)

## References

- [Johnson-Laird & Byrne (1991)](../references.md#johnson-laird-byrne-1991)
- [Evans & Over (2004)](../references.md#evans-over-2004)
- [Gibson (1998)](../references.md#gibson-1998)

See [References](../references.md) for the full bibliography.
