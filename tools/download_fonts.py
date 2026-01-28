#!/usr/bin/env python3
"""
Download brush-style fonts for the cultivation game.
Downloads from GitHub mirror of Google Fonts.
"""

import urllib.request
import os

# Output directory
FONTS_DIR = os.path.join(os.path.dirname(__file__), "..", "assets", "fonts")

def download_font(url: str, output_name: str):
    """Download a font from URL."""
    print(f"Downloading {output_name}...")

    try:
        output_path = os.path.join(FONTS_DIR, output_name)

        # Download the TTF file directly
        urllib.request.urlretrieve(url, output_path)

        print(f"  Saved to {output_path}")
        return True

    except Exception as e:
        print(f"  Error: {e}")
        return False

def main():
    os.makedirs(FONTS_DIR, exist_ok=True)

    # Direct links to TTF files from Google Fonts GitHub mirror
    fonts = [
        # Permanent Marker - bold marker/brush style
        ("https://github.com/google/fonts/raw/main/apache/permanentmarker/PermanentMarker-Regular.ttf",
         "brush.ttf"),
    ]

    for url, output_name in fonts:
        download_font(url, output_name)

    print("\nDone! Fonts saved to:", FONTS_DIR)

if __name__ == "__main__":
    main()
