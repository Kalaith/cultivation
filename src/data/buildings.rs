use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildingType {
    SectHall,
    TrainingYard,
    LibraryPavilion,
    MissionBoard,
    SpiritGarden,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u32,
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        Self {
            building_type,
            level: 1,
        }
    }
}
