use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MissionType {
    Exploration,
    ResourceGathering,
    MonsterSuppression,
    Diplomacy,
    RuinDelve,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mission {
    pub mission_type: MissionType,
    pub duration: u32, // in game ticks
    pub description: String,
    pub danger_level: u32,
    // We can add fields for rewards, thresholds, etc. later
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapNode {
    pub id: String,
    pub name: String,
    pub danger_level: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OngoingMission {
    pub mission: Mission,
    pub disciple_indices: Vec<usize>,
    pub ticks_remaining: u32,
}