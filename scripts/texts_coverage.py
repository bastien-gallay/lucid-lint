#!/usr/bin/env python3
"""Generate coverage tables for the `examples/texts.yaml` referential.

Two artefacts, two audiences:

- `examples/texts.md` (committed) carries a public-facing matrix of
  **`public_ok` counts only** — the page never reveals how many
  `check_license` / `link_only` / `restricted` sources we carry, since
  those can't ship in the repo anyway.

- `examples/local/COVERAGE.md` (gitignored) carries the full map:
  `public / total` cells plus the "load-bearing local-only" list of
  (condition × language) slots where every source is non-redistributable.
  That's the target list for F84's second half (hunt for redistributable
  replacements) — it lives outside the published repo by design.

One compute pass, two render passes. No side effects beyond writing those
two files.
"""

# /// script
# requires-python = ">=3.11"
# dependencies = ["pyyaml"]
# ///

from __future__ import annotations

import argparse
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Callable

import yaml

ROOT = Path(__file__).resolve().parent.parent
TEXTS_YAML = ROOT / "examples" / "texts.yaml"
LOCAL_TEXTS_YAML = ROOT / "examples" / "local" / "texts.yaml"
TEXTS_MD = ROOT / "examples" / "texts.md"
LOCAL_COVERAGE_MD = ROOT / "examples" / "local" / "COVERAGE.md"


def _load_merged_sources() -> list[dict]:
    """Read the public referential + the gitignored local mirror if present.

    Public surfaces read `TEXTS_YAML` directly (25 public_ok entries);
    the coverage generator wants both halves so its local artefact can
    see non-redistributable sources too. See AGENTS.md #10.
    """
    sources: list[dict] = []
    for path in (TEXTS_YAML, LOCAL_TEXTS_YAML):
        if not path.exists():
            continue
        data = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
        sources.extend(data.get("sources", []))
    return sources


BEGIN_MARKER = "<!-- coverage:begin -->"
END_MARKER = "<!-- coverage:end -->"

PUBLIC_TIERS = {"public_ok"}

LANG_LABELS = {"en": "EN", "fr": "FR", "bi": "EN+FR"}


# --- Axes -------------------------------------------------------------


@dataclass(frozen=True)
class Axis:
    """One dimension of a coverage matrix.

    `keys` fixes both the set of buckets and their display order, so the
    generator is deterministic regardless of YAML ordering. `labels`
    parallels `keys` for rendering. `of` maps one source to either a
    single key or a list of keys (multi-valued fields like `conditions`).
    """

    name: str
    keys: list[str]
    labels: list[str]
    of: Callable[[dict], str | list[str]]


def lang_slot(languages: list[str]) -> str:
    if "en" in languages and "fr" in languages:
        return "bi"
    if languages == ["en"]:
        return "en"
    if languages == ["fr"]:
        return "fr"
    return "xx"


SHAPE = Axis(
    name="Shape",
    keys=["good_example", "bad_example", "before_after", "mixed_unaligned", "neutral"],
    labels=["good", "bad", "before/after", "mixed", "neutral"],
    of=lambda s: s.get("polarity", ""),
)

LANG = Axis(
    name="Language",
    keys=["en", "fr", "bi"],
    labels=[LANG_LABELS[k] for k in ["en", "fr", "bi"]],
    of=lambda s: lang_slot(s.get("languages", [])),
)

CONDITION = Axis(
    name="Condition",
    keys=[
        "general",
        "dyslexia",
        "dyscalculia",
        "aphasia",
        "adhd",
        "non-native",
        "a11y-markup",
    ],
    labels=[
        "`general`",
        "`dyslexia`",
        "`dyscalculia`",
        "`aphasia`",
        "`adhd`",
        "`non-native`",
        "`a11y-markup`",
    ],
    of=lambda s: s.get("conditions", []),
)

TYPE = Axis(
    name="Type",
    keys=[
        "gov_guide",
        "style_guide",
        "standard",
        "research_paper",
        "corpus",
        "dataset",
        "tool_fixtures",
        "sample_collection",
        "textbook",
        "blog",
    ],
    labels=[
        "`gov_guide`",
        "`style_guide`",
        "`standard`",
        "`research_paper`",
        "`corpus`",
        "`dataset`",
        "`tool_fixtures`",
        "`sample_collection`",
        "`textbook`",
        "`blog`",
    ],
    of=lambda s: s.get("type", ""),
)


# --- Compute ----------------------------------------------------------


@dataclass(frozen=True)
class Cell:
    public: int = 0
    total: int = 0


def _as_list(x: str | list[str]) -> list[str]:
    return x if isinstance(x, list) else [x]


def count_matrix(sources: list[dict], row: Axis, col: Axis) -> dict[tuple[str, str], Cell]:
    public: dict[tuple[str, str], int] = defaultdict(int)
    total: dict[tuple[str, str], int] = defaultdict(int)
    for src in sources:
        is_public = src.get("redistribution") in PUBLIC_TIERS
        for r in _as_list(row.of(src)):
            if r not in row.keys:
                continue
            for c in _as_list(col.of(src)):
                if c not in col.keys:
                    continue
                total[(r, c)] += 1
                if is_public:
                    public[(r, c)] += 1
    return {
        (r, c): Cell(public=public[(r, c)], total=total[(r, c)]) for r in row.keys for c in col.keys
    }


def load_bearing_slots(
    sources: list[dict],
) -> dict[tuple[str, str], list[dict]]:
    """(condition × language) slots where every source is non-redistributable.

    Returns the list of offending sources per slot. Slots with at least
    one `public_ok` source are excluded — the goal is to surface places
    we actually depend on local-only data.
    """
    buckets: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for src in sources:
        slot = lang_slot(src.get("languages", []))
        if slot == "xx":
            continue
        for cond in src.get("conditions", []):
            if cond not in CONDITION.keys:
                continue
            buckets[(cond, slot)].append(src)
    return {
        slot: entries
        for slot, entries in buckets.items()
        if entries and all(s.get("redistribution") not in PUBLIC_TIERS for s in entries)
    }


# --- Render -----------------------------------------------------------


def _matrix_md(
    cells: dict[tuple[str, str], Cell],
    row: Axis,
    col: Axis,
    cell_fmt: Callable[[Cell], str],
) -> str:
    header = "| " + " | ".join([row.name, *col.labels]) + " |"
    sep = "|" + "|".join(["---"] * (len(col.keys) + 1)) + "|"
    lines = [header, sep]
    for r, r_label in zip(row.keys, row.labels):
        cs = [cell_fmt(cells[(r, c)]) for c in col.keys]
        lines.append("| " + " | ".join([r_label, *cs]) + " |")
    return "\n".join(lines)


def _public_cell(cell: Cell) -> str:
    return str(cell.public) if cell.public else "—"


def _full_cell(cell: Cell) -> str:
    if cell.total == 0:
        return "—"
    if cell.public == 0:
        return f"0 / {cell.total} ⚠"
    return f"{cell.public} / {cell.total}"


def render_public_section(sources: list[dict]) -> str:
    """Counts only `public_ok` sources — no totals, no names."""
    public_total = sum(1 for s in sources if s.get("redistribution") in PUBLIC_TIERS)
    grand_total = len(sources)
    shape_lang = count_matrix(sources, SHAPE, LANG)
    cond_lang = count_matrix(sources, CONDITION, LANG)
    type_lang = count_matrix(sources, TYPE, LANG)
    return "\n".join(
        [
            BEGIN_MARKER,
            "<!-- generated by scripts/texts_coverage.py — do not edit by hand -->",
            "",
            "## 📏 Coverage snapshot / Couverture",
            "",
            f"Auto-generated from [`texts.yaml`](./texts.yaml). Cells count"
            f" **`public_ok` sources only** — the {public_total} of"
            f" {grand_total} entries safe to commit under `examples/public/`."
            " A `—` cell means zero redistributable sources for that slot."
            " The full map (including non-redistributable slots and the"
            " load-bearing target list for sourcing work) lives at"
            " `examples/local/COVERAGE.md`, which is gitignored by design.",
            "",
            "### Shape × language",
            "",
            _matrix_md(shape_lang, SHAPE, LANG, _public_cell),
            "",
            "### Condition × language",
            "",
            _matrix_md(cond_lang, CONDITION, LANG, _public_cell),
            "",
            "### Type × language",
            "",
            _matrix_md(type_lang, TYPE, LANG, _public_cell),
            "",
            END_MARKER,
        ]
    )


def render_full_doc(sources: list[dict]) -> str:
    """Full map with `public / total` cells + load-bearing slot list."""
    total = len(sources)
    public = sum(1 for s in sources if s.get("redistribution") in PUBLIC_TIERS)
    shape_lang = count_matrix(sources, SHAPE, LANG)
    cond_lang = count_matrix(sources, CONDITION, LANG)
    type_lang = count_matrix(sources, TYPE, LANG)
    slots = load_bearing_slots(sources)

    gap_lines = [
        "| Condition | Lang | Local-only sources (tier) |",
        "|---|---|---|",
    ]
    for cond in CONDITION.keys:
        for lang in LANG.keys:
            entries = slots.get((cond, lang))
            if not entries:
                continue
            items = ", ".join(f"{s['title']} (`{s['redistribution']}`)" for s in entries)
            gap_lines.append(f"| `{cond}` | {LANG_LABELS[lang]} | {items} |")
    gap = (
        "\n".join(gap_lines)
        if len(gap_lines) > 2
        else (
            "_No load-bearing local-only slots — every (condition × language)"
            " slot with at least one source also has a redistributable one._"
        )
    )

    return "\n".join(
        [
            "# Coverage map — full (local only, do not commit)",
            "",
            "<!-- generated by scripts/texts_coverage.py — do not edit by hand -->",
            "",
            "This file mirrors `examples/texts.md`'s public coverage snapshot"
            " with the numbers the published page deliberately hides:",
            "",
            f"- Total sources in `texts.yaml`: **{total}** (public_ok: **{public}**)",
            "- Cells show `public_ok / total`; `0 / N ⚠` flags a slot where"
            "  every source is `check_license` / `link_only` / `restricted`.",
            "- The load-bearing list at the bottom is the target set for F84"
            "  part 2 — the sources we should hunt redistributable"
            "  replacements for.",
            "",
            "`examples/local/` is gitignored; this document stays off GitHub on purpose.",
            "",
            "## Shape × language",
            "",
            _matrix_md(shape_lang, SHAPE, LANG, _full_cell),
            "",
            "## Condition × language",
            "",
            _matrix_md(cond_lang, CONDITION, LANG, _full_cell),
            "",
            "## Type × language",
            "",
            _matrix_md(type_lang, TYPE, LANG, _full_cell),
            "",
            "## Load-bearing local-only slots",
            "",
            "(condition × language) slots where every source is"
            " non-redistributable — F84 part 2 target list.",
            "",
            gap,
            "",
        ]
    )


# --- Splice / write ---------------------------------------------------


def splice(old: str, section: str) -> str:
    """Replace the block between `<!-- coverage:begin/end -->` markers.

    Both markers MUST be present — the committed `examples/texts.md`
    carries them. Raising here beats silently no-op-ing on a typo.
    """
    if BEGIN_MARKER not in old or END_MARKER not in old:
        raise RuntimeError(f"Expected both {BEGIN_MARKER} and {END_MARKER} in examples/texts.md")
    before, _, rest = old.partition(BEGIN_MARKER)
    _, _, after = rest.partition(END_MARKER)
    tail = "\n\n" + after.lstrip() if after.strip() else after
    return f"{before}{section}{tail}"


def _write_if_changed(path: Path, content: str, check: bool) -> bool:
    current = path.read_text(encoding="utf-8") if path.exists() else ""
    if content == current:
        return False
    if check:
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Exit non-zero if either artefact would change.",
    )
    args = parser.parse_args()

    sources = _load_merged_sources()

    public_section = render_public_section(sources)
    public_updated = splice(TEXTS_MD.read_text(encoding="utf-8"), public_section)
    public_changed = _write_if_changed(TEXTS_MD, public_updated, args.check)

    full_doc = render_full_doc(sources)
    full_changed = _write_if_changed(LOCAL_COVERAGE_MD, full_doc, args.check)

    if args.check:
        if public_changed or full_changed:
            stale = [
                p
                for p, c in [
                    (TEXTS_MD, public_changed),
                    (LOCAL_COVERAGE_MD, full_changed),
                ]
                if c
            ]
            names = ", ".join(str(p.relative_to(ROOT)) for p in stale)
            sys.stderr.write(f"Stale: {names}. Run `just texts-coverage`.\n")
            return 1
        return 0

    for path, changed in (
        (TEXTS_MD, public_changed),
        (LOCAL_COVERAGE_MD, full_changed),
    ):
        rel = path.relative_to(ROOT)
        print(f"{'Updated' if changed else 'Up to date'}: {rel}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
