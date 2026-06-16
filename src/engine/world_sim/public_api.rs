use super::{WorldSim, WorldSimResult};
use crate::data::{
    factions::Faction,
    relations::{DiplomaticAction, FactionRelation, Treaty, TreatyRequest},
    world_events::{EventEffect, QueuedEvent},
};

impl WorldSim {
    pub fn get_faction(&self, id: &str) -> Option<&Faction> {
        self.factions.iter().find(|f| f.id == id)
    }

    pub fn get_relation(&self, faction_id: &str) -> Option<&FactionRelation> {
        self.relations.iter().find(|r| r.faction_id == faction_id)
    }

    pub fn get_relation_mut(&mut self, faction_id: &str) -> Option<&mut FactionRelation> {
        self.relations
            .iter_mut()
            .find(|r| r.faction_id == faction_id)
    }

    pub fn process_diplomatic_action(
        &mut self,
        faction_id: &str,
        action: DiplomaticAction,
    ) -> Vec<WorldSimResult> {
        let mut results = Vec::new();
        let world_tick = self.world_tick;
        let treaty_break_penalty = self.balance.treaty_break_penalty;

        let Some(relation_idx) = self
            .relations
            .iter()
            .position(|r| r.faction_id == faction_id)
        else {
            return results;
        };

        match action {
            DiplomaticAction::SendGift { value } => {
                let rep_gain = (value / 100).min(20) as i32;
                self.relations[relation_idx].modify_reputation(rep_gain);
                self.relations[relation_idx].record_friendly_action(world_tick);
                results.push(WorldSimResult::Notification(format!(
                    "Tribute sent by sect envoy. Reputation +{}",
                    rep_gain
                )));
            }
            DiplomaticAction::Threaten => {
                self.relations[relation_idx].modify_reputation(-10);
                self.relations[relation_idx].record_hostile_action(world_tick);
                results.push(WorldSimResult::Notification(
                    "Threatening edict delivered. Reputation -10".to_string(),
                ));
            }
            DiplomaticAction::RequestTreaty { treaty_type } => {
                self.request_treaty(relation_idx, treaty_type, world_tick, &mut results);
            }
            DiplomaticAction::BreakTreaty => {
                if self.relations[relation_idx].treaty.is_some() {
                    self.relations[relation_idx].treaty = None;
                    self.relations[relation_idx].modify_reputation(treaty_break_penalty);
                    self.relations[relation_idx].record_hostile_action(world_tick);
                    results.push(WorldSimResult::Notification(format!(
                        "Treaty oath broken. Reputation {}",
                        treaty_break_penalty
                    )));
                }
            }
            DiplomaticAction::DeclareWar => {
                self.relations[relation_idx].declare_war(world_tick);
                results.push(WorldSimResult::WarDeclared {
                    aggressor: "player".to_string(),
                    defender: faction_id.to_string(),
                });
            }
            DiplomaticAction::SuePeace => {
                if self.relations[relation_idx].at_war {
                    self.relations[relation_idx].end_war();
                    self.relations[relation_idx].modify_reputation(10);
                    results.push(WorldSimResult::WarEnded {
                        faction_a: "player".to_string(),
                        faction_b: faction_id.to_string(),
                        victor: None,
                    });
                }
            }
            DiplomaticAction::RequestAudience => {
                self.relations[relation_idx].record_friendly_action(world_tick);
                results.push(WorldSimResult::Notification(
                    "Audience request carried by sect envoy.".to_string(),
                ));
            }
        }

        results
    }

    pub fn respond_to_event(&mut self, event_id: &str, choice_idx: usize) -> Vec<EventEffect> {
        if let Some(event) = self
            .active_events
            .iter_mut()
            .find(|e| e.event_id == event_id)
        {
            event.resolve_with_choice(choice_idx);
            return event.get_resolution_effects();
        }
        Vec::new()
    }

    pub fn get_item_price(&self, node_id: &str, item_id: &str) -> Option<u32> {
        self.economy
            .get_effective_price(node_id, item_id, self.balance.price_elasticity)
    }

    pub fn queue_event(&mut self, event_id: String, delay_ticks: u32) {
        self.event_queue.push(QueuedEvent {
            event_id,
            trigger_tick: self.world_tick + delay_ticks as u64,
        });
    }

    fn request_treaty(
        &mut self,
        relation_idx: usize,
        treaty_type: TreatyRequest,
        world_tick: u64,
        results: &mut Vec<WorldSimResult>,
    ) {
        let acceptance_threshold = match &treaty_type {
            TreatyRequest::NonAggression { .. } => 0,
            TreatyRequest::TradeAgreement { .. } => 25,
            TreatyRequest::Alliance { .. } => 50,
        };

        if self.relations[relation_idx].reputation < acceptance_threshold {
            results.push(WorldSimResult::Notification(
                "Treaty refused. Improve relations first.".to_string(),
            ));
            return;
        }

        let treaty = match treaty_type {
            TreatyRequest::NonAggression { duration_ticks } => Treaty::NonAggression {
                expires_tick: world_tick + duration_ticks,
            },
            TreatyRequest::TradeAgreement {
                discount_percent,
                duration_ticks,
            } => Treaty::TradeAgreement {
                discount_percent,
                expires_tick: world_tick + duration_ticks,
            },
            TreatyRequest::Alliance {
                mutual_defense,
                duration_ticks,
            } => Treaty::Alliance {
                mutual_defense,
                expires_tick: world_tick + duration_ticks,
            },
        };
        results.push(WorldSimResult::Notification(format!(
            "{} accepted!",
            treaty.name()
        )));
        self.relations[relation_idx].treaty = Some(treaty);
        self.relations[relation_idx].record_friendly_action(world_tick);
    }
}
