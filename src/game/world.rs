use super::Game;
use crate::data::world_events::EventEffect;
use crate::engine::world_sim::WorldSimResult;

impl Game {
    /// Handle results from the world simulation
    pub(super) fn handle_world_sim_result(&mut self, result: WorldSimResult) {
        match result {
            WorldSimResult::EventTriggered(event) => {
                self.event_log.push(format!(
                    "[Regional Omen] {}: {}",
                    event.name, event.description
                ));
                // Apply immediate effects if no choices required
                if !event.requires_choice() {
                    for effect in &event.effects {
                        self.apply_event_effect(effect);
                    }
                }
            }
            WorldSimResult::EventResolved {
                event_id: _,
                effects,
            } => {
                for effect in effects {
                    self.apply_event_effect(&effect);
                }
            }
            WorldSimResult::FactionAction { faction_id, action } => {
                if let Some(faction) = self.world_sim.get_faction(&faction_id) {
                    let actor = faction.name.clone();
                    let deed = action.describe(&|id: &str| {
                        self.world_sim
                            .get_faction(id)
                            .map(|f| f.name.clone())
                            .unwrap_or_else(|| id.to_string())
                    });
                    self.event_log
                        .push(format!("[Power Tablet] {} {}.", actor, deed));
                }
            }
            WorldSimResult::WarDeclared {
                aggressor,
                defender,
            } => {
                let aggressor_name = self
                    .world_sim
                    .get_faction(&aggressor)
                    .map(|f| f.name.clone())
                    .unwrap_or(aggressor.clone());
                let defender_name = self
                    .world_sim
                    .get_faction(&defender)
                    .map(|f| f.name.clone())
                    .unwrap_or(defender.clone());
                self.event_log.push(format!(
                    "[War Banner] {} has declared war on {}!",
                    aggressor_name, defender_name
                ));
            }
            WorldSimResult::WarEnded {
                faction_a,
                faction_b,
                victor,
            } => {
                let a_name = self
                    .world_sim
                    .get_faction(&faction_a)
                    .map(|f| f.name.clone())
                    .unwrap_or(faction_a.clone());
                let b_name = self
                    .world_sim
                    .get_faction(&faction_b)
                    .map(|f| f.name.clone())
                    .unwrap_or(faction_b.clone());
                if let Some(v) = victor {
                    let v_name = self
                        .world_sim
                        .get_faction(&v)
                        .map(|f| f.name.clone())
                        .unwrap_or(v);
                    self.event_log.push(format!(
                        "[War Banner] The war between {} and {} has ended. {} is victorious!",
                        a_name, b_name, v_name
                    ));
                } else {
                    self.event_log.push(format!(
                        "[War Banner] The war between {} and {} has ended in a truce.",
                        a_name, b_name
                    ));
                }
            }
            WorldSimResult::TerritoryChanged {
                node_id,
                old_faction,
                new_faction,
            } => {
                self.event_log.push(format!(
                    "[Territory Tablet] {} has changed hands from {} to {}.",
                    node_id, old_faction, new_faction
                ));
            }
            WorldSimResult::PriceChanged {
                item_id,
                old_price,
                new_price,
            } => {
                // Silent unless significant change
                if (new_price as i32 - old_price as i32).abs() > 10 {
                    self.event_log.push(format!(
                        "[Caravan Market] {} prices changed: {} -> {}.",
                        item_id, old_price, new_price
                    ));
                }
            }
            WorldSimResult::RouteDisrupted { route_id, reason } => {
                self.event_log.push(format!(
                    "[Caravan Route] {} disrupted: {}",
                    route_id, reason
                ));
            }
            WorldSimResult::Notification(msg) => {
                self.event_log.push(msg);
            }
        }
    }

    /// Apply an event effect to the game state
    pub(super) fn apply_event_effect(&mut self, effect: &EventEffect) {
        match effect {
            EventEffect::ModifyRelation { faction_id, delta } => {
                if let Some(relation) = self.world_sim.get_relation_mut(faction_id) {
                    relation.modify_reputation(*delta);
                }
            }
            EventEffect::ModifyPrices {
                item_id,
                modifier,
                duration_ticks,
            } => {
                let price_mod = crate::data::economy::PriceModifier::new(
                    item_id.clone(),
                    *modifier,
                    "Event".to_string(),
                    Some(*duration_ticks),
                    self.tick,
                );
                self.world_sim.economy.add_price_modifier(price_mod);
            }
            EventEffect::SpawnMission { mission_id } => {
                self.event_log
                    .push(format!("New dispatch posted at the gate: {}", mission_id));
            }
            EventEffect::ModifyResource { resource, delta } => match resource {
                crate::data::world_events::ResourceType::SpiritStones => {
                    if *delta >= 0 {
                        self.spirit_stones += *delta as u32;
                    } else {
                        self.spirit_stones = self.spirit_stones.saturating_sub((-*delta) as u32);
                    }
                }
                crate::data::world_events::ResourceType::Influence => {
                    if *delta >= 0 {
                        self.influence += *delta as u32;
                    } else {
                        self.influence = self.influence.saturating_sub((-*delta) as u32);
                    }
                }
                crate::data::world_events::ResourceType::Relics => {
                    if *delta >= 0 {
                        self.relics += *delta as u32;
                    } else {
                        self.relics = self.relics.saturating_sub((-*delta) as u32);
                    }
                }
            },
            EventEffect::TriggerCombat {
                enemy_power,
                description,
            } => {
                self.event_log.push(format!(
                    "Hostile omen: {} (Power: {})",
                    description, enemy_power
                ));
            }
            EventEffect::UnlockTech { tech_id } => {
                if !self.unlocked_techs.contains(tech_id) {
                    self.unlocked_techs.push(tech_id.clone());
                    self.event_log
                        .push(format!("Doctrine recovered: {}", tech_id));
                }
            }
            EventEffect::ModifyCorruption { node_id, delta } => {
                if let Some(node) = self.data.map_nodes.iter_mut().find(|n| n.id == *node_id) {
                    if *delta >= 0 {
                        node.corruption += *delta as u32;
                    } else {
                        node.corruption = node.corruption.saturating_sub((-*delta) as u32);
                    }
                }
            }
            EventEffect::ChangeFactionTerritory {
                faction_id,
                node_id,
                gain,
            } => {
                if let Some(faction) = self
                    .world_sim
                    .factions
                    .iter_mut()
                    .find(|f| f.id == *faction_id)
                {
                    if *gain {
                        if !faction.territory_nodes.contains(node_id) {
                            faction.territory_nodes.push(node_id.clone());
                        }
                    } else {
                        faction.territory_nodes.retain(|n| n != node_id);
                    }
                }
            }
            EventEffect::GiveItem { item_id, amount } => {
                *self.inventory.entry(item_id.clone()).or_insert(0) += amount;
                self.event_log
                    .push(format!("Sect stores received {}x {}", amount, item_id));
            }
            EventEffect::ModifyCultivationSpeed {
                modifier,
                duration_ticks,
            } => {
                self.event_log.push(format!(
                    "Mountain qi circulation shifted by {}x for {} ticks",
                    modifier, duration_ticks
                ));
            }
            EventEffect::ChainEvent {
                event_id,
                delay_ticks,
            } => {
                self.world_sim.queue_event(event_id.clone(), *delay_ticks);
            }
        }
    }
}
