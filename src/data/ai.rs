use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeedTuning {
    pub max: f32,
    pub hunger_decay: f32,
    pub hunger_urgent: f32,
    pub rest_decay: f32,
    pub rest_urgent: f32,
    pub qi_decay: f32,
    pub qi_urgent: f32,
    pub morale_decay: f32,
    pub morale_urgent: f32,
}

impl Default for NeedTuning {
    fn default() -> Self {
        Self {
            max: 100.0,
            hunger_decay: 0.02,
            hunger_urgent: 45.0,
            rest_decay: 0.015,
            rest_urgent: 40.0,
            qi_decay: 0.01,
            qi_urgent: 35.0,
            morale_decay: 0.005,
            morale_urgent: 30.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskTuning {
    pub eat_base: f32,
    pub eat_urgency_weight: f32,
    pub eat_duration: u32,
    pub rest_base: f32,
    pub rest_urgency_weight: f32,
    pub rest_duration: u32,
    pub morale_base: f32,
    pub morale_urgency_weight: f32,
    pub morale_duration: u32,
    pub cultivate_base: f32,
    pub cultivate_urgency_weight: f32,
    pub cultivate_duration: u32,
    pub work_base: f32,
    pub work_morale_weight: f32,
    pub work_duration: u32,
    pub idle_base: f32,
    pub idle_duration: u32,
}

impl Default for TaskTuning {
    fn default() -> Self {
        Self {
            eat_base: 80.0,
            eat_urgency_weight: 100.0,
            eat_duration: 60,
            rest_base: 70.0,
            rest_urgency_weight: 90.0,
            rest_duration: 120,
            morale_base: 50.0,
            morale_urgency_weight: 70.0,
            morale_duration: 60,
            cultivate_base: 30.0,
            cultivate_urgency_weight: 80.0,
            cultivate_duration: 180,
            work_base: 25.0,
            work_morale_weight: 10.0,
            work_duration: 180,
            idle_base: 1.0,
            idle_duration: 60,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiSchedulerTuning {
    pub needs: NeedTuning,
    pub tasks: TaskTuning,
    pub cooldown_on_fail: u64,
    pub reservation_grace: u64,
    pub timeout_grace: u64,
}

impl Default for AiSchedulerTuning {
    fn default() -> Self {
        Self {
            needs: NeedTuning::default(),
            tasks: TaskTuning::default(),
            cooldown_on_fail: 60,
            reservation_grace: 60,
            timeout_grace: 60,
        }
    }
}
