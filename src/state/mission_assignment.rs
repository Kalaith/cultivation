use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use macroquad::prelude::*;

pub struct MissionAssignmentState {
    pub mission_description: String,
}

impl MissionAssignmentState {
    pub fn new(mission_description: String) -> Self {
        Self { mission_description }
    }

    pub fn update(&mut self) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }
        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        draw_text("ASSIGN DISCIPLES", 20.0, 40.0, 40.0, WHITE);
        draw_text(&self.mission_description, 20.0, 100.0, 24.0, WHITE);
        draw_text("Press ESC to return to Sect Base", 20.0, 70.0, 20.0, GRAY);
    }
}
