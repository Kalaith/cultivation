use crate::data::{
    ai::AiSchedulerTuning,
    bloodlines::Bloodline,
    buildings::Building,
    disciples::FateTrait,
    economy::{EconomyNode, TradeRoute},
    factions::Faction,
    herbs::Herb,
    missions::{MapNode, Mission},
    spirit_beasts::{BeastEquipmentItem, SpiritBeastDefinition},
    stages::StageDefinition,
    world_events::WorldEvent,
};
use crate::engine::world_sim::WorldSimBalance;


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
    /// Faction definitions for world simulation
    pub factions: Vec<Faction>,
    /// Economy nodes for trading
    pub economy_nodes: Vec<EconomyNode>,
    /// Trade routes connecting economy nodes
    pub trade_routes: Vec<TradeRoute>,
    /// World event definitions
    pub world_events: Vec<WorldEvent>,
    /// Balance configuration
    pub balance: WorldSimBalance,
    /// AI scheduler tuning configuration
    pub ai_scheduler: AiSchedulerTuning,
    /// Spirit beast species definitions
    pub spirit_beast_definitions: std::collections::HashMap<String, SpiritBeastDefinition>,
    /// Spirit beast equipment definitions
    pub beast_equipment_definitions: std::collections::HashMap<String, BeastEquipmentItem>,
}

/// Economy data loaded from JSON
#[derive(Clone, serde::Deserialize)]
pub struct EconomyData {
    pub nodes: Vec<EconomyNode>,
    pub routes: Vec<TradeRoute>,
}

#[derive(Clone, serde::Deserialize)]
pub struct BuildingDefinition {
    #[serde(rename = "type")]
    pub building_type: crate::data::buildings::BuildingType,
    pub cost: u32,
    #[serde(default = "default_repair_cost")]
    pub repair_cost: u32,
    pub name: String,
    pub description: String,
    pub tech_required: Option<String>,
    #[serde(default)]
    pub element: crate::data::elements::Element,
    #[serde(default)]
    pub unique: bool,
}

fn default_repair_cost() -> u32 {
    50
}

impl GameData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Buildings list starts empty - populated at runtime when player constructs
        let buildings: Vec<Building> = Vec::new();

        // Load building definitions (templates for what can be built)
        let defs_json = std::fs::read_to_string("assets/data/buildings.json")?;
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

        // Load faction data
        let factions_json = std::fs::read_to_string("assets/data/factions.json").unwrap_or_else(|_| "[]".to_string());
        let factions: Vec<Faction> = serde_json::from_str(&factions_json)?;

        // Load economy data
        let economy_json = std::fs::read_to_string("assets/data/economy.json").unwrap_or_else(|_| r#"{"nodes":[],"routes":[]}"#.to_string());
        let economy_data: EconomyData = serde_json::from_str(&economy_json)?;

        // Load world events
        let events_json = std::fs::read_to_string("assets/data/world_events.json").unwrap_or_else(|_| "[]".to_string());
        let world_events: Vec<WorldEvent> = serde_json::from_str(&events_json)?;

        // Load balance configuration
        let balance_json = std::fs::read_to_string("assets/data/balance.json").unwrap_or_else(|_| "{}".to_string());
        let balance: WorldSimBalance = serde_json::from_str(&balance_json).unwrap_or_default();

        let ai_json = std::fs::read_to_string("assets/data/ai_scheduler.json").unwrap_or_else(|_| "{}".to_string());
        let ai_scheduler: AiSchedulerTuning = serde_json::from_str(&ai_json).unwrap_or_default();

        let beasts_json = std::fs::read_to_string("assets/data/spirit_beasts.json").unwrap_or_else(|_| "[]".to_string());
        let beasts_list: Vec<SpiritBeastDefinition> = serde_json::from_str(&beasts_json)?;
        let spirit_beast_definitions = beasts_list
            .into_iter()
            .map(|b| (b.id.clone(), b))
            .collect();

        let beast_equipment_json = std::fs::read_to_string("assets/data/beast_equipment.json").unwrap_or_else(|_| "[]".to_string());
        let beast_equipment_list: Vec<BeastEquipmentItem> = serde_json::from_str(&beast_equipment_json)?;
        let beast_equipment_definitions = beast_equipment_list
            .into_iter()
            .map(|b| (b.id.clone(), b))
            .collect();

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
            factions,
            economy_nodes: economy_data.nodes,
            trade_routes: economy_data.routes,
            world_events,
            balance,
            ai_scheduler,
            spirit_beast_definitions,
            beast_equipment_definitions,
        })
    }
}
