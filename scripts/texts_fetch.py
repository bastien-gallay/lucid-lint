#!/usr/bin/env python3
"""Fetch raw text sources declared in `examples/texts.yaml`.

Filters to `markdownable >= 3` and writes the `public_ok` sources into
`examples/public/<lang>/<polarity>/<slug>/` with the original artefact
as `raw.<ext>` plus a `source.yaml` sidecar. GitHub sources are
shallow-cloned into `raw/`. Sources that cannot be committed are handled
separately from this pipeline.

Run with:

    uv run scripts/texts_fetch.py [--dry-run] [--force] [--only=<slug>]
"""

# /// script
# requires-python = ">=3.11"
# dependencies = ["httpx", "pyyaml", "python-slugify"]
# ///

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
import threading
import time
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from urllib.parse import urlparse

import httpx

from texts_common import (
    LOCAL_ROOT,
    Source,
    assert_under_known_root,
    extension_for_content,
    in_scope,
    is_github_repo,
    load_sources,
    now_iso,
    read_source_yaml,
    resolve_destination,
    sha256_file,
    source_to_metadata,
    write_source_yaml,
)

USER_AGENT = "lucid-lint-examples-fetcher/0.2 (+https://github.com/bastien-gallay/lucid-lint)"
ERRORS_LOG = LOCAL_ROOT / "FETCH_ERRORS.md"
REQUEST_TIMEOUT = 30.0
HOSTNAME_MIN_INTERVAL = 1.0


class HostThrottler:
    """Thread-safe per-hostname throttle to stay polite under parallelism."""

    def __init__(self, min_interval: float = HOSTNAME_MIN_INTERVAL) -> None:
        self.min_interval = min_interval
        self.last_hit: dict[str, float] = defaultdict(float)
        self._locks: dict[str, threading.Lock] = defaultdict(threading.Lock)
        self._map_lock = threading.Lock()

    def _host_lock(self, host: str) -> threading.Lock:
        with self._map_lock:
            return self._locks[host]

    def wait(self, url: str) -> None:
        host = urlparse(url).netloc
        lock = self._host_lock(host)
        with lock:
            now = time.monotonic()
            elapsed = now - self.last_hit[host]
            if elapsed < self.min_interval:
                time.sleep(self.min_interval - elapsed)
            self.last_hit[host] = time.monotonic()


_error_log_lock = threading.Lock()


def _append_error(entry: str) -> None:
    with _error_log_lock:
        ERRORS_LOG.parent.mkdir(parents=True, exist_ok=True)
        if not ERRORS_LOG.exists():
            ERRORS_LOG.write_text(
                "# Fetch errors\n\n"
                "Rows appended by `scripts/texts_fetch.py` when a source "
                "fails to download.\n\n",
                encoding="utf-8",
            )
        with ERRORS_LOG.open("a", encoding="utf-8") as f:
            f.write(entry)


def _record_error(src: Source, reason: str) -> None:
    _append_error(f"- [{now_iso()}] `{src.slug}` — {src.url} — {reason}\n")


def fetch_http(src: Source, dest_folder: Path, client: httpx.Client,
               throttler: HostThrottler, force: bool) -> bool:
    """Download `src.url` into `dest_folder/raw.<ext>`.

    Returns True on success (or skip), False on failure.
    """
    # Find any existing raw.* to decide whether to skip.
    existing = sorted(dest_folder.glob("raw.*"))
    if existing and not force:
        print(f"  skip   raw already present: {existing[0].name}")
        return True

    throttler.wait(src.url)
    try:
        response = client.get(
            src.url,
            headers={"User-Agent": USER_AGENT, "Accept": "*/*"},
            follow_redirects=True,
            timeout=REQUEST_TIMEOUT,
        )
    except httpx.HTTPError as exc:
        _record_error(src, f"network error: {exc}")
        print(f"  FAIL   {exc}")
        return False

    if response.status_code >= 400:
        _record_error(
            src,
            f"HTTP {response.status_code} (gated/paywalled?) — {response.url}",
        )
        print(f"  FAIL   HTTP {response.status_code}")
        return False

    ext = extension_for_content(
        response.headers.get("content-type", ""), str(response.url),
    )
    raw_path = dest_folder / f"raw.{ext}"

    # Clear any old raw.* extension if we're re-downloading.
    if force:
        for stale in dest_folder.glob("raw.*"):
            stale.unlink()

    dest_folder.mkdir(parents=True, exist_ok=True)
    raw_path.write_bytes(response.content)
    print(f"  ok     raw.{ext} ({len(response.content):,} bytes)")

    return True


def fetch_git(src: Source, dest_folder: Path, force: bool) -> bool:
    """Shallow-clone a GitHub repo into `dest_folder/raw/`."""
    target = dest_folder / "raw"
    if target.exists() and not force:
        print("  skip   git clone already present")
        return True
    if target.exists() and force:
        shutil.rmtree(target)

    dest_folder.mkdir(parents=True, exist_ok=True)
    try:
        subprocess.run(
            ["git", "clone", "--depth", "1", src.url, str(target)],
            check=True,
            capture_output=True,
        )
    except (subprocess.CalledProcessError, FileNotFoundError) as exc:
        _record_error(src, f"git clone failed: {exc}")
        print(f"  FAIL   {exc}")
        return False
    print("  ok     shallow clone into raw/")
    return True


def _license_drift_check(src: Source, dest_folder: Path, force: bool) -> bool:
    """Refuse to overwrite if the stored `redistribution` has changed."""
    prior = read_source_yaml(dest_folder)
    if prior is None:
        return True
    if prior.get("redistribution") == src.redistribution:
        return True
    if force:
        print(
            f"  warn   redistribution changed "
            f"({prior.get('redistribution')!r} → {src.redistribution!r}); "
            "overwriting because --force"
        )
        return True
    print(
        f"  SKIP   redistribution changed "
        f"({prior.get('redistribution')!r} → {src.redistribution!r}). "
        "Move the folder manually then re-run, or pass --force."
    )
    return False


def process(src: Source, args: argparse.Namespace,
            client: httpx.Client, throttler: HostThrottler) -> bool:
    dest = resolve_destination(src)
    assert_under_known_root(dest)

    print(f"[{src.redistribution}] {src.slug}  (md={src.markdownable})")
    if args.dry_run:
        print(f"  would write → {dest.relative_to(dest.parents[3])}")
        return True

    if not _license_drift_check(src, dest, args.force):
        return True

    # Write/refresh source.yaml (fetch metadata added post-download).
    meta = source_to_metadata(src)

    if is_github_repo(src.url):
        ok = fetch_git(src, dest, args.force)
        if ok:
            meta["fetched_at"] = now_iso()
            meta["fetch_method"] = "git_clone_depth1"
            write_source_yaml(dest, meta)
        return ok

    ok = fetch_http(src, dest, client, throttler, args.force)
    if ok:
        raws = sorted(dest.glob("raw.*"))
        raw = raws[0] if raws else None
        meta["fetched_at"] = now_iso()
        meta["fetch_method"] = "http_get"
        if raw:
            meta["raw_file"] = raw.name
            meta["raw_sha256"] = sha256_file(raw)
        write_source_yaml(dest, meta)
    return ok


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true",
                        help="Print routing plan, no I/O.")
    parser.add_argument("--force", action="store_true",
                        help="Re-download even if raw.* is present.")
    parser.add_argument("--only", metavar="SLUG",
                        help="Fetch a single source by slug.")
    parser.add_argument("--parallel", type=int, default=1, metavar="N",
                        help="Run up to N sources concurrently "
                             "(default 1). Per-host throttling stays at "
                             "1 req/sec regardless of N.")
    args = parser.parse_args()

    sources = [s for s in load_sources() if in_scope(s)]
    if args.only:
        sources = [s for s in sources if s.slug == args.only]
        if not sources:
            print(f"No in-scope source with slug {args.only!r}", file=sys.stderr)
            return 2

    workers = max(1, args.parallel)
    print(f"Fetching {len(sources)} source(s). "
          f"dry_run={args.dry_run} force={args.force} parallel={workers}")

    throttler = HostThrottler()
    ok_count = 0
    with httpx.Client() as client:
        if workers == 1:
            for src in sources:
                if process(src, args, client, throttler):
                    ok_count += 1
        else:
            with ThreadPoolExecutor(max_workers=workers) as pool:
                futures = {
                    pool.submit(process, src, args, client, throttler): src
                    for src in sources
                }
                for fut in as_completed(futures):
                    src = futures[fut]
                    try:
                        if fut.result():
                            ok_count += 1
                    except Exception as exc:  # noqa: BLE001
                        _record_error(src, f"worker exception: {exc}")
                        print(f"  FAIL   {src.slug}: worker exception: {exc}")

    print(f"\nDone. {ok_count}/{len(sources)} processed without error.")
    return 0 if ok_count == len(sources) else 1


if __name__ == "__main__":
    sys.exit(main())
