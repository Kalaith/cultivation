use crate::data::disciples::{Bottleneck, Disciple};
use crate::data::items::{EquipmentSlot, ItemType};
use crate::data::loader::GameData;
use macroquad_toolkit::rng as game_rng;

/// Generate a bottleneck for a disciple reaching breakthrough readiness.
/// Returns `Some(bottleneck)` with increasing probability at higher realms.
pub fn generate_bottleneck(
    disciple: &Disciple,
    data: &GameData,
    realm_index: usize,
) -> Option<Bottleneck> {
    let chance = (0.40 + realm_index as f64 * 0.03).min(0.70);
    if game_rng::rand() >= chance as f32 {
        return None;
    }

    let variant = game_rng::gen_range(0, 5u32);
    match variant {
        0 => {
            // CompleteMission — pick a random mission type name
            let types = ["Exploration", "ResourceGathering", "MonsterSuppression"];
            let idx = game_rng::gen_range(0, types.len());
            Some(Bottleneck::CompleteMission(types[idx].to_string()))
        }
        1 => {
            // CraftItem — pick a random recipe with difficulty <= 50
            let easy_recipes: Vec<&crate::data::items::Recipe> =
                data.recipes.iter().filter(|r| r.difficulty <= 50).collect();
            if easy_recipes.is_empty() {
                Some(Bottleneck::EquipSlot(EquipmentSlot::Weapon))
            } else {
                let idx = game_rng::gen_range(0, easy_recipes.len());
                Some(Bottleneck::CraftItem(easy_recipes[idx].id.clone()))
            }
        }
        2 => {
            // ReachStat — random stat, threshold = current + 10 + realm_index * 5
            let stats = ["Body", "Mind", "Spirit"];
            let idx = game_rng::gen_range(0, 3usize);
            let stat_name = stats[idx];
            let current = match stat_name {
                "Body" => disciple.attributes.body,
                "Mind" => disciple.attributes.mind,
                _ => disciple.attributes.spirit,
            };
            let threshold = current + 10 + (realm_index as u32) * 5;
            Some(Bottleneck::ReachStat {
                stat: stat_name.to_string(),
                value: threshold,
            })
        }
        3 => {
            // UseItem — pick a random pill/potion from data.items
            let consumables: Vec<&str> = data
                .items
                .values()
                .filter(|i| matches!(i.item_type, ItemType::Pill | ItemType::Potion))
                .map(|i| i.id.as_str())
                .collect();
            if consumables.is_empty() {
                Some(Bottleneck::EquipSlot(EquipmentSlot::Weapon))
            } else {
                let idx = game_rng::gen_range(0, consumables.len());
                Some(Bottleneck::UseItem(consumables[idx].to_string()))
            }
        }
        _ => {
            // EquipSlot — random from the main equipment slots
            let slots = [
                EquipmentSlot::Weapon,
                EquipmentSlot::Chest,
                EquipmentSlot::Head,
                EquipmentSlot::Arms,
                EquipmentSlot::Legs,
            ];
            let idx = game_rng::gen_range(0, slots.len());
            Some(Bottleneck::EquipSlot(slots[idx].clone()))
        }
    }
}

/// Check if a bottleneck has been resolved based on current game state.
pub fn is_bottleneck_resolved(
    bottleneck: &Bottleneck,
    disciple: &Disciple,
    completed_history: &[String],
    inventory: &std::collections::HashMap<String, u32>,
    data: &GameData,
) -> bool {
    match bottleneck {
        Bottleneck::CompleteMission(mt) => {
            // Check against mission definitions for matching type
            data.missions.iter().any(|m| {
                let matches_type = match (&m.mission_type, mt.as_str()) {
                    (crate::data::missions::MissionType::Exploration, "Exploration") => true,
                    (
                        crate::data::missions::MissionType::ResourceGathering,
                        "ResourceGathering",
                    ) => true,
                    (
                        crate::data::missions::MissionType::MonsterSuppression,
                        "MonsterSuppression",
                    ) => true,
                    (crate::data::missions::MissionType::Diplomacy, "Diplomacy") => true,
                    (crate::data::missions::MissionType::RuinDelve, "RuinDelve") => true,
                    _ => false,
                };
                matches_type && completed_history.iter().any(|h| h == &m.description)
            })
        }
        Bottleneck::CraftItem(recipe_id) => {
            // Check if the output item exists in inventory
            if let Some(recipe) = data.recipes.iter().find(|r| r.id == *recipe_id) {
                *inventory.get(&recipe.output_item_id).unwrap_or(&0) > 0
            } else {
                false
            }
        }
        Bottleneck::ReachStat { stat, value } => {
            let current = match stat.as_str() {
                "Body" => disciple.attributes.body,
                "Mind" => disciple.attributes.mind,
                "Spirit" => disciple.attributes.spirit,
                _ => 0,
            };
            current >= *value
        }
        Bottleneck::UseItem(_) => {
            // Cleared event-driven only (when item is actually used)
            false
        }
        Bottleneck::EquipSlot(slot) => disciple.equipment.contains_key(slot),
    }
}
