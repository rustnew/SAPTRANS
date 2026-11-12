#!/usr/bin/env python3
"""Inventory every page, link and image recorded in the supplied HTML archive."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
from urllib.parse import urlsplit

from bs4 import BeautifulSoup

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "SAPTRANS_Extraction_Ancien_Site_2026-11-09.html"
OUTPUT = ROOT / "data/legacy_archive.json"


def local_asset_path(url: str) -> str:
    path = urlsplit(url).path
    suffix = Path(path).suffix.lower() or ".bin"
    return f"public/media/legacy/{hashlib.sha256(url.encode()).hexdigest()[:16]}{suffix}"


def cells(row) -> list[str]:
    return [cell.get_text(" ", strip=True) for cell in row.find_all("td", recursive=False)]


def main() -> None:
    source_bytes = SOURCE.read_bytes()
    soup = BeautifulSoup(source_bytes, "html.parser")
    pages = []
    for details in soup.find_all("details"):
        summary = details.find("summary")
        if summary is None:
            continue
        label = summary.get_text(" ", strip=True)
        title, separator, url = label.rpartition(" — ")
        if not separator or not url.startswith("http"):
            continue
        blocks = [node.get_text("\n", strip=True) for node in details.find_all("pre")]
        links = []
        for row in details.select("table tbody tr"):
            data = cells(row)
            if len(data) < 2:
                continue
            destination = row.find_all("td", recursive=False)[1].find("a", href=True)
            if destination is not None:
                links.append({"label": data[0], "url": destination["href"].strip()})
        pages.append({
            "url": url,
            "title": title,
            "language": "en" if "lang=en" in url else "fr_or_unknown",
            "text": blocks[0] if blocks else "",
            "metadata": blocks[1] if len(blocks) > 1 else "",
            "form_fields": blocks[2] if len(blocks) > 2 else "",
            "links": links,
        })

    media_heading = next(h for h in soup.find_all("h2") if h.get_text(" ", strip=True).startswith("11."))
    media = []
    for row in media_heading.find_next("table").select("tbody tr"):
        columns = row.find_all("td", recursive=False)
        if len(columns) < 3:
            continue
        link = columns[0].find("a", href=True)
        if link is None:
            continue
        url = link["href"].strip()
        media.append({
            "url": url,
            "alt": columns[1].get_text(" ", strip=True),
            "source_pages": [a["href"].strip() for a in columns[2].find_all("a", href=True)],
            "local_path": local_asset_path(url),
        })

    coverage_heading = next(h for h in soup.find_all("h2") if h.get_text(" ", strip=True).startswith("8."))
    coverage = []
    for row in coverage_heading.find_next("table").select("tbody tr"):
        data = cells(row)
        if data:
            coverage.append({"url": data[0], "http_status": data[1] if len(data) > 1 else "", "title": data[2] if len(data) > 2 else ""})

    sections = {}
    for heading in soup.find_all("h2")[:7]:
        content = []
        for sibling in heading.next_siblings:
            if getattr(sibling, "name", None) == "h2":
                break
            value = sibling.get_text(" ", strip=True) if hasattr(sibling, "get_text") else str(sibling).strip()
            if value:
                content.append(value)
        sections[heading.get_text(" ", strip=True)] = "\n".join(content)

    all_links = sorted({link["url"] for page in pages for link in page["links"]})
    all_archive_hrefs = sorted({a["href"].strip() for a in soup.find_all("a", href=True)})
    inventoried_hrefs = set(all_links) | {item["url"] for item in media} | {item["url"] for item in coverage}
    untracked_hrefs = sorted(set(all_archive_hrefs) - inventoried_hrefs)
    if untracked_hrefs:
        raise ValueError(f"Archive contains {len(untracked_hrefs)} untracked links: {untracked_hrefs[:3]}")
    result = {
        "source_file": SOURCE.name,
        "source_sha256": hashlib.sha256(source_bytes).hexdigest(),
        "counts": {"pages": len(pages), "covered_urls": len(coverage), "media": len(media), "unique_links": len(all_links), "all_archive_hrefs": len(all_archive_hrefs)},
        "summary_sections": sections,
        "coverage": coverage,
        "pages": pages,
        "media": media,
        "unique_links": all_links,
        "all_archive_hrefs": all_archive_hrefs,
    }
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"Archive inventoriée : {len(pages)} pages, {len(coverage)} URL, {len(media)} médias, {len(all_links)} destinations de pages, {len(all_archive_hrefs)} liens HTML distincts → {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
