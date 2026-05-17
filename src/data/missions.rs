use crate::data::disciples::FateTrait;
use serde::{Deserialize, Serialize};

/// The type of mission, which determines the relevant stat and trait modifiers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MissionType {
    Exploration,
    ResourceGathering,
    MonsterSuppression,
    Diplomacy,
    RuinDelve,
}

/// Which attribute is most relevant for a given mission type.
#[derive(Clone, Copy, Debug)]
pub enum RelevantStat {
    Body,
    Mind,
    Spirit,
}

impl MissionType {
    /// Returns the primary stat for calculating mission success.
    pub fn get_relevant_stat(&self) -> RelevantStat {
        match self {
            MissionType::MonsterSuppression | MissionType::RuinDelve => RelevantStat::Body,
            MissionType::Diplomacy => RelevantStat::Mind,
            MissionType::Exploration | MissionType::ResourceGathering => RelevantStat::Spirit,
        }
    }

    /// Extracts the relevant trait modifier from a FateTrait for this mission type.
    pub fn get_trait_modifier(&self, trait_: &FateTrait) -> f32 {
        match self {
            MissionType::MonsterSuppression | MissionType::RuinDelve => trait_.combat_modifier,
            MissionType::Diplomacy => trait_.diplomacy_modifier,
            MissionType::Exploration | MissionType::ResourceGathering => {
                trait_.exploration_modifier
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mission {
    pub mission_type: MissionType,
    pub duration: u32, // in game ticks
    pub description: String,
    pub danger_level: u32,
    #[serde(default)]
    pub repeatable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapNode {
    pub id: String,
    pub name: String,
    pub danger_level: u32,
    #[serde(default)]
    pub corruption: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OngoingMission {
    pub mission: Mission,
    pub disciple_indices: Vec<usize>,
    pub ticks_remaining: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MissionOutcome {
    pub success: bool,
    pub mission_name: String,
    pub description: String,
    pub logs: Vec<String>,
    pub rewards: MissionRewards,
    pub disciple_indices: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MissionRewards {
    pub spirit_stones: u32,
    pub disciple_exp: u32,
    #[serde(default)]
    pub herbs: u32,
    #[serde(default)]
    pub influence: u32,
    #[serde(default)]
    pub relics: u32,
    #[serde(default)]
    pub items: Vec<(String, u32)>, // Item ID, Amount
    #[serde(default)]
    pub recipe_discoveries: Vec<String>, // Recipe IDs discovered
}
