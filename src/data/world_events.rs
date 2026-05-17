use crate::data::herbs::Season;
use serde::{Deserialize, Serialize};

/// Type of world event
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldEventType {
    SeasonalFestival,
    NaturalDisaster,
    FactionConflict,
    TreasureAppearance,
    DemonIncursion,
    ImperialDecree,
    MerchantCaravan,
    WanderingMaster,
    ResourceBoom,
    BeastTide,
    SpiritVeinDiscovery,
    AncientRuinOpened,
}

impl std::fmt::Display for WorldEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldEventType::SeasonalFestival => write!(f, "Seasonal Festival"),
            WorldEventType::NaturalDisaster => write!(f, "Natural Disaster"),
            WorldEventType::FactionConflict => write!(f, "Faction Conflict"),
            WorldEventType::TreasureAppearance => write!(f, "Treasure Appearance"),
            WorldEventType::DemonIncursion => write!(f, "Demon Incursion"),
            WorldEventType::ImperialDecree => write!(f, "Imperial Decree"),
            WorldEventType::MerchantCaravan => write!(f, "Merchant Caravan"),
            WorldEventType::WanderingMaster => write!(f, "Wandering Master"),
            WorldEventType::ResourceBoom => write!(f, "Resource Boom"),
            WorldEventType::BeastTide => write!(f, "Beast Tide"),
            WorldEventType::SpiritVeinDiscovery => write!(f, "Spirit Vein Discovery"),
            WorldEventType::AncientRuinOpened => write!(f, "Ancient Ruin Opened"),
        }
    }
}

/// What triggers an event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventTrigger {
    /// Triggered when season changes to this season
    SeasonChange(Season),
    /// Triggered when reputation with faction reaches threshold
    ReputationThreshold { faction_id: String, threshold: i32 },
    /// Triggered every N ticks
    TickInterval(u64),
    /// Random chance per tick (0.0-1.0)
    RandomChance(f32),
    /// Triggered when faction power balance shifts significantly
    FactionPowerShift,
    /// Triggered when player reaches certain influence level
    InfluenceThreshold(u32),
    /// Manual trigger (for story events)
    Manual,
}

/// Effects that events can cause
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventEffect {
    /// Modify relation with a faction
    ModifyRelation { faction_id: String, delta: i32 },
    /// Modify market prices temporarily
    ModifyPrices {
        item_id: String,
        modifier: f32,
        duration_ticks: u32,
    },
    /// Spawn a new mission
    SpawnMission { mission_id: String },
    /// Modify a resource
    ModifyResource { resource: ResourceType, delta: i32 },
    /// Trigger combat encounter
    TriggerCombat {
        enemy_power: u32,
        description: String,
    },
    /// Unlock a technology
    UnlockTech { tech_id: String },
    /// Modify corruption at a map node
    ModifyCorruption { node_id: String, delta: i32 },
    /// Change faction territory
    ChangeFactionTerritory {
        faction_id: String,
        node_id: String,
        gain: bool,
    },
    /// Add an item to player inventory
    GiveItem { item_id: String, amount: u32 },
    /// Modify cultivation speed globally
    ModifyCultivationSpeed { modifier: f32, duration_ticks: u32 },
    /// Trigger another event
    ChainEvent { event_id: String, delay_ticks: u32 },
}

/// Resource types for event effects
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    SpiritStones,
    Influence,
    Relics,
}

/// Requirement for choosing an event option
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChoiceRequirement {
    MinSpiritStones(u32),
    MinInfluence(u32),
    MinReputation { faction_id: String, min_rep: i32 },
    HasItem { item_id: String, amount: u32 },
    HasTech(String),
    MinDiscipleCount(usize),
    MinDiscipleRealm(String),
}

impl ChoiceRequirement {
    pub fn description(&self) -> String {
        match self {
            ChoiceRequirement::MinSpiritStones(n) => format!("Requires {} Spirit Stones", n),
            ChoiceRequirement::MinInfluence(n) => format!("Requires {} Influence", n),
            ChoiceRequirement::MinReputation {
                faction_id,
                min_rep,
            } => {
                format!("Requires {} reputation with {}", min_rep, faction_id)
            }
            ChoiceRequirement::HasItem { item_id, amount } => {
                format!("Requires {} x{}", item_id, amount)
            }
            ChoiceRequirement::HasTech(tech) => format!("Requires tech: {}", tech),
            ChoiceRequirement::MinDiscipleCount(n) => format!("Requires {} disciples", n),
            ChoiceRequirement::MinDiscipleRealm(realm) => {
                format!("Requires {} realm disciple", realm)
            }
        }
    }
}

/// A choice the player can make in response to an event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventChoice {
    pub label: String,
    pub description: String,
    #[serde(default)]
    pub requirements: Vec<ChoiceRequirement>,
    pub effects: Vec<EventEffect>,
}

/// World event definition (loaded from JSON)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub event_type: WorldEventType,
    pub trigger: EventTrigger,
    #[serde(default)]
    pub effects: Vec<EventEffect>,
    /// Duration in ticks (None = instant)
    pub duration_ticks: Option<u32>,
    /// Player response options (empty = no choice, auto-resolves)
    #[serde(default)]
    pub choices: Vec<EventChoice>,
    /// Can this event trigger multiple times?
    #[serde(default = "default_true")]
    pub repeatable: bool,
    /// Minimum tick before this can trigger
    #[serde(default)]
    pub min_tick: u64,
}

fn default_true() -> bool {
    true
}

/// An event that is currently active in the world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveWorldEvent {
    pub event_id: String,
    pub name: String,
    pub description: String,
    pub event_type: WorldEventType,
    pub started_tick: u64,
    pub expires_tick: Option<u64>,
    pub effects: Vec<EventEffect>,
    pub choices: Vec<EventChoice>,
    pub resolved: bool,
    pub choice_made: Option<usize>,
}

impl ActiveWorldEvent {
    pub fn from_event(event: &WorldEvent, current_tick: u64) -> Self {
        Self {
            event_id: event.id.clone(),
            name: event.name.clone(),
            description: event.description.clone(),
            event_type: event.event_type.clone(),
            started_tick: current_tick,
            expires_tick: event.duration_ticks.map(|d| current_tick + d as u64),
            effects: event.effects.clone(),
            choices: event.choices.clone(),
            resolved: false,
            choice_made: None,
        }
    }

    pub fn is_expired(&self, current_tick: u64) -> bool {
        self.expires_tick
            .map(|t| current_tick >= t)
            .unwrap_or(false)
    }

    pub fn requires_choice(&self) -> bool {
        !self.choices.is_empty() && !self.resolved
    }

    pub fn resolve_with_choice(&mut self, choice_idx: usize) {
        if choice_idx < self.choices.len() {
            self.choice_made = Some(choice_idx);
            self.resolved = true;
        }
    }

    pub fn get_resolution_effects(&self) -> Vec<EventEffect> {
        if let Some(idx) = self.choice_made {
            if let Some(choice) = self.choices.get(idx) {
                return choice.effects.clone();
            }
        }
        // If no choice made or invalid, return base effects
        self.effects.clone()
    }
}

/// Event queued to trigger in the future
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueuedEvent {
    pub event_id: String,
    pub trigger_tick: u64,
}

/// Seasonal event weights configuration
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SeasonalEventWeights {
    pub spring: Vec<(WorldEventType, f32)>,
    pub summer: Vec<(WorldEventType, f32)>,
    pub autumn: Vec<(WorldEventType, f32)>,
    pub winter: Vec<(WorldEventType, f32)>,
}

impl SeasonalEventWeights {
    pub fn get_weights(&self, season: &Season) -> &[(WorldEventType, f32)] {
        match season {
            Season::Spring => &self.spring,
            Season::Summer => &self.summer,
            Season::Autumn => &self.autumn,
            Season::Winter => &self.winter,
        }
    }
}

/// Result from processing an event trigger check
#[derive(Clone, Debug)]
pub struct EventTriggerResult {
    pub event_id: String,
    pub should_trigger: bool,
    pub reason: String,
}
