#!/usr/bin/env python3
"""Download the archived SAPTRANS media to local files, with a verification log."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.parse import urlsplit
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "data/legacy_archive.json"
REPORT = ROOT / "data/legacy_media_download.json"
MAX_BYTES = 20 * 1024 * 1024
TYPES = {"image/jpeg", "image/png", "image/webp", "image/avif", "image/gif", "image/svg+xml"}


def main() -> None:
    items = json.loads(INVENTORY.read_text(encoding="utf-8"))["media"]
    results = []
    for item in items:
        url = item["url"]
        destination = ROOT / item["local_path"]
        if urlsplit(url).hostname not in {"saptrans.net", "www.saptrans.net"}:
            results.append({"url": url, "status": "rejected_host"})
            continue
        if destination.is_file() and destination.stat().st_size:
            content = destination.read_bytes()
            results.append({"url": url, "status": "present", "path": item["local_path"], "bytes": len(content), "sha256": hashlib.sha256(content).hexdigest()})
            continue
        try:
            request = Request(url, headers={"User-Agent": "SAPTRANS media migration/1.0"})
            with urlopen(request, timeout=20) as response:
                media_type = response.headers.get_content_type()
                if media_type not in TYPES:
                    raise ValueError(f"unexpected content type: {media_type}")
                content = response.read(MAX_BYTES + 1)
                if not content or len(content) > MAX_BYTES:
                    raise ValueError("empty file or file exceeds 20 MiB")
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(content)
            results.append({"url": url, "status": "downloaded", "path": item["local_path"], "bytes": len(content), "sha256": hashlib.sha256(content).hexdigest(), "content_type": media_type})
        except (HTTPError, URLError, TimeoutError, ValueError, OSError) as error:
            results.append({"url": url, "status": "failed", "reason": str(error)})
            print(f"Échec : {url} — {error}")
    REPORT.write_text(json.dumps(results, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    succeeded = sum(item["status"] in {"present", "downloaded"} for item in results)
    print(f"Médias locaux : {succeeded}/{len(results)} ; détails : {REPORT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
