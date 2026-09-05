use super::*;

#[test]
fn shipped_catalogues_load_without_optional_content_silently_disappearing() {
    let data = GameData::load().expect("shipped content must load through the toolkit");
    assert!(!data.building_definitions.is_empty());
    assert!(!data.fate_traits.is_empty());
    assert!(!data.map_nodes.is_empty());
    assert!(!data.missions.is_empty());
    assert!(!data.laws.is_empty());
    assert!(!data.items.is_empty());
    assert!(!data.recipes.is_empty());
    assert!(!data.techs.is_empty());
    assert!(!data.stages_order.is_empty());
    assert!(data
        .stages_order
        .iter()
        .all(|id| data.stages.contains_key(id)));
    assert!(!data.bloodlines.is_empty());
    assert!(!data.herbs.is_empty());
    assert!(!data.factions.is_empty());
    assert!(!data.economy_nodes.is_empty());
    assert!(!data.trade_routes.is_empty());
    assert!(!data.world_events.is_empty());
    assert!(!data.spirit_beast_definitions.is_empty());
    assert!(!data.beast_equipment_definitions.is_empty());
}
