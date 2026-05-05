#!/usr/bin/env python3
"""Clean raw downloads written by `texts_fetch.py`.

Walks every `<slug>/raw.*` under `examples/{public,local}/` and writes a
`clean.<ext>` next to it. Cleaning is format-aware:

- HTML → `trafilatura.extract(output_format="html")` keeps the main
  article body, drops navigation/footer/comments. Output stays HTML so
  future format-support work has a noise-free but native artefact.
- PDF  → `pypdf` text extraction → `clean.txt`.
- txt / csv / json / xml → copy as-is to `clean.<ext>` (no cleaning
  available, but the symmetry simplifies `texts_convert.py`).
- git clones (`raw/` directory) → no-op; the per-source adapter in
  `texts_convert.py` does the extraction.

Idempotent: skips files whose `clean.*` is fresher than the `raw.*` and
already matches the recorded sha256.

Run with:

    uv run scripts/texts_clean.py [--force] [--only=<slug>]
"""

# /// script
# requires-python = ">=3.11"
# dependencies = ["pyyaml", "python-slugify", "trafilatura", "pypdf"]
# ///

from __future__ import annotations

import argparse
import shutil
import sys
from pathlib import Path

from texts_common import (
    LOCAL_ROOT,
    PUBLIC_ROOT,
    assert_under_known_root,
    read_source_yaml,
    sha256_file,
    write_source_yaml,
)


def _iter_source_folders(only_slug: str | None) -> list[Path]:
    folders: list[Path] = []
    for root in (PUBLIC_ROOT, LOCAL_ROOT):
        if not root.exists():
            continue
        for source_yaml in root.rglob("source.yaml"):
            folder = source_yaml.parent
            if only_slug and folder.name != only_slug:
                continue
            folders.append(folder)
    return folders


def _clean_html(raw: Path, out: Path) -> bool:
    import trafilatura  # lazy import — heavy

    html = raw.read_text(encoding="utf-8", errors="replace")
    cleaned = trafilatura.extract(
        html,
        output_format="html",
        include_comments=False,
        include_tables=True,
        favor_recall=True,
    )
    if not cleaned:
        # trafilatura failed to find a main article — fall back to raw.
        print("    warn: trafilatura returned empty; copying raw.")
        out.write_text(html, encoding="utf-8")
        return True
    out.write_text(cleaned, encoding="utf-8")
    return True


def _clean_pdf(raw: Path, out: Path) -> bool:
    from pypdf import PdfReader  # lazy import

    try:
        reader = PdfReader(str(raw))
    except Exception as exc:  # noqa: BLE001 — surface any pypdf issue
        print(f"    FAIL: pypdf could not open: {exc}")
        return False
    chunks: list[str] = []
    for page in reader.pages:
        try:
            chunks.append(page.extract_text() or "")
        except Exception as exc:  # noqa: BLE001
            print(f"    warn: page extraction error: {exc}")
    out.write_text("\n\n".join(chunks), encoding="utf-8")
    return True


def _clean_passthrough(raw: Path, out: Path) -> bool:
    shutil.copy2(raw, out)
    return True


def _clean_one(folder: Path, force: bool) -> bool:
    raws = sorted(folder.glob("raw.*"))
    if not raws:
        # git-cloned sources have `raw/` directory, not `raw.*`.
        if (folder / "raw").is_dir():
            return True  # nothing to do; adapter will walk on convert
        print(f"  skip   {folder.name}: no raw.* found")
        return True
    raw = raws[0]
    assert_under_known_root(raw)
    ext = raw.suffix.lstrip(".")

    # HTML → clean.html, PDF → clean.txt, anything else → clean.<ext>
    if ext in {"htm"}:
        ext = "html"
    if ext == "pdf":
        out = folder / "clean.txt"
        cleaner = _clean_pdf
    elif ext == "html":
        out = folder / "clean.html"
        cleaner = _clean_html
    else:
        out = folder / f"clean.{ext}"
        cleaner = _clean_passthrough

    meta = read_source_yaml(folder) or {}
    recorded_raw_sha = meta.get("raw_sha256")
    current_raw_sha = sha256_file(raw)

    if (out.exists() and not force and recorded_raw_sha == current_raw_sha):
        print(f"  skip   {folder.name}: {out.name} up-to-date")
        return True

    print(f"  clean  {folder.name} → {out.name}")
    ok = cleaner(raw, out)
    if ok:
        meta["raw_sha256"] = current_raw_sha
        meta["clean_file"] = out.name
        meta["clean_sha256"] = sha256_file(out)
        write_source_yaml(folder, meta)
    return ok


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--force", action="store_true",
                        help="Re-clean even if clean.* is fresh.")
    parser.add_argument("--only", metavar="SLUG",
                        help="Clean a single <slug>/ folder.")
    args = parser.parse_args()

    folders = _iter_source_folders(args.only)
    if args.only and not folders:
        print(f"No source folder named {args.only!r}", file=sys.stderr)
        return 2

    print(f"Cleaning {len(folders)} source folder(s).")
    failed = 0
    for folder in folders:
        if not _clean_one(folder, args.force):
            failed += 1

    print(f"\nDone. {len(folders) - failed}/{len(folders)} cleaned without error.")
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
