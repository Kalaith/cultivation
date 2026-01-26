# Phase 10: Feng Shui & Five Elements Implementation Plan

## Goal
Implement the core "Wu Xing" (Five Elements) system and "Feng Shui" mechanics. This involves defining elemental properties for all game entities, calculating interactions (Creation/Destruction), and evaluating room quality based on these interactions.

## User Review Required
> [!IMPORTANT]
> This phase introduces the `Element` enum which will be fundamental to all future systems (Combat, Crafting, Cultivation). I will be modifying `Item`, `Building`, and `Grid` structs.

## Proposed Changes

### Data Layer
#### [MODIFY] `src/data/mod.rs`
- Export new module `elements`.

#### [NEW] `src/data/elements.rs`
- Define `Element` enum: `Metal`, `Wood`, `Water`, `Fire`, `Earth`, `None`.
- Implement helper methods:
    - `feeds(&self) -> Element`: Returns the element this one generates (Wood -> Fire).
    - `suppresses(&self) -> Element`: Returns the element this one destroys (Water -> Fire).
    - `get_interaction(&self, other: &Element) -> InteractionResult`: Returns `feeding`, `suppressing`, or `neutral`.

#### [MODIFY] `src/data/buildings.rs`
- Add `element: Element` field to `Building` struct.
- Add `material_element: Element` field (e.g., a Bed made of Iron has `Metal` material).

### Logic Layer
#### [MODIFY] `src/state/sect_base.rs` (or new `src/engine/feng_shui.rs`)
- Implement `FengShuiMap`: A grid overlay that tracks:
    - `dominant_element`: The strongest element at this tile.
    - `element_strength`: The intensity value.
    - `feng_shui_score`: Calculated "auspiciousness".
- **Algorithm:**
    1. Iterate all buildings/furniture.
    2. Propagate elemental Qi to surrounding tiles (radius based on item tier).
    3. Calculate score:
        - **Bonus:** If item element is FED by the environment (e.g. Fire bed in Wood room).
        - **Penalty:** If item element is SUPPRESSED by environment (e.g. Fire bed in Water room).

### UI Layer
#### [MODIFY] `src/state/sect_base.rs`
- Add a "Feng Shui Overlay" toggle (hotkey 'F').
- When active, draw semi-transparent tiles colored by their dominant element (Red=Fire, Green=Wood, etc.).
- Show tooltip with specific Feng Shui score when hovering a room.

## Verification Plan

### Automated Tests
- Create unit tests in `src/data/elements.rs`:
    - Verify cycle: Wood feeds Fire, Fire feeds Earth, etc.
    - Verify suppression: Water suppresses Fire, Fire suppresses Metal, etc.

### Manual Verification
1. **Build Test:**
    - Build a "Wooden Bed" (Wood).
    - Surround it with "Water decorations" (Water).
    - Check Overlay: The Bed should be "Auspicious" (Water feeds Wood).
2. **Fail Test:**
    - Surround the Wooden Bed with "Metal decorations" (Metal).
    - Check Overlay: The Bed should be "Ominous" (Metal chops Wood).
