use crate::data::{
    buildings::{Building, BuildingType},
    disciples::FateTrait,
    missions::{MapNode, Mission},
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct GameData {
    pub buildings: HashMap<BuildingType, Building>,
    pub fate_traits: Vec<FateTrait>,
    pub map_nodes: Vec<MapNode>,
    pub missions: Vec<Mission>,
}

impl GameData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let buildings_json = std::fs::read_to_string("assets/data/buildings.json")?;
        let building_list: Vec<Building> = serde_json::from_str(&buildings_json)?;
        let buildings = building_list
            .into_iter()
            .map(|b| (b.building_type.clone(), b))
            .collect();

        let traits_json = std::fs::read_to_string("assets/data/fatetraits.json")?;
        let fate_traits: Vec<FateTrait> = serde_json::from_str(&traits_json)?;

        let map_nodes_json = std::fs::read_to_string("assets/data/map_nodes.json")?;
        let map_nodes: Vec<MapNode> = serde_json::from_str(&map_nodes_json)?;

        let missions_json = std::fs::read_to_string("assets/data/missions.json")?;
        let missions: Vec<Mission> = serde_json::from_str(&missions_json)?;

        Ok(GameData {
            buildings,
            fate_traits,
            map_nodes,
            missions,
        })
    }
}
