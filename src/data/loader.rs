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
use macroquad_toolkit::data_loader::JsonFallbackPolicy;
#[cfg(target_arch = "wasm32")]
use macroquad_toolkit::data_loader::{load_json_file, load_json_file_with_fallback};
#[cfg(not(target_arch = "wasm32"))]
use macroquad_toolkit::data_loader::{load_json_file_sync, load_json_file_with_fallback_sync};

#[derive(Clone)]
pub struct GameData {
    pub buildings: Vec<Building>,
    pub building_definitions:
        std::collections::HashMap<crate::data::buildings::BuildingType, BuildingDefinition>,
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

#[cfg(test)]
mod tests;

impl GameData {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Buildings list starts empty - populated at runtime when player constructs
        let buildings: Vec<Building> = Vec::new();

        // Load building definitions (templates for what can be built)
        let defs_list: Vec<BuildingDefinition> = load_json_file_sync("assets/data/buildings.json")?;
        let building_definitions = defs_list
            .into_iter()
            .map(|d| (d.building_type.clone(), d))
            .collect();

        let fate_traits: Vec<FateTrait> = load_json_file_sync("assets/data/fatetraits.json")?;

        let map_nodes: Vec<MapNode> = load_json_file_sync("assets/data/map_nodes.json")?;

        let missions: Vec<Mission> = load_json_file_sync("assets/data/missions.json")?;

        let laws_list: Vec<crate::data::laws::CultivationLaw> = load_json_file_with_fallback_sync(
            "assets/data/laws.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
        let laws = laws_list.into_iter().map(|l| (l.id.clone(), l)).collect();

        let items_list: Vec<crate::data::items::Item> = load_json_file_with_fallback_sync(
            "assets/data/items.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
        let items = items_list.into_iter().map(|i| (i.id.clone(), i)).collect();

        let recipes: Vec<crate::data::items::Recipe> = load_json_file_with_fallback_sync(
            "assets/data/recipes.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;

        let techs_list: Vec<crate::data::tech::Technology> = load_json_file_with_fallback_sync(
            "assets/data/tech.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
        let techs = techs_list.into_iter().map(|t| (t.id.clone(), t)).collect();

        let stages_list: Vec<StageDefinition> = load_json_file_with_fallback_sync(
            "assets/data/stages.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
        let stages_order: Vec<String> = stages_list.iter().map(|s| s.id.clone()).collect();
        let stages = stages_list.into_iter().map(|s| (s.id.clone(), s)).collect();

        let bloodlines_list: Vec<Bloodline> = load_json_file_with_fallback_sync(
            "assets/data/bloodlines.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
        let bloodlines = bloodlines_list
            .into_iter()
            .map(|b| (b.id.clone(), b))
            .collect();

        let herbs_list: Vec<Herb> = load_json_file_with_fallback_sync(
            "assets/data/herbs.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
        let herbs = herbs_list.into_iter().map(|h| (h.id.clone(), h)).collect();

        // Load faction data
        let factions: Vec<Faction> = load_json_file_with_fallback_sync(
            "assets/data/factions.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;

        // Load economy data
        let economy_data: EconomyData = load_json_file_with_fallback_sync(
            "assets/data/economy.json",
            r#"{"nodes":[],"routes":[]}"#,
            JsonFallbackPolicy::ReadError,
        )?;

        // Load world events
        let world_events: Vec<WorldEvent> = load_json_file_with_fallback_sync(
            "assets/data/world_events.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;

        // Load balance configuration
        let balance: WorldSimBalance = load_json_file_with_fallback_sync(
            "assets/data/balance.json",
            "{}",
            JsonFallbackPolicy::ReadError,
        )
        .unwrap_or_default();

        let ai_scheduler: AiSchedulerTuning = load_json_file_with_fallback_sync(
            "assets/data/ai_scheduler.json",
            "{}",
            JsonFallbackPolicy::ReadError,
        )
        .unwrap_or_default();

        let beasts_list: Vec<SpiritBeastDefinition> = load_json_file_with_fallback_sync(
            "assets/data/spirit_beasts.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
        let spirit_beast_definitions = beasts_list.into_iter().map(|b| (b.id.clone(), b)).collect();

        let beast_equipment_list: Vec<BeastEquipmentItem> = load_json_file_with_fallback_sync(
            "assets/data/beast_equipment.json",
            "[]",
            JsonFallbackPolicy::ReadError,
        )?;
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

    pub async fn load_async() -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::load()
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Buildings list starts empty - populated at runtime when player constructs.
            let buildings: Vec<Building> = Vec::new();

            let defs_list: Vec<BuildingDefinition> =
                load_json_file("assets/data/buildings.json").await?;
            let building_definitions = defs_list
                .into_iter()
                .map(|d| (d.building_type.clone(), d))
                .collect();

            let fate_traits: Vec<FateTrait> = load_json_file("assets/data/fatetraits.json").await?;
            let map_nodes: Vec<MapNode> = load_json_file("assets/data/map_nodes.json").await?;
            let missions: Vec<Mission> = load_json_file("assets/data/missions.json").await?;

            let laws_list: Vec<crate::data::laws::CultivationLaw> = load_json_file_with_fallback(
                "assets/data/laws.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
            let laws = laws_list.into_iter().map(|l| (l.id.clone(), l)).collect();

            let items_list: Vec<crate::data::items::Item> = load_json_file_with_fallback(
                "assets/data/items.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
            let items = items_list.into_iter().map(|i| (i.id.clone(), i)).collect();

            let recipes: Vec<crate::data::items::Recipe> = load_json_file_with_fallback(
                "assets/data/recipes.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;

            let techs_list: Vec<crate::data::tech::Technology> = load_json_file_with_fallback(
                "assets/data/tech.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
            let techs = techs_list.into_iter().map(|t| (t.id.clone(), t)).collect();

            let stages_list: Vec<StageDefinition> = load_json_file_with_fallback(
                "assets/data/stages.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
            let stages_order: Vec<String> = stages_list.iter().map(|s| s.id.clone()).collect();
            let stages = stages_list.into_iter().map(|s| (s.id.clone(), s)).collect();

            let bloodlines_list: Vec<Bloodline> = load_json_file_with_fallback(
                "assets/data/bloodlines.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
            let bloodlines = bloodlines_list
                .into_iter()
                .map(|b| (b.id.clone(), b))
                .collect();

            let herbs_list: Vec<Herb> = load_json_file_with_fallback(
                "assets/data/herbs.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
            let herbs = herbs_list.into_iter().map(|h| (h.id.clone(), h)).collect();

            let factions: Vec<Faction> = load_json_file_with_fallback(
                "assets/data/factions.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;

            let economy_data: EconomyData = load_json_file_with_fallback(
                "assets/data/economy.json",
                r#"{"nodes":[],"routes":[]}"#,
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;

            let world_events: Vec<WorldEvent> = load_json_file_with_fallback(
                "assets/data/world_events.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;

            let balance: WorldSimBalance = load_json_file_with_fallback(
                "assets/data/balance.json",
                "{}",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await
            .unwrap_or_default();
            let ai_scheduler: AiSchedulerTuning = load_json_file_with_fallback(
                "assets/data/ai_scheduler.json",
                "{}",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await
            .unwrap_or_default();

            let beasts_list: Vec<SpiritBeastDefinition> = load_json_file_with_fallback(
                "assets/data/spirit_beasts.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
            let spirit_beast_definitions =
                beasts_list.into_iter().map(|b| (b.id.clone(), b)).collect();

            let beast_equipment_list: Vec<BeastEquipmentItem> = load_json_file_with_fallback(
                "assets/data/beast_equipment.json",
                "[]",
                JsonFallbackPolicy::ReadOrParseError,
            )
            .await?;
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
}
