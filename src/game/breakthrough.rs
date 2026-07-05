use super::{BreakthroughResult, Game, FOUNDATION_TRIAL_MISSION};
use crate::data::disciples::{Disciple, Injury, Talent};
use crate::engine::tribulation::TribulationType;
use macroquad_toolkit::rng as game_rng;

impl Game {
    fn has_completed_foundation_trial(&self, disciple_idx: usize) -> bool {
        self.completed_missions.iter().any(|m| {
            m.success
                && m.mission_name == FOUNDATION_TRIAL_MISSION
                && m.disciple_indices.len() == 1
                && m.disciple_indices[0] == disciple_idx
        })
    }

    /// Attempts a breakthrough. Returns result logic.
    pub(super) fn attempt_breakthrough(
        &mut self,
        disciple: &mut Disciple,
        disciple_idx: usize,
    ) -> BreakthroughResult {
        // Check hidden bottleneck
        if let Some(ref bottleneck) = disciple.breakthrough_bottleneck {
            let resolved = crate::engine::bottleneck::is_bottleneck_resolved(
                bottleneck,
                disciple,
                &self.completed_history,
                &self.inventory,
                &self.data,
            );
            if resolved {
                let desc = bottleneck.description(&self.data);
                self.event_log.push(format!(
                    "{} overcame their bottleneck: {}!",
                    disciple.name, desc
                ));
                disciple.breakthrough_bottleneck = None;
            } else {
                let desc = bottleneck.player_description(&self.data);
                self.event_log.push(format!(
                    "{} cannot break through yet — bottleneck: {}",
                    disciple.name, desc
                ));
                return BreakthroughResult::Blocked;
            }
        }

        // MVP cap gate: prevent major breakthrough beyond Foundation Establishment without solo trial
        if disciple.realm == "FoundationEstablishment" {
            if let Some(stage) = self.data.stages.get(&disciple.realm) {
                let at_peak = disciple.sub_stage >= stage.sub_stages.len().saturating_sub(1);
                if at_peak && !self.has_completed_foundation_trial(disciple_idx) {
                    self.event_log.push(format!(
                        "{} cannot advance beyond Foundation Establishment without completing the solo mission '{}'.",
                        disciple.name,
                        FOUNDATION_TRIAL_MISSION
                    ));
                    return BreakthroughResult::Blocked;
                }
            }
        }

        let base_chance = match disciple.talent {
            Talent::Low => 0.3,
            Talent::Medium => 0.5,
            Talent::High => 0.7,
            Talent::Genius => 0.9,
            Talent::HeavenSent => 1.0,
        };

        // Apply trait modifiers
        let trait_modifier: f32 = disciple
            .fate_traits
            .iter()
            .map(|t| t.breakthrough_modifier)
            .sum();

        // Bloodline modifiers
        let mut bloodline_breakthrough_mod = 0.0;
        let mut bloodline_survivor = false;
        let mut bloodline_injury_mod = 0.0;

        if let Some(bloodline_id) = &disciple.bloodline.bloodline_id {
            if let Some(bloodline) = self.data.bloodlines.get(bloodline_id) {
                let effectiveness = disciple.bloodline.effectiveness();
                bloodline_breakthrough_mod =
                    bloodline.passive_effects.breakthrough_modifier * effectiveness;
                bloodline_injury_mod = bloodline.passive_effects.injury_modifier * effectiveness;
                if bloodline.passive_effects.survivor {
                    bloodline_survivor = true;
                }
            }
        }

        // Check if disciple has Survivor trait (cannot die)
        let is_survivor = disciple.fate_traits.iter().any(|t| t.survivor) || bloodline_survivor;

        // Apply Law modifiers
        let mut law_modifier = 0.0;
        if let Some(law_id) = &disciple.law_id {
            if let Some(law) = self.data.laws.get(law_id) {
                law_modifier = law.breakthrough_modifier;
            }
        }

        // Readiness bonus from accumulated experience (0 to 0.5)
        let readiness_bonus = disciple.get_readiness_bonus();

        let success_chance = (base_chance
            + trait_modifier
            + law_modifier
            + bloodline_breakthrough_mod
            + readiness_bonus)
            .clamp(0.05, 0.99);

        self.event_log.push(format!(
            "{} attempts breakthrough ({}% chance, readiness bonus: +{:.0}%)",
            disciple.name,
            (success_chance * 100.0) as u32,
            readiness_bonus * 100.0
        ));

        if game_rng::rand() < success_chance {
            // Find current stage index using stages_order
            let stage_idx = self
                .data
                .stages_order
                .iter()
                .position(|id| id == &disciple.realm);
            let stage = self.data.stages.get(&disciple.realm);

            if let (Some(stage_idx), Some(stage)) = (stage_idx, stage) {
                // Check if can advance sub-stage
                if disciple.sub_stage < stage.sub_stages.len().saturating_sub(1) {
                    // Minor breakthrough (sub-stage advance)
                    disciple.sub_stage += 1;
                    let sub_stage_name = stage
                        .sub_stages
                        .get(disciple.sub_stage)
                        .map(|s| s.name.as_str())
                        .unwrap_or("Unknown");
                    self.event_log.push(format!(
                        "{} advanced to {} - {}!",
                        disciple.name, stage.name, sub_stage_name
                    ));
                    disciple.exp = 0;
                    disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 1.2) as u32;
                } else {
                    // Major breakthrough (next realm)
                    let next_stage_id = self.data.stages_order.get(stage_idx + 1);

                    if let Some(next_id) = next_stage_id {
                        if let Some(next_stage) = self.data.stages.get(next_id) {
                            // Determine if tribulation is needed (Core Formation and above)
                            let needs_tribulation = matches!(
                                disciple.realm.as_str(),
                                "FoundationEstablishment"
                                    | "CoreFormation"
                                    | "NascentSoul"
                                    | "SoulTransformation"
                                    | "Ascension"
                                    | "TrueImmortal"
                            );

                            if needs_tribulation {
                                self.event_log.push(format!(
                                    "Tribulation clouds gather above {}...",
                                    disciple.name
                                ));

                                let t_type = match disciple.realm.as_str() {
                                    "FoundationEstablishment" => TribulationType::GoldenCore,
                                    "CoreFormation" => TribulationType::NascentSoul,
                                    "NascentSoul" => TribulationType::SpiritSevering,
                                    "SoulTransformation" => TribulationType::Ascension,
                                    _ => TribulationType::GoldenCore,
                                };

                                return BreakthroughResult::Tribulation(t_type);
                            } else {
                                // Instant success for early realms
                                disciple.realm = next_stage.id.clone();
                                disciple.sub_stage = 0;
                                self.event_log.push(format!(
                                    "{} broke through to {} realm!",
                                    disciple.name, next_stage.name
                                ));
                                disciple.exp = 0;
                                disciple.exp_to_next_level =
                                    (disciple.exp_to_next_level as f32 * 2.5) as u32;
                            }
                        }
                    } else {
                        // Pinnacle
                        self.event_log.push(format!(
                            "{} has reached the apex of this world.",
                            disciple.name
                        ));
                        disciple.exp = disciple.exp_to_next_level;
                    }
                }
            } else {
                self.event_log.push(format!(
                    "Error: Unknown realm {} for {}",
                    disciple.realm, disciple.name
                ));
            }

            BreakthroughResult::Success
        } else {
            // Failed breakthrough - check for death
            let injury_modifier: f32 = disciple.fate_traits.iter().map(|t| t.injury_modifier).sum();
            let death_chance = (0.1 + injury_modifier + bloodline_injury_mod).clamp(0.0, 0.5);

            if !is_survivor && game_rng::rand() < death_chance {
                self.event_log.push(format!(
                    "{} perished attempting to break through!",
                    disciple.name
                ));
                BreakthroughResult::Failure
            } else {
                // Survivor trait or lucky - just injured
                let injury = Injury::from_breakthrough(&disciple.realm, is_survivor);
                let recovery_secs = injury.recovery_ticks_remaining;
                let time_str = if recovery_secs >= 60 {
                    format!("~{} min", recovery_secs / 60)
                } else {
                    format!("~{} sec", recovery_secs)
                };

                if is_survivor {
                    self.event_log.push(format!(
                        "{}'s indomitable will saved them! Suffered {} {} (recovery: {}).",
                        disciple.name,
                        injury.severity_str(),
                        injury.injury_type,
                        time_str
                    ));
                    disciple.exp = (disciple.exp as f32 * 0.25) as u32; // Lose 75% EXP
                } else {
                    self.event_log.push(format!(
                        "{} failed breakthrough and suffered {} {} (recovery: {}).",
                        disciple.name,
                        injury.severity_str(),
                        injury.injury_type,
                        time_str
                    ));
                    disciple.exp = (disciple.exp as f32 * 0.5) as u32; // Lose 50% EXP
                }

                disciple.injure(injury);
                BreakthroughResult::Injured
            }
        }
    }
}
