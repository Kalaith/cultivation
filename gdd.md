# Heavenly Mandate: Disciples of the Outer World

**Working Title**

## Overview
**Genre:**
* Cultivation / Progression Fantasy
* Base Management + World Map Missions
* 2D Systems-Driven RPG / Colony Sim

**Target Engine:**
* Rust + Macroquad

**Scope:**
* Expansive Cultivation Simulator (MVP Core + Long-term Roadmap)

## High Concept
You play as the surviving elder of a **Fallen Sect**, fleeing the mysterious force that wiped out your home. Leading a ragged group of survivors, you must rebuild from scratch, uncover the conspiracy behind your sect's fall, and raise a new generation of immortals to face god-tier threats.

The main loop combines deep colony simulation (base building, logistics) with high-stakes cultivation RPG elements (nurturing disciples, epic battles).

## Core Pillars
* **Indirect Gameplay** – You guide and command, but disciples act on their own traits and needs.
* **Stratified Society** – The mundane supports the magical. Outer disciples toil so Inner disciples can ascend.
* **Elemental Harmony (Feng Shui)** – The environment itself is part of your build. Room layout, facing, and materials matter.
* **Exponential Power Growth** – From struggling to chop wood to shattering mountains with a thought.
* **Living World** – Sects, politics, and the environment evolve without you.
* **Mastery, Not Gatekeeping** – Core systems (Feng Shui, Wu Xing) reward deep engagement but never block progress. A casual player can reach mid-game; the peak is earned through understanding, not grinding.

## Game Modes
* **Story Mode (Rise of the Sect):** Narrative tutorial campaign. Rebuild your sect (named by the player), investigate the destruction, and seek revenge.
* **Classic (Sandbox):** Configurable free-play. Random start, many difficulty sliders.
* **Immortal (Hardcore):** One save, ruthless difficulty, permanent death is frequent.
* **Illusory Realm (Creative):** fast building, reduced survival pressure for testing layouts.

## Core Systems

### 1. Sect Base & Feng Shui
The base is not just a container; it is a spiritual machine.

**Feng Shui & Qi:**
* **Flow:** Every room has a Feng Shui rating based on layout, door orientation, and crowding.
* **Elements (Wu Xing):** Construction materials (Wood, Stone/Earth, Metal, Ice/Water, Fire) interact.
    * *Example:* A Fire Law cultivator needs a room capable of feeding Fire (Wood furniture) but not suppressing it (Water).
* **Qi Gathering:** Rare items (Spirit Wood, Soul Gems) emit Qi. Strategic placement creates "Qi Cushions" for faster cultivation.
* **Consequences:** Good Feng Shui boosts luck and cultivation. Bad Feng Shui causes disasters, bad moods, and stagnation.

**Facilities:**
* **Sect Hall:** Administration and diplomacy.
* **Meditation Rooms:** Highly tuned Feng Shui chambers for breakthroughs.
* **Alchemy Labs & Forges:** Industrial production using elemental fires.
* **Spirit Fields:** Farming magical crops (requires specific soil/element balance).
* **Gambling House (Future):** Risk/reward economic building for high-variance gains and events.
* **Auction House (Future):** Player-facing market hub for rare items and regional trade.

### 2. Disciple Stratification
Two distinct populations with different needs and loops.

**Outer Disciples (The Foundation):**
* **Role:** Logistics, farming, mining, building, hauling, cooking.
* **Combat:** Weak, use basic tools/weapons. Cannon fodder in emergencies.
* **Goal:** Keep the sect running. High-performing ones can be promoted to Inner.

**Inner Disciples (The Core):**
* **Role:** Cultivation, combat, high-level crafting, exploration.
* **Needs:** Need meditation time, artifacts, pills, and mental balance. Do not do menial labor.
* **Goal:** Ascension. They are your "Heroes".

### 3. Cultivation: Laws & Elements
Progression is defined by the **Law** (Cultivation Method) an Inner disciple practices.

**The Five Elements:**
* **Metal, Wood, Water, Fire, Earth.**
* Systems interact in a cycle of Creation (Wood feeds Fire) and Destruction (Water extinguishes Fire).
* This affects *everything*: Combat matchups, pill effectiveness, seasonal bonuses, and room design.

**Cultivation Laws:**
* **Unique Paths:** Each Law (e.g., "True Sun Refining", "Seven Slaughtering Swords") dictates stats, spells, and breakthrough requirements.
* **Breakthroughs:** Major bottlenecks. Success depends on:
    * Disciple potential/stats.
    * Current Qi accumulation.
    * Seasonal timing and Weather.
    * Location Feng Shui.
    * Consumed pills/items.

### 4. Crafting & Industry (MVP Systems)
Production chains feed the cultivation engine. The crafting system is divided into three core pillars for the MVP.

#### 4.1 Blacksmith (Weapons + Armor)
Focuses on equipment for physical defense and offense.
**Start with swords only.** Later unlock other weapon types as a phase feature to keep animations, balance, and UI manageable.

**Implementation Plan (Data-Driven, Extensible)**
1. **Data schemas first**
    * Define data tables for items, equipment slots, crafting recipes, materials/ores, and building definitions.
    * Ensure schemas are generic so Alchemy/Talisman can reuse them (e.g., `RecipeType`, `StationTag`, `InputTags`, `OutputTags`).
2. **Equipment system core**
    * Add equipment slots (weapon, off-hand, chest, legs, arms, head, boots, ring, amulet, belt).
    * Implement equip/unequip rules from data (slot compatibility, level/realm requirements).
    * Add item stat application/removal via a modifier pipeline (no hard-coded stat names).
3. **Item & stat pipeline**
    * Define item stats, modifiers, rarity tiers, and durability as data.
    * Add a stat aggregation layer on disciples to combine base stats + item modifiers.
4. **Resource gathering: ore**
    * Add ore nodes to world map data and/or base map resource definitions.
    * Implement mission outcomes and/or base harvesting jobs that yield ore by node type.
    * Map ores to material tags used by recipes.
5. **Blacksmith building**
    * Add a Blacksmith building entry with a `StationTag` (e.g., `forge`).
    * Hook production jobs to consume recipe inputs and output crafted items using data tables.
6. **UI hooks (minimal MVP)**
    * Equip screen: list slots, show item stats, allow equip/unequip.
    * Blacksmith station UI: recipe list, required inputs, success chance, output preview.
7. **Save/load & validation**
    * Persist equipped items, item durability, and crafting queues.
    * Add data validation to fail gracefully on missing tags or invalid recipes.
8. **Future-proofing for Alchemy/Talisman**
    * Reuse `RecipeType` and `StationTag` to plug new stations.
    * Ensure item types allow consumables and non-equipment outputs.

*   **Included in MVP:**
    *   Ingredient Gathering (1)
    *   Recipe Discovery (2)
    *   Recipe Difficulty (3)
    *   Material Quality Tiers (4)
    *   Material Purity (5)
    *   Failure Chance (8)
    *   Critical Success (9)
    *   Material Synergy (10)
    *   Weapon Crafting (Sword Focus) (11)
    *   Armor Crafting (12)
    *   Repair System (24)
    *   Kiln/Furnace Upgrade (28)

*   **Delayed features:**
    *   Artifact Forging (18)
    *   Jewelry (19)
    *   Enchanting (21)
    *   Gem socketing (22)
    *   Reforging (23)

#### 4.2 Alchemy (Pills + Potions)
Alchemy becomes the primary progression engine for cultivation stages.

*   **Included in MVP:**
    *   Ingredient Gathering (1)
    *   Recipe Discovery (2)
    *   Recipe Difficulty (3)
    *   Material Quality (4)
    *   Material Purity (5)
    *   Bulk Crafting (7)
    *   Failure Chance (8)
    *   Critical Success (9)
    *   Pill/Elixir Creation (13)
    *   Potion Brewing (15)
    *   Alchemical Reactions (27)
    *   Furnace Upgrade (28)

*   **Delayed features:**
    *   Poison Crafting (16)
    *   Legendary crafting quest (26)
    *   Bulk material conversion (29)
    *   Residue collection (30)

#### 4.3 Talisman / Inscription (Utility)
Introduces spell-like consumables and tactical depth in a Wuxia flavor without a complex economy.

*   **Included in MVP:**
    *   Recipe Discovery (2)
    *   Recipe Difficulty (3)
    *   Material Quality (4)
    *   Failure Chance (8)
    *   Critical Success (9)
    *   Talisman Inscription (14)
    *   Scroll Creation (17)

#### 4.4 Crafting Phase Roadmap

**Phase 1 (MVP Launch)**
*   **Systems:** Blacksmith (swords + armor), Alchemy (pills + potions), Talismans.
*   **Features:** Basic gathering, Recipes, Quality tiers, Failure + crit success, Furnace upgrades.
*   **Keep for MVP Summary:**
    *   Material systems: 1–5, 8–10, 28
    *   Blacksmith: 11, 12, 24
    *   Alchemy: 13, 15, 27
    *   Talismans: 14, 17

**Phase 2 – Weapon Expansion**
*   Add other weapons (spears, whips, bows, etc.).
*   Jewelry crafting (19).
*   Repair specialization.
*   Material conversion (29).

**Phase 3 – Enhancement Layer**
*   Build depth increases significantly here.
*   Enchanting (21).
*   Gem socketing (22).
*   Reforging (23).
*   Residue collection (30).

**Phase 4 – High Fantasy Endgame**
*   Artifact forging (18).
*   Legendary crafting quests (26).
*   Poison crafting (16).
*   World-level materials and Dao-infused items.

**Optional Phase – Life Skills**
*   Food cooking (20).
*   Farming systems.
*   Sect economy.

### 5. Combat & Threats
Combat centers on Inner Disciples using Artifacts and Spells.

**Mechanics:**
* **Artifact Battles:** Disciples telekinetically control artifacts to clash in mid-air.
* **Spellcasting:** Elemental nukes, barriers, and control effects.
* **Body Cultivation:** Superhuman durability and melee prowess (for specific builds).
* **Formations:** Multiple disciples linking up to combine power or cast ritual magic.

**Threats:**
* **Rival Sects:** Ranging from friendly traders to hostile invaders.
* **Monster Tides:** Seasonal attacks by spirited beasts.
* **Ancient Beasts:** Boss-level threats on the map.
* **Divine Tribulation:** Lightning strikes that try to kill disciples attempting high-tier breakthroughs.

### 6. World Map & Missions
A detailed regional map populated by factions and locations.

**Activities:**
* **Recruitment:** Finding talented orphans or refugees.
* **Adventure:** Exploring dungeons or "Secret Realms" for lost manuals.
* **Gathering:** Farming rare drops like "Cursed Flux" or specific elemental herbs.
* **Events:** Story triggers, wandering immortals, moral dilemmas.

**Current Implementation Notes:**
* **Mission Board:** Assign disciples to missions (with outcomes and rewards).
* **Mission Resolution:** Dedicated results screen for completed missions.
* **World Events:** Choice-driven events that can modify relations, resources, and unlock missions.

### 7. Social & Narrative
* **Relationships:** Friendships, rivalries, and lovers. Mood affects cultivation efficiency.
* **Sect Reputation:** Defines how the world treats you (Righteous vs. Demonic path).
* **History:** The game logs the deeds of your sect. Dead disciples are memorialized.

### 8. Diplomacy, Factions & Trade
The world sim drives faction behavior, trade dynamics, and diplomacy.

**Diplomacy & Factions:**
* **Faction Screen:** View relations, faction info, and disposition.
* **Dynamic Relations:** Reputation drifts over time and changes via events and choices.
* **Territories:** Factions control regions and can lose/gain territory via world sim.

**Trade & Economy:**
* **Economy Nodes & Trade Routes:** Regional economy sim with supply, demand, and price shifts.
* **Trade Screen:** Buy/sell through a dedicated UI.

### 9. Technology & Progression
Researchable tech unlocks new buildings and mission types.

**Tech Tree:**
* **Sect Administration:** Unlocks Mission Board.
* **Geomancy:** Unlocks Feng Shui view.
* **Basic Cultivation:** Unlocks Training Yard.
* **Spirit Gardens:** Unlocks passive income via Qi gathering.

### 10. Systems & Quality-of-Life
* **Seasons:** Seasonal modifiers affect cultivation, trade, and events.
* **Tribulation Encounters:** Breakthroughs can trigger tribulation combat events.
* **Save/Load:** Cross-platform save with versioned migrations.
* **UI States:** Dedicated screens for sect creation, roster, world map, factions, trade, mission assignment, mission results, and library.

### 11. Future Feature: Spirit Beasts (Recruiting & Training)
**Status:** Planned / Future

**Recruiting Options:**
* **Taming Hunts:** Special missions to subdue or befriend spirit beasts.
* **Spirit Contracts:** Bind a beast through a ritual (costs items/karma/qi).
* **Rescue & Adoption:** World events where injured beasts can be saved and recruited.

**Training Options:**
* **Beast Training Yard:** Facility that raises loyalty and unlocks abilities.
* **Cultivation Feed:** Feed herbs or pills to evolve beasts into higher tiers.
* **Elemental Attunement:** Align a beast to a sect’s dominant element for bonuses.
* **Battle Formations:** Pair beasts with disciples for combined techniques.

### 12. Future Feature: Individual Breakthrough Stalls (Hidden Requirements)
**Status:** Planned / Future

Some disciples have **personal bottlenecks** that prevent advancing past a specific realm until they meet a hidden requirement. The player is **not told the requirement directly**; it is treated like an achievement or fate test that must be discovered through play.

**Design Notes:**
* **Per-Disciple Gate:** Each blocked disciple has a unique stall point (e.g., Mortal → Body Refinement, or later realms in future phases).
* **Hidden Objective:** The requirement is concealed from the player, but progress should be possible through normal play patterns.
* **Always a Way Forward:** Every stall must have at least one achievable path (no dead ends), even if it is difficult or time-consuming.
* **Non-Linear Progression:** Different disciples can be stalled at different times, creating varied pacing and emergent stories.

**Example Hidden Requirements (MVP-Scale):**
* Complete a **solo mission** of a certain type.
* Accumulate **Spirit Stones** in repeated bursts (e.g., 10 separate gains).
* Work a **specific building** a certain number of times (e.g., herb garden shifts).

**Future-Phase Escalation:**
* Higher realms can demand multi-step chains (e.g., solo mission + rare item + seasonal timing).
* Some requirements can tie into world events, diplomacy choices, or location-specific Feng Shui.
* UI should surface **hints** via dreams, rumors, or subtle log entries—never explicit checklists.

## Cultivation Systems Coverage (from cultivation-systems.md)

### Have / Planned in this GDD
* **Five Elements (Wu Xing)** and elemental interactions influencing combat, pills, seasons, and rooms.
* **Cultivation progression** via Laws/Methods, with **breakthroughs** and **tribulation** events.
* **Hidden bottlenecks** for specific disciples (planned).
* **Crafting/Industry:** alchemy pills, artifact refining, talismans, and production chains.
* **Sect base building** with Feng Shui, Qi flow, and elemental room design.
* **Combat content** centered on artifacts, spellcasting, body cultivation, and formations; **monster tides** and **boss threats**.
* **World map missions**, exploration, and **world events** with outcomes.
* **Factions, diplomacy, and trade economy** with regional nodes and routes.
* **Seasons** impacting cultivation, trade, and events.
* **Spirit beasts** (recruiting/training) as a **planned** feature.
* **Save/Load and core UI states** for QoL.

### Not Yet Specified / Missing
* **Combat model details** (turn-based vs real-time), stances, combo chains, counters/parries, resource meters, cooldowns.
* **Status effects** (bleed/poison/burn/stun), crit/lifesteal/reflect, vulnerability stacking.
* **Companion depth** (beast stats/skills, evolution/breeding, equipment, parties, hunger/loyalty systems).
* **Jobs/classes** and related mechanics (job change, skill trees, prestige, inheritance).
* **Skill acquisition** paths (books, master training, quest rewards, skill slots).
* **Item/equipment systems** (slots, rarity tiers, durability, enchanting, gems, set bonuses).
* **Currency taxonomy** (silver/gold/spirit stones) and higher-level economy systems (auction, exchange).
* **Character customization** options (appearance, background, cosmetic systems).
* **Progression pacing systems** (level curves, prestige, NG+, scaling difficulty).
* **Quest taxonomy** (escort/fetch/chain/branching structures beyond mission board).
* **Stat/attribute model** (core stats, derived stats, resistances, affinities).
* **Monetization systems** (if ever desired).

## MVP vs. Full Vision
**MVP Focus:**
* Survivor premise.
* Basic Outer/Inner distinction.
* 3 elemental Laws and basic Wuxing interactions.
* Functional Base building with simple Feng Shui (Room correctness).
* Combat: Mission-only outcomes (no direct combat system in MVP).
* Early world sim scaffolding (factions, economy, events).
* Mission assignment and resolution loop.
* Tech tree gating for buildings.
* Save/load and core UI screens.
* **Progression cap:** Max cultivation stage is **Foundation Establishment** for MVP.

**Full Release:**
* 12+ Laws, including secret/forbidden ones.
* Deep Feng Shui (Spatial puzzle optimization).
* God-tier enemies and Ascension endgame.
* Full Story Campaign.
* Fully realized diplomacy, trade, and world sim.
* Spirit Beast recruitment, training, and progression.
* Combat overhaul (direct combat systems beyond mission outcomes).
