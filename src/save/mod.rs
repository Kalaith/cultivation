use crate::data::{
    buildings::Building,
    disciples::Disciple,
    history::DeceasedDisciple,
    missions::{MissionOutcome, OngoingMission},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub spirit_stones: u32,
    #[serde(default)]
    pub herbs: u32,
    pub disciples: Vec<Disciple>,
    #[serde(default)]
    pub deceased_disciples: Vec<DeceasedDisciple>,
    pub buildings: Vec<Building>,
    pub ongoing_missions: Vec<OngoingMission>,
    pub completed_missions: Vec<MissionOutcome>,
    pub tick: u64,
}
