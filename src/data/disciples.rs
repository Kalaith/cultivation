use serde::{Deserialize, Serialize};
use crate::data::bloodlines::DiscipleBloodline;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Talent {
    Low,
    Medium,
    High,
    Genius,
    HeavenSent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    pub body: u32,
    pub mind: u32,
    pub spirit: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FateTrait {
    pub name: String,
    pub description: String,
    /// Modifier to breakthrough success chance (e.g., 0.2 = +20%)
    #[serde(default)]
    pub breakthrough_modifier: f32,
    /// Modifier to injury/death chance (e.g., 0.15 = +15% injury chance)
    #[serde(default)]
    pub injury_modifier: f32,
    /// Modifier to combat mission success (MonsterSuppression, RuinDelve)
    #[serde(default)]
    pub combat_modifier: f32,
    /// Modifier to diplomacy mission success
    #[serde(default)]
    pub diplomacy_modifier: f32,
    /// Modifier to exploration/resource mission success
    #[serde(default)]
    pub exploration_modifier: f32,
    /// Modifier to cultivation speed (e.g. 0.1 = +10% exp gain)
    #[serde(default)]
    pub cultivation_speed_modifier: f32,
    /// Modifier to work speed (for base tasks)
    #[serde(default)]
    pub work_speed_modifier: f32,
    /// If true, this character cannot die from breakthroughs or missions - only injured
    #[serde(default)]
    pub survivor: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscipleRank {
    Outer,
    Inner,
    Elder,
    SectLeader,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Disciple {
    /// Unique identifier for the disciple
    #[serde(default)]
    pub id: u64,
    pub name: String,
    pub rank: DiscipleRank,
    /// ID of the cultivation stage (references stages.json)
    pub realm: String,
    /// Current sub-stage index (0-indexed)
    #[serde(default)]
    pub sub_stage: usize,
    pub talent: Talent,
    pub attributes: Attributes,
    pub loyalty: u32,
    pub fate_traits: Vec<FateTrait>,
    pub exp: u32,
    pub exp_to_next_level: u32,
    /// Current spiritual energy (only used by Inner+ disciples)
    #[serde(default)]
    pub qi: u32,
    /// Maximum spiritual energy capacity
    #[serde(default)]
    pub max_qi: u32,
    #[serde(default)]
    pub law_id: Option<String>,
    /// Bloodline inheritance and awakening state
    #[serde(default)]
    pub bloodline: DiscipleBloodline,
}

impl Disciple {
    pub fn promote(&mut self) {
        if self.rank == DiscipleRank::Outer {
            self.rank = DiscipleRank::Inner;
            // Initial Qi bonus or unlock
            self.max_qi = 100; 
            self.qi = 100;
        }
    }
}
