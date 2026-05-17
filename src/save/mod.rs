//! Save/Load System
//!
//! Cross-platform save system supporting desktop (file) and WebGL (LocalStorage).

use crate::data::{
    buildings::Building,
    disciples::Disciple,
    grid::Grid,
    herbs::Season,
    history::DeceasedDisciple,
    missions::{MissionOutcome, OngoingMission},
    spirit_beasts::SpiritBeast,
};
use crate::engine::scheduler::SavedScheduler;
use crate::engine::world_sim::SavedWorldSim;
use serde::{Deserialize, Serialize};

/// Save file version for migration support
pub const SAVE_VERSION: u32 = 6;

/// Tutorial state for save/load
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct SavedTutorialState {
    pub active: bool,
    pub step: usize,
    pub hidden: bool,
}

impl SavedTutorialState {
    pub fn from_tutorial(tutorial: &crate::state::TutorialState) -> Self {
        Self {
            active: tutorial.active,
            step: tutorial.step,
            hidden: tutorial.hidden,
        }
    }

    pub fn to_tutorial(&self) -> crate::state::TutorialState {
        crate::state::TutorialState {
            active: self.active,
            step: self.step,
            hidden: self.hidden,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    /// Save file version for migration
    #[serde(default)]
    pub version: u32,

    // Grid and sect info
    #[serde(default)]
    pub grid: Option<Grid>,
    #[serde(default)]
    pub sect_name: String,

    // Resources
    pub spirit_stones: u32,
    #[serde(default)]
    pub herbs: u32,
    #[serde(default)]
    pub influence: u32,
    #[serde(default)]
    pub relics: u32,
    #[serde(default)]
    pub inventory: std::collections::HashMap<String, u32>,

    // Tech and disciples
    #[serde(default)]
    pub unlocked_techs: Vec<String>,
    pub disciples: Vec<Disciple>,
    #[serde(default)]
    pub deceased_disciples: Vec<DeceasedDisciple>,
    #[serde(default)]
    pub spirit_beasts: Vec<SpiritBeast>,

    // Buildings and missions
    pub buildings: Vec<Building>,
    pub ongoing_missions: Vec<OngoingMission>,
    pub completed_missions: Vec<MissionOutcome>,
    #[serde(default)]
    pub completed_history: Vec<String>,

    // Time
    pub tick: u64,

    // Season system (new in v2)
    #[serde(default = "default_season")]
    pub current_season: Season,
    #[serde(default = "default_season_ticks")]
    pub season_ticks: u32,

    // Tutorial state (new in v2)
    #[serde(default)]
    pub tutorial: SavedTutorialState,

    // World simulation state (new in v3)
    #[serde(default)]
    pub world_sim: Option<SavedWorldSim>,

    // AI scheduler state (new in v4)
    #[serde(default)]
    pub scheduler: Option<SavedScheduler>,

    // Discovered recipes (new in v5)
    #[serde(default)]
    pub discovered_recipes: Vec<String>,
}

fn default_season() -> Season {
    Season::Spring
}

fn default_season_ticks() -> u32 {
    3600
}

impl SaveData {
    /// Migrate old save data to current version
    pub fn migrate(mut self) -> Self {
        if self.version < 2 {
            // v1 -> v2: Add season and tutorial defaults
            self.current_season = Season::Spring;
            self.season_ticks = 3600;
            self.tutorial = SavedTutorialState::default();
        }
        if self.version < 3 {
            // v2 -> v3: Add world simulation (will be created fresh on load)
            self.world_sim = None;
        }
        if self.version < 4 {
            // v3 -> v4: Add scheduler state (will be created fresh on load)
            self.scheduler = None;
        }
        if self.version < 5 {
            // v4 -> v5: Add discovered recipes (start with basics)
            self.discovered_recipes = vec![
                "recipe_healing_pill".to_string(),
                "recipe_iron_sword".to_string(),
            ];
        }
        if self.version < 6 {
            // v5 -> v6: Add spirit beasts list
            self.spirit_beasts = Vec::new();
        }
        self.version = SAVE_VERSION;
        self
    }
}

/// Platform-agnostic save operations
pub mod storage {
    use super::SaveData;
    use macroquad_toolkit::persistence::{json_key_exists, load_json_key, save_json_key};

    const SAVE_FILE: &str = "savegame.json";
    const GAME_NAME: &str = "cultivation";

    /// Save game data to persistent storage
    pub fn save(data: &SaveData) -> Result<(), String> {
        save_json_key(GAME_NAME, SAVE_FILE, data)
    }

    /// Load game data from persistent storage
    pub fn load() -> Result<SaveData, String> {
        let save_data: SaveData = load_json_key(GAME_NAME, SAVE_FILE)?;
        Ok(save_data.migrate())
    }

    /// Check if a save exists
    pub fn exists() -> bool {
        json_key_exists(GAME_NAME, SAVE_FILE)
    }

    // The delete function has been removed as it was unused.
}
