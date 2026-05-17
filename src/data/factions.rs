use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of faction in the world
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionType {
    Sect,
    MerchantGuild,
    DemonCult,
    ImperialCourt,
    BanditClan,
    BeastHorde,
}

impl std::fmt::Display for FactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactionType::Sect => write!(f, "Sect"),
            FactionType::MerchantGuild => write!(f, "Merchant Guild"),
            FactionType::DemonCult => write!(f, "Demon Cult"),
            FactionType::ImperialCourt => write!(f, "Imperial Court"),
            FactionType::BanditClan => write!(f, "Bandit Clan"),
            FactionType::BeastHorde => write!(f, "Beast Horde"),
        }
    }
}

/// Faction specialty determines their strengths and focus
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionSpecialty {
    Trade,
    Combat,
    Alchemy,
    Formations,
    Intelligence,
    Agriculture,
}

impl std::fmt::Display for FactionSpecialty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactionSpecialty::Trade => write!(f, "Trade"),
            FactionSpecialty::Combat => write!(f, "Combat"),
            FactionSpecialty::Alchemy => write!(f, "Alchemy"),
            FactionSpecialty::Formations => write!(f, "Formations"),
            FactionSpecialty::Intelligence => write!(f, "Intelligence"),
            FactionSpecialty::Agriculture => write!(f, "Agriculture"),
        }
    }
}

/// A faction in the world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Faction {
    pub id: String,
    pub name: String,
    pub faction_type: FactionType,
    /// Map node IDs this faction controls
    #[serde(default)]
    pub territory_nodes: Vec<String>,
    /// Military/cultivation strength
    pub power_level: u32,
    /// Economic resources
    #[serde(default)]
    pub wealth: u32,
    /// Strongest cultivator realm in the faction
    #[serde(default)]
    pub leader_realm: String,
    /// Base attitude toward player (-100 hostile to +100 friendly)
    #[serde(default)]
    pub disposition: i32,
    /// What this faction specializes in
    pub specialty: FactionSpecialty,
    /// Description of the faction
    #[serde(default)]
    pub description: String,
}

impl Faction {
    /// Check if faction is generally hostile based on disposition
    pub fn is_hostile(&self) -> bool {
        self.disposition < -30
    }

    /// Check if faction is generally friendly based on disposition
    pub fn is_friendly(&self) -> bool {
        self.disposition > 30
    }
}

/// AI goals for faction behavior
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionGoal {
    Expand,
    Defend,
    Trade,
    Consolidate,
    War,
}

/// Autonomous actions factions can take
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FactionAction {
    DeclareWar { target: String },
    ProposeAlliance { target: String },
    ExpandTerritory { node_id: String },
    RaidTradeRoute { route_id: String },
    HostFestival,
    TrainForces,
    GatherResources,
}

/// Faction AI state for autonomous behavior
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionAI {
    pub faction_id: String,
    pub goals: Vec<FactionGoal>,
    /// 0.0-1.0, affects war likelihood
    pub aggression: f32,
    /// Allied faction IDs
    #[serde(default)]
    pub alliances: Vec<String>,
    /// Enemy faction IDs
    #[serde(default)]
    pub enemies: Vec<String>,
    /// Last tick this AI made a decision
    #[serde(default)]
    pub last_action_tick: u64,
}

impl Default for FactionAI {
    fn default() -> Self {
        Self {
            faction_id: String::new(),
            goals: vec![FactionGoal::Consolidate],
            aggression: 0.3,
            alliances: Vec::new(),
            enemies: Vec::new(),
            last_action_tick: 0,
        }
    }
}

impl FactionAI {
    pub fn new(faction_id: String) -> Self {
        Self {
            faction_id,
            ..Default::default()
        }
    }

    /// Decide what action to take based on current state
    pub fn decide_action(
        &self,
        own_power: u32,
        neighbor_powers: &HashMap<String, u32>,
        current_tick: u64,
    ) -> Option<FactionAction> {
        // Only act every 3000 ticks (~50 seconds)
        if current_tick - self.last_action_tick < 3000 {
            return None;
        }

        // Find strongest and weakest neighbors
        let (strongest_id, strongest_power) = neighbor_powers
            .iter()
            .max_by_key(|(_, p)| *p)
            .map(|(id, p)| (id.clone(), *p))
            .unwrap_or_default();

        let (weakest_id, weakest_power) = neighbor_powers
            .iter()
            .filter(|(id, _)| !self.alliances.contains(id))
            .min_by_key(|(_, p)| *p)
            .map(|(id, p)| (id.clone(), *p))
            .unwrap_or_default();

        // Power comparison ratios
        let stronger_than_weakest =
            weakest_power > 0 && own_power as f32 / weakest_power as f32 > 1.3;
        let weaker_than_strongest =
            strongest_power > 0 && (own_power as f32 / strongest_power as f32) < 0.7;

        // Decision logic based on goals and power
        if self.goals.contains(&FactionGoal::War) && stronger_than_weakest && self.aggression > 0.5
        {
            return Some(FactionAction::DeclareWar { target: weakest_id });
        }

        if self.goals.contains(&FactionGoal::Expand) && stronger_than_weakest {
            // Look for uncontrolled or weakly held territory
            return Some(FactionAction::TrainForces);
        }

        if weaker_than_strongest && !self.alliances.contains(&strongest_id) {
            return Some(FactionAction::ProposeAlliance {
                target: strongest_id,
            });
        }

        if self.goals.contains(&FactionGoal::Trade) {
            return Some(FactionAction::GatherResources);
        }

        if self.goals.contains(&FactionGoal::Consolidate) {
            return Some(FactionAction::TrainForces);
        }

        None
    }
}
