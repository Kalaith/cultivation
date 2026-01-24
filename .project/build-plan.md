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
Phase 5: Missions (F3)      [██░░░░░░░░░░░░░░░░░░]  12%  🔄
Phase 6: Persistence & Build[░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
─────────────────────────────────────────────────────────
Overall Progress            [█████████████░░░░░░░]  42%
```
---

## Phase 1: Foundation & Setup

**Goal:** Establish the project structure, dependencies, and a runnable empty application.

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 1.1.1 | Initialize Git repository with `.gitignore`. |
| ✅ | 1.1.2 | Create `.project/` directory and documentation (prd, tech-stack). |
| ✅ | 1.1.3 | Create `build-plan.md` (this file). |
| ✅ | 1.1.4 | Create `changelog.md`. |
| ✅ | 1.2.1 | Initialize Cargo project (`cargo new`). |
| ✅ | 1.2.2 | Add all dependencies to `Cargo.toml` (`macroquad`, `serde`, `rand`). |
| ✅ | 1.2.3 | Create project folder structure (`src/state`, `src/data`, etc.). |
| ✅ | 1.2.4 | Implement `main.rs` entry point with window config and empty game loop. |
| ✅ | 1.2.5 | **BUILD CHECK** - Ensure project compiles and runs, showing a blank screen. |

---

## Phase 2: Core Data Structures & State Management

**Goal:** Define all core data types and the main game state machine.

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.1.1 | Implement `GameState` and `StateTransition` enums in `state/mod.rs`. |
| ✅ | 2.1.2 | Implement the main `Game` struct in `game.rs` with state and transition logic. |
| ✅ | 2.2.1 | Define `Building` structs and enums in `data/` with `serde` support. |
| ✅ | 2.2.2 | Define `Disciple` and `FateTrait` structs in `data/` with `serde` support. |
| ✅ | 2.2.3 | Define `Mission` structs in `data/` with `serde` support. |
| ✅ | 2.2.4 | Create `loader.rs` to load game data (buildings, traits) from placeholder JSON files. |
| ✅ | 2.2.5 | **BUILD CHECK** - All data structures compile and can be instantiated. |

---

## Phase 3: Sect Base Implementation (Feature 1)

**Goal:** Create a functional, viewable sect base screen.

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.1.1 | Create `SectBaseState` in `state/sect_base.rs` with `update` and `draw` methods. |
| ✅ | 3.1.2 | Add `SectBase` to the main `GameState` enum. |
| ✅ | 3.2.1 | Implement drawing logic for static building slots on the base screen. |
| ✅ | 3.2.2 | Draw the current buildings and their levels based on game state. |
| ✅ | 3.3.1 | Implement UI for selecting a building to view its details and upgrade options. |
| ✅ | 3.3.2 | Implement the upgrade logic (spend resources, increment level). |
| ⏸️ | 3.3.3 | Apply passive bonuses from building levels to the relevant game systems. |
| ✅ | 3.3.4 | **BUILD CHECK** - Can view, select, and upgrade a building. |

---

## Phase 4: Disciples & Progression Systems (Feature 2)

**Goal:** Implement disciple generation, the roster screen, and the background cultivation process.

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 4.1.1 | Create `DiscipleRosterState` in `state/roster.rs`. |
| ✅ | 4.1.2 | Implement UI to display a list of all current disciples and their core stats. |
| ✅ | 4.2.1 | Implement procedural generation logic for disciples (name, stats, traits). |
| ✅ | 4.2.2 | Add a button/event in the Sect Hall to recruit a new disciple. |
| ✅ | 4.3.1 | Implement the background cultivation tick system that increases disciple EXP over time. |
| ✅ | 4.3.2 | Implement the roll-based breakthrough logic when a disciple's EXP is full. |
| ✅ | 4.3.3 | Implement consequences for breakthrough failure (injury, death). |
| ✅ | 4.3.4 | **BUILD CHECK** - Can recruit disciples and they will progress (or die) over time. |

---

## Phase 5: World Map & Mission System (Feature 3)

**Goal:** Implement the world map, mission assignment, and outcome resolution.

| Status | Task | Description |
|--------|------|-------------|
| 🔄 | 5.1.1 | Create `WorldMapState` in `state/world_map.rs`. |
| ⬜ | 5.1.2 | Implement rendering for the node-based map, showing location names and danger levels. |
| ⬜ | 5.2.1 | Implement the Mission Board UI, listing available missions. |
| ⬜ | 5.2.2 | Create the UI for assigning 1-3 disciples to a selected mission. |
| ⬜ | 5.3.1 | Implement the backend mission resolution tick system. |
| ⬜ | 5.3.2 | Implement logic to calculate mission success based on disciple stats vs. mission difficulty. |
| ⬜ | 5.3.3 | Create the `MissionResolutionState` to display the outcome report. |
| ⬜ | 5.3.4 | Apply rewards and consequences (disciple death, resources gained) to the game state. |
| ⬜ | 5.3.5 | **BUILD CHECK** - A full mission loop can be completed. |

---

## Phase 6: Persistence & Finalization

**Goal:** Implement the save/load system and prepare final build artifacts.

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 6.1.1 | Implement the `save()` function to serialize the entire `Game` struct to `save.json`. |
| ⬜ | 6.1.2 | Implement the `load()` function to deserialize `save.json` and overwrite the current game state. |
| ⬜ | 6.1.3 | Add "Save" and "Load" buttons to the main menu/options screen. |
| ⬜ | 6.2.1 | Finalize the `publish.ps1` script for building both Windows and WebGL targets. |
| ⬜ | 6.2.2 | Create the final `index.html` file for hosting the WASM binary. |
| ⬜ | 6.2.3 | **BUILD CHECK** - Game can be saved, closed, reopened, and loaded successfully. |

---
*Last updated: 2026-01-24*
*Current Phase: Phase 4 - Disciples & Progression Systems (Feature 2)*
