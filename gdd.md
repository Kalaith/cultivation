# Heavenly Mandate: Disciples of the Outer World

**Working Title**

## Overview
**Genre:**
* Cultivation / Progression Fantasy
* Base Management + World Map Missions
* 2D Systems-Driven RPG

**Target Engine:**
* Rust + Macroquad

**Scope:**
* MVP-first, expandable into a long-term progression game

## High Concept
You are the Patriarch of a small cultivation sect, operating from a fragile main base. Rather than adventuring yourself, you train, assign, and risk disciples by sending them on missions across a dangerous world map.

The world does not wait. Sects rise, monsters migrate, realms advance. Your job is to shape destiny indirectly, through preparation, doctrine, and selective intervention.

## Core Pillars (MVP)
* **Indirect Gameplay** – You do not control disciples directly in combat.
* **Progression Fantasy** – Realms, breakthroughs, bottlenecks, destiny.
* **Base-Centric Design** – The sect is the heart of power.
* **World Map Missions** – Risk, reward, permanent consequences.
* **Minimal Art Load** – Icons, tiles, UI-first.

## Core Gameplay Loop
1. **Manage Sect Base**
2. **Train / Recruit Disciples**
3. **Assign World Map Missions**
4. **Missions Resolve Over Time**
5. **Receive Outcomes**
6. **Upgrade Base, Disciples, or Doctrine**
7. **World Evolves**
8. **Repeat**

The loop is designed to survive low motivation days: meaningful progress in short sessions.

## Player Role
You are not a hero. You are:
* A planner
* A teacher
* A risk manager
* A quiet hand nudging fate

Your power scales horizontally through people, not vertically through stats.

## Core Systems (MVP)

### 1. Sect Base
The base is a single 2D screen with modular buildings.

**Starting Buildings**

| Building | Function |
| :--- | :--- |
| **Sect Hall** | Unlocks disciples, doctrines |
| **Training Yard** | Improves cultivation speed |
| **Library Pavilion** | Unlocks techniques |
| **Mission Board** | Assign world missions |
| **Spirit Garden** | Passive resource income |

**Buildings have:**
* Level (1–3 in MVP)
* Passive bonuses
* Unlock requirements
* No freeform building placement in MVP. Static slots.

### 2. Disciples
Each disciple is a procedural character.

**Core Stats:**
* **Name**
* **Cultivation Realm**
* **Talent** (Low → Heaven-Sent)
* **Attributes** (Body, Mind, Spirit)
* **Loyalty**
* **Fate Traits** (1–2 random)

**Example Fate Traits:**
* *"Unlucky"* – Higher injury chance
* *"Dao Insight"* – Faster breakthroughs
* *"Bloodthirsty"* – Stronger vs enemies, weaker diplomacy

**Cultivation Realms (MVP):**
1. Mortal
2. Qi Refinement
3. Foundation Establishment
4. Core Formation

**Each realm unlocks:**
* New missions
* Higher survival odds
* Sect-wide bonuses at high ranks

### 3. Cultivation & Progression
Cultivation is time-based + event-based.

**Progress comes from:**
* Base bonuses
* Assigned training
* Successful missions
* Rare breakthroughs

**Breakthroughs:**
* Not guaranteed
* Roll-based with modifiers
* Failure causes injuries, stagnation, or death
* *This reinforces tension and attachment.*

### 4. World Map
A node-based 2D map, not free roaming.

**Each node represents:**
* Region
* Dungeon
* Ruin
* City
* Sect Territory

**Nodes have:**
* Danger Level
* Known Rewards
* Unknown Risks
* Faction Influence

**The map evolves over time:**
* Nodes gain corruption
* Sects rise or fall
* New threats emerge
* You are reacting to a living board.

### 5. Missions
You assign 1–3 disciples per mission.

**Mission Types (MVP):**
* Exploration
* Resource Gathering
* Monster Suppression
* Diplomacy
* Ruin Delve

**Each mission has:**
* Duration (real-time or turn-based ticks)
* Success thresholds
* Partial success states
* Failure consequences
* *Resolution is text + icons, not combat simulation.*

### 6. Consequences (Very Important)
**No reload safety.**

**Possible outcomes:**
* Injury
* Lost cultivation
* Trauma traits
* Death
* Betrayal
* Legendary success

Dead disciples are remembered. Their names remain in sect history.

### 7. Resources
Minimal but meaningful.

**Resource List**

| Resource | Use |
| :--- | :--- |
| **Spirit Stones** | Buildings, techniques |
| **Herbs** | Healing, pills |
| **Relics** | Rare upgrades |
| **Influence** | Diplomacy, sect standing |

*Avoid resource soup.*

## UI & Presentation
**Art Style:**
* Flat 2D
* Minimal animation
* Symbol-driven
* Cultivation glyph aesthetics

**Screens:**
* Sect Base
* Disciple Roster
* World Map
* Mission Resolution
* Library / Doctrines

*Macroquad-friendly. No fancy shaders needed.*

## Technical Scope (Rust + Macroquad)
**Data-Driven First:**
JSON for:
* Disciples
* Missions
* Buildings
* World nodes

**Simulation Tick:**
* Central game clock
* Missions resolve on ticks
* World events trigger probabilistically

**Save System:**
* Serialize entire game state
* Ironman-friendly

## MVP Feature Cut Line

**Included:**
* Single map region
* 10–15 mission nodes
* 3–4 buildings
* 10–20 disciples
* 4 cultivation realms

**Explicitly Excluded:**
* Direct combat
* Multiplayer
* Free movement
* Visual combat animations
* Story cutscenes

## Expansion Hooks (Not MVP)
* Rival sect AI
* Heavenly Tribulations
* Ascension events
* Sect wars
* Hidden Dao paths
* Player intervention cards
