use crate::data::{
    buildings::Building,
    disciples::Disciple,
    missions::OngoingMission,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub spirit_stones: u32,
    pub disciples: Vec<Disciple>,
    pub buildings: Vec<Building>,
    pub ongoing_missions: Vec<OngoingMission>,
    pub tick: u64,
}
