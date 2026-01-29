use serde::{Deserialize, Serialize};
use crate::data::buildings::BuildingType;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Resource,
    Pill,
    Artifact,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum EquipmentSlot {
    Weapon,
    OffHand,
    Chest,
    Legs,
    Arms,
    Head,
    Boots,
    Ring,
    Amulet,
    Belt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatModifier {
    pub stat: String,
    pub value: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentData {
    pub slot: EquipmentSlot,
    #[serde(default)]
    pub modifiers: Vec<StatModifier>,
    #[serde(default)]
    pub durability: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemEffect {
    Heal(u32),        // Reduce injury recovery time by this amount (ticks)
    BoostQi(u32),     // Add XP/Qi
    BoostBody(u32),   // Perm stat boost
    BoostMind(u32),
    BoostSpirit(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    #[serde(default)]
    pub effects: Vec<ItemEffect>,
    #[serde(default)]
    pub equipment: Option<EquipmentData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub output_item_id: String,
    pub output_amount: u32,
    pub ingredients: Vec<(String, u32)>, // Item ID (or "spirit_stones", "herbs"), Amount
    // Special case keys: "spirit_stones", "herbs" are hardcoded currencies, not items in inventory yet.
    // Or we map them? For simplicity, let's treat "herbs" as a currency field in Game, but maybe recipes reference them by string "herbs".
    pub craft_time: u32, // Ticks
    pub required_building: BuildingType,
}
