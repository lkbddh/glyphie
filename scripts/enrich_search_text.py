#!/usr/bin/env python3
"""
Enrich emojis.json search_text with CLDR keyword annotations.

Downloads Unicode CLDR English annotation files and merges keywords
into each emoji's search_text field for better search results.

Usage: python3 scripts/enrich_search_text.py
"""

import json
import xml.etree.ElementTree as ET
import urllib.request
import os

CLDR_BASE = "https://raw.githubusercontent.com/unicode-org/cldr/main/common"
ANNOTATIONS_URL = f"{CLDR_BASE}/annotations/en.xml"
DERIVED_URL = f"{CLDR_BASE}/annotationsDerived/en.xml"

EMOJI_JSON = os.path.join(os.path.dirname(__file__), "..", "data", "emojis.json")

# Minimum keyword length to include (filters single-char noise like "a", "i", "x")
MIN_KEYWORD_LENGTH = 2


def download(url: str) -> str:
    """Download a URL and return its content as a string."""
    print(f"Downloading {url}...")
    with urllib.request.urlopen(url) as resp:
        return resp.read().decode("utf-8")


def parse_cldr_keywords(xml_text: str, keywords: dict):
    """Parse CLDR annotation XML and extract keyword annotations (not tts)."""
    root = ET.fromstring(xml_text)
    for ann in root.findall(".//annotation"):
        cp = ann.get("cp")
        typ = ann.get("type", "")
        if typ == "tts" or not cp or not ann.text:
            continue
        kws = {k.strip().lower() for k in ann.text.split("|") if k.strip() and len(k.strip()) >= MIN_KEYWORD_LENGTH}
        if cp in keywords:
            keywords[cp].update(kws)
        else:
            keywords[cp] = set(kws)


def normalize_emoji(emoji: str) -> str:
    """Normalize emoji string for matching: strip VS16 (U+FE0F)."""
    return emoji.replace("\ufe0f", "")


def main():
    # Download CLDR data
    ann_xml = download(ANNOTATIONS_URL)
    derived_xml = download(DERIVED_URL)

    # Parse keywords
    keywords = {}
    parse_cldr_keywords(ann_xml, keywords)
    parse_cldr_keywords(derived_xml, keywords)
    print(f"Loaded keywords for {len(keywords)} emoji codepoints")

    # Build normalized lookup (strip VS16 for matching)
    normalized_keywords = {}
    for cp, kws in keywords.items():
        norm = normalize_emoji(cp)
        if norm in normalized_keywords:
            normalized_keywords[norm].update(kws)
        else:
            normalized_keywords[norm] = set(kws)

    # Load emojis.json
    with open(EMOJI_JSON, "r", encoding="utf-8") as f:
        emojis = json.load(f)
    print(f"Loaded {len(emojis)} emojis from {EMOJI_JSON}")

    # Enrich search_text
    enriched = 0
    missed = 0
    for emoji in emojis:
        e = emoji["e"]
        norm_e = normalize_emoji(e)

        # Try exact match, then normalized match
        kws = keywords.get(e) or normalized_keywords.get(norm_e)

        if kws:
            # Current search_text has: "name subcategory"
            # Split into existing words to avoid duplicating
            existing_words = set(emoji["st"].lower().split())
            # Add only new keywords
            new_kws = sorted(kws - existing_words)
            if new_kws:
                emoji["st"] = emoji["st"] + " " + " ".join(new_kws)
                enriched += 1
        else:
            missed += 1

    print(f"Enriched: {enriched}, Already complete: {len(emojis) - enriched - missed}, No CLDR match: {missed}")

    # Show some missed ones for debugging
    if missed > 0:
        miss_examples = []
        for emoji in emojis:
            e = emoji["e"]
            norm_e = normalize_emoji(e)
            if not keywords.get(e) and not normalized_keywords.get(norm_e):
                miss_examples.append(f"  {e} ({emoji['n']})")
                if len(miss_examples) >= 10:
                    break
        print("Sample unmatched emojis:")
        print("\n".join(miss_examples))

    # Write back
    with open(EMOJI_JSON, "w", encoding="utf-8") as f:
        json.dump(emojis, f, ensure_ascii=False, separators=(",", ":"))
    print(f"Written to {EMOJI_JSON}")

    # Verify improvements
    print("\n--- Verification ---")
    tests = [
        ("happy", "😀😃😄"),
        ("poop", "💩"),
        ("100", "💯"),
        ("laugh", "😂🤣😆"),
        ("smile", "😀😃🙂"),
        ("pray", "🙏"),
        ("wave", "👋"),
        ("sad", "😢😭"),
        ("fire", "🔥"),
        ("thumbs up", "👍"),
    ]
    for query, expected in tests:
        matches = [e for e in emojis if query in e["st"].lower()]
        found = "".join(e["e"] for e in matches[:8])
        status = "✅" if matches else "❌"
        print(f"  {status} \"{query}\" -> {len(matches)} hits: {found}")


if __name__ == "__main__":
    main()
