use super::super::Game;

impl Game {
    pub(in crate::game) fn handle_send_diplomat(
        &mut self,
        faction_id: String,
        action: crate::data::relations::DiplomaticAction,
    ) {
        let results = self
            .world_sim
            .process_diplomatic_action(&faction_id, action);
        for result in results {
            self.handle_world_sim_result(result);
        }
    }

    pub(in crate::game) fn handle_respond_to_event(&mut self, event_id: String, choice_idx: usize) {
        let effects = self.world_sim.respond_to_event(&event_id, choice_idx);
        for effect in effects {
            self.apply_event_effect(&effect);
        }
    }
}
