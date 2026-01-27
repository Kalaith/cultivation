use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuildingStatus {
    Ruined,
    Constructing,
    Active,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildingType {
    SectHall,
    Dormitory,
    TrainingYard,
    LibraryPavilion,
    MissionBoard,
    SpiritGarden,
    Decoration,
    AlchemyFurnace,
    ArtifactForge,
    // Herb system buildings
    HerbGarden,
    Greenhouse,
    DryingPavilion,
    HerbStorage,
}

impl std::fmt::Display for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildingType::SectHall => write!(f, "Sect Hall"),
            BuildingType::Dormitory => write!(f, "Dormitory"),
            BuildingType::TrainingYard => write!(f, "Training Yard"),
            BuildingType::LibraryPavilion => write!(f, "Library Pavilion"),
            BuildingType::MissionBoard => write!(f, "Mission Board"),
            BuildingType::SpiritGarden => write!(f, "Spirit Garden"),
            BuildingType::Decoration => write!(f, "Decoration"),
            BuildingType::AlchemyFurnace => write!(f, "Alchemy Furnace"),
            BuildingType::ArtifactForge => write!(f, "Artifact Forge"),
            BuildingType::HerbGarden => write!(f, "Herb Garden"),
            BuildingType::Greenhouse => write!(f, "Greenhouse"),
            BuildingType::DryingPavilion => write!(f, "Drying Pavilion"),
            BuildingType::HerbStorage => write!(f, "Herb Storage"),
        }
    }
}

use crate::data::elements::Element;
use crate::data::herbs::HerbPlot;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Building {
    #[serde(default)]
    pub id: u64,
    pub building_type: BuildingType,
    pub level: u32,
    #[serde(default = "default_element")]
    pub element: Element,
    #[serde(default = "default_element")]
    pub material_element: Element,
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    #[serde(default = "default_status")]
    pub status: BuildingStatus,
    #[serde(default)]
    pub feng_shui_score: f32,
    /// Disciple assigned to work this building (for gardens, pavilions, etc.)
    #[serde(default)]
    pub assigned_disciple: Option<u64>,
    /// Herb plots for HerbGarden and Greenhouse buildings
    #[serde(default)]
    pub herb_plots: Vec<HerbPlot>,
    /// Element infusion for Greenhouse (requires matching essence to maintain)
    #[serde(default)]
    pub infused_element: Option<Element>,
}

fn default_status() -> BuildingStatus {
    BuildingStatus::Active
}

fn default_element() -> Element {
    Element::None
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        // Initialize herb plots based on building type
        let herb_plots = match &building_type {
            BuildingType::HerbGarden => vec![HerbPlot::default()], // L1: 1 plot
            BuildingType::Greenhouse => vec![HerbPlot::default()], // L1: 1 plot
            _ => Vec::new(),
        };

        Self {
            id: 0, // Should be assigned by game logic
            building_type,
            level: 1,
            element: Element::None,
            material_element: Element::None,
            x: 0,
            y: 0,
            status: BuildingStatus::Active,
            feng_shui_score: 0.0,
            assigned_disciple: None,
            herb_plots,
            infused_element: None,
        }
    }

    /// Training Yard cultivation speed multiplier.
    /// Level 1 = 1.0x, Level 2 = 1.25x, Level 3 = 1.5x
    pub fn get_cultivation_multiplier(&self) -> f32 {
        if self.building_type == BuildingType::TrainingYard {
            1.0 + (self.level as f32 - 1.0) * 0.25
        } else {
            1.0
        }
    }

    /// Spirit Garden passive spirit stone income per tick.
    /// Level 1 = 1, Level 2 = 2, Level 3 = 4
    pub fn get_passive_income(&self) -> u32 {
        if self.building_type == BuildingType::SpiritGarden {
            match self.level {
                1 => 1,
                2 => 2,
                _ => 4,
            }
        } else {
            0
        }
    }

    /// Sect Hall max disciples allowed.
    /// Level 1 = 5, Level 2 = 10, Level 3 = 20
    pub fn get_max_disciples(&self) -> u32 {
        if self.building_type == BuildingType::SectHall {
            match self.level {
                1 => 5,
                2 => 10,
                _ => 20,
            }
        } else {
            0
        }
    }

    /// Dormitory additional disciple capacity.
    /// Level 1 = 4, Level 2 = 8, Level 3 = 12
    pub fn get_dorm_capacity(&self) -> u32 {
        if self.building_type == BuildingType::Dormitory {
            match self.level {
                1 => 4,
                2 => 8,
                _ => 12,
            }
        } else {
            0
        }
    }

    /// Get maximum number of herb plots for this building
    /// HerbGarden: L1=1, L2=2, L3=3
    /// Greenhouse: L1=1, L2=2, L3=3
    pub fn get_max_herb_plots(&self) -> usize {
        match self.building_type {
            BuildingType::HerbGarden | BuildingType::Greenhouse => self.level as usize,
            _ => 0,
        }
    }

    /// Get maximum herb tier this building can grow
    /// HerbGarden: Tier 1-2
    /// Greenhouse: Tier 2-3
    pub fn get_max_herb_tier(&self) -> u32 {
        match self.building_type {
            BuildingType::HerbGarden => 2,
            BuildingType::Greenhouse => 3,
            _ => 0,
        }
    }

    /// Get growth speed multiplier for herb buildings
    /// L3 HerbGarden: +25% growth speed
    pub fn get_growth_speed_multiplier(&self) -> f32 {
        match self.building_type {
            BuildingType::HerbGarden if self.level >= 3 => 1.25,
            _ => 1.0,
        }
    }

    /// Get decay reduction for Herb Storage
    /// L1: 50%, L2: 75%, L3: 90%
    pub fn get_decay_reduction(&self) -> f32 {
        if self.building_type == BuildingType::HerbStorage {
            match self.level {
                1 => 0.50,
                2 => 0.75,
                _ => 0.90,
            }
        } else {
            0.0
        }
    }

    /// Get drying efficiency for Drying Pavilion
    /// Base: 5 raw -> 4 dried (20% loss)
    /// Higher levels reduce loss: L2 = 15%, L3 = 10%
    pub fn get_drying_loss_rate(&self) -> f32 {
        if self.building_type == BuildingType::DryingPavilion {
            match self.level {
                1 => 0.20,
                2 => 0.15,
                _ => 0.10,
            }
        } else {
            0.20
        }
    }

    /// Ensure herb_plots matches building level (add plots when upgraded)
    pub fn sync_herb_plots(&mut self) {
        let max_plots = self.get_max_herb_plots();
        while self.herb_plots.len() < max_plots {
            self.herb_plots.push(HerbPlot::default());
        }
    }
}
