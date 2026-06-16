use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod public_api;

use crate::data::{
    economy::{EconomyNode, EconomyState, PriceModifier, TradeRoute},
    factions::{Faction, FactionAI, FactionAction},
    herbs::Season,
    relations::FactionRelation,
    world_events::{ActiveWorldEvent, EventEffect, EventTrigger, QueuedEvent, WorldEvent},
};

/// Results from world simulation tick
#[derive(Clone, Debug)]
pub enum WorldSimResult {
    /// An event triggered
    EventTriggered(ActiveWorldEvent),
    /// An event expired or was resolved
    EventResolved {
        event_id: String,
        effects: Vec<EventEffect>,
    },
    /// Faction took an action
    FactionAction {
        faction_id: String,
        action: FactionAction,
    },
    /// War declared
    WarDeclared { aggressor: String, defender: String },
    /// War ended
    WarEnded {
        faction_a: String,
        faction_b: String,
        victor: Option<String>,
    },
    /// Territory changed hands
    TerritoryChanged {
        node_id: String,
        old_faction: String,
        new_faction: String,
    },
    /// Price changed significantly
    PriceChanged {
        item_id: String,
        old_price: u32,
        new_price: u32,
    },
    /// Trade route disrupted
    RouteDisrupted { route_id: String, reason: String },
    /// Notification for player
    Notification(String),
}

/// Seasonal modifiers affecting world simulation
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SeasonalModifiers {
    /// Herb growth multiplier by herb ID
    #[serde(default)]
    pub herb_growth_mult: HashMap<String, f32>,
    /// Mission difficulty modifier
    #[serde(default = "default_one")]
    pub mission_difficulty_mod: f32,
    /// Trade activity modifier
    #[serde(default = "default_one")]
    pub trade_activity_mod: f32,
    /// Cultivation speed modifier
    #[serde(default = "default_one")]
    pub cultivation_speed_mod: f32,
}

fn default_one() -> f32 {
    1.0
}

impl SeasonalModifiers {
    pub fn for_season(season: &Season) -> Self {
        match season {
            Season::Spring => Self {
                herb_growth_mult: HashMap::new(),
                mission_difficulty_mod: 0.9,
                trade_activity_mod: 1.1,
                cultivation_speed_mod: 1.1,
            },
            Season::Summer => Self {
                herb_growth_mult: HashMap::new(),
                mission_difficulty_mod: 1.0,
                trade_activity_mod: 1.0,
                cultivation_speed_mod: 1.0,
            },
            Season::Autumn => Self {
                herb_growth_mult: HashMap::new(),
                mission_difficulty_mod: 1.0,
                trade_activity_mod: 1.2,
                cultivation_speed_mod: 1.0,
            },
            Season::Winter => Self {
                herb_growth_mult: HashMap::new(),
                mission_difficulty_mod: 1.2,
                trade_activity_mod: 0.7,
                cultivation_speed_mod: 0.9,
            },
        }
    }
}

/// Balance configuration for world simulation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldSimBalance {
    pub relation_drift_rate: i32,
    pub drift_interval_ticks: u64,
    pub treaty_break_penalty: i32,
    pub hostility_threshold: i32,
    pub price_elasticity: f32,
    pub supply_regen_rate: f32,
    pub trade_route_base_time: u32,
    pub random_event_chance_per_tick: f32,
    pub season_event_guaranteed: bool,
    pub faction_action_interval: u64,
    pub war_duration_ticks: u64,
}

impl Default for WorldSimBalance {
    fn default() -> Self {
        Self {
            relation_drift_rate: 1,
            drift_interval_ticks: 1000,
            treaty_break_penalty: -30,
            hostility_threshold: -50,
            price_elasticity: 0.1,
            supply_regen_rate: 0.05,
            trade_route_base_time: 600,
            random_event_chance_per_tick: 0.001,
            season_event_guaranteed: true,
            faction_action_interval: 3000,
            war_duration_ticks: 5000,
        }
    }
}

/// Main world simulation state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldSim {
    pub factions: Vec<Faction>,
    pub faction_ais: Vec<FactionAI>,
    pub relations: Vec<FactionRelation>,
    pub economy: EconomyState,
    pub active_events: Vec<ActiveWorldEvent>,
    pub event_queue: Vec<QueuedEvent>,
    #[serde(skip)]
    pub event_definitions: Vec<WorldEvent>,
    pub rng_seed: u64,
    pub world_tick: u64,
    pub price_modifiers: Vec<PriceModifier>,
    #[serde(default)]
    pub seasonal_modifiers: SeasonalModifiers,
    #[serde(default)]
    pub balance: WorldSimBalance,
    pub last_season: Option<Season>,
    #[serde(default)]
    pub triggered_event_ids: Vec<String>,
}

impl Default for WorldSim {
    fn default() -> Self {
        Self {
            factions: Vec::new(),
            faction_ais: Vec::new(),
            relations: Vec::new(),
            economy: EconomyState::default(),
            active_events: Vec::new(),
            event_queue: Vec::new(),
            event_definitions: Vec::new(),
            rng_seed: 12345,
            world_tick: 0,
            price_modifiers: Vec::new(),
            seasonal_modifiers: SeasonalModifiers::default(),
            balance: WorldSimBalance::default(),
            last_season: None,
            triggered_event_ids: Vec::new(),
        }
    }
}

impl WorldSim {
    /// Create a new world simulation with loaded data
    pub fn new(
        factions: Vec<Faction>,
        economy_nodes: Vec<EconomyNode>,
        trade_routes: Vec<TradeRoute>,
        event_definitions: Vec<WorldEvent>,
        balance: WorldSimBalance,
    ) -> Self {
        let faction_ais: Vec<FactionAI> = factions
            .iter()
            .map(|f| FactionAI::new(f.id.clone()))
            .collect();

        let relations: Vec<FactionRelation> = factions
            .iter()
            .map(|f| FactionRelation::new(f.id.clone(), f.disposition))
            .collect();

        Self {
            factions,
            faction_ais,
            relations,
            economy: EconomyState::new(economy_nodes, trade_routes),
            event_definitions,
            balance,
            ..Default::default()
        }
    }

    /// Main tick function - called periodically from game loop
    pub fn tick(&mut self, game_tick: u64, season: &Season) -> Vec<WorldSimResult> {
        self.world_tick = game_tick;
        let mut results = Vec::new();

        // Check for season change
        if self.last_season.as_ref() != Some(season) {
            self.on_season_change(season, &mut results);
            self.last_season = Some(season.clone());
        }

        // Process diplomacy drift
        if game_tick % self.balance.drift_interval_ticks == 0 {
            self.update_diplomacy(&mut results);
        }

        // Process economy
        if game_tick % 60 == 0 {
            self.update_economy(&mut results);
        }

        // Process faction AI
        if game_tick % self.balance.faction_action_interval == 0 {
            self.update_faction_ai(&mut results);
        }

        // Check event triggers
        self.check_event_triggers(&mut results);

        // Process event queue
        self.process_event_queue(&mut results);

        // Process active events
        self.process_active_events(&mut results);

        // Check wars
        self.update_wars(&mut results);

        results
    }

    /// Handle season change
    fn on_season_change(&mut self, season: &Season, results: &mut Vec<WorldSimResult>) {
        self.seasonal_modifiers = SeasonalModifiers::for_season(season);

        // Collect event IDs to trigger
        let events_to_trigger: Vec<String> = self
            .event_definitions
            .iter()
            .filter(|event| {
                if let EventTrigger::SeasonChange(trigger_season) = &event.trigger {
                    trigger_season == season
                } else {
                    false
                }
            })
            .map(|event| event.id.clone())
            .collect();

        // Trigger the events
        for event_id in events_to_trigger {
            self.trigger_event(&event_id, results);
        }

        results.push(WorldSimResult::Notification(format!(
            "{} has arrived.",
            season
        )));
    }

    /// Update diplomatic relations
    fn update_diplomacy(&mut self, results: &mut Vec<WorldSimResult>) {
        let drift_rate = self.balance.relation_drift_rate;
        let hostility_threshold = self.balance.hostility_threshold;
        let world_tick = self.world_tick;

        // Build faction lookup
        let faction_map: HashMap<String, (i32, String)> = self
            .factions
            .iter()
            .map(|f| (f.id.clone(), (f.disposition, f.name.clone())))
            .collect();

        for relation in &mut self.relations {
            if let Some((disposition, name)) = faction_map.get(&relation.faction_id) {
                relation.apply_drift(*disposition, drift_rate);

                if let Some(treaty) = &relation.treaty {
                    if treaty.is_expired(world_tick) {
                        results.push(WorldSimResult::Notification(format!(
                            "{} with {} has expired.",
                            treaty.name(),
                            name
                        )));
                        relation.treaty = None;
                    }
                }

                if relation.reputation < hostility_threshold && !relation.at_war {
                    results.push(WorldSimResult::Notification(format!(
                        "{} has become hostile!",
                        name
                    )));
                }
            }
        }
    }

    /// Update economy simulation
    fn update_economy(&mut self, results: &mut Vec<WorldSimResult>) {
        let trade_mod = self.seasonal_modifiers.trade_activity_mod;
        let regen_rate = self.balance.supply_regen_rate * trade_mod;
        let world_tick = self.world_tick;

        self.economy.tick(world_tick, regen_rate);

        // Build war zone info
        let war_faction_territories: Vec<Vec<String>> = self
            .relations
            .iter()
            .filter(|r| r.at_war)
            .filter_map(|r| {
                self.factions
                    .iter()
                    .find(|f| f.id == r.faction_id)
                    .map(|f| f.territory_nodes.clone())
            })
            .collect();

        // Check route safety
        for route in &mut self.economy.routes {
            let mut safety = 1.0;

            let route_nodes: Vec<String> = self
                .economy
                .nodes
                .iter()
                .filter(|n| n.id == route.from_node || n.id == route.to_node)
                .map(|n| n.map_node_id.clone())
                .collect();

            for territories in &war_faction_territories {
                for node_id in &route_nodes {
                    if territories.contains(node_id) {
                        safety *= 0.5;
                    }
                }
            }

            if route.safety_rating != safety {
                route.safety_rating = safety;
                if safety < 0.5 {
                    results.push(WorldSimResult::RouteDisrupted {
                        route_id: route.id.clone(),
                        reason: "War has disrupted trade.".to_string(),
                    });
                }
            }
        }

        self.price_modifiers.retain(|m| !m.is_expired(world_tick));
    }

    /// Update faction AI decisions
    fn update_faction_ai(&mut self, results: &mut Vec<WorldSimResult>) {
        let power_map: HashMap<String, u32> = self
            .factions
            .iter()
            .map(|f| (f.id.clone(), f.power_level))
            .collect();

        let world_tick = self.world_tick;

        for ai in &mut self.faction_ais {
            let own_power = power_map.get(&ai.faction_id).copied().unwrap_or(0);

            let neighbor_powers: HashMap<String, u32> = power_map
                .iter()
                .filter(|(id, _)| *id != &ai.faction_id)
                .map(|(id, p)| (id.clone(), *p))
                .collect();

            if let Some(action) = ai.decide_action(own_power, &neighbor_powers, world_tick) {
                ai.last_action_tick = world_tick;

                match &action {
                    FactionAction::DeclareWar { target } => {
                        if !ai.enemies.contains(target) {
                            ai.enemies.push(target.clone());
                        }
                        results.push(WorldSimResult::FactionAction {
                            faction_id: ai.faction_id.clone(),
                            action: action.clone(),
                        });
                        results.push(WorldSimResult::WarDeclared {
                            aggressor: ai.faction_id.clone(),
                            defender: target.clone(),
                        });
                    }
                    FactionAction::ProposeAlliance { target: _ } => {
                        results.push(WorldSimResult::FactionAction {
                            faction_id: ai.faction_id.clone(),
                            action: action.clone(),
                        });
                    }
                    _ => {
                        results.push(WorldSimResult::FactionAction {
                            faction_id: ai.faction_id.clone(),
                            action: action.clone(),
                        });
                    }
                }
            }
        }
    }

    /// Check for event triggers
    fn check_event_triggers(&mut self, results: &mut Vec<WorldSimResult>) {
        let world_tick = self.world_tick;

        // Collect events to trigger
        let mut events_to_trigger = Vec::new();

        // Generate random value once
        let random_roll = self.pseudo_random();

        for event in &self.event_definitions {
            if !event.repeatable && self.triggered_event_ids.contains(&event.id) {
                continue;
            }

            if world_tick < event.min_tick {
                continue;
            }

            let should_trigger = match &event.trigger {
                EventTrigger::SeasonChange(_) => false,
                EventTrigger::ReputationThreshold {
                    faction_id,
                    threshold,
                } => self
                    .relations
                    .iter()
                    .find(|r| &r.faction_id == faction_id)
                    .map(|r| r.reputation >= *threshold)
                    .unwrap_or(false),
                EventTrigger::TickInterval(interval) => world_tick % interval == 0,
                EventTrigger::RandomChance(chance) => random_roll < *chance,
                EventTrigger::FactionPowerShift => {
                    self.relations.iter().any(|r| r.at_war) && world_tick % 1000 == 0
                }
                EventTrigger::InfluenceThreshold(_) => false,
                EventTrigger::Manual => false,
            };

            if should_trigger {
                events_to_trigger.push(event.id.clone());
            }
        }

        for event_id in events_to_trigger {
            self.trigger_event(&event_id, results);
        }
    }

    /// Trigger a specific event by ID
    fn trigger_event(&mut self, event_id: &str, results: &mut Vec<WorldSimResult>) {
        if let Some(event) = self
            .event_definitions
            .iter()
            .find(|e| e.id == event_id)
            .cloned()
        {
            let active = ActiveWorldEvent::from_event(&event, self.world_tick);

            if !event.repeatable {
                self.triggered_event_ids.push(event_id.to_string());
            }

            results.push(WorldSimResult::EventTriggered(active.clone()));
            self.active_events.push(active);
        }
    }

    /// Process queued events
    fn process_event_queue(&mut self, results: &mut Vec<WorldSimResult>) {
        let world_tick = self.world_tick;
        let ready_events: Vec<_> = self
            .event_queue
            .iter()
            .filter(|q| world_tick >= q.trigger_tick)
            .map(|q| q.event_id.clone())
            .collect();

        self.event_queue.retain(|q| world_tick < q.trigger_tick);

        for event_id in ready_events {
            self.trigger_event(&event_id, results);
        }
    }

    /// Process active events
    fn process_active_events(&mut self, results: &mut Vec<WorldSimResult>) {
        let world_tick = self.world_tick;
        let mut events_to_resolve = Vec::new();

        for event in &self.active_events {
            if event.is_expired(world_tick) || event.resolved {
                events_to_resolve.push((event.event_id.clone(), event.get_resolution_effects()));
            }
        }

        for (event_id, effects) in events_to_resolve {
            self.active_events.retain(|e| e.event_id != event_id);
            results.push(WorldSimResult::EventResolved { event_id, effects });
        }
    }

    /// Update ongoing wars
    fn update_wars(&mut self, results: &mut Vec<WorldSimResult>) {
        let world_tick = self.world_tick;
        let war_duration = self.balance.war_duration_ticks;

        //let faction_ids: Vec<String> = self.factions.iter().map(|f| f.id.clone()).collect();

        for relation in &mut self.relations {
            if relation.at_war {
                if let Some(start_tick) = relation.war_start_tick {
                    if world_tick - start_tick >= war_duration {
                        let victor = Some(relation.faction_id.clone());
                        relation.end_war();
                        results.push(WorldSimResult::WarEnded {
                            faction_a: "player".to_string(),
                            faction_b: relation.faction_id.clone(),
                            victor,
                        });
                    }
                }
            }
        }
    }

    /// Simple pseudo-random number generator (0.0 to 1.0)
    fn pseudo_random(&mut self) -> f32 {
        self.rng_seed = self.rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.rng_seed >> 16) & 0x7FFF) as f32 / 32767.0
    }
}

/// Data structure for saving world sim state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedWorldSim {
    pub factions: Vec<Faction>,
    pub faction_ais: Vec<FactionAI>,
    pub relations: Vec<FactionRelation>,
    pub economy: EconomyState,
    pub active_events: Vec<ActiveWorldEvent>,
    pub event_queue: Vec<QueuedEvent>,
    pub world_tick: u64,
    pub price_modifiers: Vec<PriceModifier>,
    pub seasonal_modifiers: SeasonalModifiers,
    pub balance: WorldSimBalance,
    pub last_season: Option<Season>,
    pub triggered_event_ids: Vec<String>,
    pub rng_seed: u64,
}

impl From<&WorldSim> for SavedWorldSim {
    fn from(sim: &WorldSim) -> Self {
        Self {
            factions: sim.factions.clone(),
            faction_ais: sim.faction_ais.clone(),
            relations: sim.relations.clone(),
            economy: sim.economy.clone(),
            active_events: sim.active_events.clone(),
            event_queue: sim.event_queue.clone(),
            world_tick: sim.world_tick,
            price_modifiers: sim.price_modifiers.clone(),
            seasonal_modifiers: sim.seasonal_modifiers.clone(),
            balance: sim.balance.clone(),
            last_season: sim.last_season.clone(),
            triggered_event_ids: sim.triggered_event_ids.clone(),
            rng_seed: sim.rng_seed,
        }
    }
}

impl WorldSim {
    pub fn from_saved(saved: SavedWorldSim, event_definitions: Vec<WorldEvent>) -> Self {
        Self {
            factions: saved.factions,
            faction_ais: saved.faction_ais,
            relations: saved.relations,
            economy: saved.economy,
            active_events: saved.active_events,
            event_queue: saved.event_queue,
            event_definitions,
            rng_seed: saved.rng_seed,
            world_tick: saved.world_tick,
            price_modifiers: saved.price_modifiers,
            seasonal_modifiers: saved.seasonal_modifiers,
            balance: saved.balance,
            last_season: saved.last_season,
            triggered_event_ids: saved.triggered_event_ids,
        }
    }
}

#[cfg(test)]
mod tests {
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
}
