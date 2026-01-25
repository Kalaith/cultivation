# Heavenly Mandate: Disciples of the Outer World - Product Requirements Document

> **Document Location:** `.project/prd.md`
>
> This document defines the product requirements, features, and specifications for the Expanded MVP. It is the single source of truth for what we're building.

---

## Overview

### Problem Statement
There is a lack of deep, progression-fantasy games that focus on indirect management and strategic planning with complex underlying systems (Feng Shui, Wuxing). Players often want to simulate the "Sect Master" experience of managing resources, shaping terrain, and guiding disciples through complex cultivation laws, rather than just fighting battles.

### Solution
A 2D, systems-driven Colony Sim / RPG where the player acts as the Patriarch of the fallen Taiyi Sect. The core gameplay involves complex base building (Feng Shui), stratified disciple management (Outer vs Inner), and an elemental simulation (Five Elements) that drives combat, crafting, and progression.

### Target Users
- **Primary:** Fans of *Amazing Cultivation Simulator*, *RimWorld*, and heavy Xianxia novels (Cradle, Renegade Immortal).
- **Secondary:** Players of strategy/logistics games who enjoy optimizing complex systems and production chains.

### Success Metrics
- [ ] A player can successfully establish a base with at least one "Auspicious" (Good Feng Shui) room.
- [ ] The Outer/Inner disciple loop is functional: Outer disciples provide resources, Inner disciples protect the sect.
- [ ] At least one Cultivation Law is fully playable from Mortal to Golden Core.
- [ ] The Five Elements interactions (Creation/Destruction cycles) are correctly implemented in interacting with the environment and combat.

---

## Features

### Core Features (Expanded MVP)

#### Feature 1: Survivor Scenario & Stratified Society
**Priority:** P0 (Must Have)
**Description:**
The game starts with a specific scenario: The fall of the player's old sect. Players name their new Sect, control a "Sect Leader", and manage a few survivors. The population is split into **Outer Disciples** (Workers) and **Inner Disciples** (Cultivators).
**User Story:**
> As a Sect Leader, I need to name my new sect and assign mundane tasks to my Outer Disciples so my Inner Disciples have the time and resources to cultivate.
**Acceptance Criteria:**
- [ ] Scenario start: "Fall of the Sect" moves the player to a random map location.
- [ ] Player is prompted to input a Name for their Sect.
- [ ] Disciple Stratification: Characters can be promoted from Outer to Inner.
- [ ] Outer Disciples handle: Farming, Mining, Hauling, Building.
- [ ] Inner Disciples handle: Cultivation, Combat, Artifact Crafting.

#### Feature 2: Feng Shui & Five Elements (Wu Xing)
**Priority:** P0 (Must Have)
**Description:**
The game world acts on the Five Elements cycle (Metal > Water > Wood > Fire > Earth). Rooms have "Feng Shui" ratings based on the harmony of their furniture and structure.
**User Story:**
> As a builder, I want to construct a Fire-element cultivation room using Wood furniture (to feed Fire) so that my Fire Law disciples cultivate faster.
**Acceptance Criteria:**
- [ ] All items/materials have an Element tag and a Material tag.
- [ ] Feng Shui calculation: Rooms are rated (Very Bad to Very Auspicious) based on element interactions and door orientation.
- [ ] Element interactions: Water suppresses Fire, Wood feeds Fire, etc.
- [ ] Auspicious rooms provide buffs; Ominous rooms cause debuffs or disasters.

#### Feature 3: Cultivation Laws
**Priority:** P0 (Must Have)
**Description:**
Inner Disciples must practice a specific "Law" (Class) that dictates their spells, stat growth, and elemental needs.
**User Story:**
> As a player, I want to find the "True Sun Refining Law" so I can train a powerful fire-attribute sword master.
**Acceptance Criteria:**
- [ ] Laws framework implemented using data files.
- [ ] Disciples can "Learn" a Law upon becoming Inner Disciples.
- [ ] Laws unlock specific Spells and Stat modifiers.
- [ ] "Breakthrough" minigame/check implemented based on the Law's requirements.

#### Feature 4: Expanded Crafting & Industry
**Priority:** P1 (High)
**Description:**
A deep crafting system where output quality depends on material tier and crafter skill.
**Acceptance Criteria:**
- [ ] Alchemy: Creating Pills for healing and cultivation XP.
- [ ] Blacksmithing: Forging weapons and farming tools.
- [ ] Artifact Refining: Transforming items into flying artifacts for Inner Disciples.

---

## User Interface

### Screens/Views
- **Sect Base (Grid):** Main view. Top-down, tile-based. Needs overlays for Feng Shui and Element strength.
- **Character Gear/Psyche:** Detailed view showing relationships, mood, body parts, and equipped Law.
- **Manuals Library:** A tree or list view of known Laws and unlocked nodes.

### Design Guidelines
- **Art Style:** "Cultivation Glyph" aesthetic. Clean, readable icons over complex 3D models.
- **Visual Feedback:** Elements should be color-coded (Red=Fire, Green=Wood, etc.).

---

## Technical Requirements

### Data
- **Complex Definitions:** JSON schema must support nested Law definitions (stages, nodes, requirements).
- **Save System:** Must serialize the state of the Grid (elements, heat, qi) and all Entities efficiently.

### Performance
- **Simulation:** The "World Tick" must handle element propagation (fire spreading, Qi flowing) without stalling the game.

---

## Glossary

| Term | Definition |
|------|------------|
| **Wu Xing** | The Five Elements (Metal, Wood, Water, Fire, Earth). |
| **Feng Shui** | The art of arranging rooms to harmonize with the environment/elements. |
| **Law** | A specific cultivation method (class) that determines a character's path. |
| **Qi** | Spiritual energy. Gathered for cultivation. Flows through the map. |
| **Outer Disciple** | A worker who handles logistics and menial labor. |
| **Inner Disciple** | A cultivator who focuses on fighting and ascension. |

---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.1 | 2026-01-25 | Antigravity | Updated for "Expanded GDD" features (Taiyi, Wuxing, Laws). |
| 1.0 | 2026-01-24 | Gemini | Initial draft based on GDD v1. |

---

*Last updated: 2026-01-25*