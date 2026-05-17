use super::super::Game;
use crate::data::missions::{MissionOutcome, OngoingMission};
use crate::state::StateTransition;

impl Game {
    pub(in crate::game) fn handle_start_mission(
        &mut self,
        mission_desc: String,
        disciple_indices: Vec<usize>,
    ) {
        let Some(mission) = self
            .data
            .missions
            .iter()
            .find(|m| m.description == mission_desc)
        else {
            return;
        };

        self.event_log
            .push(format!("Mission Started: {}", mission.description));
        self.ongoing_missions.push(OngoingMission {
            mission: mission.clone(),
            disciple_indices,
            ticks_remaining: mission.duration,
        });
    }

    pub(in crate::game) fn handle_claim_rewards(&mut self, outcome: MissionOutcome) {
        if !outcome.success {
            return;
        }

        self.spirit_stones += outcome.rewards.spirit_stones;
        self.herbs += outcome.rewards.herbs;
        self.influence += outcome.rewards.influence;
        self.relics += outcome.rewards.relics;

        for (item_id, amount) in &outcome.rewards.items {
            *self.inventory.entry(item_id.clone()).or_insert(0) += amount;
            self.event_log
                .push(format!("Received {}x Item '{}'", amount, item_id));
        }

        for &idx in &outcome.disciple_indices {
            if let Some(disciple) = self.disciples.get_mut(idx) {
                disciple.exp += outcome.rewards.disciple_exp;
            }
        }

        for recipe_id in &outcome.rewards.recipe_discoveries {
            if !self.discovered_recipes.contains(recipe_id) {
                self.discovered_recipes.push(recipe_id.clone());
                if let Some(recipe) = self.data.recipes.iter().find(|r| r.id == *recipe_id) {
                    self.event_log
                        .push(format!("Discovered recipe: {}!", recipe.name));
                }
            }
        }

        self.event_log.push(format!(
            "Mission Rewards Claimed: {} SS, {} XP",
            outcome.rewards.spirit_stones, outcome.rewards.disciple_exp
        ));

        self.completed_history.push(outcome.description.clone());
        self.check_mission_bottleneck(&outcome);
    }

    fn check_mission_bottleneck(&mut self, outcome: &MissionOutcome) {
        for &idx in &outcome.disciple_indices {
            let Some(disciple) = self.disciples.get_mut(idx) else {
                continue;
            };

            let Some(crate::data::disciples::Bottleneck::CompleteMission(ref mt)) =
                disciple.breakthrough_bottleneck
            else {
                continue;
            };

            let mission_matches = self.data.missions.iter().any(|m| {
                m.description == outcome.description
                    && match (&m.mission_type, mt.as_str()) {
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
                    }
            });

            if mission_matches {
                self.event_log.push(format!(
                    "{} overcame their bottleneck: Complete a {} mission!",
                    disciple.name, mt
                ));
                disciple.breakthrough_bottleneck = None;
            }
        }
    }
}
