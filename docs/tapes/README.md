# Terminal captures — VHS tapes

This directory holds [VHS](https://github.com/charmbracelet/vhs) tape
scripts that render the terminal screenshots embedded in the README
and the mdBook documentation.

`shared.tape` is sourced by every other tape. It defines the canvas,
theme (Lucid dark), font (Commit Mono), and typing cadence so every
asset looks like it came from the same machine.

**Paths are relative to the process CWD**, not to the tape file — so
always run VHS from the repo root (`vhs docs/tapes/hero.tape`), and
reference `shared.tape` as `Source docs/tapes/shared.tape` inside
each tape. The `just tapes` recipe already runs from the repo root.

## Prerequisites

```bash
brew install vhs          # macOS
go install github.com/charmbracelet/vhs@latest
```

VHS drives a headless terminal and captures the frames as the tape
plays. The `lucid-lint` binary must be on `PATH` — build once with
`cargo install --path .` or `just install-local`.

## Rendering

```bash
# One tape
vhs docs/tapes/hero.tape

# All tapes (see Justfile recipe `tapes`)
just tapes
```

Each tape writes to `docs/src/assets/tty/<name>.{gif,webm,svg}` as
declared by its `Output` line. Assets live under `docs/src/` so mdBook
copies them into the built site, and GitHub can render them in the
README without a clone — one path serves both surfaces.

## Tape list

| Tape               | Output                            | Purpose                                                         |
| ------------------ | --------------------------------- | --------------------------------------------------------------- |
| `hero.tape`        | `docs/src/assets/tty/hero.gif`        | README hero shot — one file, score, hint line                   |
| `score-clean.tape` | `docs/src/assets/tty/score-clean.gif` | 100/100 with banner — peak-end brand moment                    |
| `score-fail.tape`  | `docs/src/assets/tty/score-fail.gif`  | Warnings + `--min-score` gate, trailing `echo $?` shows exit 1 |
| `explain.tape`     | `docs/src/assets/tty/explain.gif`     | `lucid-lint explain structure.sentence-too-long` walkthrough    |
| `profiles.tape`    | `docs/src/assets/tty/profiles.gif`    | Same file under `dev-doc` / `public` / `falc`, score drops     |

## Fixtures

- `docs/tapes/fixtures/clean.md` — heading-only file used by
  `score-clean.tape` to trigger the clean-state path (no
  diagnostics → banner + 100/100). Trivial on purpose; the tape is
  demonstrating the output, not the fixture.
- `examples/sample.md` — the project's public demo fixture. Re-used
  by `hero.tape`, `score-fail.tape`, and `profiles.tape` so every
  capture tells a consistent story about the same document.

## Accessibility

- Every image embedded in docs must carry a descriptive `alt` text.
- Every animated asset must be accompanied by a plain code-block
  transcript of the same output. The code block is the accessible
  ground truth; the animation is a visual aid.
- Prefer SVG (static) or WebM (animated, with controls) over looping
  GIFs wherever mdBook can render them.

See `.personal/research/terminal-screenshots.md` for the planning
notes that drove this setup.
