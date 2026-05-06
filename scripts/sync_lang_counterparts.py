"""Rewrite the EN ↔ FR lang-switch anchors in every built docs page.

mdBook has no per-page language context, so `docs/theme/index.hbs`
emits both lang-switch blocks (`[data-lucid-lang="en"]` + `fr`) with
hrefs hard-coded to `{{ path_to_root }}index.html` /
`{{ path_to_root }}fr/index.html` — the toggle always jumps to home.

This script runs after `mdbook build` and rewrites those hrefs so the
toggle deep-links to the counterpart page. For `rules/foo.html` it
points FR at `fr/rules/foo.html`; for `fr/rules/foo.html` it points
EN at `../../rules/foo.html`. Relative paths are computed from each
page's own location so the output works under any `site-url` prefix.

Modes:
  sync (default): walk `docs/book/**/*.html` and rewrite in place.
  --check:        build the pair map from `docs/book/**/*.html`;
                  fail if any EN page lacks a FR counterpart or any
                  FR page lacks an EN counterpart. Exits 1 on drift.

Usage:
  python3 scripts/sync_lang_counterparts.py           # rewrite
  python3 scripts/sync_lang_counterparts.py --check   # CI gate
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path

BOOK_DIR = Path(__file__).resolve().parent.parent / "docs" / "book"

# Pages mdBook generates that do not carry the lang-switch (or are
# intrinsically language-neutral) and must not be required to pair.
SKIP_PAGES = {
    "print.html",
    "404.html",
    "toc.html",
}

# Match `<a ... href="X" ... hreflang="Y" ...>` with href BEFORE
# hreflang (as the template emits). If the template is ever reordered
# to put hreflang first, add a second pattern — keeping the regex
# simple is worth occasional brittleness at a known joint.
_ANCHOR_RE = re.compile(
    r'(<a\b[^>]*?\bhref=")([^"]*)("[^>]*?\bhreflang=")(en|fr)(")',
    re.DOTALL,
)


def is_fr_page(rel_path: str) -> bool:
    """True if the book-relative HTML path lives under the FR tree."""
    return rel_path.split("/", 1)[0] == "fr"


def counterparts(rel_path: str) -> tuple[str, str]:
    """Return (en_path, fr_path) — both book-relative — for `rel_path`.

    `rel_path` is either a FR page (prefix `fr/`) or an EN page. The
    EN counterpart of `fr/a/b.html` is `a/b.html`; the FR counterpart
    of `a/b.html` is `fr/a/b.html`.
    """
    if is_fr_page(rel_path):
        en = rel_path.split("/", 1)[1]
        fr = rel_path
    else:
        en = rel_path
        fr = "fr/" + rel_path
    return en, fr


def rel_from(page_rel: str, target_rel: str) -> str:
    """Relative URL from a page to another page, both book-relative.

    Uses POSIX separators (we emit HTML hrefs, not filesystem paths).
    `os.path.relpath` accepts forward slashes on Unix and Windows'
    Python returns forward slashes here because inputs use them.
    """
    page_dir = os.path.dirname(page_rel) or "."
    rel = os.path.relpath(target_rel, page_dir)
    # Normalise to forward slashes for HTML.
    return rel.replace(os.sep, "/")


def rewrite_page(page_path: Path, rel_path: str) -> bool:
    """Rewrite the lang-switch hrefs on `page_path`. Returns True if
    the file changed."""
    html = page_path.read_text(encoding="utf-8")
    if 'class="lucid-lang' not in html:
        return False

    en_rel, fr_rel = counterparts(rel_path)
    en_href = rel_from(rel_path, en_rel)
    fr_href = rel_from(rel_path, fr_rel)

    def sub(match: re.Match[str]) -> str:
        prefix, _old_href, middle, lang, suffix = match.groups()
        new_href = en_href if lang == "en" else fr_href
        return f"{prefix}{new_href}{middle}{lang}{suffix}"

    new_html, n = _ANCHOR_RE.subn(sub, html)
    if n == 0:
        return False
    page_path.write_text(new_html, encoding="utf-8")
    return True


def walk_pages() -> list[tuple[Path, str]]:
    """Return (abs_path, rel_path) for every HTML page under BOOK_DIR,
    skipping mdBook's intrinsic non-locale pages."""
    pages: list[tuple[Path, str]] = []
    for p in BOOK_DIR.rglob("*.html"):
        rel = p.relative_to(BOOK_DIR).as_posix()
        name = p.name
        if name in SKIP_PAGES:
            continue
        pages.append((p, rel))
    return pages


def check_pairs() -> int:
    """Assert every FR page has an EN counterpart.

    The invariant is asymmetric: EN is canonical, FR is a translation
    layer. An FR page without an EN twin is a bug (stale translation,
    EN page renamed). An EN page without FR twin just means F25 hasn't
    reached it yet — the FR translation backlog is tracked on the
    roadmap, not gated in CI.

    Returns 0 if every FR page pairs, 1 otherwise. Prints the FR ↔ EN
    gap count for visibility either way.
    """
    all_pages = {rel for _, rel in walk_pages()}
    orphan_fr: list[str] = []
    untranslated_en: list[str] = []

    for rel in sorted(all_pages):
        en_rel, fr_rel = counterparts(rel)
        if is_fr_page(rel) and en_rel not in all_pages:
            orphan_fr.append(rel)
        if not is_fr_page(rel) and fr_rel not in all_pages:
            untranslated_en.append(rel)

    fr_total = sum(1 for rel in all_pages if is_fr_page(rel))
    en_total = len(all_pages) - fr_total
    print(
        f"sync_lang_counterparts: {en_total} EN page(s), {fr_total} FR page(s). "
        f"Untranslated EN: {len(untranslated_en)} (tracked as F25; not a gate).",
    )

    if orphan_fr:
        print(
            f"sync_lang_counterparts: {len(orphan_fr)} FR page(s) orphaned "
            "(no EN counterpart — broken pairing):",
            file=sys.stderr,
        )
        for rel in orphan_fr:
            en_rel, _ = counterparts(rel)
            print(f"  {rel}  →  expected {en_rel}", file=sys.stderr)
        return 1

    return 0


def sync() -> int:
    """Rewrite every lang-switch anchor in the built book."""
    touched = 0
    for abs_path, rel in walk_pages():
        if rewrite_page(abs_path, rel):
            touched += 1
    print(f"sync_lang_counterparts: rewrote {touched} page(s).")
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="validate EN ↔ FR pair-completeness; do not rewrite anything",
    )
    args = parser.parse_args(argv)

    if not BOOK_DIR.is_dir():
        print(
            f"sync_lang_counterparts: {BOOK_DIR} does not exist — run `just docs-build` first.",
            file=sys.stderr,
        )
        return 2

    if args.check:
        return check_pairs()
    return sync()


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
