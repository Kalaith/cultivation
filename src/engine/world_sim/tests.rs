use super::*;

#[test]
fn test_world_sim_creation() {
    let sim = WorldSim::default();
    assert_eq!(sim.world_tick, 0);
    assert!(sim.factions.is_empty());
}

#[test]
fn test_seasonal_modifiers() {
    let spring = SeasonalModifiers::for_season(&Season::Spring);
    assert!(spring.cultivation_speed_mod > 1.0);

    let winter = SeasonalModifiers::for_season(&Season::Winter);
    assert!(winter.trade_activity_mod < 1.0);
}

#[test]
fn test_faction_relation() {
    let mut relation = FactionRelation::new("test".to_string(), 0);
    relation.modify_reputation(60);
    assert!(relation.is_friendly());

    relation.modify_reputation(-120);
    assert!(relation.is_hostile());
}
