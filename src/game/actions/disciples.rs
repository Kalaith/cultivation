use super::super::{BreakthroughResult, Game};
use crate::data::disciples::DiscipleRank;
use crate::data::history::DeceasedDisciple;
use crate::data::spirit_beasts::SpiritBeast;
use crate::engine::proc_gen::generate_disciple;
use crate::engine::random;
use crate::engine::tribulation::TribulationState;
use crate::game::moments::MomentKind;
use crate::state::StateTransition;

impl Game {
    pub(in crate::game) fn handle_recruit_disciple(&mut self) {
        let capacity = self.get_population_capacity();
        if capacity == 0 || self.disciples.len() as u32 >= capacity {
            self.event_log.push(
                "Population cap reached. Build Dormitories or upgrade the Sect Hall.".to_string(),
            );
            return;
        }

        let new_disciple = generate_disciple(&self.data);
        let recruit_name = new_disciple.name.clone();
        self.event_log
            .push(format!("Recruited: {}", new_disciple.name));
        self.disciples.push(new_disciple);
        self.show_moment(
            MomentKind::Recruitment,
            "A Young Cultivator Kneels",
            "New disciple admitted",
            format!(
                "{} has entered the mountain gate and entrusted their fate to the sect.",
                recruit_name
            ),
        );
    }

    pub(in crate::game) fn handle_promote_disciple(&mut self, idx: usize) {
        let Some(disciple) = self.disciples.get_mut(idx) else {
            return;
        };

        if disciple.rank != DiscipleRank::Outer {
            return;
        }

        if disciple.realm == "Mortal" {
            self.event_log
                .push("Cannot Promote: Must reach Qi Refinement first.".to_string());
            return;
        }

        let cost = 100;
        if self.spirit_stones < cost {
            self.event_log
                .push("Cannot Promote: Not enough Spirit Stones (100).".to_string());
            return;
        }

        self.spirit_stones -= cost;
        disciple.rank = DiscipleRank::Inner;
        if disciple.max_qi == 0 {
            disciple.max_qi = 100;
            disciple.qi = 100;
        }
        self.event_log
            .push(format!("Promoted {} to Inner Disciple!", disciple.name));
    }

    pub(in crate::game) fn handle_attempt_breakthrough_action(&mut self, idx: usize) {
        let Some(disciple) = self.disciples.get(idx).cloned() else {
            return;
        };

        if !disciple.can_attempt_breakthrough() {
            if disciple.is_injured() {
                self.event_log.push(format!(
                    "{} cannot attempt breakthrough while injured!",
                    disciple.name
                ));
            } else {
                self.event_log
                    .push(format!("{} is not ready for breakthrough.", disciple.name));
            }
            return;
        }

        let mut disciple = disciple;
        let disciple_name = disciple.name.clone();
        let previous_realm = disciple.realm.clone();
        let previous_sub_stage = disciple.sub_stage;
        let result = self.attempt_breakthrough(&mut disciple, idx);

        match result {
            BreakthroughResult::Failure => {
                self.show_moment(
                    MomentKind::Tragedy,
                    "A Soul Scatters",
                    "Breakthrough calamity",
                    format!(
                        "{} perished while forcing open the gate beyond {}.",
                        disciple.name, disciple.realm
                    ),
                );
                self.deceased_disciples.push(DeceasedDisciple::new(
                    disciple.name.clone(),
                    disciple.realm.clone(),
                    "Failed Breakthrough".to_string(),
                    self.tick,
                ));
                self.disciples.remove(idx);
            }
            BreakthroughResult::Tribulation(t_type) => {
                self.disciples[idx] = disciple.clone();
                self.show_moment(
                    MomentKind::Breakthrough,
                    "Tribulation Clouds Gather",
                    "Heaven tests the ambitious",
                    format!(
                        "{} has touched the threshold. Thunder gathers above the sect.",
                        disciple.name
                    ),
                );
                let trib_state = TribulationState::new(t_type, &disciple);
                self.transition(StateTransition::ToTribulation(trib_state, idx));
            }
            BreakthroughResult::Blocked => {}
            _ => {
                self.show_breakthrough_result_moment(
                    &disciple_name,
                    &previous_realm,
                    previous_sub_stage,
                    &disciple,
                    result,
                );
                disciple.breakthrough_readiness = 0.0;
                self.disciples[idx] = disciple;
            }
        }
    }

    pub(in crate::game) fn handle_assign_law(&mut self, disciple_idx: usize, law_id: String) {
        let Some(disciple) = self.disciples.get_mut(disciple_idx) else {
            return;
        };

        if self.data.laws.contains_key(&law_id) {
            disciple.law_id = Some(law_id.clone());
            self.event_log
                .push(format!("{} is now practicing {}.", disciple.name, law_id));
        }
    }

    pub(in crate::game) fn handle_recruit_spirit_beast(&mut self, def_id: String) {
        let Some(def) = self.data.spirit_beast_definitions.get(&def_id) else {
            self.event_log
                .push("Failed to recruit Spirit Beast: Unknown definition.".to_string());
            return;
        };

        let mut beast = SpiritBeast::new_from_definition(def);
        beast.id = random::next_u64();
        self.event_log
            .push(format!("Recruited Spirit Beast: {}", def.name));
        self.show_moment(
            MomentKind::SpiritBeast,
            "A Spirit Beast Answers",
            "Ancient pact renewed",
            format!(
                "{} has accepted the sect's aura and now guards the mountain paths.",
                def.name
            ),
        );
        self.spirit_beasts.push(beast);
    }

    fn show_breakthrough_result_moment(
        &mut self,
        disciple_name: &str,
        previous_realm: &str,
        previous_sub_stage: usize,
        disciple: &crate::data::disciples::Disciple,
        result: BreakthroughResult,
    ) {
        match result {
            BreakthroughResult::Success => {
                let changed_realm = disciple.realm != previous_realm;
                let changed_stage = disciple.sub_stage != previous_sub_stage;
                if changed_realm || changed_stage {
                    self.show_moment(
                        MomentKind::Breakthrough,
                        "Heaven and Earth Qi Gathers",
                        "Breakthrough achieved",
                        format!(
                            "{} advances on the immortal road. The sect annals record a new step in {}.",
                            disciple_name,
                            self.realm_display_name(&disciple.realm)
                        ),
                    );
                }
            }
            BreakthroughResult::Injured => {
                self.show_moment(
                    MomentKind::Tragedy,
                    "Meridians Burn Like Fire",
                    "Breakthrough failed",
                    format!(
                        "{} survived the backlash, but the path to immortality has demanded blood.",
                        disciple_name
                    ),
                );
            }
            _ => {}
        }
    }

    fn realm_display_name(&self, realm_id: &str) -> String {
        self.data
            .stages
            .get(realm_id)
            .map(|stage| stage.name.clone())
            .unwrap_or_else(|| realm_id.to_string())
    }
}
