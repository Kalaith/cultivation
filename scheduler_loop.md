# AI Scheduler Loop – Implementation Guide (7 Phases)

This document defines a practical, incremental implementation plan for the AI scheduling system. It is designed to fit the current architecture (Macroquad + game loop) and existing gameplay constraints (Outer/Inner ranks, building assignments, missions).

---

## Phase 1 — Core Data Model

**Goal:** Establish the minimal data types needed for scheduling, without changing gameplay behavior yet.

**Deliverables**
- `NeedType` enum (e.g., Hunger, Rest, Qi, Morale).
- `NeedState` with fields:
  - `current: f32`, `max: f32`, `decay_per_tick: f32`, `urgent_threshold: f32`.
- `TaskType` enum (Idle, Rest, Eat, WorkBuilding, Cultivate, Haul, Build, Repair, MissionPrep, etc.).
- `Task` struct:
  - `task_type: TaskType`
  - `target_id: Option<u64>`
  - `duration_ticks: u32`
  - `required_rank: DiscipleRank`
  - `priority_base: f32`
  - `cooldown_until: u32`
- `TaskResult` enum: Success, Failed(TaskFailReason), Canceled.
- `TaskFailReason` enum: MissingResource, TargetUnavailable, Unreachable, Timeout, InvalidRank.
- `ScheduledTask`:
  - `disciple_id: u64`
  - `task: Task`
  - `ticks_remaining: u32`
  - `started_at: u32`

**Implementation Notes**
- Add a `needs: HashMap<NeedType, NeedState>` to `Disciple` or a `DiscipleAI` struct.
- Keep serialization in mind (derive `Serialize/Deserialize`).

---

## Phase 2 — Scheduler Loop (Baseline)

**Goal:** Assign simple tasks per tick based on needs and role.

**Deliverables**
- `Scheduler` struct containing:
  - `current_assignments: HashMap<u64, ScheduledTask>`
  - `tick: u32`
- Scoring function:
  - `score = need_urgency * weight + task.priority_base + role_bonus - distance_penalty`.
- A minimal task catalog created each tick (or cached):
  - Rest, Eat, Cultivate (Inner), WorkBuilding (Outer).

**Algorithm (per tick)**
1. Decay needs for all disciples.
2. For each disciple without an active task:
   - Build candidate task list.
   - Score and select the highest.
   - Assign and create a `ScheduledTask`.
3. For each active task:
   - Decrement `ticks_remaining`.
   - On completion, apply effects (restore need, produce resources).

**Implementation Notes**
- Start with only “non-spatial” tasks, then layer pathing later.
- Ensure `WorkBuilding` respects building assignment rules.

---

## Phase 3 — Reservations & Conflict Handling

**Goal:** Prevent multiple disciples from attempting the same target or job.

**Deliverables**
- `Reservation` struct:
  - `holder_id: u64`
  - `target_id: u64`
  - `expires_at: u32`
- `ReservationMap` (e.g., `HashMap<u64, Reservation>` keyed by target).
- Reservation API:
  - `try_reserve(target_id, disciple_id)`
  - `release(target_id)`
  - `is_reserved(target_id)`

**Conflict Handling Rules**
- If reservation fails, rescore and pick another task.
- If a reserved target becomes invalid, cancel task with reason.
- Add timeouts: if task exceeds `duration + grace`, fail and reschedule.

---

## Phase 4 — Integration with Game Loop

**Goal:** Tie the scheduler into the existing update loop without breaking gameplay.

**Deliverables**
- `Game::update()` (or equivalent) calls scheduler after time/season updates and before production ticks.
- `WorkBuilding` task uses existing `assign_disciple_to_building()` constraints.
- Inner/Outer role enforcement:
  - Outer: labor tasks.
  - Inner: cultivation and combat prep, no labor.

**Execution Order**
1. Update time/season.
2. Update missions.
3. Run scheduler tick.
4. Apply task outputs (resources, skill gains).
5. Resolve UI events.

---

## Phase 5 — Edge Case Handling

**Goal:** Stabilize behavior under invalid, blocked, or pathological states.

**Deliverables**
- “Unreachable” detection hooks (even if pathing is stubbed).
- “No valid tasks” fallback → Idle + morale decay.
- Automatic task cancellation if:
  - Resource missing.
  - Target destroyed.
  - Rank mismatch (e.g., promoted/demoted).
- Cooldown system to prevent thrashing (e.g., can’t retry same failed task for N ticks).

**Testing Scenarios**
- No food available.
- All buildings assigned but no workers.
- Inner disciple with only labor tasks present.

---

## Phase 6 — Debugging & UI Tools

**Goal:** Make it inspectable and tunable during development.

**Deliverables**
- Debug panel showing per-disciple:
  - Active task, remaining ticks, needs values, task score.
- Event log entries on task selection and cancellation.
- Toggle-able overlay (e.g., keybind) for AI state.

**Implementation Notes**
- Keep UI lightweight (use existing components).
- Add colored need bars for fast visual feedback.

---

## Phase 7 — Persistence & Tuning

**Goal:** Ensure AI state is saved and balance can be adjusted via data files.

**Deliverables**
- Save/load for:
  - `needs`, `current_assignments`, `reservations`, scheduler `tick`.
- JSON tuning file for:
  - Need decay rates.
  - Priority weights.
  - Task durations.
- Versioned save format (handle migration when adding new needs/tasks).

**Validation**
- Load mid-task and verify it completes correctly.
- Ensure reservations are rehydrated without duplicating.

---

## Suggested Milestone Breakdown

- **Milestone A:** Phases 1–2 (Minimal scheduler loop + needs).
- **Milestone B:** Phase 3 (Reservations + conflicts).
- **Milestone C:** Phases 4–5 (Integration + edge cases).
- **Milestone D:** Phases 6–7 (Debugging + persistence).

If you want, I can start implementing Milestone A next and wire the scheduler into the game loop incrementally.
