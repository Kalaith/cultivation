use serde::{Deserialize, Serialize};
use crate::data::disciples::Attributes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubStage {
    pub name: String,
    #[serde(default = "default_modifier")]
    pub stats_modifier: f32,
}

fn default_modifier() -> f32 { 1.0 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StageDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub base_hp: u32,
    pub base_qi: u32,
    pub stat_multipliers: Attributes,
    #[serde(default)]
    pub sub_stages: Vec<SubStage>,
}
