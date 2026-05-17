use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StatMultipliers {
    #[serde(default = "default_one")]
    pub body: f32,
    #[serde(default = "default_one")]
    pub mind: f32,
    #[serde(default = "default_one")]
    pub spirit: f32,
}

fn default_one() -> f32 {
    1.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubStage {
    pub name: String,
    #[serde(default = "default_one")]
    pub stats_modifier: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StageDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub base_hp: u32,
    pub base_qi: u32,
    #[serde(default)]
    pub stat_multipliers: StatMultipliers,
    #[serde(default)]
    pub sub_stages: Vec<SubStage>,
}
