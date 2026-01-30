use serde::{Deserialize, Serialize};
use crate::data::buildings::BuildingType;
use crate::data::elements::Element;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Resource,
    Pill,
    Artifact,
    Potion,
    Talisman,
    Scroll,
    Armor,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemQuality {
    Common,
    Fine,
    Superior,
    Exquisite,
    Perfect,
}

impl Default for ItemQuality {
    fn default() -> Self {
        ItemQuality::Common
    }
}

impl std::fmt::Display for ItemQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemQuality::Common => write!(f, "Common"),
            ItemQuality::Fine => write!(f, "Fine"),
            ItemQuality::Superior => write!(f, "Superior"),
            ItemQuality::Exquisite => write!(f, "Exquisite"),
            ItemQuality::Perfect => write!(f, "Perfect"),
        }
    }
}

impl ItemQuality {
    pub fn tier(&self) -> u32 {
        match self {
            ItemQuality::Common => 0,
            ItemQuality::Fine => 1,
            ItemQuality::Superior => 2,
            ItemQuality::Exquisite => 3,
            ItemQuality::Perfect => 4,
        }
    }

    pub fn from_tier(tier: u32) -> Self {
        match tier {
            0 => ItemQuality::Common,
            1 => ItemQuality::Fine,
            2 => ItemQuality::Superior,
            3 => ItemQuality::Exquisite,
            _ => ItemQuality::Perfect,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecipeType {
    Alchemy,
    Blacksmith,
    Talisman,
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
    // Potion effects: temporary buffs (stat, value, duration_ticks)
    TemporaryBuff { stat: String, value: i32, duration_ticks: u32 },
    // Talisman effects
    DamageShield(u32),
    QiBurst(u32),
    ElementalStrike(Element, u32),
    // Mission speed boost (reduces mission duration by ticks)
    MissionSpeedBoost(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    #[serde(default)]
    pub quality: ItemQuality,
    #[serde(default)]
    pub element: Option<Element>,
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
    pub craft_time: u32, // Ticks
    pub required_building: BuildingType,
    #[serde(default)]
    pub recipe_type: RecipeType,
    #[serde(default = "default_difficulty")]
    pub difficulty: u32, // 1-100, affects success rate
    #[serde(default)]
    pub element_affinity: Option<Element>,
}

fn default_difficulty() -> u32 {
    30
}

impl Default for RecipeType {
    fn default() -> Self {
        RecipeType::Alchemy
    }
}
