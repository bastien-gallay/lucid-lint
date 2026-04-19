# Brainstorm: inline-disable feature

| Field | Value |
| --- | --- |
| **Date** | 2026-04-19 |
| **Duration** | ~5 min |
| **Participants** | Bastien + AI Facilitator |
| **Problem shape** | Decision under constraints (pick a syntax + scope model) |

## Session plan

| # | Phase | Technique | Status |
| --- | --- | --- | --- |
| 0 | Intake | AI-seeded ideas + axes | Done |
| 1 | Decide | Scope narrowing (A/B/C pick) | Collapsed to single pick |
| 2 | Record | Action items + ROADMAP entries | Done |

## Ideas — starting point

1. HTML comment, next-line — `<!-- lucid-lint disable-next-line <rule-id> -->`
2. HTML comment, paired block — `<!-- lucid-lint-disable -->` … `<!-- lucid-lint-enable -->`
3. File-level directive — `<!-- lucid-lint disable-file <rule-id> -->`
4. Config-based ignores — `[[ignore]]` entries in `lucid-lint.toml`
5. Trailing same-line directive — `text... <!-- lucid-lint disable <rule-id> -->`

**Design axes:** scope (line/block/file/glob), rules (one/list/`*`), reason (optional/required), plain-text fallback.

**Flags raised:**

- `.txt`/stdin have no comment syntax → HTML-only design leaves them uncovered.
- README.md:138 (Apache-2.0 dual-license boilerplate) is intentional legal text; a `reason=` field would make that visible in review.
- Prior art: ESLint (next-line comments), markdownlint (HTML comments), rustc (attributes). HTML comments are the idiomatic Markdown choice — hidden in rendered output.

## Decision

**Ship A only in v0.1:** next-line HTML comment.

```markdown
<!-- lucid-lint disable-next-line sentence-too-long -->
A long intentional sentence on the following line.
```

- **Syntax:** HTML comment, Markdown inputs only.
- **Scope:** next non-blank line / next block element.
- **Rules:** single rule id per directive (multi-rule list → roadmap).
- **Reason:** optional today, not required.
- **Plain text / stdin:** out of scope for v0.1.

**Deferred to ROADMAP:**

- **F18** — Block form `<!-- lucid-lint-disable <rule-id> -->` / `<!-- lucid-lint-enable -->`.
- **F19** — Config-based ignores (`[[ignore]]` in `lucid-lint.toml`) covering `.txt` and stdin.
- **F20** — `reason="..."` field (surface in reports; config option to require it).
- **F21** — File-level directive and multi-rule lists.

## Action items

- [ ] Parser: recognize `<!-- lucid-lint disable-next-line <rule-id> -->` during Markdown block walk (`src/parser/markdown.rs`). Attach a `Directive { rule_id, target_line }` to the next block.
- [ ] Engine: filter diagnostics where `(path, line, rule_id)` matches an active directive (`src/engine.rs`).
- [ ] Types: add `Directive` struct; collect them alongside blocks in the parser output.
- [ ] Tests: unit test in parser (directive extraction), unit test in engine (diagnostic suppression), snapshot test covering README.md:138 scenario, corpus fixture in `tests/corpus/en/`.
- [ ] Dogfood: apply the directive to README.md:138 (Apache-2.0 dual-license boilerplate), confirm `just dogfood` reports 0 warnings.
- [ ] Docs: short section in `RULES.md` ("Suppressing diagnostics") documenting the v0.1 syntax and linking to ROADMAP for block/config forms.
- [ ] ROADMAP: entries F18–F21 added.

## Unknowns to resolve during implementation

- **Unknown rule id in directive.** Warn (don't silently accept) — probably a new `unknown-disable-directive` meta-diagnostic, or just emit a `warn!` log. Decide when we see the code.
- **Directive on a line that has no following block** (end of file, trailing directive). Warn and ignore.
- **Case sensitivity of rule id.** Match config convention: case-sensitive, kebab-case exact match.

---

## Session meta-analysis

- **Duration:** ~5 min (planned: 5 min) ✓
- **Techniques used:** AI-seeded intake, axis enumeration, multiple-choice narrowing.
- **Techniques skipped:** Decision matrix, Six Hats — unnecessary; user had clear preference for minimal-ship.
- **Adaptations:** Compressed from 3 steps to 1 after decisive A pick.
- **Convergence point:** Step 1 — user picked A immediately and pushed B/C/reason to roadmap.
- **What worked well:** Presenting A/B/C with orthogonal reason question let the user resolve two axes in one answer.
- **What could improve:** Could have surfaced the "unknown rule id" edge case during intake rather than deferring.
- **Recommendation for similar sessions:** For scope-narrowing decisions on a well-understood problem, skip the matrix and go straight to an A/B/C offer with flagged risks.
