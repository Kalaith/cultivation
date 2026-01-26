use serde::{Deserialize, Serialize};
use crate::data::buildings::BuildingType;
use crate::data::missions::MissionType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Technology {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost_spirit_stones: u32,
    #[serde(default)]
    pub unlocks_buildings: Vec<BuildingType>,
    #[serde(default)]
    pub unlocks_missions: Vec<MissionType>,
    #[serde(default)]
    pub prerequisites: Vec<String>, // IDs of other techs
}
