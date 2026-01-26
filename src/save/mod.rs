use crate::data::{
    buildings::Building,
    disciples::Disciple,
    history::DeceasedDisciple,
    missions::{MissionOutcome, OngoingMission},
    grid::Grid, // Import Grid
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    #[serde(default)]
    pub grid: Option<Grid>, // Optional for backward compatibility with old saves
    #[serde(default)]
    pub sect_name: String,
    pub spirit_stones: u32,
    #[serde(default)]
    pub herbs: u32,
    #[serde(default)]
    pub influence: u32,
    #[serde(default)]
    pub relics: u32,
    #[serde(default)]
    pub inventory: std::collections::HashMap<String, u32>,
    #[serde(default)]
    pub unlocked_techs: Vec<String>,
    pub disciples: Vec<Disciple>,
    #[serde(default)]
    pub deceased_disciples: Vec<DeceasedDisciple>,
    pub buildings: Vec<Building>,
    pub ongoing_missions: Vec<OngoingMission>,
    pub completed_missions: Vec<MissionOutcome>,
    #[serde(default)]
    pub completed_history: Vec<String>,
    pub tick: u64,
}
