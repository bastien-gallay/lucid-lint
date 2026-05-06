#!/usr/bin/env python3
"""Shared helpers for the `texts_{fetch,clean,convert}.py` triplet.

Single point of truth for:

- loading `examples/texts.yaml`
- filtering in-scope sources (`markdownable >= 3`)
- resolving destinations from `redistribution` tier
- slug generation
- per-source metadata (`source.yaml`) read/write
- the "no wrong-tier writes" safeguard

Nothing here is executed directly. Import from the three CLI scripts.
"""

# /// script
# requires-python = ">=3.11"
# dependencies = ["pyyaml", "python-slugify"]
# ///

from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass, field
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

import yaml
from slugify import slugify

ROOT = Path(__file__).resolve().parent.parent
TEXTS_YAML = ROOT / "examples" / "texts.yaml"
LOCAL_TEXTS_YAML = ROOT / "examples" / "local" / "texts.yaml"
PUBLIC_ROOT = ROOT / "examples" / "public"
LOCAL_ROOT = ROOT / "examples" / "local"

MIN_MARKDOWNABLE = 3

POLARITY_DIRS = {
    "good_example": "good",
    "bad_example": "bad",
    "before_after": "before-after",
    "mixed_unaligned": "mixed",
    "neutral": "neutral",
}

TIER_TO_ROOT = {
    "public_ok": PUBLIC_ROOT,
    "check_license": LOCAL_ROOT,
    "link_only": LOCAL_ROOT,
    "restricted": LOCAL_ROOT,
}


@dataclass
class Source:
    """One row from `texts.yaml`, normalised."""

    url: str
    title: str
    description: str
    type: str
    polarity: str
    languages: list[str]
    redistribution: str
    markdownable: int
    license_details: str
    rules_relevant: list[str] = field(default_factory=list)
    conditions: list[str] = field(default_factory=list)
    use_case: list[str] = field(default_factory=list)
    has_explanations: bool = False
    bilingual_parallel: bool = False
    license_or_access: str = ""
    interest: str = ""
    confidence: str = ""
    notes: str = ""

    @property
    def slug(self) -> str:
        return slugify(self.title, max_length=64)

    @property
    def primary_lang(self) -> str:
        """Folder name for the language slot. Bilingual sources → `bi`."""
        if len(self.languages) >= 2:
            return "bi"
        if self.languages:
            return self.languages[0]
        return "xx"

    @property
    def polarity_dir(self) -> str:
        return POLARITY_DIRS.get(self.polarity, "neutral")


def load_sources(
    path: Path = TEXTS_YAML,
    local_path: Path | None = None,
) -> list[Source]:
    """Parse `texts.yaml` (public) + `examples/local/texts.yaml` (if present).

    The public file holds only `public_ok` entries; everything else lives
    in the gitignored local mirror. Tooling gets the merged view, public
    surfaces read the public file directly (see AGENTS.md #10).
    """
    allowed = {
        "url",
        "title",
        "description",
        "type",
        "polarity",
        "languages",
        "redistribution",
        "markdownable",
        "license_details",
        "rules_relevant",
        "conditions",
        "use_case",
        "has_explanations",
        "bilingual_parallel",
        "license_or_access",
        "interest",
        "confidence",
        "notes",
    }

    def _load(p: Path) -> list[Source]:
        data = yaml.safe_load(p.read_text(encoding="utf-8")) or {}
        return [
            Source(**{k: v for k, v in row.items() if k in allowed})
            for row in data.get("sources", [])
        ]

    sources = _load(path)
    lp = LOCAL_TEXTS_YAML if local_path is None else local_path
    if lp.exists():
        sources.extend(_load(lp))
    return sources


def in_scope(src: Source) -> bool:
    """Source is in scope for the scraper (markdownable threshold)."""
    return src.markdownable >= MIN_MARKDOWNABLE


def resolve_destination(src: Source) -> Path:
    """Return the `<slug>/` folder path for this source.

    Single source of truth for tier → path mapping. Anything bypassing
    this function is a bug.
    """
    root = TIER_TO_ROOT.get(src.redistribution)
    if root is None:
        raise ValueError(f"Unknown redistribution tier {src.redistribution!r} for {src.url}")
    return root / src.primary_lang / src.polarity_dir / src.slug


def assert_under_known_root(path: Path) -> None:
    """Panic if `path` tries to escape the two managed roots."""
    resolved = path.resolve()
    for root in (PUBLIC_ROOT, LOCAL_ROOT):
        try:
            resolved.relative_to(root)
            return
        except ValueError:
            continue
    raise RuntimeError(f"Refusing to write outside examples/{{public,local}}/: {resolved}")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()


def now_iso() -> str:
    return datetime.now(UTC).isoformat(timespec="seconds")


def read_source_yaml(folder: Path) -> dict[str, Any] | None:
    meta = folder / "source.yaml"
    if not meta.exists():
        return None
    return yaml.safe_load(meta.read_text(encoding="utf-8"))


def write_source_yaml(folder: Path, data: dict[str, Any]) -> None:
    assert_under_known_root(folder)
    folder.mkdir(parents=True, exist_ok=True)
    (folder / "source.yaml").write_text(
        yaml.safe_dump(data, sort_keys=False, allow_unicode=True),
        encoding="utf-8",
    )


def source_to_metadata(src: Source) -> dict[str, Any]:
    """Freeze the source row into a plain dict for `source.yaml`."""
    return {
        "url": src.url,
        "title": src.title,
        "type": src.type,
        "polarity": src.polarity,
        "languages": src.languages,
        "redistribution": src.redistribution,
        "license_details": src.license_details,
        "markdownable": src.markdownable,
        "rules_relevant": src.rules_relevant,
        "conditions": src.conditions,
        "bilingual_parallel": src.bilingual_parallel,
    }


def extension_for_content(content_type: str, url: str) -> str:
    """Best-effort file extension for the raw download."""
    ct = content_type.lower()
    if "pdf" in ct:
        return "pdf"
    if "html" in ct or "xhtml" in ct:
        return "html"
    if "xml" in ct:
        return "xml"
    if "json" in ct:
        return "json"
    if "csv" in ct:
        return "csv"
    if "text/plain" in ct:
        return "txt"
    m = re.search(r"\.([a-zA-Z0-9]{1,6})(?:\?|#|$)", url)
    if m:
        ext = m.group(1).lower()
        if ext in {"html", "htm", "pdf", "txt", "xml", "json", "csv", "md"}:
            return "html" if ext == "htm" else ext
    return "html"


def is_github_repo(url: str) -> bool:
    return re.match(r"^https?://github\.com/[^/]+/[^/]+/?$", url) is not None


def frontmatter_block(meta: dict[str, Any]) -> str:
    body = yaml.safe_dump(meta, sort_keys=False, allow_unicode=True).strip()
    return f"---\n{body}\n---\n\n"
