use serde::{Deserialize, Serialize};

/// Treaty types between player sect and factions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Treaty {
    NonAggression {
        expires_tick: u64,
    },
    TradeAgreement {
        discount_percent: u8,
        expires_tick: u64,
    },
    Alliance {
        mutual_defense: bool,
        expires_tick: u64,
    },
    Vassal {
        tribute_rate: u8,
    },
}

impl Treaty {
    /// Check if treaty has expired
    pub fn is_expired(&self, current_tick: u64) -> bool {
        match self {
            Treaty::NonAggression { expires_tick } => current_tick >= *expires_tick,
            Treaty::TradeAgreement { expires_tick, .. } => current_tick >= *expires_tick,
            Treaty::Alliance { expires_tick, .. } => current_tick >= *expires_tick,
            Treaty::Vassal { .. } => false, // Vassal treaties don't expire automatically
        }
    }

    /// Get remaining ticks until expiration (None for permanent treaties)
    pub fn ticks_remaining(&self, current_tick: u64) -> Option<u64> {
        match self {
            Treaty::NonAggression { expires_tick } => {
                if current_tick < *expires_tick {
                    Some(*expires_tick - current_tick)
                } else {
                    Some(0)
                }
            }
            Treaty::TradeAgreement { expires_tick, .. } => {
                if current_tick < *expires_tick {
                    Some(*expires_tick - current_tick)
                } else {
                    Some(0)
                }
            }
            Treaty::Alliance { expires_tick, .. } => {
                if current_tick < *expires_tick {
                    Some(*expires_tick - current_tick)
                } else {
                    Some(0)
                }
            }
            Treaty::Vassal { .. } => None,
        }
    }

    /// Get display name for the treaty type
    pub fn name(&self) -> &'static str {
        match self {
            Treaty::NonAggression { .. } => "Non-Aggression Pact",
            Treaty::TradeAgreement { .. } => "Trade Agreement",
            Treaty::Alliance { .. } => "Alliance",
            Treaty::Vassal { .. } => "Vassalage",
        }
    }
}

/// Diplomatic relations between player sect and a faction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionRelation {
    pub faction_id: String,
    /// Reputation from -100 (hated) to +100 (revered)
    pub reputation: i32,
    /// Active treaty if any
    pub treaty: Option<Treaty>,
    /// Last tick when interaction occurred
    #[serde(default)]
    pub last_interaction_tick: u64,
    /// Count of hostile actions taken
    #[serde(default)]
    pub hostile_actions: u32,
    /// Count of friendly actions taken
    #[serde(default)]
    pub friendly_actions: u32,
    /// Whether currently at war
    #[serde(default)]
    pub at_war: bool,
    /// Tick when war started (if at war)
    #[serde(default)]
    pub war_start_tick: Option<u64>,
}

impl FactionRelation {
    pub fn new(faction_id: String, initial_reputation: i32) -> Self {
        Self {
            faction_id,
            reputation: initial_reputation,
            treaty: None,
            last_interaction_tick: 0,
            hostile_actions: 0,
            friendly_actions: 0,
            at_war: false,
            war_start_tick: None,
        }
    }

    /// Modify reputation with clamping
    pub fn modify_reputation(&mut self, delta: i32) {
        self.reputation = (self.reputation + delta).clamp(-100, 100);
    }

    /// Check if faction is hostile (reputation below -50)
    pub fn is_hostile(&self) -> bool {
        self.reputation < -50 || self.at_war
    }

    /// Check if faction is friendly (reputation above 50)
    pub fn is_friendly(&self) -> bool {
        self.reputation > 50 && !self.at_war
    }

    /// Get reputation tier for display
    pub fn reputation_tier(&self) -> ReputationTier {
        match self.reputation {
            r if r <= -75 => ReputationTier::Hated,
            r if r <= -50 => ReputationTier::Hostile,
            r if r <= -25 => ReputationTier::Unfriendly,
            r if r <= 25 => ReputationTier::Neutral,
            r if r <= 50 => ReputationTier::Friendly,
            r if r <= 75 => ReputationTier::Honored,
            _ => ReputationTier::Revered,
        }
    }

    /// Apply reputation drift toward faction's base disposition
    pub fn apply_drift(&mut self, base_disposition: i32, drift_rate: i32) {
        if self.reputation < base_disposition {
            self.reputation = (self.reputation + drift_rate).min(base_disposition);
        } else if self.reputation > base_disposition {
            self.reputation = (self.reputation - drift_rate).max(base_disposition);
        }
    }

    /// Start a war with this faction
    pub fn declare_war(&mut self, current_tick: u64) {
        self.at_war = true;
        self.war_start_tick = Some(current_tick);
        self.treaty = None; // War cancels all treaties
        self.hostile_actions += 1;
        self.modify_reputation(-30);
    }

    /// End the war
    pub fn end_war(&mut self) {
        self.at_war = false;
        self.war_start_tick = None;
    }

    /// Record a friendly action
    pub fn record_friendly_action(&mut self, current_tick: u64) {
        self.friendly_actions += 1;
        self.last_interaction_tick = current_tick;
    }

    /// Record a hostile action
    pub fn record_hostile_action(&mut self, current_tick: u64) {
        self.hostile_actions += 1;
        self.last_interaction_tick = current_tick;
    }
}

/// Display tier for reputation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReputationTier {
    Hated,
    Hostile,
    Unfriendly,
    Neutral,
    Friendly,
    Honored,
    Revered,
}

impl std::fmt::Display for ReputationTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReputationTier::Hated => write!(f, "Hated"),
            ReputationTier::Hostile => write!(f, "Hostile"),
            ReputationTier::Unfriendly => write!(f, "Unfriendly"),
            ReputationTier::Neutral => write!(f, "Neutral"),
            ReputationTier::Friendly => write!(f, "Friendly"),
            ReputationTier::Honored => write!(f, "Honored"),
            ReputationTier::Revered => write!(f, "Revered"),
        }
    }
}

impl ReputationTier {
    /// Get color for UI display (returns RGB tuple)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            ReputationTier::Hated => (180, 0, 0),
            ReputationTier::Hostile => (220, 50, 50),
            ReputationTier::Unfriendly => (200, 100, 50),
            ReputationTier::Neutral => (180, 180, 180),
            ReputationTier::Friendly => (100, 180, 100),
            ReputationTier::Honored => (50, 200, 50),
            ReputationTier::Revered => (50, 220, 150),
        }
    }
}

/// Diplomatic actions the player can take
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DiplomaticAction {
    SendGift { value: u32 },
    Threaten,
    RequestTreaty { treaty_type: TreatyRequest },
    BreakTreaty,
    DeclareWar,
    SuePeace,
    RequestAudience,
}

/// Treaty types that can be requested
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TreatyRequest {
    NonAggression {
        duration_ticks: u64,
    },
    TradeAgreement {
        discount_percent: u8,
        duration_ticks: u64,
    },
    Alliance {
        mutual_defense: bool,
        duration_ticks: u64,
    },
}
