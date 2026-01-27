use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::data::buildings::{Building, BuildingStatus};
use crate::data::ai::AiSchedulerTuning;
use crate::data::disciples::{Disciple, DiscipleRank};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskType {
    Idle,
    Rest,
    Eat,
    Cultivate,
    WorkBuilding,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Idle => write!(f, "Idle"),
            TaskType::Rest => write!(f, "Rest"),
            TaskType::Eat => write!(f, "Eat"),
            TaskType::Cultivate => write!(f, "Cultivate"),
            TaskType::WorkBuilding => write!(f, "Work Building"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub task_type: TaskType,
    pub target_id: Option<u64>,
    pub duration_ticks: u32,
    pub priority_base: f32,
    pub required_rank: DiscipleRank,
    pub cooldown_until: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskFailReason {
    MissingResource,
    TargetUnavailable,
    Unreachable,
    Timeout,
    InvalidRank,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskResult {
    Success,
    Failed(TaskFailReason),
    Canceled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub task: Task,
    pub ticks_remaining: u32,
    pub started_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reservation {
    pub holder_id: u64,
    pub target_id: u64,
    pub expires_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedScheduledTask {
    pub disciple_id: u64,
    pub scheduled: ScheduledTask,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedCooldown {
    pub disciple_id: u64,
    pub task_type: TaskType,
    pub until: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedScheduler {
    pub tick: u64,
    pub assignments: Vec<SavedScheduledTask>,
    pub reservations: Vec<Reservation>,
    pub cooldowns: Vec<SavedCooldown>,
}

#[derive(Default)]
pub struct Scheduler {
    pub tick: u64,
    assignments: HashMap<u64, ScheduledTask>,
    reservations: HashMap<u64, Reservation>,
    cooldowns: HashMap<(u64, TaskType), u64>,
    tuning: AiSchedulerTuning,
}

impl Scheduler {
    pub fn new(tuning: AiSchedulerTuning) -> Self {
        Self {
            tick: 0,
            assignments: HashMap::new(),
            reservations: HashMap::new(),
            cooldowns: HashMap::new(),
            tuning,
        }
    }

    pub fn from_saved(saved: SavedScheduler, tuning: AiSchedulerTuning) -> Self {
        let assignments = saved
            .assignments
            .into_iter()
            .map(|entry| (entry.disciple_id, entry.scheduled))
            .collect();

        let reservations = saved
            .reservations
            .into_iter()
            .map(|reservation| (reservation.target_id, reservation))
            .collect();

        let cooldowns = saved
            .cooldowns
            .into_iter()
            .map(|cooldown| ((cooldown.disciple_id, cooldown.task_type), cooldown.until))
            .collect();

        Self {
            tick: saved.tick,
            assignments,
            reservations,
            cooldowns,
            tuning,
        }
    }

    pub fn to_saved(&self) -> SavedScheduler {
        let assignments = self
            .assignments
            .iter()
            .map(|(disciple_id, scheduled)| SavedScheduledTask {
                disciple_id: *disciple_id,
                scheduled: scheduled.clone(),
            })
            .collect();

        let reservations = self.reservations.values().cloned().collect();

        let cooldowns = self
            .cooldowns
            .iter()
            .map(|((disciple_id, task_type), until)| SavedCooldown {
                disciple_id: *disciple_id,
                task_type: task_type.clone(),
                until: *until,
            })
            .collect();

        SavedScheduler {
            tick: self.tick,
            assignments,
            reservations,
            cooldowns,
        }
    }

    pub fn get_assignment(&self, disciple_id: u64) -> Option<&ScheduledTask> {
        self.assignments.get(&disciple_id)
    }

    pub fn assignment_count(&self) -> usize {
        self.assignments.len()
    }

    pub fn reservation_count(&self) -> usize {
        self.reservations.len()
    }

    pub fn tick(
        &mut self,
        disciples: &mut [Disciple],
        buildings: &[Building],
        disciples_on_mission: &HashSet<usize>,
    ) {
        self.tick += 1;

        self.clear_expired_reservations();
        self.clear_expired_cooldowns();

        for (idx, disciple) in disciples.iter_mut().enumerate() {
            if disciples_on_mission.contains(&idx) {
                continue;
            }

            disciple.needs.decay_all();

            let mut finished_task: Option<Task> = None;
            let mut has_active_task = false;

            if let Some(active) = self.assignments.get(&disciple.id) {
                has_active_task = true;
                let timeout_limit = active
                    .task
                    .duration_ticks
                    .saturating_add(self.tuning.timeout_grace as u32) as u64;
                let timed_out = self.tick.saturating_sub(active.started_at) > timeout_limit;
                let target_invalid = active
                    .task
                    .target_id
                    .map(|target_id| !is_target_valid(buildings, target_id, disciple.id))
                    .unwrap_or(false);

                if timed_out || target_invalid {
                    let task = active.task.clone();
                    self.assignments.remove(&disciple.id);
                    if let Some(target_id) = task.target_id {
                        self.release_reservation(target_id, disciple.id);
                    }
                    self.set_cooldown(disciple.id, task.task_type.clone(), self.tuning.cooldown_on_fail);
                    has_active_task = false;
                }
            }

            if has_active_task {
                if let Some(active) = self.assignments.get_mut(&disciple.id) {
                    if active.ticks_remaining > 0 {
                        active.ticks_remaining -= 1;
                    }
                    if active.ticks_remaining == 0 {
                        finished_task = Some(active.task.clone());
                        has_active_task = false;
                    }
                }
            }

            if let Some(task) = finished_task {
                self.assignments.remove(&disciple.id);
                if let Some(target_id) = task.target_id {
                    self.release_reservation(target_id, disciple.id);
                }
                let result = apply_task_effect(disciple, &task, buildings);
                if matches!(result, TaskResult::Failed(_) | TaskResult::Canceled) {
                    self.set_cooldown(disciple.id, task.task_type.clone(), self.tuning.cooldown_on_fail);
                }
            }

            if has_active_task {
                continue;
            }

            if let Some(task) = self.select_task(disciple.id, disciple, buildings) {
                let scheduled = ScheduledTask {
                    task: task.clone(),
                    ticks_remaining: task.duration_ticks,
                    started_at: self.tick,
                };
                self.assignments.insert(disciple.id, scheduled);
            }
        }
    }

    fn select_task(&mut self, disciple_id: u64, disciple: &Disciple, buildings: &[Building]) -> Option<Task> {
        let mut best: Option<(Task, f32)> = None;

        let hunger_urgency = disciple.needs.hunger.urgency();
        if hunger_urgency > 0.0 && !self.is_on_cooldown(disciple_id, TaskType::Eat) {
            let score = self.tuning.tasks.eat_base + hunger_urgency * self.tuning.tasks.eat_urgency_weight;
            consider_task(
                &mut best,
                Task::new(
                    TaskType::Eat,
                    DiscipleRank::Outer,
                    self.tuning.tasks.eat_duration,
                    None,
                    self.tuning.tasks.eat_base,
                    self.tick,
                ),
                score,
            );
        }

        let rest_urgency = disciple.needs.rest.urgency();
        if rest_urgency > 0.0 && !self.is_on_cooldown(disciple_id, TaskType::Rest) {
            let score = self.tuning.tasks.rest_base + rest_urgency * self.tuning.tasks.rest_urgency_weight;
            consider_task(
                &mut best,
                Task::new(
                    TaskType::Rest,
                    DiscipleRank::Outer,
                    self.tuning.tasks.rest_duration,
                    None,
                    self.tuning.tasks.rest_base,
                    self.tick,
                ),
                score,
            );
        }

        let morale_urgency = disciple.needs.morale.urgency();
        if morale_urgency > 0.0 && !self.is_on_cooldown(disciple_id, TaskType::Idle) {
            let score = self.tuning.tasks.morale_base + morale_urgency * self.tuning.tasks.morale_urgency_weight;
            consider_task(
                &mut best,
                Task::new(
                    TaskType::Idle,
                    DiscipleRank::Outer,
                    self.tuning.tasks.morale_duration,
                    None,
                    self.tuning.tasks.morale_base,
                    self.tick,
                ),
                score,
            );
        }

        if is_inner_or_above(&disciple.rank) && !self.is_on_cooldown(disciple_id, TaskType::Cultivate) {
            let qi_urgency = disciple.needs.qi.urgency();
            let score = self.tuning.tasks.cultivate_base + qi_urgency * self.tuning.tasks.cultivate_urgency_weight;
            consider_task(
                &mut best,
                Task::new(
                    TaskType::Cultivate,
                    DiscipleRank::Inner,
                    self.tuning.tasks.cultivate_duration,
                    None,
                    self.tuning.tasks.cultivate_base,
                    self.tick,
                ),
                score,
            );
        }

        if !self.is_on_cooldown(disciple_id, TaskType::WorkBuilding) {
            if let Some(building) = buildings.iter().find(|b| {
                b.assigned_disciple == Some(disciple.id) && b.status == BuildingStatus::Active
            }) {
                if self.is_target_available(building.id, disciple_id) {
                    let score = self.tuning.tasks.work_base
                        + disciple.needs.morale.urgency() * self.tuning.tasks.work_morale_weight;
                    consider_task(
                        &mut best,
                        Task::new(
                            TaskType::WorkBuilding,
                            DiscipleRank::Outer,
                            self.tuning.tasks.work_duration,
                            Some(building.id),
                            self.tuning.tasks.work_base,
                            self.tick,
                        ),
                        score,
                    );
                } else {
                    self.set_cooldown(disciple_id, TaskType::WorkBuilding, self.tuning.cooldown_on_fail);
                }
            }
        }

        if let Some((task, _score)) = best {
            if can_take_task(&disciple.rank, &task) {
                if let Some(target_id) = task.target_id {
                    if !self.try_reserve(target_id, disciple_id, task.duration_ticks) {
                        self.set_cooldown(disciple_id, task.task_type.clone(), self.tuning.cooldown_on_fail);
                        return Some(Task::new(
                            TaskType::Idle,
                            DiscipleRank::Outer,
                            self.tuning.tasks.idle_duration,
                            None,
                            self.tuning.tasks.idle_base,
                            self.tick,
                        ));
                    }
                }
                return Some(task);
            }
        }

        Some(Task::new(
            TaskType::Idle,
            DiscipleRank::Outer,
            self.tuning.tasks.idle_duration,
            None,
            self.tuning.tasks.idle_base,
            self.tick,
        ))
    }

    fn is_target_available(&self, target_id: u64, disciple_id: u64) -> bool {
        match self.reservations.get(&target_id) {
            Some(reservation) => reservation.holder_id == disciple_id,
            None => true,
        }
    }

    fn try_reserve(&mut self, target_id: u64, disciple_id: u64, duration_ticks: u32) -> bool {
        if !self.is_target_available(target_id, disciple_id) {
            return false;
        }

        let expires_at = self.tick + duration_ticks as u64 + self.tuning.reservation_grace;
        self.reservations.insert(
            target_id,
            Reservation {
                holder_id: disciple_id,
                target_id,
                expires_at,
            },
        );
        true
    }

    fn release_reservation(&mut self, target_id: u64, disciple_id: u64) {
        if let Some(existing) = self.reservations.get(&target_id) {
            if existing.holder_id == disciple_id {
                self.reservations.remove(&target_id);
            }
        }
    }

    fn clear_expired_reservations(&mut self) {
        let tick = self.tick;
        self.reservations.retain(|_, reservation| reservation.expires_at > tick);
    }

    fn set_cooldown(&mut self, disciple_id: u64, task_type: TaskType, duration: u64) {
        self.cooldowns
            .insert((disciple_id, task_type), self.tick + duration);
    }

    fn is_on_cooldown(&self, disciple_id: u64, task_type: TaskType) -> bool {
        self.cooldowns
            .get(&(disciple_id, task_type))
            .map(|until| *until > self.tick)
            .unwrap_or(false)
    }

    fn clear_expired_cooldowns(&mut self) {
        let tick = self.tick;
        self.cooldowns.retain(|_, until| *until > tick);
    }
}

impl Task {
    fn new(
        task_type: TaskType,
        required_rank: DiscipleRank,
        duration_ticks: u32,
        target_id: Option<u64>,
        priority_base: f32,
        tick: u64,
    ) -> Self {
        Self {
            task_type,
            target_id,
            duration_ticks,
            priority_base,
            required_rank,
            cooldown_until: tick,
        }
    }
}

fn consider_task(best: &mut Option<(Task, f32)>, task: Task, score: f32) {
    match best {
        Some((_existing, existing_score)) if *existing_score >= score => {}
        _ => *best = Some((task, score)),
    }
}

fn rank_value(rank: &DiscipleRank) -> u8 {
    match rank {
        DiscipleRank::Outer => 0,
        DiscipleRank::Inner => 1,
        DiscipleRank::Elder => 2,
        DiscipleRank::SectLeader => 3,
    }
}

fn is_inner_or_above(rank: &DiscipleRank) -> bool {
    rank_value(rank) >= rank_value(&DiscipleRank::Inner)
}

fn can_take_task(rank: &DiscipleRank, task: &Task) -> bool {
    match task.task_type {
        TaskType::WorkBuilding => *rank == DiscipleRank::Outer,
        TaskType::Cultivate => is_inner_or_above(rank),
        TaskType::Rest | TaskType::Eat | TaskType::Idle => true,
    }
}

fn apply_task_effect(
    disciple: &mut Disciple,
    task: &Task,
    buildings: &[Building],
) -> TaskResult {
    match task.task_type {
        TaskType::Rest => {
            disciple.needs.rest.restore_full();
            TaskResult::Success
        }
        TaskType::Eat => {
            disciple.needs.hunger.restore_full();
            TaskResult::Success
        }
        TaskType::Cultivate => {
            disciple.needs.qi.restore_full();
            TaskResult::Success
        }
        TaskType::Idle => {
            disciple.needs.morale.restore_full();
            TaskResult::Success
        }
        TaskType::WorkBuilding => {
            if let Some(target_id) = task.target_id {
                let is_valid = is_target_valid(buildings, target_id, disciple.id);
                if is_valid {
                    TaskResult::Success
                } else {
                    TaskResult::Failed(TaskFailReason::TargetUnavailable)
                }
            } else {
                TaskResult::Failed(TaskFailReason::TargetUnavailable)
            }
        }
    }
}

fn is_target_valid(buildings: &[Building], target_id: u64, disciple_id: u64) -> bool {
    buildings.iter().any(|b| {
        b.id == target_id
            && b.status == BuildingStatus::Active
            && b.assigned_disciple == Some(disciple_id)
    })
}