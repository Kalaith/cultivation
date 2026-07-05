use crate::data::ai::AiSchedulerTuning;
use crate::data::bloodlines::DiscipleBloodline;
use crate::data::items::EquipmentSlot;
use crate::data::loader::GameData;
use crate::data::missions::MissionType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeedState {
    pub current: f32,
    pub max: f32,
    pub decay_per_tick: f32,
    pub urgent_threshold: f32,
}

impl NeedState {
    pub fn decay(&mut self) {
        self.current = (self.current - self.decay_per_tick).max(0.0);
    }

    pub fn restore_full(&mut self) {
        self.current = self.max;
    }

    pub fn urgency(&self) -> f32 {
        if self.current >= self.urgent_threshold || self.urgent_threshold <= 0.0 {
            0.0
        } else {
            (self.urgent_threshold - self.current) / self.urgent_threshold
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscipleNeeds {
    pub hunger: NeedState,
    pub rest: NeedState,
    pub qi: NeedState,
    pub morale: NeedState,
}

impl DiscipleNeeds {
    pub fn decay_all(&mut self) {
        self.hunger.decay();
        self.rest.decay();
        self.qi.decay();
        self.morale.decay();
    }

    pub fn from_tuning(tuning: &AiSchedulerTuning) -> Self {
        let needs = &tuning.needs;
        Self {
            hunger: NeedState {
                current: needs.max,
                max: needs.max,
                decay_per_tick: needs.hunger_decay,
                urgent_threshold: needs.hunger_urgent,
            },
            rest: NeedState {
                current: needs.max,
                max: needs.max,
                decay_per_tick: needs.rest_decay,
                urgent_threshold: needs.rest_urgent,
            },
            qi: NeedState {
                current: needs.max,
                max: needs.max,
                decay_per_tick: needs.qi_decay,
                urgent_threshold: needs.qi_urgent,
            },
            morale: NeedState {
                current: needs.max,
                max: needs.max,
                decay_per_tick: needs.morale_decay,
                urgent_threshold: needs.morale_urgent,
            },
        }
    }
}

fn default_needs() -> DiscipleNeeds {
    DiscipleNeeds::from_tuning(&AiSchedulerTuning::default())
}

impl Default for DiscipleNeeds {
    fn default() -> Self {
        default_needs()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Talent {
    Low,
    Medium,
    High,
    Genius,
    HeavenSent,
}

impl std::fmt::Display for Talent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Talent::Low => write!(f, "Dim Spark"),
            Talent::Medium => write!(f, "Steady Root"),
            Talent::High => write!(f, "Bright Root"),
            Talent::Genius => write!(f, "Genius"),
            Talent::HeavenSent => write!(f, "Heaven-Sent"),
        }
    }
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

/// Type of injury sustained
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InjuryType {
    /// From failed breakthrough attempt
    QiDeviation,
    /// From combat/mission failure
    BattleWounds,
    /// From overexertion
    MeridianDamage,
}

impl std::fmt::Display for InjuryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InjuryType::QiDeviation => write!(f, "Qi Deviation"),
            InjuryType::BattleWounds => write!(f, "Battle Wounds"),
            InjuryType::MeridianDamage => write!(f, "Meridian Damage"),
        }
    }
}

/// Injury state for a disciple
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Injury {
    /// Type of injury
    pub injury_type: InjuryType,
    /// Severity 1-3: Minor, Moderate, Severe
    pub severity: u32,
    /// Ticks remaining until natural recovery
    pub recovery_ticks_remaining: u32,
    /// Description of the injury
    pub description: String,
}

impl Injury {
    /// Create a new injury from a failed breakthrough
    pub fn from_breakthrough(realm: &str, is_survivor: bool) -> Self {
        // Survivors get less severe injuries
        let severity = if is_survivor { 2 } else { 2 }; // Moderate by default

        // Base recovery time in seconds (1 heal tick = ~1 second)
        // Mortals recover faster (not dealing with Qi deviation, just exhaustion)
        // Higher realms = longer recovery due to Qi complexity
        let realm_multiplier = match realm {
            "Mortal" => 0.5,                  // ~5 sec per severity = 10 sec for moderate
            "QiRefinement" => 1.0,            // ~10 sec per severity = 20 sec for moderate
            "FoundationEstablishment" => 1.5, // 30 sec for moderate
            "CoreFormation" => 2.0,           // 40 sec for moderate
            "NascentSoul" => 2.5,             // 50 sec for moderate
            _ => 3.0,                         // 60 sec for moderate
        };

        let base_recovery = 10 * severity; // 10 seconds per severity level
        let recovery_ticks = (base_recovery as f32 * realm_multiplier) as u32;

        // Different description for mortals vs cultivators
        let description = if realm == "Mortal" {
            "Exhausted from failed breakthrough attempt".to_string()
        } else {
            "Suffered Qi deviation during breakthrough attempt".to_string()
        };

        Self {
            injury_type: if realm == "Mortal" {
                InjuryType::MeridianDamage
            } else {
                InjuryType::QiDeviation
            },
            severity,
            recovery_ticks_remaining: recovery_ticks,
            description,
        }
    }

    /// Create a new injury from combat/mission
    pub fn from_combat(severity: u32) -> Self {
        let recovery_ticks = 600 * severity;
        Self {
            injury_type: InjuryType::BattleWounds,
            severity: severity.clamp(1, 3),
            recovery_ticks_remaining: recovery_ticks,
            description: format!("Wounded in battle"),
        }
    }

    /// Get severity as a descriptive string
    pub fn severity_str(&self) -> &'static str {
        match self.severity {
            1 => "Minor",
            2 => "Moderate",
            _ => "Severe",
        }
    }

    /// Calculate natural recovery rate based on Body stat
    /// Returns ticks to recover per game tick
    pub fn get_recovery_rate(body: u32) -> u32 {
        // Base rate: 1 tick per tick
        // Body bonus: +1 per 20 Body
        1 + (body / 20)
    }

    /// Apply healing from an item (reduces recovery time)
    pub fn apply_healing(&mut self, healing_power: u32) {
        self.recovery_ticks_remaining = self.recovery_ticks_remaining.saturating_sub(healing_power);
    }

    /// Check if fully healed
    pub fn is_healed(&self) -> bool {
        self.recovery_ticks_remaining == 0
    }
}

/// A hidden bottleneck that blocks breakthrough until fulfilled
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Bottleneck {
    /// Must complete a mission of the given type
    CompleteMission(String),
    /// Must craft the given recipe
    CraftItem(String),
    /// Must reach a stat threshold
    ReachStat { stat: String, value: u32 },
    /// Must use a specific item
    UseItem(String),
    /// Must equip an item in the given slot
    EquipSlot(EquipmentSlot),
}

impl Bottleneck {
    /// Whether the sect can divine the exact nature of bottlenecks.
    /// Requires an active Library Pavilion.
    pub fn insight_unlocked(data: &GameData) -> bool {
        use crate::data::buildings::{BuildingStatus, BuildingType};
        data.buildings.iter().any(|b| {
            b.building_type == BuildingType::LibraryPavilion && b.status == BuildingStatus::Active
        })
    }

    /// Description shown to the player: exact once insight is unlocked,
    /// otherwise a vague hint pointing at the Library Pavilion.
    pub fn player_description(&self, data: &GameData) -> String {
        if Self::insight_unlocked(data) {
            self.description(data)
        } else {
            "An unseen obstruction bars the way. Raise the Library Pavilion to divine its nature."
                .to_string()
        }
    }

    pub fn description(&self, data: &GameData) -> String {
        match self {
            Bottleneck::CompleteMission(mt) => {
                let mission_name = match mt.as_str() {
                    "Exploration" => "Exploration",
                    "ResourceGathering" => "Resource Gathering",
                    "MonsterSuppression" => "Monster Suppression",
                    "Diplomacy" => "Diplomacy",
                    "RuinDelve" => "Ruin Delve",
                    other => other,
                };
                format!("Complete a {} dispatch", mission_name)
            }
            Bottleneck::CraftItem(recipe_id) => {
                let name = data
                    .recipes
                    .iter()
                    .find(|r| r.id == *recipe_id)
                    .map(|r| r.name.as_str())
                    .unwrap_or(recipe_id.as_str());
                format!("Craft {}", name)
            }
            Bottleneck::ReachStat { stat, value } => format!("Reach {} {}", value, stat),
            Bottleneck::UseItem(item_id) => {
                let name = data
                    .items
                    .get(item_id)
                    .map(|i| i.name.as_str())
                    .unwrap_or(item_id.as_str());
                format!("Use {}", name)
            }
            Bottleneck::EquipSlot(slot) => format!("Equip a {}", slot),
        }
    }
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
    /// Basic needs for scheduling and behavior
    #[serde(default = "default_needs")]
    pub needs: DiscipleNeeds,
    /// Current injury status (None = healthy)
    #[serde(default)]
    pub injury: Option<Injury>,
    /// Breakthrough readiness (0.0 = just reached threshold, 1.0 = 100% over threshold)
    /// Higher readiness improves breakthrough success chance
    #[serde(default)]
    pub breakthrough_readiness: f32,
    /// Equipped item IDs by slot
    #[serde(default)]
    pub equipment: std::collections::HashMap<EquipmentSlot, String>,
    /// Hidden bottleneck blocking breakthrough (if any)
    #[serde(default)]
    pub breakthrough_bottleneck: Option<Bottleneck>,
}

impl Disciple {
    /// Check if disciple is injured
    pub fn is_injured(&self) -> bool {
        self.injury.is_some()
    }

    /// Apply natural healing based on Body stat
    pub fn heal_tick(&mut self) {
        if let Some(ref mut injury) = self.injury {
            let rate = Injury::get_recovery_rate(self.attributes.body);
            injury.recovery_ticks_remaining = injury.recovery_ticks_remaining.saturating_sub(rate);

            if injury.is_healed() {
                self.injury = None;
            }
        }
    }

    /// Apply healing from item/herb
    pub fn apply_healing(&mut self, healing_power: u32) -> bool {
        if let Some(ref mut injury) = self.injury {
            injury.apply_healing(healing_power);
            if injury.is_healed() {
                self.injury = None;
            }
            true
        } else {
            false // Already healthy
        }
    }

    /// Inflict an injury
    pub fn injure(&mut self, injury: Injury) {
        self.injury = Some(injury);
    }

    /// Check if disciple can attempt breakthrough (has enough exp)
    pub fn can_attempt_breakthrough(&self) -> bool {
        self.exp >= self.exp_to_next_level && self.injury.is_none()
    }

    /// Update breakthrough readiness based on accumulated exp
    /// Called each cultivation tick when exp >= exp_to_next_level
    pub fn update_readiness(&mut self) {
        if self.exp >= self.exp_to_next_level {
            // Readiness is how much over the threshold we are
            // At 100% over (2x exp needed), readiness = 1.0
            let excess = self.exp.saturating_sub(self.exp_to_next_level);
            self.breakthrough_readiness = (excess as f32 / self.exp_to_next_level as f32).min(2.0);
        } else {
            self.breakthrough_readiness = 0.0;
        }
    }

    /// Get breakthrough success bonus from readiness (0.0 to 0.5)
    /// At readiness 0: no bonus
    /// At readiness 1.0 (100% over): +25% bonus
    /// At readiness 2.0 (200% over): +50% bonus (max)
    pub fn get_readiness_bonus(&self) -> f32 {
        (self.breakthrough_readiness * 0.25).min(0.5)
    }

    pub fn promote(&mut self) {
        if self.rank == DiscipleRank::Outer {
            self.rank = DiscipleRank::Inner;
            // Initial Qi bonus or unlock
            self.max_qi = 100;
            self.qi = 100;
            self.needs.qi.max = 100.0;
            self.needs.qi.current = 100.0;
        }
    }
}
