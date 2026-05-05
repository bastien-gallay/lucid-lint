"""Detect FR pages whose EN counterpart has moved past the recorded SHA.

`scripts/sync_lang_counterparts.py --check` gates *filename* parity:
every FR page must have an EN twin. This script gates *content*
parity: every FR page declares (via an HTML comment on the first
line) the EN counterpart's commit SHA at translation time, and CI
compares that stored SHA to the EN counterpart's current
last-touching commit.

A mismatch means the EN page changed after the FR was translated —
the FR is stale and either needs a re-translation pass or an
explicit acknowledgement that the EN edit was cosmetic.

Stamp shape on every FR page (except intentionally asymmetric
ones — see ``ASYMMETRIC_FR_PAGES`` below):

    <!-- en-source-sha: 563e0b8d2e1f9a4c6b8e5d3f1a2b4c6d8e0f2a4b -->

YAML front-matter would have been more idiomatic but mdBook does
not strip it — `---` renders as `<hr>` and the body lines render
as text. HTML comments pass through mdBook unchanged and are
invisible in the output. No preprocessor required.

Modes:
  default: walks `docs/src/fr/**/*.md`; reports stale + missing-stamp
           pages on stderr; exits 0 (soft, suitable for PR runs).
  --strict: same, but exits 1 on any drift or missing stamp
           (suitable for `main`-branch CI).

Usage:
  python3 scripts/check_lang_staleness.py            # report-only
  python3 scripts/check_lang_staleness.py --strict   # CI gate
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent

# Match `<!-- en-source-sha: <hex> -->` anywhere in the first 1 KB
# of the file. The stamp belongs on the first line by convention
# (clean diffs, predictable greps) but the regex is positionally
# loose so a leading title comment or BOM doesn't break detection.
_STAMP_RE = re.compile(
    r"<!--\s*en-source-sha:\s*([0-9a-fA-F]{7,40})\s*-->",
)
FR_ROOT = REPO_ROOT / "docs" / "src" / "fr"
EN_ROOT = REPO_ROOT / "docs" / "src"

# FR pages whose EN counterpart lives at a non-mirror path. Add an
# entry here only when the asymmetry is intentional (e.g. the FR
# introduction sits at `fr/index.md` while the EN equivalent is
# `introduction.md`). Pure absence of the EN counterpart is caught
# by `sync_lang_counterparts.py --check`, not here.
ASYMMETRIC_FR_PAGES: dict[str, str | None] = {
    "index.md": "introduction.md",
    # `fr/roadmap.md` is a short stub pointing at the EN roadmap;
    # its content is not a translation of `roadmap.md`. Skip the
    # SHA check until the FR roadmap becomes a real translation.
    "roadmap.md": None,
}


def en_counterpart(fr_rel: str) -> Path | None:
    """Return the EN counterpart path for an FR page (relative to
    FR_ROOT). ``None`` means the FR page is intentionally
    asymmetric and must not be SHA-checked."""
    if fr_rel in ASYMMETRIC_FR_PAGES:
        mapped = ASYMMETRIC_FR_PAGES[fr_rel]
        return None if mapped is None else EN_ROOT / mapped
    return EN_ROOT / fr_rel


def extract_stamp(text: str) -> str | None:
    """Return the recorded SHA from the `en-source-sha` HTML
    comment, or ``None`` if the file is unstamped. Searches only
    the first 1 KB to keep the scan cheap and fail loud if a
    well-meaning author buries the stamp halfway down."""
    match = _STAMP_RE.search(text[:1024])
    return match.group(1) if match else None


def en_last_commit(en_path: Path) -> str | None:
    """Return the SHA of the last commit that touched ``en_path``,
    or ``None`` if git knows nothing about the file (untracked /
    not yet committed)."""
    if not en_path.exists():
        return None
    rel = en_path.relative_to(REPO_ROOT)
    result = subprocess.run(
        ["git", "log", "-n", "1", "--pretty=%H", "--", str(rel)],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    sha = result.stdout.strip()
    return sha or None


def walk_fr_pages() -> list[tuple[Path, str]]:
    """Return (abs_path, fr_rel) for every Markdown page under
    ``docs/src/fr/``."""
    pages: list[tuple[Path, str]] = []
    for p in sorted(FR_ROOT.rglob("*.md")):
        rel = p.relative_to(FR_ROOT).as_posix()
        pages.append((p, rel))
    return pages


def check(strict: bool) -> int:
    if not FR_ROOT.is_dir():
        print(
            f"check_lang_staleness: {FR_ROOT} does not exist.",
            file=sys.stderr,
        )
        return 2

    stale: list[tuple[str, str, str]] = []  # (fr_rel, recorded, current)
    missing_stamp: list[str] = []
    skipped: list[str] = []
    fresh = 0

    for abs_path, fr_rel in walk_fr_pages():
        en_path = en_counterpart(fr_rel)
        if en_path is None:
            skipped.append(fr_rel)
            continue
        if not en_path.exists():
            # Filename pairing is `sync_lang_counterparts.py`'s job;
            # surface the orphan here as a warning anyway so the
            # signal isn't silently lost.
            print(
                f"check_lang_staleness: FR page has no EN counterpart "
                f"on disk: {fr_rel} → expected {en_path.relative_to(REPO_ROOT)}",
                file=sys.stderr,
            )
            continue

        recorded = extract_stamp(abs_path.read_text(encoding="utf-8"))
        if not recorded:
            missing_stamp.append(fr_rel)
            continue

        current = en_last_commit(en_path)
        if current is None:
            print(
                f"check_lang_staleness: cannot resolve git SHA for "
                f"{en_path.relative_to(REPO_ROOT)} (untracked?)",
                file=sys.stderr,
            )
            continue

        if not current.startswith(recorded):
            stale.append((fr_rel, recorded, current))
        else:
            fresh += 1

    total = fresh + len(stale) + len(missing_stamp) + len(skipped)
    print(
        f"check_lang_staleness: {total} FR page(s) — "
        f"{fresh} fresh, {len(stale)} stale, "
        f"{len(missing_stamp)} missing stamp, "
        f"{len(skipped)} asymmetric (skipped).",
    )

    if missing_stamp:
        print(
            f"\n{len(missing_stamp)} FR page(s) missing `en-source-sha` front-matter:",
            file=sys.stderr,
        )
        for rel in missing_stamp:
            print(f"  {rel}", file=sys.stderr)

    if stale:
        print(
            f"\n{len(stale)} FR page(s) stale (EN counterpart moved):",
            file=sys.stderr,
        )
        for rel, recorded, current in stale:
            short_r = recorded[:12]
            short_c = current[:12]
            print(
                f"  {rel}  recorded={short_r}  current={short_c}",
                file=sys.stderr,
            )

    if strict and (stale or missing_stamp):
        return 1
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--strict",
        action="store_true",
        help="exit 1 on stale or missing-stamp pages (CI gate on `main`)",
    )
    args = parser.parse_args(argv)
    return check(strict=args.strict)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
