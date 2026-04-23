#!/usr/bin/env python3
"""Convert cleaned artefacts to Markdown fixtures.

Walks each `<slug>/` under `examples/{public,local}/` and writes:

- `content.md` for single-polarity sources
- `before.md` + `after.md` for `before_after` sources (per user choice)

Every generated `.md` carries a YAML frontmatter block with the upstream
URL, licence, redistribution tier, rules_relevant list, and fetch
metadata — so the fixture is self-describing even if moved or copied.

HTML → Markdown uses `pandoc` (subprocess) when available, with
`markdownify` as a pure-Python fallback. PDF-extracted plain text is
wrapped in a minimal Markdown skeleton (title heading + body).

Also appends one row per generated `.md` to the appropriate
`SOURCES.md` index in `examples/{public,local}/`.

Run with:

    uv run scripts/texts_convert.py [--force] [--only=<slug>]
"""

# /// script
# requires-python = ">=3.11"
# dependencies = ["pyyaml", "python-slugify", "markdownify"]
# ///

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
from pathlib import Path

from texts_common import (
    LOCAL_ROOT,
    PUBLIC_ROOT,
    assert_under_known_root,
    frontmatter_block,
    now_iso,
    read_source_yaml,
    sha256_file,
    write_source_yaml,
)

BEFORE_AFTER_SPLIT_RE = re.compile(
    r"(?i)^\s*#+\s*(before|after|avant|après|apres)\b"
)


def _pandoc_available() -> bool:
    return shutil.which("pandoc") is not None


def _html_to_md(html: str) -> str:
    if _pandoc_available():
        proc = subprocess.run(
            ["pandoc", "-f", "html", "-t", "gfm", "--wrap=none"],
            input=html,
            text=True,
            capture_output=True,
            check=False,
        )
        if proc.returncode == 0:
            return proc.stdout
        print(f"    warn: pandoc failed ({proc.returncode}); falling back to markdownify")
    from markdownify import markdownify  # lazy import
    return markdownify(html, heading_style="ATX")


def _txt_to_md(text: str, title: str) -> str:
    body = text.strip()
    return f"# {title}\n\n{body}\n"


def _split_before_after(md: str) -> tuple[str, str] | None:
    """Split a Markdown body on ``## Before`` / ``## After`` headings.

    Returns (before, after) or None if the split isn't detected.
    """
    lines = md.splitlines()
    sections: dict[str, list[str]] = {}
    current: str | None = None
    for line in lines:
        m = BEFORE_AFTER_SPLIT_RE.match(line)
        if m:
            label = m.group(1).lower()
            key = "before" if label in {"before", "avant"} else "after"
            current = key
            sections.setdefault(key, [])
            continue
        if current is not None:
            sections[current].append(line)
    if "before" in sections and "after" in sections:
        before = "\n".join(sections["before"]).strip() + "\n"
        after = "\n".join(sections["after"]).strip() + "\n"
        return before, after
    return None


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


def _build_frontmatter(meta: dict, kind: str) -> str:
    fm = {
        "source_url": meta.get("url"),
        "title": meta.get("title"),
        "upstream_type": meta.get("type"),
        "polarity": kind,
        "languages": meta.get("languages", []),
        "redistribution": meta.get("redistribution"),
        "license": meta.get("license_details"),
        "rules_relevant": meta.get("rules_relevant", []),
        "conditions": meta.get("conditions", []),
        "fetched_at": meta.get("fetched_at"),
        "markdownable": meta.get("markdownable"),
    }
    return frontmatter_block({k: v for k, v in fm.items() if v is not None})


def _write_markdown(folder: Path, filename: str, frontmatter: str, body: str) -> Path:
    out = folder / filename
    assert_under_known_root(out)
    out.write_text(frontmatter + body.lstrip() + "\n", encoding="utf-8")
    return out


def _convert_one(folder: Path, force: bool) -> list[Path]:
    meta = read_source_yaml(folder)
    if not meta:
        print(f"  skip   {folder.name}: no source.yaml")
        return []

    clean_files = sorted(folder.glob("clean.*"))
    if not clean_files:
        if (folder / "raw").is_dir():
            print(f"  skip   {folder.name}: git clone — bespoke adapter TBD")
            return []
        print(f"  skip   {folder.name}: no clean.* found")
        return []

    clean = clean_files[0]
    ext = clean.suffix.lstrip(".")
    polarity = meta.get("polarity", "neutral")
    title = meta.get("title", folder.name)

    if ext == "html":
        md_body = _html_to_md(clean.read_text(encoding="utf-8", errors="replace"))
    elif ext == "txt":
        md_body = _txt_to_md(
            clean.read_text(encoding="utf-8", errors="replace"), title,
        )
    else:
        print(f"  skip   {folder.name}: no converter for .{ext}")
        return []

    written: list[Path] = []
    if polarity == "before_after":
        pair = _split_before_after(md_body)
        if pair is None:
            print(
                f"  warn   {folder.name}: before/after headings not found; "
                "writing single content.md"
            )
            before_path = folder / "content.md"
            if before_path.exists() and not force:
                print(f"  skip   {folder.name}: content.md present")
                return []
            fm = _build_frontmatter(meta, "before_after")
            written.append(_write_markdown(folder, "content.md", fm, md_body))
        else:
            before_body, after_body = pair
            before_path = folder / "before.md"
            after_path = folder / "after.md"
            if before_path.exists() and after_path.exists() and not force:
                print(f"  skip   {folder.name}: before/after already present")
                return []
            fm_before = _build_frontmatter(meta, "before_example")
            fm_after = _build_frontmatter(meta, "after_example")
            written.append(_write_markdown(folder, "before.md", fm_before, before_body))
            written.append(_write_markdown(folder, "after.md", fm_after, after_body))
    else:
        target = folder / "content.md"
        if target.exists() and not force:
            print(f"  skip   {folder.name}: content.md present")
            return []
        fm = _build_frontmatter(meta, polarity)
        written.append(_write_markdown(folder, "content.md", fm, md_body))

    for p in written:
        print(f"  wrote  {folder.name}/{p.name}")
        meta_key = f"md_{p.stem}_sha256"
        meta[meta_key] = sha256_file(p)
    meta["converted_at"] = now_iso()
    meta["md_backend"] = "pandoc" if _pandoc_available() else "markdownify"
    write_source_yaml(folder, meta)
    return written


def _sources_md_path(md_file: Path) -> Path:
    for root in (PUBLIC_ROOT, LOCAL_ROOT):
        try:
            md_file.relative_to(root)
            return root / "SOURCES.md"
        except ValueError:
            continue
    raise RuntimeError(f"{md_file} is outside managed roots")


def _ensure_sources_md(path: Path) -> None:
    if path.exists():
        return
    header = (
        "# Attribution for files in this folder\n\n"
        "Auto-appended by `scripts/texts_convert.py`. One row per "
        "generated `.md`. Keep sorted by path.\n\n"
        "| Path | Source URL | Upstream licence | Notes |\n"
        "|---|---|---|---|\n"
    )
    path.write_text(header, encoding="utf-8")


def _append_sources_row(md_file: Path, meta: dict) -> None:
    sources_md = _sources_md_path(md_file)
    _ensure_sources_md(sources_md)
    rel = md_file.relative_to(sources_md.parent).as_posix()
    existing = sources_md.read_text(encoding="utf-8")
    marker = f"| `{rel}` |"
    if marker in existing:
        return
    row = (
        f"| `{rel}` | {meta.get('url', '')} | "
        f"{meta.get('license_details', '')} | "
        f"{meta.get('redistribution', '')} |\n"
    )
    with sources_md.open("a", encoding="utf-8") as f:
        f.write(row)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--force", action="store_true",
                        help="Re-convert even if .md is present.")
    parser.add_argument("--only", metavar="SLUG",
                        help="Convert a single <slug>/ folder.")
    args = parser.parse_args()

    folders = _iter_source_folders(args.only)
    if args.only and not folders:
        print(f"No source folder named {args.only!r}", file=sys.stderr)
        return 2

    print(f"Converting {len(folders)} source folder(s). "
          f"pandoc={'yes' if _pandoc_available() else 'no (using markdownify)'}")
    total_written = 0
    failed = 0
    for folder in folders:
        try:
            written = _convert_one(folder, args.force)
        except Exception as exc:  # noqa: BLE001 — surface any per-source issue
            print(f"  FAIL   {folder.name}: {exc}")
            failed += 1
            continue
        total_written += len(written)
        meta = read_source_yaml(folder) or {}
        for md_file in written:
            _append_sources_row(md_file, meta)

    print(f"\nDone. Wrote {total_written} .md file(s) across "
          f"{len(folders) - failed}/{len(folders)} folder(s).")
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
