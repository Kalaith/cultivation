# Heavenly Mandate: Disciples of the Outer World - Product Requirements Document

> **Document Location:** `.project/prd.md`
>
> This document defines the product requirements, features, and specifications for the MVP. It is the single source of truth for what we're building.

---

## Overview

### Problem Statement
There is a lack of deep, progression-fantasy games that focus on indirect management and strategic planning rather than direct character control. Players who enjoy cultivation stories often have to play as a single hero, not as the master of a sect who guides others.

### Solution
A 2D, systems-driven RPG where the player acts as the Patriarch of a cultivation sect. The core gameplay involves managing the sect base, training procedurally-generated disciples, and sending them on missions. The focus is on indirect control, long-term progression, and permanent, meaningful consequences.

### Target Users
- **Primary:** Fans of progression/cultivation fantasy (e.g., Cradle series, xianxia novels), and players of management sims or strategy RPGs (e.g., RimWorld, Crusader Kings).
- **Secondary:** Players of idle/incremental games who desire more meaningful, systems-driven progression.

### Success Metrics
- [ ] A player can successfully guide at least one disciple from the Mortal realm to the Core Formation realm.
- [ ] The full MVP gameplay loop is implemented: Base Management -> Disciple Training -> World Map Missions -> Mission Resolution -> Upgrades.
- [ ] The game is engaging enough to encourage repeat play sessions to explore different strategies and disciple outcomes.

---

## Features

### Core Features (MVP)

#### Feature 1: Sect Base Management
**Priority:** P0 (Must Have)

**Description:**
Players manage a 2D base screen with modular, upgradable buildings. Each building provides passive bonuses and unlocks new gameplay functionality. In the MVP, buildings are placed in static, predefined slots.

**User Story:**
> As a Patriarch, I want to build and upgrade my sect's facilities so that I can improve my disciples' training speed, unlock new techniques, generate resources, and assign missions.

**Acceptance Criteria:**
- [ ] A dedicated "Sect Base" screen is implemented.
- [ ] The five starting buildings are present: Sect Hall, Training Yard, Library Pavilion, Mission Board, Spirit Garden.
- [ ] Buildings can be upgraded from Level 1 to 3.
- [ ] Upgrading a building provides a clear, documented passive bonus (e.g., `+5%` cultivation speed).

---

#### Feature 2: Disciple Management & Progression
**Priority:** P0 (Must Have)

**Description:**
Recruit, view, and manage a roster of procedurally generated disciples. Each disciple has unique stats, traits, and a cultivation path. Their progression is time-based and event-driven, with a core risk/reward mechanic for realm breakthroughs.

**User Story:**
> As a Patriarch, I want to recruit and develop unique disciples so that I can build a powerful sect capable of accomplishing dangerous missions.

**Acceptance Criteria:**
- [ ] Disciples are procedurally generated with a Name, Cultivation Realm, Talent, Attributes (Body, Mind, Spirit), Loyalty, and 1-2 random Fate Traits.
- [ ] The four MVP cultivation realms are implemented: Mortal, Qi Refinement, Foundation Establishment, Core Formation.
- [ ] Disciples cultivate over time, influenced by base bonuses and assignments.
- [ ] Breakthroughs are a roll-based check with modifiers. Failure can result in injury, stat loss, or death.
- [ ] Dead disciples are recorded in a sect history log.

---

#### Feature 3: World Map & Missions
**Priority:** P0 (Must Have)

**Description:**
A node-based 2D world map where players can assign disciples to missions. Missions are not directly controlled; they resolve over time and return a text-based report detailing the outcome.

**User Story:**
> As a Patriarch, I want to send my disciples on missions to dangerous locations so that they can gather resources, gain experience, and further the sect's influence.

**Acceptance Criteria:**
- [ ] A node-based 2D map screen is implemented with 10-15 nodes for the MVP.
- [ ] Nodes represent locations (dungeons, ruins, cities) and have a visible Danger Level.
- [ ] Players can assign 1-3 disciples to a mission from the Mission Board.
- [ ] The five MVP mission types are available: Exploration, Resource Gathering, Monster Suppression, Diplomacy, Ruin Delve.
- [ ] Mission resolution is automatic and provides a summary of events, consequences, and rewards. Consequences can include disciple injury or death.

---

## User Interface

### Screens/Views
- Sect Base: View and upgrade buildings.
- Disciple Roster: List of all disciples and their status.
- World Map: View nodes and assign missions.
- Mission Resolution: Report screen detailing mission outcomes.
- Library / Doctrines: View unlocked techniques and sect-wide buffs.

### Design Guidelines
- **Art Style:** Flat 2D, minimal animation, symbol-driven with a cultivation glyph aesthetic. UI-first.
- **Color Palette:**
| Name | Hex | Usage |
|------|-----|-------|
| Primary | #E0E0E0 | Main text, icons |
| Secondary | #B0B0B0 | Supporting text, borders |
| Accent | #60DFFF | Highlights, actions, calls to action |
| Background | #0A0A0A | Main app background |
| Surface | #111111 | Cards, panels, building slots |
- **Typography:**
  - **Headings:** A serif font with a traditional feel (e.g., Noto Serif).
  - **Body:** A clean, readable sans-serif font (e.g., Noto Sans).
  - **Code/Stats:** A clear monospace font.

---

## Technical Requirements

### Platform
- **Primary:** WebGL (WASM) for browser play.
- **Secondary:** Native Windows executable.

### Performance
- Must run smoothly on mid-range hardware and within a web browser without causing significant CPU or memory strain during idle periods.
- The main simulation tick should be efficient and not cause stuttering.

### Security
- The save file should be serialized in a way that discourages trivial manual editing to preserve the integrity of the ironman experience.

### Data
- **Game Data:** All core game data (missions, buildings, traits, etc.) will be defined in external JSON files for easy iteration.
- **Save Data:** The entire game state will be serialized to a single save file. No cloud saves for MVP.

---

## Constraints & Assumptions

### Constraints
- The art style must be minimal and achievable with simple shapes, icons, and UI elements.
- The game is strictly indirect control. The player never directly controls a disciple in combat or exploration.
- The game is ironman-by-design. Decisions have permanent consequences, and save-scumming is not an intended feature.

### Assumptions
- Players are broadly familiar with concepts from cultivation/progression fantasy genres.
- Players will find satisfaction in strategic planning and management over direct, tactical control.

### Out of Scope (For MVP)
- Direct combat control or visualization.
- Multiplayer features.
- Free-roaming on the world map.
- Complex rival sect AI and diplomacy.
- Narrative story cutscenes or a fixed plot.

---

## Glossary

| Term | Definition |
|------|------------|
| **Cultivation** | The practice of absorbing spiritual energy to grow in power, extend one's lifespan, and ascend to higher states of being. |
| **Realm** | A distinct stage or level of power in the cultivation journey (e.g., Mortal, Qi Refinement). |
| **Breakthrough** | The high-risk, high-reward process of attempting to advance from one realm to the next. |
| **Sect** | An organization or school dedicated to a particular path of cultivation, led by the player (the Patriarch). |
| **Fate Trait**| A procedurally assigned trait that permanently affects a disciple's stats or behavior. |

---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-24 | Gemini | Initial draft based on GDD v1. |

---

*Last updated: 2026-01-24*