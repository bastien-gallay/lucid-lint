#!/usr/bin/env python3
"""Tests for `scripts/texts_coverage.py`.

Stdlib `unittest` — zero dependencies beyond what `texts_coverage` itself
needs, so this runs under the same `uv run` invocation. Run directly:

    uv run scripts/test_texts_coverage.py
    just texts-coverage-test
"""

# /// script
# requires-python = ">=3.11"
# dependencies = ["pyyaml"]
# ///

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import texts_coverage as tc  # noqa: E402


def _src(**overrides) -> dict:
    """Build a minimal source row for tests."""
    base = {
        "url": "https://example.com",
        "title": "Example",
        "polarity": "good_example",
        "type": "gov_guide",
        "languages": ["en"],
        "conditions": ["general"],
        "redistribution": "public_ok",
    }
    base.update(overrides)
    return base


class LangSlot(unittest.TestCase):
    def test_en_only(self):
        self.assertEqual(tc.lang_slot(["en"]), "en")

    def test_fr_only(self):
        self.assertEqual(tc.lang_slot(["fr"]), "fr")

    def test_bilingual(self):
        self.assertEqual(tc.lang_slot(["en", "fr"]), "bi")
        self.assertEqual(tc.lang_slot(["fr", "en"]), "bi")

    def test_unknown(self):
        self.assertEqual(tc.lang_slot([]), "xx")
        self.assertEqual(tc.lang_slot(["de"]), "xx")


class CountMatrix(unittest.TestCase):
    def test_scalar_axis_public_vs_total(self):
        sources = [
            _src(polarity="good_example", languages=["en"]),
            _src(polarity="good_example", languages=["en"], redistribution="link_only"),
            _src(polarity="bad_example", languages=["en"]),
        ]
        cells = tc.count_matrix(sources, tc.SHAPE, tc.LANG)
        self.assertEqual(cells[("good_example", "en")], tc.Cell(public=1, total=2))
        self.assertEqual(cells[("bad_example", "en")], tc.Cell(public=1, total=1))
        # Empty slot still present, zeroed.
        self.assertEqual(cells[("neutral", "fr")], tc.Cell(public=0, total=0))

    def test_multi_valued_condition_axis(self):
        # One source with two conditions increments BOTH condition rows.
        sources = [_src(conditions=["dyslexia", "non-native"], languages=["en"])]
        cells = tc.count_matrix(sources, tc.CONDITION, tc.LANG)
        self.assertEqual(cells[("dyslexia", "en")].public, 1)
        self.assertEqual(cells[("non-native", "en")].public, 1)
        self.assertEqual(cells[("general", "en")].public, 0)

    def test_unknown_keys_ignored(self):
        sources = [_src(polarity="not-a-real-shape")]
        cells = tc.count_matrix(sources, tc.SHAPE, tc.LANG)
        self.assertEqual(sum(c.total for c in cells.values()), 0)

    def test_bilingual_slots_into_bi(self):
        sources = [_src(languages=["en", "fr"])]
        cells = tc.count_matrix(sources, tc.SHAPE, tc.LANG)
        self.assertEqual(cells[("good_example", "bi")].total, 1)
        self.assertEqual(cells[("good_example", "en")].total, 0)


class LoadBearingSlots(unittest.TestCase):
    def test_all_local_only_slot_flagged(self):
        sources = [
            _src(conditions=["aphasia"], languages=["fr"], redistribution="link_only"),
            _src(conditions=["aphasia"], languages=["fr"], redistribution="check_license"),
        ]
        slots = tc.load_bearing_slots(sources)
        self.assertIn(("aphasia", "fr"), slots)
        self.assertEqual(len(slots[("aphasia", "fr")]), 2)

    def test_mixed_slot_not_flagged(self):
        # One public, one local — slot is covered.
        sources = [
            _src(conditions=["aphasia"], languages=["fr"], redistribution="public_ok"),
            _src(conditions=["aphasia"], languages=["fr"], redistribution="link_only"),
        ]
        self.assertNotIn(("aphasia", "fr"), tc.load_bearing_slots(sources))

    def test_all_public_slot_not_flagged(self):
        sources = [_src(conditions=["general"], languages=["en"])]
        self.assertEqual(tc.load_bearing_slots(sources), {})

    def test_unknown_language_skipped(self):
        sources = [_src(conditions=["aphasia"], languages=[], redistribution="link_only")]
        self.assertEqual(tc.load_bearing_slots(sources), {})


class CellFormatters(unittest.TestCase):
    def test_public_cell_zero_is_dash(self):
        self.assertEqual(tc._public_cell(tc.Cell(0, 0)), "—")
        self.assertEqual(tc._public_cell(tc.Cell(0, 3)), "—")

    def test_public_cell_positive(self):
        self.assertEqual(tc._public_cell(tc.Cell(4, 7)), "4")

    def test_full_cell_warns_on_zero_public(self):
        self.assertEqual(tc._full_cell(tc.Cell(0, 3)), "0 / 3 ⚠")

    def test_full_cell_empty(self):
        self.assertEqual(tc._full_cell(tc.Cell(0, 0)), "—")

    def test_full_cell_covered(self):
        self.assertEqual(tc._full_cell(tc.Cell(4, 7)), "4 / 7")


class Splice(unittest.TestCase):
    def test_replaces_between_markers(self):
        old = f"# Title\n\n{tc.BEGIN_MARKER}\nOLD\n{tc.END_MARKER}\n\n## Next\n"
        section = f"{tc.BEGIN_MARKER}\nNEW\n{tc.END_MARKER}"
        out = tc.splice(old, section)
        self.assertIn("NEW", out)
        self.assertNotIn("OLD", out)
        self.assertIn("## Next", out)

    def test_preserves_blank_line_before_next_heading(self):
        old = f"before\n\n{tc.BEGIN_MARKER}\nold\n{tc.END_MARKER}\n\n## Next\n"
        section = f"{tc.BEGIN_MARKER}\nnew\n{tc.END_MARKER}"
        out = tc.splice(old, section)
        self.assertIn(f"{tc.END_MARKER}\n\n## Next", out)

    def test_is_idempotent(self):
        old = f"# T\n\n{tc.BEGIN_MARKER}\nX\n{tc.END_MARKER}\n\n## Tail\n"
        section = f"{tc.BEGIN_MARKER}\nX\n{tc.END_MARKER}"
        once = tc.splice(old, section)
        twice = tc.splice(once, section)
        self.assertEqual(once, twice)

    def test_missing_markers_raises(self):
        with self.assertRaises(RuntimeError):
            tc.splice("no markers here", "section")


class RenderPublic(unittest.TestCase):
    def test_never_leaks_local_only_signal(self):
        # A source that is local-only should NOT appear by title, and no
        # cell should expose totals.
        sources = [
            _src(redistribution="public_ok", title="Public Thing"),
            _src(
                redistribution="link_only",
                title="Secret Local Thing",
                conditions=["aphasia"],
                languages=["fr"],
            ),
        ]
        out = tc.render_public_section(sources)
        self.assertNotIn("Secret Local Thing", out)
        self.assertNotIn("link_only", out)
        self.assertNotIn("check_license", out)
        self.assertNotIn("⚠", out)  # the "0 / N ⚠" warning is full-doc only
        self.assertNotIn("Load-bearing", out)
        self.assertIn(tc.BEGIN_MARKER, out)
        self.assertIn(tc.END_MARKER, out)

    def test_public_count_surfaces(self):
        sources = [_src(polarity="good_example", languages=["en"])]
        out = tc.render_public_section(sources)
        # The EN `good` cell should read `1`.
        self.assertIn("| good | 1 |", out)


class RenderFull(unittest.TestCase):
    def test_includes_load_bearing_list_and_titles(self):
        sources = [
            _src(
                redistribution="link_only",
                title="Only Local",
                conditions=["aphasia"],
                languages=["fr"],
            ),
        ]
        out = tc.render_full_doc(sources)
        self.assertIn("Only Local", out)
        self.assertIn("link_only", out)
        self.assertIn("Load-bearing", out)

    def test_no_gaps_emits_placeholder(self):
        sources = [_src()]  # public_ok only → no load-bearing slots
        out = tc.render_full_doc(sources)
        self.assertIn("No load-bearing local-only slots", out)


class Integration(unittest.TestCase):
    """Sanity check against the real YAML — catches schema drift."""

    def test_real_yaml_renders_without_error(self):
        import yaml

        data = yaml.safe_load(tc.TEXTS_YAML.read_text(encoding="utf-8"))
        sources = data.get("sources", [])
        public = tc.render_public_section(sources)
        full = tc.render_full_doc(sources)
        self.assertIn(tc.BEGIN_MARKER, public)
        self.assertIn("Shape × language", public)
        self.assertIn("Load-bearing", full)


if __name__ == "__main__":
    unittest.main()
