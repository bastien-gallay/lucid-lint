#!/usr/bin/env python3
"""Mirror the root `ROADMAP.md` into `docs/src/roadmap.md`.

Keeps the mdBook site self-contained: readers browsing the published docs
see the same roadmap that lives at the repo root, without having to
follow an external link.

Link rewriting:

- Anchors (`#foo`), absolute URLs, and already-docs-relative paths
  (`./…`, `../…`) are left as-is.
- Paths under `docs/src/…` are rewritten to be docs-relative (`./…`) so
  they resolve inside the mdBook tree.
- Every other relative path is rewritten to an absolute GitHub URL on
  `main`, which `tests/rule_docs_coverage.rs::docs_links_stay_inside_docs`
  accepts (only `../../…` escapes fail that test).

Invoked automatically by `just docs-build` / `just docs-serve` so the
generated file stays in lock-step with the source.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

GITHUB_BASE = "https://github.com/bastien-gallay/lucid-lint/blob/main"
HEADER = (
    "<!-- Auto-generated from ../../ROADMAP.md by scripts/sync-roadmap.py. "
    "Edit the source, not this file. -->\n\n"
)

LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")


def rewrite_link(match: re.Match[str]) -> str:
    label, target = match.group(1), match.group(2)
    if target.startswith(("http://", "https://", "#", "./", "../")):
        return match.group(0)
    if target.startswith("docs/src/"):
        return f"[{label}](./{target[len('docs/src/') :]})"
    return f"[{label}]({GITHUB_BASE}/{target})"


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    src = repo_root / "ROADMAP.md"
    dst = repo_root / "docs" / "src" / "roadmap.md"

    body = LINK_RE.sub(rewrite_link, src.read_text(encoding="utf-8"))
    dst.write_text(HEADER + body, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
