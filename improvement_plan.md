# Game Improvement Plan

Based on game_report.md feedback, organized into implementation phases.

---

## Phase 1: Quick Fixes (Immediate)
**Effort: Low | Impact: High**

### 1.1 Block Injured Disciples from Missions
- **File:** `src/state/mission_assignment.rs`
- **Change:** Filter out injured disciples from selectable list, show them grayed out with "[INJURED]" label
- **Reason:** Sending injured disciples on missions is illogical

### 1.2 Show Building Descriptions in Construction Menu
- **File:** `src/state/sect_base.rs` (draw_construction_modal)
- **Change:** Display `BuildingDefinition.description` below each building button or as tooltip
- **Reason:** Players need to know what buildings do before spending resources

### 1.3 Reduce Mortal Injury Recovery Time
- **File:** `src/data/disciples.rs` (Injury::from_breakthrough)
- **Change:**
  - Reduce base recovery from 600 to 300 ticks per severity
  - Mortals should have 0.5x multiplier (they're not dealing with Qi deviation, just exhaustion)
  - Current: Mortal moderate injury = 1200 ticks (~20 sec)
  - Target: Mortal moderate injury = 300 ticks (~5 sec)
- **Reason:** 1200 ticks for a mortal failing breakthrough is excessive

---

## Phase 2: Tech Tree Reorganization
**Effort: Medium | Impact: Medium**

### 2.1 Spread Herb Buildings Across Multiple Techs
- **File:** `assets/data/tech.json`
- **Changes:**
  - Add "herbalism" tech (unlocks HerbGarden, DryingPavilion) - cost 75 SS, prereq: basic_infrastructure
  - Add "advanced_herbalism" tech (unlocks Greenhouse, HerbStorage) - cost 200 SS, prereq: herbalism
  - Remove herb buildings from basic_infrastructure
- **File:** `assets/data/buildings.json`
- **Changes:** Update tech_required for each herb building

### 2.2 New Tech Tree Structure
```
sect_administration (0 SS) -> MissionBoard
       |
basic_infrastructure (100 SS) -> TrainingYard, SpiritGarden
       |
       +-- dormitories (150 SS) -> Dormitory
       |
       +-- herbalism (75 SS) -> HerbGarden, DryingPavilion
       |       |
       |       +-- advanced_herbalism (200 SS) -> Greenhouse, HerbStorage
       |
       +-- advanced_facilities (300 SS) -> LibraryPavilion, AlchemyFurnace, ArtifactForge
```

---

## Phase 3: Breakthrough System Rework
**Effort: High | Impact: High**

### 3.1 Experience Accumulation Before Breakthrough
- **Concept:** Disciples accumulate experience beyond exp_to_next_level, stored as "breakthrough_readiness"
- **File:** `src/data/disciples.rs`
- **Changes:**
  - Add `breakthrough_readiness: f32` (0.0 to 1.0+) to Disciple
  - Readiness increases when exp >= exp_to_next_level
  - Base breakthrough chance modified by readiness: `base_chance * (0.5 + readiness * 0.5)`
  - At readiness 1.0 (100% over threshold): normal chance
  - At readiness 2.0 (200% over): +50% bonus to chance

### 3.2 Player-Controlled Breakthroughs
- **Concept:** Breakthroughs no longer automatic - player chooses when to attempt
- **File:** `src/state/roster.rs`
- **Changes:**
  - Add "Attempt Breakthrough" button when exp >= exp_to_next_level
  - Show readiness percentage and estimated success chance
  - Player decides when disciple is ready

### 3.3 Breakthrough Items
- **File:** `assets/data/items.json`
- **Changes:** Add breakthrough-boosting items:
  ```json
  {
    "id": "qi_stabilizing_pill",
    "name": "Qi Stabilizing Pill",
    "description": "Increases breakthrough success chance by 20%",
    "item_type": "Pill",
    "effects": [{"BreakthroughBoost": 20}]
  },
  {
    "id": "foundation_pill",
    "name": "Foundation Establishment Pill",
    "description": "Greatly increases breakthrough success for Foundation stage",
    "item_type": "Pill",
    "effects": [{"BreakthroughBoost": 40}]
  }
  ```
- **File:** `src/data/items.rs`
- **Changes:** Add `BreakthroughBoost(u32)` effect
- **File:** `src/game.rs`
- **Changes:**
  - Track temporary breakthrough modifiers on disciple
  - Apply modifier when breakthrough attempted
  - Clear modifier after attempt

---

## Phase 4: Roster UI Improvements
**Effort: Medium | Impact: High**

### 4.1 Add Scrolling to Disciple List
- **File:** `src/state/roster.rs`
- **Changes:**
  - Add `scroll_offset: f32` to state
  - Implement mouse wheel scrolling in left panel
  - Clip rendering to panel bounds
  - Add scroll bar indicator

### 4.2 Add Filtering/Sorting
- **File:** `src/state/roster.rs`
- **Changes:**
  - Add filter buttons: All | Outer | Inner | Injured | Idle
  - Add sort options: Name | Rank | Realm | Power
  - Filter state persists during session

### 4.3 Compact List View Option
- **Changes:**
  - Toggle between detailed and compact list views
  - Compact: Just name + rank icon + status icons
  - Detailed: Current full info

---

## Phase 5: Storage/Inventory System
**Effort: High | Impact: Medium**

### 5.1 Sect Storage Capacity
- **Concept:** Sect has limited storage, buildings increase capacity
- **File:** `src/game.rs`
- **Changes:**
  - Add `get_storage_capacity()` method
  - SectHall provides base storage (100 units per level)
  - Check capacity before adding resources

### 5.2 Storage Buildings
- **File:** `assets/data/buildings.json`
- **Changes:**
  - Add "Treasury" building for spirit stones (+500 capacity)
  - Add "Warehouse" for general items (+200 capacity)
  - Existing HerbStorage already handles herbs

### 5.3 Storage UI
- **File:** `src/state/sect_base.rs`
- **Changes:**
  - Show storage usage in header: "Storage: 150/500"
  - Warning when near capacity
  - Building details show storage contribution

---

## Phase 6: World Map Enhancement
**Effort: Medium | Impact: Low (for now)**

### 6.1 Add Faction Territories
- **File:** `src/state/world_map.rs`
- **Changes:**
  - Color-code regions by controlling faction
  - Show faction name on hover

### 6.2 Add Trade Route Visualization
- **Changes:**
  - Draw lines between connected economy nodes
  - Color by route safety (green safe, red dangerous)

### 6.3 Add Mission Availability Indicators
- **Changes:**
  - Show icons on nodes where missions are available
  - Click node to see available missions there

### 6.4 Quick Actions from Map
- **Changes:**
  - Click node -> popup with "Send Mission" button
  - Shows node-specific missions if any

---

## Implementation Order

1. **Phase 1** - Do first, quick wins
2. **Phase 3.2** - Player-controlled breakthroughs (blocks auto-injury spam)
3. **Phase 1.3** - Then tune injury times with new system
4. **Phase 2** - Tech reorganization
5. **Phase 4.1** - Roster scrolling (needed before more disciples)
6. **Phase 3.1, 3.3** - Full breakthrough rework
7. **Phase 4.2, 4.3** - Roster polish
8. **Phase 5** - Storage (can defer)
9. **Phase 6** - World map (can defer)

---

## Quick Start: Phase 1 Implementation

Ready to implement Phase 1? The changes are:

1. `mission_assignment.rs:45-74` - Add injury check, gray out injured
2. `sect_base.rs:823` (construction modal) - Add description text
3. `disciples.rs:217-230` - Reduce base recovery, add mortal multiplier
