use super::Game;
use crate::data::missions::{MissionOutcome, MissionRewards, OngoingMission, RelevantStat};
use macroquad_toolkit::rng as game_rng;

impl Game {
    pub(super) fn calculate_mission_outcome(&self, ongoing: OngoingMission) -> MissionOutcome {
        let mut team_power: i32 = 0;
        let mut trait_modifier: f32 = 0.0;
        let mut logs = Vec::new();

        let relevant_stat = ongoing.mission.mission_type.get_relevant_stat();
        logs.push(format!(
            "Mission Type: {:?} (Uses {:?})",
            ongoing.mission.mission_type, relevant_stat
        ));

        for &idx in &ongoing.disciple_indices {
            if let Some(disciple) = self.disciples.get(idx) {
                // Realm power from Data
                let realm_power = self
                    .data
                    .stages
                    .get(&disciple.realm)
                    .map(|s| (s.base_hp + s.base_qi) / 100)
                    .unwrap_or(1) as i32;

                // Attribute power based on mission type
                let attr_power = match relevant_stat {
                    RelevantStat::Body => disciple.attributes.body / 5,
                    RelevantStat::Mind => disciple.attributes.mind / 5,
                    RelevantStat::Spirit => disciple.attributes.spirit / 5,
                };

                let total_power = realm_power + attr_power as i32;
                team_power += total_power;

                // Sum relevant trait modifiers
                for t in &disciple.fate_traits {
                    trait_modifier += ongoing.mission.mission_type.get_trait_modifier(t);
                }

                logs.push(format!(
                    "{}: Realm Pwr {} + Attr {} = {}",
                    disciple.name, realm_power, attr_power, total_power
                ));
            }
        }

        let difficulty = (ongoing.mission.danger_level * 2) as i32;
        logs.push(format!("Team Power: {} vs Difficulty: {}", team_power, difficulty));

        let base_chance = 0.5;
        let power_modifier = (team_power - difficulty) as f32 * 0.08;
        let chance = (base_chance + power_modifier + trait_modifier).clamp(0.1, 0.95);

        logs.push(format!(
            "Success Chance: {:.0}% (Trait Mod: {:+.0}%)",
            chance * 100.0,
            trait_modifier * 100.0
        ));

        let roll = game_rng::rand();
        let success = roll < chance;

        let rewards = if success {
            logs.push("Mission Successful!".to_string());

            match ongoing.mission.mission_type {
                crate::data::missions::MissionType::Exploration => MissionRewards {
                    spirit_stones: ongoing.mission.danger_level * 10,
                    disciple_exp: ongoing.mission.danger_level * 50,
                    herbs: ongoing.mission.danger_level * 5,
                    influence: 0,
                    relics: 0,
                    items: vec![],
                },
                crate::data::missions::MissionType::ResourceGathering => {
                    let ore_amount = ongoing.mission.danger_level * 5;
                    logs.push(format!("Gathered {} Spirit Ore.", ore_amount));
                    MissionRewards {
                        spirit_stones: ongoing.mission.danger_level * 30,
                        disciple_exp: ongoing.mission.danger_level * 60,
                        herbs: ongoing.mission.danger_level * 2,
                        influence: 0,
                        relics: 0,
                        items: vec![("spirit_ore".to_string(), ore_amount)],
                    }
                }
                crate::data::missions::MissionType::MonsterSuppression => {
                    let found_relic = game_rng::chance(0.2);
                    if found_relic {
                        logs.push("Found a Monster Core (Relic)!".to_string());
                    }
                    MissionRewards {
                        spirit_stones: ongoing.mission.danger_level * 80,
                        disciple_exp: ongoing.mission.danger_level * 120,
                        herbs: 0,
                        influence: 0,
                        relics: if found_relic { 1 } else { 0 },
                        items: vec![],
                    }
                }
                crate::data::missions::MissionType::Diplomacy => MissionRewards {
                    spirit_stones: ongoing.mission.danger_level * 40,
                    disciple_exp: ongoing.mission.danger_level * 80,
                    herbs: 0,
                    influence: ongoing.mission.danger_level * 5,
                    relics: 0,
                    items: vec![],
                },
                crate::data::missions::MissionType::RuinDelve => {
                    let found_relic = game_rng::chance(0.7);
                    if found_relic {
                        logs.push("Recovered an Ancient Artifact!".to_string());
                    }
                    MissionRewards {
                        spirit_stones: ongoing.mission.danger_level * 100,
                        disciple_exp: ongoing.mission.danger_level * 150,
                        herbs: 0,
                        influence: 0,
                        relics: if found_relic { 1 } else { 0 },
                        items: vec![],
                    }
                }
            }
        } else {
            logs.push("Mission Failed.".to_string());
            MissionRewards::default()
        };

        MissionOutcome {
            success,
            mission_name: ongoing.mission.description.clone(),
            description: ongoing.mission.description,
            logs,
            rewards,
            disciple_indices: ongoing.disciple_indices,
        }
    }
}