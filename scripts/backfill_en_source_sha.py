"""One-shot: stamp every existing FR page with its EN counterpart's
SHA at the time the FR page was first committed.

Run once when introducing the F92 staleness sub-task. After this
runs, every FR page that the staleness checker monitors carries
an `en-source-sha` HTML-comment stamp; subsequent translations
add the stamp at creation time, so this script should never need
to run again.

Algorithm per FR page:
  1. Find the commit that *introduced* the FR file
     (`git log --diff-filter=A --follow --format=%H`, tail).
  2. Find the EN counterpart's last commit at or before that point
     (`git log -n1 --format=%H <fr_introducing_sha> -- <en_path>`).
  3. Insert ``<!-- en-source-sha: <sha> -->`` as the first line of
     the FR file (replacing any existing stamp).

Pages mapped to ``None`` in
``check_lang_staleness.ASYMMETRIC_FR_PAGES`` are skipped (no EN
counterpart to anchor to). Run with ``--dry-run`` to preview.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from check_lang_staleness import (  # noqa: E402
    FR_ROOT,
    REPO_ROOT,
    en_counterpart,
    extract_stamp,
    walk_fr_pages,
)

# Match a leading YAML front-matter block left over from an earlier
# (broken) iteration of this script — strip it before re-stamping.
_LEADING_YAML_BLOCK = re.compile(r"\A\s*---\n[\s\S]*?\n---\n+")
_LEADING_STAMP = re.compile(
    r"\A\s*<!--\s*en-source-sha:[^>]*-->\s*\n+",
)


def fr_introducing_commit(fr_path: Path) -> str | None:
    """Return the commit SHA that first added ``fr_path`` (oldest
    commit on the file's history). Returns ``None`` if git knows
    nothing about the file."""
    rel = fr_path.relative_to(REPO_ROOT)
    result = subprocess.run(
        [
            "git",
            "log",
            "--diff-filter=A",
            "--follow",
            "--format=%H",
            "--",
            str(rel),
        ],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    shas = [s for s in result.stdout.split() if s.strip()]
    return shas[-1] if shas else None


def en_sha_at_or_before(en_path: Path, ref_sha: str) -> str | None:
    """Return the EN counterpart's last-touching commit at or
    before ``ref_sha``. Returns ``None`` if the EN file did not
    exist at that point."""
    rel = en_path.relative_to(REPO_ROOT)
    result = subprocess.run(
        ["git", "log", "-n", "1", "--format=%H", ref_sha, "--", str(rel)],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    sha = result.stdout.strip()
    return sha or None


def insert_stamp(text: str, sha: str) -> str:
    """Return ``text`` with ``<!-- en-source-sha: <sha> -->`` as
    the first line. Strips any pre-existing stamp or stale YAML
    front-matter block before inserting."""
    body = _LEADING_STAMP.sub("", text, count=1)
    body = _LEADING_YAML_BLOCK.sub("", body, count=1)
    return f"<!-- en-source-sha: {sha} -->\n{body}"


def backfill(dry_run: bool) -> int:
    if not FR_ROOT.is_dir():
        print(f"backfill: {FR_ROOT} not found.", file=sys.stderr)
        return 2

    stamped = 0
    skipped_asymmetric = 0
    skipped_already = 0
    skipped_no_history = 0

    for fr_path, fr_rel in walk_fr_pages():
        en_path = en_counterpart(fr_rel)
        if en_path is None:
            skipped_asymmetric += 1
            continue
        if not en_path.exists():
            print(
                f"backfill: skipping {fr_rel} — EN counterpart missing on disk.",
                file=sys.stderr,
            )
            continue

        text = fr_path.read_text(encoding="utf-8")
        # Re-stamping is fine — `--force` is implicit since the
        # YAML-front-matter iteration shipped briefly and needs
        # cleanup. Skip only when a fresh HTML-comment stamp is
        # already present AND no stale YAML block remains.
        has_stamp = extract_stamp(text) is not None
        has_stale_yaml = bool(_LEADING_YAML_BLOCK.match(text))
        if has_stamp and not has_stale_yaml:
            skipped_already += 1
            continue

        intro = fr_introducing_commit(fr_path)
        if intro is None:
            print(
                f"backfill: skipping {fr_rel} — no git history.",
                file=sys.stderr,
            )
            skipped_no_history += 1
            continue

        en_sha = en_sha_at_or_before(en_path, intro)
        if en_sha is None:
            print(
                f"backfill: skipping {fr_rel} — EN had no commit at or "
                f"before FR introduction ({intro[:12]}).",
                file=sys.stderr,
            )
            skipped_no_history += 1
            continue

        new_text = insert_stamp(text, en_sha)
        if dry_run:
            print(f"would stamp {fr_rel}: en-source-sha={en_sha[:12]}")
        else:
            fr_path.write_text(new_text, encoding="utf-8")
            print(f"stamped {fr_rel}: en-source-sha={en_sha[:12]}")
        stamped += 1

    print(
        f"\nbackfill: {stamped} stamped, "
        f"{skipped_already} already-stamped, "
        f"{skipped_asymmetric} asymmetric, "
        f"{skipped_no_history} no-history.",
    )
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="preview stamps without writing files",
    )
    args = parser.parse_args(argv)
    return backfill(dry_run=args.dry_run)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
