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
    TrainingYard,
    LibraryPavilion,
    MissionBoard,
    SpiritGarden,
    Decoration,
    AlchemyFurnace,
    ArtifactForge,
}

impl std::fmt::Display for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildingType::SectHall => write!(f, "Sect Hall"),
            BuildingType::TrainingYard => write!(f, "Training Yard"),
            BuildingType::LibraryPavilion => write!(f, "Library Pavilion"),
            BuildingType::MissionBoard => write!(f, "Mission Board"),
            BuildingType::SpiritGarden => write!(f, "Spirit Garden"),
            BuildingType::Decoration => write!(f, "Decoration"),
            BuildingType::AlchemyFurnace => write!(f, "Alchemy Furnace"),
            BuildingType::ArtifactForge => write!(f, "Artifact Forge"),
        }
    }
}

use crate::data::elements::Element;

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
}

fn default_status() -> BuildingStatus {
    BuildingStatus::Active
}

fn default_element() -> Element {
    Element::None
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
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
}
