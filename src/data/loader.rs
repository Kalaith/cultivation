use crate::data::{
    bloodlines::Bloodline,
    buildings::Building,
    disciples::FateTrait,
    herbs::Herb,
    missions::{MapNode, Mission},
    stages::StageDefinition,
};


#[derive(Clone)]
pub struct GameData {
    pub buildings: Vec<Building>,
    pub building_definitions: std::collections::HashMap<crate::data::buildings::BuildingType, BuildingDefinition>,
    pub bloodlines: std::collections::HashMap<String, Bloodline>,
    pub fate_traits: Vec<FateTrait>,
    pub map_nodes: Vec<MapNode>,
    pub missions: Vec<Mission>,
    pub laws: std::collections::HashMap<String, crate::data::laws::CultivationLaw>,
    pub items: std::collections::HashMap<String, crate::data::items::Item>,
    pub recipes: Vec<crate::data::items::Recipe>,
    pub techs: std::collections::HashMap<String, crate::data::tech::Technology>,
    pub stages: std::collections::HashMap<String, StageDefinition>,
    /// Ordered list of stage IDs for progression
    pub stages_order: Vec<String>,
    /// Herb definitions for the herb system
    pub herbs: std::collections::HashMap<String, Herb>,
}

#[derive(Clone, serde::Deserialize)]
pub struct BuildingDefinition {
    #[serde(rename = "type")]
    pub building_type: crate::data::buildings::BuildingType,
    pub cost: u32,
    pub name: String,
    pub description: String,
    pub tech_required: Option<String>,
    #[serde(default)]
    pub element: crate::data::elements::Element,
    #[serde(default)]
    pub unique: bool,
}

impl GameData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let buildings_json = std::fs::read_to_string("assets/data/buildings.json")?;
        let building_list: Vec<Building> = serde_json::from_str(&buildings_json)?;
        let buildings = building_list
            .into_iter()
            .enumerate()
            .map(|(i, mut b)| {
                b.x = (i as i32 % 5) * 4 + 2;
                b.y = (i as i32 / 5) * 4 + 2;
                b.id = (i + 1) as u64; 
                b
            })
            .collect();

        // Load Building Definitions
        let defs_json = std::fs::read_to_string("assets/data/building_definitions.json")?;
        let defs_list: Vec<BuildingDefinition> = serde_json::from_str(&defs_json)?;
        let building_definitions = defs_list.into_iter().map(|d| (d.building_type.clone(), d)).collect();

        let traits_json = std::fs::read_to_string("assets/data/fatetraits.json")?;
        let fate_traits: Vec<FateTrait> = serde_json::from_str(&traits_json)?;

        let map_nodes_json = std::fs::read_to_string("assets/data/map_nodes.json")?;
        let map_nodes: Vec<MapNode> = serde_json::from_str(&map_nodes_json)?;

        let missions_json = std::fs::read_to_string("assets/data/missions.json")?;
        let missions: Vec<Mission> = serde_json::from_str(&missions_json)?;

        let laws_json = std::fs::read_to_string("assets/data/laws.json").unwrap_or_else(|_| "[]".to_string());
        let laws_list: Vec<crate::data::laws::CultivationLaw> = serde_json::from_str(&laws_json)?;
        let laws = laws_list.into_iter().map(|l| (l.id.clone(), l)).collect();

        let items_json = std::fs::read_to_string("assets/data/items.json").unwrap_or_else(|_| "[]".to_string());
        let items_list: Vec<crate::data::items::Item> = serde_json::from_str(&items_json)?;
        let items = items_list.into_iter().map(|i| (i.id.clone(), i)).collect();

        let recipes_json = std::fs::read_to_string("assets/data/recipes.json").unwrap_or_else(|_| "[]".to_string());
        let recipes: Vec<crate::data::items::Recipe> = serde_json::from_str(&recipes_json)?;

        let techs_json = std::fs::read_to_string("assets/data/tech.json").unwrap_or_else(|_| "[]".to_string());
        let techs_list: Vec<crate::data::tech::Technology> = serde_json::from_str(&techs_json)?;
        let techs = techs_list.into_iter().map(|t| (t.id.clone(), t)).collect();

        let stages_json = std::fs::read_to_string("assets/data/stages.json").unwrap_or_else(|_| "[]".to_string());
        let stages_list: Vec<StageDefinition> = serde_json::from_str(&stages_json)?;
        let stages_order: Vec<String> = stages_list.iter().map(|s| s.id.clone()).collect();
        let stages = stages_list.into_iter().map(|s| (s.id.clone(), s)).collect();

        let bloodlines_json = std::fs::read_to_string("assets/data/bloodlines.json").unwrap_or_else(|_| "[]".to_string());
        let bloodlines_list: Vec<Bloodline> = serde_json::from_str(&bloodlines_json)?;
        let bloodlines = bloodlines_list.into_iter().map(|b| (b.id.clone(), b)).collect();

        let herbs_json = std::fs::read_to_string("assets/data/herbs.json").unwrap_or_else(|_| "[]".to_string());
        let herbs_list: Vec<Herb> = serde_json::from_str(&herbs_json)?;
        let herbs = herbs_list.into_iter().map(|h| (h.id.clone(), h)).collect();

        Ok(GameData {
            buildings,
            building_definitions,
            bloodlines,
            fate_traits,
            map_nodes,
            missions,
            laws,
            items,
            recipes,
            techs,
            stages,
            stages_order,
            herbs,
        })
    }
}
