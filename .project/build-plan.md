# Heavenly Mandate - Build Plan

> **CRITICAL INSTRUCTIONS FOR ENGINEERS**
>
> ## Project Structure
> All project documentation lives in the `.project/` directory:
> ```
> .project/
> ├── prd.md           # Product Requirements Document
> ├── tech-stack.md    # Technology choices and rationale
> ├── build-plan.md    # This file - task tracking
> └── changelog.md     # Version history and updates
> ```
>
> ## Build Discipline
> 1. **Keep this document up to date** - Mark tasks as completed immediately after finishing them.
> 2. **Build after every task** - Run the build command after completing each task.
> 3. **Zero tolerance for warnings/errors** - Fix any `cargo clippy` warnings or `cargo build` errors before moving on.
> 4. **Update changelog.md** - Log significant changes at the end of each phase.
>
> ```bash
> # Build command (run after each task)
> cargo clippy -- -D warnings && cargo build
> ```

---

## Status Legend

| Icon | Status | Description |
|------|--------|-------------|
| ⬜ | Not Started | Task has not begun |
| 🔄 | In Progress | Currently being worked on |
| ✅ | Completed | Task finished |
| ⛔ | Blocked | Cannot proceed due to external dependency |
| ⚠️ | Has Blockers | Waiting on another task |
| 🔍 | In Review | Pending review/approval |
| 🚫 | Skipped | Intentionally not doing |
| ⏸️ | Deferred | Postponed to later phase/sprint |

---

## Project Progress Summary

```
Phase 1: Foundation         [████████████████████] 100%  ✅
Phase 2: Data & State       [████████████████████] 100%  ✅
Phase 3: Sect Base (F1)     [████████████████████] 100%  ✅
Phase 4: Disciples (F2)     [████████████████████] 100%  ✅
Phase 5: Missions (F3)      [████████████████████] 100%  ✅
Phase 6: Persistence & Build[████████████████████] 100%  ✅
Phase 7: Deep Systems (Old) [████████████████████] 100%  ✅
Phase 8: Polish & UI        [████████████████████] 100%  ✅
Phase 9: Taiyi & Stratif.   [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
Phase 10: Feng Shui/Elements[░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
Phase 11: Laws & Training   [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
Phase 12: Crafting          [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
─────────────────────────────────────────────────────────
Overall Progress            [█████████████░░░░░░░]  66%
```
---

## Phase 1-8: (Completed)
*Refer to previous versions or git history for detailed breakdown of completed phases.*
*(Phases 1-8 are complete as of the previous MVP delivery.)*

---

## Phase 9: Survivor Scenario & Stratification (Expanded Feature 1)

**Goal:** Implement the specific startup scenario, player-chosen Sect Name, and the separation of Outer/Inner disciples.

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 9.1.1 | **Data:** Update `Disciple` struct to include a `Rank` enum (`Outer`, `Inner`, `Elder`). |
| ✅ | 9.1.2 | **Logic:** Implement `promote_disciple` function that changes rank and unlocks Inner stats (Qi). |
| ✅ | 9.1.3 | **Logic:** Restrict `Mining`, `Building`, `Hauling` jobs to `Outer` rank in the job system. |
| ✅ | 9.2.1 | **Scenario:** Create a `New Game` flow that initializes the "Survivors" preset (1 Leader, 2 Outer). |
| 🔄 | 9.2.2 | **UI:** Create "Sect Creation" modal/screen to input Sect Name. |
| ⬜ | 9.2.3 | **UI:** Update `Character Sheet` to visually distinguish Outer vs Inner disciples. |
| ⬜ | 9.3.1 | **BUILD CHECK** - Can start a new game, name the sect, and promote a worker. |

---

## Phase 10: Feng Shui & Five Elements (Expanded Feature 2)

**Goal:** Implement the environmental interactions and room manufacturing logic.

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 10.1.1 | **Data:** Add `Element` enum (`Metal`, `Wood`, `Water`, `Fire`, `Earth`) to all `Item` and `Building` structs. |
| ⬜ | 10.1.2 | **Logic:** Implement `get_elemental_interaction(a, b)` helper to return Create/Destroy/Neutral relationships. |
| ⬜ | 10.2.1 | **Grid:** Add `FengShuiRating` and `ElementStrength` maps to the `Grid` state. |
| ⬜ | 10.2.2 | **Logic:** Implement `calculate_room_stats()` which scans a room's contents and orientation to output a Feng Shui score. |
| ⬜ | 10.3.1 | **UI:** Create a "Feng Shui Overlay" mode for the base view, coloring tiles by their dominant element. |
| ⬜ | 10.3.2 | **Mechanic:** Apply buffs/debuffs to characters inside rooms based on the Feng Shui score. |
| ⬜ | 10.4.1 | **BUILD CHECK** - Placing wood furniture increases Wood element; Water suppresses Fire. |

---

## Phase 11: Cultivation Laws (Expanded Feature 3)

**Goal:** Data-driven cultivation paths for Inner Disciples.

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 11.1.1 | **Data:** Define `CultivationLaw` struct with `Stages`, `RequiredElement`, and `StatModifiers`. |
| ⬜ | 11.1.2 | **Loaders:** Create JSON loader for `assets/data/laws/*.json`. |
| ⬜ | 11.2.1 | **Logic:** Implement xp/qi gain formula modification based on the disciple's Law vs Environment Element. |
| ⬜ | 11.2.2 | **Breakthrough:** Implement the breakthrough minigame/check that differs per Law phase. |
| ⬜ | 11.3.1 | **UI:** Create "Cultivation" tab in Character Sheet showing current Law progress and next bottleneck. |
| ⬜ | 11.4.1 | **BUILD CHECK** - Disciple with Fire Law gains more Qi in a Fire room. |

---

## Phase 12: Advanced Crafting (PCrafting)

**Goal:** Multi-step production for Pills and Artifacts.

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 12.1.1 | **Data:** Define `Recipe` struct for Alchemy and Smithing. |
| ⬜ | 12.1.2 | **Logic:** Implement `CraftingTask` that consumes specific ingredients and outputs items based on user skill. |
| ⬜ | 12.2.1 | **Alchemy:** Implement "Pill" item type with consumption effects (Heal, Boost Qi). |
| ⬜ | 12.2.2 | **Artifacts:** Implement "Artifact" item type that can be equipped by Inner Disciples for combat stats. |
| ⬜ | 12.3.1 | **BUILD CHECK** - Can craft a basic Spirit Stone Sword and equip it. |

---

*Last updated: 2026-01-25*
*Current Phase: Phase 9*