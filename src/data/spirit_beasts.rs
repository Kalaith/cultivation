use serde::{Deserialize, Serialize};

use crate::data::elements::Element;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BeastTier {
    Mortal,
    Body,
    Qi,
    Foundation,
}

impl Default for BeastTier {
    fn default() -> Self {
        BeastTier::Mortal
    }
}

impl std::fmt::Display for BeastTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeastTier::Mortal => write!(f, "Mortal"),
            BeastTier::Body => write!(f, "Body Tempering"),
            BeastTier::Qi => write!(f, "Qi Gathering"),
            BeastTier::Foundation => write!(f, "Foundation"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BeastEquipmentSlot {
    Collar,
    Harness,
    Talisman,
    Relic,
}

impl std::fmt::Display for BeastEquipmentSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeastEquipmentSlot::Collar => write!(f, "Collar"),
            BeastEquipmentSlot::Harness => write!(f, "Harness"),
            BeastEquipmentSlot::Talisman => write!(f, "Talisman"),
            BeastEquipmentSlot::Relic => write!(f, "Relic"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BeastStats {
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub qi: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpiritBeastDefinition {
    pub id: String,
    pub name: String,
    pub species: String,
    #[serde(default)]
    pub element: Element,
    #[serde(default)]
    pub base_stats: BeastStats,
    #[serde(default)]
    pub growth_rate: f32,
    #[serde(default)]
    pub traits: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeastEquipmentItem {
    pub id: String,
    pub name: String,
    pub slot: BeastEquipmentSlot,
    #[serde(default)]
    pub tier_required: BeastTier,
    #[serde(default)]
    pub stats: BeastStats,
    #[serde(default)]
    pub beast_only: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquippedBeastItem {
    pub item_id: String,
    pub slot: BeastEquipmentSlot,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpiritBeast {
    #[serde(default)]
    pub id: u64,
    pub name: String,
    pub species: String,
    #[serde(default)]
    pub tier: BeastTier,
    #[serde(default)]
    pub element: Element,
    #[serde(default)]
    pub stats: BeastStats,
    #[serde(default)]
    pub loyalty: i32,
    #[serde(default)]
    pub hunger: i32,
    #[serde(default)]
    pub growth: f32,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub abilities: Vec<String>,
    #[serde(default)]
    pub equipment: Vec<EquippedBeastItem>,
}

impl SpiritBeast {
    pub fn new_from_definition(def: &SpiritBeastDefinition) -> Self {
        Self {
            id: 0,
            name: def.name.clone(),
            species: def.species.clone(),
            tier: BeastTier::Mortal,
            element: def.element.clone(),
            stats: def.base_stats.clone(),
            loyalty: 0,
            hunger: 0,
            growth: 0.0,
            traits: def.traits.clone(),
            abilities: Vec::new(),
            equipment: Vec::new(),
        }
    }

    pub fn can_equip(&self, item: &BeastEquipmentItem) -> bool {
        item.beast_only && item.tier_required <= self.tier
    }
}
