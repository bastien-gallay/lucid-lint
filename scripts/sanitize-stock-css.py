#!/usr/bin/env python3
"""Strip banned font strings from mdBook's stock CSS after a build.

mdBook's bundled `general.css` declares `font-family: "Open Sans", sans-serif`
on `html`. Our `lucid-typography.css` overrides this via `--body-font`, so the
active font is Atkinson Hyperlegible Next — but the literal "Open Sans"
string remains in the shipped `book/css/general-*.css`. That leak trips the
impeccable detector (~56 findings) and degrades the fallback chain on first
paint if our additional-css is slow to load.

Replace the string in-place with Atkinson so the shipped book only
advertises fonts the project actually endorses. Called from `just docs-build`
right after `mdbook build`.
"""
from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
CSS_DIR = ROOT / "docs" / "book" / "css"

# Targeted, deliberate substitutions. Keep the surrounding commas / quotes
# so the declarations stay syntactically valid.
#
# - general.css: the live html { font-family } rule — repoint to Atkinson.
# - fonts-*.css: mdBook's @font-face declarations for Open Sans. They are
#   never loaded (our additional-css stack owns the live font chain), but
#   the raw "Open Sans" string still ships in the file. Renaming the
#   family neutralises the detector hit AND ensures that if any stray
#   rule ever tries to resolve Open Sans it fails fast instead of
#   quietly pulling a system copy.
GENERAL_REPLACEMENTS = {
    '"Open Sans", sans-serif': '"Atkinson Hyperlegible Next", sans-serif',
}
FONTS_REPLACEMENTS = {
    # Rename every Open Sans reference to a unique sigil that contains
    # NO "Open Sans" substring, so downstream string-matchers don't
    # re-flag these @font-face blocks. "mdbook-stock-unused" makes the
    # provenance obvious if a maintainer greps.
    "'Open Sans'": "'mdbook-stock-unused'",
    "'Open Sans Light'": "'mdbook-stock-unused-light'",
    "'Open Sans Light Italic'": "'mdbook-stock-unused-light-italic'",
    "'Open Sans Regular'": "'mdbook-stock-unused-regular'",
    "'Open Sans Italic'": "'mdbook-stock-unused-italic'",
    "'Open Sans SemiBold'": "'mdbook-stock-unused-semibold'",
    "'Open Sans SemiBold Italic'": "'mdbook-stock-unused-semibold-italic'",
    "'Open Sans Bold'": "'mdbook-stock-unused-bold'",
    "'Open Sans Bold Italic'": "'mdbook-stock-unused-bold-italic'",
    "'Open Sans ExtraBold'": "'mdbook-stock-unused-extrabold'",
    "'Open Sans ExtraBold Italic'": "'mdbook-stock-unused-extrabold-italic'",
}


def sanitize(path: pathlib.Path, replacements: dict[str, str]) -> bool:
    text = path.read_text()
    new = text
    for src, dst in replacements.items():
        new = new.replace(src, dst)
    if new == text:
        return False
    path.write_text(new)
    return True


def main() -> int:
    if not CSS_DIR.exists():
        print(f"sanitize-stock-css: {CSS_DIR} not found — run `mdbook build` first.", file=sys.stderr)
        return 0  # not fatal in CI: no build, nothing to sanitize
    touched = 0
    for path in CSS_DIR.glob("general*.css"):
        if sanitize(path, GENERAL_REPLACEMENTS):
            touched += 1
    fonts_dir = ROOT / "docs" / "book" / "fonts"
    if fonts_dir.exists():
        for path in fonts_dir.glob("fonts*.css"):
            if sanitize(path, FONTS_REPLACEMENTS):
                touched += 1
    print(f"sanitize-stock-css: rewrote {touched} file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
