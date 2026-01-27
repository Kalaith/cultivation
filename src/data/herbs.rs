use serde::{Deserialize, Serialize};
use crate::data::elements::Element;
use crate::data::items::ItemEffect;

/// Seasons affect which herbs can grow
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    /// Get the next season in cycle
    pub fn next(&self) -> Season {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Season::Spring => write!(f, "Spring"),
            Season::Summer => write!(f, "Summer"),
            Season::Autumn => write!(f, "Autumn"),
            Season::Winter => write!(f, "Winter"),
        }
    }
}

/// Herb tier affects potency and rarity
/// Tier 1: Common - Farmable, wild gathering
/// Tier 2: Uncommon - Requires Spirit Garden
/// Tier 3: Rare - Requires upgraded garden + conditions
/// Tier 4: Precious - Mission rewards, special events
/// Tier 5: Legendary - Unique finds, plot rewards
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Herb {
    pub id: String,
    pub name: String,
    pub description: String,
    pub element: Element,
    pub tier: u32,
    /// Effects when consumed directly (50% efficiency) or used in alchemy
    #[serde(default)]
    pub effects: Vec<ItemEffect>,
    /// Seasons during which this herb can grow
    pub grow_seasons: Vec<Season>,
    /// Ticks required to grow from seed to harvest
    pub grow_time_ticks: u32,
}

/// State of a growing herb in a garden plot
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrowingHerb {
    pub herb_id: String,
    pub ticks_remaining: u32,
    pub quality: f32, // 0.0-2.0, affected by disciple Spirit stat
}

impl GrowingHerb {
    pub fn new(herb_id: String, grow_time: u32) -> Self {
        Self {
            herb_id,
            ticks_remaining: grow_time,
            quality: 1.0,
        }
    }

    /// Check if herb is ready to harvest
    pub fn is_mature(&self) -> bool {
        self.ticks_remaining == 0
    }
}

/// Herb plot in a garden building
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HerbPlot {
    pub growing: Option<GrowingHerb>,
    /// Ticks since harvest - used for decay if not collected
    pub decay_ticks: u32,
}

impl Default for HerbPlot {
    fn default() -> Self {
        Self {
            growing: None,
            decay_ticks: 0,
        }
    }
}

/// Calculate elemental quality bonus for alchemy
/// Returns multiplier: 1.0 = neutral, 1.2 = matching, 1.1 = fed, 0.8 = suppressed
pub fn calculate_elemental_bonus(herb_element: &Element, recipe_element: &Element) -> f32 {
    use crate::data::elements::InteractionResult;

    if herb_element == recipe_element {
        1.2 // Matching element: +20% potency
    } else {
        match herb_element.get_interaction(recipe_element) {
            InteractionResult::Feeds => 1.1,     // Fed by herb: +10% potency
            InteractionResult::Suppresses => 0.8, // Suppressed: -20% potency
            InteractionResult::Neutral => 1.0,
        }
    }
}

/// Direct consumption efficiency (compared to refined pills)
pub const DIRECT_CONSUMPTION_EFFICIENCY: f32 = 0.5;

/// Decay rate for raw herbs per season (10%)
pub const RAW_HERB_DECAY_RATE: f32 = 0.10;

/// Decay rate for dried herbs per season (2%)
pub const DRIED_HERB_DECAY_RATE: f32 = 0.02;
