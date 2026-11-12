#!/usr/bin/env python3
"""Build a local, French-only content bundle from the archived site extraction."""

from __future__ import annotations

import html
import json
import re
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "SAPTRANS_Extraction_Ancien_Site_2026-11-09.html"
OUTPUT = ROOT / "public/data/legacy-content.json"

# Only pages with useful French company information are imported. This excludes
# the English theme demo pages and unrelated WordPress posts automatically.
PAGES = {
    "/": "home",
    "/nos-services/": "services",
    "/nos-services/transport-et-logistique/": "transport",
    "/nos-services/location-engins/": "equipment",
    "/nos-services/visite-technique/": "inspection",
    "/entreprise/": "company",
    "/entreprise/historique/": "history",
    "/entreprise/equipe/": "team",
    "/contact/": "contact",
}


class ExtractionParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.depth = 0
        self.record: dict[str, object] | None = None
        self.capture: tuple[str, list[str]] | None = None
        self.records: list[dict[str, object]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag == "details":
            if self.depth == 0:
                self.record = {"summary": "", "text": []}
            self.depth += 1
        if self.depth == 1 and self.record is not None and tag in ("summary", "pre"):
            self.capture = (tag, [])

    def handle_endtag(self, tag: str) -> None:
        if self.capture and self.capture[0] == tag and self.record is not None:
            captured_tag, chunks = self.capture
            if captured_tag == "summary":
                self.record["summary"] = "".join(chunks)
            else:
                self.record["text"].append("".join(chunks))
            self.capture = None
        if tag == "details" and self.depth:
            self.depth -= 1
            if self.depth == 0 and self.record is not None:
                self.records.append(self.record)
                self.record = None

    def handle_data(self, data: str) -> None:
        if self.capture:
            self.capture[1].append(data)


def paragraphs_for(key: str, lines: list[str]) -> list[str]:
    clean = [re.sub(r"\s+", " ", html.unescape(line)).strip() for line in lines]
    clean = [line for line in clean if line and not line.startswith("This website uses cookies")]
    if key == "transport":
        return [line for line in clean if line.startswith(("Spécialiste du transport industriel", "Notre expertise en matière", "Nous sommes en mesure de vous fournir", "Nos équipes sont formées"))]
    if key == "equipment":
        return [line for line in clean if line.startswith(("Vous êtes un particulier", "La pelle chargeuse"))]
    if key == "inspection":
        return [line for line in clean if line.startswith(("La visite techniqu", "Contrôle de la pression", "Contrôle visuel"))]
    if key == "company":
        return [line for line in clean if line.startswith(("Aujourd‘hui Saptrans", "Saptrans ayant sa genèse"))]
    if key == "history":
        return [line for line in clean if line.startswith(("La société Sapou Transport", "EN 2018,"))]
    if key == "services":
        return [line for line in clean if line.startswith(("SAPOU TRANSPORT SARL", "À côté du transport", "Dans le secteur de la sécurité routière"))]
    if key == "home":
        return [line for line in clean if line.startswith(("Nous assurons le transport", "Nous mettons à votre disposition", "AutoRelax"))]
    if key == "team":
        return [line for line in clean if line.startswith(("Sapou Kana", "Fokam", "Kapche", "Foko", "Nantia", "Nana"))]
    if key == "contact":
        return [line for line in clean if line.startswith(("SAPOU TRANSPORT SARL", "Douala-Yassa", "Limbé", "+237", "infos."))]
    return []


def main() -> None:
    if not SOURCE.exists():
        raise SystemExit(f"Fichier d’extraction introuvable : {SOURCE.name}")
    parser = ExtractionParser()
    parser.feed(SOURCE.read_text(encoding="utf-8", errors="replace"))
    imported: dict[str, dict[str, object]] = {}
    for record in parser.records:
        summary = str(record.get("summary", ""))
        match = re.search(r"https?://saptrans\.net([^\s]*)", summary)
        if not match:
            continue
        path = match.group(1).split("?", 1)[0]
        if path not in PAGES or path in imported:
            continue
        key = PAGES[path]
        text_blocks = record.get("text", [])
        raw = next((block for block in text_blocks if block.strip()), "")
        lines = [line.strip() for line in raw.splitlines()]
        imported[path] = {
            "key": key,
            "title": summary.split(" — ", 1)[0].strip(),
            "source": f"https://saptrans.net{path}",
            "paragraphs": paragraphs_for(key, lines),
        }

    missing = sorted(set(PAGES) - set(imported))
    if missing:
        raise SystemExit("Pages absentes de l’extraction : " + ", ".join(missing))
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps({"source_file": SOURCE.name, "pages": imported}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"Contenu local extrait : {len(imported)} pages → {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
