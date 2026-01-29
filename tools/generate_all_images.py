"""
Batch generator for all 95 ink-wash images for Heavenly Mandate.
Run from project root: python tools/generate_all_images.py
"""

import subprocess
import sys
from pathlib import Path

# Base command
SCRIPT_DIR = Path(__file__).resolve().parent
PROJECT_ROOT = SCRIPT_DIR.parent

GENERATOR_PATH = SCRIPT_DIR / "ink_wash_generator.py"
GENERATOR = f"python \"{GENERATOR_PATH}\""

# Output directories
ASSETS_DIR = PROJECT_ROOT / "assets" / "generated"
BG_DIR = ASSETS_DIR / "backgrounds"
BUILDINGS_DIR = ASSETS_DIR / "buildings"
REGIONS_DIR = ASSETS_DIR / "regions"
MISSIONS_DIR = ASSETS_DIR / "missions"
REALMS_DIR = ASSETS_DIR / "realms"
ELEMENTS_DIR = ASSETS_DIR / "elements"
SEASONS_DIR = ASSETS_DIR / "seasons"
FACTIONS_DIR = ASSETS_DIR / "factions"
EVENTS_DIR = ASSETS_DIR / "events"
UI_DIR = ASSETS_DIR / "ui"
ITEMS_DIR = ASSETS_DIR / "items"
LOADING_DIR = ASSETS_DIR / "loading"

# All image definitions
IMAGES = [
    # ========== 1. SCREEN BACKGROUNDS (11) ==========
    {"name": "bg_main_menu", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1042,
     "args": "--terrain world_map --time-of-day dawn --structure-density 1.2"},
    {"name": "bg_sect_base", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1105,
     "args": "--terrain river_valley --time-of-day day --structure-density 1.5"},
    {"name": "bg_roster", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1210,
     "args": "--terrain plains --time-of-day dawn --element wood"},
    {"name": "bg_world_map", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1320,
     "args": "--terrain world_map --structure-density 1.2"},
    {"name": "bg_mission_assign", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1425,
     "args": "--terrain cliffs --time-of-day dusk"},
    {"name": "bg_mission_result", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1530,
     "args": "--terrain river_valley --time-of-day dawn"},
    {"name": "bg_library", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1650,
     "args": "--terrain plains --element wood --structure-density 1.2"},
    {"name": "bg_factions", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1755,
     "args": "--terrain mountains --structure-density 1.8"},
    {"name": "bg_trade", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1860,
     "args": "--terrain river_valley --element water --time-of-day day"},
    {"name": "bg_tribulation", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 1970,
     "args": "--terrain cliffs --weather storm --time-of-day night --celestial-glow 0.5"},
    {"name": "bg_sect_creation", "dir": BG_DIR, "w": 1280, "h": 720, "seed": 2050,
     "args": "--terrain lakeside --time-of-day dawn --structure-density 0.2"},

    # ========== 2. BUILDING ILLUSTRATIONS (13) ==========
    {"name": "bld_sect_hall", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2100,
     "args": "--terrain mountains --structure-density 2.0 --no-seal"},
    {"name": "bld_dormitory", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2110,
     "args": "--terrain plains --structure-density 1.5 --no-seal"},
    {"name": "bld_training_yard", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2120,
     "args": "--terrain plains --element metal --no-seal"},
    {"name": "bld_spirit_garden", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2130,
     "args": "--terrain lakeside --element wood --celestial-glow 0.2 --no-seal"},
    {"name": "bld_mission_board", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2140,
     "args": "--terrain river_valley --structure-density 1.0 --no-seal"},
    {"name": "bld_library", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2150,
     "args": "--terrain mountains --element wood --structure-density 1.5 --no-seal"},
    {"name": "bld_alchemy", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2160,
     "args": "--terrain cliffs --element fire --time-of-day dusk --no-seal"},
    {"name": "bld_forge", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2170,
     "args": "--terrain cliffs --element fire --element metal --no-seal"},
    {"name": "bld_drying", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2180,
     "args": "--terrain plains --element wood --time-of-day day --no-seal"},
    {"name": "bld_storage", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2190,
     "args": "--terrain mountains --element earth --no-seal"},
    {"name": "bld_herb_garden", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2200,
     "args": "--terrain river_valley --element wood --no-seal"},
    {"name": "bld_greenhouse", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2210,
     "args": "--terrain lakeside --element water --celestial-glow 0.15 --no-seal"},
    {"name": "bld_mine", "dir": BUILDINGS_DIR, "w": 512, "h": 384, "seed": 2220,
     "args": "--terrain cliffs --element earth --time-of-day dusk --no-seal"},

    # ========== 3. WORLD MAP REGIONS (8) ==========
    {"name": "region_central", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3000,
     "args": "--terrain plains --time-of-day day --no-seal"},
    {"name": "region_northern", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3010,
     "args": "--terrain mountains --element metal --weather snow --no-seal"},
    {"name": "region_southern", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3020,
     "args": "--terrain river_valley --element wood --no-seal"},
    {"name": "region_eastern", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3030,
     "args": "--terrain lakeside --element water --no-seal"},
    {"name": "region_western", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3040,
     "args": "--terrain cliffs --element earth --structure-density 0.3 --no-seal"},
    {"name": "region_spirit_vein", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3050,
     "args": "--terrain cascades --celestial-glow 0.6 --no-seal"},
    {"name": "region_demon_lands", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3060,
     "args": "--terrain cliffs --time-of-day night --weather storm --no-seal"},
    {"name": "region_celestial", "dir": REGIONS_DIR, "w": 512, "h": 384, "seed": 3070,
     "args": "--terrain cascades --celestial-glow 0.8 --time-of-day dawn --no-seal"},

    # ========== 4. MISSION SCENES (5) ==========
    {"name": "mission_exploration", "dir": MISSIONS_DIR, "w": 512, "h": 384, "seed": 4000,
     "args": "--structure-density 0.2 --no-seal"},
    {"name": "mission_gathering", "dir": MISSIONS_DIR, "w": 512, "h": 384, "seed": 4010,
     "args": "--element earth --no-seal"},
    {"name": "mission_combat", "dir": MISSIONS_DIR, "w": 512, "h": 384, "seed": 4020,
     "args": "--time-of-day dusk --element fire --no-seal"},
    {"name": "mission_diplomacy", "dir": MISSIONS_DIR, "w": 512, "h": 384, "seed": 4030,
     "args": "--structure-density 1.5 --time-of-day day --no-seal"},
    {"name": "mission_ruins", "dir": MISSIONS_DIR, "w": 512, "h": 384, "seed": 4040,
     "args": "--time-of-day dusk --structure-density 0.8 --no-seal"},

    # ========== 5. CULTIVATION REALMS (9) ==========
    {"name": "realm_mortal", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5000,
     "args": "--structure-density 0.5 --no-seal"},
    {"name": "realm_body", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5010,
     "args": "--element earth --no-seal"},
    {"name": "realm_qi", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5020,
     "args": "--celestial-glow 0.1 --no-seal"},
    {"name": "realm_foundation", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5030,
     "args": "--element earth --structure-density 1.0 --no-seal"},
    {"name": "realm_core", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5040,
     "args": "--celestial-glow 0.4 --element fire --no-seal"},
    {"name": "realm_nascent", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5050,
     "args": "--celestial-glow 0.5 --no-seal"},
    {"name": "realm_soul", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5060,
     "args": "--celestial-glow 0.6 --time-of-day night --no-seal"},
    {"name": "realm_ascension", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5070,
     "args": "--celestial-glow 0.85 --time-of-day dawn --no-seal"},
    {"name": "realm_immortal", "dir": REALMS_DIR, "w": 512, "h": 384, "seed": 5080,
     "args": "--celestial-glow 1.0 --time-of-day dawn --no-seal"},

    # ========== 6. ELEMENT THEMED (5) ==========
    {"name": "elem_metal", "dir": ELEMENTS_DIR, "w": 512, "h": 512, "seed": 6000,
     "args": "--element metal --time-of-day night"},
    {"name": "elem_wood", "dir": ELEMENTS_DIR, "w": 512, "h": 512, "seed": 6010,
     "args": "--element wood --time-of-day day"},
    {"name": "elem_water", "dir": ELEMENTS_DIR, "w": 512, "h": 512, "seed": 6020,
     "args": "--element water --weather rain"},
    {"name": "elem_fire", "dir": ELEMENTS_DIR, "w": 512, "h": 512, "seed": 6030,
     "args": "--element fire --time-of-day dusk"},
    {"name": "elem_earth", "dir": ELEMENTS_DIR, "w": 512, "h": 512, "seed": 6040,
     "args": "--element earth --time-of-day day"},

    # ========== 7. SEASONAL VARIATIONS (4) ==========
    {"name": "season_spring", "dir": SEASONS_DIR, "w": 512, "h": 384, "seed": 7000,
     "args": "--element wood --time-of-day dawn"},
    {"name": "season_summer", "dir": SEASONS_DIR, "w": 512, "h": 384, "seed": 7000,
     "args": "--element wood --time-of-day day"},
    {"name": "season_autumn", "dir": SEASONS_DIR, "w": 512, "h": 384, "seed": 7000,
     "args": "--element fire --time-of-day dusk"},
    {"name": "season_winter", "dir": SEASONS_DIR, "w": 512, "h": 384, "seed": 7000,
     "args": "--element metal --weather snow --time-of-day day"},

    # ========== 8. FACTION BANNERS (6) ==========
    {"name": "faction_azure_cloud", "dir": FACTIONS_DIR, "w": 512, "h": 384, "seed": 8000,
     "args": "--element water --celestial-glow 0.2"},
    {"name": "faction_golden_merchant", "dir": FACTIONS_DIR, "w": 512, "h": 384, "seed": 8010,
     "args": "--element earth --time-of-day day --structure-density 1.5"},
    {"name": "faction_crimson_fang", "dir": FACTIONS_DIR, "w": 512, "h": 384, "seed": 8020,
     "args": "--element fire --time-of-day dusk"},
    {"name": "faction_shadow_moon", "dir": FACTIONS_DIR, "w": 512, "h": 384, "seed": 8030,
     "args": "--time-of-day night --structure-density 0.5"},
    {"name": "faction_imperial", "dir": FACTIONS_DIR, "w": 512, "h": 384, "seed": 8040,
     "args": "--celestial-glow 0.4 --structure-density 2.0"},
    {"name": "faction_player", "dir": FACTIONS_DIR, "w": 512, "h": 384, "seed": 8050,
     "args": "--time-of-day dawn --structure-density 1.0"},

    # ========== 9. EVENT ILLUSTRATIONS (10) ==========
    {"name": "event_breakthrough", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9000,
     "args": "--celestial-glow 0.7 --time-of-day dawn"},
    {"name": "event_tribulation", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9010,
     "args": "--weather storm --time-of-day night --celestial-glow 0.3"},
    {"name": "event_death", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9020,
     "args": "--time-of-day dusk --structure-density 0.3"},
    {"name": "event_discovery", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9030,
     "args": "--celestial-glow 0.4"},
    {"name": "event_war", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9040,
     "args": "--element fire --weather storm"},
    {"name": "event_peace", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9050,
     "args": "--time-of-day dawn --structure-density 1.5"},
    {"name": "event_disaster", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9060,
     "args": "--weather storm --time-of-day night"},
    {"name": "event_blessing", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9070,
     "args": "--celestial-glow 0.9 --time-of-day dawn"},
    {"name": "event_visitor", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9080,
     "args": "--time-of-day dusk --structure-density 0.5"},
    {"name": "event_ascension", "dir": EVENTS_DIR, "w": 800, "h": 600, "seed": 9090,
     "args": "--celestial-glow 1.0 --time-of-day dawn"},

    # ========== 10. UI ELEMENTS (8) ==========
    {"name": "ui_panel_bg", "dir": UI_DIR, "w": 512, "h": 512, "seed": 10000,
     "args": "--structure-density 0.0 --no-seal"},
    {"name": "ui_header_left", "dir": UI_DIR, "w": 256, "h": 128, "seed": 10010,
     "args": "--structure-density 0.0 --no-seal"},
    {"name": "ui_header_right", "dir": UI_DIR, "w": 256, "h": 128, "seed": 10020,
     "args": "--structure-density 0.0 --no-seal"},
    {"name": "ui_divider", "dir": UI_DIR, "w": 1024, "h": 64, "seed": 10030,
     "args": "--structure-density 0.0 --no-seal"},
    {"name": "ui_corner_tl", "dir": UI_DIR, "w": 128, "h": 128, "seed": 10040,
     "args": "--structure-density 0.0 --no-seal"},
    {"name": "ui_corner_tr", "dir": UI_DIR, "w": 128, "h": 128, "seed": 10050,
     "args": "--structure-density 0.0 --no-seal"},
    {"name": "ui_corner_bl", "dir": UI_DIR, "w": 128, "h": 128, "seed": 10060,
     "args": "--structure-density 0.0 --no-seal"},
    {"name": "ui_corner_br", "dir": UI_DIR, "w": 128, "h": 128, "seed": 10070,
     "args": "--structure-density 0.0 --no-seal"},

    # ========== 11. ITEM VIGNETTES (12) ==========
    {"name": "item_spirit_stone", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11000,
     "args": "--celestial-glow 0.5 --no-seal"},
    {"name": "item_spirit_grass", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11010,
     "args": "--element wood --no-seal"},
    {"name": "item_ember_root", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11020,
     "args": "--element fire --no-seal"},
    {"name": "item_moonwell_lotus", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11030,
     "args": "--element water --time-of-day night --no-seal"},
    {"name": "item_iron_fungus", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11040,
     "args": "--element metal --no-seal"},
    {"name": "item_ginseng", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11050,
     "args": "--element earth --no-seal"},
    {"name": "item_pill", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11060,
     "args": "--element fire --celestial-glow 0.3 --no-seal"},
    {"name": "item_sword", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11070,
     "args": "--element metal --no-seal"},
    {"name": "item_scroll", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11080,
     "args": "--element wood --structure-density 0.5 --no-seal"},
    {"name": "item_relic", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11090,
     "args": "--celestial-glow 0.6 --time-of-day dusk --no-seal"},
    {"name": "item_ore", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11100,
     "args": "--element earth --no-seal"},
    {"name": "item_dried_herb", "dir": ITEMS_DIR, "w": 256, "h": 256, "seed": 11110,
     "args": "--element wood --time-of-day day --no-seal"},

    # ========== 12. LOADING SCREENS (4) ==========
    {"name": "loading_default", "dir": LOADING_DIR, "w": 1920, "h": 1080, "seed": 12000,
     "args": "--time-of-day day"},
    {"name": "loading_combat", "dir": LOADING_DIR, "w": 1920, "h": 1080, "seed": 12010,
     "args": "--element fire --time-of-day dusk --weather storm"},
    {"name": "loading_peaceful", "dir": LOADING_DIR, "w": 1920, "h": 1080, "seed": 12020,
     "args": "--time-of-day dawn --element wood"},
    {"name": "loading_ominous", "dir": LOADING_DIR, "w": 1920, "h": 1080, "seed": 12030,
     "args": "--time-of-day night --weather storm"},
]


def create_directories():
    """Create all output directories."""
    dirs = [
        ASSETS_DIR, BG_DIR, BUILDINGS_DIR, REGIONS_DIR, MISSIONS_DIR,
        REALMS_DIR, ELEMENTS_DIR, SEASONS_DIR, FACTIONS_DIR, EVENTS_DIR,
        UI_DIR, ITEMS_DIR, LOADING_DIR
    ]
    for d in dirs:
        d.mkdir(parents=True, exist_ok=True)
        print(f"  Created: {d}")


def generate_image(img_def: dict, index: int, total: int) -> bool:
    """Generate a single image."""
    name = img_def["name"]
    out_dir = img_def["dir"]
    width = img_def["w"]
    height = img_def["h"]
    seed = img_def["seed"]
    extra_args = img_def.get("args", "")

    output_path = out_dir / f"{name}.png"

    cmd = f"{GENERATOR} -o \"{output_path}\" -W {width} -H {height} -s {seed} {extra_args}"

    print(f"\n[{index}/{total}] Generating: {name}")
    print(f"  Size: {width}x{height}, Seed: {seed}")
    print(f"  Args: {extra_args}")

    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            timeout=300  # 5 minute timeout per image
        )
        if result.returncode != 0:
            print(f"  ERROR: {result.stderr}")
            return False
        print(f"  Saved: {output_path}")
        return True
    except subprocess.TimeoutExpired:
        print(f"  ERROR: Timeout generating {name}")
        return False
    except Exception as e:
        print(f"  ERROR: {e}")
        return False


def main():
    print("=" * 60)
    print("Heavenly Mandate - Ink Wash Image Generator")
    print(f"Generating {len(IMAGES)} images...")
    print("=" * 60)

    print("\nCreating directories...")
    create_directories()

    success_count = 0
    fail_count = 0

    for i, img_def in enumerate(IMAGES, 1):
        if generate_image(img_def, i, len(IMAGES)):
            success_count += 1
        else:
            fail_count += 1

    print("\n" + "=" * 60)
    print("GENERATION COMPLETE")
    print(f"  Success: {success_count}")
    print(f"  Failed:  {fail_count}")
    print(f"  Total:   {len(IMAGES)}")
    print("=" * 60)

    return 0 if fail_count == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
