use serde::{Deserialize, Serialize};
use crate::data::elements::Element;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BloodlineRarity {
    Mortal,
    Spirit,
    Ancient,
    Primordial,
    Mythic,
}

impl Default for BloodlineRarity {
    fn default() -> Self {
        BloodlineRarity::Mortal
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BloodlineEffects {
    #[serde(default)]
    pub body_bonus: i32,
    #[serde(default)]
    pub mind_bonus: i32,
    #[serde(default)]
    pub spirit_bonus: i32,
    #[serde(default)]
    pub hp_multiplier: f32,
    #[serde(default)]
    pub cultivation_speed_modifier: f32,
    #[serde(default)]
    pub breakthrough_modifier: f32,
    #[serde(default)]
    pub injury_modifier: f32,
    #[serde(default)]
    pub combat_modifier: f32,
    #[serde(default)]
    pub diplomacy_modifier: f32,
    #[serde(default)]
    pub exploration_modifier: f32,
    #[serde(default)]
    pub work_speed_modifier: f32,
    #[serde(default)]
    pub lifespan_multiplier: f32,
    #[serde(default)]
    pub tribulation_resistance: f32,
    #[serde(default)]
    pub survivor: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bloodline {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: BloodlineRarity,
    #[serde(default)]
    pub element: Element,
    pub passive_effects: BloodlineEffects,
    #[serde(default)]
    pub awakening_chance: f32,
    #[serde(default)]
    pub realm_skip: bool,
    #[serde(default)]
    pub realm_skip_chance: f32,
    #[serde(default)]
    pub awakened_ability: Option<String>,
    #[serde(default)]
    pub corrupted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AwakeningState {
    #[default]
    Dormant,
    Partial,
    Full,
    Ancestor,
}

/// A disciple's bloodline instance with awakening progress
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DiscipleBloodline {
    pub bloodline_id: Option<String>,
    pub awakening_state: AwakeningState,
    pub awakening_progress: f32,
}

impl DiscipleBloodline {
    pub fn new(bloodline_id: String) -> Self {
        Self {
            bloodline_id: Some(bloodline_id),
            awakening_state: AwakeningState::Dormant,
            awakening_progress: 0.0,
        }
    }

    pub fn none() -> Self {
        Self {
            bloodline_id: None,
            awakening_state: AwakeningState::Dormant,
            awakening_progress: 0.0,
        }
    }

    /// Returns the effectiveness multiplier based on awakening state
    pub fn effectiveness(&self) -> f32 {
        match self.awakening_state {
            AwakeningState::Dormant => 0.25,   // 25% of passive effects
            AwakeningState::Partial => 0.5,    // 50% of passive effects
            AwakeningState::Full => 1.0,       // 100% of passive effects
            AwakeningState::Ancestor => 1.5,   // 150% of passive effects
        }
    }
}
